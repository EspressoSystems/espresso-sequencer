//! Abstraction over the contents of a block
//!
//! This module provides the [`Transaction`], [`BlockPayload`], and [`BlockHeader`] traits, which
//! describe the behaviors that a block is expected to have.

use std::{
    error::Error,
    fmt::{Debug, Display},
    future::Future,
    hash::Hash,
    sync::Arc,
};

use async_trait::async_trait;
use committable::{Commitment, Committable};
use jf_vid::{precomputable::Precomputable, VidScheme};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vbs::version::Version;

use super::{auction_results_provider::HasUrls, signature_key::BuilderSignatureKey};
use crate::{
    data::Leaf,
    traits::{node_implementation::NodeType, states::InstanceState, ValidatedState},
    utils::BuilderCommitment,
    vid::{vid_scheme, VidCommitment, VidCommon, VidSchemeType},
};

/// Trait for structures that need to be unambiguously encoded as bytes.
pub trait EncodeBytes {
    /// Encode `&self`
    fn encode(&self) -> Arc<[u8]>;
}

/// Abstraction over any type of transaction. Used by [`BlockPayload`].
pub trait Transaction:
    Clone + Serialize + DeserializeOwned + Debug + PartialEq + Eq + Sync + Send + Committable + Hash
{
}

/// Abstraction over the full contents of a block
///
/// This trait encapsulates the behaviors that the transactions of a block must have in order to be
/// used by consensus
///   * Must have a predefined error type ([`BlockPayload::Error`])
///   * Must have a transaction type that can be compared for equality, serialized and serialized,
///     sent between threads, and can have a hash produced of it
///   * Must be hashable
#[async_trait]
pub trait BlockPayload<TYPES: NodeType>:
    Serialize
    + Clone
    + Debug
    + Display
    + Hash
    + PartialEq
    + Eq
    + Send
    + Sync
    + DeserializeOwned
    + EncodeBytes
{
    /// The error type for this type of block
    type Error: Error + Debug + Send + Sync + Serialize + DeserializeOwned;

    /// The type of the instance-level state this state is associated with
    type Instance: InstanceState;
    /// The type of the transitions we are applying
    type Transaction: Transaction + Serialize + DeserializeOwned;
    /// Validated State
    type ValidatedState: ValidatedState<TYPES>;
    /// Data created during block building which feeds into the block header
    type Metadata: Clone
        + Debug
        + DeserializeOwned
        + Eq
        + Hash
        + Send
        + Sync
        + Serialize
        + EncodeBytes;

    /// Build a payload and associated metadata with the transactions.
    /// This function is asynchronous because it may need to request updated state from the peers via GET requests.
    /// # Errors
    /// If the transaction length conversion fails.
    async fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction> + Send,
        validated_state: &Self::ValidatedState,
        instance_state: &Self::Instance,
    ) -> Result<(Self, Self::Metadata), Self::Error>;

    /// Build a payload with the encoded transaction bytes, metadata,
    /// and the associated number of VID storage nodes
    fn from_bytes(encoded_transactions: &[u8], metadata: &Self::Metadata) -> Self;

    /// Build the payload and metadata for genesis/null block.
    fn empty() -> (Self, Self::Metadata);

    /// List of transaction commitments.
    fn transaction_commitments(
        &self,
        metadata: &Self::Metadata,
    ) -> Vec<Commitment<Self::Transaction>> {
        self.transactions(metadata).map(|tx| tx.commit()).collect()
    }

    /// Number of transactions in the block.
    fn num_transactions(&self, metadata: &Self::Metadata) -> usize {
        self.transactions(metadata).count()
    }

    /// Generate commitment that builders use to sign block options.
    fn builder_commitment(&self, metadata: &Self::Metadata) -> BuilderCommitment;

    /// Get the transactions in the payload.
    fn transactions<'a>(
        &'a self,
        metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction>;
}

/// extra functions required on block to be usable by hotshot-testing
pub trait TestableBlock<TYPES: NodeType>: BlockPayload<TYPES> + Debug {
    /// generate a genesis block
    fn genesis() -> Self;

