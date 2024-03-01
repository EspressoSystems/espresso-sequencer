use crate::{BlockBuildingSnafu, Error, VmId};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use std::mem::size_of;
use std::ops::Range;

use super::table_word::TableWord;
type Word = TableWord<u32, 4>;

pub trait Table {
    // Todo: use associated to refer to words
    // e.g. **type TableLenWord: TableLenTraits;**
    fn get_table_len(&self, offset: usize) -> Word; // **Self::TableLenWord**

    fn get_payload(&self) -> Vec<u8>;

    fn byte_len() -> usize {
        size_of::<Word>()
    }
}

#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize, Default)]
#[derivative(Hash, PartialEq)]
pub struct NameSpaceTable {
    pub(super) bytes: Vec<u8>,
}
impl Table for NameSpaceTable {
    // **type TableLenWord = TableWord<u32, 4>;**

    // TODO (Philippe) avoid code duplication with similar function in TxTable?
    fn get_table_len(&self, offset: usize) -> Word {
        let end = std::cmp::min(offset.saturating_add(Word::byte_len()), self.bytes.len());
        let start = std::cmp::min(offset, end);
        let tx_table_len_range = start..end;
        let mut entry_bytes = [0u8; Word::byte_len()];
        entry_bytes[..tx_table_len_range.len()].copy_from_slice(&self.bytes[tx_table_len_range]);
        Word::from_bytes_array(entry_bytes)
    }

    fn get_payload(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn byte_len() -> usize {
        size_of::<Word>()
    }
}

impl NameSpaceTable {
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self { bytes: v }
    }

