use std::ops::Range;

use crate::{NameSpaceTable, TxIndex, TxIterator, TxTable};
use crate::{Payload, TableWordTraits};
use serde::{Deserialize, Serialize};

impl<'a, TableWord: TableWordTraits> TxIterator<'a, TableWord> {
    pub(super) fn new(
        ns_table: &'a NameSpaceTable<TableWord>,
        block_payload: &'a Payload<TableWord>,
    ) -> Self {
        Self {
            ns_idx: 0, // arbitrary value, changed in first call to next()
            ns_iter: 0..ns_table.len(),
            tx_iter: 0..0, // empty range
            block_payload,
            ns_table,
        }
    }
}

impl<'a, TableWord: TableWordTraits> Iterator for TxIterator<'a, TableWord> {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tx_idx) = self.tx_iter.next() {
            // we still have txs left to consume in current ns
            Some(TxIndex {
                ns_idx: self.ns_idx,
                tx_idx,
            })
        } else {
            // move to the next name space
            let payload = &self.block_payload.raw_payload;
            for ns_idx in self.ns_iter.by_ref() {
                self.ns_idx = ns_idx;
                let ns_range = self.ns_table.get_payload_range(ns_idx, payload.len()).1;
                let tx_table_len = TxTable::get_tx_table_len(&payload[ns_range]);
                self.tx_iter = 0..tx_table_len;
                if let Some(tx_idx) = self.tx_iter.next() {
                    return Some(TxIndex { ns_idx, tx_idx });
                }
            }
            None // all namespaces consumed
        }
    }
}
