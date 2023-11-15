use crate::infra::WebServerDARun;
use hotshot::{
    demo::DemoTypes,
    traits::{
        election::static_committee::GeneralStaticCommittee,
        implementations::{MemoryStorage, WebCommChannel},
    },
};
use hotshot_types::{
    certificate::ViewSyncCertificate,
    data::{DAProposal, Leaf, QuorumProposal},
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

pub type ThisLeaf = Leaf<DemoTypes>;
pub type ThisMembership =
    GeneralStaticCommittee<DemoTypes, ThisLeaf, <DemoTypes as NodeType>::SignatureKey>;
pub type DANetwork = WebCommChannel<DemoTypes, NodeImpl, ThisMembership>;
pub type VIDNetwork = WebCommChannel<DemoTypes, NodeImpl, ThisMembership>;
pub type QuorumNetwork = WebCommChannel<DemoTypes, NodeImpl, ThisMembership>;
pub type ViewSyncNetwork = WebCommChannel<DemoTypes, NodeImpl, ThisMembership>;

pub type ThisDAProposal = DAProposal<DemoTypes>;

pub type ThisQuorumProposal = QuorumProposal<DemoTypes, ThisLeaf>;

pub type ThisViewSyncProposal = ViewSyncCertificate<DemoTypes>;
pub type ThisViewSyncVote = ViewSyncVote<DemoTypes>;

impl NodeImplementation<DemoTypes> for NodeImpl {
    type Storage = MemoryStorage<DemoTypes, Self::Leaf>;
    type Leaf = Leaf<DemoTypes>;
    type Exchanges = Exchanges<
        DemoTypes,
        Message<DemoTypes, Self>,
        QuorumExchange<
            DemoTypes,
            Self::Leaf,
            ThisQuorumProposal,
            ThisMembership,
            QuorumNetwork,
            Message<DemoTypes, Self>,
        >,
        CommitteeExchange<DemoTypes, ThisMembership, DANetwork, Message<DemoTypes, Self>>,
        ViewSyncExchange<
            DemoTypes,
            ThisViewSyncProposal,
            ThisMembership,
            ViewSyncNetwork,
            Message<DemoTypes, Self>,
        >,
        VIDExchange<DemoTypes, ThisMembership, VIDNetwork, Message<DemoTypes, Self>>,
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
pub type ThisRun = WebServerDARun<DemoTypes, NodeImpl, ThisMembership>;
