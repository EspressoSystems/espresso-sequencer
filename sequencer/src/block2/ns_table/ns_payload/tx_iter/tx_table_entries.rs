use crate::block2::{
    ns_table::{
        ns_payload::{num_txs::NumTxs, tx_iter::TxIndex, NsPayload},
        ns_payload_range::NsPayloadRange,
    },
    payload_bytes::{
        num_txs_as_bytes, num_txs_from_bytes, tx_offset_as_bytes, tx_offset_from_bytes,
        NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
    },
};
use std::ops::Range;

/// manual serde as a byte array.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TxTableEntries {
    cur: usize,
    prev: Option<usize>,
}

/// Manual [`serde`] impl for [`TxTableEntries`].
mod tx_table_entries_serde {
    use crate::block2::{
        ns_table::ns_payload::tx_iter::tx_table_entries::TxTableEntries,
        payload_bytes::{tx_offset_as_bytes, tx_offset_from_bytes, TX_OFFSET_BYTE_LEN},
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Serialize, Deserialize)]
    struct TxTableEntriesSerde {
        cur: [u8; TX_OFFSET_BYTE_LEN],
        prev: Option<[u8; TX_OFFSET_BYTE_LEN]>,
    }

    impl Serialize for TxTableEntries {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            TxTableEntriesSerde {
                cur: tx_offset_as_bytes(self.cur),
                prev: self.prev.map(tx_offset_as_bytes),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for TxTableEntries {
        fn deserialize<D>(deserializer: D) -> Result<TxTableEntries, D::Error>
        where
            D: Deserializer<'de>,
        {
            <TxTableEntriesSerde as Deserialize>::deserialize(deserializer).map(|x| {
                TxTableEntries {
                    cur: tx_offset_from_bytes(&x.cur),
                    prev: x.prev.map(|bytes| tx_offset_from_bytes(&bytes)),
                }
            })
        }
    }
}

impl TxTableEntries {
    /// Infallible serialization.
    ///
    /// TODO same question as `NumTxs::as_bytes`
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(TX_OFFSET_BYTE_LEN.saturating_mul(2));
        if let Some(prev) = self.prev {
            bytes.extend(tx_offset_as_bytes(prev));
        }
        bytes.extend(tx_offset_as_bytes(self.cur));
        bytes
    }

    /// Convert a [`TxTableEntries`] to a valid [`Range`], translated and capped.
    ///
    /// Returned range guaranteed to satisfy `translate <= start <= end <= cap`.
    fn as_range(&self, translate: usize, cap: usize) -> Range<usize> {
        let end = self.cur.saturating_add(translate).min(cap);
        let start = self.prev.unwrap_or(0).saturating_add(translate).min(end);
        start..end
    }
}

impl NsPayload {
    /// Read the `index`th and `(index-1)`th entries from the tx table.
    ///
    /// TODO Panics if `index >= self.num_txs()`?
    pub fn read_tx_table_entries(&self, index: &TxIndex) -> TxTableEntries {
        let cur = self.read_tx_offset(index);
        let prev = if index.0 == [0; NUM_TXS_BYTE_LEN] {
            None
        } else {
            let prev_index = TxIndex(num_txs_as_bytes(num_txs_from_bytes(&index.0) - 1));
            Some(self.read_tx_offset(&prev_index))
        };
        TxTableEntries { cur, prev }
    }

    /// Read the `index`th entry from the tx table.
    fn read_tx_offset(&self, index: &TxIndex) -> usize {
        let start = tx_offset_from_bytes(&index.0) * TX_OFFSET_BYTE_LEN + NUM_TXS_BYTE_LEN;
        tx_offset_from_bytes(&self.0[start..start + TX_OFFSET_BYTE_LEN])
    }

    /// Read data on the `index`th tx from the tx table, sanitize that data
    /// into a valid range relative to the beginning of this namespace's
    /// payload.
    ///
    /// Returned range guaranteed to satisfy `start <= end <=
    /// namespace_byte_len`.
    ///
    /// Panics if `index >= self.num_txs()`.
    pub fn tx_payload_range_relative(&self, index: &TxIndex) -> Range<usize> {
        let tx_table_byte_len = self.read_num_txs().tx_table_byte_len_unchecked();
        self.read_tx_table_entries(index)
            .as_range(tx_table_byte_len, self.0.len())
    }
}

impl NsPayloadRange {
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
}
