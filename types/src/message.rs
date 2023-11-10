//! Network message types
//!
//! This module contains types used to represent the various types of messages that
//! `HotShot` nodes can send among themselves.

use crate::simple_certificate::{DACertificate2, VIDCertificate2};
use crate::simple_vote::{DAVote2, TimeoutVote2, VIDVote2};
use crate::traits::node_implementation::CommitteeMembership;
use crate::vote2::HasViewNumber;
use crate::{
    data::{DAProposal, VidDisperse},
    simple_vote::QuorumVote,
    traits::{
        network::{NetworkMsg, ViewMessage},
        node_implementation::{
            ExchangesType, NodeImplementation, NodeType, QuorumMembership, QuorumProposalType,
            VIDMembership, ViewSyncProposalType,
        },
        signature_key::EncodedSignature,
    },
    vote::ViewSyncVote,
};

use derivative::Derivative;
use either::Either::{self, Left, Right};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// Incoming message
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
pub struct Message<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The sender of this message
    pub sender: TYPES::SignatureKey,

    /// The message kind
    pub kind: MessageKind<TYPES, I>,

    /// Phantom data.
    pub _phantom: PhantomData<I>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> NetworkMsg for Message<TYPES, I> {}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ViewMessage<TYPES> for Message<TYPES, I> {
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
pub struct Messages<TYPES: NodeType, I: NodeImplementation<TYPES>>(pub Vec<Message<TYPES, I>>);

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
pub enum MessageKind<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Messages related to the consensus protocol
    Consensus(I::ConsensusMessage),
    /// Messages relating to sharing data between nodes
    Data(DataMessage<TYPES>),
    /// Phantom data.
    _Unreachable(PhantomData<I>),
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> MessageKind<TYPES, I> {
    // Can't implement `From<I::ConsensusMessage>` directly due to potential conflict with
    // `From<DataMessage>`.
    /// Construct a [`MessageKind`] from [`I::ConsensusMessage`].
    pub fn from_consensus_message(m: I::ConsensusMessage) -> Self {
        Self::Consensus(m)
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> From<DataMessage<TYPES>>
    for MessageKind<TYPES, I>
{
    fn from(m: DataMessage<TYPES>) -> Self {
        Self::Data(m)
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ViewMessage<TYPES> for MessageKind<TYPES, I> {
    fn get_view_number(&self) -> TYPES::Time {
        match &self {
            MessageKind::Consensus(message) => message.view_number(),
            MessageKind::Data(DataMessage::SubmitTransaction(_, v)) => *v,
            MessageKind::_Unreachable(_) => unimplemented!(),
        }
    }

    fn purpose(&self) -> MessagePurpose {
        match &self {
            MessageKind::Consensus(message) => message.purpose(),
            MessageKind::Data(message) => match message {
                DataMessage::SubmitTransaction(_, _) => MessagePurpose::Data,
            },
            MessageKind::_Unreachable(_) => unimplemented!(),
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
pub enum ProcessedGeneralConsensusMessage<TYPES: NodeType, I: NodeImplementation<TYPES>>
where
    I::Exchanges: ExchangesType<TYPES, Message<TYPES, I>>,
{
    /// Message with a quorum proposal.
    Proposal(Proposal<TYPES, QuorumProposalType<TYPES, I>>, TYPES::SignatureKey),
    /// Message with a quorum vote.
    Vote(
        QuorumVote<TYPES, QuorumMembership<TYPES, I>>,
        TYPES::SignatureKey,
    ),
    /// Message with a view sync vote.
    ViewSyncVote(ViewSyncVote<TYPES>),
    /// Message with a view sync certificate.
    ViewSyncCertificate(Proposal<TYPES, ViewSyncProposalType<TYPES, I>>),
    /// Internal ONLY message indicating a view interrupt.
    #[serde(skip)]
    InternalTrigger(InternalTrigger<TYPES>),
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> From<ProcessedGeneralConsensusMessage<TYPES, I>>
    for GeneralConsensusMessage<TYPES, I>
where
    I::Exchanges: ExchangesType<TYPES, Message<TYPES, I>>,
{
    fn from(value: ProcessedGeneralConsensusMessage<TYPES, I>) -> Self {
        match value {
            ProcessedGeneralConsensusMessage::Proposal(p, _) => {
                GeneralConsensusMessage::Proposal(p)
            }
            ProcessedGeneralConsensusMessage::Vote(v, _) => GeneralConsensusMessage::Vote(v),
            ProcessedGeneralConsensusMessage::InternalTrigger(a) => {
                GeneralConsensusMessage::InternalTrigger(a)
            }
            ProcessedGeneralConsensusMessage::ViewSyncCertificate(certificate) => {
                GeneralConsensusMessage::ViewSyncCertificate(certificate)
            }
            ProcessedGeneralConsensusMessage::ViewSyncVote(vote) => {
                GeneralConsensusMessage::ViewSyncVote(vote)
            }
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ProcessedGeneralConsensusMessage<TYPES, I>
where
    I::Exchanges: ExchangesType<TYPES, Message<TYPES, I>>,
{
    /// Create a [`ProcessedGeneralConsensusMessage`] from a [`GeneralConsensusMessage`].
    /// # Panics
    /// if reaching the unimplemented `ViewSync` case.
    pub fn new(value: GeneralConsensusMessage<TYPES, I>, sender: TYPES::SignatureKey) -> Self {
        match value {
            GeneralConsensusMessage::Proposal(p) => {
                ProcessedGeneralConsensusMessage::Proposal(p, sender)
            }
            GeneralConsensusMessage::Vote(v) => ProcessedGeneralConsensusMessage::Vote(v, sender),
            GeneralConsensusMessage::InternalTrigger(a) => {
                ProcessedGeneralConsensusMessage::InternalTrigger(a)
            }
            GeneralConsensusMessage::ViewSyncVote(_)
            | GeneralConsensusMessage::ViewSyncCertificate(_) => todo!(),
            GeneralConsensusMessage::TimeoutVote(_) => todo!(),
        }
    }
}

/// A processed consensus message for the DA committee in sequencing consensus.
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(bound(deserialize = ""))]
pub enum ProcessedCommitteeConsensusMessage<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Proposal for the DA committee.
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>, TYPES::SignatureKey),
    /// Vote from the DA committee.
    DAVote(
        DAVote2<TYPES, CommitteeMembership<TYPES, I>>,
        TYPES::SignatureKey,
    ),
    /// Certificate for the DA.
    DACertificate(DACertificate2<TYPES>, TYPES::SignatureKey),
    /// VID dispersal data. Like [`DAProposal`]
    VidDisperseMsg(Proposal<TYPES, VidDisperse<TYPES>>, TYPES::SignatureKey),
    /// Vote from VID storage node. Like [`DAVote`]
    VidVote(
        VIDVote2<TYPES, VIDMembership<TYPES, I>>,
        TYPES::SignatureKey,
    ),
    /// Certificate for VID. Like [`DACertificate`]
    VidCertificate(VIDCertificate2<TYPES>, TYPES::SignatureKey),
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>>
    From<ProcessedCommitteeConsensusMessage<TYPES, I>> for CommitteeConsensusMessage<TYPES, I>
{
    fn from(value: ProcessedCommitteeConsensusMessage<TYPES, I>) -> Self {
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ProcessedCommitteeConsensusMessage<TYPES, I> {
    /// Create a [`ProcessedCommitteeConsensusMessage`] from a [`CommitteeConsensusMessage`].
    pub fn new(value: CommitteeConsensusMessage<TYPES, I>, sender: TYPES::SignatureKey) -> Self {
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
pub type ProcessedSequencingMessage<TYPES, I> = Either<
    ProcessedGeneralConsensusMessage<TYPES, I>,
    ProcessedCommitteeConsensusMessage<TYPES, I>,
>;

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    > From<ProcessedSequencingMessage<TYPES, I>> for SequencingMessage<TYPES, I>
{
    fn from(value: ProcessedSequencingMessage<TYPES, I>) -> Self {
        match value {
            Left(message) => SequencingMessage(Left(message.into())),
            Right(message) => SequencingMessage(Right(message.into())),
        }
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    > From<ProcessedGeneralConsensusMessage<TYPES, I>> for ProcessedSequencingMessage<TYPES, I>
{
    fn from(value: ProcessedGeneralConsensusMessage<TYPES, I>) -> Self {
        Left(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
/// Messages related to both validating and sequencing consensus.
pub enum GeneralConsensusMessage<TYPES: NodeType, I: NodeImplementation<TYPES>>
where
    I::Exchanges: ExchangesType<TYPES, Message<TYPES, I>>,
{
    /// Message with a quorum proposal.
    Proposal(Proposal<TYPES, QuorumProposalType<TYPES, I>>),

    /// Message with a quorum vote.
    Vote(QuorumVote<TYPES, QuorumMembership<TYPES, I>>),

    /// Message with a view sync vote.
    ViewSyncVote(ViewSyncVote<TYPES>),

    /// Message with a view sync certificate.
    ViewSyncCertificate(Proposal<TYPES, ViewSyncProposalType<TYPES, I>>),

    /// Message with a Timeout vote
    TimeoutVote(TimeoutVote2<TYPES, QuorumMembership<TYPES, I>>),

    /// Internal ONLY message indicating a view interrupt.
    #[serde(skip)]
    InternalTrigger(InternalTrigger<TYPES>),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = "", serialize = ""))]
/// Messages related to the sequencing consensus protocol for the DA committee.
pub enum CommitteeConsensusMessage<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Proposal for data availability committee
    DAProposal(Proposal<TYPES, DAProposal<TYPES>>),

    /// vote for data availability committee
    DAVote(DAVote2<TYPES, CommitteeMembership<TYPES, I>>),

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
    VidVote(VIDVote2<TYPES, VIDMembership<TYPES, I>>),
    /// VID certificate data is available
    ///
    /// Like [`DACertificate`]
    VidCertificate(VIDCertificate2<TYPES>),
}

/// Messages related to the consensus protocol.
pub trait ConsensusMessageType<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The type of messages for both validating and sequencing consensus.
    type GeneralConsensusMessage;

    /// The type of processed consensus messages.
    type ProcessedConsensusMessage: Send;

    /// Get the view number when the message was sent or the view of the timeout.
    fn view_number(&self) -> TYPES::Time;

    /// Get the message purpose.
    fn purpose(&self) -> MessagePurpose;
}

/// Messages related to the sequencing consensus protocol.
pub trait SequencingMessageType<TYPES: NodeType, I: NodeImplementation<TYPES>>:
    ConsensusMessageType<TYPES, I>
{
    /// Messages for DA committee only.
    type CommitteeConsensusMessage;
}

/// Messages for sequencing consensus.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = "", serialize = ""))]
pub struct SequencingMessage<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
>(pub Either<GeneralConsensusMessage<TYPES, I>, CommitteeConsensusMessage<TYPES, I>>);

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    > ConsensusMessageType<TYPES, I> for SequencingMessage<TYPES, I>
{
    type GeneralConsensusMessage = GeneralConsensusMessage<TYPES, I>;
    type ProcessedConsensusMessage = ProcessedSequencingMessage<TYPES, I>;

    // TODO: Disable panic after the `ViewSync` case is implemented.
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
                    GeneralConsensusMessage::ViewSyncVote(message) => message.round(),
                    GeneralConsensusMessage::ViewSyncCertificate(message) => {
                        message.data.get_view_number()
                    }
                    GeneralConsensusMessage::TimeoutVote(message) => message.get_view_number(),
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
    #[allow(clippy::panic)]
    fn purpose(&self) -> MessagePurpose {
        match &self.0 {
            Left(general_message) => match general_message {
                GeneralConsensusMessage::Proposal(_) => MessagePurpose::Proposal,
                GeneralConsensusMessage::Vote(_) | GeneralConsensusMessage::TimeoutVote(_) => {
                    MessagePurpose::Vote
                }
                GeneralConsensusMessage::InternalTrigger(_) => MessagePurpose::Internal,
                GeneralConsensusMessage::ViewSyncVote(_) => MessagePurpose::ViewSyncVote,
                GeneralConsensusMessage::ViewSyncCertificate(_) => MessagePurpose::ViewSyncProposal,
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

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    > SequencingMessageType<TYPES, I> for SequencingMessage<TYPES, I>
{
    type CommitteeConsensusMessage = CommitteeConsensusMessage<TYPES, I>;
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
pub struct Proposal<TYPES: NodeType, PROPOSAL: HasViewNumber<TYPES>> {
    // NOTE: optimization could include view number to help look up parent leaf
    // could even do 16 bit numbers if we want
    /// The data being proposed.
    pub data: PROPOSAL,
    /// The proposal must be signed by the view leader
    pub signature: EncodedSignature,
}
