use std::ops::Range;

// no serde
// TODO restrict visibility of `.0` to `NsPayload2`
pub struct NumTxsRange(pub Range<usize>);

pub struct NumTxsRangeRelative(pub Range<usize>);
