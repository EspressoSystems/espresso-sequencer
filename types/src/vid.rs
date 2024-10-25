// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! This module provides an opaque constructor [`vid_scheme`] that returns a new
//! instance of a VID scheme.
//!
//! Purpose: the specific choice of VID scheme is an implementation detail. We
//! want all communication with the VID scheme to occur only via the API exposed
//! in the [`VidScheme`] trait, but limitations of Rust make it difficult to
//! achieve this level of abstraction. Hence, there's a lot of boilerplate code
//! in this module.

#![allow(missing_docs)]
use std::{fmt::Debug, ops::Range};

use ark_bn254::Bn254;
use derive_more::Display;
use jf_pcs::{
    prelude::{UnivariateKzgPCS, UnivariateUniversalParams},
    PolynomialCommitmentScheme,
};
use jf_vid::{
    advz::{
        self,
        payload_prover::{LargeRangeProof, SmallRangeProof},
    },
    payload_prover::{PayloadProver, Statement},
    precomputable::Precomputable,
    VidDisperse, VidResult, VidScheme,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tagged_base64::TaggedBase64;

use crate::{
    constants::SRS_DEGREE,
    data::{VidDisperse as HotShotVidDisperse, VidDisperseShare},
    message::Proposal,
};

/// VID scheme constructor.
///
/// Returns an opaque type that impls jellyfish traits:
/// [`VidScheme`], [`PayloadProver`], [`Precomputable`].
///
/// # Rust forbids naming impl Trait in return types
///
/// Due to Rust limitations the return type of [`vid_scheme`] is a newtype
/// wrapper [`VidSchemeType`] that impls the above traits.
///
/// We prefer that the return type of [`vid_scheme`] be `impl Trait` for the
/// above traits. But the ability to name an impl Trait return type is
/// currently missing from Rust:
/// - [Naming impl trait in return types - Impl trait initiative](https://rust-lang.github.io/impl-trait-initiative/explainer/rpit_names.html)
/// - [RFC: Type alias impl trait (TAIT)](https://github.com/rust-lang/rfcs/blob/master/text/2515-type_alias_impl_trait.md)
///
/// # Panics
/// When the construction fails for the underlying VID scheme.
#[must_use]
#[memoize::memoize(SharedCache, Capacity: 10)]
pub fn vid_scheme(num_storage_nodes: usize) -> VidSchemeType {
    // recovery_threshold is currently num_storage_nodes rounded down to a power of two
    // TODO recovery_threshold should be a function of the desired erasure code rate
    // https://github.com/EspressoSystems/HotShot/issues/2152
    let recovery_threshold = 1 << num_storage_nodes.ilog2();

    #[allow(clippy::panic)]
    let num_storage_nodes = u32::try_from(num_storage_nodes).unwrap_or_else(|err| {
        panic!(
            "num_storage_nodes {num_storage_nodes} should fit into u32; \
                error: {err}"
        )
    });

    // TODO panic, return `Result`, or make `new` infallible upstream (eg. by panicking)?
    #[allow(clippy::panic)]
    VidSchemeType(
        Advz::new(num_storage_nodes, recovery_threshold, &*KZG_SRS).unwrap_or_else(|err| {
              panic!("advz construction failure: (num_storage nodes,recovery_threshold)=({num_storage_nodes},{recovery_threshold}); \
                      error: {err}")
        })
    )
}

/// Similar to [`vid_scheme()`], but with `KZG_SRS_TEST` for testing purpose only.
#[cfg(feature = "test-srs")]
#[memoize::memoize(SharedCache, Capacity: 10)]
pub fn vid_scheme_for_test(num_storage_nodes: usize) -> VidSchemeType {
    let recovery_threshold = 1 << num_storage_nodes.ilog2();
    #[allow(clippy::panic)]
    let num_storage_nodes = u32::try_from(num_storage_nodes).unwrap_or_else(|err| {
        panic!("num_storage_nodes {num_storage_nodes} should fit into u32; error: {err}")
    });
    #[allow(clippy::panic)]
    VidSchemeType(
        Advz::new(num_storage_nodes, recovery_threshold, &*KZG_SRS_TEST).unwrap_or_else(|err| {
           panic!("advz construction failure: (num_storage nodes,recovery_threshold)=({num_storage_nodes},{recovery_threshold});\
                   error: {err}")
        })
    )
}

/// VID proposal type
pub type VidProposal<TYPES> = (
    Proposal<TYPES, HotShotVidDisperse<TYPES>>,
    Vec<Proposal<TYPES, VidDisperseShare<TYPES>>>,
);

#[cfg(not(feature = "gpu-vid"))]
/// Internal Jellyfish VID scheme
type Advz = advz::Advz<E, H>;
#[cfg(feature = "gpu-vid")]
/// Internal Jellyfish VID scheme
type Advz = advz::AdvzGPU<'static, E, H>;

/// Newtype wrapper for a VID scheme type that impls
/// [`VidScheme`], [`PayloadProver`], [`Precomputable`].
#[derive(Clone)]
pub struct VidSchemeType(Advz);

/// Newtype wrapper for assoc type [`VidScheme::Commit`].
#[derive(Clone, Debug, Deserialize, Display, Eq, PartialEq, Hash, Serialize)]
pub struct VidCommitment(<Advz as VidScheme>::Commit);

/// Newtype wrapper for assoc type [`VidScheme::Common`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct VidCommon(<Advz as VidScheme>::Common);

/// Newtype wrapper for assoc type [`VidScheme::Share`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct VidShare(<Advz as VidScheme>::Share);

/// Newtype wrapper for assoc type [`Precomputable::PrecomputeData`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct VidPrecomputeData(<Advz as Precomputable>::PrecomputeData);

/// Newtype wrapper for a large payload range proof.
///
/// Useful for namespace proofs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LargeRangeProofType(
    // # Type complexity
    //
    // Jellyfish's `LargeRangeProof` type has a prime field generic parameter `F`.
    // This `F` is determined by the type parameter `E` for `Advz`.
    // Jellyfish needs a more ergonomic way for downstream users to refer to this type.
    //
    // There is a `KzgEval` type alias in jellyfish that helps a little, but it's currently private:
    // <https://github.com/EspressoSystems/jellyfish/issues/423>
    // If it were public then we could instead use
    // `LargeRangeProof<KzgEval<E>>`
    // but that's still pretty crufty.
    LargeRangeProof<<UnivariateKzgPCS<E> as PolynomialCommitmentScheme>::Evaluation>,
);

