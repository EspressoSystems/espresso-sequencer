//! Config file for stake table
use crate::utils::ToFields;
use ark_ff::PrimeField;
use ark_std::vec;
use jf_utils::to_bytes;

/// BLS verification key as indexing key
pub use jf_primitives::signatures::bls_over_bn254::VerKey as BLSVerKey;
/// Schnorr verification key as auxiliary information
pub type SchnorrVerKey = jf_primitives::signatures::schnorr::VerKey<ark_ed_on_bn254::EdwardsConfig>;
/// Type for commitment
pub type FieldType = ark_ed_on_bn254::Fq;

/// Hashable representation of a key
/// NOTE: commitment is only used in light client contract.
/// For this application, we needs only hash the Schnorr verfication key.
impl ToFields<FieldType> for SchnorrVerKey {
    const SIZE: usize = 2;

    fn to_fields(&self) -> Vec<FieldType> {
        let p = self.to_affine();
        vec![p.x, p.y]
    }
}

impl ToFields<FieldType> for BLSVerKey {
    const SIZE: usize = 2;

    fn to_fields(&self) -> Vec<FieldType> {
        let bytes = to_bytes!(&self.to_affine()).unwrap();
        vec![
            FieldType::from_le_bytes_mod_order(&bytes[..31]),
            FieldType::from_le_bytes_mod_order(&bytes[31..62]),
            FieldType::from_le_bytes_mod_order(&bytes[62..]),
        ]
    }
}
