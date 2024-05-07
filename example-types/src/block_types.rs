use std::{
    fmt::{Debug, Display},
    mem::size_of,
    sync::Arc,
};

use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_types::{
    data::{BlockError, Leaf},
    traits::{
        block_contents::{BlockHeader, BuilderFee, EncodeBytes, TestableBlock, Transaction},
        node_implementation::NodeType,
        BlockPayload, ValidatedState,
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidCommon},
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use time::OffsetDateTime;

use crate::{node_types::TestTypes, state_types::TestInstanceState};

/// The transaction in a [`TestBlockPayload`].
#[derive(Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct TestTransaction(pub Vec<u8>);

impl TestTransaction {
    /// Encode a list of transactions into bytes.
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    pub fn encode(transactions: &[Self]) -> Result<Vec<u8>, BlockError> {
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
            encoded.extend(&txn.0);
        }

        Ok(encoded)
    }
}

impl Committable for TestTransaction {
    fn commit(&self) -> Commitment<Self> {
        let builder = committable::RawCommitmentBuilder::new("Txn Comm");
        let mut hasher = Keccak256::new();
        hasher.update(&self.0);
        let generic_array = hasher.finalize();
        builder.generic_byte_array(&generic_array).finalize()
    }

    fn tag() -> String {
        "TEST_TXN".to_string()
    }
}

impl Transaction for TestTransaction {}

/// A [`BlockPayload`] that contains a list of `TestTransaction`.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct TestBlockPayload {
    /// List of transactions.
    pub transactions: Vec<TestTransaction>,
}

impl TestBlockPayload {
    /// Create a genesis block payload with bytes `vec![0]`, to be used for
    /// consensus task initiation.
    /// # Panics
    /// If the `VidScheme` construction fails.
    #[must_use]
    pub fn genesis() -> Self {
        TestBlockPayload {
            transactions: vec![],
        }
    }
}

impl Display for TestBlockPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockPayload #txns={}", self.transactions.len())
    }
}

impl TestableBlock for TestBlockPayload {
    fn genesis() -> Self {
        Self::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transactions.len() as u64
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestMetadata;

impl EncodeBytes for TestMetadata {
    fn encode(&self) -> Arc<[u8]> {
        Arc::new([])
    }
}

impl BlockPayload for TestBlockPayload {
    type Error = BlockError;
    type Instance = TestInstanceState;
    type Transaction = TestTransaction;
    type Metadata = TestMetadata;

    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
        _instance_state: &Self::Instance,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let txns_vec: Vec<TestTransaction> = transactions.into_iter().collect();
        Ok((
            Self {
                transactions: txns_vec,
            },
            TestMetadata,
        ))
    }

    fn from_bytes(encoded_transactions: &[u8], _metadata: &Self::Metadata) -> Self {
        let mut transactions = Vec::new();
        let mut current_index = 0;
        while current_index < encoded_transactions.len() {
            // Decode the transaction length.
            let txn_start_index = current_index + size_of::<u32>();
            let mut txn_len_bytes = [0; size_of::<u32>()];
            txn_len_bytes.copy_from_slice(&encoded_transactions[current_index..txn_start_index]);
            let txn_len: usize = u32::from_le_bytes(txn_len_bytes) as usize;

            // Get the transaction.
            let next_index = txn_start_index + txn_len;
            transactions.push(TestTransaction(
                encoded_transactions[txn_start_index..next_index].to_vec(),
            ));
            current_index = next_index;
        }

        Self { transactions }
    }

    fn genesis() -> (Self, Self::Metadata) {
        (Self::genesis(), TestMetadata)
    }

    fn encode(&self) -> Result<Arc<[u8]>, Self::Error> {
        TestTransaction::encode(&self.transactions).map(Arc::from)
    }

    fn builder_commitment(&self, _metadata: &Self::Metadata) -> BuilderCommitment {
        let mut digest = sha2::Sha256::new();
        for txn in &self.transactions {
            digest.update(&txn.0);
        }
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn get_transactions<'a>(
        &'a self,
        _metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction> {
        self.transactions.iter().cloned()
    }
}

/// A [`BlockHeader`] that commits to [`TestBlockPayload`].
#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub struct TestBlockHeader {
    /// Block number.
    pub block_number: u64,
    /// VID commitment to the payload.
    pub payload_commitment: VidCommitment,
    /// Fast commitment for builder verification
    pub builder_commitment: BuilderCommitment,
    /// Timestamp when this header was created.
    pub timestamp: u64,
}

impl<TYPES: NodeType<BlockHeader = Self, BlockPayload = TestBlockPayload>> BlockHeader<TYPES>
    for TestBlockHeader
{
    type Error = std::convert::Infallible;

    async fn new(
        _parent_state: &TYPES::ValidatedState,
        _instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        parent_leaf: &Leaf<TYPES>,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        _metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
        _builder_fee: BuilderFee<TYPES>,
        _vid_common: VidCommon,
    ) -> Result<Self, Self::Error> {
        let parent = parent_leaf.get_block_header();

        let mut timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
        if timestamp < parent.timestamp {
            // Prevent decreasing timestamps.
            timestamp = parent.timestamp;
        }

        Ok(Self {
            block_number: parent.block_number + 1,
            payload_commitment,
            builder_commitment,
            timestamp,
        })
    }

    fn genesis(
        _instance_state: &<TYPES::ValidatedState as ValidatedState<TYPES>>::Instance,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        _metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    ) -> Self {
        Self {
            block_number: 0,
            payload_commitment,
            builder_commitment,
            timestamp: 0,
        }
    }

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment
    }

    fn metadata(&self) -> &<TYPES::BlockPayload as BlockPayload>::Metadata {
        &TestMetadata
    }

    fn builder_commitment(&self) -> BuilderCommitment {
        self.builder_commitment.clone()
    }
}

impl Committable for TestBlockHeader {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("Header Comm")
            .u64_field(
                "block number",
                <TestBlockHeader as BlockHeader<TestTypes>>::block_number(self),
            )
            .constant_str("payload commitment")
            .fixed_size_bytes(
                <TestBlockHeader as BlockHeader<TestTypes>>::payload_commitment(self)
                    .as_ref()
                    .as_ref(),
            )
            .finalize()
    }

    fn tag() -> String {
        "TEST_HEADER".to_string()
    }
}