/// Newtype wrapper for a small payload range proof.
///
/// Useful for transaction proofs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SmallRangeProofType(
    // # Type complexity
    //
    // Similar to the comments in `LargeRangeProofType`.
    SmallRangeProof<<UnivariateKzgPCS<E> as PolynomialCommitmentScheme>::Proof>,
);

#[cfg(feature = "test-srs")]
lazy_static! {
    /// SRS for testing only
    static ref KZG_SRS_TEST: UnivariateUniversalParams<E> = {
        let mut rng = jf_utils::test_rng();
        UnivariateKzgPCS::<E>::gen_srs_for_testing(
            &mut rng,
            SRS_DEGREE,
        )
        .unwrap()
    };
}

// By default, use SRS from Aztec's ceremony
lazy_static! {
    /// SRS comment
    static ref KZG_SRS: UnivariateUniversalParams<E> = {
        let srs = ark_srs::kzg10::aztec20::setup(SRS_DEGREE)
            .expect("Aztec SRS failed to load");
        UnivariateUniversalParams {
            powers_of_g: srs.powers_of_g,
            h: srs.h,
            beta_h: srs.beta_h,
            powers_of_h: vec![srs.h, srs.beta_h],
        }
    };
}

/// Private type alias for the EC pairing type parameter for [`Advz`].
type E = Bn254;
/// Private type alias for the hash type parameter for [`Advz`].
type H = Sha256;

// THE REST OF THIS FILE IS BOILERPLATE
//
// All this boilerplate can be deleted when we finally get
// type alias impl trait (TAIT):
// [rfcs/text/2515-type_alias_impl_trait.md at master · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/master/text/2515-type_alias_impl_trait.md)
impl VidScheme for VidSchemeType {
    type Commit = VidCommitment;
    type Share = VidShare;
    type Common = VidCommon;

    fn commit_only<B>(&mut self, payload: B) -> VidResult<Self::Commit>
    where
        B: AsRef<[u8]>,
    {
        self.0.commit_only(payload).map(VidCommitment)
    }

    fn disperse<B>(&mut self, payload: B) -> VidResult<VidDisperse<Self>>
    where
        B: AsRef<[u8]>,
    {
        self.0.disperse(payload).map(vid_disperse_conversion)
    }

    fn verify_share(
        &self,
        share: &Self::Share,
        common: &Self::Common,
        commit: &Self::Commit,
    ) -> VidResult<Result<(), ()>> {
        self.0.verify_share(&share.0, &common.0, &commit.0)
    }

    fn recover_payload(&self, shares: &[Self::Share], common: &Self::Common) -> VidResult<Vec<u8>> {
        // TODO costly Vec copy. Is the compiler smart enough to optimize away
        // this Vec, or shall we use unsafe code to cast `shares`?
        // It's only `recover_payload` so who cares?
        let shares: Vec<_> = shares.iter().map(|s| s.0.clone()).collect();

        self.0.recover_payload(&shares, &common.0)
    }

    fn is_consistent(commit: &Self::Commit, common: &Self::Common) -> VidResult<()> {
        <Advz as VidScheme>::is_consistent(&commit.0, &common.0)
    }

    fn get_payload_byte_len(common: &Self::Common) -> u32 {
        <Advz as VidScheme>::get_payload_byte_len(&common.0)
    }

