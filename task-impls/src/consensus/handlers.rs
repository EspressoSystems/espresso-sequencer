// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use core::time::Duration;
use std::{marker::PhantomData, sync::Arc};

use anyhow::{bail, ensure, Context, Result};
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use chrono::Utc;
use committable::Committable;
use futures::FutureExt;
use hotshot_types::{
    consensus::{CommitmentAndMetadata, OuterConsensus, View},
    data::{null_block, Leaf, QuorumProposal, ViewChangeEvidence},
    event::{Event, EventType},
    message::{GeneralConsensusMessage, Proposal},
    simple_certificate::UpgradeCertificate,
    simple_vote::QuorumData,
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        states::ValidatedState,
        storage::Storage,
    },
    utils::ViewInner,
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, warn};
use vbs::version::{StaticVersionType, Version};

use super::ConsensusTaskState;
use crate::{
    consensus::{UpgradeLock, Versions},
    events::HotShotEvent,
    helpers::{
        broadcast_event, decide_from_proposal, fetch_proposal, parent_leaf_and_state, update_view,
        validate_proposal_safety_and_liveness, validate_proposal_view_and_certs, AnyhowTracing,
        SEND_VIEW_CHANGE_EVENT,
    },
};

/// Create the header for a proposal, build the proposal, and broadcast
/// the proposal send evnet.
#[allow(clippy::too_many_arguments)]
#[instrument(skip_all, fields(id = id, view = *view))]
pub async fn create_and_send_proposal<TYPES: NodeType, V: Versions>(
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: OuterConsensus<TYPES>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    view: TYPES::Time,
    commitment_and_metadata: CommitmentAndMetadata<TYPES>,
    parent_leaf: Leaf<TYPES>,
    state: Arc<TYPES::ValidatedState>,
    upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    round_start_delay: u64,
    instance_state: Arc<TYPES::InstanceState>,
    version: Version,
    id: u64,
) -> Result<()> {
    let consensus_read = consensus.read().await;
    let vid_share = consensus_read
        .vid_shares()
        .get(&view)
        .map(|shares| shares.get(&public_key).cloned())
        .context(format!(
            "Cannot propopse without our VID share, view {view:?}"
        ))?
        .context("Failed to get vid share")?;
    drop(consensus_read);

    let block_header = if version < V::Marketplace::VERSION {
        TYPES::BlockHeader::new_legacy(
            state.as_ref(),
            instance_state.as_ref(),
            &parent_leaf,
            commitment_and_metadata.commitment,
            commitment_and_metadata.builder_commitment,
            commitment_and_metadata.metadata,
            commitment_and_metadata.fees.first().clone(),
            vid_share.data.common,
            version,
        )
        .await
        .context("Failed to construct legacy block header")?
    } else {
        TYPES::BlockHeader::new_marketplace(
            state.as_ref(),
            instance_state.as_ref(),
            &parent_leaf,
            commitment_and_metadata.commitment,
            commitment_and_metadata.builder_commitment,
            commitment_and_metadata.metadata,
            commitment_and_metadata.fees.to_vec(),
            vid_share.data.common,
            commitment_and_metadata.auction_result,
            version,
        )
        .await
        .context("Failed to construct marketplace block header")?
    };

    let proposal = QuorumProposal {
        block_header,
        view_number: view,
        justify_qc: consensus.read().await.high_qc().clone(),
        proposal_certificate: proposal_cert,
        upgrade_certificate: upgrade_cert,
    };

    let proposed_leaf = Leaf::from_quorum_proposal(&proposal);

    ensure!(proposed_leaf.parent_commitment() == parent_leaf.commit());

    let signature = TYPES::SignatureKey::sign(&private_key, proposed_leaf.commit().as_ref())?;

    let message = Proposal {
        data: proposal,
        signature,
        _pd: PhantomData,
    };

    debug!(
        "Sending proposal for view {:?}",
        proposed_leaf.view_number(),
    );

    consensus
        .write()
        .await
        .update_last_proposed_view(message.clone())?;

    async_sleep(Duration::from_millis(round_start_delay)).await;

    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalSend(
            message.clone(),
            public_key,
        )),
        &event_stream,
    )
    .await;

    Ok(())
}

