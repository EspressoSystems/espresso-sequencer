use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::{bail, Context, Result};
use async_broadcast::Sender;
use chrono::Utc;
use committable::Commitment;
use hotshot_types::{
    data::{Leaf, QuorumProposal},
    event::{Event, EventType, LeafInfo},
    simple_certificate::QuorumCertificate,
    traits::{
        block_contents::BlockHeader,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        BlockPayload,
    },
    vote::HasViewNumber,
};
use tracing::debug;

use super::QuorumProposalTaskState;
use crate::{events::HotShotEvent, helpers::broadcast_event};

/// Helper type to give names and to the output values of the leaf chain traversal operation.
#[derive(Debug)]
struct LeafChainTraversalOutcome<TYPES: NodeType> {
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
    pub included_txns: HashSet<Commitment<<TYPES as NodeType>::Transaction>>,
    // TODO - add upgrade cert here and fill
}

impl<TYPES: NodeType + Default> Default for LeafChainTraversalOutcome<TYPES> {
    /// The default method for this type is to set all of the returned values to `None`.
    fn default() -> Self {
        Self {
            new_locked_view_number: None,
            new_decided_view_number: None,
            new_decide_qc: None,
            leaf_views: Vec::new(),
            leaves_decided: Vec::new(),
            included_txns: HashSet::new(),
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
async fn visit_leaf_chain<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    proposal: &QuorumProposal<TYPES>,
    task_state: &QuorumProposalTaskState<TYPES, I>,
) -> Result<LeafChainTraversalOutcome<TYPES>> {
    let proposal_view_number = proposal.view_number();
    let proposal_parent_view_number = proposal.justify_qc.view_number();

    // This is the output return type object whose members will be mutated as we traverse.
    let mut ret = LeafChainTraversalOutcome::default();

    // Are these views consecutive (1-chain)
    if proposal_parent_view_number + 1 != proposal_view_number {
        // Since they aren't we can return early before we do anything else.
        return Ok(ret);
    }

    // Unpacking here prevents the need to endlessly call the function. These values don't change during
    // the execution of this code.
    let consensus_reader = task_state.consensus.read().await;
    let validated_state_map = consensus_reader.validated_state_map();
    let saved_leaves = consensus_reader.saved_leaves();
    let last_decided_view = consensus_reader.last_decided_view();
    let saved_payloads = consensus_reader.saved_payloads();
    let vid_shares = consensus_reader.vid_shares();

    // We are in at least a 1-chain, so we start from here.
    let mut current_chain_length: usize = 1;

    // Get the state so we can traverse the chain to see if we have a 2 or 3 chain.
    let mut view_number = proposal_parent_view_number;

    // The most recently seen view number (the view number of the last leaf we saw).
    let mut last_seen_view_number = proposal_view_number;

    while let Some(leaf_state) = validated_state_map.get(&view_number) {
        let leaf_commitment = leaf_state
            .leaf_commitment()
            .context("Failed to find the leaf commitment")?;
        let leaf = saved_leaves
            .get(&leaf_commitment)
            .context("Failed to find the saved leaf")?;

        // These are all just checks to make sure we have what we need to proceed.
        let current_leaf_view_number = leaf.view_number();

        if let (Some(state), delta) = leaf_state.state_and_delta() {
            // Exit if we've reached the last anchor view.
            if current_leaf_view_number == last_decided_view {
                return Ok(ret);
            }

            // IMPORTANT: This is the logic from the paper, and is the most critical part of this function.
            if ret.new_decided_view_number.is_none() {
                // Does this leaf extend the chain?
                if last_seen_view_number == leaf.view_number() + 1 {
                    last_seen_view_number = leaf.view_number();
                    current_chain_length += 1;

                    // We've got a 2 chain, update the locked view.
                    if current_chain_length == 2 {
                        ret.new_locked_view_number = Some(leaf.view_number());

                        // The next leaf in the chain, if there is one, is decided, so this
                        // leaf's justify_qc would become the QC for the decided chain.
                        ret.new_decide_qc = Some(leaf.justify_qc().clone());
                    } else if current_chain_length == 3 {
                        // We've got the 3-chain, which means we can successfully decide on this leaf.
                        ret.new_decided_view_number = Some(leaf.view_number());
                    }
                } else {
                    // Bail out with empty values, but this is not necessarily an error, but we don't have a
                    // new chain extension.
                    return Ok(ret);
                }
            }

            // If we got a 3-chain, we can start our state updates, garbage collection, etc
            if let Some(decided_view) = ret.new_decided_view_number {
                let mut leaf = leaf.clone();
                if leaf.view_number() == decided_view {
                    consensus_reader
                        .metrics
                        .last_synced_block_height
                        .set(usize::try_from(leaf.height()).unwrap_or(0));
                }

                // TODO - Upgrade certificates
                // if let Some(cert) = leaf.upgrade_certificate() {
                //     ensure!(
                //         cert.data.decide_by >= proposal_view_number,
                //         "Failed to decide an upgrade certificate in time. Ignoring."
                //     );
                //     task_state.decided_upgrade_cert = Some(cert.clone());
                // }
                // If the block payload is available for this leaf, include it in
                // the leaf chain that we send to the client.
                if let Some(encoded_txns) = saved_payloads.get(&leaf.view_number()) {
                    let payload =
                        BlockPayload::from_bytes(encoded_txns, leaf.block_header().metadata());

                    leaf.fill_block_payload_unchecked(payload);
                }

                let vid_share = vid_shares
                    .get(&leaf.view_number())
                    .unwrap_or(&HashMap::new())
                    .get(&task_state.public_key)
                    .cloned()
                    .map(|prop| prop.data);

                // Add our data into a new `LeafInfo`
                ret.leaf_views.push(LeafInfo::new(
                    leaf.clone(),
                    Arc::clone(&state),
                    delta.clone(),
                    vid_share,
                ));
                ret.leaves_decided.push(leaf.clone());
                if let Some(ref payload) = leaf.block_payload() {
                    for txn in payload.transaction_commitments(leaf.block_header().metadata()) {
                        ret.included_txns.insert(txn);
                    }
                }
            }
        } else {
            bail!(
                    "Validated state and delta do not exist for the leaf for view {current_leaf_view_number:?}"
                )
        };

        // Move on to the next leaf at the end.
        view_number = leaf.justify_qc().view_number();
    }

    Ok(ret)
}

/// Handles the `QuorumProposalValidated` event.
pub(crate) async fn handle_quorum_proposal_validated<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
>(
    proposal: &QuorumProposal<TYPES>,
    sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    task_state: &mut QuorumProposalTaskState<TYPES, I>,
) -> Result<()> {
    let LeafChainTraversalOutcome {
        new_locked_view_number,
        new_decided_view_number,
        new_decide_qc,
        leaf_views,
        leaves_decided,
        included_txns,
    } = visit_leaf_chain(proposal, task_state).await?;

    let included_txns = if new_decided_view_number.is_some() {
        included_txns
    } else {
        HashSet::new()
    };

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

    // TODO - update decided upgrade cert

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
            *task_state.latest_proposed_view - consensus_writer.last_decided_view().u64();
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
                    block_size: Some(included_txns.len().try_into().unwrap()),
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
