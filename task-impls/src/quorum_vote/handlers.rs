#![cfg(feature = "dependency-tasks")]

use std::sync::Arc;

use anyhow::Result;
use async_broadcast::Sender;
use chrono::Utc;
use hotshot_types::{
    consensus::OuterConsensus,
    data::QuorumProposal,
    event::{Event, EventType},
    traits::node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    vote::HasViewNumber,
};
use tracing::{debug, instrument};

use super::QuorumVoteTaskState;
use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, decide_from_proposal, LeafChainTraversalOutcome},
};

/// Handles the `QuorumProposalValidated` event.
#[instrument(skip_all)]
pub(crate) async fn handle_quorum_proposal_validated<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
>(
    proposal: &QuorumProposal<TYPES>,
    sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut QuorumVoteTaskState<TYPES, I>,
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
        Arc::clone(&task_state.decided_upgrade_certificate),
        &task_state.public_key,
    )
    .await;

    if let Some(cert) = decided_upgrade_cert.clone() {
        let mut decided_certificate_lock = task_state.decided_upgrade_certificate.write().await;
        *decided_certificate_lock = Some(cert.clone());
        drop(decided_certificate_lock);
        let _ = sender
            .broadcast(Arc::new(HotShotEvent::UpgradeDecided(cert.clone())))
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

        debug!(
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
        debug!("Successfully sent decide event");
    }

    Ok(())
}
