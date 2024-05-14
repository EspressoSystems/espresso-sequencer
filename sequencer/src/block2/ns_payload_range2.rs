//! [`NsPayloadRange2`] is the only module that knows anything about the binary
//! format of a namespace payload, and is the only module that is allowed to see
//! consts such as `NUM_TXS_BYTE_LEN`, `TX_OFFSET_BYTE_LEN`

use std::ops::Range;

use super::{newtypes::NumTxs2, tx_iter::TxIndex, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN};

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
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
    /// TODO newtype for return type?
    pub fn offset(&self) -> usize {
        self.0.start
    }

    /// Number of txs in this namespace.
    ///
    /// Returns the minimum of:
    /// - `num_txs`
    /// - The maximum number of tx table entries that could fit in the namespace
    ///   payload.
    ///
    /// TODO delete me
    pub fn num_txs(&self, num_txs: &NumTxs2) -> usize {
        std::cmp::min(
            // Number of txs declared in the tx table
            num_txs.as_usize(),
            // Max number of tx table entries that could fit in the namespace payload
            self.byte_len().saturating_sub(NUM_TXS_BYTE_LEN) / TX_OFFSET_BYTE_LEN,
        )
    }

    /// Does the `index`th entry exist in the tx table?
    pub fn in_bounds(&self, index: &TxIndex, num_txs: &NumTxs2) -> bool {
        index.as_usize2() < self.num_txs(num_txs)
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
