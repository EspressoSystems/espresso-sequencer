use crate::{
    block2::{
        ns_table::{self, NsTable},
        payload_bytes::{usize_from_bytes, usize_to_bytes, NUM_NSS_BYTE_LEN},
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
        usize_to_bytes(self.0)
    }

    pub fn as_usize(&self, _: ns_table::A) -> usize {
        self.0
    }

    /// Return a decremented [`NsIndex`].
    pub fn prev(&self, _: ns_table::A) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
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
        <[u8; NUM_NSS_BYTE_LEN] as Deserialize>::deserialize(deserializer).map(
            |bytes: [u8; NUM_NSS_BYTE_LEN]| NsIndex(usize_from_bytes::<NUM_NSS_BYTE_LEN>(&bytes)),
        )
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
