//! Abstraction over the global state that blocks modify
//!
//! This module provides the [`State`] trait, which serves as an compatibility over the current
//! network state, which is modified by the transactions contained within blocks.

use crate::traits::BlockPayload;
use ark_ff::PrimeField;
use commit::Committable;
use espresso_systems_common::hotshot::tag;
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

    /// Returns true if and only if the provided block header is valid and can extend this state
    fn validate_block(&self, block_header: &Self::BlockHeader, view_number: &Self::Time) -> bool;

    /// Initialize the state.
    fn initialize() -> Self;

    /// Appends the given block header to this state, returning an new state
    ///
    /// # Errors
    ///
    /// Should produce and error if appending this block header would lead to an invalid state
    fn append(
        &self,
        block_header: &Self::BlockHeader,
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

/// Dummy implementation of `State` for unit tests
pub mod dummy {
    use super::{tag, Committable, Debug, Hash, Serialize, State, TestableState};
    use crate::{
        block_impl::{VIDBlockHeader, VIDBlockPayload, VIDTransaction},
        data::ViewNumber,
    };
    use rand::Rng;
    use serde::Deserialize;

    /// Dummy error
    #[derive(Debug)]
    pub struct DummyError;

    impl std::error::Error for DummyError {}

    impl std::fmt::Display for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("A bad thing happened")
        }
    }

    /// The dummy state
    #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DummyState {
        /// Some dummy data
        nonce: u64,
    }

    impl Committable for DummyState {
        fn commit(&self) -> commit::Commitment<Self> {
            commit::RawCommitmentBuilder::new("Dummy State Comm")
                .u64_field("Nonce", self.nonce)
                .finalize()
        }

        fn tag() -> String {
            tag::DUMMY_STATE.to_string()
        }
    }

    impl DummyState {
        /// Generate a random `DummyState`
        pub fn random(r: &mut dyn rand::RngCore) -> Self {
            Self {
                nonce: r.gen_range(1..1_000_000),
            }
        }
    }

    impl State for DummyState {
        type Error = DummyError;
        type BlockHeader = VIDBlockHeader;
        type BlockPayload = VIDBlockPayload;
        type Time = ViewNumber;

        fn validate_block(
            &self,
            _block_header: &Self::BlockHeader,
            _view_number: &Self::Time,
        ) -> bool {
            false
        }

        fn initialize() -> Self {
            let mut state = Self::default();
            state.nonce += 1;
            state
        }

        fn append(
            &self,
            _block_header: &Self::BlockHeader,
            _view_number: &Self::Time,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                nonce: self.nonce + 1,
            })
        }

        fn on_commit(&self) {}
    }

    impl TestableState for DummyState {
        fn create_random_transaction(
            _state: Option<&Self>,
            _: &mut dyn rand::RngCore,
            _: u64,
        ) -> VIDTransaction {
            VIDTransaction(vec![0u8])
        }
    }
}

/// A serialized light client state for proof generation
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LightClientState<F: PrimeField> {
    /// Current view number
    pub view_number: usize,
    /// Current block height
    pub block_height: usize,
    /// Block commitment
    pub block_comm: F,
    /// Commitment for fee ledger
    pub fee_ledger_comm: F,
    /// Commitment for the stake table
    pub stake_table_comm: (F, F, F),
}

impl<F: PrimeField> From<LightClientState<F>> for [F; 7] {
    fn from(state: LightClientState<F>) -> Self {
        [
            F::from(state.view_number as u64),
            F::from(state.block_height as u64),
            state.block_comm,
            state.fee_ledger_comm,
            state.stake_table_comm.0,
            state.stake_table_comm.1,
            state.stake_table_comm.2,
        ]
    }
}
