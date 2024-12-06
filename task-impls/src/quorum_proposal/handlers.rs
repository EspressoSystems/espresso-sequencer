// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! This module holds the dependency task for the QuorumProposalTask. It is spawned whenever an event that could
//! initiate a proposal occurs.

use std::{
    marker::PhantomData,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{ensure, Context, Result};
use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
use committable::Committable;
use hotshot_task::dependency_task::HandleDepOutput;
use hotshot_types::{
    consensus::{CommitmentAndMetadata, OuterConsensus},
    data::{Leaf2, QuorumProposal2, VidDisperse, ViewChangeEvidence},
    drb::{INITIAL_DRB_RESULT, INITIAL_DRB_SEED_INPUT},
    message::Proposal,
    simple_certificate::{QuorumCertificate2, UpgradeCertificate},
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
    },
    vote::{Certificate, HasViewNumber},
};
use tracing::instrument;
use utils::anytrace::*;
use vbs::version::StaticVersionType;

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, parent_leaf_and_state},
    quorum_proposal::{UpgradeLock, Versions},
};

/// Proposal dependency types. These types represent events that precipitate a proposal.
#[derive(PartialEq, Debug)]
pub(crate) enum ProposalDependency {
    /// For the `SendPayloadCommitmentAndMetadata` event.
    PayloadAndMetadata,

    /// For the `Qc2Formed` event.
    Qc,

    /// For the `ViewSyncFinalizeCertificateRecv` event.
    ViewSyncCert,

    /// For the `Qc2Formed` event timeout branch.
    TimeoutCert,

    /// For the `QuorumProposalRecv` event.
    Proposal,

    /// For the `VidShareValidated` event.
    VidShare,
}

/// Handler for the proposal dependency
pub struct ProposalDependencyHandle<TYPES: NodeType, V: Versions> {
    /// Latest view number that has been proposed for (proxy for cur_view).
    pub latest_proposed_view: TYPES::View,

    /// The view number to propose for.
    pub view_number: TYPES::View,

    /// The event sender.
    pub sender: Sender<Arc<HotShotEvent<TYPES>>>,

    /// The event receiver.
    pub receiver: Receiver<Arc<HotShotEvent<TYPES>>>,

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Our public key
    pub public_key: TYPES::SignatureKey,

    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Shared consensus task state
    pub consensus: OuterConsensus<TYPES>,

    /// View timeout from config.
    pub timeout: u64,
    /// The most recent upgrade certificate this node formed.
    /// Note: this is ONLY for certificates that have been formed internally,
    /// so that we can propose with them.
    ///
    /// Certificates received from other nodes will get reattached regardless of this fields,
    /// since they will be present in the leaf we propose off of.
    pub formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// Lock for a decided upgrade
    pub upgrade_lock: UpgradeLock<TYPES, V>,

    /// The node's id
    pub id: u64,

    /// The time this view started
    pub view_start_time: Instant,

    /// The highest_qc we've seen at the start of this task
    pub highest_qc: QuorumCertificate2<TYPES>,
}

