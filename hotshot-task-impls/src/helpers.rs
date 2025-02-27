// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use async_broadcast::{Receiver, SendError, Sender};
use async_lock::RwLock;
use committable::{Commitment, Committable};
use either::Either;
use hotshot_task::dependency::{Dependency, EventDependency};
use hotshot_types::{
    consensus::OuterConsensus,
    data::{Leaf2, QuorumProposalWrapper, ViewChangeEvidence2},
    event::{Event, EventType, LeafInfo},
    message::{Proposal, UpgradeLock},
    request_response::ProposalRequestPayload,
    simple_certificate::{NextEpochQuorumCertificate2, QuorumCertificate2, UpgradeCertificate},
    simple_vote::HasEpoch,
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType, Versions},
        signature_key::SignatureKey,
        BlockPayload, ValidatedState,
    },
    utils::{
        epoch_from_block_number, is_epoch_root, is_last_block_in_epoch,
        option_epoch_from_block_number, Terminator, View, ViewInner,
    },
    vote::{Certificate, HasViewNumber},
};
use hotshot_utils::anytrace::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::timeout;
use tracing::instrument;

use crate::{events::HotShotEvent, quorum_proposal_recv::ValidationInfo, request::REQUEST_TIMEOUT};

/// Trigger a request to the network for a proposal for a view and wait for the response or timeout.
#[instrument(skip_all)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn fetch_proposal<TYPES: NodeType, V: Versions>(
    view_number: TYPES::View,
    event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    membership: Arc<RwLock<TYPES::Membership>>,
    consensus: OuterConsensus<TYPES>,
    sender_public_key: TYPES::SignatureKey,
    sender_private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    upgrade_lock: &UpgradeLock<TYPES, V>,
    epoch_height: u64,
) -> Result<(Leaf2<TYPES>, View<TYPES>)> {
    // We need to be able to sign this request before submitting it to the network. Compute the
    // payload first.
    let signed_proposal_request = ProposalRequestPayload {
        view_number,
        key: sender_public_key,
    };

    // Finally, compute the signature for the payload.
    let signature = TYPES::SignatureKey::sign(
        &sender_private_key,
        signed_proposal_request.commit().as_ref(),
    )
    .wrap()
    .context(error!("Failed to sign proposal. This should never happen."))?;

    // First, broadcast that we need a proposal to the current leader
    broadcast_event(
        HotShotEvent::QuorumProposalRequestSend(signed_proposal_request, signature).into(),
        &event_sender,
    )
    .await;

    let mem = Arc::clone(&membership);
    // Make a background task to await the arrival of the event data.
    let Ok(Some(proposal)) =
        // We want to explicitly timeout here so we aren't waiting around for the data.
        timeout(REQUEST_TIMEOUT, async move {
            // We want to iterate until the proposal is not None, or until we reach the timeout.
            let mut proposal = None;
            while proposal.is_none() {
                // First, capture the output from the event dependency
                let event = EventDependency::new(
                    event_receiver.clone(),
                    Box::new(move |event| {
                        let event = event.as_ref();
                        if let HotShotEvent::QuorumProposalResponseRecv(
                            quorum_proposal,
                        ) = event
                        {
                            quorum_proposal.data.view_number() == view_number
                        } else {
                            false
                        }
                    }),
                )
                    .completed()
                    .await;

                // Then, if it's `Some`, make sure that the data is correct
                if let Some(hs_event) = event.as_ref() {
                    if let HotShotEvent::QuorumProposalResponseRecv(quorum_proposal) =
                        hs_event.as_ref()
                    {
                        // Make sure that the quorum_proposal is valid
                        let mem_reader = mem.read().await;
                        if quorum_proposal.validate_signature(&mem_reader, epoch_height).is_ok() {
                            proposal = Some(quorum_proposal.clone());
                        }

                    }
                } else {
                    // If the dep returns early return none
                    return None;
                }
            }
            proposal
        })
        .await
    else {
        bail!("Request for proposal failed");
    };

    let view_number = proposal.data.view_number();
    let justify_qc = proposal.data.justify_qc().clone();

    let justify_qc_epoch = justify_qc.data.epoch();

    let membership_reader = membership.read().await;
    let membership_stake_table = membership_reader.stake_table(justify_qc_epoch);
    let membership_success_threshold = membership_reader.success_threshold(justify_qc_epoch);
    drop(membership_reader);

    justify_qc
        .is_valid_cert(
            membership_stake_table,
            membership_success_threshold,
            upgrade_lock,
        )
        .await
        .context(|e| {
            warn!(
                "Invalid justify_qc in proposal for view {}: {}",
                *view_number, e
            )
        })?;

    let mut consensus_writer = consensus.write().await;
    let leaf = Leaf2::from_quorum_proposal(&proposal.data);
    let state = Arc::new(
        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(proposal.data.block_header()),
    );

    if let Err(e) = consensus_writer.update_leaf(leaf.clone(), Arc::clone(&state), None) {
        tracing::trace!("{e:?}");
    }
    let view = View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(),
            state,
            delta: None,
            epoch: leaf.epoch(epoch_height),
        },
    };
    Ok((leaf, view))
}

