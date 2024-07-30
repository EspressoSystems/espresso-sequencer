use std::fmt::Debug;

use hotshot::traits::implementations::CombinedNetworks;
use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResultsProvider, state_types::TestTypes,
    storage_types::TestStorage,
};
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};

use crate::infra::CombinedDaRun;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// Convenience type alias
pub type Network = CombinedNetworks<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Network = Network;
    type Storage = TestStorage<TestTypes>;
    type AuctionResultsProvider = TestAuctionResultsProvider<TestTypes>;
}
/// convenience type alias
pub type ThisRun = CombinedDaRun<TestTypes>;
