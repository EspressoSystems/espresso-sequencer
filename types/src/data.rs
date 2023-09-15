//! Provides types useful for representing `HotShot`'s data structures
//!
//! This module provides types for representing consensus internal state, such as leaves,
//! `HotShot`'s version of a block, and proposals, messages upon which to reach the consensus.

use crate::{
    certificate::{
        AssembledSignature, DACertificate, QuorumCertificate, TimeoutCertificate,
        ViewSyncCertificate,
    },
    traits::{
        election::SignedCertificate,
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, SignatureKey},
        state::{ConsensusTime, TestableBlock, TestableState},
        storage::StoredView,
        BlockPayload, State,
    },
};
use ark_bls12_381::Bls12_381;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};
use bincode::Options;
use commit::{Commitment, Committable};
use derivative::Derivative;
use either::Either;
use espresso_systems_common::hotshot::tag;
use hotshot_constants::GENESIS_PROPOSER_ID;
use hotshot_utils::bincode::bincode_opts;
use jf_primitives::pcs::{checked_fft_size, prelude::UnivariateKzgPCS, PolynomialCommitmentScheme};
use rand::Rng;
use serde::{Deserialize, Serialize};
use snafu::{ensure, Snafu};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Type-safe wrapper around `u64` so we know the thing we're talking about is a view number.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    // std::ops::Add,
    // std::ops::Div,
    // std::ops::Rem,
    Deserialize,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
pub struct ViewNumber(u64);

impl ConsensusTime for ViewNumber {
    /// Create a genesis view number (0)
    fn genesis() -> Self {
        Self(0)
    }
    /// Create a new `ViewNumber` with the given value.
    fn new(n: u64) -> Self {
        Self(n)
    }
}

impl Committable for ViewNumber {
    fn commit(&self) -> Commitment<Self> {
        let builder = commit::RawCommitmentBuilder::new("View Number Commitment");
        builder.u64(self.0).finalize()
    }
}

impl std::ops::Add<u64> for ViewNumber {
    type Output = ViewNumber;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::AddAssign<u64> for ViewNumber {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl std::ops::Deref for ViewNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Sub<u64> for ViewNumber {
    type Output = ViewNumber;
    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

/// Generate the genesis block proposer ID from the defined constant
#[must_use]
pub fn genesis_proposer_id() -> EncodedPublicKey {
    EncodedPublicKey(GENESIS_PROPOSER_ID.to_vec())
}

/// The `Transaction` type associated with a `State`, as a syntactic shortcut
pub type Transaction<STATE> = <<STATE as State>::BlockType as BlockPayload>::Transaction;
/// `Commitment` to the `Transaction` type associated with a `State`, as a syntactic shortcut
pub type TxnCommitment<STATE> = Commitment<Transaction<STATE>>;

/// subset of state that we stick into a leaf.
/// original hotstuff proposal
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Derivative, Eq)]
#[serde(bound(deserialize = ""))]
#[derivative(PartialEq, Hash)]
pub struct ValidatingProposal<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>>
where
    LEAF: Committable,
{
    ///  current view's block commitment
    pub block_commitment: Commitment<TYPES::BlockType>,

    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Height from leader when proposing leaf
    pub height: u64,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES, LEAF>,

    /// The hash of the parent `Leaf`
    /// So we can ask if it extends
    #[debug(skip)]
    pub parent_commitment: Commitment<LEAF>,

    /// BlockPayload leaf wants to apply
    pub deltas: TYPES::BlockType,

    /// What the state should be after applying `self.deltas`
    pub state_commitment: Commitment<TYPES::StateType>,

    /// Transactions that were marked for rejection while collecting deltas
    pub rejected: Vec<<TYPES::BlockType as BlockPayload>::Transaction>,

    /// the propser id
    pub proposer_id: EncodedPublicKey,
}

/// A proposal to start providing data availability for a block.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DAProposal<TYPES: NodeType> {
    /// BlockPayload leaf wants to apply
    pub deltas: TYPES::BlockType,
    /// View this proposal applies to
    pub view_number: TYPES::Time,
}

/// The VID scheme type used in `HotShot`.
pub type VidScheme = jf_primitives::vid::advz::Advz<ark_bls12_381::Bls12_381, sha2::Sha256>;
pub use jf_primitives::vid::VidScheme as VidSchemeTrait;

