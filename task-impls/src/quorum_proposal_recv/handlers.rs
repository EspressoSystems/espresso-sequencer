// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#![allow(dead_code)]

use std::sync::Arc;

use async_broadcast::{broadcast, Receiver, Sender};
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLockUpgradableReadGuard;
use committable::Committable;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{Leaf, QuorumProposal},
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::{
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
        ValidatedState,
    },
    utils::{View, ViewInner},
    vote::{Certificate, HasViewNumber},
};
use tracing::instrument;
use utils::anytrace::*;

use super::{QuorumProposalRecvTaskState, ValidationInfo};
use crate::{
    events::HotShotEvent,
    helpers::{
        broadcast_event, fetch_proposal, validate_proposal_safety_and_liveness,
        validate_proposal_view_and_certs,
    },
    quorum_proposal_recv::{UpgradeLock, Versions},
};

/// Update states in the event that the parent state is not found for a given `proposal`.
#[instrument(skip_all)]
async fn validate_proposal_liveness<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    event_sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    validation_info: &ValidationInfo<TYPES, I, V>,
) -> Result<()> {
    let view_number = proposal.data.view_number();
    let mut consensus_writer = validation_info.consensus.write().await;

    let leaf = Leaf::from_quorum_proposal(&proposal.data);

    let state = Arc::new(
        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(&proposal.data.block_header),
    );
    let view = View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(&validation_info.upgrade_lock).await,
            state,
            delta: None, // May be updated to `Some` in the vote task.
        },
    };

    if let Err(e) = consensus_writer.update_validated_state_map(view_number, view.clone()) {
        tracing::trace!("{e:?}");
    }
    consensus_writer
        .update_saved_leaves(leaf.clone(), &validation_info.upgrade_lock)
        .await;

    if let Err(e) = validation_info
        .storage
        .write()
        .await
        .update_undecided_state(
            consensus_writer.saved_leaves().clone(),
            consensus_writer.validated_state_map().clone(),
        )
        .await
    {
        tracing::warn!("Couldn't store undecided state.  Error: {:?}", e);
    }

    let liveness_check =
        proposal.data.justify_qc.clone().view_number() > consensus_writer.locked_view();

    drop(consensus_writer);

    // Broadcast that we've updated our consensus state so that other tasks know it's safe to grab.
    broadcast_event(
        HotShotEvent::ValidatedStateUpdated(view_number, view).into(),
        event_sender,
    )
    .await;

    if !liveness_check {
        bail!("Quorum Proposal failed the liveness check");
    }

    Ok(())
}

/// Spawn a task which will fire a request to get a proposal, and store it.
#[allow(clippy::too_many_arguments)]
fn spawn_fetch_proposal<TYPES: NodeType, V: Versions>(
    view: TYPES::View,
    event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    membership: Arc<TYPES::Membership>,
    consensus: OuterConsensus<TYPES>,
    sender_public_key: TYPES::SignatureKey,
    sender_private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    upgrade_lock: UpgradeLock<TYPES, V>,
) {
    async_spawn(async move {
        let lock = upgrade_lock;

        let _ = fetch_proposal(
            view,
            event_sender,
            event_receiver,
            membership,
            consensus,
            sender_public_key,
            sender_private_key,
            &lock,
        )
        .await;
    });
}

/// Handles the `QuorumProposalRecv` event by first validating the cert itself for the view, and then
/// updating the states, which runs when the proposal cannot be found in the internal state map.
///
/// This code can fail when:
/// - The justify qc is invalid.
/// - The task is internally inconsistent.
/// - The sequencer storage update fails.
#[allow(clippy::too_many_lines)]
#[instrument(skip_all)]
pub(crate) async fn handle_quorum_proposal_recv<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    quorum_proposal_sender_key: &TYPES::SignatureKey,
    event_sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: &Receiver<Arc<HotShotEvent<TYPES>>>,
    validation_info: ValidationInfo<TYPES, I, V>,
) -> Result<()> {
    let quorum_proposal_sender_key = quorum_proposal_sender_key.clone();

    validate_proposal_view_and_certs(proposal, &validation_info)
        .await
        .context(warn!("Failed to validate proposal view or attached certs"))?;

    let view_number = proposal.data.view_number();
    let justify_qc = proposal.data.justify_qc.clone();

    if !justify_qc
        .is_valid_cert(
            validation_info.quorum_membership.as_ref(),
            validation_info.cur_epoch,
            &validation_info.upgrade_lock,
        )
        .await
    {
        let consensus_reader = validation_info.consensus.read().await;
        consensus_reader.metrics.invalid_qc.update(1);
        bail!("Invalid justify_qc in proposal for view {}", *view_number);
    }

    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalPreliminarilyValidated(
            proposal.clone(),
        )),
        event_sender,
    )
    .await;

    // Get the parent leaf and state.
    let parent_leaf = validation_info
        .consensus
        .read()
        .await
        .saved_leaves()
        .get(&justify_qc.data.leaf_commit)
        .cloned();

    if parent_leaf.is_none() {
        spawn_fetch_proposal(
            justify_qc.view_number(),
            event_sender.clone(),
            event_receiver.clone(),
            Arc::clone(&validation_info.quorum_membership),
            OuterConsensus::new(Arc::clone(&validation_info.consensus.inner_consensus)),
            // Note that we explicitly use the node key here instead of the provided key in the signature.
            // This is because the key that we receive is for the prior leader, so the payload would be routed
            // incorrectly.
            validation_info.public_key.clone(),
            validation_info.private_key.clone(),
            validation_info.upgrade_lock.clone(),
        );
    }
    let consensus_reader = validation_info.consensus.read().await;

    let parent = match parent_leaf {
        Some(leaf) => {
            if let (Some(state), _) = consensus_reader.state_and_delta(leaf.view_number()) {
                Some((leaf, Arc::clone(&state)))
            } else {
                bail!("Parent state not found! Consensus internally inconsistent");
            }
        }
        None => None,
    };

    if justify_qc.view_number() > consensus_reader.high_qc().view_number {
        if let Err(e) = validation_info
            .storage
            .write()
            .await
            .update_high_qc(justify_qc.clone())
            .await
        {
            bail!("Failed to store High QC, not voting; error = {:?}", e);
        }
    }
    drop(consensus_reader);

    let mut consensus_writer = validation_info.consensus.write().await;
    if let Err(e) = consensus_writer.update_high_qc(justify_qc.clone()) {
        tracing::trace!("{e:?}");
    }
    drop(consensus_writer);

    broadcast_event(
        HotShotEvent::HighQcUpdated(justify_qc.clone()).into(),
        event_sender,
    )
    .await;

    let Some((parent_leaf, _parent_state)) = parent else {
        tracing::warn!(
            "Proposal's parent missing from storage with commitment: {:?}",
            justify_qc.data.leaf_commit
        );
        validate_proposal_liveness(proposal, event_sender, &validation_info).await?;
        broadcast_event(
            Arc::new(HotShotEvent::ViewChange(view_number)),
            event_sender,
        )
        .await;
        return Ok(());
    };

    // Validate the proposal
    validate_proposal_safety_and_liveness::<TYPES, I, V>(
        proposal.clone(),
        parent_leaf,
        &validation_info,
        event_sender.clone(),
        quorum_proposal_sender_key,
    )
    .await?;
    broadcast_event(
        Arc::new(HotShotEvent::ViewChange(view_number)),
        event_sender,
    )
    .await;

    Ok(())
}
