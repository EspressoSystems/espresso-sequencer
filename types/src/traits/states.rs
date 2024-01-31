//! Abstractions over the immutable instance-level state and hte global state that blocks modify.
//!
//! This module provides the [`InstanceState`] and [`ValidatedState`] traits, which serve as
//! compatibilities over the current network state, which is modified by the transactions contained
//! within blocks.

use super::block_contents::{BlockHeader, TestableBlock};
use crate::traits::{node_implementation::ConsensusTime, BlockPayload};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug, hash::Hash};

/// Instance-level state, which allows us to fetch missing validated state.
pub trait InstanceState: Clone + Debug + Send + Sync {}

/// Abstraction over the state that blocks modify
///
/// This trait represents the behaviors that the 'global' ledger state must have:
///   * A defined error type ([`Error`](ValidatedState::Error))
///   * The type of block that modifies this type of state ([`BlockPayload`](`ValidatedStates::
/// BlockPayload`))
///   * The ability to validate that a block header is actually a valid extension of this state and
/// produce a new state, with the modifications from the block applied
/// ([`validate_and_apply_header`](`ValidatedState::validate_and_apply_header))
pub trait ValidatedState:
    Serialize + DeserializeOwned + Clone + Debug + Default + Hash + PartialEq + Eq + Send + Sync
{
    /// The error type for this particular type of ledger state
    type Error: Error + Debug + Send + Sync;
    /// The type of the instance-level state this state is assocaited with
    type Instance: InstanceState;
    /// The type of block header this state is associated with
    type BlockHeader: BlockHeader<State = Self>;
    /// The type of block payload this state is associated with
    type BlockPayload: BlockPayload;
    /// Time compatibility needed for reward collection
    type Time: ConsensusTime;

    /// Check if the proposed block header is valid and apply it to the state if so.
    ///
    /// Returns the new state.
    ///
    /// # Arguments
    /// * `instance` - Immutable instance-level state.
    ///
    /// # Errors
    ///
    /// If the block header is invalid or appending it would lead to an invalid state.
    fn validate_and_apply_header(
        &self,
        instance: &Self::Instance,
        parent_header: &Self::BlockHeader,
        proposed_header: &Self::BlockHeader,
    ) -> Result<Self, Self::Error>;

    /// Construct the state with the given block header.
    ///
    /// This can also be used to rebuild the state for catchup.
    fn from_header(block_header: &Self::BlockHeader) -> Self;

    /// Construct a genesis validated state.
    #[must_use]
    fn genesis(instance: &Self::Instance) -> Self {
        Self::from_header(&Self::BlockHeader::genesis(instance).0)
    }

    /// Gets called to notify the persistence backend that this state has been committed
    fn on_commit(&self);
}

/// extra functions required on state to be usable by hotshot-testing
pub trait TestableState: ValidatedState
where
    <Self as ValidatedState>::BlockPayload: TestableBlock,
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
