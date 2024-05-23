use crate::{
    block2::{
        full_payload::payload::{self, PayloadByteLen},
        namespace_payload::NsPayloadRange,
        uint_bytes::{
            bytes_serde_impl, u64_from_bytes, u64_to_bytes, usize_from_bytes, usize_to_bytes,
        },
    },
    NamespaceId,
};
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_types::traits::EncodeBytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashSet, sync::Arc};

// TODO explain: the constants that dictate ns table data sizes
const NUM_NSS_BYTE_LEN: usize = 4;
const NS_OFFSET_BYTE_LEN: usize = 4;
const NS_ID_BYTE_LEN: usize = 4;

/// TODO explain: similar API to `NsPayload`
#[repr(transparent)]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct NsTable(#[serde(with = "base64_bytes")] Vec<u8>);

impl NsTable {
    /// TODO delete method [`NsTable::from_bytes_vec`] after `BlockPayload`
    /// trait has been changed to remove `Self::Metadata` args.
    pub fn from_bytes_vec(_: payload::A, bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes_slice(&self) -> &[u8] {
        &self.0
    }

    /// Read the namespace id from the `index`th entry from the namespace table.
    pub fn read_ns_id(&self, index: &NsIndex) -> NamespaceId {
        let start = index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN;

        // hack to deserialize `NamespaceId` from `NS_ID_BYTE_LEN` bytes
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

        index.0 < num_nss_with_duplicates
    }

    /// Read subslice range for the `index`th namespace from the namespace
    /// table.
    pub fn ns_range(&self, index: &NsIndex, payload_byte_len: &PayloadByteLen) -> NsPayloadRange {
        let end = self.read_ns_offset(index).min(payload_byte_len.as_usize());
        let start = if index.0 == 0 {
            0
        } else {
            self.read_ns_offset(&NsIndex(index.0 - 1))
        }
        .min(end);
        NsPayloadRange::new(start, end)
    }

    /// Iterator over all unique namespaces in the namespace table.
    pub fn iter(&self) -> impl Iterator<Item = <NsIter as Iterator>::Item> + '_ {
        NsIter::new(self)
    }

    // PRIVATE HELPERS START HERE

    /// Read the number of namespaces declared in the namespace table. This
    /// quantity might exceed the number of entries that could fit in the
    /// namespace table.
    ///
    /// For a correct count of the number of unique namespaces in this
    /// namespace table use `iter().count()`.
    fn read_num_nss(&self) -> usize {
        let num_nss_byte_len = NUM_NSS_BYTE_LEN.min(self.0.len());
        usize_from_bytes::<NUM_NSS_BYTE_LEN>(&self.0[..num_nss_byte_len])
    }

    /// Read the namespace offset from the `index`th entry from the namespace table.
    fn read_ns_offset(&self, index: &NsIndex) -> usize {
        let start =
            index.0 * (NS_ID_BYTE_LEN + NS_OFFSET_BYTE_LEN) + NUM_NSS_BYTE_LEN + NS_ID_BYTE_LEN;
        usize_from_bytes::<NS_OFFSET_BYTE_LEN>(&self.0[start..start + NS_OFFSET_BYTE_LEN])
    }
}

impl Committable for NsTable {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new(&Self::tag())
            .var_size_bytes(&self.0)
            .finalize()
    }

    fn tag() -> String {
        "NSTABLE".into()
    }
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
            .extend(u64_to_bytes::<NS_ID_BYTE_LEN>(u64::from(ns_id)));
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
        NsTable(bytes)
    }
}

impl EncodeBytes for NsTable {
    fn encode(&self) -> Arc<[u8]> {
        Arc::from(self.0.as_ref())
    }
}

/// Index for an entry in a ns table.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

/// Return type for [`Payload::ns_iter`].
pub struct NsIter<'a> {
    cur_index: usize,
    repeat_nss: HashSet<NamespaceId>,
    ns_table: &'a NsTable,
}

impl<'a> NsIter<'a> {
    pub fn new(ns_table: &'a NsTable) -> Self {
        Self {
            cur_index: 0,
            repeat_nss: HashSet::new(),
            ns_table,
        }
    }
}

impl<'a> Iterator for NsIter<'a> {
    type Item = NsIndex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let candidate_result = NsIndex(self.cur_index);
            if !self.ns_table.in_bounds(&candidate_result) {
                break None;
            }
            let ns_id = self.ns_table.read_ns_id(&candidate_result);
            self.cur_index += 1;

            // skip duplicate namespace IDs
            if !self.repeat_nss.insert(ns_id) {
                continue;
            }

            break Some(candidate_result);
        }
    }
}
