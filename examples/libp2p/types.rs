use std::fmt::Debug;

use hotshot::traits::implementations::Libp2pNetwork;
use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResultsProvider, state_types::TestTypes,
    storage_types::TestStorage,
};
use hotshot_types::traits::node_implementation::{NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};

use crate::infra::Libp2pDaRun;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// Convenience type alias
pub type Network = Libp2pNetwork<<TestTypes as NodeType>::SignatureKey>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Network = Network;
    type Storage = TestStorage<TestTypes>;
    type AuctionResultsProvider = TestAuctionResultsProvider;
}
/// convenience type alias
pub type ThisRun = Libp2pDaRun<TestTypes>;
