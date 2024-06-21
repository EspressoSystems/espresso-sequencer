use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{events::ProposalMissing, request::REQUEST_TIMEOUT};
use anyhow::bail;
use anyhow::{ensure, Context, Result};
use async_broadcast::{broadcast, Sender};
use async_compatibility_layer::art::async_timeout;
use async_lock::RwLock;
#[cfg(not(feature = "dependency-tasks"))]
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use committable::{Commitment, Committable};
use hotshot_types::{
    consensus::{Consensus, View},
    data::{Leaf, QuorumProposal, ViewChangeEvidence},
    event::{Event, EventType, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{
        block_contents::BlockHeader, election::Membership, node_implementation::NodeType,
        signature_key::SignatureKey, states::ValidatedState, BlockPayload,
    },
    utils::{Terminator, ViewInner},
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};
#[cfg(not(feature = "dependency-tasks"))]
use {
    super::ConsensusTaskState,
    crate::{
        consensus::{update_view, view_change::SEND_VIEW_CHANGE_EVENT},
        helpers::AnyhowTracing,
    },
    async_compatibility_layer::art::{async_sleep, async_spawn},
    chrono::Utc,
    core::time::Duration,
    futures::FutureExt,
    hotshot_types::{
        consensus::CommitmentAndMetadata,
        traits::{
            node_implementation::{ConsensusTime, NodeImplementation},
            storage::Storage,
        },
    },
    hotshot_types::{data::null_block, message::GeneralConsensusMessage, simple_vote::QuorumData},
    std::marker::PhantomData,
    tracing::error,
    vbs::version::Version,
};

use crate::{events::HotShotEvent, helpers::broadcast_event};

/// Validate the state and safety and liveness of a proposal then emit
/// a `QuorumProposalValidated` event.
///
/// TODO - This should just take the QuorumProposalRecv task state after
/// we merge the dependency tasks.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub async fn validate_proposal_safety_and_liveness<TYPES: NodeType>(
    proposal: Proposal<TYPES, QuorumProposal<TYPES>>,
    parent_leaf: Leaf<TYPES>,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    quorum_membership: Arc<TYPES::Membership>,
    view_leader_key: TYPES::SignatureKey,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    sender: TYPES::SignatureKey,
    event_sender: Sender<Event<TYPES>>,
) -> Result<()> {
    let view_number = proposal.data.view_number();

    let proposed_leaf = Leaf::from_quorum_proposal(&proposal.data);
    ensure!(
        proposed_leaf.parent_commitment() == parent_leaf.commit(),
        "Proposed leaf does not extend the parent leaf."
    );

    let state = Arc::new(
        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(&proposal.data.block_header),
    );
    let view = View {
        view_inner: ViewInner::Leaf {
            leaf: proposed_leaf.commit(),
            state,
            delta: None, // May be updated to `Some` in the vote task.
        },
    };

    if let Err(e) = consensus
        .write()
        .await
        .update_validated_state_map(view_number, view.clone())
    {
        tracing::trace!("{e:?}");
    }
    consensus
        .write()
        .await
        .update_saved_leaves(proposed_leaf.clone());

    // Broadcast that we've updated our consensus state so that other tasks know it's safe to grab.
    broadcast_event(
        Arc::new(HotShotEvent::ValidatedStateUpdated(view_number, view)),
        &event_stream,
    )
    .await;

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
    let read_consensus = consensus.read().await;
    let liveness_check = justify_qc.view_number() > read_consensus.locked_view();

    // Safety check.
    // Check if proposal extends from the locked leaf.
    let outcome = read_consensus.visit_leaf_ancestors(
        justify_qc.view_number(),
        Terminator::Inclusive(read_consensus.locked_view()),
        false,
        |leaf, _, _| {
            // if leaf view no == locked view no then we're done, report success by
            // returning true
            leaf.view_number() != read_consensus.locked_view()
        },
    );
    let safety_check = outcome.is_ok();

    ensure!(safety_check || liveness_check, {
        if let Err(e) = outcome {
            broadcast_event(
                Event {
                    view_number,
                    event: EventType::Error { error: Arc::new(e) },
                },
                &event_sender,
            )
            .await;
        }

        format!("Failed safety and liveness check \n High QC is {:?}  Proposal QC is {:?}  Locked view is {:?}", read_consensus.high_qc(), proposal.data.clone(), read_consensus.locked_view())
    });

    // We accept the proposal, notify the application layer

    broadcast_event(
        Event {
            view_number,
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

    Ok(())
}

/// Create the header for a proposal, build the proposal, and broadcast
/// the proposal send evnet.
#[allow(clippy::too_many_arguments)]
#[cfg(not(feature = "dependency-tasks"))]
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
    version: Version,
) {
    let consensus_read = consensus.read().await;
    let Some(Some(vid_share)) = consensus_read
        .vid_shares()
        .get(&view)
        .map(|shares| shares.get(&public_key).cloned())
    else {
        error!("Cannot propopse without our VID share, view {:?}", view);
        return;
    };
    drop(consensus_read);
    let block_header = match TYPES::BlockHeader::new(
        state.as_ref(),
        instance_state.as_ref(),
        &parent_leaf,
        commitment_and_metadata.commitment,
        commitment_and_metadata.builder_commitment,
        commitment_and_metadata.metadata,
        commitment_and_metadata.fee,
        vid_share.data.common,
        version,
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
    if proposed_leaf.parent_commitment() != parent_leaf.commit() {
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
        proposed_leaf.view_number(),
    );
    if let Err(e) = consensus
        .write()
        .await
        .update_last_proposed_view(message.clone())
    {
        tracing::trace!("{e:?}");
        return;
    }
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
    let view = proposal.data.view_number();
    ensure!(
        view >= cur_view,
        "Proposal is from an older view {:?}",
        proposal.data.clone()
    );

    let view_leader_key = quorum_membership.leader(view);
    ensure!(
        view_leader_key == *sender,
        "Leader key does not match key in proposal"
    );

    // Verify a timeout certificate OR a view sync certificate exists and is valid.
    if proposal.data.justify_qc.view_number() != view - 1 {
        let received_proposal_cert =
            proposal.data.proposal_certificate.clone().context(format!(
                "Quorum proposal for view {} needed a timeout or view sync certificate, but did not have one",
                *view
        ))?;

        match received_proposal_cert {
            ViewChangeEvidence::Timeout(timeout_cert) => {
                ensure!(
                    timeout_cert.date().view == view - 1,
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
pub(crate) async fn parent_leaf_and_state<TYPES: NodeType>(
    next_proposal_view_number: TYPES::Time,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
) -> Result<(Leaf<TYPES>, Arc<<TYPES as NodeType>::ValidatedState>)> {
    ensure!(
        quorum_membership.leader(next_proposal_view_number) == public_key,
        "Somehow we formed a QC but are not the leader for the next view {next_proposal_view_number:?}",
    );

    let consensus_reader = consensus.read().await;
    let parent_view_number = consensus_reader.high_qc().view_number();
    let parent_view = consensus_reader.validated_state_map().get(&parent_view_number).context(
        format!("Couldn't find parent view in state map, waiting for replica to see proposal; parent_view_number: {}", *parent_view_number)
    )?;

    // Leaf hash in view inner does not match high qc hash - Why?
    let (leaf_commitment, state) = parent_view.leaf_and_state().context(
        format!("Parent of high QC points to a view without a proposal; parent_view_number: {parent_view_number:?}, parent_view {parent_view:?}")
    )?;

    if leaf_commitment != consensus_reader.high_qc().date().leaf_commit {
        // NOTE: This happens on the genesis block
        debug!(
            "They don't equal: {:?}   {:?}",
            leaf_commitment,
            consensus_reader.high_qc().date().leaf_commit
        );
    }

    let leaf = consensus_reader
        .saved_leaves()
        .get(&leaf_commitment)
        .context("Failed to find high QC of parent")?;

    let reached_decided = leaf.view_number() == consensus_reader.last_decided_view();
    let parent_leaf = leaf.clone();
    let original_parent_hash = parent_leaf.commit();
    let mut next_parent_hash = original_parent_hash;

    // Walk back until we find a decide
    if !reached_decided {
        debug!("We have not reached decide");
        while let Some(next_parent_leaf) = consensus_reader.saved_leaves().get(&next_parent_hash) {
            if next_parent_leaf.view_number() <= consensus_reader.last_decided_view() {
                break;
            }
            next_parent_hash = next_parent_leaf.parent_commitment();
        }
        // TODO do some sort of sanity check on the view number that it matches decided
    }

    Ok((parent_leaf, Arc::clone(state)))
}

/// Send a proposal for the view `view` from the latest high_qc given an upgrade cert. This is the
/// standard case proposal scenario.
#[allow(clippy::too_many_arguments)]
#[cfg(not(feature = "dependency-tasks"))]
pub async fn publish_proposal_from_commitment_and_metadata<TYPES: NodeType>(
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    commitment_and_metadata: Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
    version: Version,
) -> Result<JoinHandle<()>> {
    let (parent_leaf, state) = parent_leaf_and_state(
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
        .upgrade_certificate()
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
        .clone()
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
            instance_state,
            version,
        )
        .await;
    });

    Ok(create_and_send_proposal_handle)
}

/// Publishes a proposal if there exists a value which we can propose from. Specifically, we must have either
/// `commitment_and_metadata`, or a `decided_upgrade_cert`.
#[allow(clippy::too_many_arguments)]
#[cfg(not(feature = "dependency-tasks"))]
pub async fn publish_proposal_if_able<TYPES: NodeType>(
    view: TYPES::Time,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    delay: u64,
    formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
    commitment_and_metadata: Option<CommitmentAndMetadata<TYPES>>,
    proposal_cert: Option<ViewChangeEvidence<TYPES>>,
    instance_state: Arc<TYPES::InstanceState>,
    version: Version,
) -> Result<JoinHandle<()>> {
    publish_proposal_from_commitment_and_metadata(
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
        version,
    )
    .await
}

/// Trigger a request to the network for a proposal for a view and wait for the response
pub(crate) async fn fetch_proposal<TYPES: NodeType>(
    view: TYPES::Time,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
) -> Result<Leaf<TYPES>> {
    let (tx, mut rx) = broadcast(1);
    let event = ProposalMissing {
        view,
        response_chan: tx,
    };
    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalRequest(event)),
        &event_stream,
    )
    .await;
    let Ok(Ok(Some(proposal))) = async_timeout(REQUEST_TIMEOUT, rx.recv_direct()).await else {
        bail!("Request for proposal failed");
    };
    let view_number = proposal.data.view_number();
    let justify_qc = proposal.data.justify_qc.clone();

    if !justify_qc.is_valid_cert(quorum_membership.as_ref()) {
        bail!("Invalid justify_qc in proposal for view {}", *view_number);
    }
    let mut consensus_write = consensus.write().await;
    let leaf = Leaf::from_quorum_proposal(&proposal.data);
    let state = Arc::new(
        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(&proposal.data.block_header),
    );

    let view = View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(),
            state,
            delta: None,
        },
    };
    if let Err(e) = consensus_write.update_validated_state_map(view_number, view.clone()) {
        tracing::trace!("{e:?}");
    }

    consensus_write.update_saved_leaves(leaf.clone());
    broadcast_event(
        HotShotEvent::ValidatedStateUpdated(view_number, view).into(),
        &event_stream,
    )
    .await;
    Ok(leaf)
}

/// Handle the received quorum proposal.
///
/// Returns the proposal that should be used to set the `cur_proposal` for other tasks.
#[allow(clippy::too_many_lines)]
#[cfg(not(feature = "dependency-tasks"))]
pub(crate) async fn handle_quorum_proposal_recv<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    sender: &TYPES::SignatureKey,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut ConsensusTaskState<TYPES, I>,
    version: Version,
) -> Result<Option<QuorumProposal<TYPES>>> {
    let sender = sender.clone();
    debug!(
        "Received Quorum Proposal for view {}",
        *proposal.data.view_number
    );

    let cur_view = task_state.cur_view;

    validate_proposal_view_and_certs(
        proposal,
        &sender,
        task_state.cur_view,
        &task_state.quorum_membership,
        &task_state.timeout_membership,
    )
    .context("Failed to validate proposal view and attached certs")?;

    let view = proposal.data.view_number();
    let view_leader_key = task_state.quorum_membership.leader(view);
    let justify_qc = proposal.data.justify_qc.clone();

    if !justify_qc.is_valid_cert(task_state.quorum_membership.as_ref()) {
        let consensus = task_state.consensus.read().await;
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
            event_stream.clone(),
            Arc::clone(&task_state.quorum_membership),
            Arc::clone(&task_state.consensus),
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
        #[cfg(not(feature = "dependency-tasks"))]
        {
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
                        event_stream,
                        Arc::clone(&task_state.quorum_membership),
                        task_state.public_key.clone(),
                        task_state.private_key.clone(),
                        Arc::clone(&task_state.consensus),
                        task_state.round_start_delay,
                        task_state.formed_upgrade_certificate.clone(),
                        task_state.decided_upgrade_cert.clone(),
                        task_state.payload_commitment_and_metadata.clone(),
                        task_state.proposal_cert.clone(),
                        Arc::clone(&task_state.instance_state),
                        version,
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
        }

        #[cfg(feature = "dependency-tasks")]
        return Ok(None);
    };

    task_state
        .spawned_tasks
        .entry(proposal.data.view_number())
        .or_default()
        .push(async_spawn(
            validate_proposal_safety_and_liveness(
                proposal.clone(),
                parent_leaf,
                Arc::clone(&task_state.consensus),
                task_state.decided_upgrade_cert.clone(),
                Arc::clone(&task_state.quorum_membership),
                view_leader_key,
                event_stream.clone(),
                sender,
                task_state.output_event_stream.clone(),
            )
            .map(AnyhowTracing::err_as_debug),
        ));
    Ok(None)
}

