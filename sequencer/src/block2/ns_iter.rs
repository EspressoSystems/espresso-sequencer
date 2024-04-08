use super::{
    payload_bytes::{
        ns_id_from_bytes, ns_offset_from_bytes, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN,
        NUM_NSS_BYTE_LEN,
    },
    Payload,
};
use crate::NamespaceId;
use std::{collections::HashSet, ops::Range};

impl Payload {
    pub(super) fn ns_iter_internal(&self) -> impl Iterator<Item = NsInfoInternal> + '_ {
        NsIterInternal::new(self)
    }
}

/// Return type for [`Payload::ns_iter`].
pub struct NsIter<'a>(NsIterInternal<'a>);

impl<'a> NsIter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        Self(NsIterInternal::new(block))
    }
}

impl<'a> Iterator for NsIter<'a> {
    type Item = NamespaceId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.ns_id)
    }
}

/// [`Iterator::Item`] for [`NsIterInternal`].
pub(super) struct NsInfoInternal {
    pub ns_id: NamespaceId,
    pub ns_range: Range<usize>,
}
/// Return type for [`Payload::ns_iter_internal`].
struct NsIterInternal<'a> {
    ns_table_index: usize,
    ns_payload_start: usize,
    block: &'a Payload,
    repeat_nss: HashSet<NamespaceId>,
}

impl<'a> NsIterInternal<'a> {
    fn new(block: &'a Payload) -> Self {
        Self {
            ns_table_index: NUM_NSS_BYTE_LEN,
            ns_payload_start: 0,
            block,
            repeat_nss: HashSet::new(),
        }
    }
}

impl<'a> Iterator for NsIterInternal<'a> {
    type Item = NsInfoInternal;

    fn next(&mut self) -> Option<Self::Item> {
        // this iterator is done if there's not enough room for another entry in
        // the ns_table
        while self.ns_table_index + NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN <= self.block.ns_table.len()
        {
            // read the namespace ID from the namespace table
            let ns_id = ns_id_from_bytes(
                &self.block.ns_table[self.ns_table_index..self.ns_table_index + NS_ID_BYTE_LEN],
            );

            self.ns_table_index += NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN;

            // skip duplicate namespace IDs
            if !self.repeat_nss.insert(ns_id) {
                continue;
            }

            // read the offset from the namespace table
            let ns_payload_end = ns_offset_from_bytes(
                &self.block.ns_table[self.ns_table_index - NS_OFFSET_BYTE_LEN..self.ns_table_index],
            );

            let ns_range = self.ns_payload_start..ns_payload_end;
            self.ns_payload_start = ns_payload_end;
            return Some(NsInfoInternal { ns_id, ns_range });
        }
        None
    }
}
