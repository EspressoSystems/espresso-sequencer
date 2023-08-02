//! Config file for stake table
use ark_ff::PrimeField;
use ark_std::vec;
use ethereum_types::U256;
use jf_primitives::crhf::FixedLengthRescueCRHF;

/// Branch of merkle tree.
/// Set to 3 because we are currently using RATE-3 rescue hash function
pub(crate) const TREE_BRANCH: usize = 3;

/// Internal type of Merkle node value(commitment)
pub(crate) type FieldType = ark_bn254::Fq;
/// Hash algorithm used in Merkle tree, using a RATE-3 rescue
pub(crate) type Digest = FixedLengthRescueCRHF<FieldType, TREE_BRANCH, 1>;

/// convert a U256 to a field element.
pub(crate) fn u256_to_field<F: PrimeField>(v: &U256) -> F {
    let mut bytes = vec![0u8; 32];
    v.to_little_endian(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}
