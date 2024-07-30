use hotshot::traits::{implementations::PushCdnNetwork, NodeImplementation};
use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResultsProvider, state_types::TestTypes,
    storage_types::TestStorage,
};
use serde::{Deserialize, Serialize};

use crate::infra::PushCdnDaRun;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
/// Convenience type alias
pub struct NodeImpl {}

/// Convenience type alias
pub type Network = PushCdnNetwork<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Network = Network;
    type Storage = TestStorage<TestTypes>;
    type AuctionResultsProvider = TestAuctionResultsProvider<TestTypes>;
}

/// Convenience type alias
pub type ThisRun = PushCdnDaRun<TestTypes>;
