use crate::block::entry::{TxTableEntry, TxTableEntryWord};
use crate::block::payload;
use crate::{
    bytes::{bytes, Bytes},
    BlockBuildingSnafu, Error, NamespaceId, Transaction,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use commit::Committable;
use derivative::Derivative;
use hotshot::traits::BlockPayload;
use hotshot_types::vid::{
    vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType,
};
use jf_primitives::vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use num_traits::PrimInt;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use std::default::Default;
use std::{collections::HashMap, fmt::Display};

use crate::block::tables::NameSpaceTable;
use trait_set::trait_set;

use crate::block::tables::TxTable;

trait_set! {

    pub trait TableWordTraits = CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
         + PrimInt
        + std::marker::Sync;

    // Note: this trait is not used yet as for now the Payload structs are only parametrized with the TableWord parameter.
    pub trait OffsetTraits = CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
        + std::marker::Sync;

    // Note: this trait is not used yet as for now the Payload structs are only parametrized with the TableWord parameter.
    pub trait NsIdTraits =CanonicalSerialize + CanonicalDeserialize + Default + std::marker::Sync;
}
pub(super) struct NamespaceInfo {
    // `tx_table` is a bytes representation of the following table:
    // word[0]: [number n of entries in tx table]
    // word[j>0]: [end byte index of the (j-1)th tx in the payload]
    //
    // Thus, the ith tx payload bytes range is word[i-1]..word[i].
    // Edge case: tx_table[-1] is implicitly 0.
    //
    // Word type is `TxTableEntry`.
    //
    // TODO final entry should be implicit:
    // https://github.com/EspressoSystems/espresso-sequencer/issues/757
    pub(crate) tx_table: Vec<u8>,
    pub(crate) tx_bodies: Vec<u8>, // concatenation of all tx payloads
    pub(crate) tx_bytes_end: TxTableEntry, // TODO make this field a usize instead
    pub(crate) tx_table_len: TxTableEntry, // TODO make this field a usize instead
}

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
// TODO remove the generic type param, use local constants instead
pub struct Payload<TableWord: TableWordTraits> {
    // Sequence of bytes representing the concatenated payloads for each namespace
    pub(super) raw_payload: Bytes,

    // Sequence of bytes representing the namespace table
    pub(super) ns_table: NameSpaceTable<TableWord>,
    // TODO(X) Revisit caching of frequently used items
    //
    // TODO type should be `OnceLock<SmallRangeProofType>` instead of `OnceLock<Option<SmallRangeProofType>>`.
    // We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    // #[derivative(Hash = "ignore")]
    // #[derivative(PartialEq = "ignore")]
    // #[serde(skip)]
    // pub tx_table_len_proof: OnceLock<Option<SmallRangeProofType>>,
}

impl<TableWord: TableWordTraits> Payload<TableWord> {
    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    pub fn num_namespaces(&self) -> usize {
        self.ns_table.len()
    }

    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    pub fn namespace_iter(&self) -> impl Iterator<Item = usize> {
        0..self.ns_table.len()
    }

    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    /// Returns the flat bytes for namespace `ns_id`, along with a proof of correctness for those bytes.
    ///
    /// RPC-friendly proof contains:
    /// - the namespace bytes
    /// - `vid_common` needed to verify the proof. This data is not accessible to the verifier because it's not part of the block header.
    pub fn namespace_with_proof(
        &self,
        ns_table: &NameSpaceTable<TxTableEntryWord>,
        ns_id: NamespaceId,
        vid_common: VidCommon,
    ) -> Option<NamespaceProof> {
        if self.raw_payload.len() != VidSchemeType::get_payload_byte_len(&vid_common) {
            return None; // error: vid_common inconsistent with self
        }

        let ns_index = if let Some(ns_index) = ns_table.lookup(ns_id) {
            ns_index
        } else {
            return Some(NamespaceProof::NonExistence { ns_id });
        };

        let ns_payload_range = ns_table
            .get_payload_range(ns_index, self.raw_payload.len())
            .1;

        // TODO log output for each `?`
        // fix this when we settle on an error handling pattern
        Some(NamespaceProof::Existence {
            ns_id,
            ns_payload_flat: self.raw_payload.get(ns_payload_range.clone())?.into(),
            ns_proof: vid_scheme(VidSchemeType::get_num_storage_nodes(&vid_common))
                .payload_proof(&self.raw_payload, ns_payload_range)
                .ok()?,
            vid_common,
        })
    }

    pub fn get_ns_table(&self) -> &NameSpaceTable<TableWord> {
        &self.ns_table
    }

    pub fn from_txs(
        txs: impl IntoIterator<Item = <payload::Payload<TxTableEntryWord> as BlockPayload>::Transaction>,
    ) -> Result<Self, Error> {
        let mut namespaces: HashMap<NamespaceId, NamespaceInfo> = Default::default();
        let mut structured_payload = Self {
            raw_payload: bytes![],
            ns_table: NameSpaceTable::default(),
        };
        for tx in txs.into_iter() {
            Payload::<TableWord>::update_namespace_with_tx(&mut namespaces, tx);
        }

        structured_payload.generate_raw_payload(namespaces)?;
        Ok(structured_payload)
    }

    fn update_namespace_with_tx(
        namespaces: &mut HashMap<NamespaceId, NamespaceInfo>,
        tx: <Payload<TxTableEntryWord> as BlockPayload>::Transaction,
    ) {
        let tx_bytes_len: TxTableEntry = tx.payload().len().try_into().unwrap(); // TODO (Philippe) error handling

        let namespace = namespaces.entry(tx.namespace()).or_insert(NamespaceInfo {
            tx_table: Vec::new(),
            tx_bodies: Vec::new(),
            tx_bytes_end: TxTableEntry::zero(),
            tx_table_len: TxTableEntry::zero(),
        });

        namespace
            .tx_bytes_end
            .checked_add_mut(tx_bytes_len)
            .unwrap(); // TODO (Philippe) error handling
        namespace.tx_table.extend(namespace.tx_bytes_end.to_bytes());
        namespace.tx_bodies.extend(tx.payload());

        namespace
            .tx_table_len
            .checked_add_mut(TxTableEntry::one())
            .unwrap(); // TODO (Philippe) error handling
    }

    fn generate_raw_payload(
        &mut self,
        namespaces: HashMap<NamespaceId, NamespaceInfo>,
    ) -> Result<(), Error> {
        // fill payload and namespace table
        let mut payload = bytes![];

        self.ns_table = NameSpaceTable::from_bytes(Vec::from(
            TxTableEntry::try_from(namespaces.len())
                .ok()
                .context(BlockBuildingSnafu)?
                .to_bytes(),
        ));

        let mut namespaces_offsets = vec![];
        for (id, namespace) in namespaces {
            payload.extend(namespace.tx_table_len.to_bytes());
            payload.extend(namespace.tx_table);
            payload.extend(namespace.tx_bodies);
            namespaces_offsets.push((id, payload.len()));
        }
        self.ns_table = NameSpaceTable::from_namespace_offsets(namespaces_offsets).unwrap();

        self.raw_payload = payload;
        Ok(())
    }
}

impl<TableWord: TableWordTraits + std::fmt::Debug> Display for Payload<TableWord> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl<TableWord: TableWordTraits> Committable for Payload<TableWord> {
    fn commit(&self) -> commit::Commitment<Self> {
        todo!()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(bound = "")] // for V
pub enum NamespaceProof {
    Existence {
        ns_payload_flat: Bytes,
        ns_id: NamespaceId,
        ns_proof: LargeRangeProofType,
        vid_common: VidCommon,
    },
    NonExistence {
        ns_id: NamespaceId,
    },
}

impl NamespaceProof {
    /// Verify a [`NamespaceProof`].
    ///
    /// All args must be available to the verifier in the block header.
    #[allow(dead_code)] // TODO temporary
    pub fn verify(
        &self,
        vid: &VidSchemeType,
        commit: &VidCommitment,
        ns_table: &NameSpaceTable<TxTableEntryWord>,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        match self {
            NamespaceProof::Existence {
                ns_payload_flat,
                ns_id,
                ns_proof,
                vid_common,
            } => {
                let ns_index = ns_table.lookup(*ns_id)?;

                let (ns_id, ns_payload_range) = ns_table
                    .get_payload_range(ns_index, VidSchemeType::get_payload_byte_len(vid_common));

                // verify self against args
                vid.payload_verify(
                    Statement {
                        payload_subslice: ns_payload_flat,
                        range: ns_payload_range,
                        commit,
                        common: vid_common,
                    },
                    ns_proof,
                )
                .ok()?
                .ok()?;

                // verification succeeded, return some data
                // we know ns_id is correct because the corresponding ns_payload_range passed verification
                Some((parse_ns_payload(ns_payload_flat, ns_id), ns_id))
            }
            NamespaceProof::NonExistence { ns_id } => {
                if ns_table.lookup(*ns_id).is_some() {
                    return None; // error: expect not to find ns_id in ns_table
                }
                Some((Vec::new(), *ns_id))
            }
        }
    }
}

pub fn parse_ns_payload(ns_bytes: &[u8], ns_id: NamespaceId) -> Vec<Transaction> {
    let num_txs = TxTable::get_tx_table_len(ns_bytes);
    (0..TxTable::get_tx_table_len(ns_bytes))
        .map(|tx_idx| TxTable::get_payload_range(ns_bytes, tx_idx, num_txs))
        .map(|tx_range| Transaction::new(ns_id, ns_bytes[tx_range].to_vec()))
        .collect()
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::block_contents::TestableBlock
    for Payload<crate::block::entry::TxTableEntryWord>
{
    fn genesis() -> Self {
        BlockPayload::genesis().0
    }

    fn txn_count(&self) -> u64 {
        use hotshot_query_service::availability::QueryablePayload;
        self.len(&self.ns_table) as u64
    }
}

#[cfg(test)]
mod test {
    use super::NamespaceProof;
    use crate::{
        block::{
            entry::{TxTableEntry, TxTableEntryWord},
            payload::{Payload, TableWordTraits},
            queryable,
            tables::{test::TxTableTest, NameSpaceTable, Table, TxTable},
            tx_iterator::TxIndex,
        },
        transaction::NamespaceId,
        Transaction,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use helpers::*;
    use hotshot_query_service::availability::QueryablePayload;
    use hotshot_types::{traits::BlockPayload, vid::vid_scheme};
    use jf_primitives::vid::{payload_prover::PayloadProver, VidScheme};
    use rand::RngCore;
    use std::{collections::HashMap, marker::PhantomData, ops::Range};

    const NUM_STORAGE_NODES: usize = 10;

    #[test]
    fn basic_correctness() {
        check_basic_correctness::<TxTableEntryWord>()
    }

    fn check_basic_correctness<TableWord: TableWordTraits>() {
        // play with this
        let test_cases = vec![
            // 1 namespace only
            vec![vec![5, 8, 8]], // 3 non-empty txs
            vec![vec![0, 8, 8]], // 1 empty tx at the beginning
            vec![vec![5, 0, 8]], // 1 empty tx in the middle
            vec![vec![5, 8, 0]], // 1 empty tx at the end
            vec![vec![5]],       // 1 nonempty tx
            vec![vec![0]],       // 1 empty tx
            // vec![],                 // zero txs
            vec![vec![1000, 1000, 1000]], // large payload
            //multiple namespaces
            vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8]], // 3 non-empty namespaces
        ];
        // TODO(746) future test cases
        // vec![vec![], vec![7, 9, 11], vec![10, 5, 8]], // 1 empty namespace at the beginning
        // vec![vec![5, 8, 8], vec![], vec![10, 5, 8]],  // 1 empty namespace in the middle
        // vec![vec![5, 8, 8], vec![7, 9, 11], vec![]],  // 1 empty namespace at the end
        // vec![vec![0], vec![0, 0]], // 2 non-empty namespaces with all-empty txs
        // vec![vec![], vec![]],      // 2 empty namespaces
        // vec![vec![1000, 1000, 1000], vec![2000, 2000, 2000]], // large payload

        // vec![(0,5), (0,8), (0,8), (1,7), (1,9), (1,11), (2,10), (2,5), (2,8)], // 3 non-empty namespaces, in order
        // vec![(14,5), (3,8), (7,8), (7,7), (14,9), (7,11), (3,10), (3,5), (14,8)], // 3 non-empty namespaces, out of order
        // vec![(0,0), (1,7), (1,9), (1,11), (2,10), (2,5), (2,8)], // a namespace with 1 empty tx at the beginning
        // vec![(0,5), (0,8), (0,8), (1,0), (2,10), (2,5), (2,8)],  // a namespace with 1 empty tx in the middle
        // vec![(0,0), (1,0)], // 2 namespaces, each with 1 empty tx

        setup_logging();
        setup_backtrace();
        let mut rng = jf_utils::test_rng();
        struct NamespaceInfo {
            payload_flat: Vec<u8>,
            tx_table: Vec<TxTableEntry>, // TODO Philippe => change
            #[allow(dead_code)] // TODO temporary
            tx_payloads: Vec<Vec<u8>>,
        }

        let vid = vid_scheme(NUM_STORAGE_NODES);
        let num_test_cases = test_cases.len();
        for (t, test_case) in test_cases.iter().enumerate() {
            // DERIVE A BUNCH OF STUFF FOR THIS TEST CASE
            let mut derived_nss = HashMap::new();
            let mut total_num_txs = 0;
            for (n, tx_lengths) in test_case.iter().enumerate() {
                tracing::info!(
                    "test block {} of {}, namespace {} of {}, with {} txs",
                    t + 1,
                    num_test_cases,
                    n + 1,
                    test_case.len(),
                    tx_lengths.len(),
                );
                total_num_txs += tx_lengths.len();

                // generate this namespace's tx payloads
                let entries = entries_from_lengths(tx_lengths);
                let tx_payloads_flat = random_bytes(tx_bodies_byte_len(&entries), &mut rng);
                let tx_payloads = extract_tx_payloads(&entries, &tx_payloads_flat);

                // enforce well-formed test case
                assert_eq!(
                    tx_payloads_flat,
                    tx_payloads.iter().flatten().cloned().collect::<Vec<_>>(),
                    "test block {} namespace {} is malformed",
                    t + 1,
                    n + 1
                );

                // derive this namespace's tx table
                let tx_table_derived: Vec<TxTableEntry> = tx_payloads
                    .iter()
                    .scan(TxTableEntry::zero(), |end, tx| {
                        end.checked_add_mut(TxTableEntry::try_from(tx.len()).unwrap())
                            .unwrap();
                        Some(end.clone())
                    })
                    .collect();

                // derive this namespace's payload
                let ns_payload_flat = {
                    let mut ns_payload = Vec::new();

                    // write tx table bytes
                    ns_payload.extend(TxTableEntry::from_usize(tx_table_derived.len()).to_bytes());
                    for entry in tx_table_derived.iter() {
                        ns_payload.extend(entry.to_bytes());
                    }

                    ns_payload.extend(tx_payloads_flat);
                    ns_payload
                };

                let already_exists = derived_nss.insert(
                    (n as u64).into(),
                    NamespaceInfo {
                        payload_flat: ns_payload_flat,
                        tx_table: tx_table_derived,
                        tx_payloads,
                    },
                );
                assert!(already_exists.is_none());
            }
            assert_eq!(derived_nss.len(), test_case.len());

            // COMPUTE ACTUAL STUFF AGAINST WHICH TO TEST DERIVED STUFF
            let all_txs_iter = derived_nss.iter().flat_map(|(ns_id, ns)| {
                ns.tx_payloads
                    .iter()
                    .map(|p| Transaction::new(*ns_id, p.clone()))
            });
            let (block, actual_ns_table) = Payload::from_transactions(all_txs_iter).unwrap();
            let disperse_data = vid.disperse(&block.raw_payload).unwrap();

            // TEST ACTUAL STUFF AGAINST DERIVED STUFF
            // test total ns length
            assert_eq!(block.num_namespaces(), derived_nss.len());

            // test total tx length
            tracing::info!("actual_ns_table {:?}", actual_ns_table);
            assert_eq!(block.len(&actual_ns_table), total_num_txs);
            // TODO assert the final ns table entry offset == self.payload.len()

            // test namespace table length
            let actual_ns_table_len =
                TxTableEntry::from_bytes(&actual_ns_table.get_bytes()[..TxTableEntry::byte_len()])
                    .unwrap();
            assert_eq!(
                actual_ns_table_len,
                TxTableEntry::try_from(test_case.len()).unwrap(),
                "namespace table length expect {} got {}",
                test_case.len(),
                actual_ns_table_len
            );

            // test each namespace
            // let mut tx_index_offset = 0;
            let mut ns_iter = block.namespace_iter();
            let mut block_iter = block.iter(&actual_ns_table); // test iterator correctness
            let mut prev_entry = TxTableEntry::zero();
            let mut derived_block_payload = Vec::new();
            for (ns_idx, (ns_id, entry)) in
                ns_table_iter::<TableWord>(actual_ns_table.get_bytes()).enumerate()
            {
                // warning! ns_id may not equal NamespaceId(ns_idx) due to HashMap nondeterminism

                let derived_ns = derived_nss.remove(&ns_id).unwrap();

                // test ns iterator
                let ns_iter_idx = ns_iter.next().unwrap();
                assert_eq!(ns_iter_idx, ns_idx);

                // test ns payload
                let actual_ns_payload_range = Range {
                    start: usize::try_from(prev_entry.clone()).unwrap(),
                    end: usize::try_from(entry.clone()).unwrap(),
                };
                let actual_ns_payload_flat = block
                    .raw_payload
                    .get(actual_ns_payload_range.clone())
                    .unwrap();
                assert_eq!(
                    actual_ns_payload_flat, derived_ns.payload_flat,
                    "namespace {ns_id} incorrect payload bytes",
                );

                // test ns proof
                let ns_proof = block
                    .namespace_with_proof(&actual_ns_table, ns_id, disperse_data.common.clone())
                    .unwrap();

                if let NamespaceProof::Existence {
                    ref ns_payload_flat,
                    ..
                } = ns_proof
                {
                    assert_eq!(
                        ns_payload_flat, &derived_ns.payload_flat,
                        "namespace {ns_id} incorrect payload bytes returned from namespace_with_proof",
                    );
                } else {
                    // TODO test for non-existence
                    panic!("expect NamespaceProof::Existence variant");
                };

                let (ns_proof_txs, ns_proof_ns_id) = ns_proof
                    .verify(&vid, &disperse_data.commit, &actual_ns_table)
                    .unwrap_or_else(|| panic!("namespace {ns_id} proof verification failure"));
                assert_eq!(ns_proof_ns_id, ns_id);
                assert_eq!(
                    ns_proof_txs,
                    derived_ns
                        .tx_payloads
                        .clone()
                        .into_iter()
                        .map(|p| Transaction::new(ns_id, p))
                        .collect::<Vec<Transaction>>()
                );

                // test tx table length
                let actual_tx_table_len_bytes = &actual_ns_payload_flat[..TxTableEntry::byte_len()];
                let actual_tx_table_len =
                    usize::try_from(TxTableEntry::from_bytes(actual_tx_table_len_bytes).unwrap())
                        .unwrap();
                assert_eq!(
                    actual_tx_table_len,
                    derived_ns.tx_table.len(),
                    "namespace {ns_id} tx table length expect {} got {}",
                    derived_ns.tx_table.len(),
                    actual_tx_table_len
                );

                // test tx table contents
                let actual_tx_table_body_bytes = &actual_ns_payload_flat[TxTableEntry::byte_len()
                    ..(actual_tx_table_len + 1) * TxTableEntry::byte_len()];
                // tracing::info!(ns t"x table bytes {:?}", actual_tx_table_body_bytes);
                let actual_tx_table: Vec<TxTableEntry> = actual_tx_table_body_bytes
                    .chunks(TxTableEntry::byte_len())
                    .map(|bytes| TxTableEntry::from_bytes(bytes).unwrap())
                    .collect();
                assert_eq!(
                    actual_tx_table, derived_ns.tx_table,
                    "namespace {ns_id} incorrect tx table for",
                );

                // testing tx iterator
                for tx_idx in 0..derived_ns.tx_table.len() {
                    let next_tx = block_iter.next().unwrap();
                    assert_eq!(ns_idx, next_tx.ns_idx);
                    assert_eq!(tx_idx, next_tx.tx_idx);

                    let idx = TxIndex { ns_idx, tx_idx };
                    let tx = block.transaction(&actual_ns_table, &idx).unwrap();
                    let tx_payload = derived_ns.tx_payloads[tx_idx].to_vec();
                    // test `transaction()`
                    assert_eq!(ns_id, tx.namespace());
                    assert_eq!(tx_payload, tx.payload());

                    // test `transaction_with_proof()`
                    let (tx, proof) = block
                        .transaction_with_proof(&actual_ns_table, &idx)
                        .unwrap();
                    assert_eq!(tx_payload, tx.payload());
                    proof
                        .verify(&tx, idx, &vid, &disperse_data.commit, &disperse_data.common)
                        .unwrap()
                        .unwrap();
                }

                prev_entry = entry;
                derived_block_payload.extend(derived_ns.payload_flat.clone());
            }
            assert!(
                ns_iter.next().is_none(),
                "expected ns iterator to be exhausted"
            );
            assert!(
                block_iter.next().is_none(),
                "expected tx iterator to be exhausted"
            );
            assert!(
                derived_nss.is_empty(),
                "some derived namespaces missing from namespace table"
            );

            // test full block payload
            // assert_eq!(tx_index_offset, block.len());
            assert_eq!(block.raw_payload, derived_block_payload);
        }
    }

    #[test]
    fn malformed_payloads() {
        check_malformed_payloads::<u32>();
        //check_malformed_payloads::<u64>(); // TODO Philippe this test is failing
    }
    fn check_malformed_payloads<TableWord: TableWordTraits>() {
        // play with this
        let mut rng = jf_utils::test_rng();
        let test_cases = vec![
            // negative-length txs
            TestCase::<TableWord>::from_entries(&[30, 10, 20], &mut rng), // 1 negative-length tx
            TestCase::from_entries(&[30, 20, 10], &mut rng),              // 2 negative-length txs
            // truncated payload
            TestCase::with_total_len(&[10, 20, 30], 20, &mut rng), // truncated tx payload
            TestCase::with_trimmed_body(&[10, 20, 30], 0, &mut rng), // 0-length tx payload
            TestCase::with_total_len(&[10, 20, u32::MAX as usize], 1000, &mut rng), // large tx truncated
            // negative-length txs AND truncated payload
            TestCase::with_total_len(&[30, 20, 10], 20, &mut rng), // negative-len txs, truncated tx payload
            TestCase::with_trimmed_body(&[30, 20, 10], 0, &mut rng), // negative-len txs, 0-len tx payload
            TestCase::with_total_len(&[10, u32::MAX as usize, 30], 1000, &mut rng), // negative-len tx, large tx truncated
            // tx table fits inside payload
            TestCase::from_tx_table_len(5, 100, &mut rng),
            TestCase::from_tx_table_len(25, 1000, &mut rng),
            // tx table too large for payload
            TestCase::from_tx_table_len_unchecked(100, 40, &mut rng),
            TestCase::from_tx_table_len_unchecked(
                10000, // TODO (Philippe) was TxTableEntry::MAX.try_into().unwrap(),
                100, &mut rng,
            ), // huge tx table length
            // extra payload bytes
            TestCase::with_total_len(&[10, 20, 30], 1000, &mut rng),
            TestCase::with_total_len(&[], 1000, &mut rng), // 0 txs
            // extremely small payload
            TestCase::from_tx_table_len_unchecked(1, 3, &mut rng), // 3-byte payload too small to store tx table len
            TestCase::from_tx_table_len_unchecked(1000, 3, &mut rng), // 3-byte payload, large number of txs
            TestCase::from_tx_table_len_unchecked(0, 3, &mut rng),    // 3-byte payload, 0 txs
            TestCase::from_tx_table_len_unchecked(6, 0, &mut rng),    // 0-byte payload
        ];

        // TODO(817) more test cases:
        // - this will break for extremely large payloads
        //   - should we hard-code an upper limit so arithmetic never overflows?

        setup_logging();
        setup_backtrace();

        let vid = vid_scheme(NUM_STORAGE_NODES);
        let num_test_cases = test_cases.len();
        for (t, test_case) in test_cases.into_iter().enumerate() {
            let payload_byte_len = test_case.payload.len();
            tracing::info!(
                "test payload {} of {} with {} txs and byte length {}",
                t + 1,
                num_test_cases,
                test_case.num_txs,
                payload_byte_len
            );

            // TODO don't initialize Payload with empty namespace table
            let block = Payload::from_bytes(
                test_case.payload.iter().cloned(),
                &NameSpaceTable::default(),
            );
            // assert_eq!(block.len(), test_case.num_txs);
            assert_eq!(block.raw_payload.len(), payload_byte_len);

            let _disperse_data = vid.disperse(&block.raw_payload).unwrap();

            // let mut tx_count: <BlockPayload as QueryablePayload>::TransactionIndex = 0; // test iterator correctness
            // for index in block.iter() {
            //     // tracing::info!("tx index {}", index,);
            //     let (tx, proof) = block.transaction_with_proof(&index).unwrap();
            //     proof
            //         .verify(
            //             &tx,
            //             index,
            //             &vid,
            //             &disperse_data.commit,
            //             &disperse_data.common,
            //         )
            //         .unwrap()
            //         .unwrap();
            //     tx_count += 1;
            // }
            // assert_eq!(test_case.num_txs, usize::try_from(tx_count).unwrap());

            // test: cannot make a proof for txs outside the tx table
            // assert!(block.transaction_with_proof(&tx_count).is_none());
        }
    }

    #[test]
    fn malicious_tx_inclusion_proof() {
        check_malicious_tx_inclusion_proof::<u32>();
        check_malicious_tx_inclusion_proof::<u64>();
    }

    fn check_malicious_tx_inclusion_proof<TableWord: TableWordTraits>() {
        setup_logging();
        setup_backtrace();

        let mut rng = jf_utils::test_rng();
        let test_case = TestCase::<TableWord>::from_tx_table_len_unchecked(1, 3, &mut rng); // 3-byte payload too small to store tx table len

        // TODO don't initialize Payload with empty namespace table
        let block = Payload::from_bytes(
            test_case.payload.iter().cloned(),
            &NameSpaceTable::default(),
        );
        assert_eq!(block.raw_payload.len(), test_case.payload.len());
        // assert_eq!(block.len(), test_case.num_txs);

        // test: cannot make a proof for such a small block
        // assert!(block.transaction_with_proof(&0).is_none());

        let vid = vid_scheme(NUM_STORAGE_NODES);
        let disperse_data = vid.disperse(&block.raw_payload).unwrap();

        // make a fake proof for a nonexistent tx in the small block
        let tx = Transaction::new(Default::default(), Vec::new());
        let proof = queryable::gen_tx_proof_for_testing(
            0..block.raw_payload.len(),
            TxTableEntry::from_usize(TxTable::get_tx_table_len(&block.raw_payload)),
            vid.payload_proof(
                &block.raw_payload,
                0..std::cmp::min(TxTableEntry::byte_len(), block.raw_payload.len()),
            )
            .unwrap(),
            vid.payload_proof(&block.raw_payload, 0..3).unwrap(),
        );

        // test: fake proof should get rejected
        // TODO should return Some(Err()) instead of None
        assert!(proof
            .verify(
                &tx,
                TxIndex {
                    ns_idx: 0,
                    tx_idx: 0
                },
                &vid,
                &disperse_data.commit,
                &disperse_data.common
            )
            .is_none());
    }

    struct TestCase<TableWord: TableWordTraits> {
        payload: Vec<u8>,
        num_txs: usize,
        phantomdata: PhantomData<TableWord>,
    }
    impl<TableWord: TableWordTraits> TestCase<TableWord> {
        /// Return a well-formed random block whose tx table is derived from `lengths`.
        #[allow(dead_code)]
        fn from_lengths<R: RngCore>(lengths: &[usize], rng: &mut R) -> Self {
            Self::from_entries(&entries_from_lengths(lengths), rng)
        }

        /// Return a random block whose tx table is derived from `entries`.
        ///
        /// If `entries` is well-formed then the result is well-formed.
        fn from_entries<R: RngCore>(entries: &[usize], rng: &mut R) -> Self {
            let tx_table = TxTableTest::<TableWord>::from_entries(entries);
            Self {
                payload: [
                    tx_table.get_payload(),
                    random_bytes(tx_bodies_byte_len(entries), rng),
                ]
                .concat(),
                num_txs: entries.len(),
                phantomdata: Default::default(),
            }
        }

        /// Like `from_entries` except the tx bodies byte length is `body_len`.
        ///
        /// Panics if `body_len` would not actually decrease the block size.
        fn with_trimmed_body<R: RngCore>(entries: &[usize], body_len: usize, rng: &mut R) -> Self {
            assert!(
                body_len < tx_bodies_byte_len(entries),
                "body_len too large to trim the body"
            );
            let tx_table = TxTableTest::<TableWord>::from_entries(entries);
            Self {
                payload: [tx_table.get_payload(), random_bytes(body_len, rng)].concat(),
                num_txs: entries.len(),
                phantomdata: Default::default(),
            }
        }

        /// Like `from_entries` except the byte length of the block is `block_byte_len`.
        ///
        /// Panics if `block_byte_len` would truncate the tx table.
        /// If you want to truncate the tx table then use `with_total_len_unchecked`.
        ///
        /// If `block_byte_len` would increase block size then new space is filled with random bytes.
        fn with_total_len<R: RngCore>(
            entries: &[usize],
            block_byte_len: usize,
            rng: &mut R,
        ) -> Self {
            assert!(
                tx_table_byte_len::<TableWord>(entries) <= block_byte_len,
                "tx table size {} for entries {:?} exceeds block_byte_len {}",
                tx_table_byte_len::<TableWord>(entries),
                entries,
                block_byte_len
            );
            Self::with_total_len_unchecked(entries, block_byte_len, rng)
        }

        /// Like `with_total_len` except `block_byte_len` may truncate the tx table.
        fn with_total_len_unchecked<R: RngCore>(
            entries: &[usize],
            block_byte_len: usize,
            rng: &mut R,
        ) -> Self {
            let tx_table = TxTableTest::<TableWord>::from_entries(entries);
            let mut payload = tx_table.get_payload();
            let num_txs = if block_byte_len > payload.len() {
                payload.extend(random_bytes(block_byte_len - payload.len(), rng));
                entries.len()
            } else {
                payload.truncate(block_byte_len);
                (block_byte_len / TxTableTest::<TableWord>::byte_len()).saturating_sub(1)
            };
            Self {
                payload,
                num_txs,
                phantomdata: Default::default(),
            }
        }

        /// Return a random block whose tx table indicates `tx_table_len` txs and whose total byte length is `block_byte_len`.
        ///
        /// Every byte of the block is random except the tx table header.
        ///
        /// Panics if `txs_byte_len` would truncate the tx table.
        /// If you want to truncate the tx table then use `with_total_len_unchecked`.
        fn from_tx_table_len<R: RngCore>(
            tx_table_len: usize,
            block_byte_len: usize,
            rng: &mut R,
        ) -> Self {
            let tx_table_byte_len = (tx_table_len + 1) * TxTableTest::<TableWord>::byte_len();
            assert!(
                tx_table_byte_len <= block_byte_len,
                "tx table size {} exceeds block size {}",
                tx_table_byte_len,
                block_byte_len
            );
            Self::from_tx_table_len_unchecked(tx_table_len, block_byte_len, rng)
        }

        /// Like `from_tx_table_len` except `block_byte_len` may truncate the tx table.
        fn from_tx_table_len_unchecked<R: RngCore>(
            tx_table_len: usize,
            block_byte_len: usize,
            rng: &mut R,
        ) -> Self {
            // accommodate extremely small block payload
            let header_byte_len =
                std::cmp::min(TxTableTest::<TableWord>::byte_len(), block_byte_len);
            let mut payload = vec![0; block_byte_len];
            rng.fill_bytes(&mut payload);
            payload[..header_byte_len].copy_from_slice(
                &TxTableEntry::from_usize(tx_table_len).to_bytes()[..header_byte_len], // TODO (Philippe) remove
            );
            Self {
                payload,
                num_txs: std::cmp::min(
                    tx_table_len,
                    (block_byte_len / TxTableTest::<TableWord>::byte_len()).saturating_sub(1),
                ),
                phantomdata: Default::default(),
            }
        }
    }

    mod helpers {
        use crate::block::entry::TxTableEntry;
        use crate::block::payload::TableWordTraits;
        use crate::block::tables::{test::TxTableTest, NameSpaceTable, Table};
        use crate::NamespaceId;
        use rand::RngCore;

        pub fn tx_table_byte_len<TableWord: TableWordTraits>(entries: &[usize]) -> usize {
            (entries.len() + 1) * TxTableTest::<TableWord>::byte_len()
        }

        pub fn entries_from_lengths(lengths: &[usize]) -> Vec<usize> {
            lengths
                .iter()
                .scan(0, |sum, &len| {
                    *sum += len;
                    Some(*sum)
                })
                .collect()
        }

        #[test]
        fn tx_table_helpers() {
            assert_eq!(vec![10, 20, 30], entries_from_lengths(&[10, 10, 10]));
        }

        pub fn tx_bodies_byte_len(entries: &[usize]) -> usize {
            // largest entry in the tx table dictates size of tx payloads
            *entries.iter().max().unwrap_or(&0)
        }

        pub fn random_bytes<R: RngCore>(len: usize, rng: &mut R) -> Vec<u8> {
            let mut result = vec![0; len];
            rng.fill_bytes(&mut result);
            result
        }

        pub fn extract_tx_payloads(entries: &[usize], tx_payloads_flat: &[u8]) -> Vec<Vec<u8>> {
            let mut result = Vec::with_capacity(entries.len());
            let mut start = 0;
            for end in entries {
                let end = std::cmp::min(*end, tx_payloads_flat.len());
                let tx_payload = if start >= end {
                    Vec::new()
                } else {
                    tx_payloads_flat[start..end].to_vec()
                };
                start = end;
                result.push(tx_payload);
            }
            assert_eq!(
                result.len(),
                entries.len(),
                "bug in test code: expect to extract {} txs but got {}",
                entries.len(),
                result.len()
            );
            result
        }

        pub fn ns_table_iter<TableWord: TableWordTraits>(
            ns_table_bytes: &[u8],
        ) -> impl Iterator<Item = (NamespaceId, TxTableEntry)> + '_ {
            ns_table_bytes[NameSpaceTable::<TableWord>::byte_len()..] // first few bytes is the table length, skip that
                .chunks(2 * TxTableEntry::byte_len())
                .map(|bytes| {
                    // read (namespace id, entry) from the namespace table
                    let ns_id = NamespaceId::try_from(
                        TxTableEntry::from_bytes(&bytes[..TxTableEntry::byte_len()]).unwrap(),
                    )
                    .unwrap();
                    let entry =
                        TxTableEntry::from_bytes(&bytes[TxTableEntry::byte_len()..]).unwrap();
                    (ns_id, entry)
                })
        }
    }
}
