use either::Either;
use hotshot_types::{
    consensus::ProposalDependencyData,
    data::{DAProposal, Leaf, QuorumProposal, UpgradeProposal, VidDisperse, VidDisperseShare},
    message::Proposal,
    simple_certificate::{
        DACertificate, QuorumCertificate, TimeoutCertificate, UpgradeCertificate,
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        DAVote, QuorumVote, TimeoutVote, UpgradeVote, ViewSyncCommitVote, ViewSyncFinalizeVote,
        ViewSyncPreCommitVote,
    },
    traits::{node_implementation::NodeType, BlockPayload},
    vid::VidCommitment,
    vote::VoteDependencyData,
};
use vbs::version::Version;

use crate::view_sync::ViewSyncPhase;

/// Marker that the task completed
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct HotShotTaskCompleted;

/// All of the possible events that can be passed between Sequecning `HotShot` tasks
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum HotShotEvent<TYPES: NodeType> {
    /// Shutdown the task
    Shutdown,
    /// A quorum proposal has been received from the network; handled by the consensus task
    QuorumProposalRecv(Proposal<TYPES, QuorumProposal<TYPES>>, TYPES::SignatureKey),
    /// A quorum vote has been received from the network; handled by the consensus task
    QuorumVoteRecv(QuorumVote<TYPES>),
    /// A timeout vote recevied from the network; handled by consensus task
    TimeoutVoteRecv(TimeoutVote<TYPES>),
    /// Send a timeout vote to the network; emitted by consensus task replicas
    TimeoutVoteSend(TimeoutVote<TYPES>),
    /// A DA proposal has been received from the network; handled by the DA task
    DAProposalRecv(Proposal<TYPES, DAProposal<TYPES>>, TYPES::SignatureKey),
    /// A DA proposal has been validated; handled by the DA task and VID task
    DAProposalValidated(Proposal<TYPES, DAProposal<TYPES>>, TYPES::SignatureKey),
    /// A DA vote has been received by the network; handled by the DA task
    DAVoteRecv(DAVote<TYPES>),
    /// A Data Availability Certificate (DAC) has been recieved by the network; handled by the consensus task
    DACertificateRecv(DACertificate<TYPES>),
    /// A DAC is validated.
    DACertificateValidated(DACertificate<TYPES>),
    /// Send a quorum proposal to the network; emitted by the leader in the consensus task
    QuorumProposalSend(Proposal<TYPES, QuorumProposal<TYPES>>, TYPES::SignatureKey),
    /// Send a quorum vote to the next leader; emitted by a replica in the consensus task after seeing a valid quorum proposal
    QuorumVoteSend(QuorumVote<TYPES>),
    /// All dependencies for the quorum vote are validated.
    QuorumVoteDependenciesValidated(TYPES::Time),
    /// A quorum proposal with the given parent leaf is validated.
    QuorumProposalValidated(QuorumProposal<TYPES>, Leaf<TYPES>),
    /// Send a DA proposal to the DA committee; emitted by the DA leader (which is the same node as the leader of view v + 1) in the DA task
    DAProposalSend(Proposal<TYPES, DAProposal<TYPES>>, TYPES::SignatureKey),
    /// Send a DA vote to the DA leader; emitted by DA committee members in the DA task after seeing a valid DA proposal
    DAVoteSend(DAVote<TYPES>),
    /// The next leader has collected enough votes to form a QC; emitted by the next leader in the consensus task; an internal event only
    QCFormed(Either<QuorumCertificate<TYPES>, TimeoutCertificate<TYPES>>),
    /// The DA leader has collected enough votes to form a DAC; emitted by the DA leader in the DA task; sent to the entire network via the networking task
    DACSend(DACertificate<TYPES>, TYPES::SignatureKey),
    /// The current view has changed; emitted by the replica in the consensus task or replica in the view sync task; received by almost all other tasks
    ViewChange(TYPES::Time),
    /// Timeout for the view sync protocol; emitted by a replica in the view sync task
    ViewSyncTimeout(TYPES::Time, u64, ViewSyncPhase),

    /// Receive a `ViewSyncPreCommitVote` from the network; received by a relay in the view sync task
    ViewSyncPreCommitVoteRecv(ViewSyncPreCommitVote<TYPES>),
    /// Receive a `ViewSyncCommitVote` from the network; received by a relay in the view sync task
    ViewSyncCommitVoteRecv(ViewSyncCommitVote<TYPES>),
    /// Receive a `ViewSyncFinalizeVote` from the network; received by a relay in the view sync task
    ViewSyncFinalizeVoteRecv(ViewSyncFinalizeVote<TYPES>),

    /// Send a `ViewSyncPreCommitVote` from the network; emitted by a replica in the view sync task
    ViewSyncPreCommitVoteSend(ViewSyncPreCommitVote<TYPES>),
    /// Send a `ViewSyncCommitVote` from the network; emitted by a replica in the view sync task
    ViewSyncCommitVoteSend(ViewSyncCommitVote<TYPES>),
    /// Send a `ViewSyncFinalizeVote` from the network; emitted by a replica in the view sync task
    ViewSyncFinalizeVoteSend(ViewSyncFinalizeVote<TYPES>),

    /// Receive a `ViewSyncPreCommitCertificate2` from the network; received by a replica in the view sync task
    ViewSyncPreCommitCertificate2Recv(ViewSyncPreCommitCertificate2<TYPES>),
    /// Receive a `ViewSyncCommitCertificate2` from the network; received by a replica in the view sync task
    ViewSyncCommitCertificate2Recv(ViewSyncCommitCertificate2<TYPES>),
    /// Receive a `ViewSyncFinalizeCertificate2` from the network; received by a replica in the view sync task
    ViewSyncFinalizeCertificate2Recv(ViewSyncFinalizeCertificate2<TYPES>),

    /// Send a `ViewSyncPreCommitCertificate2` from the network; emitted by a relay in the view sync task
    ViewSyncPreCommitCertificate2Send(ViewSyncPreCommitCertificate2<TYPES>, TYPES::SignatureKey),
    /// Send a `ViewSyncCommitCertificate2` from the network; emitted by a relay in the view sync task
    ViewSyncCommitCertificate2Send(ViewSyncCommitCertificate2<TYPES>, TYPES::SignatureKey),
    /// Send a `ViewSyncFinalizeCertificate2` from the network; emitted by a relay in the view sync task
    ViewSyncFinalizeCertificate2Send(ViewSyncFinalizeCertificate2<TYPES>, TYPES::SignatureKey),

    /// Trigger the start of the view sync protocol; emitted by view sync task; internal trigger only
    ViewSyncTrigger(TYPES::Time),
    /// A consensus view has timed out; emitted by a replica in the consensus task; received by the view sync task; internal event only
    Timeout(TYPES::Time),
    /// Receive transactions from the network
    TransactionsRecv(Vec<TYPES::Transaction>),
    /// Send transactions to the network
    TransactionSend(TYPES::Transaction, TYPES::SignatureKey),
    /// Event to send block payload commitment and metadata from DA leader to the quorum; internal event only
    SendPayloadCommitmentAndMetadata(
        VidCommitment,
        <TYPES::BlockPayload as BlockPayload>::Metadata,
        TYPES::Time,
    ),
    /// Event when the transactions task has sequenced transactions. Contains the encoded transactions, the metadata, and the view number
    BlockRecv(
        Vec<u8>,
        <TYPES::BlockPayload as BlockPayload>::Metadata,
        TYPES::Time,
    ),
    /// Event when the transactions task has a block formed
    BlockReady(VidDisperse<TYPES>, TYPES::Time),
    /// Event when consensus decided on a leaf
    LeafDecided(Vec<Leaf<TYPES>>),
    /// Send VID shares to VID storage nodes; emitted by the DA leader
    ///
    /// Like [`HotShotEvent::DAProposalSend`].
    VidDisperseSend(Proposal<TYPES, VidDisperse<TYPES>>, TYPES::SignatureKey),
    /// Vid disperse share has been received from the network; handled by the consensus task
    ///
    /// Like [`HotShotEvent::DAProposalRecv`].
    VIDShareRecv(Proposal<TYPES, VidDisperseShare<TYPES>>),
    /// VID share data is validated.
    VIDShareValidated(Proposal<TYPES, VidDisperseShare<TYPES>>),
    /// Upgrade proposal has been received from the network
    UpgradeProposalRecv(Proposal<TYPES, UpgradeProposal<TYPES>>, TYPES::SignatureKey),
    /// Upgrade proposal has been sent to the network
    UpgradeProposalSend(Proposal<TYPES, UpgradeProposal<TYPES>>, TYPES::SignatureKey),
    /// Upgrade vote has been received from the network
    UpgradeVoteRecv(UpgradeVote<TYPES>),
    /// Upgrade vote has been sent to the network
    UpgradeVoteSend(UpgradeVote<TYPES>),
    /// Upgrade certificate has been sent to the network
    UpgradeCertificateFormed(UpgradeCertificate<TYPES>),
    /// HotShot was upgraded, with a new network version.
    VersionUpgrade(Version),
    /// Initiate a proposal right now for a provided view.
    ProposeNow(TYPES::Time, ProposalDependencyData<TYPES>),
    /// Initiate a vote right now for the designated view.
    VoteNow(TYPES::Time, VoteDependencyData<TYPES>),
}
