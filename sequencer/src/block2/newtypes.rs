use std::ops::Range;

use super::{
    tx_iter::TxIndex,
    tx_table_entries::TxTableEntries,
    uint_bytes::{usize_from_bytes, usize_to_bytes},
    NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};

pub trait AsPayloadBytes<'a> {
    fn to_payload_bytes(&self) -> impl AsRef<[u8]>;
    fn from_payload_bytes(bytes: &'a [u8]) -> Self;
}

// TODO impl serde for any T that impls AsBytes

pub trait PayloadBytesRange {
    type Output<'a>: AsPayloadBytes<'a>;

    /// Range relative to this ns payload
    ///
    /// TODO newtype for return type?
    fn ns_payload_range(&self) -> Range<usize>;

    /// Range relative to the entire block payload
    ///
    /// TODO newtype for return type? ...for arg `ns_payload_offset`?
    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize>;
}

const TEMP: usize = 2 * TX_OFFSET_BYTE_LEN;
impl AsPayloadBytes<'_> for TxTableEntries {
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

// WIP WIP

pub struct NumTxs2(usize);

impl AsPayloadBytes<'_> for NumTxs2 {
    fn to_payload_bytes(&self) -> impl AsRef<[u8]> {
        usize_to_bytes::<NUM_TXS_BYTE_LEN>(self.0)
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
}

pub struct NumTxsRange2(Range<usize>);

impl NumTxsRange2 {
    // TODO newtype for `ns_payload_byte_len`?
    pub fn new(ns_payload_byte_len: usize) -> Self {
        Self(0..NUM_TXS_BYTE_LEN.min(ns_payload_byte_len))
    }
}

impl PayloadBytesRange for NumTxsRange2 {
    type Output<'a> = NumTxs2;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }

    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize> {
        self.0.start + ns_payload_offset..self.0.end + ns_payload_offset
    }
}

pub struct TxTableEntries2 {
    cur: usize,
    prev: Option<usize>,
}

impl TxTableEntries2 {
    const TWO_ENTRIES_BYTE_LEN: usize = 2 * TX_OFFSET_BYTE_LEN;
}

impl AsPayloadBytes<'_> for TxTableEntries2 {
    fn to_payload_bytes(&self) -> impl AsRef<[u8]> {
        let mut bytes = Vec::with_capacity(Self::TWO_ENTRIES_BYTE_LEN);
        if let Some(prev) = self.prev {
            bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(prev));
        }
        bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.cur));
        bytes
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        match bytes.len() {
            TX_OFFSET_BYTE_LEN => Self {
                cur: usize_from_bytes::<TX_OFFSET_BYTE_LEN>(bytes),
                prev: None,
            },
            Self::TWO_ENTRIES_BYTE_LEN => Self {
                cur: usize_from_bytes::<TX_OFFSET_BYTE_LEN>(&bytes[TX_OFFSET_BYTE_LEN..]),
                prev: Some(usize_from_bytes::<TX_OFFSET_BYTE_LEN>(
                    &bytes[..TX_OFFSET_BYTE_LEN],
                )),
            },
            len => panic!(
                "unexpected bytes len {} should be either {} or {}",
                len,
                TX_OFFSET_BYTE_LEN,
                Self::TWO_ENTRIES_BYTE_LEN
            ),
        }
    }
}

pub struct TxTableEntriesRange2(Range<usize>);

impl TxTableEntriesRange2 {
    pub fn new(index: &TxIndex) -> Self {
        // TODO impl directly here
        Self(index.tx_table_entries_range_relative())
    }
}

// TODO macro for impl `PayloadBytesRange`
impl PayloadBytesRange for TxTableEntriesRange2 {
    type Output<'a> = TxTableEntries2;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }

    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize> {
        self.0.start + ns_payload_offset..self.0.end + ns_payload_offset
    }
}

pub struct TxPayload<'a>(&'a [u8]);

impl<'a> AsPayloadBytes<'a> for TxPayload<'a> {
    fn to_payload_bytes(&self) -> impl AsRef<[u8]> {
        self.0
    }

    fn from_payload_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

pub struct TxPayloadRange(Range<usize>);

impl TxPayloadRange {
    // TODO instead of `new` for each of these `XRange` types: have a
    // NsPayloadByteLen newtype with a method to construct each `XRange` type.
    // Why? Each of these `XRange` types requires the ns payload byte len
    // anyway.
    pub fn new(
        num_txs: &NumTxs2,
        tx_table_entries: &TxTableEntries2,
        ns_payload_byte_len: usize,
    ) -> Self {
        let tx_table_byte_len = num_txs
            .0
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN);
        let end = tx_table_entries
            .cur
            .saturating_add(tx_table_byte_len)
            .min(ns_payload_byte_len);
        let start = tx_table_entries
            .prev
            .unwrap_or(0)
            .saturating_add(tx_table_byte_len)
            .min(end);
        Self(start..end)
    }
}

// TODO macro for impl `PayloadBytesRange`
impl PayloadBytesRange for TxPayloadRange {
    type Output<'a> = TxPayload<'a>;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }

    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize> {
        self.0.start + ns_payload_offset..self.0.end + ns_payload_offset
    }
}
