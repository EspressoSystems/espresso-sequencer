//! Network message types
//!
//! This module contains types used to represent the various types of messages that
//! `HotShot` nodes can send among themselves.

use crate::data::{QuorumProposal, UpgradeProposal};
use crate::simple_certificate::{
    DACertificate, ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2,
    ViewSyncPreCommitCertificate2,
};
use crate::simple_vote::{
    DAVote, TimeoutVote, UpgradeVote, ViewSyncCommitVote, ViewSyncFinalizeVote,
    ViewSyncPreCommitVote,
};
use crate::traits::signature_key::SignatureKey;
use crate::vote::HasViewNumber;
use crate::{
    data::{DAProposal, VidDisperse},
    simple_vote::QuorumVote,
    traits::{
        network::{NetworkMsg, ViewMessage},
        node_implementation::NodeType,
    },
};

use derivative::Derivative;
use either::Either::{self, Left, Right};
use hotshot_constants::Version;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// Incoming message
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
pub struct Message<TYPES: NodeType> {
    /// The version of the protocol in use for this message
    pub version: Version,

    /// The sender of this message
    pub sender: TYPES::SignatureKey,

    /// The message kind
    pub kind: MessageKind<TYPES>,
}

impl<TYPES: NodeType> NetworkMsg for Message<TYPES> {}

impl<TYPES: NodeType> ViewMessage<TYPES> for Message<TYPES> {
    /// get the view number out of a message
    fn get_view_number(&self) -> TYPES::Time {
        self.kind.get_view_number()
    }
    fn purpose(&self) -> MessagePurpose {
        self.kind.purpose()
    }
}

/// A wrapper type for implementing `PassType` on a vector of `Message`.
#[derive(Clone, Debug)]
pub struct Messages<TYPES: NodeType>(pub Vec<Message<TYPES>>);

/// A message type agnostic description of a message's purpose
#[derive(PartialEq, Copy, Clone)]
pub enum MessagePurpose {
    /// Message with a [quorum/DA] proposal.
    Proposal,
    /// Message with most recent [quorum/DA] proposal the server has
    LatestProposal,
    /// Message with most recent view sync certificate the server has
    LatestViewSyncCertificate,
    /// Message with a quorum vote.
    Vote,
    /// Message with a view sync vote.
    ViewSyncVote,
    /// Message with a view sync certificate.
    ViewSyncCertificate,
    /// Message with a DAC.
    DAC,
    /// Message for internal use
    Internal,
    /// Data message
    Data,
    /// VID disperse, like [`Proposal`].
    VidDisperse,
    /// Message with an upgrade proposal.
    Upgrade,
}

// TODO (da) make it more customized to the consensus layer, maybe separating the specific message
// data from the kind enum.
/// Enum representation of any message type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = "", serialize = ""))]
pub enum MessageKind<TYPES: NodeType> {
    /// Messages related to the consensus protocol
    Consensus(SequencingMessage<TYPES>),
    /// Messages relating to sharing data between nodes
    Data(DataMessage<TYPES>),
}

impl<TYPES: NodeType> MessageKind<TYPES> {
    // Can't implement `From<I::ConsensusMessage>` directly due to potential conflict with
    // `From<DataMessage>`.
    /// Construct a [`MessageKind`] from [`SequencingMessage`].
    pub fn from_consensus_message(m: SequencingMessage<TYPES>) -> Self {
        Self::Consensus(m)
    }
}

impl<TYPES: NodeType> From<DataMessage<TYPES>> for MessageKind<TYPES> {
    fn from(m: DataMessage<TYPES>) -> Self {
        Self::Data(m)
    }
}

impl<TYPES: NodeType> ViewMessage<TYPES> for MessageKind<TYPES> {
    fn get_view_number(&self) -> TYPES::Time {
        match &self {
            MessageKind::Consensus(message) => message.view_number(),
            MessageKind::Data(DataMessage::SubmitTransaction(_, v)) => *v,
        }
    }

