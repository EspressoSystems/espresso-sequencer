//! Abstraction over the contents of a block
//!
//! This module provides the [`Block`] trait, which describes the behaviors that a block is
//! expected to have.

use commit::{Commitment, Committable};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::{
    collections::HashSet,
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

/// Abstraction over the full contents of a block
///
/// This trait encapsulates the behaviors that a block must have in order to be used by consensus:
///   * Must have a predefined error type ([`Block::Error`])
///   * Must have a transaction type that can be compared for equality, serialized and serialized,
///     sent between threads, and can have a hash produced of it
///   * Must be able to be produced incrementally by appending transactions
///     ([`add_transaction_raw`](Block::add_transaction_raw))
///   * Must be hashable
pub trait Block:
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

    /// Construct an empty or genesis block.
    fn new() -> Self;

    /// Attempts to add a transaction, returning an Error if it would result in a structurally
    /// invalid block
    ///
    /// # Errors
    ///
    /// Should return an error if this transaction leads to an invalid block
    fn add_transaction_raw(&self, tx: &Self::Transaction)
        -> std::result::Result<Self, Self::Error>;

    /// returns hashes of all the transactions in this block
    /// TODO make this ordered with a vec
    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>>;
}

/// Commitment to a block, used by data availibity
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(bound(deserialize = ""), transparent)]
pub struct BlockCommitment<T: Block>(pub Commitment<T>);

/// Abstraction over any type of transaction. Used by [`Block`].
pub trait Transaction:
    Clone + Serialize + DeserializeOwned + Debug + PartialEq + Eq + Sync + Send + Committable + Hash
{
}

/// Dummy implementation of `BlockContents` for unit tests
pub mod dummy {
    use std::fmt::Display;

    use super::{Block, Commitment, Committable, Debug, Hash, HashSet, Serialize};
    use rand::Rng;
    use serde::Deserialize;

    pub use crate::traits::state::dummy::DummyState;
    use crate::traits::state::TestableBlock;

    /// The dummy block
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DummyBlock {
        /// Some dummy data
        pub nonce: u64,
    }

    impl DummyBlock {
        /// Generate a random `DummyBlock`
        pub fn random(rng: &mut dyn rand::RngCore) -> Self {
            Self { nonce: rng.gen() }
        }
    }

    /// Dummy error
    #[derive(Debug)]
    pub struct DummyError;

    /// dummy transaction. No functionality
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
    pub enum DummyTransaction {
        /// the only variant. Dummy.
        Dummy,
    }

    impl Committable for DummyTransaction {
        fn commit(&self) -> commit::Commitment<Self> {
            commit::RawCommitmentBuilder::new("Dummy Block Comm")
                .u64_field("Dummy Field", 0)
                .finalize()
        }

        fn tag() -> String {
            "DUMMY_TXN".to_string()
        }
    }
    impl super::Transaction for DummyTransaction {}

    impl std::error::Error for DummyError {}

    impl std::fmt::Display for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("A bad thing happened")
        }
    }

    impl Display for DummyBlock {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:#?}")
        }
    }

    impl Block for DummyBlock {
        type Error = DummyError;

        type Transaction = DummyTransaction;

        fn new() -> Self {
            <Self as TestableBlock>::genesis()
        }

        fn add_transaction_raw(
            &self,
            _tx: &Self::Transaction,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(Self {
                nonce: self.nonce + 1,
            })
        }

        fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
            HashSet::new()
        }
    }

    impl TestableBlock for DummyBlock {
        fn genesis() -> Self {
            Self { nonce: 0 }
        }

        fn txn_count(&self) -> u64 {
            1
        }
    }

    impl Committable for DummyBlock {
        fn commit(&self) -> commit::Commitment<Self> {
            commit::RawCommitmentBuilder::new("Dummy Block Comm")
                .u64_field("Nonce", self.nonce)
                .finalize()
        }

        fn tag() -> String {
            "DUMMY_BLOCK".to_string()
        }
    }
}
