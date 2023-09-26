//! Provides two types of cerrtificates and their accumulators.

use crate::vote::DAVoteAccumulator;
use crate::vote::QuorumVote;
use crate::vote::QuorumVoteAccumulator;
use crate::vote::ViewSyncVoteAccumulator;
use crate::vote::VoteType;
use crate::{
    data::serialize_signature,
    traits::{
        election::{SignedCertificate, VoteData, VoteToken},
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey},
        state::ConsensusTime,
    },
    vote::{DAVote, ViewSyncData, ViewSyncVote},
};
use bincode::Options;
use commit::{Commitment, CommitmentBounds, Committable};

use espresso_systems_common::hotshot::tag;
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    ops::Deref,
};
use tracing::debug;

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

    /// committment to the block
    pub block_commitment: Commitment<TYPES::BlockType>,

    /// Assembled signature for certificate aggregation
    pub signatures: AssembledSignature<TYPES>,
}

/// The type used for Quorum Certificates
///
/// A Quorum Certificate is a threshold signature of the `Leaf` being proposed, as well as some
/// metadata, such as the `Stage` of consensus the quorum certificate was generated during.
#[derive(custom_debug::Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = ""))]
pub struct QuorumCertificate<TYPES: NodeType, COMMITMENT: CommitmentBounds> {
    /// commitment to previous leaf
    #[debug(skip)]
    pub leaf_commitment: COMMITMENT,
    /// Which view this QC relates to
    pub view_number: TYPES::Time,
    /// assembled signature for certificate aggregation
    pub signatures: AssembledSignature<TYPES>,
    /// If this QC is for the genesis block
    pub is_genesis: bool,
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds> Display
    for QuorumCertificate<TYPES, COMMITMENT>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "view: {:?}, is_genesis: {:?}",
            self.view_number, self.is_genesis
        )
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
    /// These signatures are for genesis certificate
    Genesis(),
    /// These signatures are for ViewSyncPreCommit
    ViewSyncPreCommit(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for ViewSyncCommit
    ViewSyncCommit(<TYPES::SignatureKey as SignatureKey>::QCType),
    /// These signatures are for ViewSyncFinalize
    ViewSyncFinalize(<TYPES::SignatureKey as SignatureKey>::QCType),
}

/// Data from a vote needed to accumulate into a `SignedCertificate`
pub struct VoteMetaData<COMMITMENT: CommitmentBounds, T: VoteToken, TIME> {
    /// Voter's public key
    pub encoded_key: EncodedPublicKey,
    /// Votes signature
    pub encoded_signature: EncodedSignature,
    /// Commitment to what's voted on.  E.g. the leaf for a `QuorumCertificate`
    pub commitment: COMMITMENT,
    /// Data of the vote, yes, no, timeout, or DA
    pub data: VoteData<COMMITMENT>,
    /// The votes's token
    pub vote_token: T,
    /// View number for the vote
    pub view_number: TIME,
    /// The relay index for view sync
    // TODO ED Make VoteMetaData more generic to avoid this variable that only ViewSync uses
    pub relay: Option<u64>,
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, COMMITMENT>
    for QuorumCertificate<TYPES, COMMITMENT>
{
    type Vote = QuorumVote<TYPES, COMMITMENT>;
    type VoteAccumulator = QuorumVoteAccumulator<TYPES, COMMITMENT, Self::Vote>;

    fn from_signatures_and_commitment(
        signatures: AssembledSignature<TYPES>,
        vote: Self::Vote,
    ) -> Self {
        let leaf_commitment = match vote.clone() {
            QuorumVote::Yes(vote_internal) | QuorumVote::No(vote_internal) => {
                vote_internal.leaf_commitment
            }
            QuorumVote::Timeout(_) => unimplemented!(),
        };
        let qc = QuorumCertificate {
            leaf_commitment,
            view_number: vote.get_view(),
            signatures,
            is_genesis: false,
        };
        debug!("QC commitment when formed is {:?}", qc.leaf_commitment);
        qc
    }

    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        self.signatures.clone()
    }

    fn leaf_commitment(&self) -> COMMITMENT {
        self.leaf_commitment
    }

    fn set_leaf_commitment(&mut self, commitment: COMMITMENT) {
        self.leaf_commitment = commitment;
    }

    fn is_genesis(&self) -> bool {
        self.is_genesis
    }

    fn genesis() -> Self {
        // TODO GG need a new way to get fake commit now that we don't have Committable
        Self {
            leaf_commitment: COMMITMENT::default_commitment_no_preimage(),
            view_number: <TYPES::Time as ConsensusTime>::genesis(),
            signatures: AssembledSignature::Genesis(),
            is_genesis: true,
        }
    }
}

impl<TYPES: NodeType, COMMITMENT: CommitmentBounds> Committable
    for QuorumCertificate<TYPES, COMMITMENT>
{
    fn commit(&self) -> Commitment<Self> {
        let signatures_bytes = serialize_signature(&self.signatures);

        commit::RawCommitmentBuilder::new("Quorum Certificate Commitment")
            .var_size_field("leaf commitment", self.leaf_commitment.as_ref())
            .u64_field("view number", *self.view_number.deref())
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes)
            .finalize()
    }

    fn tag() -> String {
        tag::QC.to_string()
    }
}

impl<TYPES: NodeType>
    SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Commitment<TYPES::BlockType>>
    for DACertificate<TYPES>
{
    type Vote = DAVote<TYPES>;
    type VoteAccumulator = DAVoteAccumulator<TYPES, Commitment<TYPES::BlockType>, Self::Vote>;

    fn from_signatures_and_commitment(
        signatures: AssembledSignature<TYPES>,
        vote: Self::Vote,
    ) -> Self {
        DACertificate {
            view_number: vote.get_view(),
            signatures,
            block_commitment: vote.block_commitment,
        }
    }

    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }

    fn signatures(&self) -> AssembledSignature<TYPES> {
        self.signatures.clone()
    }

    fn leaf_commitment(&self) -> Commitment<TYPES::BlockType> {
        self.block_commitment
    }

    fn set_leaf_commitment(&mut self, _commitment: Commitment<TYPES::BlockType>) {
        // This function is only useful for QC. Will be removed after we have separated cert traits.
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

impl<TYPES: NodeType> Committable for ViewSyncCertificate<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        let signatures_bytes = serialize_signature(&self.signatures());

        let mut builder = commit::RawCommitmentBuilder::new("View Sync Certificate Commitment")
            // .field("leaf commitment", self.leaf_commitment)
            // .u64_field("view number", *self.view_number.deref())
            .constant_str("justify_qc signatures")
            .var_size_bytes(&signatures_bytes);

        // builder = builder
        //     .field("Leaf commitment", self.leaf_commitment)
        //     .u64_field("View number", *self.view_number.deref());

        let certificate_internal = match &self {
            // TODO ED Not the best way to do this
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
    fn from_signatures_and_commitment(
        signatures: AssembledSignature<TYPES>,
        vote: Self::Vote,
    ) -> Self {
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

    /// Set the leaf commitment.
    fn set_leaf_commitment(&mut self, _commitment: Commitment<ViewSyncData<TYPES>>) {
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
