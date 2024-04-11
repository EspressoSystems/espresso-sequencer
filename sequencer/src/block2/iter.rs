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

pub struct Iter<'a> {
    ns_iter: Peekable<NsIter<'a>>,
    tx_iter: TxIter<'a>,
}

impl<'a> Iter<'a> {
    pub fn new(block: &'a Payload) -> Self {
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