/// Handles calling add_epoch_root and sync_l1 on Membership if necessary.
async fn decide_epoch_root<TYPES: NodeType>(
    decided_leaf: &Leaf2<TYPES>,
    epoch_height: u64,
    membership: &Arc<RwLock<TYPES::Membership>>,
) {
    let decided_block_number = decided_leaf.block_header().block_number();

    // Skip if this is not the expected block.
    if epoch_height != 0 && is_epoch_root(decided_block_number, epoch_height) {
        let next_epoch_number =
            TYPES::Epoch::new(epoch_from_block_number(decided_block_number, epoch_height) + 1);

        let write_callback = {
            let membership_reader = membership.read().await;
            membership_reader
                .add_epoch_root(next_epoch_number, decided_leaf.block_header().clone())
                .await
        };

        if let Some(write_callback) = write_callback {
            let mut membership_writer = membership.write().await;
            write_callback(&mut *membership_writer);
        } else {
            // If we didn't get a write callback out of add_epoch_root, then don't bother locking and calling sync_l1
            return;
        }

        let write_callback = {
            let membership_reader = membership.read().await;
            membership_reader.sync_l1().await
        };

        if let Some(write_callback) = write_callback {
            let mut membership_writer = membership.write().await;
            write_callback(&mut *membership_writer);
        }
    }
}

/// Helper type to give names and to the output values of the leaf chain traversal operation.
#[derive(Debug)]
pub struct LeafChainTraversalOutcome<TYPES: NodeType> {
    /// The new locked view obtained from a 2 chain starting from the proposal's parent.
    pub new_locked_view_number: Option<TYPES::View>,

    /// The new decided view obtained from a 3 chain starting from the proposal's parent.
    pub new_decided_view_number: Option<TYPES::View>,

    /// The qc for the decided chain.
    pub new_decide_qc: Option<QuorumCertificate2<TYPES>>,

    /// The decided leaves with corresponding validated state and VID info.
    pub leaf_views: Vec<LeafInfo<TYPES>>,

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
            included_txns: None,
            decided_upgrade_cert: None,
        }
    }
}

