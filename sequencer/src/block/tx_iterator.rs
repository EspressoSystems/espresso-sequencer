use std::ops::Range;

use crate::block::payload::{Payload, TableWordTraits};
use crate::block::tables::{NameSpaceTable, TxTable};
use serde::{Deserialize, Serialize};

/// TODO do we really need `PartialOrd`, `Ord` here?
/// Could the `Ord` bound be removed from `QueryablePayload::TransactionIndex`?`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TxIndex {
    pub ns_idx: usize,
    pub tx_idx: usize,
}

/// TODO Decompose this iterator into
/// - a tx iterator `T` over only 1 namespace
/// - a namespace-tx iterator that reuses `T` over all namespaces
pub struct TxIterator<'a, TableWord: TableWordTraits> {
    ns_idx: usize, // simpler than using `Peekable`
    ns_iter: Range<usize>,
    tx_iter: Range<usize>,
    block_payload: &'a Payload<TableWord>,
    ns_table: &'a NameSpaceTable<TableWord>,
}

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
