use crate::{
    block::payload2::{
        ns_id_as_bytes, ns_id_from_bytes, ns_offset_as_bytes, ns_offset_from_bytes,
        num_nss_as_bytes, NamespaceBuilder,
    },
    BlockBuildingSnafu, NamespaceId, Transaction,
};
use commit::{Commitment, Committable};
use hotshot_query_service::{availability::QueryablePayload, VidCommon};
use hotshot_types::{traits::BlockPayload, vid::VidSchemeType};
use hotshot_types::{utils::BuilderCommitment, vid::vid_scheme};
use jf_primitives::vid::{payload_prover::PayloadProver, VidScheme};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use snafu::OptionExt;
use std::{collections::HashMap, fmt::Display, ops::Range};

pub mod entry;
pub mod payload;
mod payload2;
pub mod queryable;
pub mod tables;
pub mod tx_iterator;

use entry::TxTableEntryWord;
use payload::Payload;
use tables::NameSpaceTable;

use self::{
    payload::NamespaceProof,
    payload2::{NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Payload2 {
    // Sequence of bytes representing the concatenated payloads for each namespace
    #[serde(with = "base64_bytes")]
    payload: Vec<u8>,

    // Sequence of bytes representing the namespace table
    ns_table: Vec<u8>,
    // TODO(X) Revisit caching of frequently used items
    //
    // TODO type should be `OnceLock<SmallRangeProofType>` instead of `OnceLock<Option<SmallRangeProofType>>`.
    // We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    // #[derivative(Hash = "ignore")]
    // #[derivative(PartialEq = "ignore")]
    // #[serde(skip)]
    // pub tx_table_len_proof: OnceLock<Option<SmallRangeProofType>>,
}

impl Display for Payload2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Payload2 {
    fn commit(&self) -> commit::Commitment<Self> {
        todo!()
    }
}

impl BlockPayload for Payload2 {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Metadata = Vec<u8>; // namespace table bytes

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        // add each tx to its namespace
        let mut namespaces = HashMap::<NamespaceId, NamespaceBuilder>::new();
        for tx in transactions.into_iter() {
            let namespace = namespaces.entry(tx.namespace()).or_default();
            namespace.append_tx(tx);
        }

        // build block payload and namespace table
        let mut payload = Vec::new();
        let mut ns_table = Vec::from(num_nss_as_bytes(namespaces.len()));
        for (ns_id, namespace) in namespaces {
            payload.extend(namespace.into_bytes());
            ns_table.extend(ns_id_as_bytes(ns_id));
            ns_table.extend(ns_offset_as_bytes(payload.len()));
        }
        Ok((
            Self {
                payload,
                ns_table: ns_table.clone(),
            },
            ns_table,
        ))
    }

    fn from_bytes<I>(_encoded_transactions: I, _metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        todo!()
    }

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn genesis() -> (Self, Self::Metadata) {
        todo!()
    }

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(self.payload.iter().cloned())
    }

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    fn transaction_commitments(
        &self,
        _metadata: &Self::Metadata,
    ) -> Vec<Commitment<Self::Transaction>> {
        todo!()
    }

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    fn builder_commitment(&self, _metadata: &Self::Metadata) -> BuilderCommitment {
        todo!()
    }

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    fn get_transactions(&self, _metadata: &Self::Metadata) -> &Vec<Self::Transaction> {
        todo!()
    }
}

impl Payload2 {
    // TODO dead code even with `pub` because this module is private in lib.rs
    // #[allow(dead_code)]
    /// Returns the flat bytes for namespace `ns_id`, along with a proof of correctness for those bytes.
    ///
    /// RPC-friendly proof contains:
    /// - the namespace bytes
    /// - `vid_common` needed to verify the proof. This data is not accessible to the verifier because it's not part of the block header.
    pub fn namespace_with_proof(
        &self,
        ns_id: NamespaceId,
        vid_common: VidCommon,
    ) -> Option<NamespaceProof> {
        if self.payload.len() != VidSchemeType::get_payload_byte_len(&vid_common) {
            return None; // error: vid_common inconsistent with self
        }

        let ns_range = if let Some(ns_range) = self.get_namespace_range(ns_id) {
            ns_range
        } else {
            return Some(NamespaceProof::NonExistence { ns_id });
        };

        // TODO log output for each `?`
        // fix this when we settle on an error handling pattern
        Some(NamespaceProof::Existence {
            ns_id,
            ns_payload_flat: self.payload[ns_range.clone()].into(),
            ns_proof: vid_scheme(VidSchemeType::get_num_storage_nodes(&vid_common))
                .payload_proof(&self.payload, ns_range)
                .ok()?,
            vid_common,
        })
    }

    // TODO newtype for `Range<usize>` for indexing into `self.payload` to protect against misuse?
    fn get_namespace_range(&self, ns_id: NamespaceId) -> Option<Range<usize>> {
        // find ns_id in ns_table
        let ns_index = (NUM_NSS_BYTE_LEN..self.ns_table.len())
            .step_by(NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN)
            .find(|&i| ns_id == ns_id_from_bytes(&self.ns_table[i..i + NS_ID_BYTE_LEN]))?;

        // read (start,end) offsets for ns_id from ns_table
        let ns_end = self.ns_offset(ns_index);
        let ns_start = if ns_index == 0 {
            0
        } else {
            self.ns_offset(ns_index - 1)
        };

        // ensure range is valid and within payload byte length
        let ns_end = std::cmp::min(ns_end, self.payload.len());
        let ns_start = std::cmp::min(ns_start, ns_end);

        Some(ns_start..ns_end)
    }

    // TODO newtype for `ns_index` to protect against misuse?
    fn ns_offset(&self, ns_index: usize) -> usize {
        ns_offset_from_bytes(&self.ns_table[ns_table_offset_range(ns_index)])
    }
}