/// VID dispersal data
///
/// Like [`DAProposal`].
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VidDisperse<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::Time,
    /// Block commitment
    pub commitment: Commitment<TYPES::BlockType>,
    /// VID shares dispersed among storage nodes
    pub shares: Vec<<VidScheme as VidSchemeTrait>::StorageShare>,
    /// VID common data sent to all storage nodes
    pub common: <VidScheme as VidSchemeTrait>::StorageCommon,
}

/// Trusted KZG setup for VID.
///
/// TESTING ONLY: don't use this in production
/// TODO <https://github.com/EspressoSystems/HotShot/issues/1686>
///
/// # Panics
/// ...because this is only for tests. This comment exists to pacify clippy.
#[must_use]
pub fn test_srs(
    num_storage_nodes: usize,
) -> <UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::SRS {
    let mut rng = jf_utils::test_rng();
    UnivariateKzgPCS::<ark_bls12_381::Bls12_381>::gen_srs_for_testing(
        &mut rng,
        checked_fft_size(num_storage_nodes).unwrap(),
    )
    .unwrap()
}

/// Proposal to append a block.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub struct QuorumProposal<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The commitment to append.
    pub block_commitment: Commitment<TYPES::BlockType>,

    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Height from leader when proposing leaf
    pub height: u64,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES, LEAF>,

    /// Possible timeout certificate.  Only present if the justify_qc is not for the preceding view
    pub timeout_certificate: Option<TimeoutCertificate<TYPES>>,

    /// the propser id
    pub proposer_id: EncodedPublicKey,

    /// Data availibity certificate
    // TODO We should be able to remove this
    pub dac: Option<DACertificate<TYPES>>,
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> ProposalType
    for ValidatingProposal<TYPES, LEAF>
{
    type NodeType = TYPES;
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> ProposalType for DAProposal<TYPES> {
    type NodeType = TYPES;
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> ProposalType for VidDisperse<TYPES> {
    type NodeType = TYPES;
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> ProposalType
    for QuorumProposal<TYPES, LEAF>
{
    type NodeType = TYPES;
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> ProposalType for ViewSyncCertificate<TYPES> {
    type NodeType = TYPES;
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time {
        match self {
            ViewSyncCertificate::PreCommit(certificate_internal)
            | ViewSyncCertificate::Commit(certificate_internal)
            | ViewSyncCertificate::Finalize(certificate_internal) => certificate_internal.round,
        }
    }
}

/// A proposal to a network of voting nodes.
pub trait ProposalType:
    Debug + Clone + 'static + Serialize + for<'a> Deserialize<'a> + Send + Sync + PartialEq + Eq + Hash
{
    /// Type of nodes that can vote on this proposal.
    type NodeType: NodeType;

    /// Time at which this proposal is valid.
    fn get_view_number(&self) -> <Self::NodeType as NodeType>::Time;
}

/// A state change encoded in a leaf.
///
/// [`DeltasType`] represents a [block](NodeType::BlockType), but it may not contain the block in
/// full. It is guaranteed to contain, at least, a cryptographic commitment to the block, and it
/// provides an interface for resolving the commitment to a full block if the full block is
/// available.
pub trait DeltasType<BlockPayload: Committable>:
    Clone + Debug + for<'a> Deserialize<'a> + PartialEq + Eq + std::hash::Hash + Send + Serialize + Sync
{
    /// Errors reported by this type.
    type Error: std::error::Error;

    /// Get a cryptographic commitment to the block represented by this delta.
    fn block_commitment(&self) -> Commitment<BlockPayload>;

    /// Get the full block if it is available, otherwise return this object unchanged.
    ///
    /// # Errors
    ///
    /// Returns the original [`DeltasType`], unchanged, in an [`Err`] variant in the case where the
    /// full block is not currently available.
    fn try_resolve(self) -> Result<BlockPayload, Self>;

    /// Fill this [`DeltasType`] by providing a complete block.
    ///
    /// After this function succeeds, [`try_resolve`](Self::try_resolve) is guaranteed to return
    /// `Ok(block)`.
    ///
    /// # Errors
    ///
    /// Fails if `block` does not match `self.block_commitment()`, or if the block is not able to be
    /// stored for some implementation-defined reason.
    fn fill(&mut self, block: BlockPayload) -> Result<(), Self::Error>;
}

/// Error which occurs when [`DeltasType::fill`] is called with a block that does not match the
/// deltas' internal block commitment.
#[derive(Clone, Copy, Debug, Snafu)]
#[snafu(display("the block {:?} has commitment {} (expected {})", block, block.commit(), commitment))]
pub struct InconsistentDeltasError<BLOCK: Committable + Debug> {
    /// The block with the wrong commitment.
    block: BLOCK,
    /// The expected commitment.
    commitment: Commitment<BLOCK>,
}

impl<BLOCK> DeltasType<BLOCK> for BLOCK
where
    BLOCK: Committable
        + Clone
        + Debug
        + for<'a> Deserialize<'a>
        + PartialEq
        + Eq
        + std::hash::Hash
        + Send
        + Serialize
        + Sync,
{
    type Error = InconsistentDeltasError<BLOCK>;

    fn block_commitment(&self) -> Commitment<BLOCK> {
        self.commit()
    }

    fn try_resolve(self) -> Result<BLOCK, Self> {
        Ok(self)
    }

    fn fill(&mut self, block: BLOCK) -> Result<(), Self::Error> {
        ensure!(
            block.commit() == self.commit(),
            InconsistentDeltasSnafu {
                block,
                commitment: self.commit()
            }
        );
        // If the commitments are equal the blocks are equal, and we already have the block, so we
        // don't have to do anything.
        Ok(())
    }
}

impl<BLOCK> DeltasType<BLOCK> for Either<BLOCK, Commitment<BLOCK>>
where
    BLOCK: Committable
        + Clone
        + Debug
        + for<'a> Deserialize<'a>
        + PartialEq
        + Eq
        + std::hash::Hash
        + Send
        + Serialize
        + Sync,
{
    type Error = InconsistentDeltasError<BLOCK>;

    fn block_commitment(&self) -> Commitment<BLOCK> {
        match self {
            Either::Left(block) => block.commit(),
            Either::Right(comm) => *comm,
        }
    }

    fn try_resolve(self) -> Result<BLOCK, Self> {
        match self {
            Either::Left(block) => Ok(block),
            Either::Right(_) => Err(self),
        }
    }

    fn fill(&mut self, block: BLOCK) -> Result<(), Self::Error> {
        match self {
            Either::Left(curr) => curr.fill(block),
            Either::Right(comm) => {
                ensure!(
                    *comm == block.commit(),
                    InconsistentDeltasSnafu {
                        block,
                        commitment: *comm
                    }
                );
                *self = Either::Left(block);
                Ok(())
            }
        }
    }
}

/// An item which is appended to a blockchain.
pub trait LeafType:
    Debug
    + Display
    + Clone
    + 'static
    + Committable
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + Sync
    + Eq
    + std::hash::Hash
{
    /// Type of nodes participating in the network.
    type NodeType: NodeType;
    /// Type of block contained by this leaf.
    type DeltasType: DeltasType<LeafBlock<Self>>;
    /// Either state or empty
    type MaybeState: Clone
        + Debug
        + for<'a> Deserialize<'a>
        + PartialEq
        + Eq
        + std::hash::Hash
        + Send
        + Serialize
        + Sync;

    /// Create a new leaf from its components.
    fn new(
        view_number: LeafTime<Self>,
        justify_qc: QuorumCertificate<Self::NodeType, Self>,
        deltas: LeafBlock<Self>,
        state: LeafState<Self>,
    ) -> Self;
    /// Time when this leaf was created.
    fn get_view_number(&self) -> LeafTime<Self>;
    /// Height of this leaf in the chain.
    ///
    /// Equivalently, this is the number of leaves before this one in the chain.
    fn get_height(&self) -> u64;
    /// Change the height of this leaf.
    fn set_height(&mut self, height: u64);
    /// The QC linking this leaf to its parent in the chain.
    fn get_justify_qc(&self) -> QuorumCertificate<Self::NodeType, Self>;
    /// Commitment to this leaf's parent.
    fn get_parent_commitment(&self) -> Commitment<Self>;
    /// The block contained in this leaf.
    fn get_deltas(&self) -> Self::DeltasType;
    /// Fill this leaf with the entire corresponding block.
    ///
    /// After this function succeeds, `self.get_deltas().try_resolve()` is guaranteed to return
    /// `Ok(block)`.
    ///
    /// # Errors
    ///
    /// Fails if `block` does not match `self.get_deltas_commitment()`, or if the block is not able
    /// to be stored for some implementation-defined reason.
    fn fill_deltas(&mut self, block: LeafBlock<Self>) -> Result<(), LeafDeltasError<Self>>;
    /// The blockchain state after appending this leaf.
    fn get_state(&self) -> Self::MaybeState;
    /// Transactions rejected or invalidated by the application of this leaf.
    fn get_rejected(&self) -> Vec<LeafTransaction<Self>>;
    /// Real-world time when this leaf was created.
    fn get_timestamp(&self) -> i128;
    /// Identity of the network participant who proposed this leaf.
    fn get_proposer_id(&self) -> EncodedPublicKey;
    /// Create a leaf from information stored about a view.
    fn from_stored_view(stored_view: StoredView<Self::NodeType, Self>) -> Self;

    /// A commitment to the block contained in this leaf.
    fn get_deltas_commitment(&self) -> Commitment<LeafBlock<Self>> {
        self.get_deltas().block_commitment()
    }
}

/// The [`DeltasType`] in a [`LeafType`].
pub type LeafDeltas<LEAF> = <LEAF as LeafType>::DeltasType;
/// Errors reported by the [`DeltasType`] in a [`LeafType`].
pub type LeafDeltasError<LEAF> = <LeafDeltas<LEAF> as DeltasType<LeafBlock<LEAF>>>::Error;
/// The [`NodeType`] in a [`LeafType`].
pub type LeafNode<LEAF> = <LEAF as LeafType>::NodeType;
/// The [`StateType`] in a [`LeafType`].
pub type LeafState<LEAF> = <LeafNode<LEAF> as NodeType>::StateType;
/// The [`BlockPayload`] in a [`LeafType`].
pub type LeafBlock<LEAF> = <LeafNode<LEAF> as NodeType>::BlockType;
/// The [`Transaction`] in a [`LeafType`].
pub type LeafTransaction<LEAF> = <LeafBlock<LEAF> as BlockPayload>::Transaction;
/// The [`ConsensusTime`] used by a [`LeafType`].
pub type LeafTime<LEAF> = <LeafNode<LEAF> as NodeType>::Time;

/// Additional functions required to use a [`LeafType`] with hotshot-testing.
pub trait TestableLeaf {
    /// Type of nodes participating in the network.
    type NodeType: NodeType;

    /// Create a transaction that can be added to the block contained in this leaf.
    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockType as BlockPayload>::Transaction;
}

/// This is the consensus-internal analogous concept to a block, and it contains the block proper,
/// as well as the hash of its parent `Leaf`.
/// NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::BlockPayload`
#[derive(Serialize, Deserialize, Clone, Debug, Derivative)]
#[serde(bound(deserialize = ""))]
#[derivative(Hash, PartialEq, Eq)]
pub struct ValidatingLeaf<TYPES: NodeType> {
    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Number of leaves before this one in the chain
    pub height: u64,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES, Self>,

    /// The hash of the parent `Leaf`
    /// So we can ask if it extends
    pub parent_commitment: Commitment<ValidatingLeaf<TYPES>>,

    /// BlockPayload leaf wants to apply
    pub deltas: TYPES::BlockType,

    /// What the state should be AFTER applying `self.deltas`
    pub state: TYPES::StateType,

    /// Transactions that were marked for rejection while collecting deltas
    pub rejected: Vec<<TYPES::BlockType as BlockPayload>::Transaction>,

    /// the timestamp the leaf was constructed at, in nanoseconds. Only exposed for dashboard stats
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub timestamp: i128,

    /// the proposer id of the leaf
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub proposer_id: EncodedPublicKey,
}

/// This is the consensus-internal analogous concept to a block, and it contains the block proper,
/// as well as the hash of its parent `Leaf`.
/// NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::BlockPayload`
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, Eq)]
#[serde(bound(deserialize = ""))]
pub struct SequencingLeaf<TYPES: NodeType> {
    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Number of leaves before this one in the chain
    pub height: u64,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES, Self>,

    /// The hash of the parent `SequencingLeaf`
    /// So we can ask if it extends
    pub parent_commitment: Commitment<SequencingLeaf<TYPES>>,

    /// The block or block commitment to be applied
    pub deltas: Either<TYPES::BlockType, Commitment<TYPES::BlockType>>,

    /// Transactions that were marked for rejection while collecting deltas
    pub rejected: Vec<<TYPES::BlockType as BlockPayload>::Transaction>,

    /// the timestamp the leaf was constructed at, in nanoseconds. Only exposed for dashboard stats
    pub timestamp: i128,

    /// the proposer id of the leaf
    pub proposer_id: EncodedPublicKey,
}

impl<TYPES: NodeType> PartialEq for SequencingLeaf<TYPES> {
    fn eq(&self, other: &Self) -> bool {
        let delta_left = match &self.deltas {
            Either::Left(deltas) => deltas.commit(),
            Either::Right(deltas) => *deltas,
        };
        let delta_right = match &other.deltas {
            Either::Left(deltas) => deltas.commit(),
            Either::Right(deltas) => *deltas,
        };
        self.view_number == other.view_number
            && self.height == other.height
            && self.justify_qc == other.justify_qc
            && self.parent_commitment == other.parent_commitment
            && delta_left == delta_right
            && self.rejected == other.rejected
    }
}

impl<TYPES: NodeType> Hash for SequencingLeaf<TYPES> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.view_number.hash(state);
        self.height.hash(state);
        self.justify_qc.hash(state);
        self.parent_commitment.hash(state);
        match &self.deltas {
            Either::Left(deltas) => {
                deltas.commit().hash(state);
            }
            Either::Right(commitment) => {
                commitment.hash(state);
            }
        }
        // self.deltas.hash(state.commit());
        self.rejected.hash(state);
    }
}

