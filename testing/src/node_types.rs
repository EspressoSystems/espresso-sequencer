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

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct MemoryImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Libp2pImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct WebImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct CombinedImpl;

pub type StaticMembership = StaticCommittee<TestTypes>;

pub type StaticMemoryDAComm = MemoryCommChannel<TestTypes>;

type StaticLibp2pDAComm = Libp2pCommChannel<TestTypes>;

type StaticWebDAComm = WebCommChannel<TestTypes>;

type StaticCombinedDAComm = CombinedCommChannel<TestTypes>;

pub type StaticMemoryQuorumComm = MemoryCommChannel<TestTypes>;

type StaticLibp2pQuorumComm = Libp2pCommChannel<TestTypes>;

type StaticWebQuorumComm = WebCommChannel<TestTypes>;

type StaticCombinedQuorumComm = CombinedCommChannel<TestTypes>;

pub type StaticMemoryViewSyncComm = MemoryCommChannel<TestTypes>;

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
