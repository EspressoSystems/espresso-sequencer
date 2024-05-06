use crate::{
    block2::{
        payload_bytes::{
            ns_id_from_bytes, ns_offset_from_bytes, num_nss_as_bytes, num_nss_from_bytes,
            NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN,
        },
        NsTable,
    },
    NamespaceId,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;

/// Index for an entry in a ns table.
///
/// Byte length same as [`NumNss`], which doesn't exist yet.
///
/// Custom serialization and helper methods.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NsIndex(usize);

impl NsIndex {
    /// Infallible serialization.
    ///
    /// TODO same question as [`NumTxs::as_bytes`]
    pub fn as_bytes(&self) -> [u8; NUM_NSS_BYTE_LEN] {
        num_nss_as_bytes(self.0)
    }
}

// TODO so much boilerplate for serde
impl Serialize for NsIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NsIndex {
    fn deserialize<D>(deserializer: D) -> Result<NsIndex, D::Error>
    where
        D: Deserializer<'de>,
    {
        <[u8; NUM_NSS_BYTE_LEN] as Deserialize>::deserialize(deserializer)
            .map(|bytes: [u8; NUM_NSS_BYTE_LEN]| NsIndex(num_nss_from_bytes(&bytes)))
    }
}

impl NsTable {
    /// Read the namespace id from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_id(&self, index: &NsIndex) -> NamespaceId {
        let start = index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;
        ns_id_from_bytes(&self.0[start..start + NS_ID_BYTE_LEN])
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_offset(&self, index: &NsIndex) -> usize {
        // TODO refactor repeated index gymnastics code from `read_ns_id`
        let start =
            index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN + NS_ID_BYTE_LEN;
        ns_offset_from_bytes(&self.0[start..start + NS_OFFSET_BYTE_LEN])
    }

    /// Read the namespace offset from the `(index-1)`th entry from the
    /// namespace table. Returns `None` if `index` is zero.
    ///
    /// Panics if `index >= self.num_nss()`.
    pub fn read_ns_offset_prev(&self, index: &NsIndex) -> Option<usize> {
        if index.0 == 0 {
            None
        } else {
            let prev_index = NsIndex(index.0 - 1);
            Some(self.read_ns_offset(&prev_index))
        }
    }
}

/// Return type for [`Payload::ns_iter`].
pub struct NsIter<'a> {
    cur_index: usize,
    repeat_nss: HashSet<NamespaceId>,
    num_nss_with_duplicates: usize,
    ns_table: &'a NsTable,
}

impl<'a> NsIter<'a> {
    pub fn new(ns_table: &'a NsTable) -> Self {
        Self {
            cur_index: 0,
            repeat_nss: HashSet::new(),
            num_nss_with_duplicates: ns_table.num_nss_with_duplicates(),
            ns_table,
        }
    }
}

impl<'a> Iterator for NsIter<'a> {
    type Item = NsIndex;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_index < self.num_nss_with_duplicates {
            let result = NsIndex(self.cur_index);
            let ns_id = self.ns_table.read_ns_id(&result);
            self.cur_index += 1;

            // skip duplicate namespace IDs
            if !self.repeat_nss.insert(ns_id) {
                continue;
            }

            return Some(result);
        }
        None
    }
}