/// Helper type to give names and to the output values of the leaf chain traversal operation.
#[derive(Debug)]
pub struct LeafChainTraversalOutcome<TYPES: NodeType> {
    /// The new locked view obtained from a 2 chain starting from the proposal's parent.
    pub new_locked_view_number: Option<TYPES::Time>,

    /// The new decided view obtained from a 3 chain starting from the proposal's parent.
    pub new_decided_view_number: Option<TYPES::Time>,

    /// The qc for the decided chain.
    pub new_decide_qc: Option<QuorumCertificate<TYPES>>,

    /// The decided leaves with corresponding validated state and VID info.
    pub leaf_views: Vec<LeafInfo<TYPES>>,

    /// The decided leaves.
    pub leaves_decided: Vec<Leaf<TYPES>>,

    /// The transactions in the block payload for each leaf.
    pub included_txns: Option<HashSet<Commitment<<TYPES as NodeType>::Transaction>>>,

    /// The most recent upgrade certificate from one of the leaves.
    pub decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
}

/// We need Default to be implemented because the leaf ascension has very few failure branches,
/// and when they *do* happen, we still return intermediate states. Default makes the burden
/// of filling values easier.
impl<TYPES: NodeType + Default> Default for LeafChainTraversalOutcome<TYPES> {
    /// The default method for this type is to set all of the returned values to `None`.
    fn default() -> Self {
        Self {
            new_locked_view_number: None,
            new_decided_view_number: None,
            new_decide_qc: None,
            leaf_views: Vec::new(),
            leaves_decided: Vec::new(),
            included_txns: None,
            decided_upgrade_cert: None,
        }
    }
}

