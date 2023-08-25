use super::{BN254Priv, EncodedPublicKey, EncodedSignature, SignatureKey};
use bincode::Options;
use bitvec::prelude::*;
use blake3::traits::digest::generic_array::GenericArray;
use ethereum_types::U256;
use hotshot_qc::bit_vector_old::{
    BitVectorQC, QCParams as JFQCParams, StakeTableEntry as JFStakeTableEntry,
};
use hotshot_types::traits::qc::QuorumCertificate;
use hotshot_utils::bincode::bincode_opts;
use jf_primitives::signatures::{
    bls_over_bn254::{BLSOverBN254CurveSignatureScheme, VerKey},
    SignatureScheme,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Debug};
use tracing::{debug, instrument, warn};
use typenum::U32;

/// Public key type for an bn254 [`SignatureKey`] pair
///
/// This type makes use of noise for non-determinisitc signatures.
#[derive(Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, Debug)]

pub struct BN254Pub {
    /// The public key for this keypair
    pub_key: VerKey,
}

impl PartialOrd for BN254Pub {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_bytes = &self.pub_key.to_string();
        let other_bytes = &other.pub_key.to_string();
        self_bytes.partial_cmp(other_bytes)
    }
}

impl Ord for BN254Pub {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_bytes = &self.pub_key.to_string();
        let other_bytes = &other.pub_key.to_string();
        self_bytes.cmp(other_bytes)
    }
}

impl SignatureKey for BN254Pub {
    type PrivateKey = BN254Priv;
    type StakeTableEntry = JFStakeTableEntry<VerKey>;
    type QCParams = JFQCParams<
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::VerificationKey,
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::PublicParameter,
    >;
    type QCType = (
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature,
        BitVec,
    );
    // <BitVectorQC<BLSOverBN254CurveSignatureScheme> as AssembledQuorumCertificate<BLSOverBN254CurveSignatureScheme>>::QC;

    #[instrument(skip(self))]
    fn validate(&self, signature: &EncodedSignature, data: &[u8]) -> bool {
        let ver_key = self.pub_key;
        let x: Result<<BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature, _> =
            bincode_opts().deserialize(&signature.0);
        match x {
            Ok(s) => {
                // This is the validation for QC partial signature before append().
                let generic_msg: &GenericArray<u8, U32> = GenericArray::from_slice(data);
                BLSOverBN254CurveSignatureScheme::verify(&(), &ver_key, generic_msg, &s).is_ok()
            }
            Err(_) => false,
        }
    }

    fn sign(sk: &Self::PrivateKey, data: &[u8]) -> EncodedSignature {
        let generic_msg = GenericArray::from_slice(data);
        let agg_signature_wrap = BitVectorQC::<BLSOverBN254CurveSignatureScheme>::sign(
            &(),
            generic_msg,
            &sk.priv_key,
            &mut rand::thread_rng(),
        );
        match agg_signature_wrap {
            Ok(agg_signature) => {
                // Convert the signature to bytes and return
                let bytes = bincode_opts().serialize(&agg_signature);
                match bytes {
                    Ok(bytes) => EncodedSignature(bytes),
                    Err(e) => {
                        warn!(?e, "Failed to serialize signature in sign()");
                        EncodedSignature(vec![])
                    }
                }
            }
            Err(e) => {
                warn!(?e, "Failed to sign");
                EncodedSignature(vec![])
            }
        }
    }

    fn from_private(private_key: &Self::PrivateKey) -> Self {
        let pub_key = VerKey::from(&private_key.priv_key);
        Self { pub_key }
    }

    fn to_bytes(&self) -> EncodedPublicKey {
        let pub_key_bytes = bincode_opts()
            .serialize(&self.pub_key)
            .expect("This serialization shouldn't be able to fail");
        EncodedPublicKey(pub_key_bytes)
    }

    #[instrument]
    fn from_bytes(bytes: &EncodedPublicKey) -> Option<Self> {
        let x: Result<VerKey, _> = bincode_opts().deserialize(&bytes.0);
        match x {
            Ok(pub_key) => Some(BN254Pub { pub_key }),
            Err(e) => {
                debug!(?e, "Failed to deserialize public key");
                None
            }
        }
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

    fn get_sig_proof(
        signature: &Self::QCType,
    ) -> (
        <BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature,
        BitVec,
    ) {
        signature.clone()
    }

    fn assemble(
        real_qc_pp: &Self::QCParams,
        signers: &BitSlice,
        sigs: &[<BLSOverBN254CurveSignatureScheme as SignatureScheme>::Signature],
    ) -> Self::QCType {
        BitVectorQC::<BLSOverBN254CurveSignatureScheme>::assemble(real_qc_pp, signers, sigs)
            .expect("this assembling shouldn't fail")
    }
}
