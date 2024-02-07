//! Composite trait for node behavior
//!
//! This module defines the [`NodeImplementation`] trait, which is a composite trait used for
//! describing the overall behavior of a node, as a composition of implementations of the node trait.

use super::{
    block_contents::{BlockHeader, TestableBlock, Transaction},
    election::ElectionConfig,
    network::{CommunicationChannel, NetworkReliability, TestableNetworkingImplementation},
    states::TestableState,
    storage::{StorageError, StorageState, TestableStorage},
    ValidatedState,
};
use crate::{
    data::{Leaf, TestableLeaf},
    traits::{
        election::Membership, network::TestableChannelImplementation, signature_key::SignatureKey,
        states::InstanceState, storage::Storage, BlockPayload,
    },
};
use async_trait::async_trait;
use commit::Committable;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    hash::Hash,
    ops,
    ops::{Deref, Sub},
    sync::Arc,
};

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
    ) -> Box<dyn Fn(u64) -> (Self::QuorumNetwork, Self::QuorumNetwork)>;
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
    ) -> Box<dyn Fn(u64) -> (Self::QuorumNetwork, Self::QuorumNetwork)> {
        let quorum_generator = <<I::QuorumNetwork as CommunicationChannel<TYPES>>::NETWORK as TestableNetworkingImplementation<TYPES>>::generator(
                expected_node_count,
                num_bootstrap,
                0,
                da_committee_size,
                false,
                reliability_config.clone(),
            );
        let da_generator = <<I::QuorumNetwork as CommunicationChannel<TYPES>>::NETWORK as TestableNetworkingImplementation<TYPES>>::generator(
            expected_node_count,
            num_bootstrap,
            1,
            da_committee_size,
            false,
            reliability_config,
        );

        Box::new(move |id| {
            let quorum = Arc::new(quorum_generator(id));
            let da = Arc::new(da_generator(id));
            let quorum_chan =
                <I::QuorumNetwork as TestableChannelImplementation<_>>::generate_network()(quorum);
            let committee_chan =
                <I::QuorumNetwork as TestableChannelImplementation<_>>::generate_network()(da);
            (quorum_chan, committee_chan)
        })
    }
}

/// Trait for time compatibility needed for reward collection
pub trait ConsensusTime:
    PartialOrd
    + Ord
    + Send
    + Sync
    + Debug
    + Clone
    + Copy
    + Hash
    + Deref<Target = u64>
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + ops::AddAssign<u64>
    + ops::Add<u64, Output = Self>
    + Sub<u64, Output = Self>
    + 'static
    + Committable
{
    /// Create a new instance of this time unit at time number 0
    #[must_use]
    fn genesis() -> Self {
        Self::new(0)
    }
    /// Create a new instance of this time unit
    fn new(val: u64) -> Self;
    /// Get the u64 format of time
    fn get_u64(&self) -> u64;
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
    type BlockHeader: BlockHeader<Payload = Self::BlockPayload, State = Self::ValidatedState>;
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
