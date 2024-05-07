use crate::block2::{
    ns_iter::{NsIndex, NsIter},
    tx_iter::{TxIndex, TxIter},
    Payload,
};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

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
            ns_iter: NsIter::new(&block.ns_table).peekable(),
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
                .get_or_insert_with(|| TxIter::new(self.block.ns_payload(ns_index))) // ensure `tx_iter` is set
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
