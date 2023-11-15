//! This module provides implementations of block traits for examples and tests only.
//! TODO (Keyao) Organize non-production code.
//! <https://github.com/EspressoSystems/HotShot/issues/2059>
use std::{
    fmt::{Debug, Display},
    mem::size_of,
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
    ///
    /// # Panics
    /// If the conversion from the transaction length to `u32` fails.
    pub fn encode(transactions: Vec<Self>) -> Vec<u8> {
        let mut encoded = Vec::new();

        for txn in transactions {
            // The transaction length is converted from `usize` to `u32` to ensure consistent
            // number of bytes on different platforms.
            let txn_size = u32::try_from(txn.0.len())
                .expect("Conversion fails")
                .to_le_bytes();

            // Concatenate the bytes of the transaction size and the transaction itself.
            encoded.extend(txn_size);
            encoded.extend(txn.0);
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
        let vid = VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, srs).unwrap();
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
    type Encode<'a> = <Vec<u8> as IntoIterator>::IntoIter;

    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> (Self, Self::Metadata) {
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

    fn from_bytes<E>(encoded_transactions: E, _metadata: Self::Metadata) -> Self
    where
        E: Iterator<Item = u8>,
    {
        let encoded_vec: Vec<u8> = encoded_transactions.collect();
        let mut transactions = Vec::new();
        let mut current_index = 0;
        while current_index < encoded_vec.len() {
            // Decode the transaction length.
            let txn_start_index = current_index + size_of::<u32>();
            let mut txn_len_bytes = [0; size_of::<u32>()];
            txn_len_bytes.copy_from_slice(&encoded_vec[current_index..txn_start_index]);
            let txn_len: usize = u32::from_le_bytes(txn_len_bytes) as usize;

            // Get the transaction.
            let next_index = txn_start_index + txn_len;
            transactions.push(VIDTransaction(
                encoded_vec[txn_start_index..next_index].to_vec(),
            ));
            current_index = next_index;
        }

        Self {
            transactions,
            payload_commitment: Self::vid_commitment(&encoded_vec),
        }
    }

    fn genesis() -> (Self, Self::Metadata) {
        (Self::genesis(), ())
    }

    fn encode(&self) -> Self::Encode<'_> {
        VIDTransaction::encode(self.transactions.clone()).into_iter()
    }

    fn transaction_commitments(&self) -> Vec<Commitment<Self::Transaction>> {
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

    fn genesis() -> (
        Self,
        Self::Payload,
        <Self::Payload as BlockPayload>::Metadata,
    ) {
        let (payload, metadata) = <Self::Payload as BlockPayload>::genesis();
        (
            Self {
                block_number: 0,
                payload_commitment: payload.commit(),
            },
            payload,
            metadata,
        )
    }

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn payload_commitment(&self) -> Commitment<Self::Payload> {
        self.payload_commitment
    }
}
