//! Provides two types of cerrtificates and their accumulators.

use crate::{
    data::serialize_signature,
    traits::{
        election::SignedCertificate, node_implementation::NodeType, signature_key::SignatureKey,
        state::ConsensusTime,
    },
    vote::{
        DAVote, DAVoteAccumulator, QuorumVote, QuorumVoteAccumulator, TimeoutVote,
        TimeoutVoteAccumulator, VIDVote, VIDVoteAccumulator, ViewSyncData, ViewSyncVote,
        ViewSyncVoteAccumulator, VoteType,
    },
};
use bincode::Options;
use commit::{Commitment, CommitmentBounds, Committable};

use espresso_systems_common::hotshot::tag;
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    marker::PhantomData,
};

/// A `DACertificate` is a threshold signature that some data is available.
/// It is signed by the members of the DA committee, not the entire network. It is used
/// to prove that the data will be made available to those outside of the DA committee.
#[derive(Clone, PartialEq, custom_debug::Debug, serde::Serialize, serde::Deserialize, Hash)]
#[serde(bound(deserialize = ""))]
pub struct DACertificate<TYPES: NodeType> {
    /// The view number this quorum certificate was generated during
    ///
    /// This value is covered by the threshold signature.
    pub view_number: TYPES::Time,

    /// committment to the block payload
    pub payload_commitment: Commitment<TYPES::BlockPayload>,

    /// Assembled signature for certificate aggregation
    pub signatures: AssembledSignature<TYPES>,
}

/// A `VIDCertificate` is a threshold signature that some data is available.
/// It is signed by the whole quorum.
#[derive(Clone, PartialEq, custom_debug::Debug, serde::Serialize, serde::Deserialize, Hash)]
#[serde(bound(deserialize = ""))]
pub struct VIDCertificate<TYPES: NodeType> {
    /// The view number this VID certificate was generated during
    pub view_number: TYPES::Time,

    /// Committment to the block payload
    pub payload_commitment: Commitment<TYPES::BlockPayload>,

    /// Assembled signature for certificate aggregation
    pub signatures: AssembledSignature<TYPES>,
}

/// Depricated type for QC

// TODO:remove this struct https://github.com/EspressoSystems/HotShot/issues/1995
#[derive(custom_debug::Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = ""))]
pub struct QuorumCertificate<TYPES: NodeType, COMMITMENT: CommitmentBounds> {
    /// phantom data
    _pd: PhantomData<(TYPES, COMMITMENT)>,
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds> Display
    for QuorumCertificate<TYPES, COMMITMENT>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "",)
    }
}

/// Timeout Certificate
#[derive(custom_debug::Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
pub struct TimeoutCertificate<TYPES: NodeType> {
    /// View that timed out
    pub view_number: TYPES::Time,
    /// assembled signature for certificate aggregation
    pub signatures: AssembledSignature<TYPES>,
}

impl<TYPES: NodeType>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Commitment<TYPES::Time>>
    for TimeoutCertificate<TYPES>
{
    type Vote = TimeoutVote<TYPES>;

    type VoteAccumulator = TimeoutVoteAccumulator<TYPES, Commitment<TYPES::Time>, Self::Vote>;

    fn create_certificate(signatures: AssembledSignature<TYPES>, vote: Self::Vote) -> Self {
        TimeoutCertificate {
            view_number: vote.get_view(),
            signatures,
        }
    }

    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        self.signatures.clone()
    }

    fn leaf_commitment(&self) -> Commitment<TYPES::Time> {
        self.view_number.commit()
    }

    fn is_genesis(&self) -> bool {
        false
    }

    fn genesis() -> Self {
        unimplemented!()
    }
}

/// Certificate for view sync.
#[derive(custom_debug::Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub enum ViewSyncCertificate<TYPES: NodeType> {
    /// Pre-commit phase.
    PreCommit(ViewSyncCertificateInternal<TYPES>),
    /// Commit phase.
    Commit(ViewSyncCertificateInternal<TYPES>),
    /// Finalize phase.
    Finalize(ViewSyncCertificateInternal<TYPES>),
}

