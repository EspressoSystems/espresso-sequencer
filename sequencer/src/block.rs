mod full_payload;
mod namespace_payload;
mod uint_bytes;

// TODO this eliminates dead code warnings
pub use full_payload::{NsProof, NsTable, Payload};

#[cfg(test)]
mod test;