impl<TYPES: NodeType, V: Versions> ProposalDependencyHandle<TYPES, V> {
    /// Return the next HighQc we get from the event stream
    async fn wait_for_qc_event(
        &self,
        rx: &mut Receiver<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<QuorumCertificate2<TYPES>> {
        while let Ok(event) = rx.recv_direct().await {
            if let HotShotEvent::HighQcRecv(qc, _sender) = event.as_ref() {
                if qc
                    .is_valid_cert(
                        // TODO take epoch from `qc`
                        // https://github.com/EspressoSystems/HotShot/issues/3917
                        self.quorum_membership.stake_table(TYPES::Epoch::new(0)),
                        self.quorum_membership
                            .success_threshold(TYPES::Epoch::new(0)),
                        &self.upgrade_lock,
                    )
                    .await
                {
                    return Some(qc.clone());
                }
            }
        }
        None
    }
    /// Waits for the ocnfigured timeout for nodes to send HighQc messages to us.  We'll
    /// then propose with the highest QC from among these proposals.
    async fn wait_for_highest_qc(&mut self) {
        tracing::error!("waiting for QC");
        // If we haven't upgraded to Hotstuff 2 just return the high qc right away
        if self
            .upgrade_lock
            .version(self.view_number)
            .await
            .is_ok_and(|version| version < V::Epochs::VERSION)
        {
            return;
        }
        let wait_duration = Duration::from_millis(self.timeout / 2);

        // TODO configure timeout
        while self.view_start_time.elapsed() < wait_duration {
            let Some(time_spent) = Instant::now().checked_duration_since(self.view_start_time)
            else {
                // Shouldn't be possible, now must be after the start
                return;
            };
            let Some(time_left) = wait_duration.checked_sub(time_spent) else {
                // No time left
                return;
            };
            let Ok(maybe_qc) = tokio::time::timeout(
                time_left,
                self.wait_for_qc_event(&mut self.receiver.clone()),
            )
            .await
            else {
                // we timeout out, don't wait any longer
                return;
            };
            let Some(qc) = maybe_qc else {
                continue;
            };
            if qc.view_number() > self.highest_qc.view_number() {
                self.highest_qc = qc;
            }
        }
    }
    /// Publishes a proposal given the [`CommitmentAndMetadata`], [`VidDisperse`]
    /// and high qc [`hotshot_types::simple_certificate::QuorumCertificate`],
    /// with optional [`ViewChangeEvidence`].
    #[instrument(skip_all, fields(id = self.id, view_number = *self.view_number, latest_proposed_view = *self.latest_proposed_view))]
    async fn publish_proposal(
        &self,
        commitment_and_metadata: CommitmentAndMetadata<TYPES>,
        vid_share: Proposal<TYPES, VidDisperse<TYPES>>,
        view_change_evidence: Option<ViewChangeEvidence<TYPES>>,
        formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
        decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
        parent_qc: QuorumCertificate2<TYPES>,
    ) -> Result<()> {
        let (parent_leaf, state) = parent_leaf_and_state(
            self.view_number,
            &self.sender,
            &self.receiver,
            Arc::clone(&self.quorum_membership),
            self.public_key.clone(),
            self.private_key.clone(),
            OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus)),
            &self.upgrade_lock,
            parent_qc.view_number(),
        )
        .await?;

        // In order of priority, we should try to attach:
        //   - the parent certificate if it exists, or
        //   - our own certificate that we formed.
        // In either case, we need to ensure that the certificate is still relevant.
        //
        // Note: once we reach a point of potentially propose with our formed upgrade certificate,
        // we will ALWAYS drop it. If we cannot immediately use it for whatever reason, we choose
        // to discard it.
        //
        // It is possible that multiple nodes form separate upgrade certificates for the some
        // upgrade if we are not careful about voting. But this shouldn't bother us: the first
        // leader to propose is the one whose certificate will be used. And if that fails to reach
        // a decide for whatever reason, we may lose our own certificate, but something will likely
        // have gone wrong there anyway.
        let mut upgrade_certificate = parent_leaf
            .upgrade_certificate()
            .or(formed_upgrade_certificate);

        if let Some(cert) = upgrade_certificate.clone() {
            if cert
                .is_relevant(self.view_number, Arc::clone(&decided_upgrade_certificate))
                .await
                .is_err()
            {
                upgrade_certificate = None;
            }
        }

        let proposal_certificate = view_change_evidence
            .as_ref()
            .filter(|cert| cert.is_valid_for_view(&self.view_number))
            .cloned();

        ensure!(
            commitment_and_metadata.block_view == self.view_number,
            "Cannot propose because our VID payload commitment and metadata is for an older view."
        );

        let version = self.upgrade_lock.version(self.view_number).await?;

        let builder_commitment = commitment_and_metadata.builder_commitment.clone();
        let metadata = commitment_and_metadata.metadata.clone();

        let block_header = if version >= V::Epochs::VERSION
            && self.consensus.read().await.is_qc_forming_eqc(&parent_qc)
        {
            tracing::info!("Reached end of epoch. Proposing the same block again to form an eQC.");
            let block_header = parent_leaf.block_header().clone();
            tracing::debug!(
                "Proposing block no. {} to form the eQC.",
                block_header.block_number()
            );
            block_header
        } else if version < V::Marketplace::VERSION {
            TYPES::BlockHeader::new_legacy(
                state.as_ref(),
                self.instance_state.as_ref(),
                &parent_leaf,
                commitment_and_metadata.commitment,
                builder_commitment,
                metadata,
                commitment_and_metadata.fees.first().clone(),
                vid_share.data.common.clone(),
                version,
            )
            .await
            .wrap()
            .context(warn!("Failed to construct legacy block header"))?
        } else {
            TYPES::BlockHeader::new_marketplace(
                state.as_ref(),
                self.instance_state.as_ref(),
                &parent_leaf,
                commitment_and_metadata.commitment,
                commitment_and_metadata.builder_commitment,
                commitment_and_metadata.metadata,
                commitment_and_metadata.fees.to_vec(),
                *self.view_number,
                vid_share.data.common.clone(),
                commitment_and_metadata.auction_result,
                version,
            )
            .await
            .wrap()
            .context(warn!("Failed to construct marketplace block header"))?
        };