    pub fn from_namespace_offsets(namespace_offsets: Vec<(VmId, usize)>) -> Result<Self, Error> {
        let mut ns_table = NameSpaceTable::from_vec(Vec::from(
            Word::try_from(namespace_offsets.len())
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

    // TODO don't clone the entire payload
    pub fn from_bytes(b: &[u8]) -> Self {
        Self { bytes: b.to_vec() }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Find `ns_id` and return its index into this namespace table.
    ///
    /// TODO return Result or Option? Want to avoid catch-all Error type :(
    pub fn lookup(&self, ns_id: VmId) -> Option<usize> {
        // TODO don't use TxTable, need a new method
        let ns_table_len = TxTable::get_tx_table_len(&self.bytes);

        (0..ns_table_len).find(|&ns_index| ns_id == self.get_table_entry(ns_index).0)
    }

    fn add_new_entry_vmid(&mut self, id: VmId) -> Result<(), Error> {
        self.bytes.extend(
            Word::try_from(id)
                .ok()
                .context(BlockBuildingSnafu)?
                .to_bytes(),
        );
        Ok(())
    }

    fn add_new_entry_payload_len(&mut self, l: usize) -> Result<(), Error> {
        self.bytes.extend(
            Word::try_from(l)
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
        let right = (self.bytes.len() - Word::byte_len()) / (2 * Word::byte_len());
        std::cmp::min(left, right)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // returns (ns_id, ns_offset)
    // ns_offset is not checked, could be anything
    pub fn get_table_entry(&self, ns_index: usize) -> (VmId, usize) {
        // get the range for ns_id bytes in ns table
        // ensure `range` is within range for ns_table_bytes
        let start = std::cmp::min(
            ns_index
                .saturating_mul(2)
                .saturating_add(1)
                .saturating_mul(Word::byte_len()),
            self.bytes.len(),
        );
        let end = std::cmp::min(start.saturating_add(Word::byte_len()), self.bytes.len());
        let ns_id_range = start..end;

        // parse ns_id bytes from ns table
        // any failure -> VmId(0)
        let mut ns_id_bytes = [0u8; Word::byte_len()];
        ns_id_bytes[..ns_id_range.len()].copy_from_slice(&self.bytes[ns_id_range]);
        let ns_id = VmId::try_from(Word::from_bytes(&ns_id_bytes).unwrap_or(Word::zero()))
            .unwrap_or(VmId(0));

        // get the range for ns_offset bytes in ns table
        // ensure `range` is within range for ns_table_bytes
        // TODO refactor range checking code
        let start = end;
        let end = std::cmp::min(start.saturating_add(Word::byte_len()), self.bytes.len());
        let ns_offset_range = start..end;

        // parse ns_offset bytes from ns table
        // any failure -> 0 offset (?)
        // TODO refactor parsing code?
        let mut ns_offset_bytes = [0u8; Word::byte_len()];
        ns_offset_bytes[..ns_offset_range.len()].copy_from_slice(&self.bytes[ns_offset_range]);
        let ns_offset = usize::try_from(Word::from_bytes(&ns_offset_bytes).unwrap_or(Word::zero()))
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
    // Parse `TxTableEntry::byte_len()`` bytes from `raw_payload`` starting at `offset` into a `TxTableEntry`
    pub(crate) fn get_len(raw_payload: &[u8], offset: usize) -> Word {
        let end = std::cmp::min(offset.saturating_add(Word::byte_len()), raw_payload.len());
        let start = std::cmp::min(offset, end);
        let tx_table_len_range = start..end;
        let mut entry_bytes = [0u8; Word::byte_len()];
        entry_bytes[..tx_table_len_range.len()].copy_from_slice(&raw_payload[tx_table_len_range]);
        Word::from_bytes_array(entry_bytes)
    }

    // Parse the table length from the beginning of the tx table inside `ns_bytes`.
    //
    // Returned value is guaranteed to be no larger than the number of tx table entries that could possibly fit into `ns_bytes`.
    // TODO tidy this is a sloppy wrapper for get_len
    pub(crate) fn get_tx_table_len(ns_bytes: &[u8]) -> usize {
        std::cmp::min(
            Self::get_len(ns_bytes, 0).try_into().unwrap_or(0),
            (ns_bytes.len().saturating_sub(Word::byte_len())) / Word::byte_len(),
        )
    }

    pub(crate) fn tx_table_len_range(raw_payload: &[u8]) -> Range<usize> {
        0..std::cmp::min(Word::byte_len(), raw_payload.len())
    }

    pub(crate) fn tx_bodies_offset(num_txs: usize) -> usize {
        num_txs.saturating_add(1).saturating_mul(Word::byte_len())
    }

    // returns tx_offset
    // if tx_index would reach beyond ns_bytes then return 0.
    // tx_offset is not checked, could be anything
    pub(crate) fn get_table_entry(ns_bytes: &[u8], tx_index: usize) -> usize {
        // get the range for tx_offset bytes in tx table
        let tx_offset_range = {
            let start = std::cmp::min(
                tx_index.saturating_add(1).saturating_mul(Word::byte_len()),
                ns_bytes.len(),
            );
            let end = std::cmp::min(start.saturating_add(Word::byte_len()), ns_bytes.len());
            start..end
        };

        // parse tx_offset bytes from tx table
        let mut tx_offset_bytes = [0u8; Word::byte_len()];
        tx_offset_bytes[..tx_offset_range.len()].copy_from_slice(&ns_bytes[tx_offset_range]);
        usize::try_from(Word::from_bytes(&tx_offset_bytes).unwrap_or(Word::zero())).unwrap_or(0)
    }
}
#[cfg(test)]
pub(super) mod test {
    use crate::block::tables::{Table, TxTable};

    use super::Word;

    pub struct TxTableTest {
        raw_payload: Vec<u8>,
    }

    impl Table for TxTableTest {
        fn get_table_len(&self, offset: usize) -> Word {
            TxTable::get_len(&self.raw_payload, offset)
        }

        fn get_payload(&self) -> Vec<u8> {
            self.raw_payload.clone()
        }
    }
    impl TxTableTest {
        #[cfg(test)]
        pub fn from_entries(entries: &[usize]) -> Self {
            let tx_table_byte_len = entries.len() + 1;
            let mut tx_table = Vec::with_capacity(tx_table_byte_len);
            tx_table.extend(Word::from_usize(entries.len()).to_bytes());
            for entry in entries {
                tx_table.extend(Word::from_usize(*entry).to_bytes());
            }

            Self {
                raw_payload: tx_table,
            }
        }
    }
}
