use self::{boilerplate::RangeProof, tx_table_entry::TxTableEntry};
use crate::{Transaction, VmId};
use derivative::Derivative;
use hotshot_query_service::availability::QueryablePayload;
use jf_primitives::{
    pcs::{prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::payload_prover::{PayloadProver, Statement},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Range, sync::OnceLock};

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
    /// Returns (Self, metadata).
    ///
    /// `metadata` is a bytes representation of the namespace table defined for j>0:
    /// word[0]:    [number of entries in namespace table]
    /// word[2j-1]: [id for the jth namespace]
    /// word[2j]:   [end byte index of the jth namespace in the payload]
    ///
    /// Thus, for j>2 the jth namespace payload bytes range is word[2(j-1)]..word[2j].
    /// Edge case: for j=1 the jth namespace start index is implicitly 0.
    ///
    /// Word type is `TxTableEntry`.
    ///
    /// TODO(746) it's Vec<u8> to make it easy to move metadata into payload in the future.
    /// TODO(746) don't use `TxTableEntry`; make a different type
    /// TODO(746) refactor and make pretty "table" code for tx, namespace tables.
    /// TODO(746) should metadata be included in hash, serde, etc?
    fn from_txs(txs: impl IntoIterator<Item = Transaction>) -> Option<(Self, Vec<u8>)> {
        struct NamespaceInfo {
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
            tx_table: Vec<u8>,
            // concatenation of all tx payloads
            tx_bodies: Vec<u8>,
            tx_bytes_end: TxTableEntry,
            tx_table_len: TxTableEntry,
        }

        // TODO(746) do we want deterministic namespace table ordering? If so then we need BTreeMap instead of HashMap.
        let mut namespaces: HashMap<VmId, NamespaceInfo> = HashMap::new();
        for tx in txs.into_iter() {
            let tx_bytes_len: TxTableEntry = tx.payload().len().try_into().ok()?;

            let namespace = namespaces.entry(tx.vm()).or_insert(NamespaceInfo {
                tx_table: Vec::new(),
                tx_bodies: Vec::new(),
                tx_bytes_end: TxTableEntry::zero(),
                tx_table_len: TxTableEntry::zero(),
            });

            namespace.tx_bytes_end.checked_add_mut(tx_bytes_len)?;
            namespace.tx_table.extend(namespace.tx_bytes_end.to_bytes());
            namespace.tx_bodies.extend(tx.payload());
            namespace
                .tx_table_len
                .checked_add_mut(TxTableEntry::one())?;
        }

        // first word of namespace table is its length
        let namespace_table_len = namespaces.len();
        let mut namespace_table =
            Vec::from(TxTableEntry::try_from(namespace_table_len).ok()?.to_bytes());

        // fill payload and namespace table
        // TODO(746) is it worth the trouble to allocate memory in advance?
        let mut payload = Vec::new();
        let mut namespace_bytes_end = TxTableEntry::zero();
        for (id, namespace) in namespaces {
            let prev_len = payload.len();
            payload.extend(namespace.tx_table_len.to_bytes());
            payload.extend(namespace.tx_table);
            payload.extend(namespace.tx_bodies);
            namespace_table.extend(TxTableEntry::try_from(id).ok()?.to_bytes());
            namespace_bytes_end.checked_add_mut((payload.len() - prev_len).try_into().ok()?)?;
            namespace_table.extend(namespace_bytes_end.to_bytes());
        }

        Some((
            Self {
                payload,
                tx_table_len_proof: Default::default(),
            },
            namespace_table,
        ))
    }

    fn from_bytes<B>(bytes: B) -> Self
    where
        B: IntoIterator<Item = u8>,
    {
        Self {
            payload: bytes.into_iter().collect(),
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
        tracing::info!("tx_table_range_proof_start");

        // end
        tracing::info!("2");
        let tx_table_range_proof_end = index_usize
            .checked_add(2)?
            .checked_mul(TxTableEntry::byte_len())?;
        let tx_table_range_end = TxTableEntry::from_bytes(self.payload.get(
            tx_table_range_proof_end.checked_sub(TxTableEntry::byte_len())?
                ..tx_table_range_proof_end,
        )?)?;

        // correctness proof for the tx payload range
        tracing::info!("3");
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
        tracing::info!("4");
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
    use crate::VmId;
    use core::fmt;
    use std::mem::size_of;

    // Use newtype pattern so that tx table entires cannot be confused with other types.
    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct TxTableEntry(TxTableEntryWord);
    type TxTableEntryWord = u32;

    impl TxTableEntry {
        pub const MAX: TxTableEntry = Self(TxTableEntryWord::MAX);

        /// Adds `rhs` to `self` in place. Returns `None` on overflow.
        pub fn checked_add_mut(&mut self, rhs: Self) -> Option<()> {
            self.0 = self.0.checked_add(rhs.0)?;
            Some(())
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

    impl fmt::Display for TxTableEntry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
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

    impl TryFrom<VmId> for TxTableEntry {
        type Error = <TxTableEntryWord as TryFrom<u64>>::Error;

        fn try_from(value: VmId) -> Result<Self, Self::Error> {
            TxTableEntryWord::try_from(value.0).map(Self)
        }
    }
    impl TryFrom<TxTableEntry> for VmId {
        type Error = <u64 as TryFrom<TxTableEntryWord>>::Error;

        fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
            u64::try_from(value.0).map(Self)
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
        type Metadata = Vec<u8>;
        type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

        fn from_transactions(
            transactions: impl IntoIterator<Item = Self::Transaction>,
        ) -> Result<(Self, Self::Metadata), Self::Error> {
            Self::from_txs(transactions).context(BlockBuildingSnafu)
        }

        // TODO(746) from_bytes doesn't need `metadata`!
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
    use crate::VmId;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use helpers::*;
    use jf_primitives::vid::{payload_prover::PayloadProver, VidScheme};
    use rand::RngCore;
    use std::{collections::HashMap, ops::Range};

    #[test]
    fn basic_correctness() {
        // play with this
        let test_cases = vec![
            // 1 namespace only
            // vec![vec![5, 8, 8]], // 3 non-empty txs
            // vec![vec![0, 8, 8]], // 1 empty tx at the beginning
            // vec![vec![5, 0, 8]], // 1 empty tx in the middle
            // vec![vec![5, 8, 0]], // 1 empty tx at the end
            // vec![vec![5]],       // 1 nonempty tx
            // vec![vec![0]],       // 1 empty tx
            // // vec![],                 // zero txs
            // vec![vec![1000, 1000, 1000]], // large payload
            // multiple namespaces
            vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8]], // 3 non-empty namespaces
        ];
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
            tx_table_derived: Vec<TxTableEntry>,
            tx_payloads: Vec<Vec<u8>>,
        }

        let vid = test_vid_factory();
        let num_test_cases = test_cases.len();
        for (t, test_case) in test_cases.iter().enumerate() {
            // derive a bunch of stuff for this test case
            let mut txs = Vec::new();
            let mut derived_nss = HashMap::new();
            for (n, tx_lengths) in test_case.iter().enumerate() {
                tracing::info!(
                    "test block {} of {} namespace {} of {} with {} txs",
                    t + 1,
                    num_test_cases,
                    n + 1,
                    test_case.len(),
                    tx_lengths.len(),
                );

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
                tracing::info!("ns {} tx_table_derived {:?}", n, tx_table_derived);

                // derive this namespace's payload
                let ns_payload_flat = {
                    let mut ns_payload = Vec::new();

                    // write tx table bytes
                    ns_payload.extend(TxTableEntry::from_usize(tx_table_derived.len()).to_bytes());
                    for entry in tx_table_derived.iter() {
                        // ns_payload.extend(TxTableEntry::try_from(n).unwrap().to_bytes());
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
                        tx_table_derived,
                        tx_payloads,
                    },
                );
                assert!(already_exists.is_none());
            }
            assert_eq!(derived_nss.len(), test_case.len());

            let (block, actual_ns_table) = BlockPayload::from_txs(txs).unwrap();

            // test actual stuff against derived stuff

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

            // Test namespace table contents.
            //
            // We cannot derive the namespace table ourselves
            // because the order of entries in the namespace table is nondeterministic.
            // Thus, we need to need to mirror the order of entries produced by `BlockPayload::from_txs`.
            tracing::info!("actual_ns_table {:?}", actual_ns_table);
            let mut prev_entry = TxTableEntry::zero();
            for (ns_id, entry) in ns_table_iter(&actual_ns_table) {
                // test namespace payload
                let actual_ns_payload_range = Range {
                    start: usize::try_from(prev_entry.clone()).unwrap(),
                    end: usize::try_from(entry.clone()).unwrap(),
                };
                let actual_ns_payload = block.payload.get(actual_ns_payload_range).unwrap();
                let derived_ns_payload = &derived_nss.get(&ns_id).unwrap().payload_flat;
                assert_eq!(
                    actual_ns_payload, derived_ns_payload,
                    "namespace {} incorrect payload bytes expect {:?} got {:?}",
                    ns_id.0, derived_ns_payload, actual_ns_payload
                );
                prev_entry = entry;
            }

            // test namespace payloads
            let disperse_data = vid.disperse(&block.payload).unwrap();
            let mut tx_index_offset = 0;
            for (ns_id, ns) in derived_nss.iter() {
                // test tx table length
                let tx_table_len_bytes = &ns.payload_flat[..TxTableEntry::byte_len()];
                let tx_table_len = TxTableEntry::from_bytes(tx_table_len_bytes).unwrap();
                assert_eq!(
                    tx_table_len,
                    TxTableEntry::try_from(ns.tx_table_derived.len()).unwrap(),
                    "namespace {} tx table length expect {} got {}",
                    ns_id.0,
                    ns.tx_table_derived.len(),
                    tx_table_len
                );

                // test tx table contents
                let tx_table_bytes = &ns.payload_flat[TxTableEntry::byte_len()
                    ..(ns.tx_table_derived.len() + 1) * TxTableEntry::byte_len()];
                tracing::info!("ns tx table bytes {:?}", tx_table_bytes);
                let tx_table: Vec<TxTableEntry> = tx_table_bytes
                    .chunks(TxTableEntry::byte_len())
                    .map(|bytes| TxTableEntry::from_bytes(bytes).unwrap())
                    .collect();
                assert_eq!(
                    tx_table, ns.tx_table_derived,
                    "incorrect tx table for namespace {}",
                    ns_id.0
                );

                // tests for individual txs in this namespace
                let start = tx_table.len() * TxTableEntry::byte_len();
                // let mut block_iter = block.iter(); // test iterator correctness
                for (tx_index, tx_payload) in ns.tx_payloads.iter().enumerate() {
                    // assert!(block_iter.next().is_some());
                    let tx_index = TxIndex::try_from(tx_index + tx_index_offset).unwrap();
                    tracing::info!("tx index {}", tx_index,);

                    // test `transaction_with_proof()`
                    // let (tx, proof) = block.transaction_with_proof(&tx_index).unwrap();
                    // assert_eq!(tx_payload, tx.payload());
                    // proof
                    //     .verify(
                    //         &tx,
                    //         tx_index,
                    //         &vid,
                    //         &disperse_data.commit,
                    //         &disperse_data.common,
                    //     )
                    //     .unwrap()
                    //     .unwrap();
                }
                // assert!(block_iter.next().is_none());
                tx_index_offset += tx_table.len();
            }

            // test full block payload
            // assert_eq!(tx_index_offset, block.len());
            assert_eq!(
                block.payload,
                ns_table_iter(&actual_ns_table)
                    .flat_map(|(ns_id, _)| { derived_nss.remove(&ns_id).unwrap().payload_flat })
                    .collect::<Vec<u8>>()
            );
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
        use crate::{block2::tx_table_entry::TxTableEntry, VmId};
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
            assert_eq!(
                result.len(),
                entries.len(),
                "bug in test code: expect to extract {} txs but got {}",
                entries.len(),
                result.len()
            );
            result
        }

        pub fn ns_table_iter(
            ns_table_bytes: &[u8],
        ) -> impl Iterator<Item = (VmId, TxTableEntry)> + '_ {
            ns_table_bytes[TxTableEntry::byte_len()..] // first few bytes is the table lengh, skip that
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
