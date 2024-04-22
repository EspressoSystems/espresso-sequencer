use super::{
    ns_iter::{NsIndex, NsIter},
    tx_iter::{TxIndex, TxIter},
    Payload,
};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Index {
    pub(super) ns_index: NsIndex,
    pub(super) tx_index: TxIndex,
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
    tx_iter: TxIter<'a>,
    block: &'a Payload, // TODO is there a good way to reuse ns_iter.block?
}

impl<'a> Iter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        let mut ns_iter = NsIter::new(block).peekable();

        // TODO sucks that I need to:
        // - call ns_iter.peek() in this constructor
        // - call TxIter::new here *and* in next()
        let tx_iter = TxIter::new(
            ns_iter
                .peek()
                .map_or(&[], |ns| &block.payload[ns.ns_range.clone()]),
        );
        Self {
            ns_iter,
            tx_iter,
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
            if let Some(tx_index) = self.tx_iter.next() {
                return Some(Index {
                    ns_index: ns_index.clone(),
                    tx_index,
                });
            }
            // tx_iter consumed for this namespace
            self.ns_iter.next();
            self.tx_iter = TxIter::new(&self.block.payload[self.ns_iter.peek()?.ns_range.clone()]);
        }
    }
}
