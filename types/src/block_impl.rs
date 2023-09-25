//! This module provides an implementation of the `HotShot` suite of traits.
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use crate::{
    data::{test_srs, VidScheme, VidSchemeTrait},
    traits::{block_contents::Transaction, state::TestableBlock, BlockPayload},
};
use commit::{Commitment, Committable};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use snafu::Snafu;

/// The transaction in a [`VIDBlockPayload`].
#[derive(Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDTransaction(pub Vec<u8>);

impl Committable for VIDTransaction {
    fn commit(&self) -> Commitment<Self> {
        let builder = commit::RawCommitmentBuilder::new("Txn Comm");
        let mut hasher = Keccak256::new();
        hasher.update(&self.0);
        let generic_array = hasher.finalize();
        builder.generic_byte_array(&generic_array).finalize()
    }

    fn tag() -> String {
        "SEQUENCING_TXN".to_string()
    }
}

impl Transaction for VIDTransaction {
    fn bytes(&self) -> Vec<u8> {
        self.0.clone()
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
pub struct VIDBlockPayload {
    /// List of transactions.
    pub transactions: Vec<VIDTransaction>,
    /// VID commitment.
    pub commitment: <VidScheme as VidSchemeTrait>::Commit,
}

impl VIDBlockPayload {
    /// Constructor.
    #[must_use]
    pub fn new(
        transactions: Vec<VIDTransaction>,
        commitment: <VidScheme as VidSchemeTrait>::Commit,
    ) -> Self {
        Self {
            transactions,
            commitment,
        }
    }

    /// Create a genesis block payload with transaction bytes `vec![0]`.
    /// # Panics
    /// If the `VidScheme` construction fails.
    #[must_use]
    pub fn genesis() -> Self {
        // TODO <https://github.com/EspressoSystems/HotShot/issues/1693>
        const NUM_STORAGE_NODES: usize = 10;
        // TODO <https://github.com/EspressoSystems/HotShot/issues/1693>
        const NUM_CHUNKS: usize = 5;
        // TODO <https://github.com/EspressoSystems/HotShot/issues/1686>
        let srs = test_srs(NUM_STORAGE_NODES);
        let vid = VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, &srs).unwrap();
        let txn = vec![0];
        let vid_disperse = vid.disperse(&txn).unwrap();
        VIDBlockPayload::new(vec![VIDTransaction(txn)], vid_disperse.commit)
    }
}

impl Committable for VIDBlockPayload {
    fn commit(&self) -> Commitment<Self> {
        let builder = commit::RawCommitmentBuilder::new("BlockPayload Comm");
        builder.generic_byte_array(&self.commitment).finalize()
    }

    fn tag() -> String {
        "VID_BLOCK_PAYLOAD".to_string()
    }
}

impl Display for VIDBlockPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockPayload #txns={}", self.transactions.len())
    }
}

impl TestableBlock for VIDBlockPayload {
    fn genesis() -> Self {
        Self::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transactions.len() as u64
    }
}

impl BlockPayload for VIDBlockPayload {
    type Error = BlockPayloadError;

    type Transaction = VIDTransaction;

    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
        self.transactions
            .iter()
            .map(commit::Committable::commit)
            .collect()
    }
}
