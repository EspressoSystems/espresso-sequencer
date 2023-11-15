//! Abstraction over the contents of a block
//!
//! This module provides the [`Transaction`], [`BlockPayload`], and [`BlockHeader`] traits, which
//! describe the behaviors that a block is expected to have.

use commit::{Commitment, Committable};
use serde::{de::DeserializeOwned, Serialize};
use snafu::Snafu;

use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

/// The error type for block and its transactions.
#[derive(Snafu, Debug)]
pub enum BlockError {
    /// Invalid block header.
    InvalidBlockHeader,
    /// Invalid transaction length.
    InvalidTransactionLength,
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
    type Metadata: Clone + Debug + Eq + Hash + Send + Sync;

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
    ) -> Result<(Self, Self::Metadata), BlockError>;

    /// Build a payload with the encoded transaction bytes and metadata.
    ///
    /// `I` may be, but not necessarily is, the `Encode` type directly from `fn encode`.
    fn from_bytes<I>(encoded_transactions: I, metadata: Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>;

    /// Build the genesis payload and metadata.
    fn genesis() -> (Self, Self::Metadata);

    /// Encode the payload
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    fn encode(&self) -> Result<Self::Encode<'_>, BlockError>;

    /// List of transaction commitments.
    fn transaction_commitments(&self) -> Vec<Commitment<Self::Transaction>>;
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

    /// Build the genesis header, payload, and metadata.
    fn genesis() -> (
        Self,
        Self::Payload,
        <Self::Payload as BlockPayload>::Metadata,
    );

    /// Get the block number.
    fn block_number(&self) -> u64;

    /// Get the payload commitment.
    fn payload_commitment(&self) -> Commitment<Self::Payload>;
}
