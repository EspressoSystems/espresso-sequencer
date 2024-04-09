//! Config file for stake table
use ark_ff::PrimeField;
use ark_std::vec;
use jf_primitives::{crhf::FixedLengthRescueCRHF, signatures::bls_over_bn254};

use crate::utils::ToFields;

/// Branch of merkle tree.
/// Set to 3 because we are currently using RATE-3 rescue hash function
pub(crate) const TREE_BRANCH: usize = 3;

/// Internal type of Merkle node value(commitment)
pub(crate) type FieldType = ark_bn254::Fq;
/// Hash algorithm used in Merkle tree, using a RATE-3 rescue
pub(crate) type Digest = FixedLengthRescueCRHF<FieldType, TREE_BRANCH, 1>;

impl ToFields<FieldType> for FieldType {
    const SIZE: usize = 1;
    fn to_fields(&self) -> Vec<FieldType> {
        vec![*self]
    }
}

impl ToFields<FieldType> for bls_over_bn254::VerKey {
    const SIZE: usize = 2;
    fn to_fields(&self) -> Vec<FieldType> {
        #[allow(clippy::ignored_unit_patterns)]
        let bytes = jf_utils::to_bytes!(&self.to_affine()).unwrap();
        let x = <ark_bn254::Fq as PrimeField>::from_le_bytes_mod_order(&bytes[..32]);
        let y = <ark_bn254::Fq as PrimeField>::from_le_bytes_mod_order(&bytes[32..]);
        vec![x, y]
    }
}
