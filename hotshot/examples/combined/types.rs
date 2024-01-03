use crate::infra::CombinedDARun;
use hotshot::traits::implementations::{CombinedCommChannel, MemoryStorage};
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
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

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (ChannelMaps::new(start_view), None)
    }
}
/// convenience type alias
pub type ThisRun = CombinedDARun<TestTypes>;