/// calculate the new decided leaf chain based on the rules of HotStuff 2
///
/// # Panics
/// If the leaf chain contains no decided leaf while reaching a decided view, which should be
/// impossible.
pub async fn decide_from_proposal_2<TYPES: NodeType>(
    proposal: &QuorumProposalWrapper<TYPES>,
    consensus: OuterConsensus<TYPES>,
    existing_upgrade_cert: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    public_key: &TYPES::SignatureKey,
    with_epochs: bool,
    membership: &Arc<RwLock<TYPES::Membership>>,
) -> LeafChainTraversalOutcome<TYPES> {
    let mut res = LeafChainTraversalOutcome::default();
    let consensus_reader = consensus.read().await;
    let proposed_leaf = Leaf2::from_quorum_proposal(proposal);
    res.new_locked_view_number = Some(proposed_leaf.justify_qc().view_number());

    // If we don't have the proposals parent return early
    let Some(parent_info) = consensus_reader.parent_leaf_info(&proposed_leaf, public_key) else {
        return res;
    };
    // Get the parents parent and check if it's consecutive in view to the parent, if so we can decided
    // the grandparents view.  If not we're done.
    let Some(grand_parent_info) = consensus_reader.parent_leaf_info(&parent_info.leaf, public_key)
    else {
        return res;
    };
    if grand_parent_info.leaf.view_number() + 1 != parent_info.leaf.view_number() {
        return res;
    }
    res.new_decide_qc = Some(parent_info.leaf.justify_qc().clone());
    let decided_view_number = grand_parent_info.leaf.view_number();
    res.new_decided_view_number = Some(decided_view_number);
    // We've reached decide, now get the leaf chain all the way back to the last decided view, not including it.
    let old_anchor_view = consensus_reader.last_decided_view();
    let mut current_leaf_info = Some(grand_parent_info);
    let existing_upgrade_cert_reader = existing_upgrade_cert.read().await;
    let mut txns = HashSet::new();
    while current_leaf_info
        .as_ref()
        .is_some_and(|info| info.leaf.view_number() > old_anchor_view)
    {
        // unwrap is safe, we just checked that he option is some
        let info = &mut current_leaf_info.unwrap();
        // Check if there's a new upgrade certificate available.
        if let Some(cert) = info.leaf.upgrade_certificate() {
            if info.leaf.upgrade_certificate() != *existing_upgrade_cert_reader {
                if cert.data.decide_by < decided_view_number {
                    tracing::warn!("Failed to decide an upgrade certificate in time. Ignoring.");
                } else {
                    tracing::info!("Reached decide on upgrade certificate: {:?}", cert);
                    res.decided_upgrade_cert = Some(cert.clone());
                }
            }
        }

        res.leaf_views.push(info.clone());
        // If the block payload is available for this leaf, include it in
        // the leaf chain that we send to the client.
        if let Some(payload) = consensus_reader
            .saved_payloads()
            .get(&info.leaf.view_number())
        {
            info.leaf
                .fill_block_payload_unchecked(payload.as_ref().payload.clone());
        }

        if let Some(ref payload) = info.leaf.block_payload() {
            for txn in payload.transaction_commitments(info.leaf.block_header().metadata()) {
                txns.insert(txn);
            }
        }

        current_leaf_info = consensus_reader.parent_leaf_info(&info.leaf, public_key);
    }

    if !txns.is_empty() {
        res.included_txns = Some(txns);
    }

    if with_epochs && res.new_decided_view_number.is_some() {
        let epoch_height = consensus_reader.epoch_height;
        drop(consensus_reader);

        if let Some(decided_leaf_info) = res.leaf_views.last() {
            decide_epoch_root(&decided_leaf_info.leaf, epoch_height, membership).await;
        } else {
            tracing::info!("No decided leaf while a view has been decided.");
        }
    }

    res
}