impl<TYPES: NodeType> Display for ValidatingLeaf<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "view: {:?}, height: {:?}, justify: {}",
            self.view_number, self.height, self.justify_qc
        )
    }
}

impl<TYPES: NodeType> LeafType for ValidatingLeaf<TYPES> {
    type NodeType = TYPES;
    type DeltasType = TYPES::BlockType;
    type MaybeState = TYPES::StateType;

    fn new(
        view_number: <Self::NodeType as NodeType>::Time,
        justify_qc: QuorumCertificate<Self::NodeType, Self>,
        deltas: <Self::NodeType as NodeType>::BlockType,
        state: <Self::NodeType as NodeType>::StateType,
    ) -> Self {
        Self {
            view_number,
            height: 0,
            justify_qc,
            parent_commitment: fake_commitment(),
            deltas,
            state,
            rejected: Vec::new(),
            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
            proposer_id: genesis_proposer_id(),
        }
    }

    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn get_height(&self) -> u64 {
        self.height
    }

    fn set_height(&mut self, height: u64) {
        self.height = height;
    }

    fn get_justify_qc(&self) -> QuorumCertificate<TYPES, Self> {
        self.justify_qc.clone()
    }

    fn get_parent_commitment(&self) -> Commitment<Self> {
        self.parent_commitment
    }

