//! Provides types useful for representing `HotShot`'s data structures
//!
//! This module provides types for representing consensus internal state, such as leaves,
//! `HotShot`'s version of a block, and proposals, messages upon which to reach the consensus.

use crate::{
    simple_certificate::{QuorumCertificate, TimeoutCertificate},
    traits::{
        block_contents::vid_commitment,
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::NodeType,
        signature_key::SignatureKey,
        state::{ConsensusTime, TestableBlock, TestableState},
        storage::StoredView,
        BlockPayload, State,
    },
    vote::{Certificate, HasViewNumber},
};
use ark_bls12_381::Bls12_381;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bincode::Options;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use derivative::Derivative;
use hotshot_utils::bincode::bincode_opts;
use jf_primitives::{
    pcs::{checked_fft_size, prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::VidDisperse as JfVidDisperse,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
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
    /// Returen the u64 format
    fn get_u64(&self) -> u64 {
        self.0
    }
}

impl Committable for ViewNumber {
    fn commit(&self) -> Commitment<Self> {
        let builder = RawCommitmentBuilder::new("View Number Commitment");
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

/// The `Transaction` type associated with a `State`, as a syntactic shortcut
pub type Transaction<STATE> = <<STATE as State>::BlockPayload as BlockPayload>::Transaction;
/// `Commitment` to the `Transaction` type associated with a `State`, as a syntactic shortcut
pub type TxnCommitment<STATE> = Commitment<Transaction<STATE>>;

/// A proposal to start providing data availability for a block.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DAProposal<TYPES: NodeType> {
    /// Encoded transactions in the block to be applied.
    pub encoded_transactions: Vec<u8>,
    /// Metadata of the block to be applied.
    pub metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    /// View this proposal applies to
    pub view_number: TYPES::Time,
}

/// The VID scheme type used in `HotShot`.
pub type VidScheme = jf_primitives::vid::advz::Advz<ark_bls12_381::Bls12_381, sha2::Sha256>;
pub use jf_primitives::vid::VidScheme as VidSchemeTrait;
/// VID commitment.
pub type VidCommitment = <VidScheme as VidSchemeTrait>::Commit;

/// VID dispersal data
///
/// Like [`DAProposal`].
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VidDisperse<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::Time,
    /// Block payload commitment
    pub payload_commitment: VidCommitment,
    /// A storage node's key and its corresponding VID share
    pub shares: BTreeMap<TYPES::SignatureKey, <VidScheme as VidSchemeTrait>::Share>,
    /// VID common data sent to all storage nodes
    pub common: <VidScheme as VidSchemeTrait>::Common,
}

impl<TYPES: NodeType> VidDisperse<TYPES> {
    /// Create VID dispersal from a specified membership
    /// Uses the specified function to calculate share dispersal
    /// Allows for more complex stake table functionality
    pub fn from_membership(
        view_number: TYPES::Time,
        mut vid_disperse: JfVidDisperse<VidScheme>,
        membership: &Arc<TYPES::Membership>,
    ) -> Self {
        let shares = membership
            .get_committee(view_number)
            .iter()
            .map(|node| (node.clone(), vid_disperse.shares.remove(0)))
            .collect();

        Self {
            view_number,
            shares,
            common: vid_disperse.common,
            payload_commitment: vid_disperse.commit,
        }
    }
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
pub struct QuorumProposal<TYPES: NodeType> {
    /// The block header to append
    pub block_header: TYPES::BlockHeader,

    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES>,

    /// Possible timeout certificate.  Only present if the justify_qc is not for the preceding view
    pub timeout_certificate: Option<TimeoutCertificate<TYPES>>,

    /// the propser id
    pub proposer_id: TYPES::SignatureKey,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for DAProposal<TYPES> {
    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for VidDisperse<TYPES> {
    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for QuorumProposal<TYPES> {
    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

/// A state change encoded in a leaf.
///
/// [`DeltasType`] represents a [block](NodeType::BlockPayload), but it may not contain the block in
/// full. It is guaranteed to contain, at least, a cryptographic commitment to the block, and it
/// provides an interface for resolving the commitment to a full block if the full block is
/// available.
pub trait DeltasType<PAYLOAD: BlockPayload>:
    Clone + Debug + for<'a> Deserialize<'a> + PartialEq + Eq + std::hash::Hash + Send + Serialize + Sync
{
    /// Errors reported by this type.
    type Error: std::error::Error;

    /// Get a cryptographic commitment to the block represented by this delta.
    fn payload_commitment(&self) -> VidCommitment;

    /// Get the full block if it is available, otherwise return this object unchanged.
    ///
    /// # Errors
    ///
    /// Returns the original [`DeltasType`], unchanged, in an [`Err`] variant in the case where the
    /// full block is not currently available.
    fn try_resolve(self) -> Result<PAYLOAD, Self>;

    /// Fill this [`DeltasType`] by providing a complete block.
    ///
    /// After this function succeeds, [`try_resolve`](Self::try_resolve) is guaranteed to return
    /// `Ok(block)`.
    ///
    /// # Errors
    ///
    /// Fails if `block` does not match `self.payload_commitment()`, or if the block is not able to be
    /// stored for some implementation-defined reason.
    fn fill(&mut self, block: PAYLOAD) -> Result<(), Self::Error>;
}

/// The error type for block and its transactions.
#[derive(Snafu, Debug)]
pub enum BlockError {
    /// Invalid block header.
    InvalidBlockHeader,
    /// Invalid transaction length.
    InvalidTransactionLength,
    /// Inconsistent payload commitment.
    InconsistentPayloadCommitment,
}

/// Additional functions required to use a [`Leaf`] with hotshot-testing.
pub trait TestableLeaf {
    /// Type of nodes participating in the network.
    type NodeType: NodeType;

    /// Create a transaction that can be added to the block contained in this leaf.
    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockPayload as BlockPayload>::Transaction;
}

/// This is the consensus-internal analogous concept to a block, and it contains the block proper,
/// as well as the hash of its parent `Leaf`.
/// NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::BlockPayload`
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, Eq)]
#[serde(bound(deserialize = ""))]
pub struct Leaf<TYPES: NodeType> {
    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES>,

    /// The hash of the parent `Leaf`
    /// So we can ask if it extends
    pub parent_commitment: Commitment<Self>,

    /// Block header.
    pub block_header: TYPES::BlockHeader,

    /// Optional block payload.
    ///
    /// It may be empty for nodes not in the DA committee.
    pub block_payload: Option<TYPES::BlockPayload>,

    /// State.
    pub state: TYPES::StateType,

    /// Transactions that were marked for rejection while collecting the block.
    pub rejected: Vec<<TYPES::BlockPayload as BlockPayload>::Transaction>,

    /// the proposer id of the leaf
    pub proposer_id: TYPES::SignatureKey,
}

impl<TYPES: NodeType> PartialEq for Leaf<TYPES> {
    fn eq(&self, other: &Self) -> bool {
        self.view_number == other.view_number
            && self.justify_qc == other.justify_qc
            && self.parent_commitment == other.parent_commitment
            && self.block_header == other.block_header
            && self.state == other.state
            && self.rejected == other.rejected
    }
}

impl<TYPES: NodeType> Hash for Leaf<TYPES> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.view_number.hash(state);
        self.justify_qc.hash(state);
        self.parent_commitment.hash(state);
        self.block_header.hash(state);
        self.state.hash(state);
        self.rejected.hash(state);
    }
}

impl<TYPES: NodeType> Display for Leaf<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "view: {:?}, height: {:?}, justify: {}",
            self.view_number,
            self.get_height(),
            self.justify_qc
        )
    }
}

