//! Minimal compatibility over public key signatures
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bitvec::prelude::*;
use espresso_systems_common::hotshot::tag;
use ethereum_types::U256;
use jf_primitives::signatures::{
    bls_over_bn254::BLSOverBN254CurveSignatureScheme, SignatureScheme,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};
use tagged_base64::tagged;

/// Type saftey wrapper for byte encoded keys
#[tagged(tag::ENCODED_PUB_KEY)]
#[derive(
    Clone,
    custom_debug::Debug,
    Hash,
    CanonicalSerialize,
    CanonicalDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct EncodedPublicKey(
    #[debug(with = "custom_debug::hexbuf")] pub Vec<u8>, // pub <BLSOverBN254CurveSignatureScheme as SignatureScheme>::VerificationKey
);

/// Type saftey wrapper for byte encoded signature
#[derive(
    Clone, custom_debug::Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct EncodedSignature(
    #[debug(with = "custom_debug::hexbuf")] pub Vec<u8>, // pub <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature
);

impl AsRef<[u8]> for EncodedSignature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

/// Trait for abstracting public key signatures
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
{
    /// The private key type for this signature algorithm
    type PrivateKey: Send + Sync + Sized + Clone;
    /// The type of the entry that contain both public key and stake value
    type StakeTableEntry: Send
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

    // Signature type represented as a vec/slice of bytes to let the implementer handle the nuances
    // of serialization, to avoid Cryptographic pitfalls
    /// Validate a signature
    fn validate(&self, signature: &EncodedSignature, data: &[u8]) -> bool;
    /// Produce a signature
    fn sign(private_key: &Self::PrivateKey, data: &[u8]) -> EncodedSignature;
    /// Produce a public key from a private key
    fn from_private(private_key: &Self::PrivateKey) -> Self;
    /// Serialize a public key to bytes
    fn to_bytes(&self) -> EncodedPublicKey;
    /// Deserialize a public key from bytes
    fn from_bytes(bytes: &EncodedPublicKey) -> Option<Self>;

    /// Generate a new key pair
    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::PrivateKey);

    /// get the stake table entry from the public key and stake value
    fn get_stake_table_entry(&self, stake: u64) -> Self::StakeTableEntry;

    /// get the public parameter for the assembled signature checking
    fn get_public_parameter(
        stake_entries: Vec<Self::StakeTableEntry>,
        threshold: U256,
    ) -> Self::QCParams;

    /// check the quorum certificate for the assembled signature
    fn check(real_qc_pp: &Self::QCParams, data: &[u8], qc: &Self::QCType) -> bool;

    /// get the assembled signature and the `BitVec` separately from the assembled signature
    fn get_sig_proof(
        signature: &Self::QCType,
    ) -> (
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature,
        BitVec,
    );

    /// assemble the signature from the partial signature and the indication of signers in `BitVec`
    fn assemble(
        real_qc_pp: &Self::QCParams,
        signers: &BitSlice,
        sigs: &[<BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature],
    ) -> Self::QCType;
}