/// Ascends the leaf chain by traversing through the parent commitments of the proposal. We begin
/// by obtaining the parent view, and if we are in a chain (i.e. the next view from the parent is
/// one view newer), then we begin attempting to form the chain. This is a direct impl from
/// [HotStuff](https://arxiv.org/pdf/1803.05069) section 5:
///
/// > When a node b* carries a QC that refers to a direct parent, i.e., b*.justify.node = b*.parent,
/// > we say that it forms a One-Chain. Denote by b'' = b*.justify.node. Node b* forms a Two-Chain,
/// > if in addition to forming a One-Chain, b''.justify.node = b''.parent.
/// > It forms a Three-Chain, if b'' forms a Two-Chain.
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
///
/// # Panics
/// If the leaf chain contains no decided leaf while reaching a decided view, which should be
/// impossible.
pub async fn decide_from_proposal<TYPES: NodeType>(
    proposal: &QuorumProposalWrapper<TYPES>,
    consensus: OuterConsensus<TYPES>,
    existing_upgrade_cert: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    public_key: &TYPES::SignatureKey,
    with_epochs: bool,
    membership: &Arc<RwLock<TYPES::Membership>>,
) -> LeafChainTraversalOutcome<TYPES> {
    let consensus_reader = consensus.read().await;
    let existing_upgrade_cert_reader = existing_upgrade_cert.read().await;
    let view_number = proposal.view_number();
    let parent_view_number = proposal.justify_qc().view_number();
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
                            tracing::warn!(
                                "Failed to decide an upgrade certificate in time. Ignoring."
                            );
                        } else {
                            tracing::info!("Reached decide on upgrade certificate: {:?}", cert);
                            res.decided_upgrade_cert = Some(cert.clone());
                        }
                    }
                }
                // If the block payload is available for this leaf, include it in
                // the leaf chain that we send to the client.
                if let Some(payload) = consensus_reader.saved_payloads().get(&leaf.view_number()) {
                    leaf.fill_block_payload_unchecked(payload.as_ref().payload.clone());
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
        tracing::debug!("Leaf ascension failed; error={e}");
    }

    if with_epochs && res.new_decided_view_number.is_some() {
        let epoch_height = consensus_reader.epoch_height;
        drop(consensus_reader);

        if let Some(decided_leaf_info) = res.leaf_views.last() {
            decide_epoch_root(&decided_leaf_info.leaf, epoch_height, membership).await;
        } else {
            tracing::info!("No decided leaf while a view has been decided.");
        }
    }

    res
}

/// Gets the parent leaf and state from the parent of a proposal, returning an [`utils::anytrace::Error`] if not.
#[instrument(skip_all)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn parent_leaf_and_state<TYPES: NodeType, V: Versions>(
    event_sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    event_receiver: &Receiver<Arc<HotShotEvent<TYPES>>>,
    membership: Arc<RwLock<TYPES::Membership>>,
    public_key: TYPES::SignatureKey,
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    consensus: OuterConsensus<TYPES>,
    upgrade_lock: &UpgradeLock<TYPES, V>,
    parent_view_number: TYPES::View,
    epoch_height: u64,
) -> Result<(Leaf2<TYPES>, Arc<<TYPES as NodeType>::ValidatedState>)> {
    let consensus_reader = consensus.read().await;
    let vsm_contains_parent_view = consensus_reader
        .validated_state_map()
        .contains_key(&parent_view_number);
    drop(consensus_reader);

    if !vsm_contains_parent_view {
        let _ = fetch_proposal(
            parent_view_number,
            event_sender.clone(),
            event_receiver.clone(),
            membership,
            consensus.clone(),
            public_key.clone(),
            private_key.clone(),
            upgrade_lock,
            epoch_height,
        )
        .await
        .context(info!("Failed to fetch proposal"))?;
    }

    let consensus_reader = consensus.read().await;
    //let parent_view_number = consensus_reader.high_qc().view_number();
    let parent_view = consensus_reader.validated_state_map().get(&parent_view_number).context(
        debug!("Couldn't find parent view in state map, waiting for replica to see proposal; parent_view_number: {}", *parent_view_number)
    )?;

    let (leaf_commitment, state) = parent_view.leaf_and_state().context(
        info!("Parent of high QC points to a view without a proposal; parent_view_number: {parent_view_number:?}, parent_view {parent_view:?}")
    )?;

    if leaf_commitment != consensus_reader.high_qc().data().leaf_commit {
        // NOTE: This happens on the genesis block
        tracing::debug!(
            "They don't equal: {:?}   {:?}",
            leaf_commitment,
            consensus_reader.high_qc().data().leaf_commit
        );
    }

    let leaf = consensus_reader
        .saved_leaves()
        .get(&leaf_commitment)
        .context(info!("Failed to find high QC of parent"))?;

    Ok((leaf.clone(), Arc::clone(state)))
}

