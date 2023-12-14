use self::{boilerplate::RangeProof, tx_table_entry::TxTableEntry};
use crate::Transaction;
use derivative::Derivative;
use hotshot_query_service::availability::QueryablePayload;
use jf_primitives::{
    pcs::{prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::payload_prover::{PayloadProver, Statement},
};
use serde::{Deserialize, Serialize};
use std::{ops::Range, sync::OnceLock};

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
pub struct BlockPayload {
    payload: Vec<u8>,

    // cache frequently used items
    //
    // TODO type should be `OnceLock<RangeProof>` instead of `OnceLock<Option<RangeProof>>`. We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[serde(skip)]
    tx_table_len_proof: OnceLock<Option<RangeProof>>,
}

impl BlockPayload {
    #[allow(dead_code)] // TODO temporary
    fn from_txs(txs: impl IntoIterator<Item = Transaction>) -> Option<Self> {
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
        let mut tx_table = Vec::new();

        // concatenation of all tx payloads
        let mut tx_bodies = Vec::<u8>::new();

        // build tx_table, tx_bodies
        let mut tx_bytes_end = TxTableEntry::zero();
        let mut tx_table_len = TxTableEntry::zero();
        for tx in txs.into_iter() {
            let tx_bytes_len: TxTableEntry = tx.payload().len().try_into().ok()?;
            tx_bytes_end = tx_bytes_end.checked_add(tx_bytes_len)?;
            tx_table.extend(tx_bytes_end.to_bytes());
            tx_bodies.extend(tx.payload());
            tx_table_len = tx_table_len.checked_add(TxTableEntry::one())?;
        }

        // Naively copy all pieces into a flat payload.
        // We could avoid this by allocating memory in advance,
        // but that would require knowing the number of txs and their total
        // byte length in advance, which we can't do without a complete scan
        // of the `txs` iterator.
        let mut payload = Vec::new();
        payload.extend(tx_table_len.to_bytes());
        payload.extend(tx_table);
        payload.extend(tx_bodies);
        Some(Self {
            payload,
            tx_table_len_proof: Default::default(),
        })
    }

    #[allow(dead_code)] // TODO temporary
    fn from_bytes<B>(bytes: B) -> Self
    where
        B: IntoIterator<Item = u8>,
    {
        Self {
            payload: bytes.into_iter().collect(),
            tx_table_len_proof: Default::default(),
        }
    }

    // Return length of the tx table
    // == number of txs in the payload
    // == the first `TxTableEntry` word of `self.payload`.
    fn get_tx_table_len(&self) -> Option<TxTableEntry> {
        TxTableEntry::from_bytes(self.payload.get(0..TxTableEntry::byte_len())?)
    }
    fn get_tx_table_len_as<T>(&self) -> Option<T>
    where
        TxTableEntry: TryInto<T>,
    {
        self.get_tx_table_len()?.try_into().ok()
    }

    // Fetch the tx table length range proof from cache.
    // Build the proof if missing from cache.
    // Returns `None` if an error occurred.
    fn get_tx_table_len_proof(&self, vid: &impl PayloadProver<RangeProof>) -> Option<&RangeProof> {
        self.tx_table_len_proof
            .get_or_init(|| {
                vid.payload_proof(&self.payload, 0..TxTableEntry::byte_len())
                    .ok()
            })
            .as_ref()
    }
}

/// Returns the byte range for a tx in the block payload bytes.
///
/// Ensures that the returned range is valid (start <= end) and within bounds for `block_payload_byte_len`.
/// Lots of ugly type conversion and checked arithmetic.
fn tx_payload_range(
    tx_table_range_start: &Option<TxTableEntry>,
    tx_table_range_end: &TxTableEntry,
    tx_table_len: &TxTableEntry,
    block_payload_byte_len: usize,
) -> Option<Range<usize>> {
    let tx_bodies_offset = usize::try_from(tx_table_len.clone())
        .ok()?
        .checked_add(1)?
        .checked_mul(TxTableEntry::byte_len())?;
    let start = usize::try_from(tx_table_range_start.clone().unwrap_or(TxTableEntry::zero()))
        .ok()?
        .checked_add(tx_bodies_offset)?;
    let end = usize::try_from(tx_table_range_end.clone())
        .ok()?
        .checked_add(tx_bodies_offset)?;
    let end = std::cmp::max(start, end);
    let start = std::cmp::min(start, block_payload_byte_len);
    let end = std::cmp::min(end, block_payload_byte_len);
    Some(start..end)
}

impl QueryablePayload for BlockPayload {
    type TransactionIndex = u32;
    type Iter<'a> = Range<Self::TransactionIndex>;
    type InclusionProof = TxInclusionProof;

    fn len(&self) -> usize {
        self.get_tx_table_len_as().unwrap_or(0)
    }

    fn iter(&self) -> Self::Iter<'_> {
        0..self.get_tx_table_len_as().unwrap_or(0)
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        let index_usize = usize::try_from(*index).ok()?;
        if index_usize >= self.get_tx_table_len_as()? {
            return None; // error: index out of bounds
        }

        let vid = boilerplate::test_vid_factory(); // TODO temporary VID construction

        // Read the tx payload range from the tx table into `tx_table_range_[start|end]` and compute a proof that this range is correct.
        //
        // This correctness proof requires a range of its own, which we read into `tx_table_range_proof_[start|end]`.
        //
        // Edge case--the first transaction: tx payload range `start` is implicitly 0 and we do not include this item in the correctness proof.
        //
        // TODO why isn't cargo fmt wrapping these comments?

        // start
        let (tx_table_range_proof_start, tx_table_range_start) = if index_usize == 0 {
            (TxTableEntry::byte_len(), None)
        } else {
            let range_proof_start = index_usize.checked_mul(TxTableEntry::byte_len())?;
            (
                range_proof_start,
                Some(TxTableEntry::from_bytes(self.payload.get(
                    range_proof_start..range_proof_start.checked_add(TxTableEntry::byte_len())?,
                )?)?),
            )
        };

        // end
        let tx_table_range_proof_end = index_usize
            .checked_add(2)?
            .checked_mul(TxTableEntry::byte_len())?;
        let tx_table_range_end = TxTableEntry::from_bytes(self.payload.get(
            tx_table_range_proof_end.checked_sub(TxTableEntry::byte_len())?
                ..tx_table_range_proof_end,
        )?)?;

        // correctness proof for the tx payload range
        let tx_table_range_proof = vid
            .payload_proof(
                &self.payload,
                tx_table_range_proof_start..tx_table_range_proof_end,
            )
            .ok()?;

        let tx_payload_range = tx_payload_range(
            &tx_table_range_start,
            &tx_table_range_end,
            &self.get_tx_table_len()?,
            self.payload.len(),
        )?;

        Some((
            // TODO don't copy the tx bytes into the return value
            // https://github.com/EspressoSystems/hotshot-query-service/issues/267
            Transaction::new(
                crate::VmId(0),
                self.payload.get(tx_payload_range.clone())?.to_vec(),
            ),
            TxInclusionProof {
                tx_table_len: self.get_tx_table_len()?,
                tx_table_len_proof: self.get_tx_table_len_proof(&vid)?.clone(),
                tx_table_range_start,
                tx_table_range_end,
                tx_table_range_proof,
                tx_payload_proof: if tx_payload_range.is_empty() {
                    None
                } else {
                    vid.payload_proof(&self.payload, tx_payload_range).ok()
                },
            },
        ))
    }
}

