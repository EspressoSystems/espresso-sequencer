use self::tx_table_entry::TxTableEntry;
use crate::Transaction;
use ark_bls12_381::Bls12_381;
use derivative::Derivative;
use hotshot_query_service::QueryableBlock;
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

    // Return the range `r` such that the `index`th tx bytes are at `self.payload[r]`.
    fn get_tx_range_with_proof(
        &self,
        index: TxIndex,
        vid: &boilerplate::VidScheme,
    ) -> Option<(Range<usize>, RangeProof)> {
        // check args
        if index >= self.get_tx_table_len()?.try_into().ok()? {
            return None;
        }

        let tx_bodies_offset = self.tx_bodies_offset()?;

        // edge case index == 0
        // TODO refactor copied code
        if index == 0 {
            // The first entry in the tx table is the table length, so add 1
            let start = usize::try_from(index.checked_add(1)?)
                .ok()?
                .checked_mul(TxTableEntry::byte_len())?;
            let end = start.checked_add(TxTableEntry::byte_len())?;
            let tx_end: usize = TxTableEntry::from_bytes(self.payload.get(start..end)?)?
                .try_into()
                .ok()?;
            let tx_end_proof = vid.payload_proof(&self.payload, start..end).ok()?;
            return Some((
                (tx_bodies_offset..tx_end.checked_add(tx_bodies_offset)?),
                tx_end_proof,
            ));
        }

        // The first entry in the tx table is the table length, so add 1
        let start = usize::try_from(index)
            .ok()?
            .checked_mul(TxTableEntry::byte_len())?;
        let mid = start.checked_add(TxTableEntry::byte_len())?;
        let tx_start: usize = TxTableEntry::from_bytes(self.payload.get(start..mid)?)?
            .try_into()
            .ok()?;
        let end = mid.checked_add(TxTableEntry::byte_len())?;
        let tx_end: usize = TxTableEntry::from_bytes(self.payload.get(mid..end)?)?
            .try_into()
            .ok()?;
        let tx_range_proof = vid.payload_proof(&self.payload, start..end).ok()?;
        Some((
            (tx_start.checked_add(tx_bodies_offset)?..tx_end.checked_add(tx_bodies_offset)?),
            tx_range_proof,
        ))
    }

    // Return length of the tx table
    // == number of txs in the payload
    // == the first `TxTableEntry` word of `self.payload`.
    fn get_tx_table_len(&self) -> Option<usize> {
        TxTableEntry::from_bytes(self.payload.get(0..TxTableEntry::byte_len())?)?
            .try_into()
            .ok()
    }

    // Return the byte index in `self.payload` of the start of the tx bodies (after the tx table).
    fn tx_bodies_offset(&self) -> Option<usize> {
        self.get_tx_table_len()?
            .checked_add(1)?
            .checked_mul(TxTableEntry::byte_len())
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

impl QueryableBlock for BlockPayload {
    type TransactionIndex = u32;
    type Iter<'a> = Range<Self::TransactionIndex>;
    type InclusionProof = TxInclusionProof;

    fn len(&self) -> usize {
        self.get_tx_table_len().unwrap_or(0)
    }

    fn iter(&self) -> Self::Iter<'_> {
        0..self.get_tx_table_len().unwrap_or(0).try_into().unwrap_or(0)
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        let vid = boilerplate::test_vid_factory(); // TODO temporary VID construction

        let (tx_range, tx_table_range_proof) = self.get_tx_range_with_proof(*index, &vid)?;

        Some((
            // TODO don't copy the tx bytes into the return value
            // https://github.com/EspressoSystems/hotshot-query-service/issues/267
            Transaction::new(crate::VmId(0), self.payload.get(tx_range.clone())?.to_vec()),
            TxInclusionProof {
                tx_table_len_proof: self.get_tx_table_len_proof(&vid)?.clone(),
                tx_table_range_proof,
                tx_payload_proof: if tx_range.is_empty() {
                    None
                } else {
                    vid.payload_proof(&self.payload, tx_range).ok()
                },
            },
        ))
    }
}

type TxIndex = <BlockPayload as QueryableBlock>::TransactionIndex;

// TODO upstream type aliases: https://github.com/EspressoSystems/jellyfish/issues/423
type RangeProof =
    SmallRangeProof<<UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::Proof>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxInclusionProof {
    tx_table_len_proof: RangeProof,
    tx_table_range_proof: RangeProof,
    tx_payload_proof: Option<RangeProof>, // `None` if the tx has zero length
}

