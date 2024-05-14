//! [`NsPayloadRange2`] is the only module that knows anything about the binary
//! format of a namespace payload, and is the only module that is allowed to see
//! consts such as `NUM_TXS_BYTE_LEN`, `TX_OFFSET_BYTE_LEN`

use std::ops::Range;

use super::{
    newtypes::{NsPayloadByteLen, NumTxs2},
    tx_iter::TxIndex,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NsPayloadRange2(Range<usize>);

impl NsPayloadRange2 {
    pub fn new(start: usize, end: usize) -> Self {
        Self(start..end)
    }
    // TODO replace with equivalent of `PayloadBytesRange::block_payload_range`
    pub fn as_range(&self) -> Range<usize> {
        self.0.clone()
    }

    // TODO replace NsPayloadRange with 2 types: NsPayloadByteLen, NsPayloadOffset?
    /// TODO newtype for return type?
    pub fn byte_len(&self) -> NsPayloadByteLen {
        NsPayloadByteLen::from_usize(self.0.len())
    }
    /// TODO newtype for return type?
    pub fn offset(&self) -> usize {
        self.0.start
    }

    /// Does the `index`th entry exist in the tx table?
    ///
    /// TODO this method should be a method of NumTxsChecked::in_bounds(TxIndex)
    pub fn in_bounds(&self, index: &TxIndex, num_txs: &NumTxs2) -> bool {
        index.as_usize2() < num_txs.num_txs(&self.byte_len())
    }

    // TODO is `tx_offset_range_relative` needed, or can we go straight to
    // `tx_entries_range_relative`? If it is needed, do I need a
    // `tx_offset_range` method?
    // pub fn tx_offset_range_relative(&self, index: &TxIndex) -> TxOffsetRangeRelative {
    //     let start = index.as_usize2() * TX_OFFSET_BYTE_LEN + NUM_TXS_BYTE_LEN;
    //     TxOffsetRangeRelative(start..start + TX_OFFSET_BYTE_LEN)
    // }

    // private helpers
    // fn translate(&self, range: Range<usize>) -> Range<usize> {
    //     range.start + self.0.start..range.end + self.0.start
    // }
}
