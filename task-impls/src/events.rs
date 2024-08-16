// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::fmt::Display;

use async_broadcast::Sender;
use either::Either;
use hotshot_task::task::TaskEvent;
use hotshot_types::{
    data::{
        DaProposal, Leaf, PackedBundle, QuorumProposal, UpgradeProposal, VidDisperse,
        VidDisperseShare,
    },
    message::Proposal,
    simple_certificate::{
        DaCertificate, QuorumCertificate, TimeoutCertificate, UpgradeCertificate,
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        DaVote, QuorumVote, TimeoutVote, UpgradeVote, ViewSyncCommitVote, ViewSyncFinalizeVote,
        ViewSyncPreCommitVote,
    },
    traits::{block_contents::BuilderFee, node_implementation::NodeType, BlockPayload},
    utils::{BuilderCommitment, View},
    vid::VidCommitment,
    vote::HasViewNumber,
};
use vec1::Vec1;

use crate::view_sync::ViewSyncPhase;

impl<TYPES: NodeType> TaskEvent for HotShotEvent<TYPES> {
    fn shutdown_event() -> Self {
        HotShotEvent::Shutdown
    }
}

/// Wrapper type for the event to notify tasks that a proposal for a view is missing
/// and the channel to send the event back to
#[derive(Debug, Clone)]
pub struct ProposalMissing<TYPES: NodeType> {
    /// View of missing proposal
    pub view: TYPES::Time,
    /// Channel to send the response back to
    pub response_chan: Sender<Option<Proposal<TYPES, QuorumProposal<TYPES>>>>,
}

impl<TYPES: NodeType> PartialEq for ProposalMissing<TYPES> {
    fn eq(&self, other: &Self) -> bool {
        self.view == other.view
    }
}

impl<TYPES: NodeType> Eq for ProposalMissing<TYPES> {}

/// Marker that the task completed
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct HotShotTaskCompleted;

/// All of the possible events that can be passed between Sequencing `HotShot` tasks
#[derive(Eq, PartialEq, Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum HotShotEvent<TYPES: NodeType> {
    /// Shutdown the task
    Shutdown,
    /// A quorum proposal has been received from the network; handled by the consensus task
    QuorumProposalRecv(Proposal<TYPES, QuorumProposal<TYPES>>, TYPES::SignatureKey),
    /// A quorum vote has been received from the network; handled by the consensus task
    QuorumVoteRecv(QuorumVote<TYPES>),
    /// A timeout vote received from the network; handled by consensus task
    TimeoutVoteRecv(TimeoutVote<TYPES>),
    /// Send a timeout vote to the network; emitted by consensus task replicas
    TimeoutVoteSend(TimeoutVote<TYPES>),
    /// A DA proposal has been received from the network; handled by the DA task
    DaProposalRecv(Proposal<TYPES, DaProposal<TYPES>>, TYPES::SignatureKey),
    /// A DA proposal has been validated; handled by the DA task and VID task
    DaProposalValidated(Proposal<TYPES, DaProposal<TYPES>>, TYPES::SignatureKey),
    /// A DA vote has been received by the network; handled by the DA task
    DaVoteRecv(DaVote<TYPES>),
    /// A Data Availability Certificate (DAC) has been received by the network; handled by the consensus task
    DaCertificateRecv(DaCertificate<TYPES>),
    /// A DAC is validated.
    DaCertificateValidated(DaCertificate<TYPES>),
    /// Send a quorum proposal to the network; emitted by the leader in the consensus task
    QuorumProposalSend(Proposal<TYPES, QuorumProposal<TYPES>>, TYPES::SignatureKey),
    /// Send a quorum vote to the next leader; emitted by a replica in the consensus task after seeing a valid quorum proposal
    QuorumVoteSend(QuorumVote<TYPES>),
    /// All dependencies for the quorum vote are validated.
    QuorumVoteDependenciesValidated(TYPES::Time),
    /// A quorum proposal with the given parent leaf is validated.
    /// The full validation checks include:
    /// 1. The proposal is not for an old view
    /// 2. The proposal has been correctly signed by the leader of the current view
    /// 3. The justify QC is valid
    /// 4. The proposal passes either liveness or safety check.
    QuorumProposalValidated(QuorumProposal<TYPES>, Leaf<TYPES>),
    /// A quorum proposal is missing for a view that we need
    QuorumProposalRequest(ProposalMissing<TYPES>),
    /// Send a DA proposal to the DA committee; emitted by the DA leader (which is the same node as the leader of view v + 1) in the DA task
    DaProposalSend(Proposal<TYPES, DaProposal<TYPES>>, TYPES::SignatureKey),
    /// Send a DA vote to the DA leader; emitted by DA committee members in the DA task after seeing a valid DA proposal
    DaVoteSend(DaVote<TYPES>),
    /// The next leader has collected enough votes to form a QC; emitted by the next leader in the consensus task; an internal event only
    QcFormed(Either<QuorumCertificate<TYPES>, TimeoutCertificate<TYPES>>),
    /// The DA leader has collected enough votes to form a DAC; emitted by the DA leader in the DA task; sent to the entire network via the networking task
    DacSend(DaCertificate<TYPES>, TYPES::SignatureKey),
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
        BuilderCommitment,
        <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
        TYPES::Time,
        Vec1<BuilderFee<TYPES>>,
        Option<TYPES::AuctionResult>,
    ),
    /// Event when the transactions task has sequenced transactions. Contains the encoded transactions, the metadata, and the view number
    BlockRecv(PackedBundle<TYPES>),
    /// Event when the transactions task has a block formed
    BlockReady(VidDisperse<TYPES>, TYPES::Time),
    /// Event when consensus decided on a leaf
    LeafDecided(Vec<Leaf<TYPES>>),
    /// Send VID shares to VID storage nodes; emitted by the DA leader
    ///
    /// Like [`HotShotEvent::DaProposalSend`].
    VidDisperseSend(Proposal<TYPES, VidDisperse<TYPES>>, TYPES::SignatureKey),
    /// Vid disperse share has been received from the network; handled by the consensus task
    ///
    /// Like [`HotShotEvent::DaProposalRecv`].
    VidShareRecv(Proposal<TYPES, VidDisperseShare<TYPES>>),
    /// VID share data is validated.
    VidShareValidated(Proposal<TYPES, VidDisperseShare<TYPES>>),
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

    /* Consensus State Update Events */
    /// A undecided view has been created and added to the validated state storage.
    ValidatedStateUpdated(TYPES::Time, View<TYPES>),

    /// A new locked view has been created (2-chain)
    LockedViewUpdated(TYPES::Time),

    /// A new anchor view has been successfully reached by this node (3-chain).
    LastDecidedViewUpdated(TYPES::Time),

    /// A new high_qc has been reached by this node.
    UpdateHighQc(QuorumCertificate<TYPES>),

    /// A new high_qc has been updated in `Consensus`.
    HighQcUpdated(QuorumCertificate<TYPES>),

    /// A quorum proposal has been preliminarily validated.
    /// The preliminary checks include:
    /// 1. The proposal is not for an old view
    /// 2. The proposal has been correctly signed by the leader of the current view
    /// 3. The justify QC is valid
    QuorumProposalPreliminarilyValidated(Proposal<TYPES, QuorumProposal<TYPES>>),
}