    fn get_deltas(&self) -> Self::DeltasType {
        self.deltas.clone()
    }

    fn get_deltas_commitment(&self) -> Commitment<<Self::NodeType as NodeType>::BlockType> {
        self.deltas.block_commitment()
    }

    fn fill_deltas(&mut self, block: LeafBlock<Self>) -> Result<(), LeafDeltasError<Self>> {
        self.deltas.fill(block)
    }

    fn get_state(&self) -> Self::MaybeState {
        self.state.clone()
    }

    fn get_rejected(&self) -> Vec<<TYPES::BlockType as BlockPayload>::Transaction> {
        self.rejected.clone()
    }

    fn get_timestamp(&self) -> i128 {
        self.timestamp
    }

    fn get_proposer_id(&self) -> EncodedPublicKey {
        self.proposer_id.clone()
    }

    fn from_stored_view(stored_view: StoredView<Self::NodeType, Self>) -> Self {
        Self {
            view_number: stored_view.view_number,
            height: 0,
            justify_qc: stored_view.justify_qc,
            parent_commitment: stored_view.parent,
            deltas: stored_view.deltas,
            state: stored_view.state,
            rejected: stored_view.rejected,
            timestamp: stored_view.timestamp,
            proposer_id: stored_view.proposer_id,
        }
    }
}

