use crate::infra::Libp2pRun;
use hotshot::traits::implementations::MemoryStorage;
use hotshot::{
    demos::vdemo::VDemoTypes,
    traits::{
        election::static_committee::GeneralStaticCommittee, implementations::Libp2pCommChannel,
    },
};
use hotshot_types::message::{Message, ValidatingMessage};
use hotshot_types::traits::{
    election::QuorumExchange,
    node_implementation::{ChannelMaps, NodeImplementation, ValidatingExchanges},
};
use hotshot_types::{
    data::{ValidatingLeaf, ValidatingProposal},
    traits::node_implementation::NodeType,
    vote::QuorumVote,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct NodeImpl {}

pub type ThisLeaf = ValidatingLeaf<VDemoTypes>;
pub type ThisMembership =
    GeneralStaticCommittee<VDemoTypes, ThisLeaf, <VDemoTypes as NodeType>::SignatureKey>;
pub type ThisNetwork =
    Libp2pCommChannel<VDemoTypes, NodeImpl, ThisProposal, ThisVote, ThisMembership>;

pub type ThisProposal = ValidatingProposal<VDemoTypes, ThisLeaf>;
pub type ThisVote = QuorumVote<VDemoTypes, ThisLeaf>;

impl NodeImplementation<VDemoTypes> for NodeImpl {
    type Storage = MemoryStorage<VDemoTypes, Self::Leaf>;
    type Leaf = ValidatingLeaf<VDemoTypes>;
    type Exchanges = ValidatingExchanges<
        VDemoTypes,
        Message<VDemoTypes, Self>,
        QuorumExchange<
            VDemoTypes,
            Self::Leaf,
            ThisProposal,
            ThisMembership,
            ThisNetwork,
            Message<VDemoTypes, Self>,
        >,
    >;
    type ConsensusMessage = ValidatingMessage<VDemoTypes, Self>;

    fn new_channel_maps(
        start_view: <VDemoTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<VDemoTypes, Self>,
        Option<ChannelMaps<VDemoTypes, Self>>,
    ) {
        (ChannelMaps::new(start_view), None)
    }
}
pub type ThisRun = Libp2pRun<VDemoTypes, NodeImpl, ThisMembership>;
