use super::{
    ns_iter::{NsIndex, NsIter},
    tx_iter::{TxIndex, TxIter},
    Payload,
};
use std::iter::Peekable;

struct Index {
    ns_index: NsIndex,
    tx_index: TxIndex,
}

struct Iter<'a> {
    ns_iter: Peekable<NsIter<'a>>,
    tx_iter: TxIter<'a>,
}

impl<'a> Iter<'a> {
    fn new(block: &'a Payload) -> Self {
        Self {
            ns_iter: NsIter::new(block).peekable(),
            tx_iter: TxIter::new(&[]),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ns_index) = self.ns_iter.peek() {
                if let Some(tx_index) = self.tx_iter.next() {
                    return Some(Index {
                        ns_index: ns_index.clone(),
                        tx_index,
                    });
                }
                self.ns_iter.next(); // tx_iter consumed for this namespace
            } else {
                return None; // ns_iter consumed
            }
        }
    }
}