impl<TYPES: NodeType> TestableLeaf for ValidatingLeaf<TYPES>
where
    TYPES::StateType: TestableState,
    TYPES::BlockType: TestableBlock,
{
    type NodeType = TYPES;

    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockType as BlockPayload>::Transaction {
        <TYPES::StateType as TestableState>::create_random_transaction(
            Some(&self.state),
            rng,
            padding,
        )
    }
}

impl<TYPES: NodeType> Display for SequencingLeaf<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "view: {:?}, height: {:?}, justify: {}",
            self.view_number, self.height, self.justify_qc
        )
    }
}

impl<TYPES: NodeType> LeafType for SequencingLeaf<TYPES> {
    type NodeType = TYPES;
    type DeltasType = Either<TYPES::BlockType, Commitment<TYPES::BlockType>>;
    type MaybeState = ();

    fn new(
        view_number: <Self::NodeType as NodeType>::Time,
        justify_qc: QuorumCertificate<Self::NodeType, Self>,
        deltas: <Self::NodeType as NodeType>::BlockType,
        _state: <Self::NodeType as NodeType>::StateType,
    ) -> Self {
        Self {
            view_number,
            height: 0,
            justify_qc,
            parent_commitment: fake_commitment(),
            deltas: Either::Left(deltas),
            rejected: Vec::new(),
            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
            proposer_id: genesis_proposer_id(),
        }
    }

    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn get_height(&self) -> u64 {
        self.height
    }

