use crate::infra::Libp2pDARun;
use hotshot::{
    demo::DemoTypes,
    traits::implementations::{Libp2pCommChannel, MemoryStorage},
};
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = Libp2pCommChannel<DemoTypes>;
pub type VIDNetwork = Libp2pCommChannel<DemoTypes>;
pub type QuorumNetwork = Libp2pCommChannel<DemoTypes>;
pub type ViewSyncNetwork = Libp2pCommChannel<DemoTypes>;

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
pub type ThisRun = Libp2pDARun<DemoTypes>;
