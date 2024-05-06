use core::time::Duration;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};

use anyhow::{bail, ensure, Context, Result};
use async_broadcast::Sender;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use chrono::Utc;
use committable::Committable;
use futures::FutureExt;
use hotshot_types::{
    consensus::{CommitmentAndMetadata, Consensus, View},
    data::{null_block, Leaf, QuorumProposal, ViewChangeEvidence},
    event::{Event, EventType, LeafInfo},
    message::Proposal,
    simple_certificate::UpgradeCertificate,
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        states::ValidatedState,
        storage::Storage,
        BlockPayload,
    },
    utils::{Terminator, ViewInner},
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

#[cfg(not(feature = "dependency-tasks"))]
use super::ConsensusTaskState;
#[cfg(feature = "dependency-tasks")]
use crate::quorum_proposal::QuorumProposalTaskState;
#[cfg(feature = "dependency-tasks")]
use crate::quorum_proposal_recv::QuorumProposalRecvTaskState;
use crate::{
    consensus::{update_view, view_change::SEND_VIEW_CHANGE_EVENT},
    events::HotShotEvent,
    helpers::{broadcast_event, AnyhowTracing},
};

/// Validate the state and safety and liveness of a proposal then emit
/// a `QuorumProposalValidated` event.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
async fn validate_proposal_safety_and_liveness<TYPES: NodeType>(
    proposal: Proposal<TYPES, QuorumProposal<TYPES>>,
    parent_leaf: Leaf<TYPES>,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    quorum_membership: Arc<TYPES::Membership>,
    parent_state: Arc<TYPES::ValidatedState>,
    view_leader_key: TYPES::SignatureKey,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    sender: TYPES::SignatureKey,
    event_sender: Sender<Event<TYPES>>,
    storage: Arc<RwLock<impl Storage<TYPES>>>,
    instance_state: Arc<TYPES::InstanceState>,
) -> Result<()> {
    let (validated_state, state_delta) = parent_state
        .validate_and_apply_header(
            instance_state.as_ref(),
            &parent_leaf,
            &proposal.data.block_header.clone(),
        )
        .await
        .context("Block header doesn't extend the proposal!")?;

    let state = Arc::new(validated_state);
    let delta = Arc::new(state_delta);
    let view = proposal.data.get_view_number();

    let proposed_leaf = Leaf::from_quorum_proposal(&proposal.data);
    ensure!(
        proposed_leaf.get_parent_commitment() == parent_leaf.commit(),
        "Proposed leaf does not extend the parent leaf."
    );

    // Validate the proposal's signature. This should also catch if the leaf_commitment does not equal our calculated parent commitment
    //
    // There is a mistake here originating in the genesis leaf/qc commit. This should be replaced by:
    //
    //    proposal.validate_signature(&quorum_membership)?;
    //
    // in a future PR.
    ensure!(
        view_leader_key.validate(&proposal.signature, proposed_leaf.commit().as_ref()),
        "Could not verify proposal."
    );

    UpgradeCertificate::validate(&proposal.data.upgrade_certificate, &quorum_membership)?;

    // Validate that the upgrade certificate is re-attached, if we saw one on the parent
    proposed_leaf.extends_upgrade(&parent_leaf, &decided_upgrade_certificate)?;

    let justify_qc = proposal.data.justify_qc.clone();
    // Create a positive vote if either liveness or safety check
    // passes.

    // Liveness check.
    let consensus = consensus.upgradable_read().await;
    let liveness_check = justify_qc.get_view_number() > consensus.locked_view;

    // Safety check.
    // Check if proposal extends from the locked leaf.
    let outcome = consensus.visit_leaf_ancestors(
        justify_qc.get_view_number(),
        Terminator::Inclusive(consensus.locked_view),
        false,
        |leaf, _, _| {
            // if leaf view no == locked view no then we're done, report success by
            // returning true
            leaf.get_view_number() != consensus.locked_view
        },
    );
    let safety_check = outcome.is_ok();

    ensure!(safety_check || liveness_check, {
        if let Err(e) = outcome {
            broadcast_event(
                Event {
                    view_number: view,
                    event: EventType::Error { error: Arc::new(e) },
                },
                &event_sender,
            )
            .await;
        }

        format!("Failed safety and liveness check \n High QC is {:?}  Proposal QC is {:?}  Locked view is {:?}", consensus.high_qc(), proposal.data.clone(), consensus.locked_view)
    });

    // We accept the proposal, notify the application layer

    broadcast_event(
        Event {
            view_number: view,
            event: EventType::QuorumProposal {
                proposal: proposal.clone(),
                sender,
            },
        },
        &event_sender,
    )
    .await;
    // Notify other tasks
    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalValidated(
            proposal.data.clone(),
            parent_leaf,
        )),
        &event_stream,
    )
    .await;

    let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;

    consensus.validated_state_map.insert(
        view,
        View {
            view_inner: ViewInner::Leaf {
                leaf: proposed_leaf.commit(),
                state: Arc::clone(&state),
                delta: Some(Arc::clone(&delta)),
            },
        },
    );
    consensus
        .saved_leaves
        .insert(proposed_leaf.commit(), proposed_leaf.clone());

    if let Err(e) = storage
        .write()
        .await
        .update_undecided_state(
            consensus.saved_leaves.clone(),
            consensus.validated_state_map.clone(),
        )
        .await
    {
        warn!("Couldn't store undecided state.  Error: {:?}", e);
    }

    Ok(())
}

