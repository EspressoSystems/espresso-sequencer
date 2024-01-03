use hotshot::traits::election::static_committee::GeneralStaticCommittee;

use crate::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::TestState,
};

use hotshot::{
    traits::{
        election::static_committee::{StaticCommittee, StaticElectionConfig},
        implementations::{
            CombinedCommChannel, Libp2pCommChannel, MemoryCommChannel, MemoryStorage,
            WebCommChannel,
        },
        NodeImplementation,
    },
    types::bn254::BLSPubKey,
};
use hotshot_types::{
    data::ViewNumber,
    traits::node_implementation::{ChannelMaps, NodeType},
};
use serde::{Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
/// filler struct to implement node type and allow us
/// to select our traits
pub struct TestTypes;
impl NodeType for TestTypes {
    type Time = ViewNumber;
    type BlockHeader = TestBlockHeader;
    type BlockPayload = TestBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = TestTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = TestState;
    type Membership = GeneralStaticCommittee<TestTypes, Self::SignatureKey>;
}

/// Memory network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct MemoryImpl;

/// Libp2p network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Libp2pImpl;

/// Web server network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct WebImpl;

/// Combined Network implementation (libp2p + web sever)
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct CombinedImpl;

/// static committee type alias
pub type StaticMembership = StaticCommittee<TestTypes>;

/// memory network
pub type StaticMemoryDAComm = MemoryCommChannel<TestTypes>;

/// libp2p network
type StaticLibp2pDAComm = Libp2pCommChannel<TestTypes>;

/// web server network communication channel
type StaticWebDAComm = WebCommChannel<TestTypes>;

/// combined network
type StaticCombinedDAComm = CombinedCommChannel<TestTypes>;

/// memory comm channel
pub type StaticMemoryQuorumComm = MemoryCommChannel<TestTypes>;

/// libp2p comm channel
type StaticLibp2pQuorumComm = Libp2pCommChannel<TestTypes>;

/// web server comm channel
type StaticWebQuorumComm = WebCommChannel<TestTypes>;

/// combined network (libp2p + web server)
type StaticCombinedQuorumComm = CombinedCommChannel<TestTypes>;

/// memory network
pub type StaticMemoryViewSyncComm = MemoryCommChannel<TestTypes>;

/// memory network
pub type StaticMemoryVIDComm = MemoryCommChannel<TestTypes>;

impl NodeImplementation<TestTypes> for Libp2pImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = StaticLibp2pQuorumComm;
    type CommitteeNetwork = StaticLibp2pDAComm;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl NodeImplementation<TestTypes> for MemoryImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = StaticMemoryQuorumComm;
    type CommitteeNetwork = StaticMemoryDAComm;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl NodeImplementation<TestTypes> for WebImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = StaticWebQuorumComm;
    type CommitteeNetwork = StaticWebDAComm;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl NodeImplementation<TestTypes> for CombinedImpl {
    type Storage = MemoryStorage<TestTypes>;
    type QuorumNetwork = StaticCombinedQuorumComm;
    type CommitteeNetwork = StaticCombinedDAComm;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}
