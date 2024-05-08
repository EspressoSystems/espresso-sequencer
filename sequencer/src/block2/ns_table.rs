use crate::block2::{
    ns_iter::{NsIndex, NsIter},
    ns_payload_range::NsPayloadRange,
    payload,
    payload_bytes::{
        u64_from_bytes, usize_from_bytes, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN,
    },
};
use crate::NamespaceId;
use serde::{Deserialize, Serialize};

/// TODO explain: ZST to unlock visibility in other modules. can only be
/// constructed in this module.
pub struct A(());

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsTable(Vec<u8>);

impl NsTable {
    pub fn from_bytes(_: payload::A, bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Access the bytes of this [`NsTable`].
    pub fn as_byte_slice(&self) -> &[u8] {
        &self.0
    }

    /// The number of entries in the namespace table, including all duplicate
    /// namespace IDs.
    ///
    /// Returns the minimum of:
    /// - The number of namespaces declared in the ns table
    /// - The maximum number of entries that could fit into the namespace table.
    pub fn num_nss_with_duplicates(&self) -> usize {
        std::cmp::min(
            // Number of namespaces declared in the ns table
            self.read_num_nss(),
            // Max number of entries that could fit in the namespace table
            self.0.len().saturating_sub(NUM_NSS_BYTE_LEN)
                / NS_ID_BYTE_LEN.saturating_add(NS_OFFSET_BYTE_LEN),
        )
    }

    /// Read the number of namespaces declared in the namespace table.
    ///
    /// TODO newtype for return type like [`NumTxs`]?
    fn read_num_nss(&self) -> usize {
        let num_nss_byte_len = NUM_NSS_BYTE_LEN.min(self.0.len());
        usize_from_bytes::<NUM_NSS_BYTE_LEN>(&self.0[..num_nss_byte_len])
    }

    /// Search the namespace table for the ns_index belonging to `ns_id`.
    pub fn find_ns_id(&self, ns_id: &NamespaceId) -> Option<NsIndex> {
        self.iter().find(|index| self.read_ns_id(index) == *ns_id)
    }

    /// Iterator over all unique namespaces in the namespace table.
    pub fn iter(&self) -> impl Iterator<Item = <NsIter as Iterator>::Item> + '_ {
        NsIter::new(self)
    }

    /// The number of unique namespaces in the namespace table.
    pub fn num_namespaces(&self) -> usize {
        // Don't double count duplicate namespace IDs. The easiest solution is
        // to consume an iterator. If performance is a concern then we could
        // cache this count on construction of `Payload`.
        self.iter().count()
    }

    /// Read the namespace id from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_id(&self, index: &NsIndex) -> NamespaceId {
        let start =
            index.as_usize(A(())) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;

        // TODO hack to deserialize `NamespaceId` from `NS_ID_BYTE_LEN` bytes
        NamespaceId::from(u64_from_bytes::<NS_ID_BYTE_LEN>(
            &self.0[start..start + NS_ID_BYTE_LEN],
        ))
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_offset(&self, index: &NsIndex) -> usize {
        let start = index.as_usize(A(())) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN)
            + NUM_NSS_BYTE_LEN
            + NS_ID_BYTE_LEN;
        usize_from_bytes::<NS_OFFSET_BYTE_LEN>(&self.0[start..start + NS_OFFSET_BYTE_LEN])
    }

    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    ///
    /// It is the responsibility of the caller to ensure that the `index`th
    /// entry is not a duplicate of a previous entry. Otherwise the returned
    /// range will be invalid. (Can the caller even create his own `NsIndex`??)
    ///
    /// Returned range guaranteed to satisfy `start <= end <=
    /// payload_byte_len`.
    ///
    /// TODO newtype for `payload_byte_len` arg?
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn ns_payload_range(&self, index: &NsIndex, payload_byte_len: usize) -> NsPayloadRange {
        let end = self.read_ns_offset(index).min(payload_byte_len);
        let start = index
            .prev(A(()))
            .map(|prev| self.read_ns_offset(&prev))
            .unwrap_or(0)
            .min(end);
        NsPayloadRange::new(A(()), start, end)
    }
}
