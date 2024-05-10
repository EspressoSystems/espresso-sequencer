//! The only thing [`NsPayload2`] does is naively read from its payload given a
//! byte range. It doesn't know anything about the underlying binary format.
//! That's all done in `NsPayloadRange2`.

use super::newtypes::{AsBytes, BytesReader};

pub struct NsPayload2([u8]);

impl NsPayload2 {
    pub fn new(bytes: &[u8]) -> &NsPayload2 {
        // TODO boilerplate from `NsPayload`
        unsafe { &*(bytes as *const [u8] as *const NsPayload2) }
    }

    pub fn read<T, const BYTE_LEN: usize>(&self, range: T) -> T::Output
    where
        T: BytesReader<BYTE_LEN>,
    {
        <T::Output as AsBytes<BYTE_LEN>>::from_bytes(&self.0[range.range()])
    }

    // TODO write helper wrappers for `NsPayloadRange`, eg `num_txs()`?
}
