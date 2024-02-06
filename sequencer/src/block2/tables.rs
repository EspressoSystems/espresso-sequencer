use crate::block2::entry::TxTableEntry;
use crate::block2::payload::{NameSpaceTable, Payload, TableWordTraits};
use crate::{BlockBuildingSnafu, Error, VmId};
use snafu::OptionExt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Range;

pub trait Table<TableWord: TableWordTraits> {
    // Read TxTableEntry::byte_len() bytes from `table_bytes` starting at `offset`.
    // if `table_bytes` has too few bytes at this `offset` then pad with zero.
    // Parse these bytes into a `TxTableEntry` and return.
    // Returns raw bytes, no checking for large values
    fn get_table_len(&self, offset: usize) -> TxTableEntry;

    fn get_payload(&self) -> Vec<u8>;

    fn byte_len() -> usize {
        size_of::<TableWord>()
    }
}

impl<TableWord: TableWordTraits> Table<TableWord> for NameSpaceTable<TableWord> {
    // TODO (Philippe) avoid code duplication with similar function in TxTable?
    fn get_table_len(&self, offset: usize) -> TxTableEntry {
        let end = std::cmp::min(
            offset.saturating_add(TxTableEntry::byte_len()),
            self.raw_payload.len(),
        );
        let start = std::cmp::min(offset, end);
        let tx_table_len_range = start..end;
        let mut entry_bytes = [0u8; TxTableEntry::byte_len()];
        entry_bytes[..tx_table_len_range.len()]
            .copy_from_slice(&self.raw_payload[tx_table_len_range]);
        TxTableEntry::from_bytes_array(entry_bytes)
    }

    fn get_payload(&self) -> Vec<u8> {
        self.raw_payload.clone()
    }
}

impl<TableWord: TableWordTraits> NameSpaceTable<TableWord> {
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self {
            raw_payload: v,
            phantom: Default::default(),
        }
    }

    pub fn from_namespace_offsets(namespace_offsets: Vec<(VmId, usize)>) -> Result<Self, Error> {
        let mut ns_table = NameSpaceTable::from_vec(Vec::from(
            TxTableEntry::try_from(namespace_offsets.len())
                .ok()
                .context(BlockBuildingSnafu)?
                .to_bytes(),
        ));
        for (id, offset) in namespace_offsets {
            ns_table.add_new_entry_vmid(id)?;
            ns_table.add_new_entry_payload_len(offset)?;
        }
        Ok(ns_table)
    }

    // TODO see how we can  avoid cloning the whole payload
    pub fn from_bytes(b: &[u8]) -> Self {
        Self {
            raw_payload: b.to_vec(),
            phantom: Default::default(),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.raw_payload.clone()
    }

    fn add_new_entry_vmid(&mut self, id: VmId) -> Result<(), Error> {
        self.raw_payload.extend(
            TxTableEntry::try_from(id)
                .ok()
                .context(BlockBuildingSnafu)?
                .to_bytes(),
        );
        Ok(())
    }

    fn add_new_entry_payload_len(&mut self, l: usize) -> Result<(), Error> {
        self.raw_payload.extend(
            TxTableEntry::try_from(l)
                .ok()
                .context(BlockBuildingSnafu)?
                .to_bytes(),
        );
        Ok(())
    }

    // Parse the table length from the beginning of the namespace table.
    // Returned value is guaranteed to be no larger than the number of ns table entries that could possibly fit into `ns_table_bytes`.
    pub fn len(&self) -> usize {
        let left = self.get_table_len(0).try_into().unwrap_or(0);
        let right =
            (self.raw_payload.len() - TxTableEntry::byte_len()) / (2 * TxTableEntry::byte_len());
        std::cmp::min(left, right)
    }

    // returns (ns_id, payload_offset)
    // payload_offset is not checked, could be anything
    pub fn get_table_entry(&self, ns_index: usize) -> (VmId, usize) {
        // range for ns_id bytes in ns table
        // ensure `range` is within range for ns_table_bytes
        let start = std::cmp::min(
            ns_index
                .saturating_mul(2)
                .saturating_add(1)
                .saturating_mul(TxTableEntry::byte_len()),
            self.raw_payload.len(),
        );
        let end = std::cmp::min(
            start.saturating_add(TxTableEntry::byte_len()),
            self.raw_payload.len(),
        );
        let ns_id_range = start..end;

        // parse ns_id bytes from ns table
        // any failure -> VmId(0)
        let mut ns_id_bytes = [0u8; TxTableEntry::byte_len()];
        ns_id_bytes[..ns_id_range.len()].copy_from_slice(&self.raw_payload[ns_id_range]);
        let ns_id =
            VmId::try_from(TxTableEntry::from_bytes(&ns_id_bytes).unwrap_or(TxTableEntry::zero()))
                .unwrap_or(VmId(0));

        // range for ns_offset bytes in ns table
        // ensure `range` is within range for ns_table_bytes
        // TODO refactor range checking code
        let start = end;
        let end = std::cmp::min(
            start.saturating_add(TxTableEntry::byte_len()),
            self.raw_payload.len(),
        );
        let ns_offset_range = start..end;

        // parse ns_offset bytes from ns table
        // any failure -> 0 offset (?)
        // TODO refactor parsing code?
        let mut ns_offset_bytes = [0u8; TxTableEntry::byte_len()];
        ns_offset_bytes[..ns_offset_range.len()]
            .copy_from_slice(&self.raw_payload[ns_offset_range]);
        let ns_offset = usize::try_from(
            TxTableEntry::from_bytes(&ns_offset_bytes).unwrap_or(TxTableEntry::zero()),
        )
        .unwrap_or(0);

        (ns_id, ns_offset)
    }

    /// Like `tx_payload_range` except for namespaces.
    /// Returns the byte range for a ns in the block payload bytes.
    ///
    /// Ensures that the returned range is valid: `start <= end <= block_payload_byte_len`.
    pub fn get_payload_range(
        &self,
        ns_index: usize,
        block_payload_byte_len: usize,
    ) -> Range<usize> {
        let end = std::cmp::min(self.get_table_entry(ns_index).1, block_payload_byte_len);
        let start = if ns_index == 0 {
            0
        } else {
            std::cmp::min(self.get_table_entry(ns_index - 1).1, end)
        };
        start..end
    }
}