// TODO move this lower level?
// TODO newtype for `ns_index` to protect against misuse?
fn ns_table_offset_range(ns_index: usize) -> Range<usize> {
    let start =
        NUM_NSS_BYTE_LEN + (ns_index * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN)) + NS_ID_BYTE_LEN;
    start..start + NS_OFFSET_BYTE_LEN
}

// OLD: DELETE
pub type NsTable = NameSpaceTable<TxTableEntryWord>;

impl BlockPayload for Payload<TxTableEntryWord> {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Metadata = NsTable;

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    /// Returns (Self, metadata).
    ///
    /// `metadata` is a bytes representation of the namespace table.
    /// Why bytes? To make it easy to move metadata into payload in the future.
    ///
    /// Namespace table defined as follows for j>0:
    /// word[0]:    [number of entries in namespace table]
    /// word[2j-1]: [id for the jth namespace]
    /// word[2j]:   [end byte index of the jth namespace in the payload]
    ///
    /// Thus, for j>2 the jth namespace payload bytes range is word[2(j-1)]..word[2j].
    /// Edge case: for j=1 the jth namespace start index is implicitly 0.
    ///
    /// Word type is `TxTableEntry`.
    /// TODO(746) don't use `TxTableEntry`; make a different type for type safety.
    ///
    /// TODO final entry should be implicit:
    /// https://github.com/EspressoSystems/espresso-sequencer/issues/757
    ///
    /// TODO(746) refactor and make pretty "table" code for tx, namespace tables?
    fn from_transactions(
        txs: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let payload = Payload::from_txs(txs)?;
        let ns_table = payload.get_ns_table().clone(); // TODO don't clone ns_table
        Some((payload, ns_table)).context(BlockBuildingSnafu)
    }

    fn from_bytes<I>(encoded_transactions: I, metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self {
            raw_payload: encoded_transactions.into_iter().collect(),
            ns_table: metadata.clone(), // TODO don't clone ns_table
        }
    }

    fn genesis() -> (Self, Self::Metadata) {
        Self::from_transactions([]).unwrap()
    }

    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(self.raw_payload.iter().cloned())
    }

    fn transaction_commitments(&self, meta: &Self::Metadata) -> Vec<Commitment<Self::Transaction>> {
        self.enumerate(meta).map(|(_, tx)| tx.commit()).collect()
    }

    /// Generate commitment that builders use to sign block options.
    fn builder_commitment(&self, metadata: &Self::Metadata) -> BuilderCommitment {
        let mut digest = sha2::Sha256::new();
        digest.update((self.raw_payload.len() as u64).to_le_bytes());
        digest.update((self.ns_table.bytes.len() as u64).to_le_bytes());
        digest.update((metadata.bytes.len() as u64).to_le_bytes());
        digest.update(&self.raw_payload);
        digest.update(&self.ns_table.bytes);
        digest.update(&metadata.bytes);
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn get_transactions(&self, _: &Self::Metadata) -> &Vec<Self::Transaction> {
        unimplemented!()
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
        pub static ref NS_TABLE: Value = load_reference!("ns_table");
        pub static ref L1_BLOCK: Value = load_reference!("l1_block");
        pub static ref HEADER: Value = load_reference!("header");
        pub static ref TRANSACTION: Value = load_reference!("transaction");
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
    fn test_reference_ns_table() {
        reference_test::<NameSpaceTable<TxTableEntryWord>, _>(
            NS_TABLE.clone(),
            "NSTABLE~GL-lEBAwNZDldxDpySRZQChNnmn9vNzdIAL8W9ENOuh_",
            |ns_table| ns_table.commit(),
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
            "BLOCK~CltsD5AWVMRYoPCVoir_T8qU3qJTIxi5qBjyWu9vr-gC",
            |header| header.commit(),
        );
    }

    #[test]
    fn test_reference_transaction() {
        reference_test::<Transaction, _>(
            TRANSACTION.clone(),
            "COMMIT~77xOf9b3_RtGwqQ7_zOPeuJRS0iZwF7EJiV_NzOv4uID",
            |tx| tx.commit(),
        );
    }
}

#[cfg(test)]
mod test {
    use super::Payload2;
    use crate::{NamespaceId, Transaction};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use hotshot::traits::BlockPayload;
    use rand::{Rng, RngCore};

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8]], // 3 non-empty namespaces
        ];

        setup_logging();
        setup_backtrace();
        let mut rng = jf_utils::test_rng();
        let valid_tests = ValidTest::many_from_tx_lengths(test_cases, &mut rng);

        for test in valid_tests {
            let block = Payload2::from_transactions(test.txs).unwrap();
        }
    }

    struct ValidTest {
        txs: Vec<Transaction>,
    }

    impl ValidTest {
        fn from_tx_lengths<R>(tx_lengths: Vec<Vec<usize>>, rng: &mut R) -> Self
        where
            R: RngCore,
        {
            let mut txs = Vec::new();
            for (ns_index, tx_lens) in tx_lengths.into_iter().enumerate() {
                let ns_id = NamespaceId::from(ns_index as u64);
                for len in tx_lens {
                    txs.push(Transaction::new(ns_id, random_bytes(len, rng)));
                }
            }
            Self { txs }
        }

        fn many_from_tx_lengths<R>(test_cases: Vec<Vec<Vec<usize>>>, rng: &mut R) -> Vec<Self>
        where
            R: RngCore,
        {
            test_cases
                .into_iter()
                .map(|t| Self::from_tx_lengths(t, rng))
                .collect()
        }
    }

    fn random_bytes<R: RngCore>(len: usize, rng: &mut R) -> Vec<u8> {
        let mut result = vec![0; len];
        rng.fill_bytes(&mut result);
        result
    }
}