/// Ascends the leaf chain by traversing through the parent commitments of the proposal. We begin
/// by obtaining the parent view, and if we are in a chain (i.e. the next view from the parent is
/// one view newer), then we begin attempting to form the chain. This is a direct impl from
/// [HotStuff](https://arxiv.org/pdf/1803.05069) section 5:
///
/// > When a node b* carries a QC that refers to a direct parent, i.e., b*.justify.node = b*.parent,
/// we say that it forms a One-Chain. Denote by b'' = b*.justify.node. Node b* forms a Two-Chain,
/// if in addition to forming a One-Chain, b''.justify.node = b''.parent.
/// It forms a Three-Chain, if b'' forms a Two-Chain.
///
/// We follow this exact logic to determine if we are able to reach a commit and a decide. A commit
/// is reached when we have a two chain, and a decide is reached when we have a three chain.
///
/// # Example
/// Suppose we have a decide for view 1, and we then move on to get undecided views 2, 3, and 4. Further,
/// suppose that our *next* proposal is for view 5, but this leader did not see info for view 4, so the
/// justify qc of the proposal points to view 3. This is fine, and the undecided chain now becomes
/// 2-3-5.
///
/// Assuming we continue with honest leaders, we then eventually could get a chain like: 2-3-5-6-7-8. This
/// will prompt a decide event to occur (this code), where the `proposal` is for view 8. Now, since the
/// lowest value in the 3-chain here would be 5 (excluding 8 since we only walk the parents), we begin at
/// the first link in the chain, and walk back through all undecided views, making our new anchor view 5,
/// and out new locked view will be 6.
///
/// Upon receipt then of a proposal for view 9, assuming it is valid, this entire process will repeat, and
/// the anchor view will be set to view 6, with the locked view as view 7.
pub async fn decide_from_proposal<TYPES: NodeType>(
    proposal: &QuorumProposal<TYPES>,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    existing_upgrade_cert: &Option<UpgradeCertificate<TYPES>>,
    public_key: &TYPES::SignatureKey,
) -> LeafChainTraversalOutcome<TYPES> {
    let consensus_reader = consensus.read().await;
    let view_number = proposal.view_number();
    let parent_view_number = proposal.justify_qc.view_number();
    let old_anchor_view = consensus_reader.last_decided_view();

    let mut last_view_number_visited = view_number;
    let mut current_chain_length = 0usize;
    let mut res = LeafChainTraversalOutcome::default();

    if let Err(e) = consensus_reader.visit_leaf_ancestors(
        parent_view_number,
        Terminator::Exclusive(old_anchor_view),
        true,
        |leaf, state, delta| {
            // This is the core paper logic. We're implementing the chain in chained hotstuff.
            if res.new_decided_view_number.is_none() {
                // If the last view number is the child of the leaf we've moved to...
                if last_view_number_visited == leaf.view_number() + 1 {
                    last_view_number_visited = leaf.view_number();

                    // The chain grows by one
                    current_chain_length += 1;

                    // We emit a locked view when the chain length is 2
                    if current_chain_length == 2 {
                        res.new_locked_view_number = Some(leaf.view_number());
                        // The next leaf in the chain, if there is one, is decided, so this
                        // leaf's justify_qc would become the QC for the decided chain.
                        res.new_decide_qc = Some(leaf.justify_qc().clone());
                    } else if current_chain_length == 3 {
                        // And we decide when the chain length is 3.
                        res.new_decided_view_number = Some(leaf.view_number());
                    }
                } else {
                    // There isn't a new chain extension available, so we signal to the callback
                    // owner that we can exit for now.
                    return false;
                }
            }

            // Now, if we *have* reached a decide, we need to do some state updates.
            if let Some(new_decided_view) = res.new_decided_view_number {
                // First, get a mutable reference to the provided leaf.
                let mut leaf = leaf.clone();

                // Update the metrics
                if leaf.view_number() == new_decided_view {
                    consensus_reader
                        .metrics
                        .last_synced_block_height
                        .set(usize::try_from(leaf.height()).unwrap_or(0));
                }

                // Check if there's a new upgrade certificate available.
                if let Some(cert) = leaf.upgrade_certificate() {
                    if leaf.upgrade_certificate() != *existing_upgrade_cert {
                        if cert.data.decide_by < view_number {
                            warn!("Failed to decide an upgrade certificate in time. Ignoring.");
                        } else {
                            info!("Reached decide on upgrade certificate: {:?}", cert);
                            res.decided_upgrade_cert = Some(cert.clone());
                        }
                    }
                }
                // If the block payload is available for this leaf, include it in
                // the leaf chain that we send to the client.
                if let Some(encoded_txns) =
                    consensus_reader.saved_payloads().get(&leaf.view_number())
                {
                    let payload =
                        BlockPayload::from_bytes(encoded_txns, leaf.block_header().metadata());

                    leaf.fill_block_payload_unchecked(payload);
                }

                // Get the VID share at the leaf's view number, corresponding to our key
                // (if one exists)
                let vid_share = consensus_reader
                    .vid_shares()
                    .get(&leaf.view_number())
                    .unwrap_or(&HashMap::new())
                    .get(public_key)
                    .cloned()
                    .map(|prop| prop.data);

                // Add our data into a new `LeafInfo`
                res.leaf_views.push(LeafInfo::new(
                    leaf.clone(),
                    Arc::clone(&state),
                    delta.clone(),
                    vid_share,
                ));
                res.leaves_decided.push(leaf.clone());
                if let Some(ref payload) = leaf.block_payload() {
                    res.included_txns = Some(
                        payload
                            .transaction_commitments(leaf.block_header().metadata())
                            .into_iter()
                            .collect::<HashSet<_>>(),
                    );
                }
            }
            true
        },
    ) {
        debug!("Leaf ascension failed; error={e}");
    }

    res
}

