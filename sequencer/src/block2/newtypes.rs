use crate::Transaction;

use super::{
    uint_bytes::{usize_from_bytes, usize_to_bytes},
    NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Range;

// TODO explain: T param allows for both array (fixed len) and vec/slice (var len).
pub trait AsPayloadBytes<'a, T, U = T>
where
    U: Into<T>,
{
    fn to_payload_bytes(&self) -> T {
        self.to_serde_bytes().into()
    }
    fn to_serde_bytes(&self) -> U;
    fn from_payload_bytes(bytes: &'a [u8]) -> Self;
}

macro_rules! as_payload_bytes_serde_impl {
    ($T:ty) => {
        impl Serialize for $T {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.to_serde_bytes().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $T {
            fn deserialize<D>(deserializer: D) -> Result<$T, D::Error>
            where
                D: Deserializer<'de>,
            {
                <&[u8] as Deserialize>::deserialize(deserializer)
                    .map(|bytes| <$T>::from_payload_bytes(bytes))
            }
        }
    };
}

pub trait PayloadBytesRange<'a, T, U = T>
where
    U: Into<T>,
{
    type Output: AsPayloadBytes<'a, T, U>;

    /// Range relative to this ns payload
    ///
    /// TODO newtype for return type?
    fn ns_payload_range(&self) -> Range<usize>;

    /// Range relative to the entire block payload
    ///
    /// TODO newtype for return type? ...for arg `ns_payload_offset`?
    fn block_payload_range(&self, ns_payload_offset: usize) -> Range<usize> {
        let range = self.ns_payload_range();
        range.start + ns_payload_offset..range.end + ns_payload_offset
    }
}

// WIP WIP

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumTxs2(usize);
as_payload_bytes_serde_impl!(NumTxs2);

impl NumTxs2 {
    // TODO delete me
    pub fn from_usize(n: usize) -> Self {
        Self(n)
    }
}

impl AsPayloadBytes<'_, [u8; NUM_TXS_BYTE_LEN]> for NumTxs2 {
    fn to_serde_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes::<NUM_TXS_BYTE_LEN>(self.0)
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
}

/// Number of txs in a namespace.
///
/// TODO explain: `NumTxs` but checked against `NsPayloadByteLen`
/// TODO rename NumTxs -> NumTxsUncheced, NumTxsChecked -> NumTxs?
pub struct NumTxsChecked(usize);

impl NumTxsChecked {
    /// delete me
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Returns the minimum of:
    /// - `num_txs`
    /// - The maximum number of tx table entries that could fit in the namespace
    ///   payload.
    pub fn new(num_txs: &NumTxs2, byte_len: &NsPayloadByteLen) -> Self {
        Self(std::cmp::min(
            // Number of txs declared in the tx table
            num_txs.0,
            // Max number of tx table entries that could fit in the namespace payload
            byte_len.0.saturating_sub(NUM_TXS_BYTE_LEN) / TX_OFFSET_BYTE_LEN,
        ))
    }

    pub fn in_bounds(&self, index: &TxIndex) -> bool {
        index.as_usize2() < self.0
    }
}

pub struct NsPayloadByteLen(usize);

impl NsPayloadByteLen {
    // TODO restrict visibility
    pub fn from_usize(n: usize) -> Self {
        Self(n)
    }
}

pub struct NumTxsRange2(Range<usize>);

impl NumTxsRange2 {
    pub fn new(byte_len: &NsPayloadByteLen) -> Self {
        Self(0..NUM_TXS_BYTE_LEN.min(byte_len.0))
    }
}

impl PayloadBytesRange<'_, [u8; NUM_TXS_BYTE_LEN]> for NumTxsRange2 {
    type Output = NumTxs2;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxTableEntries2 {
    cur: usize,
    prev: Option<usize>, // TODO no Option, just usize
}
as_payload_bytes_serde_impl!(TxTableEntries2);

impl TxTableEntries2 {
    const TWO_ENTRIES_BYTE_LEN: usize = 2 * TX_OFFSET_BYTE_LEN;
}

impl AsPayloadBytes<'_, Vec<u8>, [u8; Self::TWO_ENTRIES_BYTE_LEN]> for TxTableEntries2 {
    fn to_payload_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::TWO_ENTRIES_BYTE_LEN);
        if let Some(prev) = self.prev {
            bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(prev));
        }
        bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.cur));
        bytes
    }

    fn to_serde_bytes(&self) -> [u8; Self::TWO_ENTRIES_BYTE_LEN] {
        let mut bytes = [0; Self::TWO_ENTRIES_BYTE_LEN];
        bytes[..TX_OFFSET_BYTE_LEN].copy_from_slice(&usize_to_bytes::<TX_OFFSET_BYTE_LEN>(
            self.prev.unwrap_or(0),
        ));
        bytes[TX_OFFSET_BYTE_LEN..]
            .copy_from_slice(&usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.cur));
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

