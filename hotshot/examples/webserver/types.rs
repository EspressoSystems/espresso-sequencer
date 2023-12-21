use crate::infra::WebServerDARun;
use hotshot::traits::implementations::{MemoryStorage, WebCommChannel};
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = WebCommChannel<TestTypes>;
pub type VIDNetwork = WebCommChannel<TestTypes>;
pub type QuorumNetwork = WebCommChannel<TestTypes>;
pub type ViewSyncNetwork = WebCommChannel<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Storage = MemoryStorage<TestTypes>;
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = WebServerDARun<TestTypes>;