/// Create the header for a proposal, build the proposal, and broadcast
/// the proposal send evnet.
#[allow(clippy::too_many_arguments)]
pub async fn create_and_send_proposal<TYPES: NodeType>(
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    view: TYPES::Time,
    commitment_and_metadata: CommitmentAndMetadata<TYPES>,
    parent_leaf: Leaf<TYPES>,
    state: Arc<TYPES::ValidatedState>,
    upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    round_start_delay: u64,
    instance_state: Arc<TYPES::InstanceState>,
) {
    let block_header = match TYPES::BlockHeader::new(
        state.as_ref(),
        instance_state.as_ref(),
        &parent_leaf,
        commitment_and_metadata.commitment,
        commitment_and_metadata.builder_commitment,
        commitment_and_metadata.metadata,
        commitment_and_metadata.fee,
    )
    .await
    {
        Ok(header) => header,
        Err(err) => {
            error!(%err, "Failed to construct block header");
            return;
        }
    };

    let proposal = QuorumProposal {
        block_header,
        view_number: view,
        justify_qc: consensus.read().await.high_qc().clone(),
        proposal_certificate: proposal_cert,
        upgrade_certificate: upgrade_cert,
    };

    let proposed_leaf = Leaf::from_quorum_proposal(&proposal);
    if proposed_leaf.get_parent_commitment() != parent_leaf.commit() {
        return;
    }

    let Ok(signature) = TYPES::SignatureKey::sign(&private_key, proposed_leaf.commit().as_ref())
    else {
        // This should never happen.
        error!("Failed to sign proposed_leaf.commit()!");
        return;
    };

    let message = Proposal {
        data: proposal,
        signature,
        _pd: PhantomData,
    };
    debug!(
        "Sending null proposal for view {:?}",
        proposed_leaf.get_view_number(),
    );

    async_sleep(Duration::from_millis(round_start_delay)).await;
    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalSend(
            message.clone(),
            public_key,
        )),
        &event_stream,
    )
    .await;
}

