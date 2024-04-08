// use serde::{Deserialize, Serialize};

use super::payload_bytes::{
    num_txs_as_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
use crate::{NamespaceId, Transaction};

// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Default)]
pub struct NamespacePayloadBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NamespacePayloadBuilder {
    /// Add a transaction's payload to this namespace
    pub fn append_tx(&mut self, tx: Transaction) {
        self.tx_bodies.extend(tx.into_payload());
        self.tx_table_entries
            .extend(tx_offset_as_bytes(self.tx_bodies.len()));
    }

    /// Serialize to bytes and consume self.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            NUM_TXS_BYTE_LEN + self.tx_table_entries.len() + self.tx_bodies.len(),
        );
        let num_txs = self.tx_table_entries.len() / TX_OFFSET_BYTE_LEN;
        result.extend(num_txs_as_bytes(num_txs));
        result.extend(self.tx_table_entries);
        result.extend(self.tx_bodies);
        result
    }
}

pub fn parse_ns_payload(ns_bytes: &[u8], ns_id: NamespaceId) -> Vec<Transaction> {
    // Impl copied from old module. I'm amazed it works.
    // TODO a proper impl requires an iterator like `NsIterInternal` for txs.
    use crate::block::tables::TxTable;

    let num_txs = TxTable::get_tx_table_len(ns_bytes);
    (0..TxTable::get_tx_table_len(ns_bytes))
        .map(|tx_idx| TxTable::get_payload_range(ns_bytes, tx_idx, num_txs))
        .map(|tx_range| Transaction::new(ns_id, ns_bytes[tx_range].to_vec()))
        .collect()
}