impl<TYPES: NodeType> Leaf<TYPES> {
    /// Create a new leaf from its components.
    #[must_use]
    pub fn genesis() -> Self {
        let (block_header, block_payload, _) = TYPES::BlockHeader::genesis();
        Self {
            view_number: TYPES::Time::genesis(),
            justify_qc: QuorumCertificate::<TYPES>::genesis(),
            parent_commitment: fake_commitment(),
            block_header,
            block_payload: Some(block_payload),
            state: <TYPES::StateType as State>::initialize(),
            rejected: Vec::new(),
            proposer_id: <<TYPES as NodeType>::SignatureKey as SignatureKey>::genesis_proposer_pk(),
        }
    }

    /// Time when this leaf was created.
    pub fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }
    /// Height of this leaf in the chain.
    ///
    /// Equivalently, this is the number of leaves before this one in the chain.
    pub fn get_height(&self) -> u64 {
        self.block_header.block_number()
    }
    /// The QC linking this leaf to its parent in the chain.
    pub fn get_justify_qc(&self) -> QuorumCertificate<TYPES> {
        self.justify_qc.clone()
    }
    /// Commitment to this leaf's parent.
    pub fn get_parent_commitment(&self) -> Commitment<Self> {
        self.parent_commitment
    }
    /// The block header contained in this leaf.
    pub fn get_block_header(&self) -> &<TYPES as NodeType>::BlockHeader {
        &self.block_header
    }
    /// Fill this leaf with the block payload.
    ///
    /// # Errors
    ///
    /// Fails if the payload commitment doesn't match `self.block_header.payload_commitment()`
    /// or if the transactions are of invalid length
    pub fn fill_block_payload(
        &mut self,
        block_payload: TYPES::BlockPayload,
        num_storage_nodes: usize,
    ) -> Result<(), BlockError> {
        let encoded_txns = match block_payload.encode() {
            // TODO (Keyao) [VALIDATED_STATE] - Avoid collect/copy on the encoded transaction bytes.
            // <https://github.com/EspressoSystems/HotShot/issues/2115>
            Ok(encoded) => encoded.into_iter().collect(),
            Err(_) => return Err(BlockError::InvalidTransactionLength),
        };
        let commitment = vid_commitment(&encoded_txns, num_storage_nodes);
        if commitment != self.block_header.payload_commitment() {
            return Err(BlockError::InconsistentPayloadCommitment);
        }
        self.block_payload = Some(block_payload);
        Ok(())
    }

    /// Fill this leaf with the block payload, without checking
    /// header and payload consistency
    pub fn fill_block_payload_unchecked(&mut self, block_payload: TYPES::BlockPayload) {
        self.block_payload = Some(block_payload);
    }

    /// Optional block payload.
    pub fn get_block_payload(&self) -> Option<TYPES::BlockPayload> {
        self.block_payload.clone()
    }

    /// A commitment to the block payload contained in this leaf.
    pub fn get_payload_commitment(&self) -> VidCommitment {
        self.get_block_header().payload_commitment()
    }
    /// The blockchain state after appending this leaf.
    pub fn get_state(&self) -> TYPES::StateType {
        self.state.clone()
    }
    /// Transactions rejected or invalidated by the application of this leaf.
    pub fn get_rejected(&self) -> Vec<<TYPES::BlockPayload as BlockPayload>::Transaction> {
        self.rejected.clone()
    }
    /// Identity of the network participant who proposed this leaf.
    pub fn get_proposer_id(&self) -> TYPES::SignatureKey {
        self.proposer_id.clone()
    }
    /// Create a leaf from information stored about a view.
    pub fn from_stored_view(stored_view: StoredView<TYPES>) -> Self {
        Self {
            view_number: stored_view.view_number,
            justify_qc: stored_view.justify_qc,
            parent_commitment: stored_view.parent,
            block_header: stored_view.block_header,
            block_payload: stored_view.block_payload,
            state: stored_view.state,
            rejected: stored_view.rejected,
            proposer_id: stored_view.proposer_id,
        }
    }
}