impl<TYPES: NodeType> ViewSyncCertificate<TYPES> {
    /// Serialize the certificate into bytes.
    /// # Panics
    /// If the serialization fails.
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode_opts().serialize(&self).unwrap()
    }
}

/// A view sync certificate representing a quorum of votes for a particular view sync phase
#[derive(custom_debug::Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub struct ViewSyncCertificateInternal<TYPES: NodeType> {
    /// Relay the votes are intended for
    pub relay: u64,
    /// View number the network is attempting to synchronize on
    pub round: TYPES::Time,
    /// Aggregated QC
    pub signatures: AssembledSignature<TYPES>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
/// Enum representing whether a signatures is for a 'Yes' or 'No' or 'DA' or 'Genesis' certificate
pub enum AssembledSignature<TYPES: NodeType> {
    // (enum, signature)
    /// These signatures are for a 'Yes' certificate
    Yes(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for a 'No' certificate
    No(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for a 'DA' certificate
    DA(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for a 'VID' certificate
    VID(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for a `Timeout` certificate
    Timeout(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for genesis certificate
    Genesis(),
    /// These signatures are for ViewSyncPreCommit
    ViewSyncPreCommit(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for ViewSyncCommit
    ViewSyncCommit(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for ViewSyncFinalize
    ViewSyncFinalize(<TYPES::SignatureKey as SignatureKey>::QCType),
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, COMMITMENT>
    for QuorumCertificate<TYPES, COMMITMENT>
{
    type Vote = QuorumVote<TYPES, COMMITMENT>;
    type VoteAccumulator = QuorumVoteAccumulator<TYPES, COMMITMENT, Self::Vote>;

    fn create_certificate(_signatures: AssembledSignature<TYPES>, _vote: Self::Vote) -> Self {
        Self { _pd: PhantomData }
    }

    fn view_number(&self) -> TYPES::Time {
        TYPES::Time::new(1)
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        AssembledSignature::Genesis()
    }

    fn leaf_commitment(&self) -> COMMITMENT {
        COMMITMENT::default_commitment_no_preimage()
    }

    fn is_genesis(&self) -> bool {
        true
    }

    fn genesis() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds> Committable
    for QuorumCertificate<TYPES, COMMITMENT>
{
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Quorum Certificate Commitment")
            .u64_field("view number", 1)
            .finalize()
    }
}

impl<TYPES: NodeType>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Commitment<TYPES::BlockPayload>>
    for DACertificate<TYPES>
{
    type Vote = DAVote<TYPES>;
    type VoteAccumulator = DAVoteAccumulator<TYPES, Commitment<TYPES::BlockPayload>, Self::Vote>;

    fn create_certificate(signatures: AssembledSignature<TYPES>, vote: Self::Vote) -> Self {
        DACertificate {
            view_number: vote.get_view(),
            signatures,
            payload_commitment: vote.payload_commitment,
        }
    }

    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        self.signatures.clone()
    }

    fn leaf_commitment(&self) -> Commitment<TYPES::BlockPayload> {
        self.payload_commitment
    }

    fn is_genesis(&self) -> bool {
        // This function is only useful for QC. Will be removed after we have separated cert traits.
        false
    }

    fn genesis() -> Self {
        // This function is only useful for QC. Will be removed after we have separated cert traits.
        unimplemented!()
    }
}

impl<TYPES: NodeType>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Commitment<TYPES::BlockPayload>>
    for VIDCertificate<TYPES>
{
    type Vote = VIDVote<TYPES>;
    type VoteAccumulator = VIDVoteAccumulator<TYPES, Commitment<TYPES::BlockPayload>, Self::Vote>;

    fn create_certificate(signatures: AssembledSignature<TYPES>, vote: Self::Vote) -> Self {
        VIDCertificate {
            view_number: vote.get_view(),
            signatures,
            payload_commitment: vote.payload_commitment,
        }
    }

    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        self.signatures.clone()
    }

    fn leaf_commitment(&self) -> Commitment<TYPES::BlockPayload> {
        self.payload_commitment
    }

    fn is_genesis(&self) -> bool {
        // This function is only useful for QC. Will be removed after we have separated cert traits.
        false
    }

    fn genesis() -> Self {
        // This function is only useful for QC. Will be removed after we have separated cert traits.
        unimplemented!()
    }
}

impl<TYPES: NodeType> Eq for DACertificate<TYPES> {}

impl<TYPES: NodeType> Eq for VIDCertificate<TYPES> {}

impl<TYPES: NodeType> Committable for ViewSyncCertificate<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        let signatures_bytes = serialize_signature(&self.signatures());

        let mut builder = commit::RawCommitmentBuilder::new("View Sync Certificate Commitment")
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes);

        let certificate_internal = match &self {
            ViewSyncCertificate::PreCommit(certificate_internal) => {
                builder = builder.var_size_field("View Sync Phase", "PreCommit".as_bytes());
                certificate_internal
            }
            ViewSyncCertificate::Commit(certificate_internal) => {
                builder = builder.var_size_field("View Sync Phase", "Commit".as_bytes());
                certificate_internal
            }
            ViewSyncCertificate::Finalize(certificate_internal) => {
                builder = builder.var_size_field("View Sync Phase", "Finalize".as_bytes());
                certificate_internal
            }
        };

        builder = builder
            .u64_field("Relay", certificate_internal.relay)
            .u64_field("Round", *certificate_internal.round);
        builder.finalize()
    }

    fn tag() -> String {
        // TODO ED Update this repo with a view sync tag
        tag::QC.to_string()
    }
}

impl<TYPES: NodeType>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Commitment<ViewSyncData<TYPES>>>
    for ViewSyncCertificate<TYPES>
{
    type Vote = ViewSyncVote<TYPES>;
    type VoteAccumulator =
        ViewSyncVoteAccumulator<TYPES, Commitment<ViewSyncData<TYPES>>, Self::Vote>;
    /// Build a QC from the threshold signature and commitment
    fn create_certificate(signatures: AssembledSignature<TYPES>, vote: Self::Vote) -> Self {
        let certificate_internal = ViewSyncCertificateInternal {
            round: vote.get_view(),
            relay: vote.relay(),
            signatures: signatures.clone(),
        };
        match signatures {
            AssembledSignature::ViewSyncPreCommit(_) => {
                ViewSyncCertificate::PreCommit(certificate_internal)
            }
            AssembledSignature::ViewSyncCommit(_) => {
                ViewSyncCertificate::Commit(certificate_internal)
            }
            AssembledSignature::ViewSyncFinalize(_) => {
                ViewSyncCertificate::Finalize(certificate_internal)
            }
            _ => unimplemented!(),
        }
    }

    /// Get the view number.
    fn view_number(&self) -> TYPES::Time {
        match self.clone() {
            ViewSyncCertificate::PreCommit(certificate_internal)
            | ViewSyncCertificate::Commit(certificate_internal)
            | ViewSyncCertificate::Finalize(certificate_internal) => certificate_internal.round,
        }
    }

    /// Get signatures.
    fn signatures(&self) -> AssembledSignature<TYPES> {
        match self.clone() {
            ViewSyncCertificate::PreCommit(certificate_internal)
            | ViewSyncCertificate::Commit(certificate_internal)
            | ViewSyncCertificate::Finalize(certificate_internal) => {
                certificate_internal.signatures
            }
        }
    }

    // TODO (da) the following functions should be refactored into a QC-specific trait.
    /// Get the leaf commitment.
    fn leaf_commitment(&self) -> Commitment<ViewSyncData<TYPES>> {
        todo!()
    }

    /// Get whether the certificate is for the genesis block.
    fn is_genesis(&self) -> bool {
        todo!()
    }

    /// To be used only for generating the genesis quorum certificate; will fail if used anywhere else
    fn genesis() -> Self {
        todo!()
    }
}
impl<TYPES: NodeType> Eq for ViewSyncCertificate<TYPES> {}
