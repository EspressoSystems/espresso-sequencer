//! Config file for stake table
use crate::utils::ToFields;
use ark_ff::PrimeField;
use ark_std::vec;
use jf_utils::to_bytes;

/// Schnorr verification key as auxiliary information
pub use hotshot_types::light_client::StateVerKey;
/// BLS verification key as indexing key
pub use jf_primitives::signatures::bls_over_bn254::VerKey as QCVerKey;
/// Type for commitment
pub type FieldType = ark_ed_on_bn254::Fq;

/// Hashable representation of a key
/// NOTE: commitment is only used in light client contract.
/// For this application, we needs only hash the Schnorr verification key.
impl ToFields<FieldType> for StateVerKey {
    const SIZE: usize = 2;

    fn to_fields(&self) -> Vec<FieldType> {
        let p = self.to_affine();
        vec![p.x, p.y]
    }
}

impl ToFields<FieldType> for QCVerKey {
    const SIZE: usize = 3;

    fn to_fields(&self) -> Vec<FieldType> {
        #[allow(clippy::ignored_unit_patterns)]
        match to_bytes!(&self.to_affine()) {
            Ok(bytes) => {
                vec![
                    FieldType::from_le_bytes_mod_order(&bytes[..31]),
                    FieldType::from_le_bytes_mod_order(&bytes[31..62]),
                    FieldType::from_le_bytes_mod_order(&bytes[62..]),
                ]
            }
            Err(_) => unreachable!(),
        }
    }
}
