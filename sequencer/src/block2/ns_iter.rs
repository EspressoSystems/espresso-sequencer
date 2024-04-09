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
    pub fn num_namespaces(&self) -> usize {
        // Don't double count duplicate namespace IDs. The easiest solution is
        // to consume an iterator. If performance is a concern then we could
        // cache this count on construction of `Payload`.
        self.ns_iter().count()
    }

    pub fn ns_iter(&self) -> impl Iterator<Item = NamespaceId> + '_ {
        NsIter::new(self)
    }

    pub(super) fn ns_index_iter(&self) -> impl Iterator<Item = NsIndex> + '_ {
        NsIndexIter::new(self)
    }
}

/// Return type for [`Payload::ns_iter`].
pub struct NsIter<'a>(NsIndexIter<'a>);

impl<'a> NsIter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        Self(NsIndexIter::new(block))
    }
}

impl<'a> Iterator for NsIter<'a> {
    type Item = NamespaceId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.ns_id)
    }
}

/// [`Iterator::Item`] for [`NsIterInternal`].
#[derive(Clone)]
pub(super) struct NsIndex {
    pub ns_id: NamespaceId,
    pub ns_range: Range<usize>,
}
/// Return type for [`Payload::ns_iter_internal`].
pub(super) struct NsIndexIter<'a> {
    ns_table_start: usize,   // byte index into the namespace table
    ns_payload_start: usize, // byte index into the payload
    block: &'a Payload,
    repeat_nss: HashSet<NamespaceId>,
}

impl<'a> NsIndexIter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        Self {
            ns_table_start: NUM_NSS_BYTE_LEN,
            ns_payload_start: 0,
            block,
            repeat_nss: HashSet::new(),
        }
    }
}

impl<'a> Iterator for NsIndexIter<'a> {
    type Item = NsIndex;

    fn next(&mut self) -> Option<Self::Item> {
        // this iterator is done if there's not enough room for another entry in
        // the ns_table
        // TODO we're completely ignoring the declared ns table length. :facepalm:
        while self.ns_table_start + NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN <= self.block.ns_table.len()
        {
            // read the namespace ID from the namespace table
            let ns_id = ns_id_from_bytes(
                &self.block.ns_table[self.ns_table_start..self.ns_table_start + NS_ID_BYTE_LEN],
            );

            self.ns_table_start += NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN;

            // skip duplicate namespace IDs
            if !self.repeat_nss.insert(ns_id) {
                continue;
            }

            // Read the offset from the namespace table.
            // This offset must not exceed the payload byte length.
            let ns_payload_end = std::cmp::min(
                ns_offset_from_bytes(
                    &self.block.ns_table
                        [self.ns_table_start - NS_OFFSET_BYTE_LEN..self.ns_table_start],
                ),
                self.block.payload.len(),
            );

            let ns_range = self.ns_payload_start..ns_payload_end;
            self.ns_payload_start = ns_payload_end;
            return Some(NsIndex { ns_id, ns_range });
        }
        None
    }
}
