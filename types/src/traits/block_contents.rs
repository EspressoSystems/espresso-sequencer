//! Abstraction over the contents of a block
//!
//! This module provides the [`Transaction`], [`BlockPayload`], and [`BlockHeader`] traits, which
//! describe the behaviors that a block is expected to have.

use std::{
    error::Error,
    fmt::{Debug, Display},
    future::Future,
    hash::Hash,
};

use committable::{Commitment, Committable};
use jf_primitives::vid::VidScheme;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    data::Leaf,
    traits::{node_implementation::NodeType, ValidatedState},
    utils::BuilderCommitment,
    vid::{vid_scheme, VidCommitment, VidSchemeType},
};

/// Abstraction over any type of transaction. Used by [`BlockPayload`].
pub trait Transaction:
    Clone + Serialize + DeserializeOwned + Debug + PartialEq + Eq + Sync + Send + Committable + Hash
{
    /// Build a transaction from bytes
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Get the length of the transaction
    fn len(&self) -> usize;

    /// Whether or not the transaction is empty
    fn is_empty(&self) -> bool;
}

/// Abstraction over the full contents of a block
///
/// This trait encapsulates the behaviors that the transactions of a block must have in order to be
/// used by consensus
///   * Must have a predefined error type ([`BlockPayload::Error`])
///   * Must have a transaction type that can be compared for equality, serialized and serialized,
///     sent between threads, and can have a hash produced of it
///   * Must be hashable
pub trait BlockPayload:
    Serialize + Clone + Debug + Display + Hash + PartialEq + Eq + Send + Sync + DeserializeOwned
{
    /// The error type for this type of block
    type Error: Error + Debug + Send + Sync + Serialize + DeserializeOwned;

    /// The type of the transitions we are applying
    type Transaction: Transaction;

    /// Data created during block building which feeds into the block header
    type Metadata: Clone + Debug + DeserializeOwned + Eq + Hash + Send + Sync + Serialize;

    /// Encoded payload.
    type Encode<'a>: 'a + Iterator<Item = u8> + Send
    where
        Self: 'a;

    /// Build a payload and associated metadata with the transactions.
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error>;

    /// Build a payload with the encoded transaction bytes, metadata,
    /// and the associated number of VID storage nodes
    ///
    /// `I` may be, but not necessarily is, the `Encode` type directly from `fn encode`.
    fn from_bytes<I>(encoded_transactions: I, metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>;

    /// Build the genesis payload and metadata.
    fn genesis() -> (Self, Self::Metadata);

    /// Encode the payload
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error>;

    /// List of transaction commitments.
    fn transaction_commitments(
        &self,
        metadata: &Self::Metadata,
    ) -> Vec<Commitment<Self::Transaction>> {
        self.get_transactions(metadata)
            .map(|tx| tx.commit())
            .collect()
    }

    /// Number of transactions in the block.
    fn num_transactions(&self, metadata: &Self::Metadata) -> usize {
        self.get_transactions(metadata).count()
    }

    /// Generate commitment that builders use to sign block options.
    fn builder_commitment(&self, metadata: &Self::Metadata) -> BuilderCommitment;

    /// Get the transactions in the payload.
    fn get_transactions<'a>(
        &'a self,
        metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction>;
}

/// extra functions required on block to be usable by hotshot-testing
pub trait TestableBlock: BlockPayload + Debug {
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
pub fn vid_commitment(
    encoded_transactions: &Vec<u8>,
    num_storage_nodes: usize,
) -> <VidSchemeType as VidScheme>::Commit {
    #[allow(clippy::panic)]
    vid_scheme(num_storage_nodes).commit_only(encoded_transactions).unwrap_or_else(|err| panic!("VidScheme::commit_only failure:\n\t(num_storage_nodes,payload_byte_len)=({num_storage_nodes},{}\n\t{err}", encoded_transactions.len()))
}

/// The number of storage nodes to use when computing the genesis VID commitment.
///
/// The number of storage nodes for the genesis VID commitment is arbitrary, since we don't actually
/// do dispersal for the genesis block. For simplicity and performance, we use 1.
pub const GENESIS_VID_NUM_STORAGE_NODES: usize = 1;

/// Header of a block, which commits to a [`BlockPayload`].
pub trait BlockHeader<TYPES: NodeType>:
    Serialize + Clone + Debug + Hash + PartialEq + Eq + Send + Sync + DeserializeOwned + Committable
{
    /// Build a header with the parent validate state, instance-level state, parent leaf, payload
    /// commitment, and metadata.
    fn new(
        parent_state: &TYPES::ValidatedState,
        instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        parent_leaf: &Leaf<TYPES>,
        payload_commitment: VidCommitment,
        metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    ) -> impl Future<Output = Self> + Send;

    /// Build the genesis header, payload, and metadata.
    fn genesis(
        instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        payload_commitment: VidCommitment,
        metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    ) -> Self;

    /// Get the block number.
    fn block_number(&self) -> u64;

    /// Get the payload commitment.
    fn payload_commitment(&self) -> VidCommitment;

    /// Get the metadata.
    fn metadata(&self) -> &<TYPES::BlockPayload as BlockPayload>::Metadata;
}
