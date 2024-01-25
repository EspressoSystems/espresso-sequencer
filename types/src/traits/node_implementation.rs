//! Composite trait for node behavior
//!
//! This module defines the [`NodeImplementation`] trait, which is a composite trait used for
//! describing the overall behavior of a node, as a composition of implementations of the node trait.

use super::{
    block_contents::{BlockHeader, TestableBlock, Transaction},
    election::ElectionConfig,
    network::{CommunicationChannel, NetworkReliability, TestableNetworkingImplementation},
    states::{ConsensusTime, TestableState},
    storage::{StorageError, StorageState, TestableStorage},
    ValidatedState,
};
use crate::{
    data::{Leaf, TestableLeaf},
    message::ProcessedSequencingMessage,
    traits::{
        election::Membership, network::TestableChannelImplementation, signature_key::SignatureKey,
        states::InstanceState, storage::Storage, BlockPayload,
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
    sync::{atomic::AtomicBool, Arc},
};

/// struct containing messages for a view to send to a replica or DA committee member.
#[derive(Clone)]
pub struct ViewQueue<TYPES: NodeType> {
    /// to send networking events to a replica or DA committee member.
    pub sender_chan: UnboundedSender<ProcessedSequencingMessage<TYPES>>,

    /// to recv networking events for a replica or DA committee member.
    pub receiver_chan: Arc<Mutex<UnboundedReceiver<ProcessedSequencingMessage<TYPES>>>>,

    /// `true` if this queue has already received a proposal
    pub has_received_proposal: Arc<AtomicBool>,
}

impl<TYPES: NodeType> Default for ViewQueue<TYPES> {
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
pub struct SendToTasks<TYPES: NodeType> {
    /// the current view number
    /// this should always be in sync with `Consensus`
    pub cur_view: TYPES::Time,

    /// a map from view number to ViewQueue
    /// one of (replica|next leader)'s' task for view i will be listening on the channel in here
    pub channel_map: BTreeMap<TYPES::Time, ViewQueue<TYPES>>,
}

impl<TYPES: NodeType> SendToTasks<TYPES> {
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
pub struct ChannelMaps<TYPES: NodeType> {
    /// Channel for the next consensus leader or DA leader.
    pub proposal_channel: Arc<RwLock<SendToTasks<TYPES>>>,

    /// Channel for the replica or DA committee member.
    pub vote_channel: Arc<RwLock<SendToTasks<TYPES>>>,
}

impl<TYPES: NodeType> ChannelMaps<TYPES> {
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
    /// Storage type for this consensus implementation
    type Storage: Storage<TYPES> + Clone;

    /// Network for all nodes
    type QuorumNetwork: CommunicationChannel<TYPES>;
    /// Network for those in the DA committee
    type CommitteeNetwork: CommunicationChannel<TYPES>;

    /// Create channels for sending/recv-ing proposals and votes for quorum and committee
    /// exchanges, the latter of which is only applicable for sequencing consensus.
    fn new_channel_maps(
        start_view: TYPES::Time,
    ) -> (ChannelMaps<TYPES>, Option<ChannelMaps<TYPES>>);
}

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
        state: Option<&TYPES::ValidatedState>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockPayload as BlockPayload>::Transaction;

    /// Creates random transaction if possible
    /// otherwise panics
    /// `padding` is the bytes of padding to add to the transaction
    fn leaf_create_random_transaction(
        leaf: &Leaf<TYPES>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockPayload as BlockPayload>::Transaction;

    /// generate a genesis block
    fn block_genesis() -> TYPES::BlockPayload;

    /// the number of transactions in a block
    fn txn_count(block: &TYPES::BlockPayload) -> u64;

    /// Create ephemeral storage
    /// Will be deleted/lost immediately after storage is dropped
    /// # Errors
    /// Errors if it is not possible to construct temporary storage.
    fn construct_tmp_storage() -> Result<Self::Storage, StorageError>;

    /// Return the full internal state. This is useful for debugging.
    async fn get_full_state(storage: &Self::Storage) -> StorageState<TYPES>;

    /// Generate the communication channels for testing
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
        reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> (Self::QuorumNetwork, Self::CommitteeNetwork)>;
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TestableNodeImplementation<TYPES> for I
where
    TYPES::ValidatedState: TestableState,
    TYPES::BlockPayload: TestableBlock,
    I::Storage: TestableStorage<TYPES>,
    I::QuorumNetwork: TestableChannelImplementation<TYPES>,
    I::CommitteeNetwork: TestableChannelImplementation<TYPES>,
    <<I as NodeImplementation<TYPES>>::QuorumNetwork as CommunicationChannel<TYPES>>::NETWORK:
        TestableNetworkingImplementation<TYPES>,
    <<I as NodeImplementation<TYPES>>::CommitteeNetwork as CommunicationChannel<TYPES>>::NETWORK:
        TestableNetworkingImplementation<TYPES>,
{
    type CommitteeElectionConfig = TYPES::ElectionConfigType;

    fn committee_election_config_generator(
    ) -> Box<dyn Fn(u64) -> Self::CommitteeElectionConfig + 'static> {
        Box::new(|num_nodes| <TYPES as NodeType>::Membership::default_election_config(num_nodes))
    }

    fn state_create_random_transaction(
        state: Option<&TYPES::ValidatedState>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockPayload as BlockPayload>::Transaction {
        <TYPES::ValidatedState as TestableState>::create_random_transaction(state, rng, padding)
    }

    fn leaf_create_random_transaction(
        leaf: &Leaf<TYPES>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockPayload as BlockPayload>::Transaction {
        Leaf::create_random_transaction(leaf, rng, padding)
    }

    fn block_genesis() -> TYPES::BlockPayload {
        <TYPES::BlockPayload as TestableBlock>::genesis()
    }

    fn txn_count(block: &TYPES::BlockPayload) -> u64 {
        <TYPES::BlockPayload as TestableBlock>::txn_count(block)
    }

    fn construct_tmp_storage() -> Result<Self::Storage, StorageError> {
        <I::Storage as TestableStorage<TYPES>>::construct_tmp_storage()
    }

    async fn get_full_state(storage: &Self::Storage) -> StorageState<TYPES> {
        <I::Storage as TestableStorage<TYPES>>::get_full_state(storage).await
    }
    fn gen_comm_channels(
        expected_node_count: usize,
        num_bootstrap: usize,
        da_committee_size: usize,
        reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> (Self::QuorumNetwork, Self::CommitteeNetwork)> {
        let network_generator = <<I::QuorumNetwork as CommunicationChannel<TYPES>>::NETWORK as TestableNetworkingImplementation<TYPES>>::generator(
                expected_node_count,
                num_bootstrap,
                0,
                da_committee_size,
                false,
                reliability_config.clone(),
            );
        let da_generator = <<I::CommitteeNetwork as CommunicationChannel<TYPES>>::NETWORK as TestableNetworkingImplementation<TYPES>>::generator(
                expected_node_count,
                num_bootstrap,
                1,
                da_committee_size,
                true,
                reliability_config
            );

        Box::new(move |id| {
            let network = Arc::new(network_generator(id));
            let network_da = Arc::new(da_generator(id));
            let quorum_chan =
                <I::QuorumNetwork as TestableChannelImplementation<_>>::generate_network()(network);
            let committee_chan =
                <I::CommitteeNetwork as TestableChannelImplementation<_>>::generate_network()(
                    network_da,
                );
            (quorum_chan, committee_chan)
        })
    }
}

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
    /// This should be the same `Time` that `ValidatedState::Time` is using.
    type Time: ConsensusTime;
    /// The block header type that this hotshot setup is using.
    type BlockHeader: BlockHeader<Payload = Self::BlockPayload, State = Self::StateType>;
    /// The block type that this hotshot setup is using.
    ///
    /// This should be the same block that `ValidatedState::BlockPayload` is using.
    type BlockPayload: BlockPayload<Transaction = Self::Transaction>;
    /// The signature key that this hotshot setup is using.
    type SignatureKey: SignatureKey;
    /// The transaction type that this hotshot setup is using.
    ///
    /// This should be equal to `BlockPayload::Transaction`
    type Transaction: Transaction;
    /// The election config type that this hotshot setup is using.
    type ElectionConfigType: ElectionConfig;

    /// The instance-level state type that this hotshot setup is using.
    type InstanceState: InstanceState;

    /// The validated state type that this hotshot setup is using.
    type ValidatedState: ValidatedState<
        Instance = Self::InstanceState,
        BlockHeader = Self::BlockHeader,
        BlockPayload = Self::BlockPayload,
        Time = Self::Time,
    >;

    /// Membership used for this implementation
    type Membership: Membership<Self>;
}