    fn set_height(&mut self, height: u64) {
        self.height = height;
    }

    fn get_justify_qc(&self) -> QuorumCertificate<TYPES, Self> {
        self.justify_qc.clone()
    }

    fn get_parent_commitment(&self) -> Commitment<Self> {
        self.parent_commitment
    }

    fn get_deltas(&self) -> Self::DeltasType {
        self.deltas.clone()
    }

    fn get_deltas_commitment(&self) -> Commitment<<Self::NodeType as NodeType>::BlockType> {
        self.deltas.block_commitment()
    }

    fn fill_deltas(&mut self, block: LeafBlock<Self>) -> Result<(), LeafDeltasError<Self>> {
        self.deltas.fill(block)
    }

    // The Sequencing Leaf doesn't have a state.
    fn get_state(&self) -> Self::MaybeState {}

    fn get_rejected(&self) -> Vec<<TYPES::BlockType as BlockPayload>::Transaction> {
        self.rejected.clone()
    }

    fn get_timestamp(&self) -> i128 {
        self.timestamp
    }

    fn get_proposer_id(&self) -> EncodedPublicKey {
        self.proposer_id.clone()
    }

    fn from_stored_view(stored_view: StoredView<Self::NodeType, Self>) -> Self {
        Self {
            view_number: stored_view.view_number,
            height: 0,
            justify_qc: stored_view.justify_qc,
            parent_commitment: stored_view.parent,
            deltas: stored_view.deltas,
            rejected: stored_view.rejected,
            timestamp: stored_view.timestamp,
            proposer_id: stored_view.proposer_id,
        }
    }
}

impl<TYPES: NodeType> TestableLeaf for SequencingLeaf<TYPES>
where
    TYPES::StateType: TestableState,
    TYPES::BlockType: TestableBlock,
{
    type NodeType = TYPES;

    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockType as BlockPayload>::Transaction {
        TYPES::StateType::create_random_transaction(None, rng, padding)
    }
}
/// Fake the thing a genesis block points to. Needed to avoid infinite recursion
#[must_use]
pub fn fake_commitment<S: Committable>() -> Commitment<S> {
    commit::RawCommitmentBuilder::new("Dummy commitment for arbitrary genesis").finalize()
}

/// create a random commitment
#[must_use]
pub fn random_commitment<S: Committable>(rng: &mut dyn rand::RngCore) -> Commitment<S> {
    let random_array: Vec<u8> = (0u8..100u8).map(|_| rng.gen_range(0..255)).collect();
    commit::RawCommitmentBuilder::new("Random Commitment")
        .constant_str("Random Field")
        .var_size_bytes(&random_array)
        .finalize()
}

