// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::sync::Arc;

use async_broadcast::{InactiveReceiver, Sender};
use async_lock::RwLock;
use chrono::Utc;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{Leaf, QuorumProposal, VidDisperseShare},
    event::{Event, EventType},
    message::{Proposal, UpgradeLock},
    simple_vote::{QuorumData, QuorumVote},
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
        ValidatedState,
    },
    utils::{View, ViewInner},
    vote::HasViewNumber,
};
use tracing::instrument;
use utils::anytrace::*;

use super::QuorumVoteTaskState;
use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, decide_from_proposal, fetch_proposal, LeafChainTraversalOutcome},
    quorum_vote::Versions,
};

/// Handles the `QuorumProposalValidated` event.
#[instrument(skip_all, fields(id = task_state.id, view = *proposal.view_number))]
pub(crate) async fn handle_quorum_proposal_validated<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: &QuorumProposal<TYPES>,
    sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut QuorumVoteTaskState<TYPES, I, V>,
) -> Result<()> {
    let LeafChainTraversalOutcome {
        new_locked_view_number,
        new_decided_view_number,
        new_decide_qc,
        leaf_views,
        leaves_decided,
        included_txns,
        decided_upgrade_cert,
    } = decide_from_proposal(
        proposal,
        OuterConsensus::new(Arc::clone(&task_state.consensus.inner_consensus)),
        Arc::clone(&task_state.upgrade_lock.decided_upgrade_certificate),
        &task_state.public_key,
    )
    .await;

    if let Some(cert) = decided_upgrade_cert.clone() {
        let mut decided_certificate_lock = task_state
            .upgrade_lock
            .decided_upgrade_certificate
            .write()
            .await;
        *decided_certificate_lock = Some(cert.clone());
        drop(decided_certificate_lock);

        let _ = task_state
            .storage
            .write()
            .await
            .update_decided_upgrade_certificate(Some(cert.clone()))
            .await;
    }

    let mut consensus_writer = task_state.consensus.write().await;
    if let Some(locked_view_number) = new_locked_view_number {
        // Broadcast the locked view update.
        broadcast_event(
            HotShotEvent::LockedViewUpdated(locked_view_number).into(),
            sender,
        )
        .await;

        consensus_writer.update_locked_view(locked_view_number)?;
    }

    #[allow(clippy::cast_precision_loss)]
    if let Some(decided_view_number) = new_decided_view_number {
        // Bring in the cleanup crew. When a new decide is indeed valid, we need to clear out old memory.

        let old_decided_view = consensus_writer.last_decided_view();
        consensus_writer.collect_garbage(old_decided_view, decided_view_number);

        // Set the new decided view.
        consensus_writer.update_last_decided_view(decided_view_number)?;
        broadcast_event(
            HotShotEvent::LastDecidedViewUpdated(decided_view_number).into(),
            sender,
        )
        .await;

        consensus_writer
            .metrics
            .last_decided_time
            .set(Utc::now().timestamp().try_into().unwrap());
        consensus_writer.metrics.invalid_qc.set(0);
        consensus_writer
            .metrics
            .last_decided_view
            .set(usize::try_from(consensus_writer.last_decided_view().u64()).unwrap());
        let cur_number_of_views_per_decide_event =
            *proposal.view_number() - consensus_writer.last_decided_view().u64();
        consensus_writer
            .metrics
            .number_of_views_per_decide_event
            .add_point(cur_number_of_views_per_decide_event as f64);

        tracing::debug!(
            "Sending Decide for view {:?}",
            consensus_writer.last_decided_view()
        );

        // We don't need to hold this while we broadcast
        drop(consensus_writer);

        // First, send an update to everyone saying that we've reached a decide
        broadcast_event(
            Event {
                view_number: decided_view_number,
                event: EventType::Decide {
                    leaf_chain: Arc::new(leaf_views),
                    // This is never *not* none if we've reached a new decide, so this is safe to unwrap.
                    qc: Arc::new(new_decide_qc.unwrap()),
                    block_size: included_txns.map(|txns| txns.len().try_into().unwrap()),
                },
            },
            &task_state.output_event_stream,
        )
        .await;

        broadcast_event(Arc::new(HotShotEvent::LeafDecided(leaves_decided)), sender).await;
        tracing::debug!("Successfully sent decide event");
    }

    Ok(())
}

