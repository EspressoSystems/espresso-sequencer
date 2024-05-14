use crate::block2::{
    uint_bytes::{usize_from_bytes, usize_to_bytes},
    NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Range;

use super::newtypes::NumTxs2;

/// Index for an entry in a tx table.
///
/// Byte length same as [`NumTxs`].
///
/// Custom serialization and helper methods.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TxIndex(usize);

// TODO so much boilerplate for serde
impl Serialize for TxIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TxIndex {
    fn deserialize<D>(deserializer: D) -> Result<TxIndex, D::Error>
    where
        D: Deserializer<'de>,
    {
        <[u8; NUM_TXS_BYTE_LEN] as Deserialize>::deserialize(deserializer)
            .map(|bytes| TxIndex(usize_from_bytes::<NUM_TXS_BYTE_LEN>(&bytes)))
    }
}

impl TxIndex {
    /// Return a byte range into a tx table for use in a transaction proof.
    ///
    /// TODO move this method to NsPayloadRange, where it can be properly
    /// translated into the payload. Ugh I can't do that because some
    /// descendants depend on `NsPayload`! There's gotta be a better way to
    /// control visibility. TODO newtype for the returned range to ensure it's
    /// not accidentally miused?
    ///
    /// The returned range `R` is relative to the beginning of a payload for a
    /// namespace `N`. If `R` is to be used to retrieve bytes in a
    /// multi-namespace payload then `R` must be translated to the beginning of
    /// `N`.
    ///
    /// `R` covers one entry in the tx table if `self` is zero, otherwise it
    /// covers two entries.
    ///
    /// It is the responsibility of the caller to ensure that `R` is used only
    /// when `self` is less than the number of entries in `N`'s tx table.
    ///
    /// This method should be `const` but that's forbidden by Rust.
    ///
    /// # Tx table format (MOVE THIS DOC ELSEWHERE)
    ///
    /// The bytes in this range encode tx table entries that contain the
    /// (start,end) byte indices for the `tx_index`th transaction payload.
    ///
    /// The `tx_index`th entry in the tx table encodes the byte index of the
    /// *end* of this transaction's payload range. By deinition, this byte index
    /// is also the *start* of the *previous* transaction's payload range. Thus,
    /// the returned range includes `(tx_index - 1)`th and `tx_index`th entries
    /// of the tx table.
    ///
    /// Special case: If `tx_index` is 0 then the start index is implicitly 0,
    /// so the returned range contains only one entry from the tx table: the
    /// first entry of the tx table.
    pub fn tx_table_entries_range_relative(&self) -> Range<usize> {
        let start = if self.0 == 0 {
            // Special case: the desired range includes only one entry from
            // the tx table: the first entry. This entry starts immediately
            // following the bytes that encode the tx table length.
            NUM_TXS_BYTE_LEN
        } else {
            // The desired range starts at the beginning of the previous tx
            // table entry.
            (self.0 - 1)
                .saturating_mul(TX_OFFSET_BYTE_LEN)
                .saturating_add(NUM_TXS_BYTE_LEN)
        };
        // The desired range ends at the end of this transaction's tx table entry
        let end = self
            .0
            .saturating_add(1)
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN);
        start..end
    }

    /// Infallible serialization.
    ///
    /// TODO same question as [`NumTxs::as_bytes`]
    pub fn as_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes(self.0)
    }

    pub fn as_usize2(&self) -> usize {
        self.0
    }
}

pub struct TxIter(Range<usize>);

impl TxIter {
    // TODO args should be NumTxs, ns_payload_byte_len (newtype!)
    // TODO probably yet another newtype for NumTxsChecked(NumTxs, ns_payload_byte_len)
    pub fn new2(num_txs: &NumTxs2, ns_payload_byte_len: usize) -> Self {
        // TODO lame, use a newtype for NumTxsChecked
        Self(0..num_txs.num_txs(ns_payload_byte_len))
    }
}

// TODO explain: boilerplate `impl Iterator` delegates to `Range`
impl Iterator for TxIter {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(TxIndex)
    }
}