    fn get_num_storage_nodes(common: &Self::Common) -> u32 {
        <Advz as VidScheme>::get_num_storage_nodes(&common.0)
    }

    fn get_multiplicity(common: &Self::Common) -> u32 {
        <Advz as VidScheme>::get_multiplicity(&common.0)
    }

    /// Helper function for testing only
    #[cfg(feature = "test-srs")]
    fn corrupt_share_index(&self, share: Self::Share) -> Self::Share {
        self.0.corrupt_share_index(share)
    }
}

impl From<VidCommitment> for TaggedBase64 {
    fn from(value: VidCommitment) -> Self {
        TaggedBase64::from(value.0)
    }
}

impl<'a> TryFrom<&'a TaggedBase64> for VidCommitment {
    type Error = <<Advz as VidScheme>::Commit as TryFrom<&'a TaggedBase64>>::Error;

    fn try_from(value: &'a TaggedBase64) -> Result<Self, Self::Error> {
        <Advz as VidScheme>::Commit::try_from(value).map(Self)
    }
}

// TODO add AsRef trait bound upstream.
//
// We impl `AsRef<[u8; _]>` instead of `AsRef<[u8]>` to maintain backward
// compatibility for downstream code that re-hashes the `VidCommitment`.
// Specifically, if we support only `AsRef<[u8]>` then code that uses
// `Committable` must switch `fixed_size_bytes` to `var_size_bytes`, which
// prepends length data into the hash, thus changing the hash.
impl AsRef<[u8; 32]> for VidCommitment {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref().as_ref()
    }
}

impl PayloadProver<LargeRangeProofType> for VidSchemeType {
    fn payload_proof<B>(&self, payload: B, range: Range<usize>) -> VidResult<LargeRangeProofType>
    where
        B: AsRef<[u8]>,
    {
        self.0
            .payload_proof(payload, range)
            .map(LargeRangeProofType)
    }

    fn payload_verify(
        &self,
        stmt: Statement<'_, Self>,
        proof: &LargeRangeProofType,
    ) -> VidResult<Result<(), ()>> {
        self.0.payload_verify(stmt_conversion(stmt), &proof.0)
    }
}

impl PayloadProver<SmallRangeProofType> for VidSchemeType {
    fn payload_proof<B>(&self, payload: B, range: Range<usize>) -> VidResult<SmallRangeProofType>
    where
        B: AsRef<[u8]>,
    {
        self.0
            .payload_proof(payload, range)
            .map(SmallRangeProofType)
    }

    fn payload_verify(
        &self,
        stmt: Statement<'_, Self>,
        proof: &SmallRangeProofType,
    ) -> VidResult<Result<(), ()>> {
        self.0.payload_verify(stmt_conversion(stmt), &proof.0)
    }
}

impl Precomputable for VidSchemeType {
    type PrecomputeData = VidPrecomputeData;

    fn commit_only_precompute<B>(
        &self,
        payload: B,
    ) -> VidResult<(Self::Commit, Self::PrecomputeData)>
    where
        B: AsRef<[u8]>,
    {
        self.0
            .commit_only_precompute(payload)
            .map(|r| (VidCommitment(r.0), VidPrecomputeData(r.1)))
    }

    fn disperse_precompute<B>(
        &self,
        payload: B,
        data: &Self::PrecomputeData,
    ) -> VidResult<VidDisperse<Self>>
    where
        B: AsRef<[u8]>,
    {
        self.0
            .disperse_precompute(payload, &data.0)
            .map(vid_disperse_conversion)
    }
}

/// Convert a [`VidDisperse<Advz>`] to a [`VidDisperse<VidSchemeType>`].
///
/// Foreign type rules prevent us from doing:
/// - `impl From<VidDisperse<VidSchemeType>> for VidDisperse<Advz>`
/// - `impl VidDisperse<VidSchemeType> {...}`
/// and similarly for `Statement`.
/// Thus, we accomplish type conversion via functions.
fn vid_disperse_conversion(vid_disperse: VidDisperse<Advz>) -> VidDisperse<VidSchemeType> {
    // TODO costly Vec copy. Is the compiler smart enough to optimize away
    // this Vec, or shall we use unsafe code to cast `shares`?
    let shares = vid_disperse.shares.into_iter().map(VidShare).collect();

    VidDisperse {
        shares,
        common: VidCommon(vid_disperse.common),
        commit: VidCommitment(vid_disperse.commit),
    }
}

/// Convert a [`Statement<'_, VidSchemeType>`] to a [`Statement<'_, Advz>`].
fn stmt_conversion(stmt: Statement<'_, VidSchemeType>) -> Statement<'_, Advz> {
    Statement {
        payload_subslice: stmt.payload_subslice,
        range: stmt.range,
        commit: &stmt.commit.0,
        common: &stmt.common.0,
    }
}
