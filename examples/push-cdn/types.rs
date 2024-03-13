use crate::infra::PushCdnDaRun;
use hotshot::traits::implementations::{MemoryStorage, PushCdnNetwork};
use hotshot_example_types::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
/// Convenience type alias
pub struct NodeImpl {}

/// Convenience type alias
pub type DANetwork = PushCdnNetwork<TestTypes>;
/// Convenience type alias
pub type VIDNetwork = PushCdnNetwork<TestTypes>;
/// Convenience type alias
pub type QuorumNetwork = PushCdnNetwork<TestTypes>;
/// Convenience type alias
pub type ViewSyncNetwork = PushCdnNetwork<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Storage = MemoryStorage<TestTypes>;
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;
}

/// Convenience type alias
pub type ThisRun = PushCdnDaRun<TestTypes>;
