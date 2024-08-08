// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! This module holds the dependency task for the QuorumProposalTask. It is spawned whenever an event that could
//! initiate a proposal occurs.

use std::{marker::PhantomData, sync::Arc, time::Duration};

use anyhow::{ensure, Context, Result};
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use committable::Committable;
use hotshot_task::{
    dependency::{Dependency, EventDependency},
    dependency_task::HandleDepOutput,
};
use hotshot_types::{
    consensus::{CommitmentAndMetadata, OuterConsensus},
    constants::MarketplaceVersion,
    data::{Leaf, QuorumProposal, VidDisperse, ViewChangeEvidence},
    message::Proposal,
    simple_certificate::{version, UpgradeCertificate},
    traits::{
        block_contents::BlockHeader, node_implementation::NodeType, signature_key::SignatureKey,
    },
};
use tracing::{debug, error, instrument};
use vbs::version::StaticVersionType;

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, fetch_proposal, parent_leaf_and_state},
};

/// Proposal dependency types. These types represent events that precipitate a proposal.
#[derive(PartialEq, Debug)]
pub(crate) enum ProposalDependency {
    /// For the `SendPayloadCommitmentAndMetadata` event.
    PayloadAndMetadata,

    /// For the `QcFormed` event.
    Qc,

    /// For the `ViewSyncFinalizeCertificate2Recv` event.
    ViewSyncCert,

    /// For the `QcFormed` event timeout branch.
    TimeoutCert,

    /// For the `QuroumProposalRecv` event.
    Proposal,

    /// For the `VidShareValidated` event.
    VidShare,
}

/// Handler for the proposal dependency
pub struct ProposalDependencyHandle<TYPES: NodeType> {
    /// Latest view number that has been proposed for (proxy for cur_view).
    pub latest_proposed_view: TYPES::Time,

    /// The view number to propose for.
    pub view_number: TYPES::Time,

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

    /// Round start delay from config, in milliseconds.
    pub round_start_delay: u64,

    /// Shared consensus task state
    pub consensus: OuterConsensus<TYPES>,

    /// The most recent upgrade certificate this node formed.
    /// Note: this is ONLY for certificates that have been formed internally,
    /// so that we can propose with them.
    ///
    /// Certificates received from other nodes will get reattached regardless of this fields,
    /// since they will be present in the leaf we propose off of.
    pub formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// An upgrade certificate that has been decided on, if any.
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,

    /// The node's id
    pub id: u64,
}

impl<TYPES: NodeType> ProposalDependencyHandle<TYPES> {
    /// Publishes a proposal given the [`CommitmentAndMetadata`], [`VidDisperse`]
    /// and high qc [`hotshot_types::simple_certificate::QuorumCertificate`],
    /// with optional [`ViewChangeEvidence`].
    #[instrument(skip_all, target = "ProposalDependencyHandle", fields(id = self.id, view_number = *self.view_number, latest_proposed_view = *self.latest_proposed_view))]
    async fn publish_proposal(
        &self,
        commitment_and_metadata: CommitmentAndMetadata<TYPES>,
        vid_share: Proposal<TYPES, VidDisperse<TYPES>>,
        view_change_evidence: Option<ViewChangeEvidence<TYPES>>,
        formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
        decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    ) -> Result<()> {
        let (parent_leaf, state) = parent_leaf_and_state(
            self.view_number,
            &self.sender,
            Arc::clone(&self.quorum_membership),
            self.public_key.clone(),
            OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus)),
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

        let version = version(
            self.view_number,
            &self.decided_upgrade_certificate.read().await.clone(),
        )?;

