//! Config file for stake table
use crate::utils::ToFields;
use ark_std::vec;
use jf_primitives::signatures::bls_over_bn254::VerKey as BLSVerKey;
use jf_primitives::signatures::schnorr::VerKey as SchnorrVerKey;

/// Key type
pub type KeyType = (BLSVerKey, SchnorrVerKey<ark_ed_on_bn254::EdwardsConfig>);
/// Type for commitment
pub type FieldType = ark_ed_on_bn254::Fq;

/// Hashable representation of a key
/// NOTE: commitment is only used in light client contract.
/// For this application, we needs only hash the Schnorr verfication key.
impl ToFields<FieldType> for KeyType {
    const SIZE: usize = 2;

    fn to_fields(&self) -> Vec<FieldType> {
        let p = self.1.to_affine();
        vec![p.x, p.y]
    }
}
