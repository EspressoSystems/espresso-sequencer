//! This module provides an implementation of the `HotShot` suite of traits.
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use crate::{
    data::{test_srs, VidScheme, VidSchemeTrait},
    traits::{
        block_contents::{BlockHeader, Transaction},
        state::TestableBlock,
        BlockPayload,
    },
};
use ark_serialize::CanonicalDeserialize;
use commit::{Commitment, Committable};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use snafu::Snafu;

// TODO <https://github.com/EspressoSystems/HotShot/issues/1693>
/// Number of storage nodes for VID initiation.
pub const NUM_STORAGE_NODES: usize = 8;
// TODO <https://github.com/EspressoSystems/HotShot/issues/1693>
/// Number of chunks for VID initiation.
pub const NUM_CHUNKS: usize = 8;

/// The error type for block and its transactions.
#[derive(Snafu, Debug)]
pub enum BlockError {
    /// Invalid block header.
    InvalidBlockHeader,
}

/// The transaction in a [`VIDBlockPayload`].
#[derive(Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDTransaction(pub Vec<u8>);

impl VIDTransaction {
    #[must_use]
    /// Encode a list of transactions into bytes.
    pub fn encode(transactions: Vec<Self>) -> Vec<u8> {
        let mut encoded = Vec::new();

        for txn in transactions {
            // Encode the length of the inner transaction and the transaction bytes.
            if let Ok(len) = txn.0.len().try_into() {
                encoded.extend::<Vec<u8>>(vec![len]);
                encoded.extend(txn.0);
            }
        }

        encoded
    }
}

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

impl Transaction for VIDTransaction {}

/// A [`BlockPayload`] that contains a list of `VIDTransaction`.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDBlockPayload {
    /// List of transactions.
    pub transactions: Vec<VIDTransaction>,
    /// VID commitment to the block payload.
    pub payload_commitment: <VidScheme as VidSchemeTrait>::Commit,
}

impl VIDBlockPayload {
    #[must_use]
    /// Compute the VID payload commitment.
    /// # Panics
    /// If the VID computation fails.
    pub fn vid_commitment(encoded_transactions: &[u8]) -> <VidScheme as VidSchemeTrait>::Commit {
        // TODO <https://github.com/EspressoSystems/HotShot/issues/1686>
        let srs = test_srs(NUM_STORAGE_NODES);
        // TODO We are using constant numbers for now, but they will change as the quorum size
        // changes.
        // TODO <https://github.com/EspressoSystems/HotShot/issues/1693>
        let vid = VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, &srs).unwrap();
        vid.disperse(encoded_transactions).unwrap().commit
    }

    /// Create a genesis block payload with transaction bytes `vec![0]`, to be used for
    /// consensus task initiation.
    /// # Panics
    /// If the `VidScheme` construction fails.
    #[must_use]
    pub fn genesis() -> Self {
        let txns: Vec<u8> = vec![0];
        let encoded = VIDTransaction::encode(vec![VIDTransaction(txns.clone())]);
        VIDBlockPayload {
            transactions: vec![VIDTransaction(txns)],
            payload_commitment: Self::vid_commitment(&encoded),
        }
    }
}

impl Committable for VIDBlockPayload {
    fn commit(&self) -> Commitment<Self> {
        <Commitment<Self> as CanonicalDeserialize>::deserialize(&*self.payload_commitment)
            .expect("conversion from VidScheme::Commit to Commitment should succeed")
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
    type Error = BlockError;
    type Transaction = VIDTransaction;
    type Metadata = ();

    fn build(transactions: impl IntoIterator<Item = Self::Transaction>) -> (Self, Self::Metadata) {
        let txns_vec: Vec<VIDTransaction> = transactions.into_iter().collect();
        let encoded = VIDTransaction::encode(txns_vec.clone());
        (
            Self {
                transactions: txns_vec,
                payload_commitment: Self::vid_commitment(&encoded),
            },
            (),
        )
    }

    fn encode(&self) -> Vec<u8> {
        VIDTransaction::encode(self.transactions.clone())
    }

    fn decode(encoded_transactions: &[u8]) -> Self {
        let mut transactions = Vec::new();
        let mut current_index = 0;
        while current_index < encoded_transactions.len() {
            let txn_len = encoded_transactions[current_index] as usize;
            let next_index = current_index + txn_len;
            transactions.push(VIDTransaction(
                encoded_transactions[current_index..next_index].to_vec(),
            ));
            current_index = next_index;
        }

        Self {
            transactions,
            payload_commitment: Self::vid_commitment(encoded_transactions),
        }
    }

    fn transaction_commitments(&self) -> HashSet<Commitment<Self::Transaction>> {
        self.transactions
            .iter()
            .map(commit::Committable::commit)
            .collect()
    }
}

/// A [`BlockHeader`] that commits to [`VIDBlockPayload`].
#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub struct VIDBlockHeader {
    /// Block number.
    pub block_number: u64,
    /// VID commitment to the payload.
    pub payload_commitment: Commitment<VIDBlockPayload>,
}

impl BlockHeader for VIDBlockHeader {
    type Payload = VIDBlockPayload;

    fn new(
        payload_commitment: Commitment<Self::Payload>,
        _metadata: <Self::Payload as BlockPayload>::Metadata,
        parent_header: &Self,
    ) -> Self {
        Self {
            block_number: parent_header.block_number + 1,
            payload_commitment,
        }
    }

    fn genesis(payload: Self::Payload) -> Self {
        Self {
            block_number: 0,
            payload_commitment: payload.commit(),
        }
    }

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn payload_commitment(&self) -> Commitment<Self::Payload> {
        self.payload_commitment
    }
}