type TxIndex = <BlockPayload as QueryablePayload>::TransactionIndex;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxInclusionProof {
    tx_table_len: TxTableEntry,
    tx_table_len_proof: RangeProof,

    tx_table_range_start: Option<TxTableEntry>, // `None` for the 0th tx
    tx_table_range_end: TxTableEntry,
    tx_table_range_proof: RangeProof,

    tx_payload_proof: Option<RangeProof>, // `None` if the tx has zero length
}

impl TxInclusionProof {
    // TODO prototype only!
    //
    // - We need to decide where to store VID params.
    // - Returns `None` if an error occurred.
    // - Use of `Result<(),()>` pattern to enable use of `?` for concise abort-on-failure.
    #[allow(dead_code)] // TODO temporary
    #[allow(clippy::too_many_arguments)]
    fn verify<V>(
        &self,
        tx: &Transaction,
        tx_index: TxIndex,
        vid: &V,
        vid_commit: &V::Commit,
        vid_common: &V::Common,
    ) -> Option<Result<(), ()>>
    where
        V: PayloadProver<RangeProof>,
    {
        V::is_consistent(vid_commit, vid_common).ok()?;

        // Verify proof for tx payload.
        // Proof is `None` if and only if tx has zero length.
        let tx_payload_range = tx_payload_range(
            &self.tx_table_range_start,
            &self.tx_table_range_end,
            &self.tx_table_len,
            V::get_payload_byte_len(vid_common),
        )?;
        match &self.tx_payload_proof {
            Some(tx_payload_proof) => {
                if vid
                    .payload_verify(
                        Statement {
                            payload_subslice: tx.payload(),
                            range: tx_payload_range,
                            commit: vid_commit,
                            common: vid_common,
                        },
                        tx_payload_proof,
                    )
                    .ok()?
                    .is_err()
                {
                    return Some(Err(())); // TODO it would be nice to use ? here...
                }
            }
            None => {
                if !tx.payload().is_empty() || !tx_payload_range.is_empty() {
                    return None; // error: nonempty payload but no proof
                }
            }
        };

        // Verify proof for tx table len.
        if vid
            .payload_verify(
                Statement {
                    payload_subslice: &self.tx_table_len.to_bytes(),
                    range: 0..TxTableEntry::byte_len(),
                    commit: vid_commit,
                    common: vid_common,
                },
                &self.tx_table_len_proof,
            )
            .ok()?
            .is_err()
        {
            return Some(Err(()));
        }

        // Verify proof for tx table entries.
        // Start index missing for the 0th tx
        let index: usize = tx_index.try_into().ok()?;
        let mut tx_table_range_bytes =
            Vec::with_capacity(2usize.checked_mul(TxTableEntry::byte_len())?);
        let start = if let Some(tx_table_range_start) = &self.tx_table_range_start {
            if index == 0 {
                return None; // error: first tx should have empty start index
            }
            tx_table_range_bytes.extend(tx_table_range_start.to_bytes());
            index * TxTableEntry::byte_len()
        } else {
            if index != 0 {
                return None; // error: only the first tx should have empty start index
            }
            TxTableEntry::byte_len()
        };
        tx_table_range_bytes.extend(self.tx_table_range_end.to_bytes());
        let range = start
            ..index
                .checked_add(2)?
                .checked_mul(TxTableEntry::byte_len())?;

        if vid
            .payload_verify(
                Statement {
                    payload_subslice: &tx_table_range_bytes,
                    range,
                    commit: vid_commit,
                    common: vid_common,
                },
                &self.tx_table_range_proof,
            )
            .ok()?
            .is_err()
        {
            return Some(Err(()));
        }

        Some(Ok(()))
    }
}

