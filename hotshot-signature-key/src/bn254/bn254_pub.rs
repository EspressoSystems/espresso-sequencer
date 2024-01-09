use super::{BLSPrivKey, SignatureKey};
use bitvec::prelude::*;
use blake3::traits::digest::generic_array::GenericArray;
use ethereum_types::U256;
use hotshot_qc::bit_vector_old::{
    BitVectorQC, QCParams as JFQCParams, StakeTableEntry as JFStakeTableEntry,
};
use hotshot_types::traits::qc::QuorumCertificate;
use jf_primitives::errors::PrimitivesError;
use jf_primitives::signatures::{
    bls_over_bn254::{BLSOverBN254CurveSignatureScheme, VerKey},
    SignatureScheme,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Debug};
use tracing::{instrument, warn};
use typenum::U32;

/// Public key type for an bn254 [`SignatureKey`] pair
///
/// This type makes use of noise for non-deterministic signatures.
#[derive(Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, Debug)]

pub struct BLSPubKey {
    /// The public key for this keypair
    pub_key: VerKey,
}

impl PartialOrd for BLSPubKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BLSPubKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_bytes = &self.pub_key.to_string();
        let other_bytes = &other.pub_key.to_string();
        self_bytes.cmp(other_bytes)
    }
}

impl SignatureKey for BLSPubKey {
    type PrivateKey = BLSPrivKey;
    type StakeTableEntry = JFStakeTableEntry<VerKey>;
    type QCParams = JFQCParams<
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::VerificationKey,
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::PublicParameter,
    >;
    type PureAssembledSignatureType =
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature;
    type QCType = (Self::PureAssembledSignatureType, BitVec);
    type SignError = PrimitivesError;

    #[instrument(skip(self))]
    fn validate(&self, signature: &Self::PureAssembledSignatureType, data: &[u8]) -> bool {
        let ver_key = self.pub_key;

        // This is the validation for QC partial signature before append().
        let generic_msg: &GenericArray<u8, U32> = GenericArray::from_slice(data);
        BLSOverBN254CurveSignatureScheme::verify(&(), &ver_key, generic_msg, signature).is_ok()
    }

    fn sign(
        sk: &Self::PrivateKey,
        data: &[u8],
    ) -> Result<Self::PureAssembledSignatureType, Self::SignError> {
        let generic_msg = GenericArray::from_slice(data);
        BitVectorQC::<BLSOverBN254CurveSignatureScheme>::sign(
            &(),
            generic_msg,
            &sk.priv_key,
            &mut rand::thread_rng(),
        )
    }

    fn from_private(private_key: &Self::PrivateKey) -> Self {
        let pub_key = VerKey::from(&private_key.priv_key);
        Self { pub_key }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];
        ark_serialize::CanonicalSerialize::serialize_compressed(&self.pub_key, &mut buf)
            .expect("Serialization should not fail.");
        buf
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PrimitivesError> {
        let pub_key: VerKey = ark_serialize::CanonicalDeserialize::deserialize_compressed(bytes)?;
        Ok(Self { pub_key })
    }

    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::PrivateKey) {
        let priv_key = Self::PrivateKey::generated_from_seed_indexed(seed, index);
        (Self::from_private(&priv_key), priv_key)
    }

    fn get_stake_table_entry(&self, stake: u64) -> Self::StakeTableEntry {
        JFStakeTableEntry {
            stake_key: self.pub_key,
            stake_amount: U256::from(stake),
        }
    }

    fn get_public_key(entry: &Self::StakeTableEntry) -> Self {
        Self {
            pub_key: entry.stake_key,
        }
    }

    fn get_public_parameter(
        stake_entries: Vec<Self::StakeTableEntry>,
        threshold: U256,
    ) -> Self::QCParams {
        JFQCParams {
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
        use jf_primitives::signatures::bls_over_bn254::KeyPair;
        use rand::rngs::mock::StepRng;
        let mut my_rng = StepRng::new(42, 1337);
        let kp = KeyPair::generate(&mut my_rng);
        BLSPubKey {
            pub_key: kp.ver_key(),
        }
    }
}