/// Validates, from a given `proposal` that the view that it is being submitted for is valid when
/// compared to `cur_view` which is the highest proposed view (so far) for the caller. If the proposal
/// is for a view that's later than expected, that the proposal includes a timeout or view sync certificate.
pub fn validate_proposal_view_and_certs<TYPES: NodeType>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    sender: &TYPES::SignatureKey,
    cur_view: TYPES::Time,
    quorum_membership: &Arc<TYPES::Membership>,
    timeout_membership: &Arc<TYPES::Membership>,
) -> Result<()> {
    let view = proposal.data.get_view_number();
    ensure!(
        view >= cur_view,
        "Proposal is from an older view {:?}",
        proposal.data.clone()
    );

    let view_leader_key = quorum_membership.get_leader(view);
    ensure!(
        view_leader_key == *sender,
        "Leader key does not match key in proposal"
    );

    // Verify a timeout certificate OR a view sync certificate exists and is valid.
    if proposal.data.justify_qc.get_view_number() != view - 1 {
        let received_proposal_cert =
            proposal.data.proposal_certificate.clone().context(format!(
                "Quorum proposal for view {} needed a timeout or view sync certificate, but did not have one",
                *view
        ))?;

        match received_proposal_cert {
            ViewChangeEvidence::Timeout(timeout_cert) => {
                ensure!(
                    timeout_cert.get_data().view == view - 1,
                    "Timeout certificate for view {} was not for the immediately preceding view",
                    *view
                );
                ensure!(
                    timeout_cert.is_valid_cert(timeout_membership.as_ref()),
                    "Timeout certificate for view {} was invalid",
                    *view
                );
            }
            ViewChangeEvidence::ViewSync(view_sync_cert) => {
                ensure!(
                    view_sync_cert.view_number == view,
                    "View sync cert view number {:?} does not match proposal view number {:?}",
                    view_sync_cert.view_number,
                    view
                );

                // View sync certs must also be valid.
                ensure!(
                    view_sync_cert.is_valid_cert(quorum_membership.as_ref()),
                    "Invalid view sync finalize cert provided"
                );
            }
        }
    }

    // Validate the upgrade certificate -- this is just a signature validation.
    // Note that we don't do anything with the certificate directly if this passes; it eventually gets stored as part of the leaf if nothing goes wrong.
    UpgradeCertificate::validate(&proposal.data.upgrade_certificate, quorum_membership)?;

    Ok(())
}

/// Gets the parent leaf and state from the parent of a proposal, returning an [`anyhow::Error`] if not.
pub async fn get_parent_leaf_and_state<TYPES: NodeType>(
    cur_view: TYPES::Time,
    view: TYPES::Time,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
) -> Result<(Leaf<TYPES>, Arc<<TYPES as NodeType>::ValidatedState>)> {
    ensure!(
        quorum_membership.get_leader(view) == public_key,
        "Somehow we formed a QC but are not the leader for the next view {view:?}",
    );

    let consensus = consensus.read().await;
    let parent_view_number = &consensus.high_qc().get_view_number();
    let parent_view = consensus.validated_state_map.get(parent_view_number).context(
        format!("Couldn't find parent view in state map, waiting for replica to see proposal; parent_view_number: {}", **parent_view_number)
    )?;

    // Leaf hash in view inner does not match high qc hash - Why?
    let (leaf_commitment, state) = parent_view.get_leaf_and_state().context(
        format!("Parent of high QC points to a view without a proposal; parent_view_number: {parent_view_number:?}, parent_view {parent_view:?}")
    )?;

    if leaf_commitment != consensus.high_qc().get_data().leaf_commit {
        // NOTE: This happens on the genesis block
        debug!(
            "They don't equal: {:?}   {:?}",
            leaf_commitment,
            consensus.high_qc().get_data().leaf_commit
        );
    }

    let leaf = consensus
        .saved_leaves
        .get(&leaf_commitment)
        .context("Failed to find high QC of parent")?;

    let reached_decided = leaf.get_view_number() == consensus.last_decided_view;
    let parent_leaf = leaf.clone();
    let original_parent_hash = parent_leaf.commit();
    let mut next_parent_hash = original_parent_hash;

    // Walk back until we find a decide
    if !reached_decided {
        debug!("We have not reached decide from view {:?}", cur_view);
        while let Some(next_parent_leaf) = consensus.saved_leaves.get(&next_parent_hash) {
            if next_parent_leaf.get_view_number() <= consensus.last_decided_view {
                break;
            }
            next_parent_hash = next_parent_leaf.get_parent_commitment();
        }
        // TODO do some sort of sanity check on the view number that it matches decided
    }

    Ok((parent_leaf, Arc::clone(state)))
}