    fn purpose(&self) -> MessagePurpose {
        match &self {
            MessageKind::Consensus(message) => message.purpose(),
            MessageKind::Data(message) => match message {
                DataMessage::SubmitTransaction(_, _) => MessagePurpose::Data,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
/// Messages related to both validating and sequencing consensus.
pub enum GeneralConsensusMessage<TYPES: NodeType> {
    /// Message with a quorum proposal.
    Proposal(Proposal<TYPES, QuorumProposal<TYPES>>),

    /// Message with a quorum vote.
    Vote(QuorumVote<TYPES>),

    /// Message with a view sync pre-commit vote
    ViewSyncPreCommitVote(ViewSyncPreCommitVote<TYPES>),

    /// Message with a view sync commit vote
    ViewSyncCommitVote(ViewSyncCommitVote<TYPES>),

    /// Message with a view sync finalize vote
    ViewSyncFinalizeVote(ViewSyncFinalizeVote<TYPES>),

    /// Message with a view sync pre-commit certificate
    ViewSyncPreCommitCertificate(ViewSyncPreCommitCertificate2<TYPES>),

    /// Message with a view sync commit certificate
    ViewSyncCommitCertificate(ViewSyncCommitCertificate2<TYPES>),

    /// Message with a view sync finalize certificate
    ViewSyncFinalizeCertificate(ViewSyncFinalizeCertificate2<TYPES>),

    /// Message with a Timeout vote
    TimeoutVote(TimeoutVote<TYPES>),

    /// Message with an upgrade proposal
    UpgradeProposal(Proposal<TYPES, UpgradeProposal<TYPES>>),

    /// Message with an upgrade vote
    UpgradeVote(UpgradeVote<TYPES>),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = "", serialize = ""))]
/// Messages related to the sequencing consensus protocol for the DA committee.
pub enum CommitteeConsensusMessage<TYPES: NodeType> {
    /// Proposal for data availability committee
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>),

    /// vote for data availability committee
    DAVote(DAVote<TYPES>),

    /// Certificate data is available
    DACertificate(DACertificate<TYPES>),

    /// Initiate VID dispersal.
    ///
    /// Like [`DAProposal`]. Use `Msg` suffix to distinguish from [`VidDisperse`].
    /// TODO this variant should not be a [`CommitteeConsensusMessage`] because <https://github.com/EspressoSystems/HotShot/issues/1696>
    VidDisperseMsg(Proposal<TYPES, VidDisperse<TYPES>>),
}

/// Messages for sequencing consensus.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
pub struct SequencingMessage<TYPES: NodeType>(
    pub Either<GeneralConsensusMessage<TYPES>, CommitteeConsensusMessage<TYPES>>,
);

impl<TYPES: NodeType> SequencingMessage<TYPES> {
    // TODO: Disable panic after the `ViewSync` case is implemented.
    /// Get the view number this message relates to
    #[allow(clippy::panic)]
    fn view_number(&self) -> TYPES::Time {
        match &self.0 {
            Left(general_message) => {
                match general_message {
                    GeneralConsensusMessage::Proposal(p) => {
                        // view of leader in the leaf when proposal
                        // this should match replica upon receipt
                        p.data.get_view_number()
                    }
                    GeneralConsensusMessage::Vote(vote_message) => vote_message.get_view_number(),
                    GeneralConsensusMessage::TimeoutVote(message) => message.get_view_number(),
                    GeneralConsensusMessage::ViewSyncPreCommitVote(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::ViewSyncCommitVote(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::ViewSyncFinalizeVote(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::ViewSyncPreCommitCertificate(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::ViewSyncCommitCertificate(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::ViewSyncFinalizeCertificate(message) => {
                        message.get_view_number()
                    }
                    GeneralConsensusMessage::UpgradeProposal(message) => {
                        message.data.get_view_number()
                    }
                    GeneralConsensusMessage::UpgradeVote(message) => message.get_view_number(),
                }
            }
            Right(committee_message) => {
                match committee_message {
                    CommitteeConsensusMessage::DAProposal(p) => {
                        // view of leader in the leaf when proposal
                        // this should match replica upon receipt
                        p.data.get_view_number()
                    }
                    CommitteeConsensusMessage::DAVote(vote_message) => {
                        vote_message.get_view_number()
                    }
                    CommitteeConsensusMessage::DACertificate(cert) => cert.view_number,
                    CommitteeConsensusMessage::VidDisperseMsg(disperse) => {
                        disperse.data.get_view_number()
                    }
                }
            }
        }
    }

    // TODO: Disable panic after the `ViewSync` case is implemented.
    /// Get the message purpos
    #[allow(clippy::panic)]
    fn purpose(&self) -> MessagePurpose {
        match &self.0 {
            Left(general_message) => match general_message {
                GeneralConsensusMessage::Proposal(_) => MessagePurpose::Proposal,
                GeneralConsensusMessage::Vote(_) | GeneralConsensusMessage::TimeoutVote(_) => {
                    MessagePurpose::Vote
                }
                GeneralConsensusMessage::ViewSyncPreCommitVote(_)
                | GeneralConsensusMessage::ViewSyncCommitVote(_)
                | GeneralConsensusMessage::ViewSyncFinalizeVote(_) => MessagePurpose::ViewSyncVote,

                GeneralConsensusMessage::ViewSyncPreCommitCertificate(_)
                | GeneralConsensusMessage::ViewSyncCommitCertificate(_)
                | GeneralConsensusMessage::ViewSyncFinalizeCertificate(_) => {
                    MessagePurpose::ViewSyncCertificate
                }

                GeneralConsensusMessage::UpgradeProposal(_)
                | GeneralConsensusMessage::UpgradeVote(_) => MessagePurpose::Upgrade,
            },
            Right(committee_message) => match committee_message {
                CommitteeConsensusMessage::DAProposal(_) => MessagePurpose::Proposal,
                CommitteeConsensusMessage::DAVote(_) => MessagePurpose::Vote,
                CommitteeConsensusMessage::DACertificate(_) => MessagePurpose::DAC,
                CommitteeConsensusMessage::VidDisperseMsg(_) => MessagePurpose::VidDisperse,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
/// Messages related to sending data between nodes
pub enum DataMessage<TYPES: NodeType> {
    /// Contains a transaction to be submitted
    /// TODO rethink this when we start to send these messages
    /// we only need the view number for broadcast
    SubmitTransaction(TYPES::Transaction, TYPES::Time),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
/// Prepare qc from the leader
pub struct Proposal<TYPES: NodeType, PROPOSAL: HasViewNumber<TYPES> + DeserializeOwned> {
    // NOTE: optimization could include view number to help look up parent leaf
    // could even do 16 bit numbers if we want
    /// The data being proposed.
    pub data: PROPOSAL,
    /// The proposal must be signed by the view leader
    pub signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    /// Phantom for TYPES
    pub _pd: PhantomData<TYPES>,
}
