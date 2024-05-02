use crate::block2::{
    ns_table::{ns_payload::tx_iter::TxIndex, NsIndex, NsTable},
    payload_bytes::{NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN},
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

// TODO need to manually impl serde to [u8; NS_OFFSET_BYTE_LEN]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsPayloadRange(pub(crate) Range<usize>); // TODO temporary pub(crate) for tx_proof.rs

impl NsPayloadRange {
    // TODO newtype wrapper over return type?
    pub fn num_txs_range(&self) -> Range<usize> {
        let start = self.0.start;
        Range {
            start,
            end: start.saturating_add(NUM_TXS_BYTE_LEN).min(self.0.end),
        }
    }

    /// TODO fix: Byte length of this namespace's tx table.
    ///
    /// Guaranteed to be no larger than this namespace's payload byte length.
    ///
    /// TODO num_txs arg should be a newtype for [u8; NUM_TXS_BYTE_LEN]
    pub fn tx_table_byte_len(&self, num_txs: usize) -> usize {
        num_txs
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN)
            .min(self.0.len())
    }

    /// TODO explain: compute tx table entries range, translated by this namespace's start index
    pub fn tx_table_entries_range(&self, index: &TxIndex) -> Range<usize> {
        let result = index.tx_table_entries_range_relative();
        Range {
            start: result.start.saturating_add(self.0.start),
            end: result.end.saturating_add(self.0.start),
        }
    }

    /// TODO newtype wrapper `TxPayloadRange` like `NsPayloadRange`
    ///
    /// Read subslice range for the `index`th tx from the tx
    /// table, translated by [`ns_payload_range`].
    ///
    /// Returned range guaranteed to satisfy `start <= end <= namespace_byte_len`.
    ///
    /// Panics if `index >= self.num_txs()`.
    pub fn tx_payload_range(
        &self,
        num_txs: usize,        // TODO newtype. unchecked payload value, or corrected?
        tx_table_entry: usize, // TODO newtype. pair with `tx_table_entry_prev`?
        tx_table_entry_prev: usize, // TODO newtype. unchecked payload value, or corrected?
    ) -> Range<usize> {
        let tx_payloads_start = num_txs
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN) // tx_table_byte_len plus...
            .saturating_add(self.0.start); // ...namespace start

        let end = tx_table_entry
            .saturating_add(tx_payloads_start)
            .min(self.0.end);
        let start = tx_table_entry_prev
            .saturating_add(tx_payloads_start)
            .min(end);
        start..end

        // let result = self.tx_payload_range_relative(index);
    }

    pub fn as_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

impl NsTable {
    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    ///
    /// It is the responsibility of the caller to ensure that the `index`th
    /// entry is not a duplicate of a previous entry. Otherwise the returned
    /// range will be invalid. (Can the caller even create his own `NsIndex`??)
    ///
    /// Returned range guaranteed to satisfy `start <= end <=
    /// payload_byte_len`.
    ///
    /// TODO remove `payload_byte_len` arg and do not check `end`?
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn ns_payload_range(&self, index: &NsIndex, payload_byte_len: usize) -> NsPayloadRange {
        let end = self.read_ns_offset(index).min(payload_byte_len);
        let start = self.read_ns_offset_prev(index).unwrap_or(0).min(end);
        NsPayloadRange(start..end)
    }
}
