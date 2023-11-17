use crate::infra::WebServerDARun;
use hotshot::{
    demo::DemoTypes,
    traits::implementations::{MemoryStorage, WebCommChannel},
};
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = WebCommChannel<DemoTypes>;
pub type VIDNetwork = WebCommChannel<DemoTypes>;
pub type QuorumNetwork = WebCommChannel<DemoTypes>;
pub type ViewSyncNetwork = WebCommChannel<DemoTypes>;

impl NodeImplementation<DemoTypes> for NodeImpl {
    type Storage = MemoryStorage<DemoTypes>;
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;

    fn new_channel_maps(
        start_view: <DemoTypes as NodeType>::Time,
    ) -> (ChannelMaps<DemoTypes>, Option<ChannelMaps<DemoTypes>>) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = WebServerDARun<DemoTypes>;