        let block_header = if version < MarketplaceVersion::VERSION {
            TYPES::BlockHeader::new_legacy(
                state.as_ref(),
                self.instance_state.as_ref(),
                &parent_leaf,
                commitment_and_metadata.commitment,
                commitment_and_metadata.builder_commitment,
                commitment_and_metadata.metadata,
                commitment_and_metadata.fees.first().clone(),
                vid_share.data.common.clone(),
                version,
            )
            .await
            .context("Failed to construct legacy block header")?
        } else {
            TYPES::BlockHeader::new_marketplace(
                state.as_ref(),
                self.instance_state.as_ref(),
                &parent_leaf,
                commitment_and_metadata.commitment,
                commitment_and_metadata.builder_commitment,
                commitment_and_metadata.metadata,
                commitment_and_metadata.fees.to_vec(),
                vid_share.data.common.clone(),
                commitment_and_metadata.auction_result,
                version,
            )
            .await
            .context("Failed to construct marketplace block header")?
        };

        let proposal = QuorumProposal {
            block_header,
            view_number: self.view_number,
            justify_qc: self.consensus.read().await.high_qc().clone(),
            upgrade_certificate,
            proposal_certificate,
        };

        let proposed_leaf = Leaf::from_quorum_proposal(&proposal);
        ensure!(
            proposed_leaf.parent_commitment() == parent_leaf.commit(),
            "Proposed leaf parent does not equal high qc"
        );

        let signature =
            TYPES::SignatureKey::sign(&self.private_key, proposed_leaf.commit().as_ref())
                .context("Failed to compute proposed_leaf.commit()")?;

        let message = Proposal {
            data: proposal,
            signature,
            _pd: PhantomData,
        };
        debug!(
            "Sending proposal for view {:?}",
            proposed_leaf.view_number(),
        );

        self.consensus
            .write()
            .await
            .update_last_proposed_view(message.clone())?;
        async_sleep(Duration::from_millis(self.round_start_delay)).await;
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
impl<TYPES: NodeType> HandleDepOutput for ProposalDependencyHandle<TYPES> {
    type Output = Vec<Vec<Vec<Arc<HotShotEvent<TYPES>>>>>;

    #[allow(clippy::no_effect_underscore_binding)]
    async fn handle_dep_result(self, res: Self::Output) {
        let high_qc_view_number = self.consensus.read().await.high_qc().view_number;
        if !self
            .consensus
            .read()
            .await
            .validated_state_map()
            .contains_key(&high_qc_view_number)
        {
            // The proposal for the high qc view is missing, try to get it asynchronously
            let membership = Arc::clone(&self.quorum_membership);
            let sender = self.sender.clone();
            let consensus = OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus));
            async_spawn(async move {
                fetch_proposal(high_qc_view_number, sender, membership, consensus).await
            });
            // Block on receiving the event from the event stream.
            EventDependency::new(
                self.receiver.clone(),
                Box::new(move |event| {
                    let event = event.as_ref();
                    if let HotShotEvent::ValidatedStateUpdated(view_number, _) = event {
                        *view_number == high_qc_view_number
                    } else {
                        false
                    }
                }),
            )
            .completed()
            .await;
        }

        let mut commit_and_metadata: Option<CommitmentAndMetadata<TYPES>> = None;
        let mut timeout_certificate = None;
        let mut view_sync_finalize_cert = None;
        let mut vid_share = None;
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
                HotShotEvent::QcFormed(cert) => match cert {
                    either::Right(timeout) => {
                        timeout_certificate = Some(timeout.clone());
                    }
                    either::Left(_) => {
                        // Handled by the UpdateHighQc event.
                    }
                },
                HotShotEvent::ViewSyncFinalizeCertificate2Recv(cert) => {
                    view_sync_finalize_cert = Some(cert.clone());
                }
                HotShotEvent::VidDisperseSend(share, _) => {
                    vid_share = Some(share.clone());
                }
                _ => {}
            }
        }

        if commit_and_metadata.is_none() {
            error!(
                "Somehow completed the proposal dependency task without a commitment and metadata"
            );
            return;
        }

        if vid_share.is_none() {
            error!("Somehow completed the proposal dependency task without a VID share");
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
                Arc::clone(&self.decided_upgrade_certificate),
            )
            .await
        {
            error!("Failed to publish proposal; error = {e}");
        }
    }
}
