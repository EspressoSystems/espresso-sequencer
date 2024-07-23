use crate::{FeeAccount, FeeAmount, NamespaceId};
use anyhow::Result;
use async_trait::async_trait;
use ethers::types::Signature;
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeType};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersionType;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
/// Wrapper enum for Full Network Transactions. Each transaction type
/// will be a variant of this enum.
pub enum FullNetworkTx {
    Bid(BidTx),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
/// A transaction to bid for the sequencing rights of a namespace. It
/// is the `signed` form of `BidTxBody`. Expected usage is *build*
/// it by calling `signed` on `BidTxBody`.
pub struct BidTx {
    pub(crate) body: BidTxBody,
    pub(crate) signature: Signature,
}

/// A transaction body holding data required for bid submission.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct BidTxBody {
    /// Account responsible for the signature
    pub(crate) account: FeeAccount,
    /// Fee to be sequenced in the network.  Different than the bid_amount fee
    // FULL_NETWORK_GAS * MINIMUM_GAS_PRICE
    pub(crate) gas_price: FeeAmount,
    /// The bid amount designated in Wei.  This is different than
    /// the sequencing fee (gas price) for this transaction
    pub(crate) bid_amount: FeeAmount,
    /// The URL the HotShot leader will use to request a bundle
    /// from this sequencer if they win the auction
    pub(crate) url: Url,
    /// The slot this bid is for
    pub(crate) view: ViewNumber,
    /// The set of namespace ids the sequencer is bidding for
    pub(crate) namespaces: Vec<NamespaceId>,
}

// TODO this will be in HotShot
pub trait HasUrls {
    /// Returns the builer url associated with the datatype
    fn urls(&self) -> Vec<Url>;
}

// TODO this will be in HotShot
#[async_trait]
pub trait AuctionResultsProvider<TYPES: NodeType>: Send + Sync + Clone {
    /// The AuctionSolverResult is a type that holds the data associated with a particular solver
    /// run, for a particular view.
    type AuctionResult: HasUrls;

    /// Fetches the auction result for a view. Does not cache the result,
    /// subsequent calls will invoke additional wasted calls.
    async fn fetch_auction_result(&self, view_number: TYPES::Time) -> Result<Self::AuctionResult>;
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct AuctionResults {
    /// view number the results are for
    pub(crate) view_number: ViewNumber,
    /// A list of the bid txs that won
    pub(crate) winning_bids: Vec<BidTx>,
    /// A list of reserve sequencers being used
    pub(crate) reserve_bids: Vec<(NamespaceId, Url)>,
}

type SurfClient<Ver> = surf_disco::Client<ServerError, Ver>;

#[derive(Debug, Clone)]
pub struct SolverClient<Ver: StaticVersionType> {
    agent: SurfClient<Ver>,
    _url: Url,
}

impl<Ver: StaticVersionType> SolverClient<Ver> {
    pub fn new(url: Url) -> Self {
        Self {
            agent: SurfClient::new(url.clone()),
            _url: url,
        }
    }

    pub fn get<T: DeserializeOwned>(&self, route: &str) -> Request<T, ServerError, Ver> {
        self.agent.get(route)
    }
}
