//! This module provides an implementation of the `HotShot` suite of traits.
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    ops::Deref,
};

use commit::{Commitment, Committable};
use hotshot_types::traits::{block_contents::Transaction, state::TestableBlock, BlockPayload};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

/// The transaction in a [`VIDBlockPayload`].
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDTransaction {
    /// identifier for the transaction
    pub id: u64,
    /// padding to add to txn (to make it larger and thereby more realistic)
    pub padding: Vec<u8>,
}

impl Deref for VIDTransaction {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl Committable for VIDTransaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("SDemo Txn Comm")
            .u64_field("id", self.id)
            .finalize()
    }

    fn tag() -> String {
        "SEQUENCING_DEMO_TXN".to_string()
    }
}

impl Transaction for VIDTransaction {}

impl VIDTransaction {
    /// create a new transaction
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            padding: vec![],
        }
    }
}

/// The error type for block payload.
#[derive(Snafu, Debug)]
pub enum BlockPayloadError {
    /// Previous state commitment does not match
    PreviousStateMismatch,
    /// Nonce was reused
    ReusedTxn,
    /// Genesis failure
    GenesisFailed,
    /// Genesis reencountered after initialization
    GenesisAfterStart,
    /// no transasctions added to genesis
    GenesisCantHaveTransactions,
    /// invalid block
    InvalidBlock,
}

/// genesis block
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct GenesisBlockPayload {}

/// Any block after genesis
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct NormalBlockPayload {
    /// [`BlockPayload`] state commitment
    pub previous_state: (),
    /// [`VIDTransaction`] vector
    pub transactions: Vec<VIDTransaction>,
}

/// The block for the sequencing demo
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub enum VIDBlockPayload {
    /// genesis block payload
    Genesis(GenesisBlockPayload),
    /// normal block payload
    Normal(NormalBlockPayload),
}

impl Committable for VIDBlockPayload {
    fn commit(&self) -> Commitment<Self> {
        match &self {
            VIDBlockPayload::Genesis(_) => {
                commit::RawCommitmentBuilder::new("Genesis Comm").finalize()
            }
            VIDBlockPayload::Normal(block) => {
                let mut builder = commit::RawCommitmentBuilder::new("Normal Comm");
                for txn in &block.transactions {
                    builder = builder.u64_field("transaction", **txn);
                }
                builder.finalize()
            }
        }
    }

    fn tag() -> String {
        "VID_BLOCK_PAYLOAD".to_string()
    }
}

impl Display for VIDBlockPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VIDBlockPayload::Genesis(_) => {
                write!(f, "Genesis BlockPayload")
            }
            VIDBlockPayload::Normal(block) => {
                write!(f, "Normal BlockPayload #txns={}", block.transactions.len())
            }
        }
    }
}

impl TestableBlock for VIDBlockPayload {
    fn genesis() -> Self {
        VIDBlockPayload::Genesis(GenesisBlockPayload {})
    }

    fn txn_count(&self) -> u64 {
        match self {
            VIDBlockPayload::Genesis(_) => 0,
            VIDBlockPayload::Normal(n) => n.transactions.len() as u64,
        }
    }
}

impl BlockPayload for VIDBlockPayload {
    type Error = BlockPayloadError;

    type Transaction = VIDTransaction;

    fn new() -> Self {
        <Self as TestableBlock>::genesis()
    }

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        match self {
            VIDBlockPayload::Genesis(_) => Err(BlockPayloadError::GenesisCantHaveTransactions),
            VIDBlockPayload::Normal(n) => {
                let mut new = n.clone();
                new.transactions.push(tx.clone());
                Ok(VIDBlockPayload::Normal(new))
            }
        }
    }

    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
        match self {
            VIDBlockPayload::Genesis(_) => HashSet::new(),
            VIDBlockPayload::Normal(n) => n
                .transactions
                .iter()
                .map(commit::Committable::commit)
                .collect(),
        }
    }
}
