//! Types related to a namespace table.
//!
//! All code that needs to know the binary format of a namespace table is
//! restricted to this file.
//!
//! See [`NsTable`] for a full specification of the binary format of a namespace
//! table.
use crate::{
    block::{
        full_payload::payload::PayloadByteLen,
        namespace_payload::NsPayloadRange,
        uint_bytes::{
            bytes_serde_impl, u32_from_bytes, u32_to_bytes, usize_from_bytes, usize_to_bytes,
        },
    },
    NamespaceId,
};
use committable::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::Display;
use hotshot_types::traits::EncodeBytes;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{ops::Range, sync::Arc};
use thiserror::Error;

/// Byte lengths for the different items that could appear in a namespace table.
const NUM_NSS_BYTE_LEN: usize = 4;
const NS_OFFSET_BYTE_LEN: usize = 4;

// TODO prefer [`NS_ID_BYTE_LEN`] set to `8` because [`NamespaceId`] is a `u64`
// but we need to maintain serialization compatibility.
// https://github.com/EspressoSystems/espresso-sequencer/issues/1574
const NS_ID_BYTE_LEN: usize = 4;

/// Raw binary data for a namespace table.
///
/// Any sequence of bytes is a valid [`NsTable`].
///
/// # Binary format of a namespace table
///
/// Byte lengths for the different items that could appear in a namespace table
/// are specified in local private constants [`NUM_NSS_BYTE_LEN`],
/// [`NS_OFFSET_BYTE_LEN`], [`NS_ID_BYTE_LEN`].
///
/// ## Number of entries in the namespace table
///
/// The first [`NUM_NSS_BYTE_LEN`] bytes of the namespace table indicate the
/// number `n` of entries in the table as a little-endian unsigned integer. If
/// the entire table length is smaller than [`NUM_NSS_BYTE_LEN`] then the
/// missing bytes are zero-padded.
///
/// The bytes in the namespace table beyond the first [`NUM_NSS_BYTE_LEN`] bytes
/// encode table entries. Each entry consumes exactly [`NS_ID_BYTE_LEN`] `+`
/// [`NS_OFFSET_BYTE_LEN`] bytes.
///
/// The number `n` could be anything, including a number much larger than the
/// number of entries that could fit in the namespace table. As such, the actual
/// number of entries in the table is defined as the minimum of `n` and the
/// maximum number of whole entries that could fit in the table.
///
/// See [`Self::in_bounds`] for clarification.
///
/// ## Namespace table entry
///
/// ### Namespace ID
///
/// The first [`NS_ID_BYTE_LEN`] bytes of each table entry indicate the
/// [`NamespaceId`] for this namespace. Any table entry whose [`NamespaceId`] is
/// a duplicate of a previous entry is ignored. A correct count of the number of
/// *unique* (non-ignored) entries is given by `NsTable::iter().count()`.
///
/// ### Namespace offset
///
/// The next [`NS_OFFSET_BYTE_LEN`] bytes of each table entry indicate the
/// end-index of a namespace in the block payload bytes
/// [`Payload`](super::payload::Payload). This end-index is a little-endian
/// unsigned integer.
///
/// # How to deduce a namespace's byte range
///
/// In order to extract the payload bytes of a single namespace `N` from the
/// block payload one needs both the start- and end-indices for `N`.
///
/// See [`Self::ns_range`] for clarification. What follows is a description of
/// what's implemented in [`Self::ns_range`].
///
/// If `N` occupies the `i`th entry in the namespace table for `i>0` then the
/// start-index for `N` is defined as the end-index of the `(i-1)`th entry in
/// the table.
///
/// Even if the `(i-1)`the entry would otherwise be ignored (due to a duplicate
/// [`NamespaceId`] or any other reason), that entry's end-index still defines
/// the start-index of `N`. This rule guarantees that both start- and
/// end-indices for any namespace `N` can be read from a constant-size byte
/// range in the namespace table, and it eliminates the need to traverse an
/// unbounded number of previous entries of the namespace table looking for a
/// previous non-ignored entry.
///
/// The start-index of the 0th entry in the table is implicitly defined to be
/// `0`.
///
/// The start- and end-indices `(declared_start, declared_end)` declared in the
/// namespace table could be anything. As such, the actual start- and
/// end-indices `(start, end)` are defined so as to ensure that the byte range
/// is well-defined and in-bounds for the block payload:
/// ```ignore
/// end = min(declared_end, block_payload_byte_length)
/// start = min(declared_start, end)
/// ```
///
/// In a "honestly-prepared" namespace table the end-index of the final
/// namespace equals the byte length of the block payload. (Otherwise the block
/// payload might have bytes that are not included in any namespace.)
///
/// It is possible that a namespace table could indicate two distinct namespaces
/// whose byte ranges overlap, though no "honestly-prepared" namespace table
/// would do this.
///
/// TODO prefer [`NsTable`] to be a newtype like this
/// ```ignore
/// #[repr(transparent)]
/// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// #[serde(transparent)]
/// pub struct NsTable(#[serde(with = "base64_bytes")] Vec<u8>);
/// ```
/// but we need to maintain serialization compatibility.
/// <https://github.com/EspressoSystems/espresso-sequencer/issues/1575>
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
// Boilerplate: `#[serde(remote = "Self")]` needed to check invariants on
// deserialization. See
// https://github.com/serde-rs/serde/issues/1220#issuecomment-382589140
#[serde(remote = "Self")]
pub struct NsTable {
    #[serde(with = "base64_bytes")]
    bytes: Vec<u8>,
}