/// Send a proposal for the view `view` from the latest high_qc given an upgrade cert. This is the
/// standard case proposal scenario.
#[allow(clippy::too_many_arguments)]
#[instrument(skip_all)]
pub async fn publish_proposal_from_commitment_and_metadata<TYPES: NodeType, V: Versions>(
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: OuterConsensus<TYPES>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    upgrade_lock: UpgradeLock<TYPES, V>,
    commitment_and_metadata: Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
    id: u64,
) -> Result<JoinHandle<()>> {
    let (parent_leaf, state) = parent_leaf_and_state(
        view,
        &sender,
        &receiver,
        quorum_membership,
        public_key.clone(),
        OuterConsensus::new(Arc::clone(&consensus.inner_consensus)),
        &upgrade_lock,
    )
    .await?;

    // In order of priority, we should try to attach:
    //   - the parent certificate if it exists, or
    //   - our own certificate that we formed.
    // In either case, we need to ensure that the certificate is still relevant.
    //
    // Note: once we reach a point of potentially propose with our formed upgrade certificate, we will ALWAYS drop it. If we cannot immediately use it for whatever reason, we choose to discard it.
    // It is possible that multiple nodes form separate upgrade certificates for the some upgrade if we are not careful about voting. But this shouldn't bother us: the first leader to propose is the one whose certificate will be used. And if that fails to reach a decide for whatever reason, we may lose our own certificate, but something will likely have gone wrong there anyway.
    let mut proposal_upgrade_certificate = parent_leaf
        .upgrade_certificate()
        .or(formed_upgrade_certificate);

    if let Some(cert) = proposal_upgrade_certificate.clone() {
        if cert
            .is_relevant(view, Arc::clone(&upgrade_lock.decided_upgrade_certificate))
            .await
            .is_err()
        {
            proposal_upgrade_certificate = None;
        }
    }

    // We only want to proposal to be attached if any of them are valid.
    let proposal_certificate = proposal_cert
        .as_ref()
        .filter(|cert| cert.is_valid_for_view(&view))
        .cloned();

    // FIXME - This is not great, and will be fixed later.
    // If it's > July, 2024 and this is still here, something has gone horribly wrong.
    let cnm = commitment_and_metadata
        .clone()
        .context("Cannot propose because we don't have the VID payload commitment and metadata")?;

    ensure!(
        cnm.block_view == view,
        "Cannot propose because our VID payload commitment and metadata is for an older view."
    );

    let version = upgrade_lock.version(view).await?;

    let create_and_send_proposal_handle = async_spawn(async move {
        match create_and_send_proposal::<TYPES, V>(
            public_key,
            private_key,
            OuterConsensus::new(Arc::clone(&consensus.inner_consensus)),
            sender,
            view,
            cnm,
            parent_leaf.clone(),
            state,
            proposal_upgrade_certificate,
            proposal_certificate,
            delay,
            instance_state,
            version,
            id,
        )
        .await
        {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("Failed to send proposal: {}", e);
            }
        };
    });

    Ok(create_and_send_proposal_handle)
}

/// Publishes a proposal if there exists a value which we can propose from. Specifically, we must have either
/// `commitment_and_metadata`, or a `decided_upgrade_certificate`.
#[allow(clippy::too_many_arguments)]
#[instrument(skip_all)]
pub async fn publish_proposal_if_able<TYPES: NodeType, V: Versions>(
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: OuterConsensus<TYPES>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    upgrade_lock: UpgradeLock<TYPES, V>,
    commitment_and_metadata: Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
    id: u64,
) -> Result<JoinHandle<()>> {
    publish_proposal_from_commitment_and_metadata(
        view,
        sender,
        receiver,
        quorum_membership,
        public_key,
        private_key,
        consensus,
        delay,
        formed_upgrade_certificate,
        upgrade_lock,
        commitment_and_metadata,
        proposal_cert,
        instance_state,
        id,
    )
    .await
}

