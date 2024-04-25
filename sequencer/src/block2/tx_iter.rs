use super::{
    ns_payload::NsPayload,
    payload_bytes::{num_txs_as_bytes, NUM_TXS_BYTE_LEN},
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// TODO explain: index has same byte length as num_txs, store in serialized form
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TxIndex(pub(super) [u8; NUM_TXS_BYTE_LEN]); // TODO remove pub(super)

pub struct TxIter(Range<usize>);

impl TxIter {
    pub fn new(ns_payload: &NsPayload) -> Self {
        Self(0..ns_payload.num_txs())
    }
}

// TODO explain: boilerplate `impl Iterator` delegates to `Range`
impl Iterator for TxIter {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| TxIndex(num_txs_as_bytes(i)))
    }
}
