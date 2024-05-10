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

pub trait AsBytes<const BYTE_LEN: usize> {
    fn as_bytes(&self) -> [u8; BYTE_LEN];
    fn from_bytes(bytes: &[u8]) -> Self;
}

// TODO impl serde for any T that impls AsBytes

pub trait BytesReader<const BYTE_LEN: usize> {
    type Output: AsBytes<BYTE_LEN>;
    fn range(&self) -> Range<usize>;
}

impl AsBytes<NUM_TXS_BYTE_LEN> for NumTxs {
    fn as_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        self.as_bytes() // TODO just impl it directly
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_bytes2(bytes) // TODO just impl directly
    }
}

impl BytesReader<NUM_TXS_BYTE_LEN> for NumTxsRangeRelative {
    type Output = NumTxs;

    fn range(&self) -> Range<usize> {
        self.0.clone()
    }
}

const TEMP: usize = 2 * TX_OFFSET_BYTE_LEN;
impl AsBytes<TEMP> for TxTableEntries {
    fn as_bytes(&self) -> [u8; TEMP] {
        todo!()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
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

impl BytesReader<{ 2 * TX_OFFSET_BYTE_LEN }> for TxTableEntriesRangeRelative {
    type Output = TxTableEntries;

    fn range(&self) -> Range<usize> {
        self.0.clone()
    }
}
