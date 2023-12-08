use self::tx_table_entry::TxTableEntry;
use crate::Transaction;
use ark_bls12_381::Bls12_381;
use derivative::Derivative;
use hotshot_query_service::availability::QueryablePayload;
use jf_primitives::{
    pcs::{prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::{
        advz::payload_prover::SmallRangeProof,
        payload_prover::{PayloadProver, Statement},
    },
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
    fn get_tx_table_len_proof(&self, vid: &boilerplate::VidScheme) -> Option<&RangeProof> {
        self.tx_table_len_proof
            .get_or_init(|| {
                vid.payload_proof(&self.payload, 0..TxTableEntry::byte_len())
                    .ok()
            })
            .as_ref()
    }
}

// Returns the range `range_start+len..max(range_start,range_end)+len` or `None` on error.
//
// Lots of ugly type conversion and checked arithmetic.
// Range end must be max(range_start,range_end) otherwise Rust will barf.
fn tx_payload_range(
    tx_table_range_start: &Option<TxTableEntry>,
    tx_table_range_end: &TxTableEntry,
    tx_table_len: &TxTableEntry,
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
    Some(start..std::cmp::max(start, end))
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

// TODO upstream type aliases: https://github.com/EspressoSystems/jellyfish/issues/423
type RangeProof =
    SmallRangeProof<<UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::Proof>;

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
    fn verify(
        &self,
        tx: &Transaction,
        tx_index: TxIndex,
        vid: &boilerplate::VidScheme,
        vid_commit: &boilerplate::VidSchemeCommit,
        vid_common: &boilerplate::VidSchemeCommon,
    ) -> Option<Result<(), ()>> {
        // Verify proof for tx payload.
        // Proof is `None` if and only if tx has zero length.
        match &self.tx_payload_proof {
            Some(tx_payload_proof) => {
                let tx_payload_range = tx_payload_range(
                    &self.tx_table_range_start,
                    &self.tx_table_range_end,
                    &self.tx_table_len,
                )?;
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
                if !tx.payload().is_empty() {
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
    use jf_primitives::{pcs::checked_fft_size, vid::advz::Advz};
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

    // TODO temporary
    pub(super) fn test_vid_factory() -> Advz<Bls12_381, sha2::Sha256> {
        let (payload_chunk_size, num_storage_nodes) = (8, 10);

        let mut rng = jf_utils::test_rng();
        let srs = UnivariateKzgPCS::<Bls12_381>::gen_srs_for_testing(
            &mut rng,
            checked_fft_size(payload_chunk_size - 1).unwrap(),
        )
        .unwrap();
        Advz::<_, _>::new(payload_chunk_size, num_storage_nodes, srs).unwrap()
    }

    pub(super) type VidScheme = Advz<Bls12_381, sha2::Sha256>;
    pub(super) type VidSchemeCommit = <VidScheme as jf_primitives::vid::VidScheme>::Commit;
    pub(super) type VidSchemeCommon = <VidScheme as jf_primitives::vid::VidScheme>::Common;
}

#[cfg(test)]
mod test {
    use super::{
        boilerplate::test_vid_factory, BlockPayload, QueryablePayload, Transaction, TxIndex,
        TxTableEntry,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use jf_primitives::vid::VidScheme;
    use rand::RngCore;

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            vec![5, 13, 21], // 3 non-empty txs
            vec![0, 8, 16],  // 1 empty tx at the beginning
            vec![5, 5, 13],  // 1 empty tx in the middle
            vec![5, 13, 13], // 1 empty tx at the end
            vec![5],         // 1 nonempty tx
            vec![0],         // 1 empty tx
            vec![],          // zero txs
        ];

        setup_logging();
        setup_backtrace();
        let mut rng = jf_utils::test_rng();

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, tx_table_entries) in test_cases.into_iter().enumerate() {
            tracing::info!(
                "test block payload {} of {} with {} txs",
                t + 1,
                num_test_cases,
                tx_table_entries.len()
            );

            // prepare things as a function of the test case
            let tx_payloads_flat = random_tx_payloads_flat(&tx_table_entries, &mut rng);
            let tx_bodies = extract_tx_payloads(&tx_table_entries, &tx_payloads_flat);
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
            for (index_usize, tx_body) in tx_bodies.iter().enumerate() {
                let index = TxIndex::try_from(index_usize).unwrap();
                tracing::info!("tx index {}", index,);

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
        }
    }

    #[test]
    fn malformed_payloads() {
        // play with this
        let test_cases = vec![
            vec![10, 9, 20], // 1 negative-length tx
            vec![10, 9, 5],  // 2 negative-length txs
        ];

        // TODO more test cases:
        // - overflow u32
        // - txs off the end of the payload
        // - valid tx proof P made from large payload, checked against a prefix of that payload where P is invalid

        setup_logging();
        setup_backtrace();

        let mut rng = jf_utils::test_rng();
        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, tx_table_entries) in test_cases.into_iter().enumerate() {
            tracing::info!(
                "test payload {} of {} with {} txs",
                t + 1,
                num_test_cases,
                tx_table_entries.len()
            );

            let tx_table = tx_table(&tx_table_entries);
            let tx_payloads_flat = random_tx_payloads_flat(&tx_table_entries, &mut rng);
            let tx_bodies = extract_tx_payloads(&tx_table_entries, &tx_payloads_flat);

            let block = BlockPayload::from_bytes([tx_table, tx_payloads_flat].concat());
            let disperse_data = vid.disperse(&block.payload).unwrap();

            for (i, tx_body) in tx_bodies.iter().enumerate() {
                let index = TxIndex::try_from(i).unwrap();
                tracing::info!("tx index {}", index,);

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
        }
    }

    fn tx_table(entries: &[usize]) -> Vec<u8> {
        let mut tx_table = Vec::with_capacity(entries.len() + TxTableEntry::byte_len());
        tx_table.extend(TxTableEntry::from_usize(entries.len()).to_bytes());
        for entry in entries {
            tx_table.extend(TxTableEntry::from_usize(*entry).to_bytes());
        }
        tx_table
    }

    fn random_tx_payloads_flat<R>(entries: &[usize], rng: &mut R) -> Vec<u8>
    where
        R: RngCore,
    {
        // lergest entry dictates size of tx bodies
        let mut result = vec![0; *entries.iter().max().unwrap_or(&0)];
        rng.fill_bytes(&mut result);
        result
    }

    fn extract_tx_payloads(entries: &[usize], tx_payloads_flat: &[u8]) -> Vec<Vec<u8>> {
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
