use crate::{
    eth_signature_key::{EthKeyPair, SigningError},
    state::{FeeAccount, FeeAmount, FeeError, FeeInfo},
    NamespaceId, ValidatedState,
};
use committable::{Commitment, Committable};
use ethers::types::Signature;
use hotshot_types::{
    data::ViewNumber,
    traits::{node_implementation::ConsensusTime, signature_key::BuilderSignatureKey},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
/// Wrapper enum for Full Network Transactions. Each transaction type
/// will be a variant of this enum.
pub enum FullNetworkTx {
    Bid(BidTx),
}

impl FullNetworkTx {
    pub fn execute(
        &self,
        state: &mut ValidatedState,
    ) -> Result<(), (ExecutionError, FullNetworkTx)> {
        match self {
            Self::Bid(bid) => bid.execute(state),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct BidTx {
    body: BidTxBody,
    signature: Signature,
}

/// A transaction to bid for the sequencing rights of a namespace
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct BidTxBody {
    /// Account responsible for the signature
    account: FeeAccount,
    /// Fee to be sequenced in the network.  Different than the bid_amount fee
    // FULL_NETWORK_GAS * MINIMUM_GAS_PRICE
    gas_price: FeeAmount,
    /// The bid amount designated in Wei.  This is different than
    /// the sequencing fee (gas price) for this transaction
    bid_amount: FeeAmount,
    // TODO I think this will end up being a `FeeAccount`
    /// The public key of this sequencer
    public_key: FeeAccount,
    /// The URL the HotShot leader will use to request a bundle
    /// from this sequencer if they win the auction
    url: Url,
    /// The slot this bid is for
    view: ViewNumber,
    /// The set of namespace ids the sequencer is bidding for
    namespaces: Vec<NamespaceId>,
}

// TODO consider a committable derive macro
impl Committable for BidTxBody {
    fn tag() -> String {
        "BID_TX".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        let comm = committable::RawCommitmentBuilder::new(&Self::tag())
            .fixed_size_field("account", &self.account.to_fixed_bytes())
            .fixed_size_field("gas_price", &self.gas_price.to_fixed_bytes())
            .fixed_size_field("bid_amount", &self.bid_amount.to_fixed_bytes())
            .var_size_field("url", self.url.as_str().as_ref())
            .u64_field("view", self.view.u64())
            .var_size_field("namespaces", &bincode::serialize(&self.namespaces).unwrap());
        comm.finalize()
    }
}

impl BidTxBody {
    /// Sign Tx
    pub fn sign(&self, key: &EthKeyPair) -> Result<Signature, SigningError> {
        FeeAccount::sign_builder_message(key, self.commit().as_ref())
    }
    /// Get account submitting the bid
    pub fn account(&self) -> FeeAccount {
        self.account
    }
    /// Get amount of bid
    pub fn amount(&self) -> FeeAmount {
        self.bid_amount
    }
}

impl Default for BidTxBody {
    fn default() -> Self {
        let key = FeeAccount::test_key_pair();
        let nsid = NamespaceId::from(999);
        Self {
            // TODO url will be builder_url, needs to be passed in from somewhere
            url: Url::from_str("htts://sequencer:3939").unwrap(),
            account: key.fee_account(),
            public_key: FeeAccount::default(),
            gas_price: FeeAmount::default(),
            bid_amount: FeeAmount::default(),
            view: ViewNumber::genesis(),
            namespaces: vec![nsid],
        }
    }
}
impl Default for BidTx {
    fn default() -> Self {
        let body = BidTxBody::default();
        let key = FeeAccount::test_key_pair();
        let signature = FeeAccount::sign_builder_message(&key, body.commit().as_ref()).unwrap();
        Self { signature, body }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
/// Failure cases of transaction execution
pub enum ExecutionError {
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid Phase")]
    InvalidPhase,
    #[error("FeeError: {0}")]
    FeeError(FeeError),
    #[error("Could not resolve ChainConfig")]
    UnresolvableChainConfig,
}

impl From<FeeError> for ExecutionError {
    fn from(e: FeeError) -> Self {
        Self::FeeError(e)
    }
}

// TODO consider moving common functionality to trait.
impl BidTx {
    /// Executes `BidTx`.
    /// * verify signature
    /// * charge fee
    // The rational behind the `Err` is to provide not only what
    // failed, but for which variant. The entire Tx is probably
    // overkill, but we can narrow down how much we want to know about
    // Failed Tx in the future. Maybe we just want its name.
    pub fn execute(
        &self,
        state: &mut ValidatedState,
    ) -> Result<(), (ExecutionError, FullNetworkTx)> {
        self.verify()
            .map_err(|e| (e, FullNetworkTx::Bid(self.clone())))?;

        // In JIT sequencer only receives winning bids. In AOT all
        // bids are charged as received (losing bids are refunded). In
        // any case we can charge the bids and gas during execution.
        self.charge(state)
            .map_err(|e| (e, FullNetworkTx::Bid(self.clone())))?;

        // TODO what do we return in good result?
        Ok(())
    }
    /// Charge Bid. Only winning bids are charged in JIT (I think).
    fn charge(&self, state: &mut ValidatedState) -> Result<(), ExecutionError> {
        // As the code is currently organized, I think chain_config
        // will always be resolved here. But let's guard against the
        // error in case code is shifted around in the future.
        if let Some(chain_config) = state.chain_config.resolve() {
            let recipient = chain_config.bid_recipient;
            // TODO also charge gas fee
            state
                .charge_fee(FeeInfo::from(self.clone()), recipient)
                .map_err(ExecutionError::from)
        } else {
            Err(ExecutionError::UnresolvableChainConfig)
        }
    }
    /// Cryptographic signature verification
    fn verify(&self) -> Result<(), ExecutionError> {
        self.body
            .account
            .validate_builder_signature(&self.signature, self.body.commit().as_ref())
            .then_some(())
            .ok_or(ExecutionError::InvalidSignature)
    }
    /// Return the body of the transaction
    pub fn body(self) -> BidTxBody {
        self.body
    }
}

pub fn mock_full_network_txs(key: Option<EthKeyPair>) -> Vec<FullNetworkTx> {
    // if no key is supplied, use `test_key_pair`. Since default `BidTxBody` is
    // signed with `test_key_pair`, it will verify successfully
    let key = key.unwrap_or_else(FeeAccount::test_key_pair);
    vec![FullNetworkTx::Bid(BidTx::mock(key))]
}

mod test {
    use super::*;

    impl BidTx {
        pub fn mock(key: EthKeyPair) -> Self {
            let body = BidTxBody::default();
            let signature = body.sign(&key).unwrap();
            Self { signature, body }
        }
    }

    #[test]
    fn test_sign_and_verify_mock_bid() {
        let key = FeeAccount::test_key_pair();
        let bidtx = BidTx::mock(key);
        bidtx.verify().unwrap();
    }

    #[test]
    fn test_charge_mock_bid() {
        let mut state = ValidatedState::default();
        let key = FeeAccount::test_key_pair();
        let bidtx = BidTx::mock(key);
        bidtx.charge(&mut state).unwrap();
    }
}
