use crate::infra::CombinedDARun;
use hotshot::traits::implementations::CombinedNetworks;
use hotshot_example_types::{state_types::TestTypes, storage_types::TestStorage};
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// convenience type alias
pub type DANetwork = CombinedNetworks<TestTypes>;
/// convenience type alias
pub type VIDNetwork = CombinedNetworks<TestTypes>;
/// convenience type alias
pub type QuorumNetwork = CombinedNetworks<TestTypes>;
/// convenience type alias
pub type ViewSyncNetwork = CombinedNetworks<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type QuorumNetwork = QuorumNetwork;
    type CommitteeNetwork = DANetwork;
    type Storage = TestStorage<TestTypes>;
}
/// convenience type alias
pub type ThisRun = CombinedDARun<TestTypes>;
