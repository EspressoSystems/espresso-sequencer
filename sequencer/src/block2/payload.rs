use crate::block2::entry::TxTableEntry;
use crate::block2::{
    get_ns_payload_range, get_ns_table_len, test_vid_factory, NamespaceProof, RangeProof,
};
use crate::VmId;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use commit::Committable;
use derivative::Derivative;
use jf_primitives::vid::payload_prover::PayloadProver;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::sync::OnceLock;
use std::{collections::HashMap, fmt::Display, ops::Range};

#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
// TODO (Philippe) make it private?
pub struct NamespaceInfo {
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

    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[serde(skip)]
    tx_bytes_end: TxTableEntry,
    pub(crate) tx_table_len: TxTableEntry,
}

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
pub struct Payload<
    TableLen: CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
        + std::marker::Sync,
    Offset: CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
        + std::marker::Sync,
    NsId: CanonicalSerialize + CanonicalDeserialize + Default + std::marker::Sync,
> {
    // Sequence of bytes representing the concatenated payloads for each namespace
    #[serde(skip)]
    pub raw_payload: Vec<u8>,

    // Sequence of bytes representing the namespace table
    pub raw_namespace_table: Vec<u8>,

    #[derivative(Hash = "ignore")]
    pub namespaces: HashMap<VmId, NamespaceInfo>,

    // cache frequently used items
    //
    // TODO type should be `OnceLock<RangeProof>` instead of `OnceLock<Option<RangeProof>>`. We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[serde(skip)]
    pub tx_table_len_proof: OnceLock<Option<RangeProof>>,
    pub table_len: TableLen,
    pub offset: Offset,
    pub ns_id: NsId,
}

impl Payload<u32, u32, [u8; 32]> {
    pub fn new() -> Self {
        Self {
            raw_payload: vec![],
            raw_namespace_table: vec![],
            namespaces: HashMap::new(),
            tx_table_len_proof: Default::default(),
            table_len: Default::default(),
            offset: Default::default(),
            ns_id: Default::default(),
        }
    }
    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    pub fn num_namespaces(&self, ns_table_bytes: &[u8]) -> usize {
        get_ns_table_len(ns_table_bytes)
    }

    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    pub fn namespace_iter(&self, ns_table_bytes: &[u8]) -> impl Iterator<Item = usize> {
        0..get_ns_table_len(ns_table_bytes)
    }

    // TODO dead code even with `pub` because this module is private in lib.rs
    #[allow(dead_code)]
    /// Returns (ns_payload, ns_proof) where ns_payload is raw bytes.
    pub fn namespace_with_proof(
        &self,
        meta: &<Self as hotshot_types::traits::BlockPayload>::Metadata,
        ns_index: usize,
    ) -> Option<(Vec<u8>, NamespaceProof)> {
        if ns_index >= get_ns_table_len(meta) {
            return None; // error: index out of bounds
        }

        let ns_payload_range = get_ns_payload_range(meta, ns_index, self.raw_payload.len());

        let vid = test_vid_factory(); // TODO temporary VID construction

        // TODO log output for each `?`
        // fix this when we settle on an error handling pattern
        Some((
            self.raw_payload.get(ns_payload_range.clone())?.to_vec(),
            vid.payload_proof(&self.raw_payload, ns_payload_range)
                .ok()?,
        ))
    }

    /// Return a range `r` such that `self.payload[r]` is the bytes of the tx table length.
    ///
    /// Typically `r` is `0..TxTableEntry::byte_len()`.
    /// But it might differ from this if the payload byte length is less than `TxTableEntry::byte_len()`.
    fn tx_table_len_range(&self) -> Range<usize> {
        0..std::cmp::min(TxTableEntry::byte_len(), self.raw_payload.len())
    }

    /// Return length of the tx table, read from the payload bytes.
    ///
    /// This quantity equals number of txs in the payload.
    pub fn get_tx_table_len(&self) -> TxTableEntry {
        let tx_table_len_range = self.tx_table_len_range();
        let mut entry_bytes = [0u8; TxTableEntry::byte_len()];
        entry_bytes[..tx_table_len_range.len()]
            .copy_from_slice(&self.raw_payload[tx_table_len_range]);

        TxTableEntry::from_bytes_array(entry_bytes)
    }
    fn _get_tx_table_len_as<T>(&self) -> Option<T>
    where
        TxTableEntry: TryInto<T>,
    {
        self.get_tx_table_len().try_into().ok()
    }

    // Fetch the tx table length range proof from cache.
    // Build the proof if missing from cache.
    // Returns `None` if an error occurred.
    pub fn get_tx_table_len_proof(
        &self,
        vid: &impl PayloadProver<RangeProof>,
    ) -> Option<&RangeProof> {
        self.tx_table_len_proof
            .get_or_init(|| {
                vid.payload_proof(&self.raw_payload, self.tx_table_len_range())
                    .ok()
            })
            .as_ref()
    }

    pub fn update_namespace_with_tx(
        &mut self,
        tx: <Payload<u32, u32, [u8; 32]> as hotshot::traits::BlockPayload>::Transaction,
    ) {
        let tx_bytes_len: TxTableEntry = tx.payload().len().try_into().unwrap(); // TODO (Philippe) error handling

        let namespace = self.namespaces.entry(tx.vm()).or_insert(NamespaceInfo {
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
}

impl Display for Payload<u32, u32, [u8; 32]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Payload<u32, u32, [u8; 32]> {
    fn commit(&self) -> commit::Commitment<Self> {
        todo!()
    }
}
