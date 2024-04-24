use super::payload_bytes::{
    ns_id_from_bytes, ns_offset_from_bytes, num_nss_from_bytes, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN,
    NUM_NSS_BYTE_LEN,
};
use crate::NamespaceId;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Range};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsTable(pub(super) Vec<u8>); // TODO remove pub(super)

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NsIndex(u32);

impl NsTable {
    /// The number of bytes used to encode the number of entries in the
    /// namespace table.
    ///
    /// Returns the minimum of [`NUM_NSS_BYTE_LEN`] and the byte length of the
    /// entire namespace table.
    ///
    /// In all nontrivial cases this quantity is [`NUM_NSS_BYTE_LEN`]. Anything
    /// else is a degenerate case.
    pub fn num_nss_byte_len(&self) -> usize {
        NUM_NSS_BYTE_LEN.min(self.0.len())
    }

    /// The number of entries in the namespace table.
    ///
    /// Returns the minimum of:
    /// - The declared number of namespaces from the namespace table.
    /// - The maximum number of entries that could fit into the namespace table.
    pub fn num_nss(&self) -> usize {
        let num_nss_byte_len = self.num_nss_byte_len();
        std::cmp::min(
            // Read the declared number of namespaces from the namespace table
            num_nss_from_bytes(&self.0[..num_nss_byte_len]),
            // Max number of entries that could fit in the namespace table
            self.0.len().saturating_sub(num_nss_byte_len)
                / NS_ID_BYTE_LEN.saturating_add(NS_OFFSET_BYTE_LEN),
        )
    }

    /// Read the namespace id from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_id(&self, index: &NsIndex) -> NamespaceId {
        let start = (index.0 as usize) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;
        ns_id_from_bytes(&self.0[start..start + NS_ID_BYTE_LEN])
    }

    /// Search the namespace table for the ns_index belonging to `ns_id`.
    pub fn find_ns_id(&self, ns_id: &NamespaceId) -> Option<NsIndex> {
        self.iter().find(|index| self.read_ns_id(index) == *ns_id)
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_offset(&self, index: &NsIndex) -> usize {
        let start = (index.0 as usize) * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN)
            + NUM_NSS_BYTE_LEN
            + NS_ID_BYTE_LEN;
        ns_offset_from_bytes(&self.0[start..start + NS_OFFSET_BYTE_LEN])
    }

    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    ///
    /// Returned range guaranteed to satisfy `start <= end <= payload_byte_len`.
    ///
    /// TODO remove `payload_byte_len` arg and do not check `end`?
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn ns_payload_range(&self, index: &NsIndex, payload_byte_len: usize) -> Range<usize> {
        let end = self.read_ns_offset(index).min(payload_byte_len);
        let start = if index.0 == 0 {
            0
        } else {
            self.read_ns_offset(&NsIndex(index.0 - 1)).min(end)
        };
        start..end
    }

    pub fn iter(&self) -> impl Iterator<Item = <NsIter as Iterator>::Item> + '_ {
        NsIter::new(self)
    }

    pub fn num_namespaces(&self) -> usize {
        // Don't double count duplicate namespace IDs. The easiest solution is
        // to consume an iterator. If performance is a concern then we could
        // cache this count on construction of `Payload`.
        self.iter().count()
    }
}

/// Return type for [`Payload::ns_iter`].
pub struct NsIter<'a> {
    index: NsIndex,
    repeat_nss: HashSet<NamespaceId>,
    ns_table: &'a NsTable,
}

impl<'a> NsIter<'a> {
    pub fn new(ns_table: &'a NsTable) -> Self {
        Self {
            index: NsIndex(0),
            repeat_nss: HashSet::new(),
            ns_table,
        }
    }
}

impl<'a> Iterator for NsIter<'a> {
    type Item = NsIndex;

    fn next(&mut self) -> Option<Self::Item> {
        while (self.index.0 as usize) < self.ns_table.num_nss() {
            let ns_id = self.ns_table.read_ns_id(&self.index);
            let result = self.index.clone();
            self.index.0 += 1;

            // skip duplicate namespace IDs
            if !self.repeat_nss.insert(ns_id) {
                continue;
            }

            return Some(result);
        }
        None
    }
}
