//! Provides two types of cerrtificates and their accumulators.

use crate::{
    data::serialize_signature,
    traits::{
        election::SignedCertificate, node_implementation::NodeType, signature_key::SignatureKey,
    },
    vote::{ViewSyncData, ViewSyncVote, ViewSyncVoteAccumulator, VoteType},
};
use bincode::Options;
use commit::{Commitment, Committable};

use espresso_systems_common::hotshot::tag;
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

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