/// Send a proposal for the view `view` from the latest high_qc given an upgrade cert. This is a special
/// case proposal scenario.
#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
async fn publish_proposal_from_upgrade_cert<TYPES: NodeType>(
    cur_view: TYPES::Time,
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    upgrade_cert: UpgradeCertificate<TYPES>,
    delay: u64,
    instance_state: Arc<TYPES::InstanceState>,
) -> Result<JoinHandle<()>> {
    let (parent_leaf, state) = get_parent_leaf_and_state(
        cur_view,
        view,
        Arc::clone(&quorum_membership),
        public_key.clone(),
        Arc::clone(&consensus),
    )
    .await?;

    // Special case: if we have a decided upgrade certificate AND it does not apply a version to the current view, we MUST propose with a null block.
    ensure!(upgrade_cert.in_interim(cur_view), "Cert is not in interim");
    let (payload, metadata) = <TYPES::BlockPayload as BlockPayload>::from_transactions(
        Vec::new(),
        Arc::<<TYPES as NodeType>::InstanceState>::clone(&instance_state),
    )
    .context("Failed to build null block payload and metadata")?;

    let builder_commitment = payload.builder_commitment(&metadata);
    let null_block_commitment = null_block::commitment(quorum_membership.total_nodes())
        .context("Failed to calculate null block commitment")?;

    let null_block_fee = null_block::builder_fee::<TYPES>(
        quorum_membership.total_nodes(),
        Arc::<<TYPES as NodeType>::InstanceState>::clone(&instance_state),
    )
    .context("Failed to calculate null block fee info")?;

    Ok(async_spawn(async move {
        create_and_send_proposal(
            public_key,
            private_key,
            consensus,
            sender,
            view,
            CommitmentAndMetadata {
                commitment: null_block_commitment,
                builder_commitment,
                metadata,
                fee: null_block_fee,
                block_view: view,
            },
            parent_leaf,
            state,
            Some(upgrade_cert),
            None,
            delay,
            Arc::clone(&instance_state),
        )
        .await;
    }))
}