/// Handle `QuorumProposalValidated` event content and submit a proposal if possible.
#[allow(clippy::too_many_lines)]
#[cfg(not(feature = "dependency-tasks"))]
pub async fn handle_quorum_proposal_validated<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    proposal: &QuorumProposal<TYPES>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut ConsensusTaskState<TYPES, I>,
) -> Result<()> {
    let view = proposal.view_number();
    #[cfg(not(feature = "dependency-tasks"))]
    {
        task_state.current_proposal = Some(proposal.clone());
    }

    let res = decide_from_proposal(
        proposal,
        Arc::clone(&task_state.consensus),
        &task_state.decided_upgrade_cert,
        &task_state.public_key,
    )
    .await;

    if let Some(cert) = res.decided_upgrade_cert {
        task_state.decided_upgrade_cert = Some(cert.clone());

        let mut decided_certificate_lock = task_state.decided_upgrade_certificate.write().await;
        *decided_certificate_lock = Some(cert.clone());
        drop(decided_certificate_lock);
        let _ = event_stream
            .broadcast(Arc::new(HotShotEvent::UpgradeDecided(cert.clone())))
            .await;
    }

    let mut consensus = task_state.consensus.write().await;
    if let Some(new_locked_view) = res.new_locked_view_number {
        if let Err(e) = consensus.update_locked_view(new_locked_view) {
            tracing::trace!("{e:?}");
        }
    }

    drop(consensus);

    #[cfg(not(feature = "dependency-tasks"))]
    {
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
        task_state.spawn_vote_task(view, event_stream.clone()).await;
        if should_propose {
            debug!(
                "Attempting to publish proposal after voting; now in view: {}",
                *new_view
            );
            if let Err(e) = task_state
                .publish_proposal(new_view, event_stream.clone())
                .await
            {
                debug!("Failed to propose; error = {e:?}");
            };
        }
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
            &event_stream,
        )
        .await;
        debug!("decide send succeeded");
    }

    Ok(())
}