/// Handle the received quorum proposal.
///
/// Returns the proposal that should be used to set the `cur_proposal` for other tasks.
#[allow(clippy::too_many_lines)]
#[instrument(skip_all)]
pub(crate) async fn handle_quorum_proposal_recv<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    sender: &TYPES::SignatureKey,
    event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut ConsensusTaskState<TYPES, I, V>,
) -> Result<Option<QuorumProposal<TYPES>>> {
    let sender = sender.clone();
    debug!(
        "Received Quorum Proposal for view {}",
        *proposal.data.view_number
    );

    let cur_view = task_state.cur_view;

    validate_proposal_view_and_certs(
        proposal,
        task_state.cur_view,
        &task_state.quorum_membership,
        &task_state.timeout_membership,
        &task_state.upgrade_lock,
    )
    .await
    .context("Failed to validate proposal view and attached certs")?;

    let view = proposal.data.view_number();
    let justify_qc = proposal.data.justify_qc.clone();

    if !justify_qc
        .is_valid_cert(
            task_state.quorum_membership.as_ref(),
            &task_state.upgrade_lock,
        )
        .await
    {
        let consensus = task_state.consensus.read().await;
        consensus.metrics.invalid_qc.update(1);
        bail!("Invalid justify_qc in proposal for view {}", *view);
    }

    // NOTE: We could update our view with a valid TC but invalid QC, but that is not what we do here
    if let Err(e) = update_view::<TYPES>(
        view,
        &event_sender,
        task_state.timeout,
        OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
        &mut task_state.cur_view,
        &mut task_state.cur_view_time,
        &mut task_state.timeout_task,
        &task_state.output_event_stream,
        SEND_VIEW_CHANGE_EVENT,
        task_state.quorum_membership.leader(cur_view) == task_state.public_key,
    )
    .await
    {
        debug!("Failed to update view; error = {e:#}");
    }

    let mut parent_leaf = task_state
        .consensus
        .read()
        .await
        .saved_leaves()
        .get(&justify_qc.date().leaf_commit)
        .cloned();

    parent_leaf = match parent_leaf {
        Some(p) => Some(p),
        None => fetch_proposal(
            justify_qc.view_number(),
            event_sender.clone(),
            event_receiver.clone(),
            Arc::clone(&task_state.quorum_membership),
            OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
            task_state.public_key.clone(),
            &task_state.upgrade_lock,
        )
        .await
        .ok(),
    };
    let consensus_read = task_state.consensus.read().await;

    // Get the parent leaf and state.
    let parent = match parent_leaf {
        Some(leaf) => {
            if let (Some(state), _) = consensus_read.state_and_delta(leaf.view_number()) {
                Some((leaf, Arc::clone(&state)))
            } else {
                bail!("Parent state not found! Consensus internally inconsistent");
            }
        }
        None => None,
    };

    if justify_qc.view_number() > consensus_read.high_qc().view_number {
        if let Err(e) = task_state
            .storage
            .write()
            .await
            .update_high_qc(justify_qc.clone())
            .await
        {
            bail!("Failed to store High QC not voting. Error: {:?}", e);
        }
    }

    drop(consensus_read);
    let mut consensus_write = task_state.consensus.write().await;

    if let Err(e) = consensus_write.update_high_qc(justify_qc.clone()) {
        tracing::trace!("{e:?}");
    }

    // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
    let Some((parent_leaf, _parent_state)) = parent else {
        warn!(
            "Proposal's parent missing from storage with commitment: {:?}",
            justify_qc.date().leaf_commit
        );
        let leaf = Leaf::from_quorum_proposal(&proposal.data);

        let state = Arc::new(
            <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(
                &proposal.data.block_header,
            ),
        );

        if let Err(e) = consensus_write.update_validated_state_map(
            view,
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                    state,
                    delta: None,
                },
            },
        ) {
            tracing::trace!("{e:?}");
        }

        consensus_write.update_saved_leaves(leaf.clone());
        let new_leaves = consensus_write.saved_leaves().clone();
        let new_state = consensus_write.validated_state_map().clone();
        drop(consensus_write);

        if let Err(e) = task_state
            .storage
            .write()
            .await
            .update_undecided_state(new_leaves, new_state)
            .await
        {
            warn!("Couldn't store undecided state.  Error: {:?}", e);
        }

        // If we are missing the parent from storage, the safety check will fail.  But we can
        // still vote if the liveness check succeeds.
        let consensus_read = task_state.consensus.read().await;
        let liveness_check = justify_qc.view_number() > consensus_read.locked_view();

        let high_qc = consensus_read.high_qc().clone();
        let locked_view = consensus_read.locked_view();

        drop(consensus_read);

        let mut current_proposal = None;
        if liveness_check {
            current_proposal = Some(proposal.data.clone());
            let new_view = proposal.data.view_number + 1;

            // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
            let should_propose = task_state.quorum_membership.leader(new_view)
                == task_state.public_key
                && high_qc.view_number == current_proposal.clone().unwrap().view_number;

            let qc = high_qc.clone();
            if should_propose {
                debug!(
                    "Attempting to publish proposal after voting for liveness; now in view: {}",
                    *new_view
                );
                let create_and_send_proposal_handle = publish_proposal_if_able(
                    qc.view_number + 1,
                    event_sender,
                    event_receiver,
                    Arc::clone(&task_state.quorum_membership),
                    task_state.public_key.clone(),
                    task_state.private_key.clone(),
                    OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
                    task_state.round_start_delay,
                    task_state.formed_upgrade_certificate.clone(),
                    task_state.upgrade_lock.clone(),
                    task_state.payload_commitment_and_metadata.clone(),
                    task_state.proposal_cert.clone(),
                    Arc::clone(&task_state.instance_state),
                    task_state.id,
                )
                .await?;

                task_state
                    .spawned_tasks
                    .entry(view)
                    .or_default()
                    .push(create_and_send_proposal_handle);
            }
        } else {
            warn!(?high_qc, ?proposal.data, ?locked_view, "Failed liveneess check; cannot find parent either.");
        }

        return Ok(current_proposal);
    };

    task_state
        .spawned_tasks
        .entry(proposal.data.view_number())
        .or_default()
        .push(async_spawn(
            validate_proposal_safety_and_liveness(
                proposal.clone(),
                parent_leaf,
                OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
                Arc::clone(&task_state.upgrade_lock.decided_upgrade_certificate),
                Arc::clone(&task_state.quorum_membership),
                event_sender.clone(),
                sender,
                task_state.output_event_stream.clone(),
                task_state.id,
                task_state.upgrade_lock.clone(),
            )
            .map(AnyhowTracing::err_as_debug),
        ));
    Ok(None)
}

