//! Composite trait for node behavior
//!
//! This module defines the [`NodeImplementation`] trait, which is a composite trait used for
//! describing the overall behavior of a node, as a composition of implementations of the node trait.

use super::{
    block_contents::Transaction,
    election::{
        CommitteeExchangeType, ConsensusExchange, ElectionConfig, QuorumExchangeType,
        TimeoutExchange, TimeoutExchangeType, ViewSyncExchangeType, VoteToken,
    },
    network::{CommunicationChannel, NetworkMsg, TestableNetworkingImplementation},
    state::{ConsensusTime, TestableBlock, TestableState},
    storage::{StorageError, StorageState, TestableStorage},
    State,
};
use crate::{
    data::{LeafType, SequencingLeaf, TestableLeaf},
    message::{ConsensusMessageType, Message, SequencingMessage},
    traits::{
        election::Membership, network::TestableChannelImplementation, signature_key::SignatureKey,
        storage::Storage, BlockPayload,
    },
};
use async_compatibility_layer::channel::{unbounded, UnboundedReceiver, UnboundedSender};
use async_lock::{Mutex, RwLock};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    sync::{atomic::AtomicBool, Arc},
};
/// Alias for the [`ProcessedConsensusMessage`] type of a [`NodeImplementation`].
type ProcessedConsensusMessageType<TYPES, I> = <<I as NodeImplementation<TYPES>>::ConsensusMessage as ConsensusMessageType<TYPES, I>>::ProcessedConsensusMessage;

