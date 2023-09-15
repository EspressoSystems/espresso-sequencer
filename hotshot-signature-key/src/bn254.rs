//! Demonstration implementation of the [`SignatureKey`] trait using BN254
use hotshot_types::traits::signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey};
/// `BLSPrivKey` implementation
mod bn254_priv;
/// `BN254Pub` implementation
mod bn254_pub;

pub use self::{bn254_priv::BLSPrivKey, bn254_pub::BN254Pub};
