use super::payload_bytes::{
    num_txs_from_bytes, tx_offset_from_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
use crate::{NamespaceId, Transaction};
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub fn parse_ns_payload(ns_payload: &[u8], ns_id: NamespaceId) -> Vec<Transaction> {
    TxIter::new(ns_payload)
        .map(|info| Transaction::new(ns_id, ns_payload[info.tx_range].to_vec()))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxIndex {
    pub(super) tx_range: Range<usize>,
}

pub struct TxIter<'a> {
    tx_table_start: usize,   // byte index into the tx table
    tx_payload_start: usize, // byte index into the tx payloads
    tx_table_byte_len: usize,
    ns_payload: &'a [u8],
}

impl<'a> TxIter<'a> {
    pub fn new(ns_payload: &'a [u8]) -> Self {
        let tx_table_byte_len = if ns_payload.len() < NUM_TXS_BYTE_LEN {
            // `ns_table` is too short to store the number of txs.
            // So there are zero txs in this namespace.
            0
        } else {
            std::cmp::min(
                num_txs_from_bytes(&ns_payload[..NUM_TXS_BYTE_LEN])
                    .saturating_mul(TX_OFFSET_BYTE_LEN)
                    .saturating_add(NUM_TXS_BYTE_LEN),
                ns_payload.len(),
            )
        };

        Self {
            tx_table_start: NUM_TXS_BYTE_LEN,
            tx_payload_start: tx_table_byte_len,
            tx_table_byte_len,
            ns_payload,
        }
    }
}

impl<'a> Iterator for TxIter<'a> {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        // this iterator is done if there's not enough room for another entry in
        // the tx_table
        if self.tx_table_start + TX_OFFSET_BYTE_LEN > self.tx_table_byte_len {
            return None;
        }

        // Read the offset from the tx table.
        // Offsets are 0-based; shift it to after the tx table.
        // This offset must not exceed the namespace byte length.
        let tx_payloads_end = std::cmp::min(
            tx_offset_from_bytes(
                &self.ns_payload[self.tx_table_start..self.tx_table_start + TX_OFFSET_BYTE_LEN],
            ) + self.tx_table_byte_len,
            self.ns_payload.len(),
        );

        let tx_range = self.tx_payload_start..tx_payloads_end;
        self.tx_payload_start = tx_payloads_end;
        self.tx_table_start += TX_OFFSET_BYTE_LEN;
        Some(TxIndex { tx_range })
    }
}