/// struct containing messages for a view to send to a replica or DA committee member.
#[derive(Clone)]
pub struct ViewQueue<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// to send networking events to a replica or DA committee member.
    pub sender_chan: UnboundedSender<ProcessedConsensusMessageType<TYPES, I>>,

    /// to recv networking events for a replica or DA committee member.
    pub receiver_chan: Arc<Mutex<UnboundedReceiver<ProcessedConsensusMessageType<TYPES, I>>>>,

    /// `true` if this queue has already received a proposal
    pub has_received_proposal: Arc<AtomicBool>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> Default for ViewQueue<TYPES, I> {
    /// create new view queue
    fn default() -> Self {
        let (s, r) = unbounded();
        ViewQueue {
            sender_chan: s,
            receiver_chan: Arc::new(Mutex::new(r)),
            has_received_proposal: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// metadata for sending information to the leader, replica, or DA committee member.
pub struct SendToTasks<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// the current view number
    /// this should always be in sync with `Consensus`
    pub cur_view: TYPES::Time,

    /// a map from view number to ViewQueue
    /// one of (replica|next leader)'s' task for view i will be listening on the channel in here
    pub channel_map: BTreeMap<TYPES::Time, ViewQueue<TYPES, I>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SendToTasks<TYPES, I> {
    /// create new sendtosasks
    #[must_use]
    pub fn new(view_num: TYPES::Time) -> Self {
        SendToTasks {
            cur_view: view_num,
            channel_map: BTreeMap::default(),
        }
    }
}

/// Channels for sending/recv-ing proposals and votes.
#[derive(Clone)]
pub struct ChannelMaps<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Channel for the next consensus leader or DA leader.
    pub proposal_channel: Arc<RwLock<SendToTasks<TYPES, I>>>,

    /// Channel for the replica or DA committee member.
    pub vote_channel: Arc<RwLock<SendToTasks<TYPES, I>>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ChannelMaps<TYPES, I> {
    /// Create channels starting from a given view.
    pub fn new(start_view: TYPES::Time) -> Self {
        Self {
            proposal_channel: Arc::new(RwLock::new(SendToTasks::new(start_view))),
            vote_channel: Arc::new(RwLock::new(SendToTasks::new(start_view))),
        }
    }
}

/// Node implementation aggregate trait
///
/// This trait exists to collect multiple behavior implementations into one type, to allow
/// `HotShot` to avoid annoying numbers of type arguments and type patching.
///
/// It is recommended you implement this trait on a zero sized type, as `HotShot`does not actually
/// store or keep a reference to any value implementing this trait.

pub trait NodeImplementation<TYPES: NodeType>:
    Send + Sync + Debug + Clone + Eq + Hash + 'static + Serialize + for<'de> Deserialize<'de>
{
    /// Leaf type for this consensus implementation
    type Leaf: LeafType<NodeType = TYPES>;

    /// Storage type for this consensus implementation
    type Storage: Storage<TYPES, Self::Leaf> + Clone;

    /// Consensus message type.
    type ConsensusMessage: ConsensusMessageType<TYPES, Self>
        + Clone
        + Debug
        + Send
        + Sync
        + 'static
        + for<'a> Deserialize<'a>
        + Serialize;

    /// Consensus type selected exchanges.
    ///
    /// Implements either `ValidatingExchangesType` or `SequencingExchangesType`.
    type Exchanges: ExchangesType<TYPES, Self::Leaf, Message<TYPES, Self>>;

    /// Create channels for sending/recv-ing proposals and votes for quorum and committee
    /// exchanges, the latter of which is only applicable for sequencing consensus.
    fn new_channel_maps(
        start_view: TYPES::Time,
    ) -> (ChannelMaps<TYPES, Self>, Option<ChannelMaps<TYPES, Self>>);
}

/// Contains the protocols for exchanging proposals and votes.
#[allow(clippy::type_complexity)]
#[async_trait]
pub trait ExchangesType<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>, MESSAGE: NetworkMsg>:
    Send + Sync
{
    /// Protocol for exchanging data availability proposals and votes.
    type CommitteeExchange: CommitteeExchangeType<TYPES, MESSAGE> + Clone + Debug;

    /// Get the committee exchange
    fn committee_exchange(&self) -> &Self::CommitteeExchange;

    /// Get the timeout exchange
    fn timeout_exchange(&self) -> &Self::TimeoutExchange;

    /// Protocol for exchanging quorum proposals and votes.
    type QuorumExchange: QuorumExchangeType<TYPES, LEAF, MESSAGE> + Clone + Debug;

    /// Protocol for exchanging view sync proposals and votes.
    type ViewSyncExchange: ViewSyncExchangeType<TYPES, MESSAGE> + Clone + Debug;

    /// Protocol for receiving timeout votes
    type TimeoutExchange: TimeoutExchangeType<TYPES, MESSAGE> + Clone + Debug;

    /// Election configurations for exchanges
    type ElectionConfigs;

    /// Create all exchanges.
    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        configs: Self::ElectionConfigs,
        networks: (
            <Self::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
            <Self::CommitteeExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
            <Self::ViewSyncExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
        ),
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self;

    /// Get the quorum exchange.
    fn quorum_exchange(&self) -> &Self::QuorumExchange;

    /// Get the view sync exchange.
    fn view_sync_exchange(&self) -> &Self::ViewSyncExchange;

    /// BlockPayload the underlying networking interfaces until node is successfully initialized into the
    /// networks.
    async fn wait_for_networks_ready(&self);

    /// Shut down the the underlying networking interfaces.
    async fn shut_down_networks(&self);
}

/// an exchange that is testable
pub trait TestableExchange<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>, MESSAGE: NetworkMsg>:
    ExchangesType<TYPES, LEAF, MESSAGE>
{
    /// generate communication channels
    #[allow(clippy::type_complexity)]
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
    ) -> Box<
        dyn Fn(
                u64,
            ) -> (
                <Self::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
                <Self::CommitteeExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
                <Self::ViewSyncExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
            ) + 'static,
    >;
}

/// Implementes [`SequencingExchangesType`].
#[derive(Clone, Debug)]
pub struct SequencingExchanges<
    TYPES: NodeType,
    MESSAGE: NetworkMsg,
    QUORUMEXCHANGE: QuorumExchangeType<TYPES, SequencingLeaf<TYPES>, MESSAGE> + Clone + Debug,
    COMMITTEEEXCHANGE: CommitteeExchangeType<TYPES, MESSAGE> + Clone + Debug,
    VIEWSYNCEXCHANGE: ViewSyncExchangeType<TYPES, MESSAGE> + Clone + Debug,
> {
    /// Quorum exchange.
    quorum_exchange: QUORUMEXCHANGE,

    /// View sync exchange.
    view_sync_exchange: VIEWSYNCEXCHANGE,

    /// Committee exchange.
    committee_exchange: COMMITTEEEXCHANGE,

    /// Timeout exchange
    // This type can be simplified once we rework the exchanges trait
    // It is here to avoid needing to instantiate it where all the other exchanges are instantiated
    // https://github.com/EspressoSystems/HotShot/issues/1799
    #[allow(clippy::type_complexity)]

    pub timeout_exchange: TimeoutExchange<TYPES, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Proposal, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Membership, <QUORUMEXCHANGE as ConsensusExchange<TYPES, MESSAGE>>::Networking, MESSAGE>,

    /// Phantom data
    _phantom: PhantomData<(TYPES, MESSAGE)>,
}

#[async_trait]
impl<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE>
    ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>
    for SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE>
where
    TYPES: NodeType,
    MESSAGE: NetworkMsg,
    QUORUMEXCHANGE: QuorumExchangeType<TYPES, SequencingLeaf<TYPES>, MESSAGE> + Clone + Debug,
    COMMITTEEEXCHANGE: CommitteeExchangeType<TYPES, MESSAGE> + Clone + Debug,
    VIEWSYNCEXCHANGE: ViewSyncExchangeType<TYPES, MESSAGE> + Clone + Debug,
{
    type CommitteeExchange = COMMITTEEEXCHANGE;
    type QuorumExchange = QUORUMEXCHANGE;
    type ViewSyncExchange = VIEWSYNCEXCHANGE;
    #[allow(clippy::type_complexity)]
    type TimeoutExchange = TimeoutExchange<TYPES, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Proposal, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Membership, <QUORUMEXCHANGE as ConsensusExchange<TYPES, MESSAGE>>::Networking, MESSAGE>;
    type ElectionConfigs = (TYPES::ElectionConfigType, TYPES::ElectionConfigType);

    fn committee_exchange(&self) -> &COMMITTEEEXCHANGE {
        &self.committee_exchange
    }

    fn timeout_exchange(&self) -> &Self::TimeoutExchange {
        &self.timeout_exchange
    }

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        configs: Self::ElectionConfigs,
        networks: (
            <Self::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
            <Self::CommitteeExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
            <Self::ViewSyncExchange as ConsensusExchange<TYPES, MESSAGE>>::Networking,
        ),
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let quorum_exchange = QUORUMEXCHANGE::create(
            entries.clone(),
            configs.0.clone(),
            networks.0.clone(),
            pk.clone(),
            entry.clone(),
            sk.clone(),
        );
        #[allow(clippy::type_complexity)]
        let timeout_exchange: TimeoutExchange<TYPES, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Proposal, <<SequencingExchanges<TYPES, MESSAGE, QUORUMEXCHANGE, COMMITTEEEXCHANGE, VIEWSYNCEXCHANGE> as ExchangesType<TYPES, SequencingLeaf<TYPES>, MESSAGE>>::QuorumExchange as ConsensusExchange<TYPES, MESSAGE>>::Membership, <QUORUMEXCHANGE as ConsensusExchange<TYPES, MESSAGE>>::Networking, MESSAGE> = TimeoutExchange::create(
            entries.clone(),
            configs.0.clone(),
            networks.0,
            pk.clone(),
            entry.clone(),
            sk.clone(),
        );

        let view_sync_exchange = VIEWSYNCEXCHANGE::create(
            entries.clone(),
            configs.0,
            networks.2,
            pk.clone(),
            entry.clone(),
            sk.clone(),
        );
        let committee_exchange =
            COMMITTEEEXCHANGE::create(entries, configs.1, networks.1, pk, entry, sk);

        Self {
            quorum_exchange,
            committee_exchange,
            view_sync_exchange,
            timeout_exchange,
            _phantom: PhantomData,
        }
    }

    fn quorum_exchange(&self) -> &Self::QuorumExchange {
        &self.quorum_exchange
    }

    fn view_sync_exchange(&self) -> &Self::ViewSyncExchange {
        &self.view_sync_exchange
    }

    async fn wait_for_networks_ready(&self) {
        self.quorum_exchange.network().wait_for_ready().await;
        self.committee_exchange.network().wait_for_ready().await;
    }

    async fn shut_down_networks(&self) {
        self.quorum_exchange.network().shut_down().await;
        self.committee_exchange.network().shut_down().await;
    }
}

/// Alias for the [`QuorumExchange`] type.
pub type QuorumEx<TYPES, I> = <<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
    TYPES,
    <I as NodeImplementation<TYPES>>::Leaf,
    Message<TYPES, I>,
>>::QuorumExchange;

/// Alias for the [`CommitteeExchange`] type for sequencing consensus.
pub type SequencingQuorumEx<TYPES, I> =
    <<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        <I as NodeImplementation<TYPES>>::Leaf,
        Message<TYPES, I>,
    >>::QuorumExchange;

/// Alias for `TimeoutExchange` type
pub type SequencingTimeoutEx<TYPES, I> =
    <<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        <I as NodeImplementation<TYPES>>::Leaf,
        Message<TYPES, I>,
    >>::TimeoutExchange;

/// Alias for the [`CommitteeExchange`] type.
pub type CommitteeEx<TYPES, I> = <<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
    TYPES,
    <I as NodeImplementation<TYPES>>::Leaf,
    Message<TYPES, I>,
>>::CommitteeExchange;

/// Alias for the [`ViewSyncExchange`] type.
pub type ViewSyncEx<TYPES, I> = <<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
    TYPES,
    <I as NodeImplementation<TYPES>>::Leaf,
    Message<TYPES, I>,
>>::ViewSyncExchange;

/// extra functions required on a node implementation to be usable by hotshot-testing
#[allow(clippy::type_complexity)]
#[async_trait]
pub trait TestableNodeImplementation<TYPES: NodeType>: NodeImplementation<TYPES> {
    /// Election config for the DA committee
    type CommitteeElectionConfig;

    /// Generates a committee-specific election
    fn committee_election_config_generator(
    ) -> Box<dyn Fn(u64) -> Self::CommitteeElectionConfig + 'static>;

    /// Creates random transaction if possible
    /// otherwise panics
    /// `padding` is the bytes of padding to add to the transaction
    fn state_create_random_transaction(
        state: Option<&TYPES::StateType>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockType as BlockPayload>::Transaction;

    /// Creates random transaction if possible
    /// otherwise panics
    /// `padding` is the bytes of padding to add to the transaction
    fn leaf_create_random_transaction(
        leaf: &Self::Leaf,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockType as BlockPayload>::Transaction;

    /// generate a genesis block
    fn block_genesis() -> TYPES::BlockType;

    /// the number of transactions in a block
    fn txn_count(block: &TYPES::BlockType) -> u64;

    /// Create ephemeral storage
    /// Will be deleted/lost immediately after storage is dropped
    /// # Errors
    /// Errors if it is not possible to construct temporary storage.
    fn construct_tmp_storage() -> Result<Self::Storage, StorageError>;

    /// Return the full internal state. This is useful for debugging.
    async fn get_full_state(storage: &Self::Storage) -> StorageState<TYPES, Self::Leaf>;
}

#[async_trait]
impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, Self>>,
    > TestableNodeImplementation<TYPES> for I
where
    CommitteeNetwork<TYPES, I>: TestableNetworkingImplementation<TYPES, Message<TYPES, I>>,
    QuorumNetwork<TYPES, I>: TestableNetworkingImplementation<TYPES, Message<TYPES, I>>,
    QuorumCommChannel<TYPES, I>: TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        QuorumMembership<TYPES, I>,
        QuorumNetwork<TYPES, I>,
    >,
    CommitteeCommChannel<TYPES, I>: TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        CommitteeMembership<TYPES, I>,
        QuorumNetwork<TYPES, I>,
    >,
    ViewSyncCommChannel<TYPES, I>: TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        ViewSyncMembership<TYPES, I>,
        QuorumNetwork<TYPES, I>,
    >,
    TYPES::StateType: TestableState,
    TYPES::BlockType: TestableBlock,
    I::Storage: TestableStorage<TYPES, I::Leaf>,
    I::Leaf: TestableLeaf<NodeType = TYPES>,
{
    type CommitteeElectionConfig = TYPES::ElectionConfigType;

    fn committee_election_config_generator(
    ) -> Box<dyn Fn(u64) -> Self::CommitteeElectionConfig + 'static> {
        Box::new(|num_nodes| <CommitteeMembership<TYPES, I>>::default_election_config(num_nodes))
    }

    fn state_create_random_transaction(
        state: Option<&TYPES::StateType>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockType as BlockPayload>::Transaction {
        <TYPES::StateType as TestableState>::create_random_transaction(state, rng, padding)
    }

    fn leaf_create_random_transaction(
        leaf: &Self::Leaf,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockType as BlockPayload>::Transaction {
        <Self::Leaf as TestableLeaf>::create_random_transaction(leaf, rng, padding)
    }

    fn block_genesis() -> TYPES::BlockType {
        <TYPES::BlockType as TestableBlock>::genesis()
    }

    fn txn_count(block: &TYPES::BlockType) -> u64 {
        <TYPES::BlockType as TestableBlock>::txn_count(block)
    }

    fn construct_tmp_storage() -> Result<Self::Storage, StorageError> {
        <I::Storage as TestableStorage<TYPES, I::Leaf>>::construct_tmp_storage()
    }

    async fn get_full_state(storage: &Self::Storage) -> StorageState<TYPES, Self::Leaf> {
        <I::Storage as TestableStorage<TYPES, I::Leaf>>::get_full_state(storage).await
    }
}

/// A proposal to append a new leaf to the log which is output by consensus.
pub type QuorumProposalType<TYPES, I> =
    <QuorumEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Proposal;

/// A proposal to provide data availability for a new leaf.
pub type CommitteeProposalType<TYPES, I> =
    <CommitteeEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Proposal;

/// A proposal to sync the view.
pub type ViewSyncProposalType<TYPES, I> =
    <ViewSyncEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Proposal;

/// A vote on a [`QuorumProposalType`].
pub type QuorumVoteType<TYPES, I> =
    <QuorumEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Vote;

/// A vote on a [`ComitteeProposal`].
pub type CommitteeVote<TYPES, I> =
    <CommitteeEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Vote;

/// A vote on a [`ViewSyncProposal`].
pub type ViewSyncVoteType<TYPES, I> =
    <ViewSyncEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Vote;

/// Communication channel for [`QuorumProposalType`] and [`QuorumVote`].
pub type QuorumCommChannel<TYPES, I> =
    <QuorumEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking;

/// Communication channel for [`ViewSyncProposalType`] and [`ViewSyncVote`].
pub type ViewSyncCommChannel<TYPES, I> =
    <ViewSyncEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking;

/// Communication channel for [`CommitteeProposalType`] and [`DAVote`].
pub type CommitteeCommChannel<TYPES, I> =
    <CommitteeEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking;

/// Protocol for determining membership in a consensus committee.
pub type QuorumMembership<TYPES, I> =
    <QuorumEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Membership;

/// Protocol for determining membership in a DA committee.
pub type CommitteeMembership<TYPES, I> =
    <CommitteeEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Membership;

/// Protocol for determining membership in a view sync committee.
pub type ViewSyncMembership<TYPES, I> = QuorumMembership<TYPES, I>;

/// Type for the underlying quorum `ConnectedNetwork` that will be shared (for now) b/t Communication Channels
pub type QuorumNetwork<TYPES, I> = <QuorumCommChannel<TYPES, I> as CommunicationChannel<
    TYPES,
    Message<TYPES, I>,
    QuorumMembership<TYPES, I>,
>>::NETWORK;

/// Type for the underlying committee `ConnectedNetwork` that will be shared (for now) b/t Communication Channels
pub type CommitteeNetwork<TYPES, I> = <CommitteeCommChannel<TYPES, I> as CommunicationChannel<
    TYPES,
    Message<TYPES, I>,
    CommitteeMembership<TYPES, I>,
>>::NETWORK;

/// Type for the underlying view sync `ConnectedNetwork` that will be shared (for now) b/t Communication Channels
pub type ViewSyncNetwork<TYPES, I> = <ViewSyncCommChannel<TYPES, I> as CommunicationChannel<
    TYPES,
    Message<TYPES, I>,
    ViewSyncMembership<TYPES, I>,
>>::NETWORK;

/// Trait with all the type definitions that are used in the current hotshot setup.
pub trait NodeType:
    Clone
    + Copy
    + Debug
    + Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Default
    + serde::Serialize
    + for<'de> Deserialize<'de>
    + Send
    + Sync
    + 'static
{
    /// The time type that this hotshot setup is using.
    ///
    /// This should be the same `Time` that `StateType::Time` is using.
    type Time: ConsensusTime;
    /// The block type that this hotshot setup is using.
    ///
    /// This should be the same block that `StateType::BlockType` is using.
    type BlockType: BlockPayload<Transaction = Self::Transaction>;
    /// The signature key that this hotshot setup is using.
    type SignatureKey: SignatureKey;
    /// The vote token that this hotshot setup is using.
    type VoteTokenType: VoteToken;
    /// The transaction type that this hotshot setup is using.
    ///
    /// This should be equal to `BlockPayload::Transaction`
    type Transaction: Transaction;
    /// The election config type that this hotshot setup is using.
    type ElectionConfigType: ElectionConfig;

    /// The state type that this hotshot setup is using.
    type StateType: State<BlockType = Self::BlockType, Time = Self::Time>;
}
