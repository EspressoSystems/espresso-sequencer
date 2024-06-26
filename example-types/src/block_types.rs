use std::{
    fmt::{Debug, Display},
    mem::size_of,
    sync::Arc,
};

use async_trait::async_trait;
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
use snafu::Snafu;
use time::OffsetDateTime;
use vbs::version::Version;

use crate::{
    node_types::TestTypes,
    state_types::{TestInstanceState, TestValidatedState},
};

/// The transaction in a [`TestBlockPayload`].
#[derive(Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "Vec<u8>")]
pub struct TestTransaction(Vec<u8>);

#[derive(Debug, Snafu)]
pub struct TransactionTooLong;

impl TryFrom<Vec<u8>> for TestTransaction {
    type Error = TransactionTooLong;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_new(value).ok_or(TransactionTooLong)
    }
}

impl TestTransaction {
    /// Construct a new transaction
    ///
    /// # Panics
    /// If `bytes.len()` > `u32::MAX`
    pub fn new(bytes: Vec<u8>) -> Self {
        Self::try_new(bytes).expect("Vector too long")
    }

    /// Construct a new transaction.
    /// Returns `None` if `bytes.len()` > `u32::MAX`
    /// for cross-platform compatibility
    pub fn try_new(bytes: Vec<u8>) -> Option<Self> {
        if u32::try_from(bytes.len()).is_err() {
            None
        } else {
            Some(Self(bytes))
        }
    }

    /// Get reference to raw bytes of transaction
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }

    /// Convert transaction to raw vector of bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Encode a list of transactions into bytes.
    ///
    /// # Errors
    /// If the transaction length conversion fails.
    pub fn encode(transactions: &[Self]) -> Vec<u8> {
        let mut encoded = Vec::new();

        for txn in transactions {
            // The transaction length is converted from `usize` to `u32` to ensure consistent
            // number of bytes on different platforms.
            let txn_size = u32::try_from(txn.0.len())
                .expect("Invalid transaction length")
                .to_le_bytes();

            // Concatenate the bytes of the transaction size and the transaction itself.
            encoded.extend(txn_size);
            encoded.extend(&txn.0);
        }

        encoded
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

impl<TYPES: NodeType> TestableBlock<TYPES> for TestBlockPayload {
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

impl EncodeBytes for TestBlockPayload {
    fn encode(&self) -> Arc<[u8]> {
        TestTransaction::encode(&self.transactions).into()
    }
}

#[async_trait]
impl<TYPES: NodeType> BlockPayload<TYPES> for TestBlockPayload {
    type Error = BlockError;
    type Instance = TestInstanceState;
    type Transaction = TestTransaction;
    type Metadata = TestMetadata;
    type ValidatedState = TestValidatedState;

    async fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction> + Send,
        _validated_state: &Self::ValidatedState,
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

    fn empty() -> (Self, Self::Metadata) {
        (Self::genesis(), TestMetadata)
    }

    fn builder_commitment(&self, _metadata: &Self::Metadata) -> BuilderCommitment {
        let mut digest = sha2::Sha256::new();
        for txn in &self.transactions {
            digest.update(&txn.0);
        }
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn transactions<'a>(
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
        _metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
        _builder_fee: BuilderFee<TYPES>,
        _vid_common: VidCommon,
        _version: Version,
    ) -> Result<Self, Self::Error> {
        let parent = parent_leaf.block_header();

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
        _metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
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

    fn metadata(&self) -> &<TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata {
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
