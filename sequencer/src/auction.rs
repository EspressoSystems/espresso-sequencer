use crate::{
    eth_signature_key::{EthKeyPair, SigningError},
    state::{FeeAccount, FeeAmount, FeeError, FeeInfo},
    NamespaceId, ValidatedState,
};
use committable::{Commitment, Committable};
use ethers::types::Signature;
use hotshot_types::{data::ViewNumber, traits::signature_key::BuilderSignatureKey};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
    str::FromStr,
};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
struct SequencerKey;

// for MVP-0(-1) (JIT)
// Sum of all sequencing fee match current check
// builder signature no longer includes payload
// new fee info flag (fee or bid)

pub struct MarketplaceResults {
    /// Slot these results are for
    slot: Slot,
    /// Bids that did not win, intially all bids are added to this,
    /// and then the winning bids are removed
    pending_bids: Vec<BidTx>,
    /// Map of winning sequencer public keys to the bundle of namespaces they bid for
    winner_map: HashMap<SequencerKey, HashSet<NamespaceId>>,
    /// Whether refunds have been processed for this slot
    refunds_processed: bool,
}

// - needs to be configured in genesis block
// - needs to be updatable
/// Configuration for the auction system
struct AuctionConfig {
    bid_phase_num_views: NonZeroU64,
    auction_phase_num_views: NonZeroU64,
    sequencing_phase_num_views: NonZeroU64,
}

/// Uniquely identifies an auction for sequencing rights of namespaces in the network
#[derive(Clone, Copy)]
struct AuctionId(u64);

/// Uniquely identifies one auction phase for a specific auction
#[derive(Clone, Copy)]
struct AuctionPhaseId(AuctionId, AuctionPhaseKind);

/// Describes one auction phase for a specific auction
#[derive(Clone, Copy)]
struct AuctionPhase {
    id: AuctionPhaseId,
    kind: AuctionPhaseKind,
    start: ViewNumber,
    end: ViewNumber,
}

/// Describes the 3 kinds of phases an active auction can be in
#[derive(Clone, Copy, Eq, PartialEq)]
enum AuctionPhaseKind {
    Bid,
    Assign,
    Sequence,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct Slot(u64);
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
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
    // TODO What is the correct type?
    /// The public key of this sequencer
    public_key: SequencerKey,
    /// The URL the HotShot leader will use to request a bundle
    /// from this sequencer if they win the auction
    url: Url,
    /// The slot this bid is for
    slot: Slot,
    /// The set of namespace ids the sequencer is bidding for
    bundle: Vec<NamespaceId>,
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
            .u64_field("slot", self.slot.0)
            .var_size_field(
                "bundle",
                // TODO what is the correct way to serialize `Vec<NamespaceId>`
                &bincode::serialize(&self.bundle.as_slice()).unwrap(),
            );
        comm.finalize()
    }
}

impl BidTxBody {
    /// Sign Tx
    pub fn sign(&self, key: &EthKeyPair) -> Result<Signature, SigningError> {
        FeeAccount::sign_builder_message(key, self.commit().as_ref())
    }
    /// Get account responsible for bid
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
            url: Url::from_str("htts://sequencer:3939/request_budle").unwrap(),
            account: key.fee_account(),
            public_key: SequencerKey,
            gas_price: FeeAmount::default(),
            bid_amount: FeeAmount::default(),
            slot: Slot::default(),
            bundle: vec![nsid],
        }
    }
}
impl Default for BidTx {
    fn default() -> Self {
        let body = BidTxBody::default();
        let commitment = body.commit();
        let key = FeeAccount::test_key_pair();
        let signature = FeeAccount::sign_builder_message(&key, commitment.as_ref()).unwrap();
        Self { signature, body }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ExecutionError {
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid Phase")]
    InvalidPhase,
    #[error("FeeError: {0}")]
    FeeError(FeeError),
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
    /// * store state (maybe not for JIT)
    // The rational behind the `Err` is to provide not only what
    // failed, but for which varient. The entire Tx is probably
    // overkill, but we can narrow down how much we want to know about
    // Failed Tx in the future. Maybe we just want its name.
    pub fn execute(
        &self,
        state: &mut ValidatedState,
    ) -> Result<(), (ExecutionError, FullNetworkTx)> {
        if get_phase() != AuctionPhaseKind::Bid {
            return Err((
                ExecutionError::InvalidPhase,
                FullNetworkTx::Bid(self.clone()),
            ));
        }
        self.verify()
            .map_err(|e| (e, FullNetworkTx::Bid(self.clone())))?;

        // TODO review when this actually occurs in JIT auction
        // charge the bid
        self.charge(state)
            .map_err(|e| (e, FullNetworkTx::Bid(self.clone())))?;

        // TODO do we still do this in JIT auction?
        store_in_marketplace_state();

        // TODO what do we return in good result?
        Ok(())
    }
    /// Charge Bid. Only winning bids are charged in JIT (I think).
    fn charge(&self, state: &mut ValidatedState) -> Result<(), ExecutionError> {
        // TODO can we assume validated_state.chain_config has been resolved at this point?
        let chain_config = state.chain_config.resolve().unwrap();
        let recipient = chain_config.bid_recipient;
        state
            .charge_fee(FeeInfo::from(self.clone()), recipient)
            .map_err(|e| e.into())
    }
    /// Cryptographic signature verification
    fn verify(&self) -> Result<(), ExecutionError> {
        if !self
            .body
            .account
            .validate_builder_signature(&self.signature, self.body.commit().as_ref())
        {
            return Err(ExecutionError::InvalidSignature);
        };

        Ok(())
    }
    pub fn body(&self) -> BidTxBody {
        self.body.clone()
    }
}

// TODO I'm not sure how phases will work in JIT
pub fn get_phase() -> AuctionPhaseKind {
    AuctionPhaseKind::Bid
}

fn store_in_marketplace_state() {
    unimplemented!();
}

/// Nonce for special (auction) transactions
struct Nonce(u64);

pub fn mock_full_network_txs() -> Vec<FullNetworkTx> {
    let x = FullNetworkTx::Bid(BidTx::default());
    dbg!(&x);
    vec![x]
}

mod test {
    use super::*;

    impl BidTx {
        fn mock(key: EthKeyPair) -> Self {
            let body = BidTxBody::default();
            let signature = body.sign(&key).unwrap();
            Self { signature, body }
        }
    }

    #[test]
    fn test_default_bidtx_body() {
        let bid = BidTxBody::default();
        dbg!(&bid);
    }

    #[test]
    fn test_mock_full_network_txs() {
        let x = mock_full_network_txs();
        dbg!(&x);
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
