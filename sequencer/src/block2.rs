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

    // TODO(746) should metadata be included in hash, serde, etc?
    metadata: NamespaceTable,

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
            metadata: NamespaceTable(),
            tx_table_len_proof: Default::default(),
        })
    }

    fn from_bytes<B>(bytes: B) -> Self
    where
        B: IntoIterator<Item = u8>,
    {
        Self {
            payload: bytes.into_iter().collect(),
            metadata: NamespaceTable(),
            tx_table_len_proof: Default::default(),
        }
    }

    /// Return a range `r` such that `self.payload[r]` is the bytes of the tx table length.
    ///
    /// Typically `r` is `0..TxTableEntry::byte_len()`.
    /// But it might differ from this if the payload byte length is less than `TxTableEntry::byte_len()`.
    fn tx_table_len_range(&self) -> Range<usize> {
        0..std::cmp::min(TxTableEntry::byte_len(), self.payload.len())
    }

    /// Return length of the tx table, read from the payload bytes.
    ///
    /// This quantity equals number of txs in the payload.
    fn get_tx_table_len(&self) -> TxTableEntry {
        let tx_table_len_range = self.tx_table_len_range();
        let mut entry_bytes = [0u8; TxTableEntry::byte_len()];
        entry_bytes[..tx_table_len_range.len()].copy_from_slice(&self.payload[tx_table_len_range]);

        TxTableEntry::from_bytes_array(entry_bytes)
    }
    fn get_tx_table_len_as<T>(&self) -> Option<T>
    where
        TxTableEntry: TryInto<T>,
    {
        self.get_tx_table_len().try_into().ok()
    }

    // Fetch the tx table length range proof from cache.
    // Build the proof if missing from cache.
    // Returns `None` if an error occurred.
    fn get_tx_table_len_proof(&self, vid: &impl PayloadProver<RangeProof>) -> Option<&RangeProof> {
        self.tx_table_len_proof
            .get_or_init(|| {
                vid.payload_proof(&self.payload, self.tx_table_len_range())
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
    // TODO(817) allow arbitrary tx_table_len
    // eg: if overflow then just return a 0-length tx
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]

pub struct NamespaceTable();

impl QueryablePayload for BlockPayload {
    type TransactionIndex = u32;
    type Iter<'a> = Range<Self::TransactionIndex>;
    type InclusionProof = TxInclusionProof;

    fn len(&self) -> usize {
        // The number of txs in a block is defined as the minimum of:
        // (1) the number of txs indicated in the tx table
        // (2) the number of tx table entries that could fit into the payload
        // Why? Because (1) could be anything. A block should not be allowed to contain 4 billion 0-length txs.
        //
        // The quantity (2) must exclude the first entry of the tx table because this entry indicates only the length of the tx table, not an actual tx.
        std::cmp::min(
            self.get_tx_table_len_as().unwrap_or(0),
            (self.payload.len() / TxTableEntry::byte_len()).saturating_sub(1), // allow space for the tx table length
        )
    }

    fn iter(&self) -> Self::Iter<'_> {
        0..self.len().try_into().unwrap_or(0)
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        let index_usize = usize::try_from(*index).ok()?;
        if index_usize >= self.len() {
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
            &self.get_tx_table_len(),
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
                tx_table_len: self.get_tx_table_len(),
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
        pub const MAX: TxTableEntry = Self(TxTableEntryWord::MAX);

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
        /// Infallible constructor.
        pub fn from_bytes_array(bytes: [u8; TxTableEntry::byte_len()]) -> Self {
            Self(TxTableEntryWord::from_le_bytes(bytes))
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
        BlockPayload, NamespaceTable, PolynomialCommitmentScheme, QueryablePayload, Transaction,
        UnivariateKzgPCS,
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
        type Metadata = NamespaceTable;
        type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

        fn from_transactions(
            transactions: impl IntoIterator<Item = Self::Transaction>,
        ) -> Result<(Self, Self::Metadata), Self::Error> {
            let payload = Self::from_txs(transactions).context(BlockBuildingSnafu)?;
            Ok((payload, NamespaceTable()))
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
        boilerplate::test_vid_factory, BlockPayload, QueryablePayload, Transaction,
        TxInclusionProof, TxIndex, TxTableEntry,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use helpers::*;
    use jf_primitives::vid::{payload_prover::PayloadProver, VidScheme};
    use rand::RngCore;

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            vec![5, 8, 8],          // 3 non-empty txs
            vec![0, 8, 8],          // 1 empty tx at the beginning
            vec![5, 0, 8],          // 1 empty tx in the middle
            vec![5, 8, 0],          // 1 empty tx at the end
            vec![5],                // 1 nonempty tx
            vec![0],                // 1 empty tx
            vec![],                 // zero txs
            vec![1000, 1000, 1000], // large payload
        ];

        setup_logging();
        setup_backtrace();
        let mut rng = jf_utils::test_rng();

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, lengths) in test_cases.into_iter().enumerate() {
            tracing::info!(
                "test payload {} of {} with {} txs",
                t + 1,
                num_test_cases,
                lengths.len(),
            );

            // prepare things as a function of the test case
            let entries = entries_from_lengths(&lengths);
            let tx_payloads_flat = random_bytes(tx_bodies_byte_len(&entries), &mut rng);
            let tx_bodies = extract_tx_payloads(&entries, &tx_payloads_flat);
            assert_eq!(tx_bodies.len(), entries.len());
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
    fn malformed_payloads() {
        // play with this
        let mut rng = jf_utils::test_rng();
        let test_cases = vec![
            // negative-length txs
            TestCase::from_entries(&[30, 10, 20], &mut rng), // 1 negative-length tx
            TestCase::from_entries(&[30, 20, 10], &mut rng), // 2 negative-length txs
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
                TxTableEntry::MAX.try_into().unwrap(),
                100,
                &mut rng,
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

            let block = BlockPayload::from_bytes(test_case.payload);
            assert_eq!(block.len(), test_case.num_txs);
            assert_eq!(block.payload.len(), payload_byte_len);

            let disperse_data = vid.disperse(&block.payload).unwrap();

            let mut tx_count: <BlockPayload as QueryablePayload>::TransactionIndex = 0; // test iterator correctness
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
            assert_eq!(test_case.num_txs, usize::try_from(tx_count).unwrap());

            // test: cannot make a proof for txs outside the tx table
            assert!(block.transaction_with_proof(&tx_count).is_none());
        }
    }

    #[test]
    fn malicious_tx_inclusion_proof() {
        setup_logging();
        setup_backtrace();

        let mut rng = jf_utils::test_rng();
        let test_case = TestCase::from_tx_table_len_unchecked(1, 3, &mut rng); // 3-byte payload too small to store tx table len
        let block = BlockPayload::from_bytes(test_case.payload.iter().cloned());
        assert_eq!(block.payload.len(), test_case.payload.len());
        assert_eq!(block.len(), test_case.num_txs);

        // test: cannot make a proof for such a small block
        assert!(block.transaction_with_proof(&0).is_none());

        let vid = test_vid_factory();
        let disperse_data = vid.disperse(&block.payload).unwrap();

        // make a fake proof for a nonexistent tx in the small block
        let tx = Transaction::new(crate::VmId(0), Vec::new());
        let proof = TxInclusionProof {
            tx_table_len: block.get_tx_table_len(),
            tx_table_len_proof: block.get_tx_table_len_proof(&vid).unwrap().clone(),
            tx_table_range_start: None,
            tx_table_range_end: TxTableEntry::from_usize(1),
            tx_table_range_proof: vid.payload_proof(&block.payload, 0..3).unwrap(),
            tx_payload_proof: None,
        };

        // test: fake proof should get rejected
        // TODO should return Some(Err()) instead of None
        assert!(proof
            .verify(&tx, 0, &vid, &disperse_data.commit, &disperse_data.common)
            .is_none());
    }

    struct TestCase {
        payload: Vec<u8>,
        num_txs: usize,
    }
    impl TestCase {
        /// Return a well-formed random block whose tx table is derived from `lengths`.
        #[allow(dead_code)]
        fn from_lengths<R: RngCore>(lengths: &[usize], rng: &mut R) -> Self {
            Self::from_entries(&entries_from_lengths(lengths), rng)
        }

        /// Return a random block whose tx table is derived from `entries`.
        ///
        /// If `entries` is well-formed then the result is well-formed.
        fn from_entries<R: RngCore>(entries: &[usize], rng: &mut R) -> Self {
            Self {
                payload: [
                    tx_table(entries),
                    random_bytes(tx_bodies_byte_len(entries), rng),
                ]
                .concat(),
                num_txs: entries.len(),
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
            Self {
                payload: [tx_table(entries), random_bytes(body_len, rng)].concat(),
                num_txs: entries.len(),
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
                tx_table_byte_len(entries) <= block_byte_len,
                "tx table size {} for entries {:?} exceeds block_byte_len {}",
                tx_table_byte_len(entries),
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
            let mut payload = tx_table(entries);
            let num_txs = if block_byte_len > payload.len() {
                payload.extend(random_bytes(block_byte_len - payload.len(), rng));
                entries.len()
            } else {
                payload.truncate(block_byte_len);
                (block_byte_len / TxTableEntry::byte_len()).saturating_sub(1)
            };
            Self { payload, num_txs }
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
            let tx_table_byte_len = (tx_table_len + 1) * TxTableEntry::byte_len();
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
            let header_byte_len = std::cmp::min(TxTableEntry::byte_len(), block_byte_len);
            let mut payload = vec![0; block_byte_len];
            rng.fill_bytes(&mut payload);
            payload[..header_byte_len].copy_from_slice(
                &TxTableEntry::from_usize(tx_table_len).to_bytes()[..header_byte_len],
            );
            Self {
                payload,
                num_txs: std::cmp::min(
                    tx_table_len,
                    (block_byte_len / TxTableEntry::byte_len()).saturating_sub(1),
                ),
            }
        }
    }

    mod helpers {
        use crate::block2::tx_table_entry::TxTableEntry;
        use rand::RngCore;

        pub fn tx_table(entries: &[usize]) -> Vec<u8> {
            let tx_table_byte_len = tx_table_byte_len(entries);
            let mut tx_table = Vec::with_capacity(tx_table_byte_len);
            tx_table.extend(TxTableEntry::from_usize(entries.len()).to_bytes());
            for entry in entries {
                tx_table.extend(TxTableEntry::from_usize(*entry).to_bytes());
            }
            assert_eq!(
                tx_table.len(),
                tx_table_byte_len,
                "bug in test code: unexpected tx table byte length"
            );
            tx_table
        }

        pub fn tx_table_byte_len(entries: &[usize]) -> usize {
            (entries.len() + 1) * TxTableEntry::byte_len()
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
            assert_eq!(result.len(), entries.len());
            result
        }
    }
}
