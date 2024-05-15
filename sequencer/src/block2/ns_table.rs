use crate::block2::{
    ns_iter::{NsIndex, NsIter},
    payload,
    uint_bytes::{u64_from_bytes, usize_from_bytes},
    NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN,
};
use crate::NamespaceId;
use serde::{Deserialize, Serialize};

use super::{payload::PayloadByteLen, NsPayloadRange};

/// TODO explain: ZST to unlock visibility in other modules. can only be
/// constructed in this module.
pub struct A(());

/// TODO explain: similar API to `NsPayload`
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsTable(Vec<u8>);

impl NsTable {
    pub fn from_bytes_vec(_: payload::A, bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    pub fn as_bytes_slice(&self) -> &[u8] {
        &self.0
    }

    /// Read the namespace id from the `index`th entry from the namespace table.
    pub fn read_ns_id(&self, index: &NsIndex) -> NamespaceId {
        let start =
            index.as_usize(A(())) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;

        // TODO hack to deserialize `NamespaceId` from `NS_ID_BYTE_LEN` bytes
        NamespaceId::from(u64_from_bytes::<NS_ID_BYTE_LEN>(
            &self.0[start..start + NS_ID_BYTE_LEN],
        ))
    }

    /// Search the namespace table for the ns_index belonging to `ns_id`.
    pub fn find_ns_id(&self, ns_id: &NamespaceId) -> Option<NsIndex> {
        self.iter().find(|index| self.read_ns_id(index) == *ns_id)
    }

    /// Does the `index`th entry exist in the namespace table?
    pub fn in_bounds(&self, index: &NsIndex) -> bool {
        // The number of entries in the namespace table, including all duplicate
        // namespace IDs.
        let num_nss_with_duplicates = std::cmp::min(
            // Number of namespaces declared in the ns table
            self.read_num_nss(),
            // Max number of entries that could fit in the namespace table
            self.0.len().saturating_sub(NUM_NSS_BYTE_LEN)
                / NS_ID_BYTE_LEN.saturating_add(NS_OFFSET_BYTE_LEN),
        );

        index.as_usize(A(())) < num_nss_with_duplicates
    }

    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    pub fn ns_payload_range(
        &self,
        index: &NsIndex,
        payload_byte_len: &PayloadByteLen,
    ) -> NsPayloadRange {
        let end = self.read_ns_offset(index).min(payload_byte_len.as_usize());
        let start = index
            .prev(A(()))
            .map(|prev| self.read_ns_offset(&prev))
            .unwrap_or(0)
            .min(end);
        NsPayloadRange::new(start, end)
    }

    // PRIVATE HELPERS START HERE

    /// Read the number of namespaces declared in the namespace table. This
    /// quantity might exceed the number of entries that could fit in the
    /// namespace table.
    ///
    /// For a correct count of the number of namespaces in this namespace table
    /// use [`self.iter().count()`].
    fn read_num_nss(&self) -> usize {
        let num_nss_byte_len = NUM_NSS_BYTE_LEN.min(self.0.len());
        usize_from_bytes::<NUM_NSS_BYTE_LEN>(&self.0[..num_nss_byte_len])
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    fn read_ns_offset(&self, index: &NsIndex) -> usize {
        let start = index.as_usize(A(())) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN)
            + NUM_NSS_BYTE_LEN
            + NS_ID_BYTE_LEN;
        usize_from_bytes::<NS_OFFSET_BYTE_LEN>(&self.0[start..start + NS_OFFSET_BYTE_LEN])
    }

    /// Iterator over all unique namespaces in the namespace table.
    fn iter(&self) -> impl Iterator<Item = <NsIter as Iterator>::Item> + '_ {
        NsIter::new(self)
    }
}

#[cfg(test)]
impl NsTable {
    pub fn iter_test(&self) -> impl Iterator<Item = <NsIter as Iterator>::Item> + '_ {
        self.iter()
    }
}