mod tx_table_entry {
    use super::{Deserialize, Serialize, TxIndex};
    use std::mem::size_of;

    // Use newtype pattern so that tx table entires cannot be confused with other types.
    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct TxTableEntry(TxTableEntryWord);
    type TxTableEntryWord = u32;

    impl TxTableEntry {
        pub const fn checked_add(self, rhs: Self) -> Option<Self> {
            // `?` is not allowed in a `const fn` https://github.com/rust-lang/rust/issues/74935
            // Some(Self(self.0.checked_add(rhs.0)?))
            match self.0.checked_add(rhs.0) {
                Some(val) => Some(Self(val)),
                None => None,
            }
        }
        pub const fn zero() -> Self {
            Self(0)
        }
        pub const fn one() -> Self {
            Self(1)
        }
        pub const fn to_bytes(&self) -> [u8; size_of::<TxTableEntryWord>()] {
            self.0.to_le_bytes()
        }
        pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
            Some(Self(TxTableEntryWord::from_le_bytes(
                bytes.try_into().ok()?,
            )))
        }
        pub const fn byte_len() -> usize {
            size_of::<TxTableEntryWord>()
        }

        #[cfg(test)]
        pub fn from_usize(val: usize) -> Self {
            Self(
                val.try_into()
                    .expect("usize -> TxTableEntry should succeed"),
            )
        }
    }

    impl TryFrom<usize> for TxTableEntry {
        type Error = <TxTableEntryWord as TryFrom<usize>>::Error;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            TxTableEntryWord::try_from(value).map(Self)
        }
    }
    impl TryFrom<TxTableEntry> for usize {
        type Error = <usize as TryFrom<TxTableEntryWord>>::Error;

        fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
            usize::try_from(value.0)
        }
    }

    impl TryFrom<TxIndex> for TxTableEntry {
        type Error = <TxTableEntryWord as TryFrom<TxIndex>>::Error;

        fn try_from(value: TxIndex) -> Result<Self, Self::Error> {
            TxTableEntryWord::try_from(value).map(Self)
        }
    }
    impl TryFrom<TxTableEntry> for TxIndex {
        type Error = <TxIndex as TryFrom<TxTableEntryWord>>::Error;

        fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
            TxIndex::try_from(value.0)
        }
    }
}