// Boilerplate: `#[serde(remote = "Self")]` allows invariant checking on
// deserialization via re-implementation of `Deserialize` in terms of default
// derivation. See
// https://github.com/serde-rs/serde/issues/1220#issuecomment-382589140
impl<'de> Deserialize<'de> for NsTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unchecked = NsTable::deserialize(deserializer)?;
        unchecked
            .validate_deserialization_invariants()
            .map_err(de::Error::custom)?;
        Ok(unchecked)
    }
}

// Boilerplate: use of `#[serde(remote = "Self")]` must include a trivial
// `Serialize` impl. See
// https://github.com/serde-rs/serde/issues/1220#issuecomment-382589140
impl Serialize for NsTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NsTable::serialize(self, serializer)
    }
}

impl NsTable {
    /// Search the namespace table for the ns_index belonging to `ns_id`.
    pub fn find_ns_id(&self, ns_id: &NamespaceId) -> Option<NsIndex> {
        self.iter()
            .find(|index| self.read_ns_id_unchecked(index) == *ns_id)
    }

    /// Number of entries in the namespace table.
    ///
    /// Defined as the maximum number of entries that could fit in the namespace
    /// table, ignoring what's declared in the table header.
    pub fn len(&self) -> NumNss {
        NumNss(
            self.bytes.len().saturating_sub(NUM_NSS_BYTE_LEN)
                / NS_ID_BYTE_LEN.saturating_add(NS_OFFSET_BYTE_LEN),
        )
    }