/// Validate the state and safety and liveness of a proposal then emit
/// a `QuorumProposalValidated` event.
///
///
/// # Errors
/// If any validation or state update fails.
#[allow(clippy::too_many_lines)]
#[instrument(skip_all, fields(id = validation_info.id, view = *proposal.data.view_number()))]
pub async fn validate_proposal_safety_and_liveness<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: Proposal<TYPES, QuorumProposalWrapper<TYPES>>,
    parent_leaf: Leaf2<TYPES>,
    validation_info: &ValidationInfo<TYPES, I, V>,
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    sender: TYPES::SignatureKey,
) -> Result<()> {
    let view_number = proposal.data.view_number();

    let proposed_leaf = Leaf2::from_quorum_proposal(&proposal.data);
    ensure!(
        proposed_leaf.parent_commitment() == parent_leaf.commit(),
        "Proposed leaf does not extend the parent leaf."
    );
    let proposal_epoch = option_epoch_from_block_number::<TYPES>(
        validation_info
            .upgrade_lock
            .epochs_enabled(view_number)
            .await,
        proposed_leaf.height(),
        validation_info.epoch_height,
    );

    let state = Arc::new(
        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(proposal.data.block_header()),
    );

    {
        let mut consensus_writer = validation_info.consensus.write().await;
        if let Err(e) = consensus_writer.update_leaf(proposed_leaf.clone(), state, None) {
            tracing::trace!("{e:?}");
        }

        // Update our internal storage of the proposal. The proposal is valid, so
        // we swallow this error and just log if it occurs.
        if let Err(e) = consensus_writer.update_proposed_view(proposal.clone()) {
            tracing::debug!("Internal proposal update failed; error = {e:#}");
        };
    }

    UpgradeCertificate::validate(
        proposal.data.upgrade_certificate(),
        &validation_info.membership,
        proposal_epoch,
        &validation_info.upgrade_lock,
    )
    .await?;

    // Validate that the upgrade certificate is re-attached, if we saw one on the parent
    proposed_leaf
        .extends_upgrade(
            &parent_leaf,
            &validation_info.upgrade_lock.decided_upgrade_certificate,
        )
        .await?;

    let justify_qc = proposal.data.justify_qc().clone();
    // Create a positive vote if either liveness or safety check
    // passes.

    {
        let consensus_reader = validation_info.consensus.read().await;
        // Epoch safety check:
        // The proposal is safe if
        // 1. the proposed block and the justify QC block belong to the same epoch or
        // 2. the justify QC is the eQC for the previous block
        let justify_qc_epoch = option_epoch_from_block_number::<TYPES>(
            validation_info
                .upgrade_lock
                .epochs_enabled(view_number)
                .await,
            parent_leaf.height(),
            validation_info.epoch_height,
        );
        ensure!(
            proposal_epoch == justify_qc_epoch
                || consensus_reader.check_eqc(&proposed_leaf, &parent_leaf),
            {
                error!(
                    "Failed epoch safety check \n Proposed leaf is {:?} \n justify QC leaf is {:?}",
                    proposed_leaf.clone(),
                    parent_leaf.clone(),
                )
            }
        );

        // Make sure that the epoch transition proposal includes the next epoch QC
        if is_last_block_in_epoch(parent_leaf.height(), validation_info.epoch_height)
            && validation_info
                .upgrade_lock
                .epochs_enabled(view_number)
                .await
        {
            ensure!(proposal.data.next_epoch_justify_qc().is_some(),
            "Epoch transition proposal does not include the next epoch justify QC. Do not vote!");
        }

        // Liveness check.
        let liveness_check = justify_qc.view_number() > consensus_reader.locked_view();

        // Safety check.
        // Check if proposal extends from the locked leaf.
        let outcome = consensus_reader.visit_leaf_ancestors(
            justify_qc.view_number(),
            Terminator::Inclusive(consensus_reader.locked_view()),
            false,
            |leaf, _, _| {
                // if leaf view no == locked view no then we're done, report success by
                // returning true
                leaf.view_number() != consensus_reader.locked_view()
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
                    &validation_info.output_event_stream,
                )
                .await;
            }

            error!("Failed safety and liveness check \n High QC is {:?}  Proposal QC is {:?}  Locked view is {:?}", consensus_reader.high_qc(), proposal.data.clone(), consensus_reader.locked_view())
        });
    }

    // We accept the proposal, notify the application layer
    broadcast_event(
        Event {
            view_number,
            event: EventType::QuorumProposal {
                proposal: proposal.clone(),
                sender,
            },
        },
        &validation_info.output_event_stream,
    )
    .await;

    // Notify other tasks
    broadcast_event(
        Arc::new(HotShotEvent::QuorumProposalValidated(
            proposal.clone(),
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
pub(crate) async fn validate_proposal_view_and_certs<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    proposal: &Proposal<TYPES, QuorumProposalWrapper<TYPES>>,
    validation_info: &ValidationInfo<TYPES, I, V>,
) -> Result<()> {
    let view_number = proposal.data.view_number();
    ensure!(
        view_number >= validation_info.consensus.read().await.cur_view(),
        "Proposal is from an older view {:?}",
        proposal.data.clone()
    );

    // Validate the proposal's signature. This should also catch if the leaf_commitment does not equal our calculated parent commitment
    let membership_reader = validation_info.membership.read().await;
    proposal.validate_signature(&membership_reader, validation_info.epoch_height)?;
    drop(membership_reader);

    // Verify a timeout certificate OR a view sync certificate exists and is valid.
    if proposal.data.justify_qc().view_number() != view_number - 1 {
        let received_proposal_cert =
            proposal.data.view_change_evidence().clone().context(debug!(
                "Quorum proposal for view {} needed a timeout or view sync certificate, but did not have one",
                *view_number
        ))?;

        match received_proposal_cert {
            ViewChangeEvidence2::Timeout(timeout_cert) => {
                ensure!(
                    timeout_cert.data().view == view_number - 1,
                    "Timeout certificate for view {} was not for the immediately preceding view",
                    *view_number
                );
                let timeout_cert_epoch = timeout_cert.data().epoch();

                let membership_reader = validation_info.membership.read().await;
                let membership_stake_table = membership_reader.stake_table(timeout_cert_epoch);
                let membership_success_threshold =
                    membership_reader.success_threshold(timeout_cert_epoch);
                drop(membership_reader);

                timeout_cert
                    .is_valid_cert(
                        membership_stake_table,
                        membership_success_threshold,
                        &validation_info.upgrade_lock,
                    )
                    .await
                    .context(|e| {
                        warn!(
                            "Timeout certificate for view {} was invalid: {}",
                            *view_number, e
                        )
                    })?;
            }
            ViewChangeEvidence2::ViewSync(view_sync_cert) => {
                ensure!(
                    view_sync_cert.view_number == view_number,
                    "View sync cert view number {:?} does not match proposal view number {:?}",
                    view_sync_cert.view_number,
                    view_number
                );

                let view_sync_cert_epoch = view_sync_cert.data().epoch();

                let membership_reader = validation_info.membership.read().await;
                let membership_stake_table = membership_reader.stake_table(view_sync_cert_epoch);
                let membership_success_threshold =
                    membership_reader.success_threshold(view_sync_cert_epoch);
                drop(membership_reader);

                // View sync certs must also be valid.
                view_sync_cert
                    .is_valid_cert(
                        membership_stake_table,
                        membership_success_threshold,
                        &validation_info.upgrade_lock,
                    )
                    .await
                    .context(|e| warn!("Invalid view sync finalize cert provided: {}", e))?;
            }
        }
    }

    // Validate the upgrade certificate -- this is just a signature validation.
    // Note that we don't do anything with the certificate directly if this passes; it eventually gets stored as part of the leaf if nothing goes wrong.
    {
        let epoch = option_epoch_from_block_number::<TYPES>(
            proposal.data.epoch().is_some(),
            proposal.data.block_header().block_number(),
            validation_info.epoch_height,
        );
        UpgradeCertificate::validate(
            proposal.data.upgrade_certificate(),
            &validation_info.membership,
            epoch,
            &validation_info.upgrade_lock,
        )
        .await?;
    }

    Ok(())
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

/// Gets the next epoch QC corresponding to this epoch QC from the shared consensus state;
/// if it's not yet available, waits for it with a given timeout.
pub async fn wait_for_next_epoch_qc<TYPES: NodeType>(
    high_qc: &QuorumCertificate2<TYPES>,
    consensus: &OuterConsensus<TYPES>,
    timeout: u64,
    view_start_time: Instant,
    receiver: &Receiver<Arc<HotShotEvent<TYPES>>>,
) -> Option<NextEpochQuorumCertificate2<TYPES>> {
    tracing::debug!("getting the next epoch QC");
    if let Some(next_epoch_qc) = consensus.read().await.next_epoch_high_qc() {
        if next_epoch_qc.data.leaf_commit == high_qc.data.leaf_commit {
            // We have it already, no reason to wait
            return Some(next_epoch_qc.clone());
        }
    };

    let wait_duration = Duration::from_millis(timeout / 2);

    // TODO configure timeout
    let Some(time_spent) = Instant::now().checked_duration_since(view_start_time) else {
        // Shouldn't be possible, now must be after the start
        return None;
    };
    let Some(time_left) = wait_duration.checked_sub(time_spent) else {
        // No time left
        return None;
    };
    let receiver = receiver.clone();
    let Ok(Some(event)) = tokio::time::timeout(time_left, async move {
        let this_epoch_high_qc = high_qc.clone();
        EventDependency::new(
            receiver,
            Box::new(move |event| {
                let event = event.as_ref();
                if let HotShotEvent::NextEpochQc2Formed(Either::Left(qc)) = event {
                    qc.data.leaf_commit == this_epoch_high_qc.data.leaf_commit
                } else {
                    false
                }
            }),
        )
        .completed()
        .await
    })
    .await
    else {
        // Check again, there is a chance we missed it
        if let Some(next_epoch_qc) = consensus.read().await.next_epoch_high_qc() {
            if next_epoch_qc.data.leaf_commit == high_qc.data.leaf_commit {
                return Some(next_epoch_qc.clone());
            }
        };
        return None;
    };
    let HotShotEvent::NextEpochQc2Formed(Either::Left(next_epoch_qc)) = event.as_ref() else {
        // this shouldn't happen
        return None;
    };
    Some(next_epoch_qc.clone())
}

/// Validates qc's signatures and, if provided, validates next_epoch_qc's signatures and whether it
/// corresponds to the provided high_qc.
pub async fn validate_qc_and_next_epoch_qc<TYPES: NodeType, V: Versions>(
    qc: &QuorumCertificate2<TYPES>,
    maybe_next_epoch_qc: Option<&NextEpochQuorumCertificate2<TYPES>>,
    consensus: &OuterConsensus<TYPES>,
    membership: &Arc<RwLock<TYPES::Membership>>,
    upgrade_lock: &UpgradeLock<TYPES, V>,
) -> Result<()> {
    let membership_reader = membership.read().await;
    let membership_stake_table = membership_reader.stake_table(qc.data.epoch);
    let membership_success_threshold = membership_reader.success_threshold(qc.data.epoch);
    drop(membership_reader);

    {
        let consensus_reader = consensus.read().await;
        qc.is_valid_cert(
            membership_stake_table,
            membership_success_threshold,
            upgrade_lock,
        )
        .await
        .context(|e| {
            consensus_reader.metrics.invalid_qc.update(1);

            warn!("Invalid certificate: {}", e)
        })?;
    }

    if let Some(next_epoch_qc) = maybe_next_epoch_qc {
        // If the next epoch qc exists, make sure it's equal to the qc
        if qc.view_number() != next_epoch_qc.view_number() || qc.data != *next_epoch_qc.data {
            bail!("Next epoch qc exists but it's not equal with qc.");
        }

        let membership_reader = membership.read().await;
        let membership_next_stake_table =
            membership_reader.stake_table(qc.data.epoch.map(|x| x + 1));
        let membership_next_success_threshold =
            membership_reader.success_threshold(qc.data.epoch.map(|x| x + 1));
        drop(membership_reader);

        // Validate the next epoch qc as well
        next_epoch_qc
            .is_valid_cert(
                membership_next_stake_table,
                membership_next_success_threshold,
                upgrade_lock,
            )
            .await
            .context(|e| warn!("Invalid next epoch certificate: {}", e))?;
    }
    Ok(())
}
