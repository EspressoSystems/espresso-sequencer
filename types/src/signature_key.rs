//! Types and structs for the hotshot signature keys

use bitvec::{slice::BitSlice, vec::BitVec};
use ethereum_types::U256;
use generic_array::GenericArray;
use jf_primitives::{
    errors::PrimitivesError,
    signatures::{
        bls_over_bn254::{BLSOverBN254CurveSignatureScheme, KeyPair, SignKey, VerKey},
        SignatureScheme,
    },
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use tracing::instrument;

use crate::{
    qc::{BitVectorQC, QCParams},
    stake_table::StakeTableEntry,
    traits::{qc::QuorumCertificateScheme, signature_key::SignatureKey},
};

/// BLS private key used to sign a message
pub type BLSPrivKey = SignKey;
/// BLS public key used to verify a signature
pub type BLSPubKey = VerKey;
/// Public parameters for BLS signature scheme
pub type BLSPublicParam = ();

impl SignatureKey for BLSPubKey {
    type PrivateKey = BLSPrivKey;
    type StakeTableEntry = StakeTableEntry<VerKey>;
    type QCParams =
        QCParams<BLSPubKey, <BLSOverBN254CurveSignatureScheme as SignatureScheme>::PublicParameter>;
    type PureAssembledSignatureType =
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature;
    type QCType = (Self::PureAssembledSignatureType, BitVec);
    type SignError = PrimitivesError;

    #[instrument(skip(self))]
    fn validate(&self, signature: &Self::PureAssembledSignatureType, data: &[u8]) -> bool {
        // This is the validation for QC partial signature before append().
        BLSOverBN254CurveSignatureScheme::verify(&(), self, data, signature).is_ok()
    }

    fn sign(
        sk: &Self::PrivateKey,
        data: &[u8],
    ) -> Result<Self::PureAssembledSignatureType, Self::SignError> {
        BitVectorQC::<BLSOverBN254CurveSignatureScheme>::sign(
            &(),
            sk,
            data,
            &mut rand::thread_rng(),
        )
    }

    fn from_private(private_key: &Self::PrivateKey) -> Self {
        BLSPubKey::from(private_key)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];
        ark_serialize::CanonicalSerialize::serialize_compressed(self, &mut buf)
            .expect("Serialization should not fail.");
        buf
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PrimitivesError> {
        Ok(ark_serialize::CanonicalDeserialize::deserialize_compressed(
            bytes,
        )?)
    }

    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::PrivateKey) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&index.to_le_bytes());
        let new_seed = *hasher.finalize().as_bytes();
        let kp = KeyPair::generate(&mut ChaCha20Rng::from_seed(new_seed));
        (kp.ver_key(), kp.sign_key_ref().clone())
    }

    fn get_stake_table_entry(&self, stake: u64) -> Self::StakeTableEntry {
        StakeTableEntry {
            stake_key: *self,
            stake_amount: U256::from(stake),
        }
    }

    fn get_public_key(entry: &Self::StakeTableEntry) -> Self {
        entry.stake_key
    }

    fn get_public_parameter(
        stake_entries: Vec<Self::StakeTableEntry>,
        threshold: U256,
    ) -> Self::QCParams {
        QCParams {
            stake_entries,
            threshold,
            agg_sig_pp: (),
        }
    }

    fn check(real_qc_pp: &Self::QCParams, data: &[u8], qc: &Self::QCType) -> bool {
        let msg = GenericArray::from_slice(data);
        BitVectorQC::<BLSOverBN254CurveSignatureScheme>::check(real_qc_pp, msg, qc).is_ok()
    }

    fn get_sig_proof(signature: &Self::QCType) -> (Self::PureAssembledSignatureType, BitVec) {
        signature.clone()
    }

    fn assemble(
        real_qc_pp: &Self::QCParams,
        signers: &BitSlice,
        sigs: &[Self::PureAssembledSignatureType],
    ) -> Self::QCType {
        BitVectorQC::<BLSOverBN254CurveSignatureScheme>::assemble(real_qc_pp, signers, sigs)
            .expect("this assembling shouldn't fail")
    }

    fn genesis_proposer_pk() -> Self {
        let kp = KeyPair::generate(&mut ChaCha20Rng::from_seed([0u8; 32]));
        kp.ver_key()
    }
}
