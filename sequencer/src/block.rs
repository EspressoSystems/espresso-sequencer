use crate::{NMTRoot, NamespaceProofType, Transaction, TransactionNMT, VmId, MAX_NMT_DEPTH};
use commit::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::{traits::block_contents::BlockPayload, utils::BuilderCommitment};
use jf_primitives::merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, prelude::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Payload {
    #[serde(with = "nmt_serializer")]
    pub(crate) transaction_nmt: TransactionNMT,
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
        let nmt = TransactionNMT::from_elems(Some(MAX_NMT_DEPTH), leaves)
            .map_err(|_| de::Error::custom("Failed to build NMT from serialized leaves"))?;
        Ok(nmt)
    }
}

impl QueryablePayload for Payload {
    type TransactionIndex = u64;
    type InclusionProof = <TransactionNMT as MerkleTreeScheme>::MembershipProof;
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self, _meta: &Self::Metadata) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        _meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(txn, proof) => Some((txn.clone(), proof)),
            _ => None,
        }
    }

    fn iter(&self, meta: &Self::Metadata) -> Self::Iter<'_> {
        Box::new(0..self.len(meta) as u64)
    }
}

impl BlockPayload for Payload {
    type Error = bincode::Error;
    type Transaction = Transaction;
    type Metadata = NMTRoot;
    type Encode<'a> = <Vec<u8> as IntoIterator>::IntoIter;

    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let mut transactions = transactions.into_iter().collect::<Vec<_>>();
        transactions.sort_by_key(|tx| tx.vm());
        let transaction_nmt =
            TransactionNMT::from_elems(Some(MAX_NMT_DEPTH), transactions).unwrap();
        let root = NMTRoot {
            root: transaction_nmt.commitment().digest(),
        };
        Ok((Self { transaction_nmt }, root))
    }

    fn from_bytes<I>(encoded_transactions: I, _metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        // TODO for now, we panic if the transactions are not properly encoded. This only works as
        // long as all proposers are honest. We should soon replace this with the VID-specific
        // payload implementation in block2.rs.
        bincode::deserialize(encoded_transactions.collect::<Vec<u8>>().as_slice()).unwrap()
    }

    fn genesis() -> (Self, Self::Metadata) {
        Self::from_transactions([]).unwrap()
    }

    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(bincode::serialize(self)?.into_iter())
    }

    fn transaction_commitments(
        &self,
        metadata: &Self::Metadata,
    ) -> Vec<Commitment<Self::Transaction>> {
        self.enumerate(metadata)
            .map(|(_, tx)| tx.commit())
            .collect()
    }
    /// Generate commitment that builders use to sign block options.
    fn builder_commitment(&self, _metadata: &Self::Metadata) -> BuilderCommitment {
        unimplemented!("TODO builder_commitment");
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::block_contents::TestableBlock for Payload {
    fn genesis() -> Self {
        BlockPayload::genesis().0
    }

    fn txn_count(&self) -> u64 {
        self.transaction_nmt.num_leaves()
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Payload {
    /// Return namespace proof for a `V`, which can be used to extract the transactions for `V` in this block
    /// and the root of the NMT
    pub fn get_namespace_proof(&self, vm_id: VmId) -> NamespaceProofType {
        self.transaction_nmt.get_namespace_proof(vm_id)
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
    use crate::{Header, L1BlockInfo};
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
            "L1BLOCK~4HpzluLK2Isz3RdPNvNrDAyQcWOF2c9JeLZzVNLmfpQ9",
            |block| block.commit(),
        );
    }

    #[test]
    fn test_reference_header() {
        reference_test::<Header, _>(
            HEADER.clone(),
            "BLOCK~WFEGaS0Y4bxWAj6K6TZtjrfoHKGUNpJtXVH9skBg6Js_",
            |header| header.commit(),
        );
    }
}