/// Handle `QuorumProposalValidated` event content and submit a proposal if possible.
#[allow(clippy::too_many_lines)]
#[instrument(skip_all)]
pub async fn handle_quorum_proposal_validated<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: &QuorumProposal<TYPES>,
    event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut ConsensusTaskState<TYPES, I, V>,
) -> Result<()> {
    let view = proposal.view_number();
    task_state.current_proposal = Some(proposal.clone());

    let res = decide_from_proposal(
        proposal,
        OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
        Arc::clone(&task_state.upgrade_lock.decided_upgrade_certificate),
        &task_state.public_key,
    )
    .await;

    if let Some(cert) = res.decided_upgrade_cert {
        let mut decided_certificate_lock = task_state
            .upgrade_lock
            .decided_upgrade_certificate
            .write()
            .await;
        *decided_certificate_lock = Some(cert.clone());
        drop(decided_certificate_lock);
    }

    let mut consensus = task_state.consensus.write().await;
    if let Some(new_locked_view) = res.new_locked_view_number {
        if let Err(e) = consensus.update_locked_view(new_locked_view) {
            tracing::trace!("{e:?}");
        }
    }

    drop(consensus);

    let new_view = task_state.current_proposal.clone().unwrap().view_number + 1;
    // In future we can use the mempool model where we fetch the proposal if we don't have it, instead of having to wait for it here
    // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
    let should_propose = task_state.quorum_membership.leader(new_view) == task_state.public_key
        && task_state.consensus.read().await.high_qc().view_number
            == task_state.current_proposal.clone().unwrap().view_number;

    if let Some(new_decided_view) = res.new_decided_view_number {
        task_state.cancel_tasks(new_decided_view).await;
    }
    task_state.current_proposal = Some(proposal.clone());
    task_state
        .spawn_vote_task(view, event_sender.clone(), event_receiver.clone())
        .await;
    if should_propose {
        debug!(
            "Attempting to publish proposal after voting; now in view: {}",
            *new_view
        );
        if let Err(e) = task_state
            .publish_proposal(new_view, event_sender.clone(), event_receiver.clone())
            .await
        {
            debug!("Failed to propose; error = {e:?}");
        };
    }

    #[allow(clippy::cast_precision_loss)]
    if let Some(new_anchor_view) = res.new_decided_view_number {
        let block_size = res.included_txns.map(|set| set.len().try_into().unwrap());
        let decide_sent = broadcast_event(
            Event {
                view_number: new_anchor_view,
                event: EventType::Decide {
                    leaf_chain: Arc::new(res.leaf_views),
                    qc: Arc::new(res.new_decide_qc.unwrap()),
                    block_size,
                },
            },
            &task_state.output_event_stream,
        );
        let mut consensus = task_state.consensus.write().await;

        let old_anchor_view = consensus.last_decided_view();
        consensus.collect_garbage(old_anchor_view, new_anchor_view);
        if let Err(e) = consensus.update_last_decided_view(new_anchor_view) {
            tracing::trace!("{e:?}");
        }
        consensus
            .metrics
            .last_decided_time
            .set(Utc::now().timestamp().try_into().unwrap());
        consensus.metrics.invalid_qc.set(0);
        consensus
            .metrics
            .last_decided_view
            .set(usize::try_from(consensus.last_decided_view().u64()).unwrap());
        let cur_number_of_views_per_decide_event =
            *task_state.cur_view - consensus.last_decided_view().u64();
        consensus
            .metrics
            .number_of_views_per_decide_event
            .add_point(cur_number_of_views_per_decide_event as f64);

        debug!(
            "Sending Decide for view {:?}",
            consensus.last_decided_view()
        );
        drop(consensus);
        debug!("Decided txns len {:?}", block_size);
        decide_sent.await;
        broadcast_event(
            Arc::new(HotShotEvent::LeafDecided(res.leaves_decided)),
            &event_sender,
        )
        .await;
        debug!("decide send succeeded");
    }

    Ok(())
}

