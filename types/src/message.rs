//! Network message types
//!
//! This module contains types used to represent the various types of messages that
//! `HotShot` nodes can send among themselves.

use crate::data::QuorumProposal;
use crate::simple_certificate::{
    DACertificate2, VIDCertificate2, ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2,
    ViewSyncPreCommitCertificate2,
};
use crate::simple_vote::{
    DAVote2, TimeoutVote2, VIDVote2, ViewSyncCommitVote, ViewSyncFinalizeVote,
    ViewSyncPreCommitVote,
};
use crate::vote2::HasViewNumber;
use crate::{
    data::{DAProposal, VidDisperse},
    simple_vote::QuorumVote,
    traits::{
        network::{NetworkMsg, ViewMessage},
        node_implementation::NodeType,
        signature_key::EncodedSignature,
    },
};

use derivative::Derivative;
use either::Either::{self, Left, Right};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// Incoming message
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
pub struct Message<TYPES: NodeType> {
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
    /// Message with a quorum proposal.
    Proposal,
    /// Message with most recent proposal the server has
    CurrentProposal,
    /// Message with a quorum vote.
    Vote,
    /// Message with a view sync vote.
    ViewSyncVote,
    /// Message with a view sync proposal.
    ViewSyncProposal,
    /// Message with a DAC.
    DAC,
    /// Message for internal use
    Internal,
    /// Data message
    Data,
    /// VID disperse, like [`Proposal`].
    VidDisperse,
    /// VID vote, like [`Vote`].
    VidVote,
    /// VID certificate, like [`DAC`].
    VidCert,
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
    /// Construct a [`MessageKind`] from [`I::ConsensusMessage`].
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

/// Internal triggers sent by consensus messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
pub enum InternalTrigger<TYPES: NodeType> {
    // May add other triggers if necessary.
    /// Internal timeout at the specified view number.
    Timeout(TYPES::Time),
}

/// A processed consensus message for both validating and sequencing consensus.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(bound(deserialize = ""))]
pub enum ProcessedGeneralConsensusMessage<TYPES: NodeType> {
    /// Message with a quorum proposal.
    Proposal(Proposal<TYPES, QuorumProposal<TYPES>>, TYPES::SignatureKey),
    /// Message with a quorum vote.
    Vote(QuorumVote<TYPES>, TYPES::SignatureKey),
    /// Internal ONLY message indicating a view interrupt.
    #[serde(skip)]
    InternalTrigger(InternalTrigger<TYPES>),
}

impl<TYPES: NodeType> From<ProcessedGeneralConsensusMessage<TYPES>>
    for GeneralConsensusMessage<TYPES>
{
    fn from(value: ProcessedGeneralConsensusMessage<TYPES>) -> Self {
        match value {
            ProcessedGeneralConsensusMessage::Proposal(p, _) => {
                GeneralConsensusMessage::Proposal(p)
            }
            ProcessedGeneralConsensusMessage::Vote(v, _) => GeneralConsensusMessage::Vote(v),
            ProcessedGeneralConsensusMessage::InternalTrigger(a) => {
                GeneralConsensusMessage::InternalTrigger(a)
            }
        }
    }
}

impl<TYPES: NodeType> ProcessedGeneralConsensusMessage<TYPES> {
    /// Create a [`ProcessedGeneralConsensusMessage`] from a [`GeneralConsensusMessage`].
    /// # Panics
    /// if reaching the unimplemented `ViewSync` case.
    pub fn new(value: GeneralConsensusMessage<TYPES>, sender: TYPES::SignatureKey) -> Self {
        match value {
            GeneralConsensusMessage::Proposal(p) => {
                ProcessedGeneralConsensusMessage::Proposal(p, sender)
            }
            GeneralConsensusMessage::Vote(v) => ProcessedGeneralConsensusMessage::Vote(v, sender),
            GeneralConsensusMessage::InternalTrigger(a) => {
                ProcessedGeneralConsensusMessage::InternalTrigger(a)
            }
            // ED NOTE These are deprecated
            GeneralConsensusMessage::TimeoutVote(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncPreCommitVote(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncCommitVote(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncFinalizeVote(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncPreCommitCertificate(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncCommitCertificate(_) => unimplemented!(),
            GeneralConsensusMessage::ViewSyncFinalizeCertificate(_) => unimplemented!(),
        }
    }
}

/// A processed consensus message for the DA committee in sequencing consensus.
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(bound(deserialize = ""))]
pub enum ProcessedCommitteeConsensusMessage<TYPES: NodeType> {
    /// Proposal for the DA committee.
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>, TYPES::SignatureKey),
    /// Vote from the DA committee.
    DAVote(DAVote2<TYPES>, TYPES::SignatureKey),
    /// Certificate for the DA.
    DACertificate(DACertificate2<TYPES>, TYPES::SignatureKey),
    /// VID dispersal data. Like [`DAProposal`]
    VidDisperseMsg(Proposal<TYPES, VidDisperse<TYPES>>, TYPES::SignatureKey),
    /// Vote from VID storage node. Like [`DAVote`]
    VidVote(VIDVote2<TYPES>, TYPES::SignatureKey),
    /// Certificate for VID. Like [`DACertificate`]
    VidCertificate(VIDCertificate2<TYPES>, TYPES::SignatureKey),
}

impl<TYPES: NodeType> From<ProcessedCommitteeConsensusMessage<TYPES>>
    for CommitteeConsensusMessage<TYPES>
{
    fn from(value: ProcessedCommitteeConsensusMessage<TYPES>) -> Self {
        match value {
            ProcessedCommitteeConsensusMessage::DAProposal(p, _) => {
                CommitteeConsensusMessage::DAProposal(p)
            }
            ProcessedCommitteeConsensusMessage::DAVote(v, _) => {
                CommitteeConsensusMessage::DAVote(v)
            }
            ProcessedCommitteeConsensusMessage::DACertificate(cert, _) => {
                CommitteeConsensusMessage::DACertificate(cert)
            }
            ProcessedCommitteeConsensusMessage::VidDisperseMsg(disperse, _) => {
                CommitteeConsensusMessage::VidDisperseMsg(disperse)
            }
            ProcessedCommitteeConsensusMessage::VidVote(v, _) => {
                CommitteeConsensusMessage::VidVote(v)
            }
            ProcessedCommitteeConsensusMessage::VidCertificate(cert, _) => {
                CommitteeConsensusMessage::VidCertificate(cert)
            }
        }
    }
}

impl<TYPES: NodeType> ProcessedCommitteeConsensusMessage<TYPES> {
    /// Create a [`ProcessedCommitteeConsensusMessage`] from a [`CommitteeConsensusMessage`].
    pub fn new(value: CommitteeConsensusMessage<TYPES>, sender: TYPES::SignatureKey) -> Self {
        match value {
            CommitteeConsensusMessage::DAProposal(p) => {
                ProcessedCommitteeConsensusMessage::DAProposal(p, sender)
            }
            CommitteeConsensusMessage::DAVote(v) => {
                ProcessedCommitteeConsensusMessage::DAVote(v, sender)
            }
            CommitteeConsensusMessage::DACertificate(cert) => {
                ProcessedCommitteeConsensusMessage::DACertificate(cert, sender)
            }
            CommitteeConsensusMessage::VidDisperseMsg(disperse) => {
                ProcessedCommitteeConsensusMessage::VidDisperseMsg(disperse, sender)
            }
            CommitteeConsensusMessage::VidVote(v) => {
                ProcessedCommitteeConsensusMessage::VidVote(v, sender)
            }
            CommitteeConsensusMessage::VidCertificate(cert) => {
                ProcessedCommitteeConsensusMessage::VidCertificate(cert, sender)
            }
        }
    }
}

/// A processed consensus message for sequencing consensus.
pub type ProcessedSequencingMessage<TYPES> =
    Either<ProcessedGeneralConsensusMessage<TYPES>, ProcessedCommitteeConsensusMessage<TYPES>>;

impl<TYPES: NodeType> From<ProcessedSequencingMessage<TYPES>> for SequencingMessage<TYPES> {
    fn from(value: ProcessedSequencingMessage<TYPES>) -> Self {
        match value {
            Left(message) => SequencingMessage(Left(message.into())),
            Right(message) => SequencingMessage(Right(message.into())),
        }
    }
}

impl<TYPES: NodeType> From<ProcessedGeneralConsensusMessage<TYPES>>
    for ProcessedSequencingMessage<TYPES>
{
    fn from(value: ProcessedGeneralConsensusMessage<TYPES>) -> Self {
        Left(value)
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
    TimeoutVote(TimeoutVote2<TYPES>),

    /// Internal ONLY message indicating a view interrupt.
    #[serde(skip)]
    InternalTrigger(InternalTrigger<TYPES>),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = "", serialize = ""))]
/// Messages related to the sequencing consensus protocol for the DA committee.
pub enum CommitteeConsensusMessage<TYPES: NodeType> {
    /// Proposal for data availability committee
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>),

    /// vote for data availability committee
    DAVote(DAVote2<TYPES>),

    /// Certificate data is available
    DACertificate(DACertificate2<TYPES>),

    /// Initiate VID dispersal.
    ///
    /// Like [`DAProposal`]. Use `Msg` suffix to distinguish from [`VidDisperse`].
    /// TODO this variant should not be a [`CommitteeConsensusMessage`] because <https://github.com/EspressoSystems/HotShot/issues/1696>
    VidDisperseMsg(Proposal<TYPES, VidDisperse<TYPES>>),

    /// Vote for VID disperse data
    ///
    /// Like [`DAVote`].
    VidVote(VIDVote2<TYPES>),
    /// VID certificate data is available
    ///
    /// Like [`DACertificate`]
    VidCertificate(VIDCertificate2<TYPES>),
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
                    GeneralConsensusMessage::InternalTrigger(trigger) => match trigger {
                        InternalTrigger::Timeout(time) => *time,
                    },

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
                    CommitteeConsensusMessage::VidCertificate(cert) => cert.view_number,
                    CommitteeConsensusMessage::VidDisperseMsg(disperse) => {
                        disperse.data.get_view_number()
                    }
                    CommitteeConsensusMessage::VidVote(vote) => vote.get_view_number(),
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
                GeneralConsensusMessage::InternalTrigger(_) => MessagePurpose::Internal,
                GeneralConsensusMessage::ViewSyncPreCommitVote(_)
                | GeneralConsensusMessage::ViewSyncCommitVote(_)
                | GeneralConsensusMessage::ViewSyncFinalizeVote(_) => MessagePurpose::ViewSyncVote,

                GeneralConsensusMessage::ViewSyncPreCommitCertificate(_)
                | GeneralConsensusMessage::ViewSyncCommitCertificate(_)
                | GeneralConsensusMessage::ViewSyncFinalizeCertificate(_) => {
                    MessagePurpose::ViewSyncProposal
                }
            },
            Right(committee_message) => match committee_message {
                CommitteeConsensusMessage::DAProposal(_) => MessagePurpose::Proposal,
                CommitteeConsensusMessage::DAVote(_) => MessagePurpose::Vote,
                CommitteeConsensusMessage::VidVote(_) => MessagePurpose::VidVote,
                CommitteeConsensusMessage::DACertificate(_) => MessagePurpose::DAC,
                CommitteeConsensusMessage::VidDisperseMsg(_) => MessagePurpose::VidDisperse,
                CommitteeConsensusMessage::VidCertificate(_) => MessagePurpose::VidCert,
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
    pub signature: EncodedSignature,
    /// Phantom for TYPES
    pub _pd: PhantomData<TYPES>,
}
