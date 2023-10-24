use hotshot::traits::implementations::CombinedNetworks;
use std::{marker::PhantomData, sync::Arc};

use hotshot::{
    demo::SDemoState,
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
    data::{QuorumProposal, SequencingLeaf, ViewNumber},
    message::{Message, SequencingMessage},
    traits::{
        election::{CommitteeExchange, QuorumExchange, VIDExchange, ViewSyncExchange},
        network::{TestableChannelImplementation, TestableNetworkingImplementation},
        node_implementation::{ChannelMaps, NodeType, SequencingExchanges, TestableExchange},
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
pub struct SequencingTestTypes;
impl NodeType for SequencingTestTypes {
    type Time = ViewNumber;
    type BlockHeader = VIDBlockHeader;
    type BlockPayload = VIDBlockPayload;
    type SignatureKey = BLSPubKey;
    type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
    type Transaction = VIDTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = SDemoState;
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct SequencingMemoryImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct SequencingLibp2pImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct SequencingWebImpl;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct SequencingCombinedImpl;

pub type StaticMembership =
    StaticCommittee<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;

pub type StaticMemoryDAComm =
    MemoryCommChannel<SequencingTestTypes, SequencingMemoryImpl, StaticMembership>;

type StaticLibp2pDAComm =
    Libp2pCommChannel<SequencingTestTypes, SequencingLibp2pImpl, StaticMembership>;

type StaticWebDAComm = WebCommChannel<SequencingTestTypes, SequencingWebImpl, StaticMembership>;

type StaticCombinedDAComm =
    CombinedCommChannel<SequencingTestTypes, SequencingCombinedImpl, StaticMembership>;

pub type StaticMemoryQuorumComm =
    MemoryCommChannel<SequencingTestTypes, SequencingMemoryImpl, StaticMembership>;

type StaticLibp2pQuorumComm =
    Libp2pCommChannel<SequencingTestTypes, SequencingLibp2pImpl, StaticMembership>;

type StaticWebQuorumComm = WebCommChannel<SequencingTestTypes, SequencingWebImpl, StaticMembership>;

type StaticCombinedQuorumComm =
    CombinedCommChannel<SequencingTestTypes, SequencingCombinedImpl, StaticMembership>;

pub type StaticMemoryViewSyncComm =
    MemoryCommChannel<SequencingTestTypes, SequencingMemoryImpl, StaticMembership>;

type StaticLibp2pViewSyncComm =
    Libp2pCommChannel<SequencingTestTypes, SequencingLibp2pImpl, StaticMembership>;

type StaticWebViewSyncComm =
    WebCommChannel<SequencingTestTypes, SequencingWebImpl, StaticMembership>;

type StaticCombinedViewSyncComm =
    CombinedCommChannel<SequencingTestTypes, SequencingCombinedImpl, StaticMembership>;

pub type StaticMemoryVIDComm =
    MemoryCommChannel<SequencingTestTypes, SequencingMemoryImpl, StaticMembership>;

type StaticLibp2pVIDComm =
    Libp2pCommChannel<SequencingTestTypes, SequencingLibp2pImpl, StaticMembership>;

type StaticWebVIDComm = WebCommChannel<SequencingTestTypes, SequencingWebImpl, StaticMembership>;

type StaticCombinedVIDComm =
    CombinedCommChannel<SequencingTestTypes, SequencingCombinedImpl, StaticMembership>;

pub type SequencingLibp2pExchange = SequencingExchanges<
    SequencingTestTypes,
    Message<SequencingTestTypes, SequencingLibp2pImpl>,
    QuorumExchange<
        SequencingTestTypes,
        <SequencingLibp2pImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
        StaticMembership,
        StaticLibp2pQuorumComm,
        Message<SequencingTestTypes, SequencingLibp2pImpl>,
    >,
    CommitteeExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticLibp2pDAComm,
        Message<SequencingTestTypes, SequencingLibp2pImpl>,
    >,
    ViewSyncExchange<
        SequencingTestTypes,
        ViewSyncCertificate<SequencingTestTypes>,
        StaticMembership,
        StaticLibp2pViewSyncComm,
        Message<SequencingTestTypes, SequencingLibp2pImpl>,
    >,
    VIDExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticLibp2pVIDComm,
        Message<SequencingTestTypes, SequencingLibp2pImpl>,
    >,
>;

impl NodeImplementation<SequencingTestTypes> for SequencingLibp2pImpl {
    type Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
    type Leaf = SequencingLeaf<SequencingTestTypes>;
    type Exchanges = SequencingLibp2pExchange;
    type ConsensusMessage = SequencingMessage<SequencingTestTypes, Self>;

    fn new_channel_maps(
        start_view: <SequencingTestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<SequencingTestTypes, Self>,
        Option<ChannelMaps<SequencingTestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl
    TestableExchange<
        SequencingTestTypes,
        <SequencingLibp2pImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        Message<SequencingTestTypes, SequencingLibp2pImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<Libp2pNetwork<
            Message<SequencingTestTypes, SequencingLibp2pImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingLibp2pImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingLibp2pImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

pub type SequencingMemoryExchange = SequencingExchanges<
    SequencingTestTypes,
    Message<SequencingTestTypes, SequencingMemoryImpl>,
    QuorumExchange<
        SequencingTestTypes,
        <SequencingMemoryImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
        StaticMembership,
        StaticMemoryQuorumComm,
        Message<SequencingTestTypes, SequencingMemoryImpl>,
    >,
    CommitteeExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticMemoryDAComm,
        Message<SequencingTestTypes, SequencingMemoryImpl>,
    >,
    ViewSyncExchange<
        SequencingTestTypes,
        ViewSyncCertificate<SequencingTestTypes>,
        StaticMembership,
        StaticMemoryViewSyncComm,
        Message<SequencingTestTypes, SequencingMemoryImpl>,
    >,
    VIDExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticMemoryVIDComm,
        Message<SequencingTestTypes, SequencingMemoryImpl>,
    >,
>;

impl
    TestableExchange<
        SequencingTestTypes,
        <SequencingMemoryImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        Message<SequencingTestTypes, SequencingMemoryImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<MemoryNetwork<
            Message<SequencingTestTypes, SequencingMemoryImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingMemoryImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<MemoryNetwork<
            Message<SequencingTestTypes, SequencingMemoryImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingMemoryImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingMemoryImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<SequencingTestTypes> for SequencingMemoryImpl {
    type Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
    type Leaf = SequencingLeaf<SequencingTestTypes>;
    type Exchanges = SequencingMemoryExchange;
    type ConsensusMessage = SequencingMessage<SequencingTestTypes, Self>;

    fn new_channel_maps(
        start_view: <SequencingTestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<SequencingTestTypes, Self>,
        Option<ChannelMaps<SequencingTestTypes, Self>>,
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

pub type SequencingWebExchanges = SequencingExchanges<
    SequencingTestTypes,
    Message<SequencingTestTypes, SequencingWebImpl>,
    QuorumExchange<
        SequencingTestTypes,
        <SequencingWebImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
        StaticMembership,
        StaticWebQuorumComm,
        Message<SequencingTestTypes, SequencingWebImpl>,
    >,
    CommitteeExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticWebDAComm,
        Message<SequencingTestTypes, SequencingWebImpl>,
    >,
    ViewSyncExchange<
        SequencingTestTypes,
        ViewSyncCertificate<SequencingTestTypes>,
        StaticMembership,
        StaticWebViewSyncComm,
        Message<SequencingTestTypes, SequencingWebImpl>,
    >,
    VIDExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticWebVIDComm,
        Message<SequencingTestTypes, SequencingWebImpl>,
    >,
>;

impl
    TestableExchange<
        SequencingTestTypes,
        <SequencingWebImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        Message<SequencingTestTypes, SequencingWebImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let network_generator = Arc::new(<WebServerNetwork<
            Message<SequencingTestTypes, SequencingWebImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingWebImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));
        let network_da_generator = Arc::new(<WebServerNetwork<
            Message<SequencingTestTypes, SequencingWebImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
            SequencingTestTypes,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingWebImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);
            let vid_chan =
                <<Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingWebImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);

            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}

impl NodeImplementation<SequencingTestTypes> for SequencingWebImpl {
    type Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
    type Leaf = SequencingLeaf<SequencingTestTypes>;
    type Exchanges = SequencingWebExchanges;
    type ConsensusMessage = SequencingMessage<SequencingTestTypes, Self>;

    fn new_channel_maps(
        start_view: <SequencingTestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<SequencingTestTypes, Self>,
        Option<ChannelMaps<SequencingTestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

pub type SequencingCombinedExchange = SequencingExchanges<
    SequencingTestTypes,
    Message<SequencingTestTypes, SequencingCombinedImpl>,
    QuorumExchange<
        SequencingTestTypes,
        <SequencingCombinedImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
        StaticMembership,
        StaticCombinedQuorumComm,
        Message<SequencingTestTypes, SequencingCombinedImpl>,
    >,
    CommitteeExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticCombinedDAComm,
        Message<SequencingTestTypes, SequencingCombinedImpl>,
    >,
    ViewSyncExchange<
        SequencingTestTypes,
        ViewSyncCertificate<SequencingTestTypes>,
        StaticMembership,
        StaticCombinedViewSyncComm,
        Message<SequencingTestTypes, SequencingCombinedImpl>,
    >,
    VIDExchange<
        SequencingTestTypes,
        StaticMembership,
        StaticCombinedVIDComm,
        Message<SequencingTestTypes, SequencingCombinedImpl>,
    >,
>;

impl NodeImplementation<SequencingTestTypes> for SequencingCombinedImpl {
    type Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
    type Leaf = SequencingLeaf<SequencingTestTypes>;
    type Exchanges = SequencingCombinedExchange;
    type ConsensusMessage = SequencingMessage<SequencingTestTypes, Self>;

    fn new_channel_maps(
        start_view: <SequencingTestTypes as NodeType>::Time,
    ) -> (
        ChannelMaps<SequencingTestTypes, Self>,
        Option<ChannelMaps<SequencingTestTypes, Self>>,
    ) {
        (
            ChannelMaps::new(start_view),
            Some(ChannelMaps::new(start_view)),
        )
    }
}

impl
    TestableExchange<
        SequencingTestTypes,
        <SequencingCombinedImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
        Message<SequencingTestTypes, SequencingCombinedImpl>,
    > for SequencingCombinedExchange
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking,
                <Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking,
                <Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking,
                <Self::VIDExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking,
            ) + 'static,
    > {
        let web_server_network_generator = Arc::new(<WebServerNetwork<
            Message<SequencingTestTypes, SequencingCombinedImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
            _,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingCombinedImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            0,
            da_committee_size,
            false,
        ));

        let web_server_network_da_generator = Arc::new(<WebServerNetwork<
            Message<SequencingTestTypes, SequencingCombinedImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
            SequencingTestTypes,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingCombinedImpl>,
        >>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            true,
        ));

        let libp2p_network_generator = Arc::new(<Libp2pNetwork<
            Message<SequencingTestTypes, SequencingCombinedImpl>,
            <SequencingTestTypes as NodeType>::SignatureKey,
        > as TestableNetworkingImplementation<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingCombinedImpl>,
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
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network.clone());
            let committee_chan =
                <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da.clone());
            let view_sync_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network);

            let vid_chan =
                <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<
                    SequencingTestTypes,
                    Message<SequencingTestTypes, SequencingCombinedImpl>,
                >>::Networking as TestableChannelImplementation<_, _, _, _>>::generate_network(
                )(network_da);
            (quorum_chan, committee_chan, view_sync_chan, vid_chan)
        })
    }
}
