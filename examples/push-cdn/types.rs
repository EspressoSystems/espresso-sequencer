use crate::infra::PushCdnDaRun;
use hotshot::traits::implementations::PushCdnNetwork;
use hotshot_example_types::{state_types::TestTypes, storage_types::TestStorage};
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
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;
    type Storage = TestStorage<TestTypes>;
}

/// Convenience type alias
pub type ThisRun = PushCdnDaRun<TestTypes>;
