use crate::infra::Libp2pDARun;
use hotshot::traits::implementations::{Libp2pCommChannel, MemoryStorage};
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = Libp2pCommChannel<TestTypes>;
pub type VIDNetwork = Libp2pCommChannel<TestTypes>;
pub type QuorumNetwork = Libp2pCommChannel<TestTypes>;
pub type ViewSyncNetwork = Libp2pCommChannel<TestTypes>;

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
pub type ThisRun = Libp2pDARun<TestTypes>;
