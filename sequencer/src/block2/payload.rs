use crate::block2::entry::TxTableEntry;
use crate::block2::{
    get_ns_payload_range, get_ns_table_len, test_vid_factory, NamespaceProof, RangeProof,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use derivative::Derivative;
use jf_primitives::vid::payload_prover::PayloadProver;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::sync::OnceLock;

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
pub struct Payload<
    TableLen: CanonicalSerialize + CanonicalDeserialize + TryFrom<usize> + TryInto<usize> + std::marker::Sync,
    Offset: CanonicalSerialize + CanonicalDeserialize + TryFrom<usize> + TryInto<usize> + std::marker::Sync,
    NsId: CanonicalSerialize + CanonicalDeserialize + std::marker::Sync,
> {
    pub payload: Vec<u8>,

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

        let ns_payload_range = get_ns_payload_range(meta, ns_index, self.payload.len());

        let vid = test_vid_factory(); // TODO temporary VID construction

        // TODO log output for each `?`
        // fix this when we settle on an error handling pattern
        Some((
            self.payload.get(ns_payload_range.clone())?.to_vec(),
            vid.payload_proof(&self.payload, ns_payload_range).ok()?,
        ))
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
    pub fn get_tx_table_len(&self) -> TxTableEntry {
        let tx_table_len_range = self.tx_table_len_range();
        let mut entry_bytes = [0u8; TxTableEntry::byte_len()];
        entry_bytes[..tx_table_len_range.len()].copy_from_slice(&self.payload[tx_table_len_range]);

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
                vid.payload_proof(&self.payload, self.tx_table_len_range())
                    .ok()
            })
            .as_ref()
    }
}
