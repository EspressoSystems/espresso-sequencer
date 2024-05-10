mod iter;
mod newtypes;
mod ns_iter;
mod ns_payload;
mod ns_payload2;
mod ns_payload_range;
mod ns_payload_range2;
mod ns_proof;
mod ns_table;
mod num_txs;
mod payload;
mod tx_iter;
mod tx_proof;
mod tx_table_entries;
mod uint_bytes;

// TODO this eliminates dead code warnings
pub use ns_payload2::NsPayload2;
pub use ns_payload_range2::NsPayloadRange2;
pub use ns_proof::NsProof;

const NUM_TXS_BYTE_LEN: usize = 4;
const TX_OFFSET_BYTE_LEN: usize = 4;
const NUM_NSS_BYTE_LEN: usize = NUM_TXS_BYTE_LEN;
const NS_OFFSET_BYTE_LEN: usize = TX_OFFSET_BYTE_LEN;
const NS_ID_BYTE_LEN: usize = 4;

#[cfg(test)]
mod test;