/// Send a proposal for the view `view` from the latest high_qc given an upgrade cert. This is the
/// standard case proposal scenario.
#[allow(clippy::too_many_arguments)]
async fn publish_proposal_from_commitment_and_metadata<TYPES: NodeType>(
    cur_view: TYPES::Time,
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    commitment_and_metadata: &mut Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: &mut Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
) -> Result<JoinHandle<()>> {
    let (parent_leaf, state) = get_parent_leaf_and_state(
        cur_view,
        view,
        quorum_membership,
        public_key.clone(),
        Arc::clone(&consensus),
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
        .get_upgrade_certificate()
        .or(formed_upgrade_certificate);

    if !proposal_upgrade_certificate
        .clone()
        .is_some_and(|cert| cert.is_relevant(view, decided_upgrade_cert).is_ok())
    {
        proposal_upgrade_certificate = None;
    }

    // We only want to proposal to be attached if any of them are valid.
    let proposal_certificate = proposal_cert
        .as_ref()
        .filter(|cert| cert.is_valid_for_view(&view))
        .cloned();

    // FIXME - This is not great, and will be fixed later.
    // If it's > July, 2024 and this is still here, something has gone horribly wrong.
    let cnm = commitment_and_metadata
        .take()
        .context("Cannot propose because we don't have the VID payload commitment and metadata")?;

    ensure!(
        cnm.block_view == view,
        "Cannot propose because our VID payload commitment and metadata is for an older view."
    );

    let create_and_send_proposal_handle = async_spawn(async move {
        create_and_send_proposal(
            public_key,
            private_key,
            consensus,
            sender,
            view,
            cnm,
            parent_leaf.clone(),
            state,
            proposal_upgrade_certificate,
            proposal_certificate,
            delay,
            Arc::clone(&instance_state),
        )
        .await;
    });

    *proposal_cert = None;
    *commitment_and_metadata = None;

    Ok(create_and_send_proposal_handle)
}

/// Publishes a proposal if there exists a value which we can propose from. Specifically, we must have either
/// `commitment_and_metadata`, or a `decided_upgrade_cert`.
#[allow(clippy::too_many_arguments)]
pub async fn publish_proposal_if_able<TYPES: NodeType>(
    cur_view: TYPES::Time,
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    commitment_and_metadata: &mut Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: &mut Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
) -> Result<JoinHandle<()>> {
    if let Some(upgrade_cert) = decided_upgrade_cert {
        publish_proposal_from_upgrade_cert(
            cur_view,
            view,
            sender,
            quorum_membership,
            public_key,
            private_key,
            consensus,
            upgrade_cert,
            delay,
            instance_state,
        )
        .await
    } else {
        publish_proposal_from_commitment_and_metadata(
            cur_view,
            view,
            sender,
            quorum_membership,
            public_key,
            private_key,
            consensus,
            delay,
            formed_upgrade_certificate,
            decided_upgrade_cert,
            commitment_and_metadata,
            proposal_cert,
            instance_state,
        )
        .await
    }
}

/// TEMPORARY TYPE: Quorum proposal recv task state when using dependency tasks
#[cfg(feature = "dependency-tasks")]
type TemporaryProposalRecvCombinedType<TYPES, I> = QuorumProposalRecvTaskState<TYPES, I>;

/// TEMPORARY TYPE: Consensus task state when not using dependency tasks
#[cfg(not(feature = "dependency-tasks"))]
type TemporaryProposalRecvCombinedType<TYPES, I> = ConsensusTaskState<TYPES, I>;

// TODO: Fix `clippy::too_many_lines`.
/// Handle the received quorum proposal.
///
/// Returns the proposal that should be used to set the `cur_proposal` for other tasks.
#[allow(clippy::too_many_lines)]
pub async fn handle_quorum_proposal_recv<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    sender: &TYPES::SignatureKey,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut TemporaryProposalRecvCombinedType<TYPES, I>,
) -> Result<Option<QuorumProposal<TYPES>>> {
    let sender = sender.clone();
    debug!(
        "Received Quorum Proposal for view {}",
        *proposal.data.view_number
    );

    validate_proposal_view_and_certs(
        proposal,
        &sender,
        task_state.cur_view,
        &task_state.quorum_membership,
        &task_state.timeout_membership,
    )
    .context("Failed to validate proposal view and attached certs")?;

    let view = proposal.data.get_view_number();
    let view_leader_key = task_state.quorum_membership.get_leader(view);
    let justify_qc = proposal.data.justify_qc.clone();

    if !justify_qc.is_valid_cert(task_state.quorum_membership.as_ref()) {
        let consensus = task_state.consensus.write().await;
        consensus.metrics.invalid_qc.update(1);
        bail!("Invalid justify_qc in proposal for view {}", *view);
    }

    // NOTE: We could update our view with a valid TC but invalid QC, but that is not what we do here
    if let Err(e) = update_view::<TYPES>(
        view,
        &event_stream,
        task_state.timeout,
        Arc::clone(&task_state.consensus),
        &mut task_state.cur_view,
        &mut task_state.timeout_task,
        SEND_VIEW_CHANGE_EVENT,
    )
    .await
    {
        debug!("Failed to update view; error = {e:#}");
    }

    let consensus_read = task_state.consensus.upgradable_read().await;

    // Get the parent leaf and state.
    let parent = match consensus_read
        .saved_leaves
        .get(&justify_qc.get_data().leaf_commit)
        .cloned()
    {
        Some(leaf) => {
            if let (Some(state), _) = consensus_read.get_state_and_delta(leaf.get_view_number()) {
                Some((leaf, Arc::clone(&state)))
            } else {
                bail!("Parent state not found! Consensus internally inconsistent");
            }
        }
        None => None,
    };

    if justify_qc.get_view_number() > consensus_read.high_qc().view_number {
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

    let mut consensus_write = RwLockUpgradableReadGuard::upgrade(consensus_read).await;

    consensus_write.update_high_qc_if_new(justify_qc.clone());

    // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
    let Some((parent_leaf, parent_state)) = parent else {
        warn!(
            "Proposal's parent missing from storage with commitment: {:?}",
            justify_qc.get_data().leaf_commit
        );
        let leaf = Leaf::from_quorum_proposal(&proposal.data);

        let state = Arc::new(
            <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(
                &proposal.data.block_header,
            ),
        );

        consensus_write.validated_state_map.insert(
            view,
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                    state,
                    delta: None,
                },
            },
        );
        consensus_write
            .saved_leaves
            .insert(leaf.commit(), leaf.clone());

        if let Err(e) = task_state
            .storage
            .write()
            .await
            .update_undecided_state(
                consensus_write.saved_leaves.clone(),
                consensus_write.validated_state_map.clone(),
            )
            .await
        {
            warn!("Couldn't store undecided state.  Error: {:?}", e);
        }

        // If we are missing the parent from storage, the safety check will fail.  But we can
        // still vote if the liveness check succeeds.
        #[cfg(not(feature = "dependency-tasks"))]
        {
            let liveness_check = justify_qc.get_view_number() > consensus_write.locked_view;

            let high_qc = consensus_write.high_qc().clone();
            let locked_view = consensus_write.locked_view;

            drop(consensus_write);

            let mut current_proposal = None;
            if liveness_check {
                current_proposal = Some(proposal.data.clone());
                let new_view = proposal.data.view_number + 1;

                // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
                let should_propose = task_state.quorum_membership.get_leader(new_view)
                    == task_state.public_key
                    && high_qc.view_number == current_proposal.clone().unwrap().view_number;

                let qc = high_qc.clone();
                if should_propose {
                    debug!(
                        "Attempting to publish proposal after voting; now in view: {}",
                        *new_view
                    );
                    let create_and_send_proposal_handle = publish_proposal_if_able(
                        task_state.cur_view,
                        qc.view_number + 1,
                        event_stream,
                        Arc::clone(&task_state.quorum_membership),
                        task_state.public_key.clone(),
                        task_state.private_key.clone(),
                        Arc::clone(&task_state.consensus),
                        task_state.round_start_delay,
                        task_state.formed_upgrade_certificate.clone(),
                        task_state.decided_upgrade_cert.clone(),
                        &mut task_state.payload_commitment_and_metadata,
                        &mut task_state.proposal_cert,
                        Arc::clone(&task_state.instance_state),
                    )
                    .await?;

                    task_state
                        .spawned_tasks
                        .entry(view)
                        .or_default()
                        .push(create_and_send_proposal_handle);
                }
            }

            warn!(?high_qc, ?proposal.data, ?locked_view, "Failed liveneess check; cannot find parent either.");

            return Ok(current_proposal);
        }

        #[cfg(feature = "dependency-tasks")]
        return Ok(None);
    };

    task_state
        .spawned_tasks
        .entry(proposal.data.get_view_number())
        .or_default()
        .push(async_spawn(
            validate_proposal_safety_and_liveness(
                proposal.clone(),
                parent_leaf,
                Arc::clone(&task_state.consensus),
                task_state.decided_upgrade_cert.clone(),
                Arc::clone(&task_state.quorum_membership),
                Arc::clone(&parent_state),
                view_leader_key,
                event_stream.clone(),
                sender,
                task_state.output_event_stream.clone(),
                Arc::clone(&task_state.storage),
                Arc::clone(&task_state.instance_state),
            )
            .map(AnyhowTracing::err_as_debug),
        ));
    Ok(None)
}

