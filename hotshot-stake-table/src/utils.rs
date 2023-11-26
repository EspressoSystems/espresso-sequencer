//! Utilities to help building a stake table.

use ark_ff::{Field, PrimeField};
use ethereum_types::U256;

/// A trait that converts into a field element.
pub trait ToFields<F: Field> {
    /// The number of field elements needed to represent the given struct.
    const SIZE: usize;

    /// Convert the given struct into a list of field elements.
    fn to_fields(&self) -> Vec<F>;
}

/// convert a U256 to a field element.
pub(crate) fn u256_to_field<F: PrimeField>(v: &U256) -> F {
    let mut bytes = vec![0u8; 32];
    v.to_little_endian(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}
