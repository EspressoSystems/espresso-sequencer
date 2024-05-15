use std::ops::Range;

/// Any struct `X` whose data is read from a namespace payload impls
/// [`FromNsPayloadBytes`]. There should be an accompanying struct `XRange` that
/// impls [`NsPayloadBytesRange`]. These traits are used in [`NsPayload::read`].
pub trait FromNsPayloadBytes<'a> {
    fn from_payload_bytes(bytes: &'a [u8]) -> Self;
}

/// Companion trait for [`FromNsPayloadBytes`].
pub trait NsPayloadBytesRange<'a> {
    type Output: FromNsPayloadBytes<'a>;

    /// Range relative to this ns payload
    fn ns_payload_range(&self) -> Range<usize>;

    /// Range relative to the entire block payload
    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize> {
        let range = self.ns_payload_range();
        range.start + ns_payload_offset..range.end + ns_payload_offset
    }
}