/// Serialization for the QC assembled signature
/// # Panics
/// if serialization fails
pub fn serialize_signature<TYPES: NodeType>(signature: &AssembledSignature<TYPES>) -> Vec<u8> {
    let mut signatures_bytes = vec![];
    let signatures: Option<<TYPES::SignatureKey as SignatureKey>::QCType> = match &signature {
        AssembledSignature::DA(signatures) => {
            signatures_bytes.extend("DA".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::Yes(signatures) => {
            signatures_bytes.extend("Yes".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::No(signatures) => {
            signatures_bytes.extend("No".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::ViewSyncPreCommit(signatures) => {
            signatures_bytes.extend("ViewSyncPreCommit".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::ViewSyncCommit(signatures) => {
            signatures_bytes.extend("ViewSyncCommit".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::ViewSyncFinalize(signatures) => {
            signatures_bytes.extend("ViewSyncFinalize".as_bytes());
            Some(signatures.clone())
        }
        AssembledSignature::Genesis() => None,
    };
    if let Some(sig) = signatures {
        let (sig, proof) = TYPES::SignatureKey::get_sig_proof(&sig);
        let proof_bytes = bincode_opts()
            .serialize(&proof.as_bitslice())
            .expect("This serialization shouldn't be able to fail");
        signatures_bytes.extend("bitvec proof".as_bytes());
        signatures_bytes.extend(proof_bytes.as_slice());
        let sig_bytes = bincode_opts()
            .serialize(&sig)
            .expect("This serialization shouldn't be able to fail");
        signatures_bytes.extend("aggregated signature".as_bytes());
        signatures_bytes.extend(sig_bytes.as_slice());
    } else {
        signatures_bytes.extend("genesis".as_bytes());
    }

    signatures_bytes
}

impl<TYPES: NodeType> Committable for ValidatingLeaf<TYPES> {
    fn commit(&self) -> commit::Commitment<Self> {
        let signatures_bytes = serialize_signature(&self.justify_qc.signatures);

        commit::RawCommitmentBuilder::new("leaf commitment")
            .u64_field("view number", *self.view_number)
            .u64_field("height", self.height)
            .field("parent Leaf commitment", self.parent_commitment)
            .field("block commitment", self.deltas.commit())
            .field("state commitment", self.state.commit())
            .constant_str("justify_qc view number")
            .u64(*self.justify_qc.view_number)
            .field(
                "justify_qc leaf commitment",
                self.justify_qc.leaf_commitment(),
            )
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes)
            .finalize()
    }

    fn tag() -> String {
        tag::LEAF.to_string()
    }
}

impl<TYPES: NodeType> Committable for SequencingLeaf<TYPES> {
    fn commit(&self) -> commit::Commitment<Self> {
        // Commit the block commitment, rather than the block, so that the replicas can reconstruct
        // the leaf.
        let block_commitment = match &self.deltas {
            Either::Left(block) => block.commit(),
            Either::Right(commitment) => *commitment,
        };

        let signatures_bytes = serialize_signature(&self.justify_qc.signatures);

        commit::RawCommitmentBuilder::new("leaf commitment")
            .u64_field("view number", *self.view_number)
            .u64_field("height", self.height)
            .field("parent Leaf commitment", self.parent_commitment)
            .field("block commitment", block_commitment)
            .constant_str("justify_qc view number")
            .u64(*self.justify_qc.view_number)
            .field(
                "justify_qc leaf commitment",
                self.justify_qc.leaf_commitment(),
            )
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes)
            .finalize()
    }
}

impl<TYPES: NodeType> From<ValidatingLeaf<TYPES>>
    for ValidatingProposal<TYPES, ValidatingLeaf<TYPES>>
{
    fn from(leaf: ValidatingLeaf<TYPES>) -> Self {
        Self {
            view_number: leaf.view_number,
            height: leaf.height,
            justify_qc: leaf.justify_qc,
            parent_commitment: leaf.parent_commitment,
            deltas: leaf.deltas.clone(),
            state_commitment: leaf.state.commit(),
            rejected: leaf.rejected,
            proposer_id: leaf.proposer_id,
            block_commitment: leaf.deltas.commit(),
        }
    }
}

impl<TYPES, LEAF> From<LEAF> for StoredView<TYPES, LEAF>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
{
    fn from(leaf: LEAF) -> Self {
        StoredView {
            view_number: leaf.get_view_number(),
            height: leaf.get_height(),
            parent: leaf.get_parent_commitment(),
            justify_qc: leaf.get_justify_qc(),
            state: leaf.get_state(),
            deltas: leaf.get_deltas(),
            rejected: leaf.get_rejected(),
            timestamp: leaf.get_timestamp(),
            proposer_id: leaf.get_proposer_id(),
        }
    }
}
