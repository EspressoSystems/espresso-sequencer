//! This module provides implementations of block traits for examples and tests only.
//! TODO (Keyao) Organize non-production code.
//! <https://github.com/EspressoSystems/HotShot/issues/2059>
use std::{
    fmt::{Debug, Display},
    mem::size_of,
};

use crate::{
    data::{BlockError, VidCommitment, VidScheme, VidSchemeTrait},
    traits::{
        block_contents::{vid_commitment, BlockHeader, Transaction},
        state::TestableBlock,
        BlockPayload,
    },
};
use commit::{Commitment, Committable};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// The transaction in a [`VIDBlockPayload`].
#[derive(Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct VIDTransaction(pub Vec<u8>);

impl VIDTransaction {
    /// Encode a list of transactions into bytes.
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    pub fn encode(transactions: Vec<Self>) -> Result<Vec<u8>, BlockError> {
        let mut encoded = Vec::new();

        for txn in transactions {
            // The transaction length is converted from `usize` to `u32` to ensure consistent
            // number of bytes on different platforms.
            let txn_size = match u32::try_from(txn.0.len()) {
                Ok(len) => len.to_le_bytes(),
                Err(_) => {
                    return Err(BlockError::InvalidTransactionLength);
                }
            };

            // Concatenate the bytes of the transaction size and the transaction itself.
            encoded.extend(txn_size);
            encoded.extend(txn.0);
        }

        Ok(encoded)
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
    /// Create a genesis block payload with transaction bytes `vec![0]`, to be used for
    /// consensus task initiation.
    /// # Panics
    /// If the `VidScheme` construction fails.
    #[must_use]
    pub fn genesis() -> Self {
        let txns: Vec<u8> = vec![0];
        // It's impossible for `encode` to fail because the transaciton length is very small.
        let encoded = VIDTransaction::encode(vec![VIDTransaction(txns.clone())]).unwrap();
        VIDBlockPayload {
            transactions: vec![VIDTransaction(txns)],
            payload_commitment: vid_commitment(encoded),
        }
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
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let txns_vec: Vec<VIDTransaction> = transactions.into_iter().collect();
        let encoded = VIDTransaction::encode(txns_vec.clone())?;
        Ok((
            Self {
                transactions: txns_vec,
                payload_commitment: vid_commitment(encoded),
            },
            (),
        ))
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
            payload_commitment: vid_commitment(encoded_vec),
        }
    }

    fn genesis() -> (Self, Self::Metadata) {
        (Self::genesis(), ())
    }

    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(VIDTransaction::encode(self.transactions.clone())?.into_iter())
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
    pub payload_commitment: VidCommitment,
}

impl BlockHeader for VIDBlockHeader {
    type Payload = VIDBlockPayload;

    fn new(
        payload_commitment: VidCommitment,
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
                payload_commitment: payload.payload_commitment,
            },
            payload,
            metadata,
        )
    }

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment
    }

    fn metadata(&self) -> <Self::Payload as BlockPayload>::Metadata {}
}
