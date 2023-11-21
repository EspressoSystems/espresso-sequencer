use crate::infra::CombinedDARun;
use hotshot::{
    demo::DemoTypes,
    traits::implementations::{CombinedCommChannel, MemoryStorage},
};
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = CombinedCommChannel<DemoTypes>;
pub type VIDNetwork = CombinedCommChannel<DemoTypes>;
pub type QuorumNetwork = CombinedCommChannel<DemoTypes>;
pub type ViewSyncNetwork = CombinedCommChannel<DemoTypes>;

impl NodeImplementation<DemoTypes> for NodeImpl {
    type Storage = MemoryStorage<DemoTypes>;
    type QuorumNetwork = QuorumNetwork;
    type CommitteeNetwork = DANetwork;

    fn new_channel_maps(
        start_view: <DemoTypes as NodeType>::Time,
    ) -> (ChannelMaps<DemoTypes>, Option<ChannelMaps<DemoTypes>>) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = CombinedDARun<DemoTypes>;