mod boilerplate {
    use super::{
        BlockPayload, PolynomialCommitmentScheme, QueryablePayload, Transaction, UnivariateKzgPCS,
    };
    use crate::BlockBuildingSnafu;
    use ark_bls12_381::Bls12_381;
    use commit::{Commitment, Committable};
    use jf_primitives::{
        pcs::checked_fft_size,
        vid::advz::{payload_prover::SmallRangeProof, Advz},
    };
    use snafu::OptionExt;
    use std::fmt::Display;

    // Skeleton impl for now so as to enable `QueryablePayload`.
    impl hotshot::traits::BlockPayload for BlockPayload {
        type Error = crate::Error;
        type Transaction = Transaction;
        type Metadata = ();
        type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

        fn from_transactions(
            transactions: impl IntoIterator<Item = Self::Transaction>,
        ) -> Result<(Self, Self::Metadata), Self::Error> {
            let payload = Self::from_txs(transactions).context(BlockBuildingSnafu)?;
            Ok((payload, ()))
        }

        fn from_bytes<I>(encoded_transactions: I, _metadata: Self::Metadata) -> Self
        where
            I: Iterator<Item = u8>,
        {
            Self::from_bytes(encoded_transactions)
        }

        fn genesis() -> (Self, Self::Metadata) {
            Self::from_transactions([]).unwrap()
        }

        fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
            Ok(self.payload.iter().cloned())
        }

        fn transaction_commitments(&self) -> Vec<Commitment<Self::Transaction>> {
            self.enumerate().map(|(_, tx)| tx.commit()).collect()
        }
    }

    impl Display for BlockPayload {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:#?}")
        }
    }

    impl Committable for BlockPayload {
        fn commit(&self) -> commit::Commitment<Self> {
            todo!()
        }
    }

    /// Opaque (not really though) constructor to return an abstract [`PayloadProver`].
    ///
    /// Unfortunately, [`PayloadProver`] has a generic type param.
    /// I'd like to return `impl PayloadProver<impl Foo>` but "nested `impl Trait` is not allowed":
    /// <https://github.com/rust-lang/rust/issues/57979#issuecomment-459387604>
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
}

#[cfg(test)]
mod test {
    use super::{
        boilerplate::test_vid_factory, BlockPayload, QueryablePayload, Transaction, TxIndex,
        TxTableEntry,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use helpers::*;
    use jf_primitives::vid::VidScheme;
    use rand::RngCore;

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            TestCase::from_tx_lengths(&[5, 8, 8]), // 3 non-empty txs
            TestCase::from_tx_lengths(&[0, 8, 8]), // 1 empty tx at the beginning
            TestCase::from_tx_lengths(&[5, 0, 8]), // 1 empty tx in the middle
            TestCase::from_tx_lengths(&[5, 8, 0]), // 1 empty tx at the end
            TestCase::from_tx_lengths(&[5]),       // 1 nonempty tx
            TestCase::from_tx_lengths(&[0]),       // 1 empty tx
            TestCase::from_tx_lengths(&[]),        // zero txs
            TestCase::from_tx_lengths(&[1000, 1000, 1000]), // large payload
        ];

        struct TestCase {
            entries: Vec<usize>,
            num_txs: usize,
        }
        impl TestCase {
            fn from_tx_lengths(lengths: &[usize]) -> Self {
                Self {
                    entries: entries_from_lengths(lengths),
                    num_txs: lengths.len(),
                }
            }
        }

