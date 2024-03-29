//! The quorum certificate (QC) trait is a certificate of a sufficient quorum of distinct
//! parties voted for a message or statement.

use ark_std::{
    rand::{CryptoRng, RngCore},
    vec::Vec,
};
use bitvec::prelude::*;
use generic_array::{ArrayLength, GenericArray};
use jf_primitives::{errors::PrimitivesError, signatures::AggregateableSignatureSchemes};
use serde::{Deserialize, Serialize};

/// Trait for validating a QC built from different signatures on the same message
pub trait QuorumCertificateScheme<
    A: AggregateableSignatureSchemes + Serialize + for<'a> Deserialize<'a>,
>
{
    /// Public parameters for generating the QC
    /// E.g: snark proving/verifying keys, list of (or pointer to) public keys stored in the smart contract.
    type QCProverParams: Serialize + for<'a> Deserialize<'a>;

    /// Public parameters for validating the QC
    /// E.g: verifying keys, stake table commitment
    type QCVerifierParams: Serialize + for<'a> Deserialize<'a>;

    /// Allows to fix the size of the message at compilation time.
    type MessageLength: ArrayLength<A::MessageUnit>;

    /// Type of the actual quorum certificate object
    type QC;

    /// Type of the quorum size (e.g. number of votes or accumulated weight of signatures)
    type QuorumSize;

    /// Produces a partial signature on a message with a single user signing key
    /// NOTE: the original message (vote) should be prefixed with the hash of the stake table.
    /// * `agg_sig_pp` - public parameters for aggregate signature
    /// * `message` - message to be signed
    /// * `sk` - user signing key
    /// * `returns` - a "simple" signature
    ///
    /// # Errors
    ///
    /// Should return error if the underlying signature scheme fail to sign.
    fn sign<R: CryptoRng + RngCore, M: AsRef<[A::MessageUnit]>>(
        pp: &A::PublicParameter,
        sk: &A::SigningKey,
        msg: M,
        prng: &mut R,
    ) -> Result<A::Signature, PrimitivesError> {
        A::sign(pp, sk, msg, prng)
    }

    /// Computes an aggregated signature from a set of partial signatures and the verification keys involved
    /// * `qc_pp` - public parameters for generating the QC
    /// * `signers` - a bool vector indicating the list of verification keys corresponding to the set of partial signatures
    /// * `sigs` - partial signatures on the same message
    ///
    /// # Errors
    ///
    /// Will return error if some of the partial signatures provided are invalid or the number of
    /// partial signatures / verifications keys are different.
    fn assemble(
        qc_pp: &Self::QCProverParams,
        signers: &BitSlice,
        sigs: &[A::Signature],
    ) -> Result<Self::QC, PrimitivesError>;

    /// Checks an aggregated signature over some message provided as input
    /// * `qc_vp` - public parameters for validating the QC
    /// * `message` - message to check the aggregated signature against
    /// * `qc` - quorum certificate
    /// * `returns` - the quorum size if the qc is valid, an error otherwise.
    ///
    /// # Errors
    ///
    /// Return error if the QC is invalid, either because accumulated weight didn't exceed threshold,
    /// or some partial signatures are invalid.
    fn check(
        qc_vp: &Self::QCVerifierParams,
        message: &GenericArray<A::MessageUnit, Self::MessageLength>,
        qc: &Self::QC,
    ) -> Result<Self::QuorumSize, PrimitivesError>;

    /// Trace the list of signers given a qc.
    ///
    /// # Errors
    ///
    /// Return error if the inputs mismatch (e.g. wrong verifier parameter or original message).
    fn trace(
        qc_vp: &Self::QCVerifierParams,
        message: &GenericArray<A::MessageUnit, Self::MessageLength>,
        qc: &Self::QC,
    ) -> Result<Vec<A::VerificationKey>, PrimitivesError>;
}
