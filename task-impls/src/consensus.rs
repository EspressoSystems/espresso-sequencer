use crate::{
    events::HotShotEvent,
    helpers::cancel_task,
    vote::{spawn_vote_accumulator, AccumulatorInfo},
};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use commit::Committable;
use core::time::Duration;
use hotshot_constants::LOOK_AHEAD;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{HotShotTaskCompleted, TS},
    task_impls::HSTWithEvent,
};
use hotshot_types::{
    consensus::{Consensus, View},
    data::{Leaf, QuorumProposal, VidCommitment, VidDisperse},
    event::{Event, EventType},
    message::{GeneralConsensusMessage, Proposal},
    simple_certificate::{QuorumCertificate, TimeoutCertificate},
    simple_vote::{QuorumData, QuorumVote, TimeoutData, TimeoutVote},
    traits::{
        block_contents::BlockHeader,
        consensus_api::ConsensusApi,
        election::Membership,
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        state::ConsensusTime,
        BlockPayload,
    },
    utils::{Terminator, ViewInner},
    vote::{Certificate, HasViewNumber},
};
use tracing::warn;

use snafu::Snafu;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument};

/// Error returned by the consensus task
#[derive(Snafu, Debug)]
pub struct ConsensusTaskError {}

/// Alias for the block payload commitment and the associated metadata.
type CommitmentAndMetadata<PAYLOAD> = (VidCommitment, <PAYLOAD as BlockPayload>::Metadata);

