use crate::block2::{
    ns_table::{NsIndex, NsTable},
    num_txs::NumTxs,
    payload_bytes::NUM_TXS_BYTE_LEN,
    tx_iter::TxIndex,
    tx_table_entries::TxTableEntries,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

// TODO need to manually impl serde to [u8; NS_OFFSET_BYTE_LEN]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsPayloadRange(Range<usize>);

impl NsPayloadRange {
    // TODO newtype wrapper over return type?
    pub fn num_txs_range(&self) -> Range<usize> {
        let start = self.0.start;
        Range {
            start,
            end: start.saturating_add(NUM_TXS_BYTE_LEN).min(self.0.end),
        }
    }

    /// TODO explain: compute tx table entries range, translated by this namespace's start index
    pub fn tx_table_entries_range(&self, index: &TxIndex) -> Range<usize> {
        let result = index.tx_table_entries_range_relative();
        Range {
            start: result.start.saturating_add(self.0.start),
            end: result.end.saturating_add(self.0.start),
        }
    }

    /// Compute a subslice range for a tx payload, relative to an entire
    /// block payload.
    ///
    /// Returned range guaranteed to lay within this namespace's payload
    /// range.
    pub fn tx_payload_range(
        &self,
        num_txs: &NumTxs,
        tx_table_entries: &TxTableEntries,
    ) -> Range<usize> {
        let tx_payloads_start = num_txs
            .tx_table_byte_len_unchecked()
            .saturating_add(self.0.start);
        tx_table_entries.as_range(tx_payloads_start, self.0.end)
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
