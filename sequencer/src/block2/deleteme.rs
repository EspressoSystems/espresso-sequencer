use super::{boilerplate::test_vid_factory, get_ns_table_entry, get_ns_table_len};
use ark_bls12_381::Bls12_381;
use jf_primitives::{
    pcs::{prelude::UnivariateKzgPCS, PolynomialCommitmentScheme},
    vid::{advz::payload_prover::LargeRangeProof, payload_prover::PayloadProver},
};
use std::ops::Range;

/// Namespace proof type
///
/// # Type complexity
///
/// Jellyfish's `LargeRangeProof` type has a prime field generic parameter `F`.
/// This `F` is determined by the pairing parameter for `Advz` currently returned by `test_vid_factory()`.
/// Jellyfish needs a more ergonomic way for downstream users to refer to this type.
///
/// There is a `KzgEval` type alias in jellyfish that helps a little, but it's currently private.
/// If it were public then we could instead use
/// ```rust
/// LargeRangeProof<KzgEval<Bls12_281>>
/// ```
/// but that's still pretty crufty.
pub type NamespaceProof =
    LargeRangeProof<<UnivariateKzgPCS<Bls12_381> as PolynomialCommitmentScheme>::Evaluation>;

impl super::BlockPayload {
    /// Returns (ns_payload, ns_proof) where ns_payload is raw bytes.
    pub fn namespace_with_proof(
        &self,
        meta: &<Self as hotshot_types::traits::BlockPayload>::Metadata,
        ns_index: usize,
    ) -> Option<(Vec<u8>, NamespaceProof)> {
        if ns_index >= get_ns_table_len(meta) {
            return None; // error: index out of bounds
        }

        let ns_payload_range = get_ns_payload_range(meta, ns_index, self.payload.len());

        let vid = test_vid_factory(); // TODO temporary VID construction

        // TODO log output for each `?`
        Some((
            self.payload.get(ns_payload_range.clone())?.to_vec(),
            vid.payload_proof(&self.payload, ns_payload_range).ok()?,
        ))
    }
}

/// Like `tx_payload_range` except for namespaces.
/// Returns the byte range for a ns in the block payload bytes.
///
/// Ensures that the returned range is valid: `start <= end <= block_payload_byte_len`.
/// TODO make fns such as this methods of a new `NsTable` struct?
pub fn get_ns_payload_range(
    ns_table_bytes: &[u8],
    ns_index: usize,
    block_payload_byte_len: usize,
) -> Range<usize> {
    let end = std::cmp::min(
        get_ns_table_entry(ns_table_bytes, ns_index).1,
        block_payload_byte_len,
    );
    let start = if ns_index == 0 {
        0
    } else {
        std::cmp::min(get_ns_table_entry(ns_table_bytes, ns_index - 1).1, end)
    };
    start..end
}
