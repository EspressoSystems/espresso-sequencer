use crate::infra::CombinedDARun;
use hotshot::traits::implementations::{CombinedCommChannel, MemoryStorage};
use hotshot_example_types::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// convenience type alias
pub type DANetwork = CombinedCommChannel<TestTypes>;
/// convenience type alias
pub type VIDNetwork = CombinedCommChannel<TestTypes>;
/// convenience type alias
pub type QuorumNetwork = CombinedCommChannel<TestTypes>;
/// convenience type alias
pub type ViewSyncNetwork = CombinedCommChannel<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = QuorumNetwork;
    type CommitteeNetwork = DANetwork;
}
/// convenience type alias
pub type ThisRun = CombinedDARun<TestTypes>;