        let proposal = QuorumProposal2 {
            block_header,
            view_number: self.view_number,
            justify_qc: parent_qc,
            upgrade_certificate,
            view_change_evidence: proposal_certificate,
            drb_seed: INITIAL_DRB_SEED_INPUT,
            drb_result: INITIAL_DRB_RESULT,
        };

        let proposed_leaf = Leaf2::from_quorum_proposal(&proposal);
        ensure!(
            proposed_leaf.parent_commitment() == parent_leaf.commit(),
            "Proposed leaf parent does not equal high qc"
        );

        let signature =
            TYPES::SignatureKey::sign(&self.private_key, proposed_leaf.commit().as_ref())
                .wrap()
                .context(error!("Failed to compute proposed_leaf.commit()"))?;

        let message = Proposal {
            data: proposal,
            signature,
            _pd: PhantomData,
        };
        tracing::debug!(
            "Sending proposal for view {:?}",
            proposed_leaf.view_number(),
        );

        broadcast_event(
            Arc::new(HotShotEvent::QuorumProposalSend(
                message.clone(),
                self.public_key.clone(),
            )),
            &self.sender,
        )
        .await;

        Ok(())
    }
}

impl<TYPES: NodeType, V: Versions> HandleDepOutput for ProposalDependencyHandle<TYPES, V> {
    type Output = Vec<Vec<Vec<Arc<HotShotEvent<TYPES>>>>>;

    #[allow(clippy::no_effect_underscore_binding, clippy::too_many_lines)]
    async fn handle_dep_result(mut self, res: Self::Output) {
        let mut commit_and_metadata: Option<CommitmentAndMetadata<TYPES>> = None;
        let mut timeout_certificate = None;
        let mut view_sync_finalize_cert = None;
        let mut vid_share = None;
        let mut parent_qc = None;
        for event in res.iter().flatten().flatten() {
            match event.as_ref() {
                HotShotEvent::SendPayloadCommitmentAndMetadata(
                    payload_commitment,
                    builder_commitment,
                    metadata,
                    view,
                    fees,
                    auction_result,
                ) => {
                    commit_and_metadata = Some(CommitmentAndMetadata {
                        commitment: *payload_commitment,
                        builder_commitment: builder_commitment.clone(),
                        metadata: metadata.clone(),
                        fees: fees.clone(),
                        block_view: *view,
                        auction_result: auction_result.clone(),
                    });
                }
                HotShotEvent::Qc2Formed(cert) => match cert {
                    either::Right(timeout) => {
                        timeout_certificate = Some(timeout.clone());
                    }
                    either::Left(qc) => {
                        parent_qc = Some(qc.clone());
                    }
                },
                HotShotEvent::ViewSyncFinalizeCertificateRecv(cert) => {
                    view_sync_finalize_cert = Some(cert.clone());
                }
                HotShotEvent::VidDisperseSend(share, _) => {
                    vid_share = Some(share.clone());
                }
                _ => {}
            }
        }

        let Ok(version) = self.upgrade_lock.version(self.view_number).await else {
            tracing::error!(
                "Failed to get version for view {:?}, not proposing",
                self.view_number
            );
            return;
        };
        let parent_qc = if let Some(qc) = parent_qc {
            qc
        } else if version < V::Epochs::VERSION {
            self.consensus.read().await.high_qc().clone()
        } else {
            self.wait_for_highest_qc().await;
            self.highest_qc.clone()
        };

        if commit_and_metadata.is_none() {
            tracing::error!(
                "Somehow completed the proposal dependency task without a commitment and metadata"
            );
            return;
        }

        if vid_share.is_none() {
            tracing::error!("Somehow completed the proposal dependency task without a VID share");
            return;
        }

        let proposal_cert = if let Some(view_sync_cert) = view_sync_finalize_cert {
            Some(ViewChangeEvidence::ViewSync(view_sync_cert))
        } else {
            timeout_certificate.map(ViewChangeEvidence::Timeout)
        };

        if let Err(e) = self
            .publish_proposal(
                commit_and_metadata.unwrap(),
                vid_share.unwrap(),
                proposal_cert,
                self.formed_upgrade_certificate.clone(),
                Arc::clone(&self.upgrade_lock.decided_upgrade_certificate),
                parent_qc,
            )
            .await
        {
            tracing::error!("Failed to publish proposal; error = {e:#}");
        }
    }
}
