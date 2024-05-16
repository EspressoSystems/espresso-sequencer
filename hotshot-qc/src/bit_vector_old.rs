//! Implementation for `BitVectorQC` that uses BLS signature + Bit vector.
//! See more details in `HotShot` paper.

use ark_std::{
    fmt::Debug,
    format,
    marker::PhantomData,
    rand::{CryptoRng, RngCore},
    vec,
    vec::Vec,
};
use bitvec::prelude::*;
use ethereum_types::U256;
use generic_array::GenericArray;
use hotshot_types::traits::{qc::QuorumCertificate, signature_key::StakeTableEntryType};
use jf_signature::AggregateableSignatureSchemes;
use serde::{Deserialize, Serialize};
use typenum::U32;

/// An implementation of QC using BLS signature and a bit-vector.
#[derive(Serialize, Deserialize)]
pub struct BitVectorQC<A: AggregateableSignatureSchemes + Serialize + for<'a> Deserialize<'a>>(
    PhantomData<A>,
);

/// Stake table entry
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash, Eq)]
pub struct StakeTableEntry<V> {
    /// Stake table key
    pub stake_key: V,
    /// Stake table value
    pub stake_amount: U256,
}

impl<V> StakeTableEntryType for StakeTableEntry<V> {
    fn stake(&self) -> U256 {
        self.stake_amount
    }
}

/// Public parameters of [`BitVectorQC`]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct QCParams<V, P> {
    /// the stake table (snapshot) this QC is verified against
    pub stake_entries: Vec<StakeTableEntry<V>>,
    /// threshold for the accumulated "weight" of votes to form a QC
    pub threshold: U256,
    /// public parameter for the aggregated signature scheme
    pub agg_sig_pp: P,
}

impl<A> QuorumCertificate<A> for BitVectorQC<A>
where
    A: AggregateableSignatureSchemes + Serialize + for<'a> Deserialize<'a>,
{
    type QCProverParams = QCParams<A::VerificationKey, A::PublicParameter>;

    // TODO: later with SNARKs we'll use a smaller verifier parameter
    type QCVerifierParams = QCParams<A::VerificationKey, A::PublicParameter>;

    type QC = (A::Signature, BitVec);
    type MessageLength = U32;
    type QuorumSize = U256;

    fn sign<R: CryptoRng + RngCore>(
        agg_sig_pp: &A::PublicParameter,
        message: &GenericArray<A::MessageUnit, Self::MessageLength>,
        sk: &A::SigningKey,
        prng: &mut R,
    ) -> Result<A::Signature, PrimitivesError> {
        A::sign(agg_sig_pp, sk, message, prng)
    }

    fn assemble(
        qc_pp: &Self::QCProverParams,
        signers: &BitSlice,
        sigs: &[A::Signature],
    ) -> Result<Self::QC, PrimitivesError> {
        if signers.len() != qc_pp.stake_entries.len() {
            return Err(ParameterError(format!(
                "bit vector len {} != the number of stake entries {}",
                signers.len(),
                qc_pp.stake_entries.len(),
            )));
        }
        let total_weight: U256 =
            qc_pp
                .stake_entries
                .iter()
                .zip(signers.iter())
                .fold(U256::zero(), |acc, (entry, b)| {
                    if *b {
                        acc + entry.stake_amount
                    } else {
                        acc
                    }
                });
        if total_weight < qc_pp.threshold {
            return Err(ParameterError(format!(
                "total_weight {} less than threshold {}",
                total_weight, qc_pp.threshold,
            )));
        }
        let mut ver_keys = vec![];
        for (entry, b) in qc_pp.stake_entries.iter().zip(signers.iter()) {
            if *b {
                ver_keys.push(entry.stake_key.clone());
            }
        }
        if ver_keys.len() != sigs.len() {
            return Err(ParameterError(format!(
                "the number of ver_keys {} != the number of partial signatures {}",
                ver_keys.len(),
                sigs.len(),
            )));
        }
        let sig = A::aggregate(&qc_pp.agg_sig_pp, &ver_keys[..], sigs)?;

        Ok((sig, signers.into()))
    }

    fn check(
        qc_vp: &Self::QCVerifierParams,
        message: &GenericArray<A::MessageUnit, Self::MessageLength>,
        qc: &Self::QC,
    ) -> Result<Self::QuorumSize, PrimitivesError> {
        let (sig, signers) = qc;
        if signers.len() != qc_vp.stake_entries.len() {
            return Err(ParameterError(format!(
                "signers bit vector len {} != the number of stake entries {}",
                signers.len(),
                qc_vp.stake_entries.len(),
            )));
        }
        let total_weight: U256 =
            qc_vp
                .stake_entries
                .iter()
                .zip(signers.iter())
                .fold(U256::zero(), |acc, (entry, b)| {
                    if *b {
                        acc + entry.stake_amount
                    } else {
                        acc
                    }
                });
        if total_weight < qc_vp.threshold {
            return Err(ParameterError(format!(
                "total_weight {} less than threshold {}",
                total_weight, qc_vp.threshold,
            )));
        }
        let mut ver_keys = vec![];
        for (entry, b) in qc_vp.stake_entries.iter().zip(signers.iter()) {
            if *b {
                ver_keys.push(entry.stake_key.clone());
            }
        }
        A::multi_sig_verify(&qc_vp.agg_sig_pp, &ver_keys[..], message, sig)?;

        Ok(total_weight)
    }

    fn trace(
        qc_vp: &Self::QCVerifierParams,
        message: &GenericArray<<A>::MessageUnit, Self::MessageLength>,
        qc: &Self::QC,
    ) -> Result<Vec<<A>::VerificationKey>, PrimitivesError> {
        let (_sig, signers) = qc;
        if signers.len() != qc_vp.stake_entries.len() {
            return Err(ParameterError(format!(
                "signers bit vector len {} != the number of stake entries {}",
                signers.len(),
                qc_vp.stake_entries.len(),
            )));
        }

        Self::check(qc_vp, message, qc)?;

        let signer_pks: Vec<_> = qc_vp
            .stake_entries
            .iter()
            .zip(signers.iter())
            .filter(|(_, b)| **b)
            .map(|(pk, _)| pk.stake_key.clone())
            .collect();
        Ok(signer_pks)
    }
}

