mod iter;
mod newtypes;
mod ns_iter;
mod ns_payload;
mod ns_payload_range;
mod ns_proof;
mod ns_table;
mod payload;
mod tx_proof;
mod uint_bytes;

// TODO this eliminates dead code warnings
pub use ns_payload::NsPayload;
pub use ns_payload_range::NsPayloadRange;
pub use ns_proof::NsProof;
pub use tx_proof::TxProof2;

const NUM_TXS_BYTE_LEN: usize = 4;
const TX_OFFSET_BYTE_LEN: usize = 4;
const NUM_NSS_BYTE_LEN: usize = NUM_TXS_BYTE_LEN;
const NS_OFFSET_BYTE_LEN: usize = TX_OFFSET_BYTE_LEN;
const NS_ID_BYTE_LEN: usize = 4;

#[cfg(test)]
mod test;
