//! Minimal compatibility over public key signatures
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use ark_serialize::SerializationError;
use bitvec::prelude::*;
use ethereum_types::U256;
use jf_vid::VidScheme;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tagged_base64::TaggedBase64;

use super::EncodeBytes;
use crate::{utils::BuilderCommitment, vid::VidSchemeType};

/// Type representing stake table entries in a `StakeTable`
pub trait StakeTableEntryType {
    /// Get the stake value
    fn stake(&self) -> U256;
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
    type QcParams: Send + Sync + Sized + Clone + Debug + Hash;
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
        + Into<TaggedBase64>
        + for<'a> TryFrom<&'a TaggedBase64>;
    /// The type of the assembled qc: assembled signature + `BitVec`
    type QcType: Send
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
    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError>;

    /// Generate a new key pair
    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::PrivateKey);

    /// get the stake table entry from the public key and stake value
    fn stake_table_entry(&self, stake: u64) -> Self::StakeTableEntry;

    /// only get the public key from the stake table entry
    fn public_key(entry: &Self::StakeTableEntry) -> Self;

    /// get the public parameter for the assembled signature checking
    fn public_parameter(
        stake_entries: Vec<Self::StakeTableEntry>,
        threshold: U256,
    ) -> Self::QcParams;

    /// check the quorum certificate for the assembled signature
    fn check(real_qc_pp: &Self::QcParams, data: &[u8], qc: &Self::QcType) -> bool;

    /// get the assembled signature and the `BitVec` separately from the assembled signature
    fn sig_proof(signature: &Self::QcType) -> (Self::PureAssembledSignatureType, BitVec);

    /// assemble the signature from the partial signature and the indication of signers in `BitVec`
    fn assemble(
        real_qc_pp: &Self::QcParams,
        signers: &BitSlice,
        sigs: &[Self::PureAssembledSignatureType],
    ) -> Self::QcType;

    /// generates the genesis public key. Meant to be dummy/filler
    #[must_use]
    fn genesis_proposer_pk() -> Self;
}

/// Builder Signature Key trait with minimal requirements
pub trait BuilderSignatureKey:
    Send
    + Sync
    + Clone
    + Sized
    + Debug
    + Hash
    + Serialize
    + DeserializeOwned
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Display
{
    /// The type of the keys builder would use to sign its messages
    type BuilderPrivateKey: Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>
        + Hash;

    /// The type of the signature builder would use to sign its messages
    type BuilderSignature: Send
        + Sync
        + Sized
        + Clone
        + Debug
        + Eq
        + Serialize
        + for<'a> Deserialize<'a>
        + Hash;

    /// Type of error that can occur when signing data
    type SignError: std::error::Error + Send + Sync;

    /// validate the message with the builder's public key
    fn validate_builder_signature(&self, signature: &Self::BuilderSignature, data: &[u8]) -> bool;

    /// validate signature over fee information with the builder's public key
    fn validate_fee_signature<Metadata: EncodeBytes>(
        &self,
        signature: &Self::BuilderSignature,
        fee_amount: u64,
        metadata: &Metadata,
        vid_commitment: &<VidSchemeType as VidScheme>::Commit,
    ) -> bool {
        self.validate_builder_signature(
            signature,
            &aggregate_fee_data(fee_amount, metadata, vid_commitment),
        )
    }

    /// validate signature over block information with the builder's public key
    fn validate_block_info_signature(
        &self,
        signature: &Self::BuilderSignature,
        block_size: u64,
        fee_amount: u64,
        payload_commitment: &BuilderCommitment,
    ) -> bool {
        self.validate_builder_signature(
            signature,
            &aggregate_block_info_data(block_size, fee_amount, payload_commitment),
        )
    }

    /// sign the message with the builder's private key
    /// # Errors
    /// If unable to sign the data with the key
    fn sign_builder_message(
        private_key: &Self::BuilderPrivateKey,
        data: &[u8],
    ) -> Result<Self::BuilderSignature, Self::SignError>;

    /// sign fee offer for proposed payload
    /// # Errors
    /// If unable to sign the data with the key
    fn sign_fee<Metadata: EncodeBytes>(
        private_key: &Self::BuilderPrivateKey,
        fee_amount: u64,
        metadata: &Metadata,
        vid_commitment: &<VidSchemeType as VidScheme>::Commit,
    ) -> Result<Self::BuilderSignature, Self::SignError> {
        Self::sign_builder_message(
            private_key,
            &aggregate_fee_data(fee_amount, metadata, vid_commitment),
        )
    }

    /// sign information about offered block
    /// # Errors
    /// If unable to sign the data with the key
    fn sign_block_info(
        private_key: &Self::BuilderPrivateKey,
        block_size: u64,
        fee_amount: u64,
        payload_commitment: &BuilderCommitment,
    ) -> Result<Self::BuilderSignature, Self::SignError> {
        Self::sign_builder_message(
            private_key,
            &aggregate_block_info_data(block_size, fee_amount, payload_commitment),
        )
    }

    /// Generate a new key pair
    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::BuilderPrivateKey);
}

/// Aggregate all inputs used for signature over fee data
fn aggregate_fee_data<Metadata: EncodeBytes>(
    fee_amount: u64,
    metadata: &Metadata,
    vid_commitment: &<VidSchemeType as VidScheme>::Commit,
) -> Vec<u8> {
    let mut fee_info = Vec::new();
    fee_info.extend_from_slice(fee_amount.to_be_bytes().as_ref());
    fee_info.extend_from_slice(metadata.encode().as_ref());
    fee_info.extend_from_slice(vid_commitment.as_ref());
    fee_info
}

/// Aggregate all inputs used for signature over block info
fn aggregate_block_info_data(
    block_size: u64,
    fee_amount: u64,
    payload_commitment: &BuilderCommitment,
) -> Vec<u8> {
    let mut block_info = Vec::new();
    block_info.extend_from_slice(block_size.to_be_bytes().as_ref());
    block_info.extend_from_slice(fee_amount.to_be_bytes().as_ref());
    block_info.extend_from_slice(payload_commitment.as_ref());
    block_info
}
