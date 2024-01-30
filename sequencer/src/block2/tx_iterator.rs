use std::ops::Range;

use crate::block2::payload::Payload;
use serde::{Deserialize, Serialize};

use super::{get_ns_table_entry, get_ns_table_len, get_tx_table_len};

type NsTable = <Payload<u32, u32, [u8; 32]> as hotshot::traits::BlockPayload>::Metadata;

/// TODO do we really need `PartialOrd`, `Ord` here?
/// Could the `Ord` bound be removed from `QueryablePayload::TransactionIndex`?`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TxIndex {
    pub ns_idx: usize,
    pub tx_idx: usize,
}

pub struct TxIterator<'a> {
    ns_idx: usize, // simpler than using `Peekable`
    ns_iter: Range<usize>,
    tx_iter: Range<usize>,
    block_payload: &'a Payload<u32, u32, [u8; 32]>,
    ns_table: &'a NsTable,
}

impl<'a> TxIterator<'a> {
    pub fn new(ns_table: &'a NsTable, block_payload: &'a Payload<u32, u32, [u8; 32]>) -> Self {
        Self {
            ns_idx: 0, // arbitrary value, changed in first call to next()
            ns_iter: 0..get_ns_table_len(ns_table),
            tx_iter: 0..0, // empty range
            block_payload,
            ns_table,
        }
    }
}

impl<'a> Iterator for TxIterator<'a> {
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
            let payload_len = self.block_payload.payload.len();
            for ns_idx in self.ns_iter.by_ref() {
                self.ns_idx = ns_idx;
                let start = if self.ns_idx == 0 {
                    0
                } else {
                    get_ns_table_entry(self.ns_table, self.ns_idx - 1).1
                };
                let end = get_ns_table_entry(self.ns_table, self.ns_idx).1;

                // TODO refactor range-checking code
                let end = std::cmp::min(end, payload_len);
                let start = std::cmp::min(start, end);

                let tx_table_len = get_tx_table_len(&self.block_payload.payload[start..end]);

                self.tx_iter = 0..tx_table_len;
                if let Some(tx_idx) = self.tx_iter.next() {
                    return Some(TxIndex {
                        ns_idx: self.ns_idx,
                        tx_idx,
                    });
                } else {
                    continue;
                }
            }
            None // all namespaces consumed
        }
    }
}
