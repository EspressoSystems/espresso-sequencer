use super::newtypes::NsPayloadByteLen;
use std::ops::Range;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NsPayloadRange(Range<usize>);

impl NsPayloadRange {
    /// TODO restrict visibility?
    pub fn new(start: usize, end: usize) -> Self {
        Self(start..end)
    }

    /// TODO replace with equivalent of `PayloadBytesRange::block_payload_range`
    pub fn as_range(&self) -> Range<usize> {
        self.0.clone()
    }

    pub fn byte_len(&self) -> NsPayloadByteLen {
        NsPayloadByteLen::from_usize(self.0.len())
    }

    /// TODO newtype for return type?
    pub fn offset(&self) -> usize {
        self.0.start
    }
}