impl TxInclusionProof {
    // TODO(795) prototype only!
    //
    // - Obnoxious arg list; where to store all this cruft?
    // - Returns `None` if an error occurred.
    // - Use of `Result<(),()>` pattern to indicate a must-use bool return type.
    #[allow(dead_code)] // TODO temporary
    #[allow(clippy::too_many_arguments)]
    fn verify(
        &self,
        tx: &Transaction,
        vid: &boilerplate::VidScheme,
        vid_commit: &boilerplate::VidSchemeCommit,
        vid_common: &boilerplate::VidSchemeCommon,
        tx_index: TxIndex,
        tx_payload_range: Range<usize>,
        tx_table_len_bytes: &[u8],
        tx_table_entry_start_bytes: &[u8],
        tx_table_entry_end_bytes: &[u8],
    ) -> Option<Result<(), ()>> {
        // verify tx payload proof
        // proof is `None` if and only if tx has zero length.
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
                    // TODO(795) it would be nice to use ? here...
                    return Some(Err(()));
                }
            }
            None => {
                if !tx.payload().is_empty() {
                    return None; // error
                }
            }
        };

        // verify tx table len proof
        if vid
            .payload_verify(
                Statement {
                    payload_subslice: tx_table_len_bytes,
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

        // verify tx table entry proof
        let index: usize = tx_index.try_into().ok()?;

        // TODO don't use bytes, it's ugly
        let bytes = [tx_table_entry_start_bytes, tx_table_entry_end_bytes].concat();
        let (range, bytes) = if index == 0 {
            // start index is 0
            if !tx_table_entry_start_bytes.is_empty() {
                return None;
            }
            (
                TxTableEntry::byte_len()..2 * TxTableEntry::byte_len(),
                tx_table_entry_end_bytes,
            )
        } else {
            // TODO checked...
            (
                index * TxTableEntry::byte_len()..(index + 2) * TxTableEntry::byte_len(),
                bytes.as_slice(),
            )
        };
        if vid
            .payload_verify(
                Statement {
                    payload_subslice: bytes,
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
    use std::mem::size_of;

    // Use newtype pattern so that tx table entires cannot be confused with other types.
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    impl TryFrom<super::TxIndex> for TxTableEntry {
        type Error = <TxTableEntryWord as TryFrom<super::TxIndex>>::Error;

        fn try_from(value: super::TxIndex) -> Result<Self, Self::Error> {
            TxTableEntryWord::try_from(value).map(Self)
        }
    }
}

mod boilerplate {
    use super::{BlockPayload, PolynomialCommitmentScheme, Transaction, UnivariateKzgPCS};
    use ark_bls12_381::Bls12_381;
    use commit::Committable;
    use jf_primitives::{pcs::checked_fft_size, vid::advz::Advz};
    use std::fmt::Display;

    // TODO temporary.
    // `Block` trait subject to change/delection.
    // Skeleton impl for now so as to enable `QueryableBlock`.
    impl hotshot::traits::Block for BlockPayload {
        type Error = crate::Error;
        type Transaction = Transaction;

        fn new() -> Self {
            todo!()
        }

        fn add_transaction_raw(
            &self,
            _tx: &Self::Transaction,
        ) -> std::result::Result<Self, Self::Error> {
            todo!()
        }

        fn contained_transactions(
            &self,
        ) -> std::collections::HashSet<commit::Commitment<Self::Transaction>> {
            todo!()
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
        boilerplate::test_vid_factory, BlockPayload, QueryableBlock, Transaction, TxIndex,
        TxTableEntry,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use jf_primitives::vid::VidScheme;

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            // 3 non-empty txs
            vec![
                vec![0, 1, 2, 3, 4],
                vec![5, 6, 7, 8, 9, 10, 11, 12],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx at the beginning
            vec![
                vec![],
                vec![5, 6, 7, 8, 9, 10, 11, 12],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx in the middle
            vec![
                vec![0, 1, 2, 3, 4],
                vec![],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx at the end
            vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9, 10, 11, 12], vec![]],
            // 1 nonempty tx
            vec![vec![0, 1, 2, 3, 4]],
            // 1 empty tx
            vec![vec![]],
            // zero txs
            vec![],
        ];

        setup_logging();
        setup_backtrace();

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, tx_bodies) in test_cases.into_iter().enumerate() {
            tracing::info!(
                "test payload {} of {} with {} txs",
                t + 1,
                num_test_cases,
                tx_bodies.len()
            );

            // prepare things as a function of the test case
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
            let tx_payloads_flat: Vec<u8> = tx_bodies.iter().flatten().cloned().collect();
            assert_eq!(payload, tx_payloads_flat);
            assert_eq!(tx_bodies.len(), block.len());

            // tests for individual txs
            let disperse_data = vid.disperse(&block.payload).unwrap();
            for (index_usize, tx_body) in tx_bodies.iter().enumerate() {
                let index = TxIndex::try_from(index_usize).unwrap();

                // test get_tx_range_with_proof()
                let tx_range = block.get_tx_range_with_proof(index, &vid).unwrap().0;
                let block_tx_body = block.payload.get(tx_range.clone()).unwrap();
                assert_eq!(tx_body, block_tx_body);

                tracing::info!(
                    "test: index {} tx range start {} end {}",
                    index,
                    tx_range.start,
                    tx_range.end
                );

                // test `transaction_with_proof()`
                let (tx, proof) = block.transaction_with_proof(&index).unwrap();
                assert_eq!(tx_body, tx.payload());

                // TODO(795): verify() is ugly. This cruft is temporary.
                let tx_table_entry_start_bytes = if index == 0 {
                    &[]
                } else {
                    block
                        .payload
                        .get(
                            index_usize * TxTableEntry::byte_len()
                                ..(index_usize + 1) * TxTableEntry::byte_len(),
                        )
                        .unwrap()
                };
                let tx_table_entry_end_bytes = block
                    .payload
                    .get(
                        (index_usize + 1) * TxTableEntry::byte_len()
                            ..(index_usize + 2) * TxTableEntry::byte_len(),
                    )
                    .unwrap();

                proof
                    .verify(
                        &tx,
                        &vid,
                        &disperse_data.commit,
                        &disperse_data.common,
                        index,
                        tx_range,
                        tx_table_len_bytes,
                        tx_table_entry_start_bytes,
                        tx_table_entry_end_bytes,
                    )
                    .unwrap()
                    .unwrap();
            }
        }
    }
}