#[cfg(test)]
mod tests {
    use jf_signature::{
        bls_over_bn254::{BLSOverBN254CurveSignatureScheme, KeyPair},
        SignatureScheme,
    };

    use super::*;

    macro_rules! test_quorum_certificate {
        ($aggsig:tt) => {
            let mut rng = jf_utils::test_rng();
            let agg_sig_pp = $aggsig::param_gen(Some(&mut rng)).unwrap();
            let key_pair1 = KeyPair::generate(&mut rng);
            let key_pair2 = KeyPair::generate(&mut rng);
            let key_pair3 = KeyPair::generate(&mut rng);
            let entry1 = StakeTableEntry {
                stake_key: key_pair1.ver_key(),
                stake_amount: U256::from(3u8),
            };
            let entry2 = StakeTableEntry {
                stake_key: key_pair2.ver_key(),
                stake_amount: U256::from(5u8),
            };
            let entry3 = StakeTableEntry {
                stake_key: key_pair3.ver_key(),
                stake_amount: U256::from(7u8),
            };
            let qc_pp = QCParams {
                stake_entries: vec![entry1, entry2, entry3],
                threshold: U256::from(10u8),
                agg_sig_pp,
            };
            let msg = [72u8; 32];
            let sig1 = BitVectorQC::<$aggsig>::sign(
                &agg_sig_pp,
                &msg.into(),
                key_pair1.sign_key_ref(),
                &mut rng,
            )
            .unwrap();
            let sig2 = BitVectorQC::<$aggsig>::sign(
                &agg_sig_pp,
                &msg.into(),
                key_pair2.sign_key_ref(),
                &mut rng,
            )
            .unwrap();
            let sig3 = BitVectorQC::<$aggsig>::sign(
                &agg_sig_pp,
                &msg.into(),
                key_pair3.sign_key_ref(),
                &mut rng,
            )
            .unwrap();

            // happy path
            let signers = bitvec![0, 1, 1];
            let qc = BitVectorQC::<$aggsig>::assemble(
                &qc_pp,
                signers.as_bitslice(),
                &[sig2.clone(), sig3.clone()],
            )
            .unwrap();
            assert!(BitVectorQC::<$aggsig>::check(&qc_pp, &msg.into(), &qc).is_ok());
            assert_eq!(
                BitVectorQC::<$aggsig>::trace(&qc_pp, &msg.into(), &qc).unwrap(),
                vec![key_pair2.ver_key(), key_pair3.ver_key()],
            );

            // Check the QC and the QCParams can be serialized / deserialized
            assert_eq!(
                qc,
                Serializer::<STATIC_VER_0_1>::deserialize(
                    &Serializer::<STATIC_VER_0_1>::serialize(&qc).unwrap()
                )
                .unwrap()
            );

            assert_eq!(
                qc_pp,
                Serializer::<STATIC_VER_0_1>::deserialize(
                    &Serializer::<STATIC_VER_0_1>::serialize(&qc_pp).unwrap()
                )
                .unwrap()
            );

            // bad paths
            // number of signatures unmatch
            assert!(BitVectorQC::<$aggsig>::assemble(
                &qc_pp,
                signers.as_bitslice(),
                &[sig2.clone()]
            )
            .is_err());
            // total weight under threshold
            let active_bad = bitvec![1, 1, 0];
            assert!(BitVectorQC::<$aggsig>::assemble(
                &qc_pp,
                active_bad.as_bitslice(),
                &[sig1.clone(), sig2.clone()]
            )
            .is_err());
            // wrong bool vector length
            let active_bad_2 = bitvec![0, 1, 1, 0];
            assert!(BitVectorQC::<$aggsig>::assemble(
                &qc_pp,
                active_bad_2.as_bitslice(),
                &[sig2, sig3],
            )
            .is_err());

            assert!(BitVectorQC::<$aggsig>::check(
                &qc_pp,
                &msg.into(),
                &(qc.0.clone(), active_bad)
            )
            .is_err());
            assert!(BitVectorQC::<$aggsig>::check(
                &qc_pp,
                &msg.into(),
                &(qc.0.clone(), active_bad_2)
            )
            .is_err());
            let bad_msg = [70u8; 32];
            assert!(BitVectorQC::<$aggsig>::check(&qc_pp, &bad_msg.into(), &qc).is_err());

            let bad_sig = &sig1;
            assert!(
                BitVectorQC::<$aggsig>::check(&qc_pp, &msg.into(), &(bad_sig.clone(), qc.1))
                    .is_err()
            );
        };
    }
    #[test]
    fn crypto_test_quorum_certificate() {
        test_quorum_certificate!(BLSOverBN254CurveSignatureScheme);
    }
}