        setup_logging();
        setup_backtrace();
        let mut rng = jf_utils::test_rng();

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, test_case) in test_cases.into_iter().enumerate() {
            tracing::info!(
                "test payload {} of {} with {} txs",
                t + 1,
                num_test_cases,
                test_case.num_txs,
            );

            // prepare things as a function of the test case
            let tx_payloads_flat = random_tx_payloads_flat(&test_case.entries, &mut rng);
            let tx_bodies = extract_tx_payloads(&test_case.entries, &tx_payloads_flat);
            assert_eq!(tx_bodies.len(), test_case.num_txs);
            assert_eq!(
                // enforce well-formed test case
                tx_payloads_flat,
                tx_bodies.iter().flatten().cloned().collect::<Vec<_>>(),
                "test block payload {} is malformed",
                t + 1
            );
            let txs = tx_bodies
                .iter()
                .cloned()
                .map(|payload| Transaction::new(crate::VmId(0), payload));
            let tx_offsets: Vec<TxTableEntry> = tx_bodies
                .iter()
                .scan(TxTableEntry::zero(), |end, tx| {
                    *end = end
                        .clone()
                        .checked_add(TxTableEntry::try_from(tx.len()).unwrap())
                        .unwrap();
                    Some(end.clone())
                })
                .collect();

            let block = BlockPayload::from_txs(txs).unwrap();

            // test tx table length
            let (tx_table_len_bytes, payload) = block.payload.split_at(TxTableEntry::byte_len());
            let tx_table_len = TxTableEntry::from_bytes(tx_table_len_bytes).unwrap();
            assert_eq!(
                tx_table_len,
                TxTableEntry::try_from(tx_bodies.len()).unwrap()
            );

            // test tx table contents
            let (tx_table_bytes, payload) =
                payload.split_at(tx_bodies.len() * TxTableEntry::byte_len());
            let tx_table: Vec<TxTableEntry> = tx_table_bytes
                .chunks(TxTableEntry::byte_len())
                .map(|len_bytes| TxTableEntry::from_bytes(len_bytes).unwrap())
                .collect();
            assert_eq!(tx_table, tx_offsets);

            // test block payload body
            assert_eq!(payload, tx_payloads_flat);
            assert_eq!(tx_bodies.len(), block.len());

            // tests for individual txs
            let disperse_data = vid.disperse(&block.payload).unwrap();
            let mut block_iter = block.iter(); // test iterator correctness
            for (index_usize, tx_body) in tx_bodies.iter().enumerate() {
                assert!(block_iter.next().is_some());
                let index = TxIndex::try_from(index_usize).unwrap();
                // tracing::info!("tx index {}", index,);

                // test `transaction_with_proof()`
                let (tx, proof) = block.transaction_with_proof(&index).unwrap();
                assert_eq!(tx_body, tx.payload());
                proof
                    .verify(
                        &tx,
                        index,
                        &vid,
                        &disperse_data.commit,
                        &disperse_data.common,
                    )
                    .unwrap()
                    .unwrap();
            }
            assert!(block_iter.next().is_none());
        }
    }

    #[test]
    fn negative_len_txs_and_truncated_tx_payload() {
        // play with this
        let mut rng = jf_utils::test_rng();
        let test_cases = vec![
            TestCase::from_entries(&[30, 10, 20], &mut rng), // 1 negative-length tx
            TestCase::from_entries(&[30, 20, 10], &mut rng), // 2 negative-length txs
            TestCase::from_entries_truncated(&[10, 20, 30], 15, &mut rng), // truncated tx payload
            TestCase::from_entries_no_payload(&[10, 20, 30]), // 0-length tx payload
            TestCase::from_entries_truncated(&[30, 20, 10], 15, &mut rng), // negative-len txs, truncated tx payload
            TestCase::from_entries_no_payload(&[30, 20, 10]), // negative-len txs, 0-len tx payload
            TestCase::from_entries_truncated(&[10, 20, u32::MAX as usize], 1000, &mut rng), // large tx truncated
            TestCase::from_entries_truncated(&[10, u32::MAX as usize, 30], 1000, &mut rng), // negative-len tx, large tx truncated
            TestCase::from_tx_table_len(5, 100, &mut rng), // random payload except tx table len
            TestCase::from_tx_table_len(25, 1000, &mut rng), // random payload except tx table len
        ];
        struct TestCase {
            payload: Vec<u8>,
            num_txs: usize,
        }
        impl TestCase {
            fn from_entries<R: RngCore>(entries: &[usize], rng: &mut R) -> Self {
                Self {
                    payload: random_payload(entries, rng),
                    num_txs: entries.len(),
                }
            }
            fn from_entries_truncated<R: RngCore>(
                entries: &[usize],
                txs_byte_len: usize,
                rng: &mut R,
            ) -> Self {
                Self {
                    payload: random_payload_truncated(entries, txs_byte_len, rng),
                    num_txs: entries.len(),
                }
            }
            fn from_entries_no_payload(entries: &[usize]) -> Self {
                Self {
                    payload: tx_table(entries),
                    num_txs: entries.len(),
                }
            }
            fn from_tx_table_len<R: RngCore>(
                tx_table_len: usize,
                block_byte_len: usize,
                rng: &mut R,
            ) -> Self {
                assert!(
                    tx_table_len * TxTableEntry::byte_len() <= block_byte_len,
                    "tx table size exceeds block size"
                );
                Self {
                    payload: random_block_with_tx_table_len(tx_table_len, block_byte_len, rng),
                    num_txs: tx_table_len,
                }
            }
        }

        // TODO more test cases:
        // - valid tx proof P made from large payload, checked against a prefix of that payload where P is invalid
        // - payload <4 bytes

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

            let block = BlockPayload::from_bytes(test_case.payload);
            assert_eq!(block.len(), test_case.num_txs);

            let disperse_data = vid.disperse(&block.payload).unwrap();

            let mut tx_count = 0; // test iterator correctness
            for index in block.iter() {
                // tracing::info!("tx index {}", index,);
                let (tx, proof) = block.transaction_with_proof(&index).unwrap();
                proof
                    .verify(
                        &tx,
                        index,
                        &vid,
                        &disperse_data.commit,
                        &disperse_data.common,
                    )
                    .unwrap()
                    .unwrap();
                tx_count += 1;
            }
            assert_eq!(tx_count, block.len());
        }
    }

    mod helpers {
        use crate::block2::tx_table_entry::TxTableEntry;
        use rand::RngCore;

        pub fn tx_table(entries: &[usize]) -> Vec<u8> {
            let mut tx_table = Vec::with_capacity(entries.len() + TxTableEntry::byte_len());
            tx_table.extend(TxTableEntry::from_usize(entries.len()).to_bytes());
            for entry in entries {
                tx_table.extend(TxTableEntry::from_usize(*entry).to_bytes());
            }
            tx_table
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

        pub fn random_tx_payloads_flat<R>(entries: &[usize], rng: &mut R) -> Vec<u8>
        where
            R: RngCore,
        {
            random_tx_payloads_flat_truncated_inner(entries, None, rng)
        }

        #[allow(dead_code)]
        pub fn random_tx_payloads_flat_truncated<R>(
            entries: &[usize],
            max_tx_payloads_byte_len: usize,
            rng: &mut R,
        ) -> Vec<u8>
        where
            R: RngCore,
        {
            random_tx_payloads_flat_truncated_inner(entries, Some(max_tx_payloads_byte_len), rng)
        }

        fn random_tx_payloads_flat_truncated_inner<R>(
            entries: &[usize],
            max_tx_payloads_byte_len: Option<usize>,
            rng: &mut R,
        ) -> Vec<u8>
        where
            R: RngCore,
        {
            // largest entry dictates size of tx bodies
            let tx_payloads_flat_byte_len = *entries.iter().max().unwrap_or(&0);

            // enforce max length if present
            let tx_payloads_flat_byte_len = if let Some(max) = max_tx_payloads_byte_len {
                std::cmp::min(tx_payloads_flat_byte_len, max)
            } else {
                tx_payloads_flat_byte_len
            };

            let mut result = vec![0; tx_payloads_flat_byte_len];
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
            assert_eq!(result.len(), entries.len());
            result
        }

        pub fn random_payload<R>(entries: &[usize], rng: &mut R) -> Vec<u8>
        where
            R: RngCore,
        {
            random_payload_truncated_inner(entries, None, rng)
        }
        pub fn random_payload_truncated<R>(
            entries: &[usize],
            max_tx_payloads_byte_len: usize,
            rng: &mut R,
        ) -> Vec<u8>
        where
            R: RngCore,
        {
            random_payload_truncated_inner(entries, Some(max_tx_payloads_byte_len), rng)
        }
        fn random_payload_truncated_inner<R>(
            entries: &[usize],
            max_tx_payloads_byte_len: Option<usize>,
            rng: &mut R,
        ) -> Vec<u8>
        where
            R: RngCore,
        {
            let mut result = tx_table(entries);
            result.extend(random_tx_payloads_flat_truncated_inner(
                entries,
                max_tx_payloads_byte_len,
                rng,
            ));
            result
        }

        pub fn random_block_with_tx_table_len<R>(
            tx_table_len: usize,
            block_byte_len: usize,
            rng: &mut R,
        ) -> Vec<u8>
        where
            R: RngCore,
        {
            let mut result = vec![0; block_byte_len];
            rng.fill_bytes(&mut result);
            result[..TxTableEntry::byte_len()]
                .copy_from_slice(&TxTableEntry::from_usize(tx_table_len).to_bytes());
            result
        }
    }
}