/// Private key, latest decided upgrade certificate, committee membership, and event stream, for
/// sending the vote.
pub(crate) struct VoteInfo<TYPES: NodeType, V: Versions> {
    /// The private key of the voting node.
    pub private_key: <<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey,

    /// The locked upgrade of the voting node.
    pub upgrade_lock: UpgradeLock<TYPES, V>,

    /// The DA Membership handle
    pub da_membership: Arc<<TYPES as NodeType>::Membership>,

    /// The event sending stream.
    pub event_sender: Sender<Arc<HotShotEvent<TYPES>>>,

    /// The event receiver stream.
    pub event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[allow(unused_variables)]
/// Check if we are able to vote, like whether the proposal is valid,
/// whether we have DAC and VID share, and if so, vote.
#[instrument(skip_all, fields(id = id, view = *cur_view))]
pub async fn update_state_and_vote_if_able<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    cur_view: TYPES::Time,
    proposal: QuorumProposal<TYPES>,
    public_key: TYPES::SignatureKey,
    consensus: OuterConsensus<TYPES>,
    storage: Arc<RwLock<I::Storage>>,
    quorum_membership: Arc<TYPES::Membership>,
    instance_state: Arc<TYPES::InstanceState>,
    vote_info: VoteInfo<TYPES, V>,
    id: u64,
    upgrade_lock: &UpgradeLock<TYPES, V>,
) -> bool {
    use hotshot_types::simple_vote::QuorumVote;

    if !quorum_membership.has_stake(&public_key) {
        debug!("We were not chosen for quorum committee on {:?}", cur_view);
        return false;
    }

    let read_consnesus = consensus.read().await;
    // Only vote if you has seen the VID share for this view
    let Some(vid_shares) = read_consnesus.vid_shares().get(&proposal.view_number) else {
        debug!(
            "We have not seen the VID share for this view {:?} yet, so we cannot vote.",
            proposal.view_number
        );
        return false;
    };
    let Some(vid_share) = vid_shares.get(&public_key).cloned() else {
        debug!("we have not seen our VID share yet");
        return false;
    };

    if let Some(upgrade_cert) = &vote_info
        .upgrade_lock
        .decided_upgrade_certificate
        .read()
        .await
        .clone()
    {
        if upgrade_cert.upgrading_in(cur_view)
            && Some(proposal.block_header.payload_commitment())
                != null_block::commitment(quorum_membership.total_nodes())
        {
            info!("Refusing to vote on proposal because it does not have a null commitment, and we are between versions. Expected:\n\n{:?}\n\nActual:{:?}", null_block::commitment(quorum_membership.total_nodes()), Some(proposal.block_header.payload_commitment()));
            return false;
        }
    }

    // Only vote if you have the DA cert
    // ED Need to update the view number this is stored under?
    let Some(cert) = read_consnesus.saved_da_certs().get(&cur_view).cloned() else {
        return false;
    };
    drop(read_consnesus);

    let view = cert.view_number;
    // TODO: do some of this logic without the vote token check, only do that when voting.
    let justify_qc = proposal.justify_qc.clone();
    let mut parent = consensus
        .read()
        .await
        .saved_leaves()
        .get(&justify_qc.date().leaf_commit)
        .cloned();
    parent = match parent {
        Some(p) => Some(p),
        None => fetch_proposal(
            justify_qc.view_number(),
            vote_info.event_sender.clone(),
            vote_info.event_receiver.clone(),
            Arc::clone(&quorum_membership),
            OuterConsensus::new(Arc::clone(&consensus.inner_consensus)),
            public_key.clone(),
            upgrade_lock,
        )
        .await
        .ok(),
    };

    let read_consnesus = consensus.read().await;

    // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
    let Some(parent) = parent else {
        error!(
            "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
            justify_qc.date().leaf_commit,
            proposal.view_number,
        );
        return false;
    };
    let (Some(parent_state), _) = read_consnesus.state_and_delta(parent.view_number()) else {
        warn!("Parent state not found! Consensus internally inconsistent");
        return false;
    };
    drop(read_consnesus);

    let version = match vote_info.upgrade_lock.version(view).await {
        Ok(version) => version,
        Err(e) => {
            error!("Failed to calculate the version: {e:?}");
            return false;
        }
    };
    let Ok((validated_state, state_delta)) = parent_state
        .validate_and_apply_header(
            instance_state.as_ref(),
            &parent,
            &proposal.block_header.clone(),
            vid_share.data.common.clone(),
            version,
        )
        .await
    else {
        warn!("Block header doesn't extend the proposal!");
        return false;
    };

    let state = Arc::new(validated_state);
    let delta = Arc::new(state_delta);
    let parent_commitment = parent.commit();

    let proposed_leaf = Leaf::from_quorum_proposal(&proposal);
    if proposed_leaf.parent_commitment() != parent_commitment {
        return false;
    }

    // Validate the DAC.
    let message = if cert
        .is_valid_cert(vote_info.da_membership.as_ref(), upgrade_lock)
        .await
    {
        // Validate the block payload commitment for non-genesis DAC.
        if cert.date().payload_commit != proposal.block_header.payload_commitment() {
            warn!(
                "Block payload commitment does not equal da cert payload commitment. View = {}",
                *view
            );
            return false;
        }
        if let Ok(vote) = QuorumVote::<TYPES>::create_signed_vote(
            QuorumData {
                leaf_commit: proposed_leaf.commit(),
            },
            view,
            &public_key,
            &vote_info.private_key,
            &vote_info.upgrade_lock,
        )
        .await
        {
            GeneralConsensusMessage::<TYPES>::Vote(vote)
        } else {
            error!("Unable to sign quorum vote!");
            return false;
        }
    } else {
        error!(
            "Invalid DAC in proposal! Skipping proposal. {:?} cur view is: {:?}",
            cert, cur_view
        );
        return false;
    };

    let mut consensus = consensus.write().await;
    if let Err(e) = consensus.update_validated_state_map(
        cur_view,
        View {
            view_inner: ViewInner::Leaf {
                leaf: proposed_leaf.commit(),
                state: Arc::clone(&state),
                delta: Some(Arc::clone(&delta)),
            },
        },
    ) {
        tracing::trace!("{e:?}");
    }
    consensus.update_saved_leaves(proposed_leaf.clone());
    let new_leaves = consensus.saved_leaves().clone();
    let new_state = consensus.validated_state_map().clone();
    drop(consensus);

    if let Err(e) = storage
        .write()
        .await
        .update_undecided_state(new_leaves, new_state)
        .await
    {
        error!("Couldn't store undecided state.  Error: {:?}", e);
    }

    if let GeneralConsensusMessage::Vote(vote) = message {
        debug!(
            "Sending vote to next quorum leader {:?}",
            vote.view_number() + 1
        );
        // Add to the storage that we have received the VID disperse for a specific view
        if let Err(e) = storage.write().await.append_vid(&vid_share).await {
            warn!(
                "Failed to store VID Disperse Proposal with error {:?}, aborting vote",
                e
            );
            return false;
        }
        broadcast_event(
            Arc::new(HotShotEvent::QuorumVoteSend(vote)),
            &vote_info.event_sender,
        )
        .await;
        return true;
    }
    debug!(
        "Received VID share, but couldn't find DAC cert for view {:?}",
        *proposal.view_number(),
    );
    false
}
