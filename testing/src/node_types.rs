use hotshot::traits::implementations::CombinedNetworks;
use std::{marker::PhantomData, sync::Arc};

use hotshot::{
    demo::DemoState,
    traits::{
        election::static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
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
    certificate::ViewSyncCertificate,
    data::{Leaf, QuorumProposal, ViewNumber},
    message::{Message, SequencingMessage},
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
    type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
    type Transaction = VIDTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = DemoState;
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct MemoryImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Libp2pImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct WebImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct CombinedImpl;

pub type StaticMembership = StaticCommittee<TestTypes, Leaf<TestTypes>>;

pub type StaticMemoryDAComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pDAComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebDAComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedDAComm = CombinedCommChannel<TestTypes, CombinedImpl, StaticMembership>;

pub type StaticMemoryQuorumComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pQuorumComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebQuorumComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedQuorumComm = CombinedCommChannel<TestTypes, CombinedImpl, StaticMembership>;

pub type StaticMemoryViewSyncComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pViewSyncComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebViewSyncComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedViewSyncComm = CombinedCommChannel<TestTypes, CombinedImpl, StaticMembership>;

pub type StaticMemoryVIDComm = MemoryCommChannel<TestTypes, MemoryImpl, StaticMembership>;

type StaticLibp2pVIDComm = Libp2pCommChannel<TestTypes, Libp2pImpl, StaticMembership>;

type StaticWebVIDComm = WebCommChannel<TestTypes, WebImpl, StaticMembership>;

type StaticCombinedVIDComm = CombinedCommChannel<TestTypes, CombinedImpl, StaticMembership>;

pub type SequencingLibp2pExchange = Exchanges<
    TestTypes,
    Message<TestTypes, Libp2pImpl>,
    QuorumExchange<
        TestTypes,
        <Libp2pImpl as NodeImplementation<TestTypes>>::Leaf,
        QuorumProposal<TestTypes, Leaf<TestTypes>>,
        StaticMembership,
        StaticLibp2pQuorumComm,
        Message<TestTypes, Libp2pImpl>,
    >,
    CommitteeExchange<
        TestTypes,
        StaticMembership,
        StaticLibp2pDAComm,
        Message<TestTypes, Libp2pImpl>,
    >,
    ViewSyncExchange<
        TestTypes,
        ViewSyncCertificate<TestTypes>,
        StaticMembership,
        StaticLibp2pViewSyncComm,
        Message<TestTypes, Libp2pImpl>,
    >,
    VIDExchange<TestTypes, StaticMembership, StaticLibp2pVIDComm, Message<TestTypes, Libp2pImpl>>,
>;

impl NodeImplementation<TestTypes> for Libp2pImpl {
    type Storage = MemoryStorage<TestTypes, Leaf<TestTypes>>;
    type Exchanges = SequencingLibp2pExchange;
    type ConsensusMessage = SequencingMessage<TestTypes, Self>;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<TestTypes, Self>,
        Option<ChannelMaps<TestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl
    TestableExchange<
        TestTypes,
        <Libp2pImpl as NodeImplementation<TestTypes>>::Leaf,
        Message<TestTypes, Libp2pImpl>,
    > for SequencingLibp2pExchange
{
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
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<Libp2pNetwork<
            Message<TestTypes, Libp2pImpl>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, Libp2pImpl>,
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
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, Libp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

pub type SequencingMemoryExchange = Exchanges<
    TestTypes,
    Message<TestTypes, MemoryImpl>,
    QuorumExchange<
        TestTypes,
        <MemoryImpl as NodeImplementation<TestTypes>>::Leaf,
        QuorumProposal<TestTypes, Leaf<TestTypes>>,
        StaticMembership,
        StaticMemoryQuorumComm,
        Message<TestTypes, MemoryImpl>,
    >,
    CommitteeExchange<
        TestTypes,
        StaticMembership,
        StaticMemoryDAComm,
        Message<TestTypes, MemoryImpl>,
    >,
    ViewSyncExchange<
        TestTypes,
        ViewSyncCertificate<TestTypes>,
        StaticMembership,
        StaticMemoryViewSyncComm,
        Message<TestTypes, MemoryImpl>,
    >,
    VIDExchange<TestTypes, StaticMembership, StaticMemoryVIDComm, Message<TestTypes, MemoryImpl>>,
>;

impl
    TestableExchange<
        TestTypes,
        <MemoryImpl as NodeImplementation<TestTypes>>::Leaf,
        Message<TestTypes, MemoryImpl>,
    > for SequencingMemoryExchange
{
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
                    Message<TestTypes, MemoryImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<MemoryNetwork<
            Message<TestTypes, MemoryImpl>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, MemoryImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<MemoryNetwork<
            Message<TestTypes, MemoryImpl>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, MemoryImpl>,
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
                    Message<TestTypes, MemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, MemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<TestTypes> for MemoryImpl {
    type Storage = MemoryStorage<TestTypes, Leaf<TestTypes>>;
    type Exchanges = SequencingMemoryExchange;
    type ConsensusMessage = SequencingMessage<TestTypes, Self>;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<TestTypes, Self>,
        Option<ChannelMaps<TestTypes, Self>>,
    ) {
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
    Message<TestTypes, WebImpl>,
    QuorumExchange<
        TestTypes,
        <WebImpl as NodeImplementation<TestTypes>>::Leaf,
        QuorumProposal<TestTypes, Leaf<TestTypes>>,
        StaticMembership,
        StaticWebQuorumComm,
        Message<TestTypes, WebImpl>,
    >,
    CommitteeExchange<TestTypes, StaticMembership, StaticWebDAComm, Message<TestTypes, WebImpl>>,
    ViewSyncExchange<
        TestTypes,
        ViewSyncCertificate<TestTypes>,
        StaticMembership,
        StaticWebViewSyncComm,
        Message<TestTypes, WebImpl>,
    >,
    VIDExchange<TestTypes, StaticMembership, StaticWebVIDComm, Message<TestTypes, WebImpl>>,
>;

impl
    TestableExchange<
        TestTypes,
        <WebImpl as NodeImplementation<TestTypes>>::Leaf,
        Message<TestTypes, WebImpl>,
    > for SequencingWebExchanges
{
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
                    Message<TestTypes, WebImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes, WebImpl>,
            <TestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, WebImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes, WebImpl>,
            <TestTypes as NodeType>::SignatureKey,
            TestTypes,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, WebImpl>,
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
                    Message<TestTypes, WebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, WebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<TestTypes> for WebImpl {
    type Storage = MemoryStorage<TestTypes, Leaf<TestTypes>>;
    type Exchanges = SequencingWebExchanges;
    type ConsensusMessage = SequencingMessage<TestTypes, Self>;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<TestTypes, Self>,
        Option<ChannelMaps<TestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

pub type CombinedExchange = Exchanges<
    TestTypes,
    Message<TestTypes, CombinedImpl>,
    QuorumExchange<
        TestTypes,
        QuorumProposal<TestTypes>,
        StaticMembership,
        StaticCombinedQuorumComm,
        Message<TestTypes, CombinedImpl>,
    >,
    CommitteeExchange<
        TestTypes,
        StaticMembership,
        StaticCombinedDAComm,
        Message<TestTypes, CombinedImpl>,
    >,
    ViewSyncExchange<
        TestTypes,
        ViewSyncCertificate<TestTypes>,
        StaticMembership,
        StaticCombinedViewSyncComm,
        Message<TestTypes, CombinedImpl>,
    >,
    VIDExchange<
        TestTypes,
        StaticMembership,
        StaticCombinedVIDComm,
        Message<TestTypes, CombinedImpl>,
    >,
>;

impl NodeImplementation<TestTypes> for CombinedImpl {
    type Storage = MemoryStorage<TestTypes>;
    type Exchanges = CombinedExchange;
    type ConsensusMessage = SequencingMessage<TestTypes, Self>;

    fn new_channel_maps(
        start_view: <TestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<TestTypes, Self>,
        Option<ChannelMaps<TestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl TestableExchange<TestTypes, Message<TestTypes, CombinedImpl>> for CombinedExchange {
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
                    Message<TestTypes, CombinedImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let web_server_network_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes, CombinedImpl>,
            <TestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, CombinedImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));

        let web_server_network_da_generator = Arc::new(<WebServerNetwork<
            Message<TestTypes, CombinedImpl>,
            <TestTypes as NodeType>::SignatureKey,
            TestTypes,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, CombinedImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            true,
        ));

        let libp2p_network_generator = Arc::new(<Libp2pNetwork<
            Message<TestTypes, CombinedImpl>,
            <TestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            TestTypes,
            Message<TestTypes, CombinedImpl>,
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

            let network = Arc::new(CombinedNetworks(
                web_server_network,
                libp2p_network.clone(),
                PhantomData,
            ));
            let network_da = Arc::new(CombinedNetworks(
                web_server_network_da,
                libp2p_network,
                PhantomData,
            ));

            let quorum_chan =
                <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            let vid_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    TestTypes,
                    Message<TestTypes, CombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}