impl<TYPES: NodeType> TestableLeaf for Leaf<TYPES>
where
    TYPES::StateType: TestableState,
    TYPES::BlockPayload: TestableBlock,
{
    type NodeType = TYPES;

    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockPayload as BlockPayload>::Transaction {
        TYPES::StateType::create_random_transaction(None, rng, padding)
    }
}
/// Fake the thing a genesis block points to. Needed to avoid infinite recursion
#[must_use]
pub fn fake_commitment<S: Committable>() -> Commitment<S> {
    RawCommitmentBuilder::new("Dummy commitment for arbitrary genesis").finalize()
}

/// create a random commitment
#[must_use]
pub fn random_commitment<S: Committable>(rng: &mut dyn rand::RngCore) -> Commitment<S> {
    let random_array: Vec<u8> = (0u8..100u8).map(|_| rng.gen_range(0..255)).collect();
    RawCommitmentBuilder::new("Random Commitment")
        .constant_str("Random Field")
        .var_size_bytes(&random_array)
        .finalize()
}

/// Serialization for the QC assembled signature
/// # Panics
/// if serialization fails
pub fn serialize_signature2<TYPES: NodeType>(
    signatures: &<TYPES::SignatureKey as SignatureKey>::QCType,
) -> Vec<u8> {
    let mut signatures_bytes = vec![];
    signatures_bytes.extend("Yes".as_bytes());

    let (sig, proof) = TYPES::SignatureKey::get_sig_proof(signatures);
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
    signatures_bytes
}

impl<TYPES: NodeType> Committable for Leaf<TYPES> {
    fn commit(&self) -> commit::Commitment<Self> {
        let signatures_bytes = if self.justify_qc.is_genesis {
            let mut bytes = vec![];
            bytes.extend("genesis".as_bytes());
            bytes
        } else {
            serialize_signature2::<TYPES>(self.justify_qc.signatures.as_ref().unwrap())
        };

        // Skip the transaction commitments, so that the repliacs can reconstruct the leaf.
        RawCommitmentBuilder::new("leaf commitment")
            .u64_field("view number", *self.view_number)
            .u64_field("block number", self.get_height())
            .field("parent Leaf commitment", self.parent_commitment)
            .constant_str("block payload commitment")
            .fixed_size_bytes(self.get_payload_commitment().as_ref().as_ref())
            .constant_str("justify_qc view number")
            .u64(*self.justify_qc.view_number)
            .field(
                "justify_qc leaf commitment",
                self.justify_qc.get_data().leaf_commit,
            )
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes)
            .finalize()
    }
}

impl<TYPES> From<Leaf<TYPES>> for StoredView<TYPES>
where
    TYPES: NodeType,
{
    fn from(leaf: Leaf<TYPES>) -> Self {
        StoredView {
            view_number: leaf.get_view_number(),
            parent: leaf.get_parent_commitment(),
            justify_qc: leaf.get_justify_qc(),
            block_header: leaf.get_block_header().clone(),
            block_payload: leaf.get_block_payload(),
            state: leaf.get_state(),
            rejected: leaf.get_rejected(),
            proposer_id: leaf.get_proposer_id(),
        }
    }
}
