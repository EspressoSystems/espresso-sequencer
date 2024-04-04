//! Minimal compatibility over public key signatures
use bitvec::prelude::*;
use ethereum_types::U256;
use jf_primitives::errors::PrimitivesError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use tagged_base64::TaggedBase64;

/// Type representing stake table entries in a `StakeTable`
pub trait StakeTableEntryType {
    /// Get the stake value
    fn get_stake(&self) -> U256;
}

/// Trait for abstracting public key signatures
/// Self is the public key type
pub trait SignatureKey:
    Send
    + Sync
    + Clone
    + Sized
    + Debug
    + Hash
    + Serialize
    + for<'a> Deserialize<'a>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Display
    + for<'a> TryFrom<&'a TaggedBase64>
    + Into<TaggedBase64>
{
    /// The private key type for this signature algorithm
    type PrivateKey: Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>
        + Hash;
    /// The type of the entry that contain both public key and stake value
    type StakeTableEntry: StakeTableEntryType
        + Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Hash
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>;
    /// The type of the quorum certificate parameters used for assembled signature
    type QCParams: Send + Sync + Sized + Clone + Debug + Hash;
    /// The type of the assembled signature, without `BitVec`
    type PureAssembledSignatureType: Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Hash
        + PartialEq
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>
        + Into<TaggedBase64>;
    /// The type of the assembled qc: assembled signature + `BitVec`
    type QCType: Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Hash
        + PartialEq
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>;

    /// Type of error that can occur when signing data
    type SignError: std::error::Error + Send + Sync;

    // Signature type represented as a vec/slice of bytes to let the implementer handle the nuances
    // of serialization, to avoid Cryptographic pitfalls
    /// Validate a signature
    fn validate(&self, signature: &Self::PureAssembledSignatureType, data: &[u8]) -> bool;

    /// Produce a signature
    /// # Errors
    /// If unable to sign the data with the key
    fn sign(
        private_key: &Self::PrivateKey,
        data: &[u8],
    ) -> Result<Self::PureAssembledSignatureType, Self::SignError>;

    /// Produce a public key from a private key
    fn from_private(private_key: &Self::PrivateKey) -> Self;
    /// Serialize a public key to bytes
    fn to_bytes(&self) -> Vec<u8>;
    /// Deserialize a public key from bytes
    /// # Errors
    ///
    /// Will return `Err` if deserialization fails
    fn from_bytes(bytes: &[u8]) -> Result<Self, PrimitivesError>;

    /// Generate a new key pair
    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::PrivateKey);

    /// get the stake table entry from the public key and stake value
    fn get_stake_table_entry(&self, stake: u64) -> Self::StakeTableEntry;

    /// only get the public key from the stake table entry
    fn get_public_key(entry: &Self::StakeTableEntry) -> Self;

    /// get the public parameter for the assembled signature checking
    fn get_public_parameter(
        stake_entries: Vec<Self::StakeTableEntry>,
        threshold: U256,
    ) -> Self::QCParams;

    /// check the quorum certificate for the assembled signature
    fn check(real_qc_pp: &Self::QCParams, data: &[u8], qc: &Self::QCType) -> bool;

    /// get the assembled signature and the `BitVec` separately from the assembled signature
    fn get_sig_proof(signature: &Self::QCType) -> (Self::PureAssembledSignatureType, BitVec);

    /// assemble the signature from the partial signature and the indication of signers in `BitVec`
    fn assemble(
        real_qc_pp: &Self::QCParams,
        signers: &BitSlice,
        sigs: &[Self::PureAssembledSignatureType],
    ) -> Self::QCType;

    /// generates the genesis public key. Meant to be dummy/filler
    #[must_use]
    fn genesis_proposer_pk() -> Self;
}