/// TODO cleanup. Return a byte range into a tx table for use in a transaction proof.
///
/// TODO move this method to NsPayloadRange, where it can be properly
/// translated into the payload. Ugh I can't do that because some
/// descendants depend on `NsPayload`! There's gotta be a better way to
/// control visibility. TODO newtype for the returned range to ensure it's
/// not accidentally miused?
///
/// The returned range `R` is relative to the beginning of a payload for a
/// namespace `N`. If `R` is to be used to retrieve bytes in a
/// multi-namespace payload then `R` must be translated to the beginning of
/// `N`.
///
/// `R` covers one entry in the tx table if `self` is zero, otherwise it
/// covers two entries.
///
/// It is the responsibility of the caller to ensure that `R` is used only
/// when `self` is less than the number of entries in `N`'s tx table.
///
/// This method should be `const` but that's forbidden by Rust.
///
/// # Tx table format (MOVE THIS DOC ELSEWHERE)
///
/// The bytes in this range encode tx table entries that contain the
/// (start,end) byte indices for the `tx_index`th transaction payload.
///
/// The `tx_index`th entry in the tx table encodes the byte index of the
/// *end* of this transaction's payload range. By deinition, this byte index
/// is also the *start* of the *previous* transaction's payload range. Thus,
/// the returned range includes `(tx_index - 1)`th and `tx_index`th entries
/// of the tx table.
///
/// Special case: If `tx_index` is 0 then the start index is implicitly 0,
/// so the returned range contains only one entry from the tx table: the
/// first entry of the tx table.
pub struct TxTableEntriesRange2(Range<usize>);

impl TxTableEntriesRange2 {
    pub fn new(index: &TxIndex) -> Self {
        let start = if index.as_usize2() == 0 {
            // Special case: the desired range includes only one entry from
            // the tx table: the first entry. This entry starts immediately
            // following the bytes that encode the tx table length.
            NUM_TXS_BYTE_LEN
        } else {
            // The desired range starts at the beginning of the previous tx
            // table entry.
            (index.as_usize2() - 1)
                .saturating_mul(TX_OFFSET_BYTE_LEN)
                .saturating_add(NUM_TXS_BYTE_LEN)
        };
        // The desired range ends at the end of this transaction's tx table entry
        let end = index
            .as_usize2()
            .saturating_add(1)
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN);
        Self(start..end)
    }
}

// TODO macro for impl `PayloadBytesRange`
impl PayloadBytesRange<'_, Vec<u8>, [u8; TxTableEntries2::TWO_ENTRIES_BYTE_LEN]>
    for TxTableEntriesRange2
{
    type Output = TxTableEntries2;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

pub struct TxPayload<'a>(&'a [u8]);

impl<'a> AsPayloadBytes<'a, &'a [u8]> for TxPayload<'a> {
    fn to_serde_bytes(&self) -> &'a [u8] {
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

// TODO macro for impl `PayloadBytesRange`
impl<'a> PayloadBytesRange<'a, &'a [u8]> for TxPayloadRange {
    type Output = TxPayload<'a>;

    fn ns_payload_range(&self) -> Range<usize> {
        self.0.clone()
    }
}

// TODO is this a good home for NamespacePayloadBuilder?
#[derive(Default)]
pub struct NamespacePayloadBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NamespacePayloadBuilder {
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
        let num_txs = NumTxs2::from_usize(self.tx_table_entries.len() / TX_OFFSET_BYTE_LEN);
        result.extend(num_txs.to_payload_bytes().as_ref());
        result.extend(self.tx_table_entries);
        result.extend(self.tx_bodies);
        result
    }
}

/// Index for an entry in a tx table.
///
/// Byte length same as [`NumTxs`].
///
/// Custom serialization and helper methods.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TxIndex(usize);
as_payload_bytes_serde_impl!(TxIndex);

impl TxIndex {
    /// Infallible serialization.
    ///
    /// TODO same question as [`NumTxs::as_bytes`]
    pub fn as_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes(self.0)
    }

    pub fn as_usize2(&self) -> usize {
        self.0
    }
}

impl AsPayloadBytes<'_, [u8; NUM_TXS_BYTE_LEN]> for TxIndex {
    fn to_serde_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes::<NUM_TXS_BYTE_LEN>(self.0)
    }

    fn from_payload_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
}

pub struct TxIter(Range<usize>);

impl TxIter {
    pub fn new2(num_txs: &NumTxsChecked) -> Self {
        Self(0..num_txs.as_usize())
    }
}

// TODO explain: boilerplate `impl Iterator` delegates to `Range`
impl Iterator for TxIter {
    type Item = TxIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(TxIndex)
    }
}