    /// Iterator over all unique namespaces in the namespace table.
    pub fn iter(&self) -> impl Iterator<Item = NsIndex> + '_ {
        NsIter::new(&self.len())
    }

    /// Read the namespace id from the `index`th entry from the namespace table.
    /// Returns `None` if `index` is out of bounds.
    ///
    /// TODO I want to restrict visibility to `pub(crate)` or lower but this
    /// method is currently used in `nasty-client`.
    pub fn read_ns_id(&self, index: &NsIndex) -> Option<NamespaceId> {
        if !self.in_bounds(index) {
            None
        } else {
            Some(self.read_ns_id_unchecked(index))
        }
    }

    /// Like [`Self::read_ns_id`] except `index` is not checked. Use [`Self::in_bounds`] as needed.
    pub fn read_ns_id_unchecked(&self, index: &NsIndex) -> NamespaceId {
        let start = index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;

        // TODO hack to deserialize `NamespaceId` from `NS_ID_BYTE_LEN` bytes
        // https://github.com/EspressoSystems/espresso-sequencer/issues/1574
        NamespaceId::from(u32_from_bytes::<NS_ID_BYTE_LEN>(
            &self.bytes[start..start + NS_ID_BYTE_LEN],
        ))
    }

    /// Does the `index`th entry exist in the namespace table?
    pub fn in_bounds(&self, index: &NsIndex) -> bool {
        self.len().in_bounds(index)
    }

    /// Are the bytes of this [`NsTable`] uncorrupted?
    ///
    /// # Checks
    /// 1. Byte length must hold a whole number of entries.
    /// 2. All namespace IDs and offsets must increase monotonically. Offsets
    ///    must be nonzero.
    /// 3. Header consistent with byte length (obsolete after
    ///    <https://github.com/EspressoSystems/espresso-sequencer/issues/1604>)
    /// 4. Final offset must equal `payload_byte_len` (obsolete after
    ///    <https://github.com/EspressoSystems/espresso-sequencer/issues/1604>)
    pub fn validate(
        &self,
        payload_byte_len: &PayloadByteLen,
    ) -> Result<(), NsTableValidationError> {
        use NsTableValidationError::*;

        // conditions 1-3
        self.validate_deserialization_invariants()?;

        // condition 4
        let len = self.len().0;
        if len > 0 {
            let final_ns_index = NsIndex(len - 1);
            let final_offset = self.read_ns_offset_unchecked(&final_ns_index);
            if final_offset != payload_byte_len.as_usize() {
                return Err(InvalidFinalOffset);
            }
        }

        Ok(())
    }

    // CRATE-VISIBLE HELPERS START HERE

    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    pub(in crate::block) fn ns_range(
        &self,
        index: &NsIndex,
        payload_byte_len: &PayloadByteLen,
    ) -> NsPayloadRange {
        let end = self
            .read_ns_offset_unchecked(index)
            .min(payload_byte_len.as_usize());
        let start = if index.0 == 0 {
            0
        } else {
            self.read_ns_offset_unchecked(&NsIndex(index.0 - 1))
        }
        .min(end);
        NsPayloadRange::new(start, end)
    }

    // PRIVATE HELPERS START HERE

    /// Read the number of namespaces declared in the namespace table. THIS
    /// QUANTITY IS NEVER USED. Instead use [`NsTable::len`].
    ///
    /// TODO Delete this method after
    /// <https://github.com/EspressoSystems/espresso-sequencer/issues/1604>
    fn read_num_nss(&self) -> usize {
        let num_nss_byte_len = NUM_NSS_BYTE_LEN.min(self.bytes.len());
        usize_from_bytes::<NUM_NSS_BYTE_LEN>(&self.bytes[..num_nss_byte_len])
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    fn read_ns_offset_unchecked(&self, index: &NsIndex) -> usize {
        let start =
            index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN + NS_ID_BYTE_LEN;
        usize_from_bytes::<NS_OFFSET_BYTE_LEN>(&self.bytes[start..start + NS_OFFSET_BYTE_LEN])
    }

    /// Helper for [`NsTable::validate`], used in our custom [`serde`]
    /// implementation.
    ///
    /// Checks conditions 1-3 of [`NsTable::validate`]. Those conditions can be
    /// checked by looking only at the contents of the [`NsTable`].
    fn validate_deserialization_invariants(&self) -> Result<(), NsTableValidationError> {
        use NsTableValidationError::*;

        // Byte length for a table with `x` entries must be exactly `x *
        // NsTableBuilder::entry_byte_len() +
        // NsTableBuilder::header_byte_len()`.
        //
        // Explanation for the following `if` condition:
        //
        // The above condition is equivalent to `[byte length] -
        // header_byte_len` equals 0 modulo `entry_byte_len`. In order to
        // compute `[byte length] - header_byte_len` we must first check that
        // `[byte length]` is not exceeded by `header_byte_len`
        if self.bytes.len() < NsTableBuilder::header_byte_len()
            || (self.bytes.len() - NsTableBuilder::header_byte_len())
                % NsTableBuilder::entry_byte_len()
                != 0
        {
            return Err(InvalidByteLen);
        }

        // Header must declare the correct number of namespaces
        //
        // TODO this check obsolete after
        // https://github.com/EspressoSystems/espresso-sequencer/issues/1604
        if self.len().0 != self.read_num_nss() {
            return Err(InvalidHeader);
        }

        // Namespace IDs and offsets must increase monotonically. Offsets must
        // be nonzero.
        {
            let (mut prev_ns_id, mut prev_offset) = (None, 0);
            for (ns_id, offset) in self.iter().map(|i| {
                (
                    self.read_ns_id_unchecked(&i),
                    self.read_ns_offset_unchecked(&i),
                )
            }) {
                if let Some(prev_ns_id) = prev_ns_id {
                    if ns_id <= prev_ns_id {
                        return Err(NonIncreasingEntries);
                    }
                }
                if offset <= prev_offset {
                    return Err(NonIncreasingEntries);
                }
                (prev_ns_id, prev_offset) = (Some(ns_id), offset);
            }
        }

        Ok(())
    }
}

