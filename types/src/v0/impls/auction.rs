use crate::{
    eth_signature_key::{EthKeyPair, SigningError},
    v0_1::ValidatedState,
    v0_3::{BidTx, BidTxBody, FullNetworkTx},
    FeeAccount, FeeAmount, FeeError, FeeInfo, NamespaceId,
};
use committable::{Commitment, Committable};
use ethers::types::Signature;
use hotshot_types::{
    data::ViewNumber,
    traits::{
        auction_results_provider::HasUrl, node_implementation::ConsensusTime,
        signature_key::BuilderSignatureKey,
    },
};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

impl FullNetworkTx {
    /// Proxy for `execute` method of each transaction variant.
    pub fn execute(&self, state: &mut ValidatedState) -> Result<(), ExecutionError> {
        match self {
            Self::Bid(bid) => bid.execute(state),
        }
    }
}

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
    /// Construct a new `BidTxBody`.
    pub fn new(
        account: FeeAccount,
        bid: FeeAmount,
        view: ViewNumber,
        namespaces: Vec<NamespaceId>,
        url: Url,
    ) -> Self {
        Self {
            account,
            bid_amount: bid,
            view,
            namespaces,
            url,
            // TODO gas_price will come from config probably, but we
            // can use any value for first round of integration
            ..Self::default()
        }
    }

    /// Sign `BidTxBody` and return the signature.
    pub fn sign(&self, key: &EthKeyPair) -> Result<Signature, SigningError> {
        FeeAccount::sign_builder_message(key, self.commit().as_ref())
    }
    /// Sign Body and return a `BidTx`. This is the expected way to obtain a `BidTx`.
    /// ```
    /// # use espresso_types::FeeAccount;
    /// # use espresso_types::v0_3::BidTxBody;
    ///
    /// let key = FeeAccount::test_key_pair();
    /// BidTxBody::default().signed(&key).unwrap();
    /// ```
    pub fn signed(self, key: &EthKeyPair) -> Result<BidTx, SigningError> {
        let signature = self.sign(key)?;
        let bid = BidTx {
            body: self,
            signature,
        };
        Ok(bid)
    }

    /// Get account submitting the bid
    pub fn account(&self) -> FeeAccount {
        self.account
    }
    /// Get amount of bid
    pub fn amount(&self) -> FeeAmount {
        self.bid_amount
    }
    /// Update `url` field on a previously instantiated `BidTxBody`.
    pub fn with_url(self, url: Url) -> Self {
        Self { url, ..self }
    }
}

impl Default for BidTxBody {
    fn default() -> Self {
        let key = FeeAccount::test_key_pair();
        let nsid = NamespaceId::from(999u64);
        Self {
            url: Url::from_str("https://sequencer:3939").unwrap(),
            account: key.fee_account(),
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
    /// Transaction Signature could not be verified.
    InvalidSignature,
    #[error("Invalid Phase")]
    /// Transaction submitted during incorrect Marketplace Phase
    InvalidPhase,
    #[error("FeeError: {0}")]
    /// Insufficient funds or MerkleTree error.
    FeeError(FeeError),
    #[error("Could not resolve `ChainConfig`")]
    /// Could not resolve `ChainConfig`.
    UnresolvableChainConfig,
}

impl From<FeeError> for ExecutionError {
    fn from(e: FeeError) -> Self {
        Self::FeeError(e)
    }
}

impl BidTx {
    /// Execute `BidTx`.
    /// * verify signature
    /// * charge bid amount
    /// * charge gas
    pub fn execute(&self, state: &mut ValidatedState) -> Result<(), ExecutionError> {
        self.verify()?;

        // In JIT sequencer only receives winning bids. In AOT all
        // bids are charged as received (losing bids are refunded). In
        // any case we can charge the bids and gas during execution.
        self.charge(state)?;

        Ok(())
    }
    /// Charge Bid. Only winning bids are charged in JIT.
    fn charge(&self, state: &mut ValidatedState) -> Result<(), ExecutionError> {
        // As the code is currently organized, I think chain_config
        // will always be resolved here. But let's guard against the
        // error in case code is shifted around in the future.
        let Some(chain_config) = state.chain_config.resolve() else {
            return Err(ExecutionError::UnresolvableChainConfig);
        };

        // TODO change to `bid_recipient` when this logic is finally enabled
        let recipient = chain_config.fee_recipient;
        // Charge the bid amount
        state
            .charge_fee(FeeInfo::new(self.account(), self.amount()), recipient)
            .map_err(ExecutionError::from)?;

        // Charge the the gas amount
        state
            .charge_fee(FeeInfo::new(self.account(), self.gas_price()), recipient)
            .map_err(ExecutionError::from)?;

        Ok(())
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
    /// Update `url` field on a previously instantiated `BidTxBody`.
    pub fn with_url(self, url: Url) -> Self {
        let body = self.body.with_url(url);
        Self { body, ..self }
    }
    /// get gas price
    pub fn gas_price(&self) -> FeeAmount {
        self.body.gas_price
    }
    /// get bid amount
    pub fn amount(&self) -> FeeAmount {
        self.body.bid_amount
    }
    /// get bid amount
    pub fn account(&self) -> FeeAccount {
        self.body.account
    }
}

impl HasUrl for BidTx {
    /// Get the `url` field from the body.
    fn url(&self) -> Url {
        self.body.url()
    }
}

impl HasUrl for BidTxBody {
    /// Get the cloned `url` field.
    fn url(&self) -> Url {
        self.url.clone()
    }
}

mod test {
    use super::*;

    impl BidTx {
        pub fn mock(key: EthKeyPair) -> Self {
            BidTxBody::default().signed(&key).unwrap()
        }
    }

    #[test]
    fn test_mock_bid_tx_sign_and_verify() {
        let key = FeeAccount::test_key_pair();
        let bidtx = BidTx::mock(key);
        bidtx.verify().unwrap();
    }

    #[test]
    fn test_mock_bid_tx_charge() {
        let mut state = ValidatedState::default();
        let key = FeeAccount::test_key_pair();
        let bidtx = BidTx::mock(key);
        bidtx.charge(&mut state).unwrap();
    }

    #[test]
    fn test_bid_tx_construct() {
        let key_pair = EthKeyPair::random();
        BidTxBody::new(
            key_pair.fee_account(),
            FeeAmount::from(1),
            ViewNumber::genesis(),
            vec![NamespaceId::from(999u64)],
            Url::from_str("https://sequencer:3131").unwrap(),
        )
        .signed(&key_pair)
        .unwrap()
        .verify()
        .unwrap();
    }
}
