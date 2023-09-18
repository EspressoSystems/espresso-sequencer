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
    /// invalid block
    InvalidBlock,
}

/// A [`BlockPayload`] that contains a list of `VIDTransaction`.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDBlockPayload(pub Vec<VIDTransaction>);

impl Committable for VIDBlockPayload {
    fn commit(&self) -> Commitment<Self> {
        let mut builder = commit::RawCommitmentBuilder::new("Normal Comm");
        for txn in &self.0 {
            builder = builder.u64_field("transaction", **txn);
        }
        builder.finalize()
    }

    fn tag() -> String {
        "VID_BLOCK_PAYLOAD".to_string()
    }
}

impl Display for VIDBlockPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockPayload #txns={}", self.0.len())
    }
}

impl TestableBlock for VIDBlockPayload {
    fn genesis() -> Self {
        VIDBlockPayload(Vec::new())
    }

    fn txn_count(&self) -> u64 {
        self.0.len() as u64
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
        let mut new = self.0.clone();
        new.push(tx.clone());
        Ok(VIDBlockPayload(new))
    }

    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
        self.0.iter().map(commit::Committable::commit).collect()
    }
}