    /// the number of transactions in this block
    fn txn_count(&self) -> u64;
}

/// Compute the VID payload commitment.
/// TODO(Gus) delete this function?
/// # Panics
/// If the VID computation fails.
#[must_use]
#[allow(clippy::panic)]
pub fn vid_commitment(
    encoded_transactions: &[u8],
    num_storage_nodes: usize,
) -> <VidSchemeType as VidScheme>::Commit {
    let encoded_tx_len = encoded_transactions.len();
    vid_scheme(num_storage_nodes).commit_only(encoded_transactions).unwrap_or_else(|err| panic!("VidScheme::commit_only failure:(num_storage_nodes,payload_byte_len)=({num_storage_nodes},{encoded_tx_len}) error: {err}"))
}

/// Compute the VID payload commitment along with precompute data reducing time in VID Disperse
/// # Panics
/// If the VID computation fails.
#[must_use]
#[allow(clippy::panic)]
pub fn precompute_vid_commitment(
    encoded_transactions: &[u8],
    num_storage_nodes: usize,
) -> (
    <VidSchemeType as VidScheme>::Commit,
    <VidSchemeType as Precomputable>::PrecomputeData,
) {
    let encoded_tx_len = encoded_transactions.len();
    vid_scheme(num_storage_nodes).commit_only_precompute(encoded_transactions).unwrap_or_else(|err| panic!("VidScheme::commit_only failure:(num_storage_nodes,payload_byte_len)=({num_storage_nodes},{encoded_tx_len}) error: {err}"))
}

/// The number of storage nodes to use when computing the genesis VID commitment.
///
/// The number of storage nodes for the genesis VID commitment is arbitrary, since we don't actually
/// do dispersal for the genesis block. For simplicity and performance, we use 1.
pub const GENESIS_VID_NUM_STORAGE_NODES: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Information about builder fee for proposed block
pub struct BuilderFee<TYPES: NodeType> {
    /// Proposed fee amount
    pub fee_amount: u64,
    /// Account authorizing the fee.
    pub fee_account: TYPES::BuilderSignatureKey,
    /// Signature over fee amount by `fee_account`.
    pub fee_signature: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
}

/// Header of a block, which commits to a [`BlockPayload`].
pub trait BlockHeader<TYPES: NodeType>:
    Serialize + Clone + Debug + Hash + PartialEq + Eq + Send + Sync + DeserializeOwned + Committable
{
    /// Error type for this type of block header
    type Error: Error + Debug + Send + Sync;
    /// Type of Auction Results
    type AuctionResult: HasUrls + Send;

    /// Build a header with the parent validate state, instance-level state, parent leaf, payload
    /// commitment, and metadata. This is only used in pre-marketplace versions
    #[allow(clippy::too_many_arguments)]
    fn new_legacy(
        parent_state: &TYPES::ValidatedState,
        instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        parent_leaf: &Leaf<TYPES>,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
        builder_fee: BuilderFee<TYPES>,
        vid_common: VidCommon,
        version: Version,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send;

    /// Build a header with the parent validate state, instance-level state, parent leaf, payload
    /// commitment, metadata, and auction results. This is only used in post-marketplace versions
    #[allow(clippy::too_many_arguments)]
    fn new_marketplace(
        parent_state: &TYPES::ValidatedState,
        instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        parent_leaf: &Leaf<TYPES>,
        payload_commitment: VidCommitment,
        metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
        builder_fee: Vec<BuilderFee<TYPES>>,
        vid_common: VidCommon,
        auction_results: Option<Self::AuctionResult>,
        version: Version,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send;

    /// Build the genesis header, payload, and metadata.
    fn genesis(
        instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    ) -> Self;

    /// Get the block number.
    fn block_number(&self) -> u64;

    /// Get the payload commitment.
    fn payload_commitment(&self) -> VidCommitment;

    /// Get the metadata.
    fn metadata(&self) -> &<TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata;

    /// Get the builder commitment
    fn builder_commitment(&self) -> BuilderCommitment;

    /// Get the results of the auction for this Header. Only used in post-marketplace versions
    fn get_auction_results(&self) -> Option<Self::AuctionResult>;
}