impl EncodeBytes for NsTable {
    fn encode(&self) -> Arc<[u8]> {
        Arc::from(self.bytes.as_ref())
    }
}

impl Committable for NsTable {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new(&Self::tag())
            .var_size_bytes(&self.bytes)
            .finalize()
    }

    fn tag() -> String {
        "NSTABLE".into()
    }
}

/// Return type for [`NsTable::validate`].
#[derive(Error, Debug, Display, Eq, PartialEq)]
pub enum NsTableValidationError {
    InvalidByteLen,
    NonIncreasingEntries,
    InvalidHeader, // TODO this variant obsolete after https://github.com/EspressoSystems/espresso-sequencer/issues/1604
    InvalidFinalOffset, // TODO this variant obsolete after https://github.com/EspressoSystems/espresso-sequencer/issues/1604
}

pub struct NsTableBuilder {
    bytes: Vec<u8>,
    num_entries: usize,
}

impl NsTableBuilder {
    pub fn new() -> Self {
        // pre-allocate space for the ns table header
        Self {
            bytes: Vec::from([0; NUM_NSS_BYTE_LEN]),
            num_entries: 0,
        }
    }

    /// Add an entry to the namespace table.
    pub fn append_entry(&mut self, ns_id: NamespaceId, offset: usize) {
        // hack to serialize `NamespaceId` to `NS_ID_BYTE_LEN` bytes
        self.bytes
            .extend(u32_to_bytes::<NS_ID_BYTE_LEN>(u32::from(ns_id)));
        self.bytes
            .extend(usize_to_bytes::<NS_OFFSET_BYTE_LEN>(offset));
        self.num_entries += 1;
    }

    /// Serialize to bytes and consume self.
    pub fn into_ns_table(self) -> NsTable {
        let mut bytes = self.bytes;
        // write the number of entries to the ns table header
        bytes[..NUM_NSS_BYTE_LEN]
            .copy_from_slice(&usize_to_bytes::<NUM_NSS_BYTE_LEN>(self.num_entries));
        NsTable { bytes }
    }

    /// Byte length of a namespace table header.
    pub const fn header_byte_len() -> usize {
        NUM_NSS_BYTE_LEN
    }

    /// Byte length of a single namespace table entry.
    pub const fn entry_byte_len() -> usize {
        NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN
    }
}

/// Index for an entry in a ns table.
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub struct NsIndex(usize);
bytes_serde_impl!(NsIndex, to_bytes, [u8; NUM_NSS_BYTE_LEN], from_bytes);

impl NsIndex {
    pub fn to_bytes(&self) -> [u8; NUM_NSS_BYTE_LEN] {
        usize_to_bytes::<NUM_NSS_BYTE_LEN>(self.0)
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_NSS_BYTE_LEN>(bytes))
    }
}

/// Number of entries in a namespace table.
pub struct NumNss(usize);

impl NumNss {
    pub fn in_bounds(&self, index: &NsIndex) -> bool {
        index.0 < self.0
    }
}

/// Return type for [`Payload::ns_iter`].
pub(in crate::block) struct NsIter(Range<usize>);

impl NsIter {
    pub fn new(num_nss: &NumNss) -> Self {
        Self(0..num_nss.0)
    }
}

// Simple `impl Iterator` delegates to `Range`.
impl Iterator for NsIter {
    type Item = NsIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(NsIndex)
    }
}

#[cfg(test)]
mod test;
