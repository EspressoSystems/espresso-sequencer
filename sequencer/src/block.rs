use crate::{Error, NMTRoot, NamespaceProofType, Transaction, TransactionNMT, VmId, MAX_NMT_DEPTH};
use ark_serialize::CanonicalSerialize;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::U256;
use hotshot::traits::Block as HotShotBlock;
use hotshot_query_service::QueryableBlock;
use hotshot_types::traits::state::TestableBlock;
use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NamespacedMerkleTreeScheme, AppendableMerkleTreeScheme, LookupResult,
    MerkleCommitment, MerkleTreeScheme,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::max;
use std::fmt::{Debug, Display};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    pub timestamp: u64,
    pub l1_block: L1BlockInfo,
    pub transactions_root: NMTRoot,
}

impl Header {
    pub fn commit(&self) -> Commitment<Block> {
        RawCommitmentBuilder::new(&Block::tag())
            .u64_field("timestamp", self.timestamp)
            .field("l1_block", self.l1_block.commit())
            .field("transactions_root", self.transactions_root.commit())
            .finalize()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    timestamp: u64,
    l1_block: L1BlockInfo,

    #[serde(with = "nmt_serializer")]
    pub(crate) transaction_nmt: TransactionNMT,
}

impl From<&Block> for Header {
    fn from(b: &Block) -> Self {
        Self {
            timestamp: b.timestamp,
            l1_block: b.l1_block,
            transactions_root: NMTRoot {
                root: b.transaction_nmt.commitment().digest(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub number: u64,
    pub timestamp: U256,
}

impl Committable for L1BlockInfo {
    fn commit(&self) -> Commitment<Self> {
        let mut timestamp = [0u8; 32];
        self.timestamp.to_little_endian(&mut timestamp);

        RawCommitmentBuilder::new(&Self::tag())
            .u64_field("number", self.number)
            // `RawCommitmentBuilder` doesn't have a `u256_field` method, so we simulate it:
            .constant_str("timestamp")
            .fixed_size_bytes(&timestamp)
            .finalize()
    }

    fn tag() -> String {
        "L1BLOCK".into()
    }
}

mod nmt_serializer {
    use super::*;

    // Serialize the NMT as a compact Vec<Transaction>
    pub fn serialize<S>(nmt: &TransactionNMT, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let leaves = nmt.leaves().cloned().collect::<Vec<Transaction>>();
        leaves.serialize(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TransactionNMT, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;

        let leaves = <Vec<Transaction>>::deserialize(deserializer)?;
        let nmt = TransactionNMT::from_elems(MAX_NMT_DEPTH, leaves)
            .map_err(|_| de::Error::custom("Failed to build NMT from serialized leaves"))?;
        Ok(nmt)
    }
}

impl QueryableBlock for Block {
    type TransactionIndex = u64;
    type InclusionProof = <TransactionNMT as MerkleTreeScheme>::MembershipProof;
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(txn, proof) => Some((txn, proof)),
            _ => None,
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Box::new(0..self.len() as u64)
    }
}

impl HotShotBlock for Block {
    type Error = Error;

    type Transaction = Transaction;

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        let mut new = self.clone();
        new.transaction_nmt
            .push(tx.clone())
            .map_err(|e| Error::MerkleTreeError {
                error: e.to_string(),
            })?;
        Ok(new)
    }

    fn contained_transactions(&self) -> std::collections::HashSet<Commitment<Self::Transaction>> {
        self.transaction_nmt
            .leaves()
            .map(|tx| tx.commit())
            .collect()
    }

    fn new() -> Self {
        // HotShot calls `new()` to create a "dummy" block for various reasons, so we just give it
        // the genesis block. Creating a real block, with a proper L1 block info, will be handled by
        // [`State::next_block`].
        Self::genesis()
    }
}

#[cfg(any(test, feature = "testing"))]
impl TestableBlock for Block {
    fn genesis() -> Self {
        Block::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transaction_nmt.num_leaves()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        Header::from(self).commit()
    }

    fn tag() -> String {
        "BLOCK".into()
    }
}

impl Committable for NMTRoot {
    fn commit(&self) -> Commitment<Self> {
        let mut comm_bytes = vec![];
        self.root
            .serialize_with_mode(&mut comm_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        RawCommitmentBuilder::new(&Self::tag())
            .var_size_field("root", &comm_bytes)
            .finalize()
    }

    fn tag() -> String {
        "NMTROOT".into()
    }
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            timestamp: 0,
            l1_block: L1BlockInfo {
                number: 0,
                timestamp: 0.into(),
            },
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }

    pub fn from_l1(l1_block: L1BlockInfo) -> Self {
        Self {
            // Enforce that the sequencer block timestamp is not behind the L1 block timestamp. This
            // can only happen if our clock is out of sync with L1.
            timestamp: max(
                OffsetDateTime::now_utc().unix_timestamp() as u64,
                l1_block.timestamp.as_u64(),
            ),
            l1_block,
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }

    /// Visit all transactions in this block.
    pub fn transactions(&self) -> impl ExactSizeIterator<Item = &Transaction> + '_ {
        self.transaction_nmt.leaves()
    }

    /// Return namespace proof for a `V`, which can be used to extract the transactions for `V` in this block
    /// and the root of the NMT
    pub fn get_namespace_proof(&self, vm_id: VmId) -> NamespaceProofType {
        self.transaction_nmt.get_namespace_proof(vm_id)
    }

    /// Currently, HotShot consensus does not enforce any relationship between
    /// the NMT root and the block commitment. This returns the NMT root of the block,
    /// mocking the consistency check between the block and NMT commitments.
    pub fn get_nmt_root(&self) -> NMTRoot {
        NMTRoot {
            root: self.transaction_nmt.commitment().digest(),
        }
    }

    /// The block's timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Information about the L1 block with which this sequencer block is synchronized.
    pub fn l1_block(&self) -> &L1BlockInfo {
        &self.l1_block
    }
}

#[cfg(test)]
mod reference {
    //! Reference data types.
    //!
    //! This module provides some reference instantiations of various data types which have an
    //! external, language-independent interface (e.g. commitment scheme). Ports of the sequencer to
    //! other languages, as well as downstream packages written in other languages, can use these
    //! references objects and their known commitments to check that their implementations of the
    //! commitment scheme are compatible with this reference implementation. To get the byte
    //! representation or U256 representation of a commitment for testing in other packages, run the
    //! tests and look for "commitment bytes" or "commitment U256" in the logs.
    //!
    //! For convenience, the reference objects are provided in serialized form, as they will appear
    //! in query service responses and the like, in the JSON files in the `data` directory of the
    //! repo for this crate. These JSON files are compiled into the crate binary and deserialized in
    //! this module to generate tests for the serialization format and commitment scheme.
    //!
    //! These tests may fail if you make a breaking change to a commitment scheme, serialization,
    //! etc. If this happens, be sure you _want_ to break the API, and, if so, simply replace the
    //! relevant constant in this module with the "actual" value that can be found in the logs of
    //! the failing test.

    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use lazy_static::lazy_static;
    use sequencer_utils::commitment_to_u256;
    use serde::de::DeserializeOwned;
    use serde_json::Value;

    macro_rules! load_reference {
        ($name:expr) => {
            serde_json::from_str(include_str!(std::concat!("../../data/", $name, ".json"))).unwrap()
        };
    }

    lazy_static! {
        pub static ref NMT_ROOT: Value = load_reference!("nmt_root");
        pub static ref L1_BLOCK: Value = load_reference!("l1_block");
        pub static ref HEADER: Value = load_reference!("header");
        pub static ref BLOCK: Value = load_reference!("block");
    }

    fn reference_test<T: DeserializeOwned, C: Committable>(
        reference: Value,
        expected: &str,
        commit: impl FnOnce(&T) -> Commitment<C>,
    ) {
        setup_logging();
        setup_backtrace();

        let reference: T = serde_json::from_value(reference).unwrap();
        let actual = commit(&reference);

        // Print information about the commitment that might be useful in generating tests for other
        // languages.
        let bytes: &[u8] = actual.as_ref();
        let u256 = commitment_to_u256(actual);
        tracing::info!("actual commitment: {}", actual);
        tracing::info!("commitment bytes: {:?}", bytes);
        tracing::info!("commitment U256: {}", u256);

        assert_eq!(actual, expected.parse().unwrap());
    }

    #[test]
    fn test_reference_nmt_root() {
        reference_test::<NMTRoot, _>(
            NMT_ROOT.clone(),
            "NMTROOT~-1Dow1sCihLw5x-sNsxaKtcqSLsPHIBDlXUacug5vgpx",
            |root| root.commit(),
        );
    }

    #[test]
    fn test_reference_l1_block() {
        reference_test::<L1BlockInfo, _>(
            L1_BLOCK.clone(),
            "L1BLOCK~i_2nsYHZCxW16USM7fnDryhITeIMDkshibzma7rhlsm-",
            |block| block.commit(),
        );
    }

    #[test]
    fn test_reference_header() {
        reference_test::<Header, _>(
            HEADER.clone(),
            "BLOCK~2xOyAKvho84kCCJUo2D4NOlV_lld-XGOX8xmZ0r6LWcB",
            |header| header.commit(),
        );
    }

    #[test]
    fn test_reference_block() {
        reference_test::<Block, _>(
            BLOCK.clone(),
            "BLOCK~2xOyAKvho84kCCJUo2D4NOlV_lld-XGOX8xmZ0r6LWcB",
            |block| block.commit(),
        );
    }

    #[test]
    fn test_header_block_commitment_equivalence() {
        let block: Block = serde_json::from_value(BLOCK.clone()).unwrap();
        assert_eq!(block.commit(), Header::from(&block).commit());
    }
}
