use crate::infra::Libp2pDARun;
use hotshot::{
    demo::{DemoMembership, DemoTypes},
    traits::implementations::{Libp2pCommChannel, MemoryStorage},
};
use hotshot_types::{
    message::Message,
    traits::{
        election::{CommitteeExchange, QuorumExchange, VIDExchange, ViewSyncExchange},
        node_implementation::{ChannelMaps, Exchanges, NodeImplementation, NodeType},
    },
    vote::ViewSyncVote,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

pub type DANetwork = Libp2pCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type VIDNetwork = Libp2pCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type QuorumNetwork = Libp2pCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type ViewSyncNetwork = Libp2pCommChannel<DemoTypes, NodeImpl, DemoMembership>;

pub type ThisViewSyncVote = ViewSyncVote<DemoTypes>;

impl NodeImplementation<DemoTypes> for NodeImpl {
    type Storage = MemoryStorage<DemoTypes>;
    type Exchanges = Exchanges<
        DemoTypes,
        Message<DemoTypes, Self>,
        QuorumExchange<DemoTypes, DemoMembership, QuorumNetwork, Message<DemoTypes, Self>>,
        CommitteeExchange<DemoTypes, DemoMembership, DANetwork, Message<DemoTypes, Self>>,
        ViewSyncExchange<DemoTypes, DemoMembership, ViewSyncNetwork, Message<DemoTypes, Self>>,
        VIDExchange<DemoTypes, DemoMembership, VIDNetwork, Message<DemoTypes, Self>>,
    >;

    fn new_channel_maps(
        start_view: <DemoTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<DemoTypes, Self>,
        Option<ChannelMaps<DemoTypes, Self>>,
    ) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = Libp2pDARun<DemoTypes, NodeImpl, DemoMembership>;
