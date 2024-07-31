use core::time::Duration;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::{bail, ensure, Context, Result};
use async_broadcast::{broadcast, SendError, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn, async_timeout};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use chrono::Utc;
use committable::{Commitment, Committable};
use hotshot_types::{
    consensus::{ConsensusUpgradableReadLockGuard, OuterConsensus},
    data::{Leaf, QuorumProposal, ViewChangeEvidence},
    event::{Event, EventType, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        BlockPayload, ValidatedState,
    },
    utils::{Terminator, View, ViewInner},
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, warn};

use crate::{
    events::{HotShotEvent, ProposalMissing},
    request::REQUEST_TIMEOUT,
};

/// Trigger a request to the network for a proposal for a view and wait for the response
#[instrument(skip_all)]
pub(crate) async fn fetch_proposal<TYPES: NodeType>(
    view: TYPES::Time,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    quorum_membership: Arc<TYPES::Membership>,
    consensus: OuterConsensus<TYPES>,
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
    consensus: OuterConsensus<TYPES>,
    existing_upgrade_cert: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    public_key: &TYPES::SignatureKey,
) -> LeafChainTraversalOutcome<TYPES> {
    let consensus_reader = consensus.read().await;
    let existing_upgrade_cert_reader = existing_upgrade_cert.read().await;
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
                    if leaf.upgrade_certificate() != *existing_upgrade_cert_reader {
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

/// Gets the parent leaf and state from the parent of a proposal, returning an [`anyhow::Error`] if not.
#[instrument(skip_all)]
pub(crate) async fn parent_leaf_and_state<TYPES: NodeType>(
    next_proposal_view_number: TYPES::Time,
    quorum_membership: Arc<TYPES::Membership>,
    public_key: TYPES::SignatureKey,
    consensus: OuterConsensus<TYPES>,
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

/// Validate the state and safety and liveness of a proposal then emit
/// a `QuorumProposalValidated` event.
///
///
/// # Errors
/// If any validation or state update fails.
/// TODO - This should just take the QuorumProposalRecv task state after
/// we merge the dependency tasks.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[instrument(skip_all, fields(id = id, view = *proposal.data.view_number()))]
pub async fn validate_proposal_safety_and_liveness<TYPES: NodeType>(
    proposal: Proposal<TYPES, QuorumProposal<TYPES>>,
    parent_leaf: Leaf<TYPES>,
    consensus: OuterConsensus<TYPES>,
    decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    quorum_membership: Arc<TYPES::Membership>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    sender: TYPES::SignatureKey,
    event_sender: Sender<Event<TYPES>>,
    id: u64,
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

    UpgradeCertificate::validate(&proposal.data.upgrade_certificate, &quorum_membership)?;

    // Validate that the upgrade certificate is re-attached, if we saw one on the parent
    proposed_leaf
        .extends_upgrade(&parent_leaf, &decided_upgrade_certificate)
        .await?;

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

/// Validates, from a given `proposal` that the view that it is being submitted for is valid when
/// compared to `cur_view` which is the highest proposed view (so far) for the caller. If the proposal
/// is for a view that's later than expected, that the proposal includes a timeout or view sync certificate.
///
/// # Errors
/// If any validation or view number check fails.
pub fn validate_proposal_view_and_certs<TYPES: NodeType>(
    proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
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

    // Validate the proposal's signature. This should also catch if the leaf_commitment does not equal our calculated parent commitment
    proposal.validate_signature(quorum_membership)?;

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

/// Constant which tells [`update_view`] to send a view change event when called.
pub(crate) const SEND_VIEW_CHANGE_EVENT: bool = true;

/// Constant which tells `update_view` to not send a view change event when called.
pub const DONT_SEND_VIEW_CHANGE_EVENT: bool = false;

/// Update the view if it actually changed, takes a mutable reference to the `cur_view` and the
/// `timeout_task` which are updated during the operation of the function.
///
/// # Errors
/// Returns an [`anyhow::Error`] when the new view is not greater than the current view.
/// TODO: Remove args when we merge dependency tasks.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn update_view<TYPES: NodeType>(
    new_view: TYPES::Time,
    event_stream: &Sender<Arc<HotShotEvent<TYPES>>>,
    timeout: u64,
    consensus: OuterConsensus<TYPES>,
    cur_view: &mut TYPES::Time,
    cur_view_time: &mut i64,
    timeout_task: &mut JoinHandle<()>,
    output_event_stream: &Sender<Event<TYPES>>,
    send_view_change_event: bool,
    is_old_view_leader: bool,
) -> Result<()> {
    ensure!(
        new_view > *cur_view,
        "New view is not greater than our current view"
    );

    let old_view = *cur_view;

    debug!("Updating view from {} to {}", *old_view, *new_view);

    if *old_view / 100 != *new_view / 100 {
        // TODO (https://github.com/EspressoSystems/HotShot/issues/2296):
        // switch to info! when INFO logs become less cluttered
        error!("Progress: entered view {:>6}", *new_view);
    }

    *cur_view = new_view;

    // The next view is just the current view + 1
    let next_view = *cur_view + 1;

    if send_view_change_event {
        futures::join! {
            broadcast_event(Arc::new(HotShotEvent::ViewChange(new_view)), event_stream),
            broadcast_event(
                Event {
                    view_number: old_view,
                    event: EventType::ViewFinished {
                        view_number: old_view,
                    },
                },
                output_event_stream,
            )
        };
    }

    // Spawn a timeout task if we did actually update view
    let new_timeout_task = async_spawn({
        let stream = event_stream.clone();
        // Nuance: We timeout on the view + 1 here because that means that we have
        // not seen evidence to transition to this new view
        let view_number = next_view;
        let timeout = Duration::from_millis(timeout);
        async move {
            async_sleep(timeout).await;
            broadcast_event(
                Arc::new(HotShotEvent::Timeout(TYPES::Time::new(*view_number))),
                &stream,
            )
            .await;
        }
    });

    // cancel the old timeout task
    cancel_task(std::mem::replace(timeout_task, new_timeout_task)).await;

    let consensus = consensus.upgradable_read().await;
    consensus
        .metrics
        .current_view
        .set(usize::try_from(cur_view.u64()).unwrap());
    let new_view_time = Utc::now().timestamp();
    if is_old_view_leader {
        #[allow(clippy::cast_precision_loss)]
        consensus
            .metrics
            .view_duration_as_leader
            .add_point((new_view_time - *cur_view_time) as f64);
    }
    *cur_view_time = new_view_time;

    // Do the comparison before the subtraction to avoid potential overflow, since
    // `last_decided_view` may be greater than `cur_view` if the node is catching up.
    if usize::try_from(cur_view.u64()).unwrap()
        > usize::try_from(consensus.last_decided_view().u64()).unwrap()
    {
        consensus.metrics.number_of_views_since_last_decide.set(
            usize::try_from(cur_view.u64()).unwrap()
                - usize::try_from(consensus.last_decided_view().u64()).unwrap(),
        );
    }
    let mut consensus = ConsensusUpgradableReadLockGuard::upgrade(consensus).await;
    if let Err(e) = consensus.update_view(new_view) {
        tracing::trace!("{e:?}");
    }
    tracing::trace!("View updated successfully");

    Ok(())
}

/// Cancel a task
pub async fn cancel_task<T>(task: JoinHandle<T>) {
    #[cfg(async_executor_impl = "async-std")]
    task.cancel().await;
    #[cfg(async_executor_impl = "tokio")]
    task.abort();
}

/// Helper function to send events and log errors
pub async fn broadcast_event<E: Clone + std::fmt::Debug>(event: E, sender: &Sender<E>) {
    match sender.broadcast_direct(event).await {
        Ok(None) => (),
        Ok(Some(overflowed)) => {
            tracing::error!(
                "Event sender queue overflow, Oldest event removed form queue: {:?}",
                overflowed
            );
        }
        Err(SendError(e)) => {
            tracing::warn!(
                "Event: {:?}\n Sending failed, event stream probably shutdown",
                e
            );
        }
    }
}

/// Utilities to print anyhow logs.
pub trait AnyhowTracing {
    /// Print logs as debug
    fn err_as_debug(self);
}

impl<T> AnyhowTracing for anyhow::Result<T> {
    fn err_as_debug(self) {
        let _ = self.inspect_err(|e| tracing::debug!("{}", format!("{:?}", e)));
    }
}
