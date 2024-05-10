use std::ops::Range;

// - no serde: this data is not read from payload bytes.
// - TODO restrict visibility: construction only in `NsPayloadRange`, access to
// `.0` only in `NsPayload2`
pub struct NumTxsRange(pub Range<usize>);

pub struct NumTxsRangeRelative(pub Range<usize>);

pub struct TxOffsetRangeRelative(pub Range<usize>);

// - serde: this data is read from payload bytes, like `NumTxs`.
// - idea: trait `AsByteArray<const BYTE_LEN: usize>` with `to_bytes`,
//   `from_bytes` for `[u8; BYTE_LEN]` with a blanket impl for serde.
pub struct TxOffset(pub usize);
