use std::ops::Range;

use super::{
    num_txs::NumTxs, tx_table_entries::TxTableEntries, uint_bytes::usize_from_bytes,
    NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};

// - no serde: this data is not read from payload bytes.
// - TODO restrict visibility: construction only in `NsPayloadRange`, access to
// `.0` only in `NsPayload2`
// pub struct NumTxsRange(pub Range<usize>);

pub struct NumTxsRangeRelative(pub Range<usize>);
pub struct TxTableEntriesRangeRelative(pub Range<usize>);
// pub struct TxOffsetRangeRelative(pub Range<usize>);

// - serde: this data is read from payload bytes, like `NumTxs`.
// - idea: trait `AsByteArray<const BYTE_LEN: usize>` with `to_bytes`,
//   `from_bytes` for `[u8; BYTE_LEN]` with a blanket impl for serde.
// pub struct TxOffset(pub usize);

// TODO replace array return type with `impl AsRef<[u8]>` to accommodate
// variable-size return types eg `TxTableEntries`
pub trait AsPayloadBytes {
    fn to_payload_bytes(&self) -> impl AsRef<[u8]>;
    fn from_payload_bytes(bytes: &[u8]) -> Self;
}

// TODO impl serde for any T that impls AsBytes

pub trait PayloadBytesRange {
    type Output: AsPayloadBytes;
    fn range(&self) -> Range<usize>;
}

impl AsPayloadBytes for NumTxs {
    fn to_payload_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        self.as_bytes() // TODO just impl it directly
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        Self::from_bytes2(bytes) // TODO just impl directly
    }
}

impl PayloadBytesRange for NumTxsRangeRelative {
    type Output = NumTxs;

    fn range(&self) -> Range<usize> {
        self.0.clone()
    }
}

const TEMP: usize = 2 * TX_OFFSET_BYTE_LEN;
impl AsPayloadBytes for TxTableEntries {
    fn to_payload_bytes(&self) -> [u8; TEMP] {
        todo!()
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        match bytes.len() {
            TX_OFFSET_BYTE_LEN => Self::new2(usize_from_bytes::<TX_OFFSET_BYTE_LEN>(bytes), None),
            TEMP => Self::new2(
                usize_from_bytes::<TX_OFFSET_BYTE_LEN>(&bytes[TX_OFFSET_BYTE_LEN..]),
                Some(usize_from_bytes::<TX_OFFSET_BYTE_LEN>(
                    &bytes[..TX_OFFSET_BYTE_LEN],
                )),
            ),
            len => panic!(
                "unexpected bytes len {} should be either {} or {}",
                len, TX_OFFSET_BYTE_LEN, TEMP
            ),
        }
    }
}

impl PayloadBytesRange for TxTableEntriesRangeRelative {
    type Output = TxTableEntries;

    fn range(&self) -> Range<usize> {
        self.0.clone()
    }
}

// WIP WIP

// pub struct NumTxs2(usize);
// pub struct NumTxsRange2(Range<usize>);

// impl NumTxsRange2 {
//     // TODO newtype for `ns_payload_byte_len`?
//     pub fn new(ns_payload_byte_len: usize) -> Self {
//         Self(0..NUM_TXS_BYTE_LEN.min(ns_payload_byte_len))
//     }
// }
