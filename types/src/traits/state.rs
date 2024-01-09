//! Abstraction over the global state that blocks modify
//!
//! This module provides the [`State`] trait, which serves as an compatibility over the current
//! network state, which is modified by the transactions contained within blocks.

use crate::traits::BlockPayload;
use commit::Committable;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fmt::Debug,
    hash::Hash,
    ops,
    ops::{Deref, Sub},
};

use super::block_contents::BlockHeader;

/// Abstraction over the state that blocks modify
///
/// This trait represents the behaviors that the 'global' ledger state must have:
///   * A defined error type ([`Error`](State::Error))
///   * The type of block that modifies this type of state ([`BlockPayload`](State::BlockPayload))
///   * The ability to validate that a block is actually a valid extension of this state
///     ([`validate_block`](State::validate_block))
///   * The ability to produce a new state, with the modifications from the block applied
///     ([`append`](State::append))
pub trait State:
    Serialize
    + DeserializeOwned
    + Clone
    + Debug
    + Default
    + Hash
    + PartialEq
    + Eq
    + Send
    + Sync
    + Committable
{
    /// The error type for this particular type of ledger state
    type Error: Error + Debug + Send + Sync;
    /// The type of block header this state is associated with
    type BlockHeader: BlockHeader;
    /// The type of block payload this state is associated with
    type BlockPayload: BlockPayload;
    /// Time compatibility needed for reward collection
    type Time: ConsensusTime;

    /// Returns true if and only if the current block header is valid and can extend this state.
    fn validate_block(
        &self,
        curr_block_header: &Self::BlockHeader,
        prev_block_header: &Self::BlockHeader,
        view_number: &Self::Time,
    ) -> bool;

    /// Initialize the state.
    fn initialize() -> Self;

    /// Appends the given block header to this state, returning an new state
    ///
    /// # Errors
    ///
    /// Should produce and error if appending this block header would lead to an invalid state
    fn append(
        &self,
        curr_block_header: &Self::BlockHeader,
        prev_block_header: &Self::BlockHeader,
        view_number: &Self::Time,
    ) -> Result<Self, Self::Error>;

    /// Gets called to notify the persistence backend that this state has been committed
    fn on_commit(&self);
}

// TODO Seuqnecing here means involving DA in consensus

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

/// extra functions required on state to be usable by hotshot-testing
pub trait TestableState: State
where
    <Self as State>::BlockPayload: TestableBlock,
{
    /// Creates random transaction if possible
    /// otherwise panics
    /// `padding` is the bytes of padding to add to the transaction
    fn create_random_transaction(
        state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <Self::BlockPayload as BlockPayload>::Transaction;
}

/// extra functions required on block to be usable by hotshot-testing
pub trait TestableBlock: BlockPayload + Debug {
    /// generate a genesis block
    fn genesis() -> Self;

    /// the number of transactions in this block
    fn txn_count(&self) -> u64;
}
