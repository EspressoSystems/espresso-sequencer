use crate::infra::Libp2pDARun;
use hotshot::traits::implementations::{Libp2pCommChannel, MemoryStorage};
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// convenience type alias
pub type DANetwork = Libp2pCommChannel<TestTypes>;
/// convenience type alias
pub type VIDNetwork = Libp2pCommChannel<TestTypes>;
/// convenience type alias
pub type QuorumNetwork = Libp2pCommChannel<TestTypes>;
/// convenience type alias
pub type ViewSyncNetwork = Libp2pCommChannel<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = QuorumNetwork;
    type CommitteeNetwork = DANetwork;
}
/// convenience type alias
pub type ThisRun = Libp2pDARun<TestTypes>;