/// Updates the shared consensus state with the new voting data.
#[instrument(skip_all, target = "VoteDependencyHandle", fields(view = *view_number))]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn update_shared_state<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    consensus: OuterConsensus<TYPES>,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    receiver: InactiveReceiver<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    upgrade_lock: UpgradeLock<TYPES, V>,
    view_number: TYPES::View,
    instance_state: Arc<TYPES::InstanceState>,
    storage: Arc<RwLock<I::Storage>>,
    proposed_leaf: &Leaf<TYPES>,
    vid_share: &Proposal<TYPES, VidDisperseShare<TYPES>>,
) -> Result<()> {
    let justify_qc = &proposed_leaf.justify_qc();

    // Justify qc's leaf commitment should be the same as the parent's leaf commitment.
    let mut maybe_parent = consensus
        .read()
        .await
        .saved_leaves()
        .get(&justify_qc.data.leaf_commit)
        .cloned();
    maybe_parent = match maybe_parent {
        Some(p) => Some(p),
        None => fetch_proposal(
            justify_qc.view_number(),
            sender.clone(),
            receiver.activate_cloned(),
            Arc::clone(&quorum_membership),
            OuterConsensus::new(Arc::clone(&consensus.inner_consensus)),
            public_key.clone(),
            private_key.clone(),
            &upgrade_lock,
        )
        .await
        .ok(),
    };
    let parent = maybe_parent.context(info!(
        "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
        justify_qc.data.leaf_commit,
        proposed_leaf.view_number(),
    ))?;
    let consensus_reader = consensus.read().await;

    let (Some(parent_state), _) = consensus_reader.state_and_delta(parent.view_number()) else {
        bail!("Parent state not found! Consensus internally inconsistent");
    };

    drop(consensus_reader);

    let version = upgrade_lock.version(view_number).await?;

    let (validated_state, state_delta) = parent_state
        .validate_and_apply_header(
            &instance_state,
            &parent,
            &proposed_leaf.block_header().clone(),
            vid_share.data.common.clone(),
            version,
        )
        .await
        .wrap()
        .context(warn!("Block header doesn't extend the proposal!"))?;

    let state = Arc::new(validated_state);
    let delta = Arc::new(state_delta);

    // Now that we've rounded everyone up, we need to update the shared state and broadcast our events.
    // We will defer broadcast until all states are updated to avoid holding onto the lock during a network call.
    let mut consensus_writer = consensus.write().await;

    let view = View {
        view_inner: ViewInner::Leaf {
            leaf: proposed_leaf.commit(&upgrade_lock).await,
            state: Arc::clone(&state),
            delta: Some(Arc::clone(&delta)),
        },
    };
    if let Err(e) =
        consensus_writer.update_validated_state_map(proposed_leaf.view_number(), view.clone())
    {
        tracing::trace!("{e:?}");
    }
    consensus_writer
        .update_saved_leaves(proposed_leaf.clone(), &upgrade_lock)
        .await;

    // Kick back our updated structures for downstream usage.
    let new_leaves = consensus_writer.saved_leaves().clone();
    let new_state = consensus_writer.validated_state_map().clone();
    drop(consensus_writer);

    // Broadcast now that the lock is dropped.
    broadcast_event(
        HotShotEvent::ValidatedStateUpdated(proposed_leaf.view_number(), view).into(),
        &sender,
    )
    .await;

    // Send the new state up to the sequencer.
    storage
        .write()
        .await
        .update_undecided_state(new_leaves, new_state)
        .await
        .wrap()
        .context(error!("Failed to update undecided state"))?;

    Ok(())
}

/// Submits the `QuorumVoteSend` event if all the dependencies are met.
#[instrument(skip_all, fields(name = "Submit quorum vote", level = "error"))]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn submit_vote<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    upgrade_lock: UpgradeLock<TYPES, V>,
    view_number: TYPES::View,
    epoch_number: TYPES::Epoch,
    storage: Arc<RwLock<I::Storage>>,
    leaf: Leaf<TYPES>,
    vid_share: Proposal<TYPES, VidDisperseShare<TYPES>>,
) -> Result<()> {
    ensure!(
        quorum_membership.has_stake(&public_key, epoch_number),
        info!(
            "We were not chosen for quorum committee on {:?}",
            view_number
        )
    );

    // Create and send the vote.
    let vote = QuorumVote::<TYPES>::create_signed_vote(
        QuorumData {
            leaf_commit: leaf.commit(&upgrade_lock).await,
        },
        view_number,
        &public_key,
        &private_key,
        &upgrade_lock,
    )
    .await
    .wrap()
    .context(error!("Failed to sign vote. This should never happen."))?;
    tracing::debug!(
        "sending vote to next quorum leader {:?}",
        vote.view_number() + 1
    );
    // Add to the storage.
    storage
        .write()
        .await
        .append_vid(&vid_share)
        .await
        .wrap()
        .context(error!("Failed to store VID share"))?;
    broadcast_event(Arc::new(HotShotEvent::QuorumVoteSend(vote)), &sender).await;

    Ok(())
}