/// The state for the consensus task.  Contains all of the information for the implementation
/// of consensus
pub struct ConsensusTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
    /// Our public key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// The global task registry
    pub registry: GlobalRegistry,
    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,
    /// View timeout from config.
    pub timeout: u64,
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// The commitment to the current block payload and its metadata submitted to DA.
    pub payload_commitment_and_metadata: Option<CommitmentAndMetadata<TYPES::BlockPayload>>,

    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for DA committee
    pub committee_network: Arc<I::CommitteeNetwork>,

    /// Membership for Timeout votes/certs
    pub timeout_membership: Arc<TYPES::Membership>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Membership for DA committee Votes/certs
    pub committee_membership: Arc<TYPES::Membership>,

    /// Consensus api
    pub api: A,

    /// needed to typecheck
    pub _pd: PhantomData<I>,

    /// Current Vote collection task, with it's view.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Current timeout vote collection task with its view
    pub timeout_vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// timeout task handle
    pub timeout_task: Option<JoinHandle<()>>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// Event stream to publish events to the application layer
    pub output_event_stream: ChannelStream<Event<TYPES>>,

    /// All the VID shares we've received for current and future views.
    /// In the future we will need a different struct similar to VidDisperse except
    /// it stores only one share.
    /// TODO https://github.com/EspressoSystems/HotShot/issues/2146
    /// TODO https://github.com/EspressoSystems/HotShot/issues/1732
    pub vid_shares: HashMap<TYPES::Time, Proposal<TYPES, VidDisperse<TYPES>>>,

    /// The most recent proposal we have, will correspond to the current view if Some()
    /// Will be none if the view advanced through timeout/view_sync
    pub current_proposal: Option<QuorumProposal<TYPES>>,

    // ED Should replace this with config information since we need it anyway
    /// The node's id
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    ConsensusTaskState<TYPES, I, A>
{
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus genesis leaf", level = "error")]

    async fn genesis_leaf(&self) -> Option<Leaf<TYPES>> {
        let consensus = self.consensus.read().await;

        let Some(genesis_view) = consensus.state_map.get(&TYPES::Time::genesis()) else {
            error!("Couldn't find genesis view in state map.");
            return None;
        };
        let Some(leaf) = genesis_view.get_leaf_commitment() else {
            error!(
                ?genesis_view,
                "Genesis view points to a view without a leaf"
            );
            return None;
        };
        let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
            error!("Failed to find genesis leaf.");
            return None;
        };
        Some(leaf.clone())
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus vote if able", level = "error")]
    // Check if we are able to vote, like whether the proposal is valid,
    // whether we have DAC and VID share, and if so, vote.
    async fn vote_if_able(&self) -> bool {
        if !self.quorum_membership.has_stake(&self.public_key) {
            debug!(
                "We were not chosen for consensus committee on {:?}",
                self.cur_view
            );
            return false;
        }
        if let Some(proposal) = &self.current_proposal {
            // ED Need to account for the genesis DA cert
            // No need to check vid share nor da cert for genesis
            if proposal.justify_qc.is_genesis && proposal.view_number == TYPES::Time::new(1) {
                info!("Proposal is genesis!");

                let view = TYPES::Time::new(*proposal.view_number);
                let justify_qc = proposal.justify_qc.clone();
                let parent = if justify_qc.is_genesis {
                    self.genesis_leaf().await
                } else {
                    self.consensus
                        .read()
                        .await
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                };

                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some(parent) = parent else {
                    error!(
                                "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
                                justify_qc.get_data().leaf_commit,
                                proposal.view_number,
                            );
                    return false;
                };
                let parent_commitment = parent.commit();

                let leaf: Leaf<_> = Leaf {
                    view_number: view,
                    justify_qc: proposal.justify_qc.clone(),
                    parent_commitment,
                    block_header: proposal.block_header.clone(),
                    block_payload: None,
                    rejected: Vec::new(),
                    timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                    proposer_id: self.quorum_membership.get_leader(view).to_bytes(),
                };
                let vote = QuorumVote::<TYPES>::create_signed_vote(
                    QuorumData {
                        leaf_commit: leaf.commit(),
                    },
                    view,
                    &self.public_key,
                    &self.private_key,
                );
                let message = GeneralConsensusMessage::<TYPES>::Vote(vote);

                if let GeneralConsensusMessage::Vote(vote) = message {
                    debug!(
                        "Sending vote to next quorum leader {:?}",
                        vote.get_view_number() + 1
                    );
                    self.event_stream
                        .publish(HotShotEvent::QuorumVoteSend(vote))
                        .await;
                    return true;
                }
            }

            // TODO: re-enable this when HotShot/the sequencer needs the shares for something
            // issue: https://github.com/EspressoSystems/HotShot/issues/2236
            // Only vote if you has seen the VID share for this view
            // if let Some(_vid_share) = self.vid_shares.get(&proposal.view_number) {
            // } else {
            //     debug!(
            //         "We have not seen the VID share for this view {:?} yet, so we cannot vote.",
            //         proposal.view_number
            //     );
            //     return false;
            // }

            // Only vote if you have the DA cert
            // ED Need to update the view number this is stored under?
            if let Some(cert) = self
                .consensus
                .read()
                .await
                .saved_da_certs
                .get(&(proposal.get_view_number()))
            {
                let view = cert.view_number;
                // TODO: do some of this logic without the vote token check, only do that when voting.
                let justify_qc = proposal.justify_qc.clone();
                let parent = if justify_qc.is_genesis {
                    self.genesis_leaf().await
                } else {
                    self.consensus
                        .read()
                        .await
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                };

                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some(parent) = parent else {
                    error!(
                                "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
                                justify_qc.get_data().leaf_commit,
                                proposal.view_number,
                            );
                    return false;
                };
                let parent_commitment = parent.commit();

                let leaf: Leaf<_> = Leaf {
                    view_number: view,
                    justify_qc: proposal.justify_qc.clone(),
                    parent_commitment,
                    block_header: proposal.block_header.clone(),
                    block_payload: None,
                    rejected: Vec::new(),
                    timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                    proposer_id: self.quorum_membership.get_leader(view).to_bytes(),
                };

                // Validate the DAC.
                let message = if cert.is_valid_cert(self.committee_membership.as_ref()) {
                    // Validate the block payload commitment for non-genesis DAC.
                    if !cert.is_genesis
                        && cert.get_data().payload_commit
                            != proposal.block_header.payload_commitment()
                    {
                        error!("Block payload commitment does not equal parent commitment");
                        return false;
                    }
                    let vote = QuorumVote::<TYPES>::create_signed_vote(
                        QuorumData {
                            leaf_commit: leaf.commit(),
                        },
                        view,
                        &self.public_key,
                        &self.private_key,
                    );
                    GeneralConsensusMessage::<TYPES>::Vote(vote)
                } else {
                    error!(
                        "Invalid DAC in proposal! Skipping proposal. {:?} cur view is: {:?}",
                        cert, self.cur_view
                    );
                    return false;
                };

                if let GeneralConsensusMessage::Vote(vote) = message {
                    debug!(
                        "Sending vote to next quorum leader {:?}",
                        vote.get_view_number()
                    );
                    self.event_stream
                        .publish(HotShotEvent::QuorumVoteSend(vote))
                        .await;
                    return true;
                }
            }
            debug!(
                "Couldn't find DAC cert in certs, meaning we haven't received it yet for view {:?}",
                *proposal.get_view_number(),
            );
            return false;
        }
        debug!(
            "Could not vote because we don't have a proposal yet for view {}",
            *self.cur_view
        );
        false
    }

    /// Must only update the view and GC if the view actually changes
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus update view", level = "error")]

    async fn update_view(&mut self, new_view: TYPES::Time) -> bool {
        if *self.cur_view < *new_view {
            debug!(
                "Updating view from {} to {} in consensus task",
                *self.cur_view, *new_view
            );
            // cancel the old timeout task
            if let Some(timeout_task) = self.timeout_task.take() {
                cancel_task(timeout_task).await;
            }
            self.cur_view = new_view;

            // Poll the future leader for lookahead
            let lookahead_view = new_view + LOOK_AHEAD;
            if self.quorum_membership.get_leader(lookahead_view) != self.public_key {
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::PollFutureLeader(
                        *lookahead_view,
                        self.quorum_membership.get_leader(lookahead_view),
                    ))
                    .await;
            }

            // Start polling for proposals for the new view
            self.quorum_network
                .inject_consensus_info(ConsensusIntentEvent::PollForProposal(*self.cur_view + 1))
                .await;

            self.quorum_network
                .inject_consensus_info(ConsensusIntentEvent::PollForDAC(*self.cur_view + 1))
                .await;

            if self.quorum_membership.get_leader(self.cur_view + 1) == self.public_key {
                debug!("Polling for quorum votes for view {}", *self.cur_view);
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVotes(*self.cur_view))
                    .await;
            }

            self.event_stream
                .publish(HotShotEvent::ViewChange(new_view))
                .await;

            // Spawn a timeout task if we did actually update view
            let timeout = self.timeout;
            self.timeout_task = Some(async_spawn({
                let stream = self.event_stream.clone();
                // Nuance: We timeout on the view + 1 here because that means that we have
                // not seen evidence to transition to this new view
                let view_number = self.cur_view + 1;
                async move {
                    async_sleep(Duration::from_millis(timeout)).await;
                    stream
                        .publish(HotShotEvent::Timeout(TYPES::Time::new(*view_number)))
                        .await;
                }
            }));
            let consensus = self.consensus.read().await;
            consensus
                .metrics
                .current_view
                .set(usize::try_from(self.cur_view.get_u64()).unwrap());
            consensus.metrics.number_of_views_since_last_decide.set(
                usize::try_from(self.cur_view.get_u64()).unwrap()
                    - usize::try_from(consensus.last_decided_view.get_u64()).unwrap(),
            );

            return true;
        }
        false
    }

    /// Handles a consensus event received on the event stream
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus replica task", level = "error")]
    pub async fn handle_event(&mut self, event: HotShotEvent<TYPES>) {
        match event {
            HotShotEvent::QuorumProposalRecv(proposal, sender) => {
                debug!(
                    "Receved Quorum Proposal for view {}",
                    *proposal.data.view_number
                );

                // stop polling for the received proposal
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(
                        *proposal.data.view_number,
                    ))
                    .await;

                let view = proposal.data.get_view_number();
                if view < self.cur_view {
                    debug!("Proposal is from an older view {:?}", proposal.data.clone());
                    return;
                }

                let view_leader_key = self.quorum_membership.get_leader(view);
                if view_leader_key != sender {
                    warn!("Leader key does not match key in proposal");
                    return;
                }

                // Verify a timeout certificate exists and is valid
                if proposal.data.justify_qc.get_view_number() != view - 1 {
                    let Some(timeout_cert) = proposal.data.timeout_certificate.clone() else {
                        warn!(
                            "Quorum proposal for view {} needed a timeout certificate but did not have one",
                            *view);
                        return;
                    };

                    if timeout_cert.get_data().view != view - 1 {
                        warn!("Timeout certificate for view {} was not for the immediately preceding view", *view);
                        return;
                    }

                    if !timeout_cert.is_valid_cert(self.timeout_membership.as_ref()) {
                        warn!("Timeout certificate for view {} was invalid", *view);
                        return;
                    }
                }

                let justify_qc = proposal.data.justify_qc.clone();

                if !justify_qc.is_valid_cert(self.quorum_membership.as_ref()) {
                    error!("Invalid justify_qc in proposal for view {}", *view);
                    let consensus = self.consensus.write().await;
                    consensus.metrics.invalid_qc.update(1);
                    return;
                }

                // NOTE: We could update our view with a valid TC but invalid QC, but that is not what we do here
                self.update_view(view).await;

                self.current_proposal = Some(proposal.data.clone());

                let consensus = self.consensus.upgradable_read().await;

                // Construct the leaf.
                let parent = if justify_qc.is_genesis {
                    self.genesis_leaf().await
                } else {
                    consensus
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                };

                //
                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some(parent) = parent else {
                    // If no parent then just update our state map and return.  We will not vote.
                    error!(
                        "Proposal's parent missing from storage with commitment: {:?}",
                        justify_qc.get_data().leaf_commit
                    );
                    let leaf = Leaf {
                        view_number: view,
                        justify_qc: justify_qc.clone(),
                        parent_commitment: justify_qc.get_data().leaf_commit,
                        block_header: proposal.data.block_header,
                        block_payload: None,
                        rejected: Vec::new(),
                        timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                        proposer_id: sender.to_bytes(),
                    };

                    let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
                    consensus.state_map.insert(
                        view,
                        View {
                            view_inner: ViewInner::Leaf {
                                leaf: leaf.commit(),
                            },
                        },
                    );
                    consensus.saved_leaves.insert(leaf.commit(), leaf.clone());

                    return;
                };
                let parent_commitment = parent.commit();
                let leaf: Leaf<_> = Leaf {
                    view_number: view,
                    justify_qc: justify_qc.clone(),
                    parent_commitment,
                    block_header: proposal.data.block_header,
                    block_payload: None,
                    rejected: Vec::new(),
                    timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                    proposer_id: sender.to_bytes(),
                };
                let leaf_commitment = leaf.commit();

                // Validate the signature. This should also catch if the leaf_commitment does not equal our calculated parent commitment
                if !view_leader_key.validate(&proposal.signature, leaf_commitment.as_ref()) {
                    error!(?proposal.signature, "Could not verify proposal.");
                    return;
                }
                // Create a positive vote if either liveness or safety check
                // passes.

                // Liveness check.
                let liveness_check = justify_qc.get_view_number() > consensus.locked_view;

                // Safety check.
                // Check if proposal extends from the locked leaf.
                let outcome = consensus.visit_leaf_ancestors(
                    justify_qc.get_view_number(),
                    Terminator::Inclusive(consensus.locked_view),
                    false,
                    |leaf| {
                        // if leaf view no == locked view no then we're done, report success by
                        // returning true
                        leaf.view_number != consensus.locked_view
                    },
                );
                let safety_check = outcome.is_ok();
                if let Err(e) = outcome {
                    self.api
                        .send_event(Event {
                            view_number: view,
                            event: EventType::Error { error: Arc::new(e) },
                        })
                        .await;
                }

                // Skip if both saftey and liveness checks fail.
                if !safety_check && !liveness_check {
                    error!("Failed safety check and liveness check");
                    return;
                }

                let high_qc = leaf.justify_qc.clone();
                let mut new_anchor_view = consensus.last_decided_view;
                let mut new_locked_view = consensus.locked_view;
                let mut last_view_number_visited = view;
                let mut new_commit_reached: bool = false;
                let mut new_decide_reached = false;
                let mut new_decide_qc = None;
                let mut leaf_views = Vec::new();
                let mut included_txns = HashSet::new();
                let old_anchor_view = consensus.last_decided_view;
                let parent_view = leaf.justify_qc.get_view_number();
                let mut current_chain_length = 0usize;
                if parent_view + 1 == view {
                    current_chain_length += 1;
                    if let Err(e) = consensus.visit_leaf_ancestors(
                        parent_view,
                        Terminator::Exclusive(old_anchor_view),
                        true,
                        |leaf| {
                            if !new_decide_reached {
                                if last_view_number_visited == leaf.view_number + 1 {
                                    last_view_number_visited = leaf.view_number;
                                    current_chain_length += 1;
                                    if current_chain_length == 2 {
                                        new_locked_view = leaf.view_number;
                                        new_commit_reached = true;
                                        // The next leaf in the chain, if there is one, is decided, so this
                                        // leaf's justify_qc would become the QC for the decided chain.
                                        new_decide_qc = Some(leaf.justify_qc.clone());
                                    } else if current_chain_length == 3 {
                                        new_anchor_view = leaf.view_number;
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
                                if leaf.view_number == new_anchor_view {
                                    consensus
                                        .metrics
                                        .last_synced_block_height
                                        .set(usize::try_from(leaf.get_height()).unwrap_or(0));
                                }
                                // If the block payload is available for this leaf, include it in
                                // the leaf chain that we send to the client.
                                if let Some(encoded_txns) =
                                    consensus.saved_payloads.get(leaf.get_payload_commitment())
                                {
                                    let payload = BlockPayload::from_bytes(
                                        encoded_txns.clone().into_iter(),
                                        leaf.get_block_header().metadata(),
                                    );

                                    leaf.fill_block_payload_unchecked(payload);
                                }

                                leaf_views.push(leaf.clone());
                                if let Some(payload) = leaf.block_payload {
                                    for txn in payload.transaction_commitments() {
                                        included_txns.insert(txn);
                                    }
                                }
                            }
                            true
                        },
                    ) {
                        error!("publishing view error");
                        self.output_event_stream
                            .publish(Event {
                                view_number: view,
                                event: EventType::Error { error: e.into() },
                            })
                            .await;
                    }
                }

                let included_txns_set: HashSet<_> = if new_decide_reached {
                    included_txns
                } else {
                    HashSet::new()
                };

                // promote lock here to add proposal to statemap
                let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
                if high_qc.view_number > consensus.high_qc.view_number {
                    consensus.high_qc = high_qc;
                }
                consensus.state_map.insert(
                    view,
                    View {
                        view_inner: ViewInner::Leaf {
                            leaf: leaf.commit(),
                        },
                    },
                );
                consensus.saved_leaves.insert(leaf.commit(), leaf.clone());
                if new_commit_reached {
                    consensus.locked_view = new_locked_view;
                }
                #[allow(clippy::cast_precision_loss)]
                if new_decide_reached {
                    self.event_stream
                        .publish(HotShotEvent::LeafDecided(leaf_views.clone()))
                        .await;
                    let decide_sent = self.output_event_stream.publish(Event {
                        view_number: consensus.last_decided_view,
                        event: EventType::Decide {
                            leaf_chain: Arc::new(leaf_views),
                            qc: Arc::new(new_decide_qc.unwrap()),
                            block_size: Some(included_txns_set.len().try_into().unwrap()),
                        },
                    });
                    let old_anchor_view = consensus.last_decided_view;
                    consensus
                        .collect_garbage(old_anchor_view, new_anchor_view)
                        .await;
                    consensus.last_decided_view = new_anchor_view;
                    consensus.metrics.invalid_qc.set(0);
                    consensus
                        .metrics
                        .last_decided_view
                        .set(usize::try_from(consensus.last_decided_view.get_u64()).unwrap());
                    let cur_number_of_views_per_decide_event =
                        *self.cur_view - consensus.last_decided_view.get_u64();
                    consensus
                        .metrics
                        .number_of_views_per_decide_event
                        .add_point(cur_number_of_views_per_decide_event as f64);

                    // We're only storing the last QC. We could store more but we're realistically only going to retrieve the last one.
                    if let Err(e) = self.api.store_leaf(old_anchor_view, leaf).await {
                        error!("Could not insert new anchor into the storage API: {:?}", e);
                    }

                    debug!("Sending Decide for view {:?}", consensus.last_decided_view);
                    debug!("Decided txns len {:?}", included_txns_set.len());
                    decide_sent.await;
                }

                let new_view = self.current_proposal.clone().unwrap().view_number + 1;
                // In future we can use the mempool model where we fetch the proposal if we don't have it, instead of having to wait for it here
                // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
                let should_propose = self.quorum_membership.get_leader(new_view) == self.public_key
                    && consensus.high_qc.view_number
                        == self.current_proposal.clone().unwrap().view_number;
                // todo get rid of this clone
                let qc = consensus.high_qc.clone();

                drop(consensus);
                if should_propose {
                    debug!(
                        "Attempting to publish proposal after voting; now in view: {}",
                        *new_view
                    );
                    self.publish_proposal_if_able(qc.clone(), qc.view_number + 1, None)
                        .await;
                }
                if !self.vote_if_able().await {
                    return;
                }
                self.current_proposal = None;
            }
            HotShotEvent::QuorumVoteRecv(ref vote) => {
                debug!("Received quroum vote: {:?}", vote.get_view_number());
                if self
                    .quorum_membership
                    .get_leader(vote.get_view_number() + 1)
                    != self.public_key
                {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view_number() + 1,
                        self.quorum_membership
                            .get_leader(vote.get_view_number() + 2)
                            == self.public_key
                    );
                    return;
                }

                let collection_view =
                    if let Some((collection_view, collection_task, _)) = &self.vote_collector {
                        if vote.get_view_number() > *collection_view {
                            // ED I think we'd want to let that task timeout to avoid a griefing vector
                            self.registry.shutdown_task(*collection_task).await;
                        }
                        *collection_view
                    } else {
                        TYPES::Time::new(0)
                    };

                if vote.get_view_number() > collection_view {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.quorum_membership.clone(),
                        view: vote.get_view_number(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                        registry: self.registry.clone(),
                    };
                    let name = "Quorum Vote Collection";
                    self.vote_collector = spawn_vote_accumulator::<
                        TYPES,
                        QuorumVote<TYPES>,
                        QuorumCertificate<TYPES>,
                    >(
                        &info, vote.clone(), event, name.to_string()
                    )
                    .await;
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::QuorumVoteRecv(vote.clone()))
                        .await;
                }
            }
            HotShotEvent::TimeoutVoteRecv(ref vote) => {
                if self
                    .timeout_membership
                    .get_leader(vote.get_view_number() + 1)
                    != self.public_key
                {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view_number() + 1,
                        self.timeout_membership
                            .get_leader(vote.get_view_number() + 2)
                            == self.public_key
                    );
                    return;
                }
                let collection_view =
                    if let Some((collection_view, collection_task, _)) = &self.vote_collector {
                        if vote.get_view_number() > *collection_view {
                            // ED I think we'd want to let that task timeout to avoid a griefing vector
                            self.registry.shutdown_task(*collection_task).await;
                        }
                        *collection_view
                    } else {
                        TYPES::Time::new(0)
                    };

                if vote.get_view_number() > collection_view {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.timeout_membership.clone(),
                        view: vote.get_view_number(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                        registry: self.registry.clone(),
                    };
                    let name = "Timeout Vote Collection";
                    self.vote_collector =
                        spawn_vote_accumulator(&info, vote.clone(), event, name.to_string()).await;
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::TimeoutVoteRecv(vote.clone()))
                        .await;
                }
            }
            HotShotEvent::QCFormed(cert) => {
                debug!("QC Formed event happened!");

                if let either::Right(qc) = cert.clone() {
                    // cancel poll for votes
                    self.quorum_network
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    debug!(
                        "Attempting to publish proposal after forming a TC for view {}",
                        *qc.view_number
                    );

                    let view = qc.view_number + 1;

                    let high_qc = self.consensus.read().await.high_qc.clone();

                    if self
                        .publish_proposal_if_able(high_qc, view, Some(qc.clone()))
                        .await
                    {
                    } else {
                        warn!("Wasn't able to publish proposal");
                    }
                }
                if let either::Left(qc) = cert {
                    let mut consensus = self.consensus.write().await;
                    consensus.high_qc = qc.clone();

                    // cancel poll for votes
                    self.quorum_network
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    drop(consensus);
                    debug!(
                        "Attempting to publish proposal after forming a QC for view {}",
                        *qc.view_number
                    );

                    if !self
                        .publish_proposal_if_able(qc.clone(), qc.view_number + 1, None)
                        .await
                    {
                        debug!(
                            "Wasn't able to publish proposal when QC was formed, still may publish"
                        );
                    }
                }
            }
            HotShotEvent::DACRecv(cert) => {
                debug!("DAC Recved for view ! {}", *cert.view_number);
                let view = cert.view_number;

                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForDAC(*view))
                    .await;

                self.committee_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                self.consensus
                    .write()
                    .await
                    .saved_da_certs
                    .insert(view, cert);

                if self.vote_if_able().await {
                    self.current_proposal = None;
                }
            }
            HotShotEvent::VidDisperseRecv(disperse, sender) => {
                let view = disperse.data.get_view_number();

                debug!(
                    "VID disperse received for view: {:?} in consensus task",
                    view
                );

                // Allow VID disperse date that is one view older, in case we have updated the
                // view.
                // Adding `+ 1` on the LHS rather than `- 1` on the RHS, to avoid the overflow
                // error due to subtracting the genesis view number.
                if view + 1 < self.cur_view {
                    warn!("Throwing away VID disperse data that is more than one view older");
                    return;
                }

                debug!("VID disperse data is not more than one view older.");
                let payload_commitment = disperse.data.payload_commitment;

                // Check whether the sender is the right leader for this view
                let view_leader_key = self.quorum_membership.get_leader(view);
                if view_leader_key != sender {
                    warn!(
                        "VID dispersal/share is not from expected leader key for view {} \n",
                        *view
                    );
                    return;
                }

                if !view_leader_key.validate(&disperse.signature, payload_commitment.as_ref()) {
                    warn!("Could not verify VID dispersal/share sig.");
                    return;
                }

                // stop polling for the received disperse after verifying it's valid
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDDisperse(
                        *disperse.data.view_number,
                    ))
                    .await;

                // Add to the storage that we have received the VID disperse for a specific view
                // TODO: re-enable this when HotShot/the sequencer needs the shares for something
                // issue: https://github.com/EspressoSystems/HotShot/issues/2236
                // self.vid_shares.insert(view, disperse);
            }
            HotShotEvent::ViewChange(new_view) => {
                debug!("View Change event for view {} in consensus task", *new_view);

                let old_view_number = self.cur_view;

                // Start polling for VID disperse for the new view
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *old_view_number + 1,
                    ))
                    .await;

                // update the view in state to the one in the message
                // Publish a view change event to the application
                if !self.update_view(new_view).await {
                    debug!("view not updated");
                    return;
                }

                self.output_event_stream
                    .publish(Event {
                        view_number: old_view_number,
                        event: EventType::ViewFinished {
                            view_number: old_view_number,
                        },
                    })
                    .await;
            }
            HotShotEvent::Timeout(view) => {
                // NOTE: We may optionally have the timeout task listen for view change events
                if self.cur_view >= view {
                    return;
                }
                if !self.timeout_membership.has_stake(&self.public_key) {
                    debug!(
                        "We were not chosen for consensus committee on {:?}",
                        self.cur_view
                    );
                    return;
                }

                // cancel poll for votes
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                // cancel poll for proposal
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(*view))
                    .await;

                let vote = TimeoutVote::create_signed_vote(
                    TimeoutData { view },
                    view,
                    &self.public_key,
                    &self.private_key,
                );

                self.event_stream
                    .publish(HotShotEvent::TimeoutVoteSend(vote))
                    .await;
                debug!(
                    "We did not receive evidence for view {} in time, sending timeout vote for that view!",
                    *view
                );
                self.output_event_stream
                    .publish(Event {
                        view_number: view,
                        event: EventType::ReplicaViewTimeout { view_number: view },
                    })
                    .await;
                let consensus = self.consensus.read().await;
                consensus.metrics.number_of_timeouts.add(1);
            }
            HotShotEvent::SendPayloadCommitmentAndMetadata(payload_commitment, metadata) => {
                self.payload_commitment_and_metadata = Some((payload_commitment, metadata));
                let high_qc = self.consensus.read().await.high_qc.clone();
                let leader_view = high_qc.get_view_number() + 1;
                if self.quorum_membership.get_leader(leader_view) == self.public_key {
                    self.publish_proposal_if_able(high_qc, leader_view, None)
                        .await;
                }
            }
            _ => {}
        }
    }

    /// Sends a proposal if possible from the high qc we have
    #[allow(clippy::too_many_lines)]
    pub async fn publish_proposal_if_able(
        &mut self,
        _qc: QuorumCertificate<TYPES>,
        view: TYPES::Time,
        timeout_certificate: Option<TimeoutCertificate<TYPES>>,
    ) -> bool {
        if self.quorum_membership.get_leader(view) != self.public_key {
            // This is expected for view 1, so skipping the logging.
            if view != TYPES::Time::new(1) {
                error!(
                    "Somehow we formed a QC but are not the leader for the next view {:?}",
                    view
                );
            }
            return false;
        }

        let consensus = self.consensus.read().await;
        let parent_view_number = &consensus.high_qc.get_view_number();
        let mut reached_decided = false;

        let Some(parent_view) = consensus.state_map.get(parent_view_number) else {
            // This should have been added by the replica?
            error!("Couldn't find parent view in state map, waiting for replica to see proposal\n parent view number: {}", **parent_view_number);
            return false;
        };
        // Leaf hash in view inner does not match high qc hash - Why?
        let Some(leaf_commitment) = parent_view.get_leaf_commitment() else {
            error!(
                ?parent_view_number,
                ?parent_view,
                "Parent of high QC points to a view without a proposal"
            );
            return false;
        };
        if leaf_commitment != consensus.high_qc.get_data().leaf_commit {
            // NOTE: This happens on the genesis block
            debug!(
                "They don't equal: {:?}   {:?}",
                leaf_commitment,
                consensus.high_qc.get_data().leaf_commit
            );
        }
        let Some(leaf) = consensus.saved_leaves.get(&leaf_commitment) else {
            error!("Failed to find high QC of parent.");
            return false;
        };
        if leaf.view_number == consensus.last_decided_view {
            reached_decided = true;
        }

        let parent_leaf = leaf.clone();
        let parent_header = parent_leaf.block_header.clone();

        let original_parent_hash = parent_leaf.commit();

        let mut next_parent_hash = original_parent_hash;

        // Walk back until we find a decide
        if !reached_decided {
            debug!("not reached decide fro view {:?}", self.cur_view);
            while let Some(next_parent_leaf) = consensus.saved_leaves.get(&next_parent_hash) {
                if next_parent_leaf.view_number <= consensus.last_decided_view {
                    break;
                }
                next_parent_hash = next_parent_leaf.parent_commitment;
            }
            debug!("updated saved leaves");
            // TODO do some sort of sanity check on the view number that it matches decided
        }

        if let Some((payload_commitment, metadata)) = &self.payload_commitment_and_metadata {
            let leaf = Leaf {
                view_number: view,
                justify_qc: consensus.high_qc.clone(),
                parent_commitment: parent_leaf.commit(),
                block_header: TYPES::BlockHeader::new(
                    *payload_commitment,
                    metadata.clone(),
                    &parent_header,
                ),
                block_payload: None,
                rejected: vec![],
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                proposer_id: self.api.public_key().to_bytes(),
            };

            let signature = TYPES::SignatureKey::sign(&self.private_key, leaf.commit().as_ref());
            // TODO: DA cert is sent as part of the proposal here, we should split this out so we don't have to wait for it.
            let proposal = QuorumProposal {
                block_header: leaf.block_header.clone(),
                view_number: leaf.view_number,
                justify_qc: consensus.high_qc.clone(),
                timeout_certificate: timeout_certificate.or_else(|| None),
                proposer_id: leaf.proposer_id,
            };

            let message = Proposal {
                data: proposal,
                signature,
                _pd: PhantomData,
            };
            debug!(
                "Sending proposal for view {:?} \n {:?}",
                leaf.view_number, ""
            );
            self.event_stream
                .publish(HotShotEvent::QuorumProposalSend(
                    message,
                    self.public_key.clone(),
                ))
                .await;
            self.payload_commitment_and_metadata = None;
            return true;
        }
        debug!("Cannot propose because we don't have the VID payload commitment and metadata");
        false
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I>> TS
    for ConsensusTaskState<TYPES, I, A>
{
}

/// Type alias for Consensus task
pub type ConsensusTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    ConsensusTaskState<TYPES, I, A>,
>;

/// Event handle for consensus
pub async fn sequencing_consensus_handle<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
>(
    event: HotShotEvent<TYPES>,
    mut state: ConsensusTaskState<TYPES, I, A>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    ConsensusTaskState<TYPES, I, A>,
) {
    if let HotShotEvent::Shutdown = event {
        (Some(HotShotTaskCompleted::ShutDown), state)
    } else {
        state.handle_event(event).await;
        (None, state)
    }
}

/// Filter for consensus, returns true for event types the consensus task subscribes to.
pub fn consensus_event_filter<TYPES: NodeType>(event: &HotShotEvent<TYPES>) -> bool {
    matches!(
        event,
        HotShotEvent::QuorumProposalRecv(_, _)
            | HotShotEvent::QuorumVoteRecv(_)
            | HotShotEvent::QCFormed(_)
            | HotShotEvent::DACRecv(_)
            | HotShotEvent::ViewChange(_)
            | HotShotEvent::SendPayloadCommitmentAndMetadata(_, _)
            | HotShotEvent::Timeout(_)
            | HotShotEvent::TimeoutVoteRecv(_)
            | HotShotEvent::VidDisperseRecv(_, _)
            | HotShotEvent::Shutdown,
    )
}