pub struct TxTable {}
impl TxTable {
    pub(crate) fn get_len(raw_payload: &[u8], offset: usize) -> TxTableEntry {
        let end = std::cmp::min(
            offset.saturating_add(TxTableEntry::byte_len()),
            raw_payload.len(),
        );
        let start = std::cmp::min(offset, end);
        let tx_table_len_range = start..end;
        let mut entry_bytes = [0u8; TxTableEntry::byte_len()];
        entry_bytes[..tx_table_len_range.len()].copy_from_slice(&raw_payload[tx_table_len_range]);
        TxTableEntry::from_bytes_array(entry_bytes)
    }
}

pub struct TxTableTest<TableWord: TableWordTraits> {
    raw_payload: Vec<u8>,
    phantom: PhantomData<TableWord>,
}

impl<TableWord: TableWordTraits> Table<TableWord> for TxTableTest<TableWord> {
    fn get_table_len(&self, offset: usize) -> TxTableEntry {
        TxTable::get_len(&self.raw_payload, offset)
    }

    fn get_payload(&self) -> Vec<u8> {
        self.raw_payload.clone()
    }
}
impl<TableWord: TableWordTraits> TxTableTest<TableWord> {
    #[cfg(test)]
    pub fn from_entries(entries: &[usize]) -> Self {
        let tx_table_byte_len = entries.len() + 1;
        let mut tx_table = Vec::with_capacity(tx_table_byte_len);
        tx_table.extend(TxTableEntry::from_usize(entries.len()).to_bytes());
        for entry in entries {
            tx_table.extend(TxTableEntry::from_usize(*entry).to_bytes());
        }

        Self {
            raw_payload: tx_table,
            phantom: Default::default(),
        }
    }
}
// TODO currently unused but contains code that might get re-used in the near future.
fn _get_tx_table_entry(
    ns_offset: usize,
    block_payload: &Payload<u32>,
    block_payload_len: usize,
    tx_index: usize,
) -> TxTableEntry {
    let start = ns_offset.saturating_add((tx_index + 1) * TxTableEntry::byte_len());

    let end = std::cmp::min(
        start.saturating_add(TxTableEntry::byte_len()),
        block_payload_len,
    );
    // todo: clamp offsets
    let tx_id_range = start..end;
    let mut tx_id_bytes = [0u8; TxTableEntry::byte_len()];
    tx_id_bytes[..tx_id_range.len()].copy_from_slice(&block_payload.raw_payload[tx_id_range]);

    TxTableEntry::from_bytes(&tx_id_bytes).unwrap_or(TxTableEntry::zero())
}
