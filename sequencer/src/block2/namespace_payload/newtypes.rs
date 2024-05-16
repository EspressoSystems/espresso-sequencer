use crate::block2::uint_bytes::{bytes_serde_impl, usize_from_bytes, usize_to_bytes};
use crate::Transaction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Range;

mod ns_payload_traits;
pub use ns_payload_traits::{FromNsPayloadBytes, NsPayloadBytesRange};

// TODO explain: the constants that dictate tx table data sizes
const NUM_TXS_BYTE_LEN: usize = 4;
const TX_OFFSET_BYTE_LEN: usize = 4;

/// Number of txs in a namespace.
///
/// Like [`NumTxsUnchecked`] but checked against a [`NsPayloadByteLen`].
pub struct NumTxs(usize);

impl NumTxs {
    /// Returns the minimum of:
    /// - `num_txs`
    /// - The maximum number of tx table entries that could fit in the namespace
    ///   payload.
    pub fn new(num_txs: &NumTxsUnchecked, byte_len: &NsPayloadByteLen) -> Self {
        Self(std::cmp::min(
            // Number of txs declared in the tx table
            num_txs.0,
            // Max number of tx table entries that could fit in the namespace payload
            byte_len.0.saturating_sub(NUM_TXS_BYTE_LEN) / TX_OFFSET_BYTE_LEN,
        ))
    }

    pub fn in_bounds(&self, index: &TxIndex) -> bool {
        index.0 < self.0
    }
}

/// Byte length of a namespace payload.
pub struct NsPayloadByteLen(usize);

impl NsPayloadByteLen {
    // TODO restrict visibility
    pub fn from_usize(n: usize) -> Self {
        Self(n)
    }
}

/// The part of a tx table that declares the number of txs in the payload.
/// "Unchecked" because this quantity might be larger than the number of tx
/// table entries that could fit into the namespace that contains it.
///
/// Use [`NumTxs`] for the actual number of txs in this namespace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumTxsUnchecked(usize);
bytes_serde_impl!(
    NumTxsUnchecked,
    to_payload_bytes,
    [u8; NUM_TXS_BYTE_LEN],
    from_payload_bytes
);

impl NumTxsUnchecked {
    pub fn to_payload_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes::<NUM_TXS_BYTE_LEN>(self.0)
    }
}

impl FromNsPayloadBytes<'_> for NumTxsUnchecked {
    fn from_payload_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
}

/// Byte range for the part of a tx table that declares the number of txs in the
/// payload.
pub struct NumTxsRange(Range<usize>);

impl NumTxsRange {
    pub fn new(byte_len: &NsPayloadByteLen) -> Self {
        Self(0..NUM_TXS_BYTE_LEN.min(byte_len.0))
    }
}

impl NsPayloadBytesRange<'_> for NumTxsRange {
    type Output = NumTxsUnchecked;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

/// Entries from a tx table in a namespace for use in a transaction proof.
///
/// Contains either one or two entries according to whether it was derived from
/// the first transaction in the namespace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxTableEntries {
    cur: usize,
    prev: Option<usize>, // TODO no Option, just usize
}

// This serde impl uses Vec. We could save space by using an array of
// length `TWO_ENTRIES_BYTE_LEN`, but then we need a way to distinguish
// `prev=Some(0)` from `prev=None`.
bytes_serde_impl!(
    TxTableEntries,
    to_payload_bytes,
    Vec<u8>,
    from_payload_bytes
);

impl TxTableEntries {
    const TWO_ENTRIES_BYTE_LEN: usize = 2 * TX_OFFSET_BYTE_LEN;

    pub fn to_payload_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::TWO_ENTRIES_BYTE_LEN);
        if let Some(prev) = self.prev {
            bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(prev));
        }
        bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.cur));
        bytes
    }
}

impl FromNsPayloadBytes<'_> for TxTableEntries {
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

/// Byte range for entries from a tx table for use in a transaction proof.
///
/// This range covers either one or two entries from a tx table according to
/// whether it was derived from the first transaction in the namespace.
pub struct TxTableEntriesRange(Range<usize>);

impl TxTableEntriesRange {
    pub fn new(index: &TxIndex) -> Self {
        let start = if index.0 == 0 {
            // Special case: the desired range includes only one entry from
            // the tx table: the first entry. This entry starts immediately
            // following the bytes that encode the tx table length.
            NUM_TXS_BYTE_LEN
        } else {
            // The desired range starts at the beginning of the previous tx
            // table entry.
            (index.0 - 1)
                .saturating_mul(TX_OFFSET_BYTE_LEN)
                .saturating_add(NUM_TXS_BYTE_LEN)
        };
        // The desired range ends at the end of this transaction's tx table entry
        let end = index
            .0
            .saturating_add(1)
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN);
        Self(start..end)
    }
}

impl NsPayloadBytesRange<'_> for TxTableEntriesRange {
    type Output = TxTableEntries;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

/// A transaction's payload data.
pub struct TxPayload<'a>(&'a [u8]);

impl<'a> TxPayload<'a> {
    pub fn to_payload_bytes(&self) -> &'a [u8] {
        self.0
    }
}

impl<'a> FromNsPayloadBytes<'a> for TxPayload<'a> {
    fn from_payload_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

/// Byte range for a transaction's payload data.
pub struct TxPayloadRange(Range<usize>);

impl TxPayloadRange {
    pub fn new(
        num_txs: &NumTxsUnchecked,
        tx_table_entries: &TxTableEntries,
        byte_len: &NsPayloadByteLen,
    ) -> Self {
        let tx_table_byte_len = num_txs
            .0
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN);
        let end = tx_table_entries
            .cur
            .saturating_add(tx_table_byte_len)
            .min(byte_len.0);
        let start = tx_table_entries
            .prev
            .unwrap_or(0)
            .saturating_add(tx_table_byte_len)
            .min(end);
        Self(start..end)
    }
}

impl<'a> NsPayloadBytesRange<'a> for TxPayloadRange {
    type Output = TxPayload<'a>;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

/// Index for an entry in a tx table.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TxIndex(usize);
bytes_serde_impl!(TxIndex, to_bytes, [u8; NUM_TXS_BYTE_LEN], from_bytes);

impl TxIndex {
    pub fn to_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes::<NUM_TXS_BYTE_LEN>(self.0)
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
}

pub struct TxIter(Range<usize>);

impl TxIter {
    pub fn new(num_txs: &NumTxs) -> Self {
        Self(0..num_txs.0)
    }
}

// Simple `impl Iterator` delegates to `Range`.
impl Iterator for TxIter {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(TxIndex)
    }
}

/// Build an individual namespace payload one transaction at a time.
///
/// Use [`Self::append_tx`] to add each transaction. Use [`Self::into_bytes`]
/// when you're done. The returned bytes include a well-formed tx table and all
/// tx payloads.
#[derive(Default)]
pub struct NsPayloadBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NsPayloadBuilder {
    /// Add a transaction's payload to this namespace
    pub fn append_tx(&mut self, tx: Transaction) {
        self.tx_bodies.extend(tx.into_payload());
        self.tx_table_entries
            .extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.tx_bodies.len()));
    }

    /// Serialize to bytes and consume self.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            NUM_TXS_BYTE_LEN + self.tx_table_entries.len() + self.tx_bodies.len(),
        );
        let num_txs = NumTxsUnchecked(self.tx_table_entries.len() / TX_OFFSET_BYTE_LEN);
        result.extend(num_txs.to_payload_bytes());
        result.extend(self.tx_table_entries);
        result.extend(self.tx_bodies);
        result
    }
}
