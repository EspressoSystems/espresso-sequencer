//! Abstraction over the contents of a block
//!
//! This module provides the [`Transaction`], [`BlockPayload`], and [`BlockHeader`] traits, which
//! describe the behaviors that a block is expected to have.

use commit::{Commitment, Committable};
use serde::{de::DeserializeOwned, Serialize};

use std::{
    collections::HashSet,
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

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
pub trait BlockPayload:
    Serialize
    + Clone
    + Debug
    + Display
    + Hash
    + PartialEq
    + Eq
    + Send
    + Sync
    + Committable
    + DeserializeOwned
{
    /// The error type for this type of block
    type Error: Error + Debug + Send + Sync;

    /// The type of the transitions we are applying
    type Transaction: Transaction;

    /// Data created during block building which feeds into the block header
    type Metadata;

    /// Build a payload and associated metadata with the transactions.
    fn build(transactions: impl IntoIterator<Item = Self::Transaction>) -> (Self, Self::Metadata);

    /// Encode the payload
    fn encode(&self) -> Vec<u8>;

    /// Decode the payload
    fn decode(encoded_transactions: &[u8]) -> Self;

    /// returns hashes of all the transactions in this block
    /// TODO make this ordered with a vec
    fn transaction_commitments(&self) -> HashSet<Commitment<Self::Transaction>>;
}

/// Header of a block, which commits to a [`BlockPayload`].
pub trait BlockHeader:
    Serialize + Clone + Debug + Hash + PartialEq + Eq + Send + Sync + DeserializeOwned
{
    /// Block payload associated with the commitment.
    type Payload: BlockPayload;

    /// Build a header with the payload commitment, metadata, and parent header.
    fn new(
        payload_commitment: Commitment<Self::Payload>,
        metadata: <Self::Payload as BlockPayload>::Metadata,
        parent_header: &Self,
    ) -> Self;

    /// Build a genesis header with the genesis payload.
    fn genesis(payload: Self::Payload) -> Self;

    /// Get the block number.
    fn block_number(&self) -> u64;

    /// Get the payload commitment.
    fn payload_commitment(&self) -> Commitment<Self::Payload>;
}
