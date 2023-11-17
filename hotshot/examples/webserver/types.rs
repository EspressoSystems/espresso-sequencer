use crate::infra::WebServerDARun;
use hotshot::{
    demo::{DemoMembership, DemoTypes},
    traits::implementations::{MemoryStorage, WebCommChannel},
};
use hotshot_types::{
    message::{Message, SequencingMessage},
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

pub type DANetwork = WebCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type VIDNetwork = WebCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type QuorumNetwork = WebCommChannel<DemoTypes, NodeImpl, DemoMembership>;
pub type ViewSyncNetwork = WebCommChannel<DemoTypes, NodeImpl, DemoMembership>;

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
    type ConsensusMessage = SequencingMessage<DemoTypes, Self>;

    fn new_channel_maps(
        start_view: <DemoTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<DemoTypes, Self>,
        Option<ChannelMaps<DemoTypes, Self>>,
    ) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = WebServerDARun<DemoTypes, NodeImpl, DemoMembership>;
