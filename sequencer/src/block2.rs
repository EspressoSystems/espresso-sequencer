use crate::{BlockBuildingSnafu, Transaction};
use ark_bls12_381::Bls12_381;
use commit::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::traits::BlockPayload;
use jf_primitives::{
    pcs::{checked_fft_size, prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::advz::{
        payload_prover::{LargeRangeProof, SmallRangeProof},
        Advz,
    },
};

use serde::{Deserialize, Serialize};
use snafu::OptionExt;

pub mod entry;
pub mod payload;
pub mod queryable;
mod tables;
pub mod tx_iterator;

use payload::Payload;

impl BlockPayload for Payload<u32, u32, [u8; 32]> {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Metadata = Vec<u8>;
    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    /// Returns (Self, metadata).
    ///
    /// `metadata` is a bytes representation of the namespace table.
    /// Why bytes? To make it easy to move metdata into payload in the future.
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
        let mut structured_payload = Payload::new();

        for tx in txs.into_iter() {
            structured_payload.update_namespace_with_tx(tx);
        }

        structured_payload.generate_raw_payload()?;

        Some((
            structured_payload.clone(),
            structured_payload.ns_table.get_bytes(),
        ))
        .context(BlockBuildingSnafu)
    }

    // TODO(746) from_bytes doesn't need `metadata`!
    fn from_bytes<I>(encoded_transactions: I, _metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self {
            raw_payload: encoded_transactions.into_iter().collect(),
            tx_table_len_proof: Default::default(),
            table_len: 0,
            offset: 0,
            ns_id: [0; 32],
            ns_table: Default::default(),
            namespaces: Default::default(), // TODO (philippe) update
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
}
/// Opaque (not really though) constructor to return an abstract [`PayloadProver`].
///
/// Unfortunately, [`PayloadProver`] has a generic type param.
/// I'd like to return `impl PayloadProver<impl Foo>` but "nested `impl Trait` is not allowed":
/// <https://github.com/rust-lang/rust/issues/57979#issuecomment-459387604>
/// TODO Workaround using generic params, which is allows the caller to influence the return type:
/// https://stackoverflow.com/a/52886787
///
/// TODO temporary VID constructor.
pub(super) fn test_vid_factory() -> Advz<Bls12_381, sha2::Sha256> {
    // -> impl PayloadProver<RangeProof, Common = impl LengthGetter + CommitChecker<Self>> {
    let (payload_chunk_size, num_storage_nodes) = (8, 10);

    let mut rng = jf_utils::test_rng();
    let srs = UnivariateKzgPCS::<Bls12_381>::gen_srs_for_testing(
        &mut rng,
        checked_fft_size(payload_chunk_size - 1).unwrap(),
    )
    .unwrap();
    Advz::new(payload_chunk_size, num_storage_nodes, srs).unwrap()
}

// TODO type alias needed only because nested impl Trait is not allowed
// TODO upstream type aliases: https://github.com/EspressoSystems/jellyfish/issues/423
pub(super) type RangeProof =
    SmallRangeProof<<UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::Proof>;

/// Namespace proof type
///
/// # Type complexity
///
/// Jellyfish's `LargeRangeProof` type has a prime field generic parameter `F`.
/// This `F` is determined by the pairing parameter for `Advz` currently returned by `test_vid_factory()`.
/// Jellyfish needs a more ergonomic way for downstream users to refer to this type.
///
/// There is a `KzgEval` type alias in jellyfish that helps a little, but it's currently private.
/// If it were public then we could instead use
/// ```compile_fail
/// LargeRangeProof<KzgEval<Bls12_281>>
/// ```
/// but that's still pretty crufty.
pub type NamespaceProof =
    LargeRangeProof<<UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::Evaluation>;

#[cfg(test)]
mod test {
    use super::{queryable, test_vid_factory, Transaction};
    use crate::block2::payload::{Payload, TableLenTraits};
    use crate::block2::tables::{Table, TxTable};

    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use helpers::*;
    use hotshot_query_service::availability::QueryablePayload;
    use hotshot_types::traits::BlockPayload;
    use jf_primitives::vid::{
        payload_prover::{PayloadProver, Statement},
        VidScheme,
    };

    use crate::block2::entry::TxTableEntry;
    use crate::block2::tx_iterator::TxIndex;
    use rand::RngCore;
    use std::marker::PhantomData;
    use std::{collections::HashMap, ops::Range};

    #[test]
    fn basic_correctness() {
        // TODO Philippe parametrize with TableLen
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

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, test_case) in test_cases.iter().enumerate() {
            // DERIVE A BUNCH OF STUFF FOR THIS TEST CASE
            let mut txs = Vec::new();
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

                let ns_id = crate::VmId(n.try_into().unwrap());
                txs.extend(
                    tx_payloads
                        .iter()
                        .cloned()
                        .map(|tx_payload| Transaction::new(ns_id, tx_payload)),
                );
                let already_exists = derived_nss.insert(
                    ns_id,
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
            let (block, actual_ns_table) = Payload::from_transactions(txs).unwrap();
            let disperse_data = vid.disperse(&block.raw_payload).unwrap();

            // TEST ACTUAL STUFF AGAINST DERIVED STUFF
            // test total ns length
            assert_eq!(block.num_namespaces(&actual_ns_table), derived_nss.len());

            // test total tx length
            tracing::info!("actual_ns_table {:?}", actual_ns_table);
            assert_eq!(block.len(&actual_ns_table), total_num_txs);
            // TODO assert the final ns table entry offset == self.payload.len()

            // test namespace table length
            let actual_ns_table_len =
                TxTableEntry::from_bytes(&actual_ns_table[..TxTableEntry::byte_len()]).unwrap();
            assert_eq!(
                actual_ns_table_len,
                TxTableEntry::try_from(test_case.len()).unwrap(),
                "namespace table length expect {} got {}",
                test_case.len(),
                actual_ns_table_len
            );

            // test each namespace
            // let mut tx_index_offset = 0;
            let mut ns_iter = block.namespace_iter(&actual_ns_table);
            let mut block_iter = block.iter(&actual_ns_table); // test iterator correctness
            let mut prev_entry = TxTableEntry::zero();
            let mut derived_block_payload = Vec::new();
            for (ns_idx, (ns_id, entry)) in ns_table_iter::<u32>(&actual_ns_table).enumerate() {
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
                    "namespace {} incorrect payload bytes",
                    ns_id.0,
                );

                // test ns proof
                let (ns_payload_flat_from_proof, ns_proof) = block
                    .namespace_with_proof(&actual_ns_table, ns_idx)
                    .unwrap();
                assert_eq!(
                    ns_payload_flat_from_proof, derived_ns.payload_flat,
                    "namespace {} incorrect payload bytes returned from namespace_with_proof",
                    ns_id.0,
                );
                // NOTE: There is no NamespaceProof::verify method because it's quite simple.
                // compare: there is a TxInclusionProof::verify method for txs because that's complex.
                // TODO make a NamespaceProof::verify method?
                vid.payload_verify(
                    Statement {
                        payload_subslice: &ns_payload_flat_from_proof,
                        range: actual_ns_payload_range,
                        commit: &disperse_data.commit,
                        common: &disperse_data.common,
                    },
                    &ns_proof,
                )
                .unwrap()
                .unwrap_or_else(|_| panic!("namespace {} proof verification failure", ns_id.0));

                // test tx table length
                let actual_tx_table_len_bytes = &actual_ns_payload_flat[..TxTableEntry::byte_len()];
                let actual_tx_table_len =
                    usize::try_from(TxTableEntry::from_bytes(actual_tx_table_len_bytes).unwrap())
                        .unwrap();
                assert_eq!(
                    actual_tx_table_len,
                    derived_ns.tx_table.len(),
                    "namespace {} tx table length expect {} got {}",
                    ns_id.0,
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
                    "namespace {} incorrect tx table for",
                    ns_id.0
                );

                // testing tx iterator
                // TODO(746) incorporate this test into the following commented code when it's fixed
                for tx_idx in 0..derived_ns.tx_table.len() {
                    let next_tx = block_iter.next().unwrap();
                    assert_eq!(ns_idx, next_tx.ns_idx);
                    assert_eq!(tx_idx, next_tx.tx_idx);
                }

                // tests for individual txs in this namespace
                // TODO(746) rework this part
                //
                // let mut block_iter = block.iter(); // test iterator correctness
                // for (tx_index, tx_payload) in ns.tx_payloads.iter().enumerate() {
                //     assert!(block_iter.next().is_some());
                //     let tx_index = TxIndex::try_from(tx_index + tx_index_offset).unwrap();
                //     tracing::info!("tx index {}", tx_index,);
                //
                //     // test `transaction_with_proof()`
                //     let (tx, proof) = block.transaction_with_proof(&tx_index).unwrap();
                //     assert_eq!(tx_payload, tx.payload());
                //     proof
                //         .verify(
                //             &tx,
                //             tx_index,
                //             &vid,
                //             &disperse_data.commit,
                //             &disperse_data.common,
                //         )
                //         .unwrap()
                //         .unwrap();
                // }
                // assert!(block_iter.next().is_none());

                // prepare for the next loop iteration
                // tx_index_offset += actual_tx_table.len();
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
        // check_malformed_payloads::<u64>(); TODO Philippe this test is failing
    }
    fn check_malformed_payloads<TableLen: TableLenTraits>() {
        // play with this
        let mut rng = jf_utils::test_rng();
        let test_cases = vec![
            // negative-length txs
            TestCase::<TableLen>::from_entries(&[30, 10, 20], &mut rng), // 1 negative-length tx
            TestCase::from_entries(&[30, 20, 10], &mut rng),             // 2 negative-length txs
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

        let vid = test_vid_factory();
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

            let block = Payload::from_bytes(test_case.payload.iter().cloned(), &Vec::new());
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

    fn check_malicious_tx_inclusion_proof<
        TableLen: CanonicalSerialize
            + CanonicalDeserialize
            + TryFrom<usize>
            + TryInto<usize>
            + Default
            + std::marker::Sync,
    >() {
        setup_logging();
        setup_backtrace();

        let mut rng = jf_utils::test_rng();
        let test_case = TestCase::<TableLen>::from_tx_table_len_unchecked(1, 3, &mut rng); // 3-byte payload too small to store tx table len
        let block = Payload::from_bytes(test_case.payload.iter().cloned(), &Vec::new());
        assert_eq!(block.raw_payload.len(), test_case.payload.len());
        // assert_eq!(block.len(), test_case.num_txs);

        // test: cannot make a proof for such a small block
        // assert!(block.transaction_with_proof(&0).is_none());

        let vid = test_vid_factory();
        let disperse_data = vid.disperse(&block.raw_payload).unwrap();

        // make a fake proof for a nonexistent tx in the small block
        let tx = Transaction::new(crate::VmId(0), Vec::new());
        let proof = queryable::gen_tx_proof_for_testing(
            block.get_tx_table_len(),
            block.get_tx_table_len_proof(&vid).unwrap().clone(),
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

    struct TestCase<TableLen: TableLenTraits> {
        payload: Vec<u8>,
        num_txs: usize,
        phantomdata: PhantomData<TableLen>,
    }
    impl<TableLen: TableLenTraits> TestCase<TableLen> {
        /// Return a well-formed random block whose tx table is derived from `lengths`.
        #[allow(dead_code)]
        fn from_lengths<R: RngCore>(lengths: &[usize], rng: &mut R) -> Self {
            Self::from_entries(&entries_from_lengths(lengths), rng)
        }

        /// Return a random block whose tx table is derived from `entries`.
        ///
        /// If `entries` is well-formed then the result is well-formed.
        fn from_entries<R: RngCore>(entries: &[usize], rng: &mut R) -> Self {
            let tx_table = TxTable::<TableLen>::from_entries(entries);
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
        /// Panics if `body_len` would not actually decrese the block size.
        fn with_trimmed_body<R: RngCore>(entries: &[usize], body_len: usize, rng: &mut R) -> Self {
            assert!(
                body_len < tx_bodies_byte_len(entries),
                "body_len too large to trim the body"
            );
            let tx_table = TxTable::<TableLen>::from_entries(entries);
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
                tx_table_byte_len::<TableLen>(entries) <= block_byte_len,
                "tx table size {} for entries {:?} exceeds block_byte_len {}",
                tx_table_byte_len::<TableLen>(entries),
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
            let tx_table = TxTable::<TableLen>::from_entries(entries);
            let mut payload = tx_table.get_payload();
            let num_txs = if block_byte_len > payload.len() {
                payload.extend(random_bytes(block_byte_len - payload.len(), rng));
                entries.len()
            } else {
                payload.truncate(block_byte_len);
                (block_byte_len / TxTable::<TableLen>::byte_len()).saturating_sub(1)
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
            let tx_table_byte_len = (tx_table_len + 1) * TxTable::<TableLen>::byte_len();
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
            let header_byte_len = std::cmp::min(TxTable::<TableLen>::byte_len(), block_byte_len);
            let mut payload = vec![0; block_byte_len];
            rng.fill_bytes(&mut payload);
            payload[..header_byte_len].copy_from_slice(
                &TxTableEntry::from_usize(tx_table_len).to_bytes()[..header_byte_len], // TODO (Philippe) remove
            );
            Self {
                payload,
                num_txs: std::cmp::min(
                    tx_table_len,
                    (block_byte_len / TxTable::<TableLen>::byte_len()).saturating_sub(1),
                ),
                phantomdata: Default::default(),
            }
        }
    }

    mod helpers {
        use crate::block2::entry::TxTableEntry;
        use crate::block2::payload::{NameSpaceTable, TableLenTraits};
        use crate::block2::tables::{Table, TxTable};
        use crate::VmId;
        use rand::RngCore;

        pub fn tx_table_byte_len<TableLen: TableLenTraits>(entries: &[usize]) -> usize {
            (entries.len() + 1) * TxTable::<TableLen>::byte_len()
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

        pub fn ns_table_iter<TableLen: TableLenTraits>(
            ns_table_bytes: &[u8],
        ) -> impl Iterator<Item = (VmId, TxTableEntry)> + '_ {
            ns_table_bytes[NameSpaceTable::<TableLen>::byte_len()..] // first few bytes is the table lengh, skip that
                .chunks(2 * TxTableEntry::byte_len())
                .map(|bytes| {
                    // read (namespace id, entry) from the namespace table
                    let ns_id = VmId::try_from(
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
