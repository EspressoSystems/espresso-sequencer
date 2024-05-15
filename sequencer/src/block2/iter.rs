use crate::block2::{
    ns_iter::{NsIndex, NsIter},
    payload::Payload,
};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

use super::newtypes::{NumTxsChecked, NumTxsRange2, TxIndex, TxIter};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Index {
    ns_index: NsIndex,
    tx_index: TxIndex,
}

impl Index {
    pub fn ns(&self) -> &NsIndex {
        &self.ns_index
    }
    pub fn tx(&self) -> &TxIndex {
        &self.tx_index
    }
}

// TODO don't impl `PartialOrd`
impl PartialOrd for Index {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(_other))
    }
}
// TODO don't impl `Ord`
impl Ord for Index {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unimplemented!()
    }
}

/// Cartesian product of [`NsIter`], [`TxIter`].
pub struct Iter<'a> {
    ns_iter: Peekable<NsIter<'a>>,
    tx_iter: Option<TxIter>,
    block: &'a Payload,
}

impl<'a> Iter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        Self {
            ns_iter: NsIter::new(block.ns_table()).peekable(),
            tx_iter: None,
            block,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(ns_index) = self.ns_iter.peek() else {
                return None; // ns_iter consumed
            };

            if let Some(tx_index) = self
                .tx_iter
                .get_or_insert_with(|| {
                    // TODO newtype for full block payload byte len
                    // TODO desperate need for helpers
                    let ns_payload_range = self
                        .block
                        .ns_table()
                        .ns_payload_range2(ns_index, self.block.as_byte_slice().len());
                    let ns_payload = self.block.read_ns_payload(&ns_payload_range);
                    let byte_len = ns_payload.byte_len();
                    let num_txs_range = NumTxsRange2::new(&byte_len);
                    let num_txs = ns_payload.read(&num_txs_range);
                    let num_txs_checked = NumTxsChecked::new(&num_txs, &byte_len);
                    TxIter::new2(&num_txs_checked)
                }) // ensure `tx_iter` is set
                .next()
            {
                return Some(Index {
                    ns_index: ns_index.clone(),
                    tx_index,
                });
            }

            self.tx_iter = None; // unset `tx_iter`; it's consumed for this namespace
            self.ns_iter.next();
        }
    }
}