/// Private key, latest decided upgrade certificate, committee membership, and event stream, for
/// sending the vote.
#[cfg(not(feature = "dependency-tasks"))]
type VoteInfo<TYPES> = (
    <<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    Option<UpgradeCertificate<TYPES>>,
    Arc<<TYPES as NodeType>::Membership>,
    Sender<Arc<HotShotEvent<TYPES>>>,
);

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[allow(unused_variables)]
#[cfg(not(feature = "dependency-tasks"))]
/// Check if we are able to vote, like whether the proposal is valid,
/// whether we have DAC and VID share, and if so, vote.
pub async fn update_state_and_vote_if_able<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    cur_view: TYPES::Time,
    proposal: QuorumProposal<TYPES>,
    public_key: TYPES::SignatureKey,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    storage: Arc<RwLock<I::Storage>>,
    quorum_membership: Arc<TYPES::Membership>,
    instance_state: Arc<TYPES::InstanceState>,
    vote_info: VoteInfo<TYPES>,
    version: Version,
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

    if let Some(upgrade_cert) = &vote_info.1 {
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
            vote_info.3.clone(),
            Arc::clone(&quorum_membership),
            Arc::clone(&consensus),
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
    let message = if cert.is_valid_cert(vote_info.2.as_ref()) {
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
            &vote_info.0,
        ) {
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
        broadcast_event(Arc::new(HotShotEvent::QuorumVoteSend(vote)), &vote_info.3).await;
        return true;
    }
    debug!(
        "Received VID share, but couldn't find DAC cert for view {:?}",
        *proposal.view_number(),
    );
    false
}