/// TEMPORARY TYPE: Quorum proposal task state when using dependency tasks
#[cfg(feature = "dependency-tasks")]
type TemporaryProposalValidatedCombinedType<TYPES, I> = QuorumProposalTaskState<TYPES, I>;

/// TEMPORARY TYPE: Consensus task state when not using dependency tasks
#[cfg(not(feature = "dependency-tasks"))]
type TemporaryProposalValidatedCombinedType<TYPES, I> = ConsensusTaskState<TYPES, I>;

/// Handle `QuorumProposalValidated` event content and submit a proposal if possible.
#[allow(clippy::too_many_lines)]
pub async fn handle_quorum_proposal_validated<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    proposal: &QuorumProposal<TYPES>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut TemporaryProposalValidatedCombinedType<TYPES, I>,
) -> Result<()> {
    let consensus = task_state.consensus.upgradable_read().await;
    let view = proposal.get_view_number();
    #[cfg(not(feature = "dependency-tasks"))]
    {
        task_state.current_proposal = Some(proposal.clone());
    }

    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut decided_upgrade_cert: Option<UpgradeCertificate<TYPES>> = None;
    let mut new_anchor_view = consensus.last_decided_view;
    let mut new_locked_view = consensus.locked_view;
    let mut last_view_number_visited = view;
    let mut new_commit_reached: bool = false;
    let mut new_decide_reached = false;
    let mut new_decide_qc = None;
    let mut leaf_views = Vec::new();
    let mut leafs_decided = Vec::new();
    let mut included_txns = HashSet::new();
    let old_anchor_view = consensus.last_decided_view;
    let parent_view = proposal.justify_qc.get_view_number();
    let mut current_chain_length = 0usize;
    if parent_view + 1 == view {
        current_chain_length += 1;
        if let Err(e) = consensus.visit_leaf_ancestors(
            parent_view,
            Terminator::Exclusive(old_anchor_view),
            true,
            |leaf, state, delta| {
                if !new_decide_reached {
                    if last_view_number_visited == leaf.get_view_number() + 1 {
                        last_view_number_visited = leaf.get_view_number();
                        current_chain_length += 1;
                        if current_chain_length == 2 {
                            new_locked_view = leaf.get_view_number();
                            new_commit_reached = true;
                            // The next leaf in the chain, if there is one, is decided, so this
                            // leaf's justify_qc would become the QC for the decided chain.
                            new_decide_qc = Some(leaf.get_justify_qc().clone());
                        } else if current_chain_length == 3 {
                            new_anchor_view = leaf.get_view_number();
                            new_decide_reached = true;
                        }
                    } else {
                        // nothing more to do here... we don't have a new chain extension
                        return false;
                    }
                }
                // starting from the first iteration with a three chain, e.g. right after the else if case nested in the if case above
                if new_decide_reached {
                    let mut leaf = leaf.clone();
                    if leaf.get_view_number() == new_anchor_view {
                        consensus
                            .metrics
                            .last_synced_block_height
                            .set(usize::try_from(leaf.get_height()).unwrap_or(0));
                    }
                    if let Some(cert) = leaf.get_upgrade_certificate() {
                        if cert.data.decide_by < view {
                            warn!("Failed to decide an upgrade certificate in time. Ignoring.");
                        } else {
                            info!(
                                "Updating consensus state with decided upgrade certificate: {:?}",
                                cert
                            );
                            #[cfg(not(feature = "dependency-tasks"))]
                            {
                                task_state.decided_upgrade_cert = Some(cert.clone());
                            }

                            #[cfg(feature = "dependency-tasks")]
                            {
                                decided_upgrade_cert = Some(cert.clone());
                            }
                        }
                    }
                    // If the block payload is available for this leaf, include it in
                    // the leaf chain that we send to the client.
                    if let Some(encoded_txns) =
                        consensus.saved_payloads.get(&leaf.get_view_number())
                    {
                        let payload = BlockPayload::from_bytes(
                            encoded_txns,
                            leaf.get_block_header().metadata(),
                        );

                        leaf.fill_block_payload_unchecked(payload);
                    }

                    // Get the VID share at the leaf's view number, corresponding to our key
                    // (if one exists)
                    let vid_share = consensus
                        .vid_shares
                        .get(&leaf.get_view_number())
                        .unwrap_or(&HashMap::new())
                        .get(&task_state.public_key)
                        .cloned()
                        .map(|prop| prop.data);

                    // Add our data into a new `LeafInfo`
                    leaf_views.push(LeafInfo::new(
                        leaf.clone(),
                        Arc::clone(&state),
                        delta.clone(),
                        vid_share,
                    ));
                    leafs_decided.push(leaf.clone());
                    if let Some(ref payload) = leaf.get_block_payload() {
                        for txn in
                            payload.transaction_commitments(leaf.get_block_header().metadata())
                        {
                            included_txns.insert(txn);
                        }
                    }
                }
                true
            },
        ) {
            error!("view publish error {e}");
        }
    }

    let included_txns_set: HashSet<_> = if new_decide_reached {
        included_txns
    } else {
        HashSet::new()
    };

    let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
    if new_commit_reached {
        consensus.locked_view = new_locked_view;
    }

    // This is ALWAYS None if "dependency-tasks" is not active.
    #[cfg(feature = "dependency-tasks")]
    {
        consensus.dontuse_decided_upgrade_cert = decided_upgrade_cert;
    }

    #[allow(clippy::cast_precision_loss)]
    if new_decide_reached {
        broadcast_event(
            Arc::new(HotShotEvent::LeafDecided(leafs_decided)),
            &event_stream,
        )
        .await;
        let decide_sent = broadcast_event(
            Event {
                view_number: consensus.last_decided_view,
                event: EventType::Decide {
                    leaf_chain: Arc::new(leaf_views),
                    qc: Arc::new(new_decide_qc.unwrap()),
                    block_size: Some(included_txns_set.len().try_into().unwrap()),
                },
            },
            &task_state.output_event_stream,
        );
        let old_anchor_view = consensus.last_decided_view;
        consensus.collect_garbage(old_anchor_view, new_anchor_view);
        consensus.last_decided_view = new_anchor_view;
        consensus
            .metrics
            .last_decided_time
            .set(Utc::now().timestamp().try_into().unwrap());
        consensus.metrics.invalid_qc.set(0);
        consensus
            .metrics
            .last_decided_view
            .set(usize::try_from(consensus.last_decided_view.get_u64()).unwrap());
        let cur_number_of_views_per_decide_event = {
            #[cfg(not(feature = "dependency-tasks"))]
            {
                *task_state.cur_view - consensus.last_decided_view.get_u64()
            }

            #[cfg(feature = "dependency-tasks")]
            {
                *task_state.latest_proposed_view - consensus.last_decided_view.get_u64()
            }
        };
        consensus
            .metrics
            .number_of_views_per_decide_event
            .add_point(cur_number_of_views_per_decide_event as f64);

        debug!("Sending Decide for view {:?}", consensus.last_decided_view);
        debug!("Decided txns len {:?}", included_txns_set.len());
        decide_sent.await;
        debug!("decide send succeeded");
    }

    #[cfg(not(feature = "dependency-tasks"))]
    {
        let new_view = task_state.current_proposal.clone().unwrap().view_number + 1;
        // In future we can use the mempool model where we fetch the proposal if we don't have it, instead of having to wait for it here
        // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
        let should_propose = task_state.quorum_membership.get_leader(new_view)
            == task_state.public_key
            && consensus.high_qc().view_number
                == task_state.current_proposal.clone().unwrap().view_number;
        // todo get rid of this clone
        let qc = consensus.high_qc().clone();

        drop(consensus);
        if new_decide_reached {
            task_state.cancel_tasks(new_anchor_view).await;
        }
        if should_propose {
            debug!(
                "Attempting to publish proposal after voting; now in view: {}",
                *new_view
            );
            if let Err(e) = task_state
                .publish_proposal(qc.view_number + 1, event_stream.clone())
                .await
            {
                debug!("Failed to propose; error = {e:#}");
            };
        }

        ensure!(
            task_state.vote_if_able(&event_stream).await,
            "Failed to vote"
        );
        task_state.current_proposal = None;
    }

    Ok(())
}