impl<TYPES: NodeType> Display for HotShotEvent<TYPES> {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotShotEvent::Shutdown => write!(f, "Shutdown"),
            HotShotEvent::QuorumProposalRecv(proposal, _) => write!(
                f,
                "QuorumProposalRecv(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::QuorumVoteRecv(v) => {
                write!(f, "QuorumVoteRecv(view_number={:?})", v.view_number())
            }
            HotShotEvent::TimeoutVoteRecv(v) => {
                write!(f, "TimeoutVoteRecv(view_number={:?})", v.view_number())
            }
            HotShotEvent::TimeoutVoteSend(v) => {
                write!(f, "TimeoutVoteSend(view_number={:?})", v.view_number())
            }
            HotShotEvent::DaProposalRecv(proposal, _) => write!(
                f,
                "DaProposalRecv(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::DaProposalValidated(proposal, _) => write!(
                f,
                "DaProposalValidated(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::DaVoteRecv(vote) => {
                write!(f, "DaVoteRecv(view_number={:?})", vote.view_number())
            }
            HotShotEvent::DaCertificateRecv(cert) => {
                write!(f, "DaCertificateRecv(view_number={:?})", cert.view_number())
            }
            HotShotEvent::DaCertificateValidated(cert) => write!(
                f,
                "DaCertificateValidated(view_number={:?})",
                cert.view_number()
            ),
            HotShotEvent::QuorumProposalSend(proposal, _) => write!(
                f,
                "QuorumProposalSend(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::QuorumVoteSend(vote) => {
                write!(f, "QuorumVoteSend(view_number={:?})", vote.view_number())
            }
            HotShotEvent::QuorumVoteDependenciesValidated(view_number) => {
                write!(
                    f,
                    "QuorumVoteDependenciesValidated(view_number={view_number:?})"
                )
            }
            HotShotEvent::QuorumProposalValidated(proposal, _) => write!(
                f,
                "QuorumProposalValidated(view_number={:?})",
                proposal.view_number()
            ),
            HotShotEvent::DaProposalSend(proposal, _) => write!(
                f,
                "DaProposalSend(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::DaVoteSend(vote) => {
                write!(f, "DaVoteSend(view_number={:?})", vote.view_number())
            }
            HotShotEvent::QcFormed(cert) => match cert {
                either::Left(qc) => write!(f, "QcFormed(view_number={:?})", qc.view_number()),
                either::Right(tc) => write!(f, "QcFormed(view_number={:?})", tc.view_number()),
            },
            HotShotEvent::DacSend(cert, _) => {
                write!(f, "DacSend(view_number={:?})", cert.view_number())
            }
            HotShotEvent::ViewChange(view_number) => {
                write!(f, "ViewChange(view_number={view_number:?})")
            }
            HotShotEvent::ViewSyncTimeout(view_number, _, _) => {
                write!(f, "ViewSyncTimeout(view_number={view_number:?})")
            }
            HotShotEvent::ViewSyncPreCommitVoteRecv(vote) => write!(
                f,
                "ViewSyncPreCommitVoteRecv(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncCommitVoteRecv(vote) => write!(
                f,
                "ViewSyncCommitVoteRecv(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncFinalizeVoteRecv(vote) => write!(
                f,
                "ViewSyncFinalizeVoteRecv(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncPreCommitVoteSend(vote) => write!(
                f,
                "ViewSyncPreCommitVoteSend(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncCommitVoteSend(vote) => write!(
                f,
                "ViewSyncCommitVoteSend(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncFinalizeVoteSend(vote) => write!(
                f,
                "ViewSyncFinalizeVoteSend(view_nuber={:?})",
                vote.view_number()
            ),
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(cert) => {
                write!(
                    f,
                    "ViewSyncPreCommitCertificate2Recv(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncCommitCertificate2Recv(cert) => {
                write!(
                    f,
                    "ViewSyncCommitCertificate2Recv(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncFinalizeCertificate2Recv(cert) => {
                write!(
                    f,
                    "ViewSyncFinalizeCertificate2Recv(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncPreCommitCertificate2Send(cert, _) => {
                write!(
                    f,
                    "ViewSyncPreCommitCertificate2Send(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncCommitCertificate2Send(cert, _) => {
                write!(
                    f,
                    "ViewSyncCommitCertificate2Send(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncFinalizeCertificate2Send(cert, _) => {
                write!(
                    f,
                    "ViewSyncFinalizeCertificate2Send(view_number={:?})",
                    cert.view_number()
                )
            }
            HotShotEvent::ViewSyncTrigger(view_number) => {
                write!(f, "ViewSyncTrigger(view_number={view_number:?})")
            }
            HotShotEvent::Timeout(view_number) => write!(f, "Timeout(view_number={view_number:?})"),
            HotShotEvent::TransactionsRecv(_) => write!(f, "TransactionsRecv"),
            HotShotEvent::TransactionSend(_, _) => write!(f, "TransactionSend"),
            HotShotEvent::SendPayloadCommitmentAndMetadata(_, _, _, view_number, _, _) => {
                write!(
                    f,
                    "SendPayloadCommitmentAndMetadata(view_number={view_number:?})"
                )
            }
            HotShotEvent::BlockRecv(packed_bundle) => {
                write!(f, "BlockRecv(view_number={:?})", packed_bundle.view_number)
            }
            HotShotEvent::BlockReady(_, view_number) => {
                write!(f, "BlockReady(view_number={view_number:?})")
            }
            HotShotEvent::LeafDecided(leaves) => {
                let view_numbers: Vec<<TYPES as NodeType>::Time> =
                    leaves.iter().map(Leaf::view_number).collect();
                write!(f, "LeafDecided({view_numbers:?})")
            }
            HotShotEvent::VidDisperseSend(proposal, _) => write!(
                f,
                "VidDisperseSend(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::VidShareRecv(proposal) => write!(
                f,
                "VIDShareRecv(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::VidShareValidated(proposal) => write!(
                f,
                "VIDShareValidated(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::UpgradeProposalRecv(proposal, _) => write!(
                f,
                "UpgradeProposalRecv(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::UpgradeProposalSend(proposal, _) => write!(
                f,
                "UpgradeProposalSend(view_number={:?})",
                proposal.data.view_number()
            ),
            HotShotEvent::UpgradeVoteRecv(vote) => {
                write!(f, "UpgradeVoteRecv(view_number={:?})", vote.view_number())
            }
            HotShotEvent::UpgradeVoteSend(vote) => {
                write!(f, "UpgradeVoteSend(view_number={:?})", vote.view_number())
            }
            HotShotEvent::UpgradeCertificateFormed(cert) => write!(
                f,
                "UpgradeCertificateFormed(view_number={:?})",
                cert.view_number()
            ),
            HotShotEvent::QuorumProposalRequest(view_number) => {
                write!(f, "QuorumProposalRequest(view_number={view_number:?})")
            }
            HotShotEvent::ValidatedStateUpdated(view_number, _) => {
                write!(f, "ValidatedStateUpdated(view_number={view_number:?})")
            }
            HotShotEvent::LockedViewUpdated(view_number) => {
                write!(f, "LockedViewUpdated(view_number={view_number:?})")
            }
            HotShotEvent::LastDecidedViewUpdated(view_number) => {
                write!(f, "LastDecidedViewUpdated(view_number={view_number:?})")
            }
            HotShotEvent::UpdateHighQc(cert) => {
                write!(f, "UpdateHighQc(view_number={:?})", cert.view_number())
            }
            HotShotEvent::HighQcUpdated(cert) => {
                write!(f, "HighQcUpdated(view_number={:?})", cert.view_number())
            }
            HotShotEvent::QuorumProposalPreliminarilyValidated(proposal) => {
                write!(
                    f,
                    "QuorumProposalPreliminarilyValidated(view_number={:?}",
                    proposal.data.view_number()
                )
            }
        }
    }
}
