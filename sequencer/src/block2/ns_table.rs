use super::payload_bytes::{
    num_nss_from_bytes, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN,
};
use crate::NamespaceId;
use ns_iter::{NsIndex, NsIter};
use serde::{Deserialize, Serialize};

// TODO do these all need to be pub?
pub mod ns_iter;
pub mod ns_payload;
pub mod ns_payload_range;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsTable(pub(super) Vec<u8>); // TODO remove pub(super)

impl NsTable {
    /// The number of bytes used to encode the number of entries in the
    /// namespace table.
    ///
    /// Returns the minimum of [`NUM_NSS_BYTE_LEN`] and the byte length of the
    /// entire namespace table.
    ///
    /// In all nontrivial cases this quantity is [`NUM_NSS_BYTE_LEN`]. Anything
    /// else is a degenerate case.
    fn num_nss_byte_len(&self) -> usize {
        NUM_NSS_BYTE_LEN.min(self.0.len())
    }

    /// The number of entries in the namespace table, including all duplicate
    /// namespace IDs.
    ///
    /// Returns the minimum of:
    /// - The number of namespaces declared in the ns table
    /// - The maximum number of entries that could fit into the namespace table.
    fn num_nss_with_duplicates(&self) -> usize {
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
        num_nss_from_bytes(&self.0[..self.num_nss_byte_len()])
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
}
