use hotshot::traits::{
    election::static_committee::GeneralStaticCommittee, implementations::CombinedNetworks,
};
use std::sync::Arc;

use hotshot::{
    demo::DemoState,
    traits::{
        election::static_committee::{StaticCommittee, StaticElectionConfig},
        implementations::{
            CombinedCommChannel, Libp2pCommChannel, Libp2pNetwork, MemoryCommChannel,
            MemoryNetwork, MemoryStorage, WebCommChannel, WebServerNetwork,
        },
        NodeImplementation,
    },
    types::bn254::BLSPubKey,
};
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload, VIDTransaction},
    data::ViewNumber,
    message::Message,
    traits::{
        election::{CommitteeExchange, QuorumExchange, VIDExchange, ViewSyncExchange},
        network::{TestableChannelImplementation, TestableNetworkingImplementation},
        node_implementation::{ChannelMaps, Exchanges, NodeType, TestableExchange},
    },
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
    type BlockHeader = VIDBlockHeader;
    type BlockPayload = VIDBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = VIDTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = DemoState;
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

pub type StaticMemoryDAComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pDAComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebDAComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedDAComm = CombinedCommChannel<TestTypes>;

pub type StaticMemoryQuorumComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pQuorumComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebQuorumComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedQuorumComm = CombinedCommChannel<TestTypes>;

pub type StaticMemoryViewSyncComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pViewSyncComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebViewSyncComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedViewSyncComm = CombinedCommChannel<TestTypes>;

pub type StaticMemoryVIDComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pVIDComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebVIDComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedVIDComm = CombinedCommChannel<TestTypes>;

pub type SequencingLibp2pExchange = Exchanges<
    TestTypes,
    Message<TestTypes>,
    QuorumExchange<TestTypes, StaticMembership, StaticLibp2pQuorumComm, Message<TestTypes>>,
    CommitteeExchange<TestTypes, StaticMembership, StaticLibp2pDAComm, Message<TestTypes>>,
    ViewSyncExchange<TestTypes, StaticMembership, StaticLibp2pViewSyncComm, Message<TestTypes>>,
    VIDExchange<TestTypes, StaticMembership, StaticLibp2pVIDComm, Message<TestTypes>>,
>;

impl NodeImplementation<TestTypes> for Libp2pImpl {
    type Storage = MemoryStorage<TestTypes>;
    type Exchanges = SequencingLibp2pExchange;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl TestableExchange<TestTypes, Message<TestTypes>> for SequencingLibp2pExchange {
    #[allow(clippy::arc_with_non_send_sync)]
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
    ) -> Box<
        dyn Fn(
                u64,
            ) -> (
                <Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<Libp2pNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));

        Box::new(move |id| {
            let network = Arc::new(network_generator(id));
            let quorum_chan =
                <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

pub type SequencingMemoryExchange = Exchanges<
    TestTypes,
    Message<TestTypes>,
    QuorumExchange<TestTypes, StaticMembership, StaticMemoryQuorumComm, Message<TestTypes>>,
    CommitteeExchange<TestTypes, StaticMembership, StaticMemoryDAComm, Message<TestTypes>>,
    ViewSyncExchange<TestTypes, StaticMembership, StaticMemoryViewSyncComm, Message<TestTypes>>,
    VIDExchange<TestTypes, StaticMembership, StaticMemoryVIDComm, Message<TestTypes>>,
>;

impl TestableExchange<TestTypes, Message<TestTypes>> for SequencingMemoryExchange {
    #[allow(clippy::arc_with_non_send_sync)]
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
    ) -> Box<
        dyn Fn(
                u64,
            ) -> (
                <Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<MemoryNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<MemoryNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            true,
        ));
        Box::new(move |id| {
            let network = Arc::new(network_generator(id));
            let network_da = Arc::new(network_da_generator(id));
            let quorum_chan =
                <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<TestTypes> for MemoryImpl {
    type Storage = MemoryStorage<TestTypes>;
    type Exchanges = SequencingMemoryExchange;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

// man these generics are big oof
// they're a LOT
// when are we getting HKT for rust
// smh my head

pub type SequencingWebExchanges = Exchanges<
    TestTypes,
    Message<TestTypes>,
    QuorumExchange<TestTypes, StaticMembership, StaticWebQuorumComm, Message<TestTypes>>,
    CommitteeExchange<TestTypes, StaticMembership, StaticWebDAComm, Message<TestTypes>>,
    ViewSyncExchange<TestTypes, StaticMembership, StaticWebViewSyncComm, Message<TestTypes>>,
    VIDExchange<TestTypes, StaticMembership, StaticWebVIDComm, Message<TestTypes>>,
>;

impl TestableExchange<TestTypes, Message<TestTypes>> for SequencingWebExchanges {
    #[allow(clippy::arc_with_non_send_sync)]
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
    ) -> Box<
        dyn Fn(
                u64,
            ) -> (
                <Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
            TestTypes,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            true,
        ));
        Box::new(move |id| {
            let network = Arc::new(network_generator(id));
            let network_da = Arc::new(network_da_generator(id));
            let quorum_chan =
                <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<TestTypes> for WebImpl {
    type Storage = MemoryStorage<TestTypes>;
    type Exchanges = SequencingWebExchanges;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

pub type CombinedExchange = Exchanges<
    TestTypes,
    Message<TestTypes>,
    QuorumExchange<TestTypes, StaticMembership, StaticCombinedQuorumComm, Message<TestTypes>>,
    CommitteeExchange<TestTypes, StaticMembership, StaticCombinedDAComm, Message<TestTypes>>,
    ViewSyncExchange<TestTypes, StaticMembership, StaticCombinedViewSyncComm, Message<TestTypes>>,
    VIDExchange<TestTypes, StaticMembership, StaticCombinedVIDComm, Message<TestTypes>>,
>;

impl NodeImplementation<TestTypes> for CombinedImpl {
    type Storage = MemoryStorage<TestTypes>;
    type Exchanges = CombinedExchange;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (ChannelMaps<TestTypes>, Option<ChannelMaps<TestTypes>>) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl TestableExchange<TestTypes, Message<TestTypes>> for CombinedExchange {
    #[allow(clippy::arc_with_non_send_sync)]
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
    ) -> Box<
        dyn Fn(
                u64,
            ) -> (
                <Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking,
            ) + 'static,
    > {
        let web_server_network_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));

        let web_server_network_da_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
            TestTypes,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            true,
        ));

        let libp2p_network_generator = Arc::new(<Libp2pNetwork<
            Message<TestTypes>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            2,
            da_committee_size,
            true,
        ));

        // libp2p
        Box::new(move |id| {
            let web_server_network = web_server_network_generator(id);
            let web_server_network_da = web_server_network_da_generator(id);

            let libp2p_network = libp2p_network_generator(id);

            let network = Arc::new(CombinedNetworks(web_server_network, libp2p_network.clone()));
            let network_da = Arc::new(CombinedNetworks(web_server_network_da, libp2p_network));

            let quorum_chan =
                <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            let vid_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}
