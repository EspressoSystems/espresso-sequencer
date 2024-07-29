use super::state::ValidatedState;
use crate::{
    eth_signature_key::{EthKeyPair, SigningError},
    v0_3::{BidTx, BidTxBody, FullNetworkTx, SolverAuctionResults},
    FeeAccount, FeeAmount, FeeError, FeeInfo, NamespaceId,
};
use async_trait::async_trait;
use committable::{Commitment, Committable};
use ethers::types::Signature;
use hotshot_types::{
    data::ViewNumber,
    traits::{
        auction_results_provider::{AuctionResultsProvider, HasUrls},
        node_implementation::{ConsensusTime, NodeType},
        signature_key::BuilderSignatureKey,
    },
};
use std::str::FromStr;
use thiserror::Error;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersion;

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
    fn sign(&self, key: &EthKeyPair) -> Result<Signature, SigningError> {
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
    /// get the view number
    pub fn view(&self) -> ViewNumber {
        self.view
    }
    /// Instantiate a `BidTxBody` containing the values of `self`
    /// with a new `url` field.
    pub fn with_url(self, url: Url) -> Self {
        Self { url, ..self }
    }

    /// Get the cloned `url` field.
    fn url(&self) -> Url {
        self.url.clone()
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
        BidTxBody::default()
            .signed(&FeeAccount::test_key_pair())
            .unwrap()
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

        let recipient = chain_config.bid_recipient.unwrap();
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
    /// Instantiate a `BidTx` containing the values of `self`
    /// with a new `url` field on `body`.
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
    /// get bid account
    pub fn account(&self) -> FeeAccount {
        self.body.account
    }
    /// get the view number
    pub fn view(&self) -> ViewNumber {
        self.body.view
    }
    /// Get the `url` field from the body.
    fn url(&self) -> Url {
        self.body.url()
    }
}

impl SolverAuctionResults {
    pub fn winning_bids(&self) -> &[BidTx] {
        &self.winning_bids
    }
    pub fn reserve_bids(&self) -> &[(NamespaceId, Url)] {
        &self.reserve_bids
    }
    pub fn genesis() -> Self {
        Self {
            view_number: ViewNumber::genesis(),
            winning_bids: vec![],
            reserve_bids: vec![],
        }
    }
}

impl HasUrls for SolverAuctionResults {
    fn urls(&self) -> Vec<Url> {
        self.winning_bids()
            .iter()
            .map(|bid| bid.url())
            .chain(self.reserve_bids().iter().map(|bid| bid.1.clone()))
            .collect()
    }
}

const SOLVER_URL: &str = "https://solver:1234";
type Ver = StaticVersion<0, 3>;
type SurfClient<Ver> = surf_disco::Client<ServerError, Ver>;

#[async_trait]
impl<TYPES: NodeType> AuctionResultsProvider<TYPES> for SolverAuctionResults {
    type AuctionResult = SolverAuctionResults;

    /// Fetch the auction results.
    async fn fetch_auction_result(
        &self,
        view_number: TYPES::Time,
    ) -> anyhow::Result<Self::AuctionResult> {
        let resp = SurfClient::<Ver>::new(Url::from_str(SOLVER_URL).unwrap())
            .get::<SolverAuctionResults>(&format!("/v0/api/auction_results/{}", *view_number))
            .send()
            .await
            .unwrap();
        Ok(resp)
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
