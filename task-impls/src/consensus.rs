use crate::events::HotShotEvent;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use bitvec::prelude::*;
use commit::{Commitment, Committable};
use core::time::Duration;
use either::{Either, Left, Right};
use futures::FutureExt;
use hotshot_constants::LOOK_AHEAD;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::{
    certificate::{DACertificate, QuorumCertificate, TimeoutCertificate, VIDCertificate},
    consensus::{Consensus, View},
    data::{Leaf, LeafType, ProposalType, QuorumProposal},
    event::{Event, EventType},
    message::{GeneralConsensusMessage, Message, Proposal, SequencingMessage},
    simple_certificate::QuorumCertificate2,
    simple_vote::{YesData, YesVote},
    traits::{
        consensus_api::ConsensusApi,
        election::{ConsensusExchange, QuorumExchangeType, SignedCertificate, TimeoutExchangeType},
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{
            CommitteeEx, NodeImplementation, NodeType, QuorumEx, QuorumMembership, TimeoutEx,
        },
        signature_key::SignatureKey,
        state::ConsensusTime,
        BlockPayload,
    },
    utils::{Terminator, ViewInner},
    vote::{TimeoutVoteAccumulator, VoteType},
    vote2::{Certificate2, HasViewNumber, VoteAccumulator2},
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

/// The state for the consensus task.  Contains all of the information for the implementation
/// of consensus
pub struct ConsensusTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>, ConsensusMessage = SequencingMessage<TYPES, I>>,
    A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
> where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockType>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
    /// The global task registry
    pub registry: GlobalRegistry,
    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, Leaf<TYPES>>>>,
    /// View timeout from config.
    pub timeout: u64,
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Current block submitted to DA
    pub block: Option<TYPES::BlockType>,

    /// the quorum exchange
    pub quorum_exchange: Arc<QuorumEx<TYPES, I>>,

    /// The timeout exchange
    pub timeout_exchange: Arc<TimeoutEx<TYPES, I>>,

    /// Consensus api
    pub api: A,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// needed to typecheck
    pub _pd: PhantomData<I>,

    /// Current Vote collection task, with it's view.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Have we already sent a proposal for a particular view
    /// since proposal can be sent either on QCFormed event or ViewChange event
    // pub proposal_sent: HashMap<TYPES::Time, bool>,

    /// timeout task handle
    pub timeout_task: JoinHandle<()>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,

    /// Event stream to publish events to the application layer
    pub output_event_stream: ChannelStream<Event<TYPES, I::Leaf>>,

    /// All the DA certs we've received for current and future views.
    pub da_certs: HashMap<TYPES::Time, DACertificate<TYPES>>,

    /// All the VID certs we've received for current and future views.
    pub vid_certs: HashMap<TYPES::Time, VIDCertificate<TYPES>>,

    /// The most recent proposal we have, will correspond to the current view if Some()
    /// Will be none if the view advanced through timeout/view_sync
    pub current_proposal: Option<QuorumProposal<TYPES, I::Leaf>>,

    // ED Should replace this with config information since we need it anyway
    /// The node's id
    pub id: u64,

    /// The most Recent QC we've formed from votes, if we've formed it.
    pub qc: Option<QuorumCertificate<TYPES, Commitment<I::Leaf>>>,
}

/// State for the vote collection task.  This handles the building of a QC from a votes received
pub struct VoteCollectionTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>,
> where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
    /// the quorum exchange
    pub quorum_exchange: Arc<QuorumEx<TYPES, I>>,
    /// the timeout exchange
    pub timeout_exchange: Arc<TimeoutEx<TYPES, I>>,

    #[allow(clippy::type_complexity)]
    /// Accumulator for votes
    pub accumulator: Either<
        VoteAccumulator2<
            TYPES,
            YesVote<TYPES, I::Leaf, QuorumMembership<TYPES, I>>,
            QuorumCertificate2<TYPES, I::Leaf>,
        >,
        QuorumCertificate2<TYPES, I::Leaf>,
    >,

    /// Accumulator for votes
    #[allow(clippy::type_complexity)]
    pub timeout_accumulator: Either<
        <TimeoutCertificate<TYPES> as SignedCertificate<
            TYPES,
            TYPES::Time,
            TYPES::VoteTokenType,
            Commitment<TYPES::Time>,
        >>::VoteAccumulator,
        TimeoutCertificate<TYPES>,
    >,
    /// View which this vote collection task is collecting votes in
    pub cur_view: TYPES::Time,
    /// The event stream shared by all tasks
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
    /// Node id
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>> TS
    for VoteCollectionTaskState<TYPES, I>
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
}

#[instrument(skip_all, fields(id = state.id, view = *state.cur_view), name = "Quorum Vote Collection Task", level = "error")]

async fn vote_handle<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>>(
    mut state: VoteCollectionTaskState<TYPES, I>,
    event: HotShotEvent<TYPES, I>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    VoteCollectionTaskState<TYPES, I>,
)
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
    match event {
        HotShotEvent::QuorumVoteRecv(vote) => {
            // For the case where we receive votes after we've made a certificate
            if state.accumulator.is_right() {
                return (None, state);
            }

            if vote.get_view_number() != state.cur_view {
                error!(
                    "Vote view does not match! vote view is {} current view is {}",
                    *vote.get_view_number(),
                    *state.cur_view
                );
                return (None, state);
            }

            let accumulator = state.accumulator.left().unwrap();

            match accumulator.accumulate(&vote, state.quorum_exchange.membership()) {
                Either::Left(acc) => {
                    state.accumulator = Either::Left(acc);
                    return (None, state);
                }
                Either::Right(qc) => {
                    debug!("QCFormed! {:?}", qc.view_number);
                    state
                        .event_stream
                        .publish(HotShotEvent::QCFormed(either::Left(qc.clone())))
                        .await;
                    state.accumulator = Either::Right(qc.clone());

                    // No longer need to poll for votes
                    state
                        .quorum_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    return (Some(HotShotTaskCompleted::ShutDown), state);
                }
            }
        }
        // TODO: Code below is redundant of code above; can be fixed
        // during exchange refactor
        // https://github.com/EspressoSystems/HotShot/issues/1799
        HotShotEvent::TimeoutVoteRecv(vote) => {
            debug!("Received timeout vote for view {}", *vote.get_view());
            if state.timeout_accumulator.is_right() {
                return (None, state);
            }

            if vote.get_view() != state.cur_view {
                error!(
                    "Vote view does not match! vote view is {} current view is {}",
                    *vote.get_view(),
                    *state.cur_view
                );
                return (None, state);
            }

            let accumulator = state.timeout_accumulator.left().unwrap();

            match state.timeout_exchange.accumulate_vote(
                accumulator,
                &vote,
                &vote.get_view().commit(),
            ) {
                Either::Left(acc) => {
                    state.timeout_accumulator = Either::Left(acc);
                    return (None, state);
                }
                Either::Right(qc) => {
                    debug!("QCFormed! {:?}", qc.view_number);
                    state
                        .event_stream
                        .publish(HotShotEvent::QCFormed(either::Right(qc.clone())))
                        .await;
                    state.timeout_accumulator = Either::Right(qc.clone());

                    // No longer need to poll for votes
                    state
                        .quorum_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    return (Some(HotShotTaskCompleted::ShutDown), state);
                }
            }
        }
        HotShotEvent::Shutdown => {
            return (Some(HotShotTaskCompleted::ShutDown), state);
        }
        _ => {
            error!("Unexpected event");
        }
    }
    (None, state)
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
    > ConsensusTaskState<TYPES, I, A>
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockType>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
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

    async fn vote_if_able(&self) -> bool {
        if let Some(proposal) = &self.current_proposal {
            // ED Need to account for the genesis DA cert
            if proposal.justify_qc.is_genesis && proposal.view_number == TYPES::Time::new(1) {
                // warn!("Proposal is genesis!");

                let view = TYPES::Time::new(*proposal.view_number);
                let vote_token = self.quorum_exchange.make_vote_token(view);

                match vote_token {
                    Err(e) => {
                        error!("Failed to generate vote token for {:?} {:?}", view, e);
                    }
                    Ok(None) => {
                        debug!("We were not chosen for consensus committee on {:?}", view);
                    }
                    Ok(Some(_vote_token)) => {
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
                            height: proposal.height,
                            justify_qc: proposal.justify_qc.clone(),
                            parent_commitment,
                            deltas: Right(proposal.block_commitment),
                            rejected: Vec::new(),
                            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                            proposer_id: self.quorum_exchange.get_leader(view).to_bytes(),
                        };
                        let vote =
                            YesVote::<TYPES, I::Leaf, QuorumMembership<TYPES, I>>::create_signed_vote(
                                YesData { leaf_commit: leaf.commit() },
                                view,
                                &self.quorum_exchange.public_key(),
                                &self.quorum_exchange.private_key(),
                            );
                        let message = GeneralConsensusMessage::<TYPES, I>::Vote(vote);

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
                }
            }

            // Only vote if you have the DA cert
            // ED Need to update the view number this is stored under?
            if let Some(cert) = self.da_certs.get(&(proposal.get_view_number())) {
                let view = cert.view_number;
                let vote_token = self.quorum_exchange.make_vote_token(view);
                // TODO: do some of this logic without the vote token check, only do that when voting.
                match vote_token {
                    Err(e) => {
                        error!("Failed to generate vote token for {:?} {:?}", view, e);
                    }
                    Ok(None) => {
                        debug!("We were not chosen for consensus committee on {:?}", view);
                    }
                    Ok(Some(vote_token)) => {
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
                            height: proposal.height,
                            justify_qc: proposal.justify_qc.clone(),
                            parent_commitment,
                            deltas: Right(proposal.block_commitment),
                            rejected: Vec::new(),
                            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                            proposer_id: self.quorum_exchange.get_leader(view).to_bytes(),
                        };

                        // Validate the DAC.
                        let message = if self.committee_exchange.is_valid_cert(cert) {
                            // Validate the block commitment for non-genesis DAC.
                            if !cert.is_genesis()
                                && cert.leaf_commitment() != proposal.block_commitment
                            {
                                error!("Block commitment does not equal parent commitment");
                                return false;
                            }
                            let vote =
                            YesVote::<TYPES, I::Leaf, QuorumMembership<TYPES, I>>::create_signed_vote(
                                YesData { leaf_commit: leaf.commit() },
                                view,
                                &self.quorum_exchange.public_key(),
                                &self.quorum_exchange.private_key(),
                            );
                            GeneralConsensusMessage::<TYPES, I>::Vote(vote)
                        } else {
                            error!("Invalid DAC in proposal! Skipping proposal. {:?} cur view is: {:?}", cert, self.cur_view );
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
                }
            }
            info!(
                "Couldn't find DAC cert in certs, meaning we haven't received it yet for view {:?}",
                *proposal.get_view_number(),
            );
            return false;
        }
        info!(
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

            // Remove old certs, we won't vote on past views
            for view in *self.cur_view..*new_view - 1 {
                let v = TYPES::Time::new(view);
                self.da_certs.remove(&v);
            }
            self.cur_view = new_view;

            // Poll the future leader for lookahead
            let lookahead_view = new_view + LOOK_AHEAD;
            if !self.quorum_exchange.is_leader(lookahead_view) {
                self.quorum_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollFutureLeader(
                        *lookahead_view,
                        self.quorum_exchange.get_leader(lookahead_view),
                    ))
                    .await;
            }

            // Start polling for proposals for the new view
            self.quorum_exchange
                .network()
                .inject_consensus_info(ConsensusIntentEvent::PollForProposal(*self.cur_view + 1))
                .await;

            self.quorum_exchange
                .network()
                .inject_consensus_info(ConsensusIntentEvent::PollForDAC(*self.cur_view + 1))
                .await;

            if self.quorum_exchange.is_leader(self.cur_view + 1) {
                debug!("Polling for quorum votes for view {}", *self.cur_view);
                self.quorum_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollForVotes(*self.cur_view))
                    .await;
            }

            self.event_stream
                .publish(HotShotEvent::ViewChange(new_view))
                .await;

            // Spawn a timeout task if we did actually update view
            let timeout = self.timeout;
            self.timeout_task = async_spawn({
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
            });
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
    pub async fn handle_event(&mut self, event: HotShotEvent<TYPES, I>) {
        match event {
            HotShotEvent::QuorumProposalRecv(proposal, sender) => {
                debug!(
                    "Receved Quorum Propsoal for view {}",
                    *proposal.data.view_number
                );

                let view = proposal.data.get_view_number();
                if view < self.cur_view {
                    debug!("Proposal is from an older view {:?}", proposal.data.clone());
                    return;
                }

                let view_leader_key = self.quorum_exchange.get_leader(view);
                if view_leader_key != sender {
                    error!("Leader key does not match key in proposal");
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

                    if timeout_cert.view_number != view - 1 {
                        warn!("Timeout certificate for view {} was not for the immediately preceding view", *view);
                        return;
                    }

                    if !self
                        .timeout_exchange
                        .is_valid_timeout_cert(&timeout_cert.clone(), view - 1)
                    {
                        warn!("Timeout certificate for view {} was invalid", *view);
                        return;
                    }
                }

                let justify_qc = proposal.data.justify_qc.clone();

                if !justify_qc.is_valid_cert(self.quorum_exchange.membership()) {
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
                        height: proposal.data.height,
                        justify_qc: justify_qc.clone(),
                        parent_commitment: justify_qc.get_data().leaf_commit,
                        deltas: Right(proposal.data.block_commitment),
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
                    height: proposal.data.height,
                    justify_qc: justify_qc.clone(),
                    parent_commitment,
                    deltas: Right(proposal.data.block_commitment),
                    rejected: Vec::new(),
                    timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                    proposer_id: sender.to_bytes(),
                };
                let leaf_commitment = leaf.commit();

                // Validate the `height`
                // TODO Remove height from proposal validation; view number is sufficient
                // https://github.com/EspressoSystems/HotShot/issues/1796
                if leaf.height != parent.height + 1 {
                    error!(
                        "Incorrect height in proposal (expected {}, got {})",
                        parent.height + 1,
                        leaf.height
                    );
                    return;
                }
                // Validate the signature. This should also catch if the leaf_commitment does not equal our calculated parent commitment
                else if !view_leader_key.validate(&proposal.signature, leaf_commitment.as_ref()) {
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
                    self.api.send_view_error(view, Arc::new(e)).await;
                    return;
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
                                    consensus
                                    .metrics
                                    .last_synced_block_height
                                    .set(usize::try_from(leaf.height).unwrap_or(0));

                                            // If the full block is available for this leaf, include it in the leaf
                                            // chain that we send to the client.
                                            if let Some(block) =
                                                consensus.saved_blocks.get(leaf.get_deltas_commitment())
                                            {
                                                if let Err(err) = leaf.fill_deltas(block.clone()) {
                                                    error!("unable to fill leaf {} with block {}, block will not be available: {}",
                                                        leaf.commit(), block.commit(), err);
                                                }
                                            }

                                    leaf_views.push(leaf.clone());
                                    match &leaf.deltas {
                                        Left(block) => {
                                            let txns = block.contained_transactions();
                                            for txn in txns {
                                                included_txns.insert(txn);
                                            }
                                        }
                                        Right(_) => {}
                                }
                            }
                                true
                            },
                        ) {
                            error!("publishing view error");
                            self.output_event_stream.publish(Event {
                                view_number: view,
                                event: EventType::Error { error: e.into() },
                            }).await;
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
                    debug!("about to publish decide");
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
                let should_propose = self.quorum_exchange.is_leader(new_view)
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

                for v in (*self.cur_view)..=(*view) {
                    let time = TYPES::Time::new(v);
                    self.da_certs.remove(&time);
                }
            }
            HotShotEvent::QuorumVoteRecv(vote) => {
                debug!("Received quroum vote: {:?}", vote.get_view_number());

                if !self.quorum_exchange.is_leader(vote.get_view_number() + 1) {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view_number() + 1,
                        self.quorum_exchange.is_leader(vote.get_view_number() + 2)
                    );
                    return;
                }

                let handle_event = HandleEvent(Arc::new(move |event, state| {
                    async move { vote_handle(state, event).await }.boxed()
                }));
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

                // Todo check if we are the leader
                let new_accumulator = VoteAccumulator2 {
                    vote_outcomes: HashMap::new(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.quorum_exchange.total_nodes()],
                    phantom: PhantomData,
                };

                let accumulator =
                    new_accumulator.accumulate(&vote, self.quorum_exchange.membership());

                // TODO Create default functions for accumulators
                // https://github.com/EspressoSystems/HotShot/issues/1797
                let timeout_accumulator = TimeoutVoteAccumulator {
                    da_vote_outcomes: HashMap::new(),
                    success_threshold: self.timeout_exchange.success_threshold(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.timeout_exchange.total_nodes()],
                    phantom: PhantomData,
                };

                if vote.get_view_number() > collection_view {
                    let state = VoteCollectionTaskState {
                        quorum_exchange: self.quorum_exchange.clone(),
                        timeout_exchange: self.timeout_exchange.clone(),
                        accumulator,
                        timeout_accumulator: either::Left(timeout_accumulator),
                        cur_view: vote.get_view_number(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                    };
                    let name = "Quorum Vote Collection";
                    let filter = FilterEvent(Arc::new(|event| {
                        matches!(
                            event,
                            HotShotEvent::QuorumVoteRecv(_) | HotShotEvent::TimeoutVoteRecv(_)
                        )
                    }));

                    let builder =
                        TaskBuilder::<VoteCollectionTypes<TYPES, I>>::new(name.to_string())
                            .register_event_stream(self.event_stream.clone(), filter)
                            .await
                            .register_registry(&mut self.registry.clone())
                            .await
                            .register_state(state)
                            .register_event_handler(handle_event);
                    let id = builder.get_task_id().unwrap();
                    let stream_id = builder.get_stream_id().unwrap();

                    self.vote_collector = Some((vote.get_view_number(), id, stream_id));

                    let _task = async_spawn(async move {
                        VoteCollectionTypes::build(builder).launch().await;
                    });
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::QuorumVoteRecv(vote))
                        .await;
                }
            }
            HotShotEvent::TimeoutVoteRecv(vote) => {
                if !self.timeout_exchange.is_leader(vote.get_view() + 1) {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view() + 1,
                        self.timeout_exchange.is_leader(vote.get_view() + 2)
                    );
                    return;
                }

                let handle_event = HandleEvent(Arc::new(move |event, state| {
                    async move { vote_handle(state, event).await }.boxed()
                }));
                let collection_view =
                    if let Some((collection_view, collection_task, _)) = &self.vote_collector {
                        if vote.get_view() > *collection_view {
                            // ED I think we'd want to let that task timeout to avoid a griefing vector
                            self.registry.shutdown_task(*collection_task).await;
                        }
                        *collection_view
                    } else {
                        TYPES::Time::new(0)
                    };

                //         // Todo check if we are the leader
                let new_accumulator = TimeoutVoteAccumulator {
                    da_vote_outcomes: HashMap::new(),

                    success_threshold: self.timeout_exchange.success_threshold(),

                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.timeout_exchange.total_nodes()],
                    phantom: PhantomData,
                };

                let timeout_accumulator = self.timeout_exchange.accumulate_vote(
                    new_accumulator,
                    &vote,
                    &vote.get_view().commit(),
                );

                let quorum_accumulator = VoteAccumulator2 {
                    vote_outcomes: HashMap::new(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.quorum_exchange.total_nodes()],
                    phantom: PhantomData,
                };

                // self.timeout_accumulator = accumulator;

                if vote.get_view() > collection_view {
                    let state = VoteCollectionTaskState {
                        quorum_exchange: self.quorum_exchange.clone(),
                        timeout_exchange: self.timeout_exchange.clone(),
                        accumulator: either::Left(quorum_accumulator),
                        timeout_accumulator,
                        cur_view: vote.get_view(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                    };
                    let name = "Quorum Vote Collection";
                    let filter = FilterEvent(Arc::new(|event| {
                        matches!(
                            event,
                            HotShotEvent::QuorumVoteRecv(_) | HotShotEvent::TimeoutVoteRecv(_)
                        )
                    }));

                    let builder =
                        TaskBuilder::<VoteCollectionTypes<TYPES, I>>::new(name.to_string())
                            .register_event_stream(self.event_stream.clone(), filter)
                            .await
                            .register_registry(&mut self.registry.clone())
                            .await
                            .register_state(state)
                            .register_event_handler(handle_event);
                    let id = builder.get_task_id().unwrap();
                    let stream_id = builder.get_stream_id().unwrap();

                    self.vote_collector = Some((vote.get_view(), id, stream_id));

                    let _task = async_spawn(async move {
                        VoteCollectionTypes::build(builder).launch().await;
                    });
                    debug!("Starting vote handle for view {:?}", vote.get_view());
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::TimeoutVoteRecv(vote))
                        .await;
                }
            }
            HotShotEvent::QCFormed(cert) => {
                debug!("QC Formed event happened!");

                if let either::Right(qc) = cert.clone() {
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

                    drop(consensus);
                    debug!(
                        "Attempting to publish proposal after forming a QC for view {}",
                        *qc.view_number
                    );

                    if !self
                        .publish_proposal_if_able(qc.clone(), qc.view_number + 1, None)
                        .await
                    {
                        warn!("Wasn't able to publish proposal");
                    }
                }
            }
            HotShotEvent::DACRecv(cert) => {
                debug!("DAC Recved for view ! {}", *cert.view_number);

                let view = cert.view_number;
                self.da_certs.insert(view, cert);

                if self.vote_if_able().await {
                    self.current_proposal = None;
                }
            }
            HotShotEvent::VidCertRecv(cert) => {
                debug!("VID cert received for view ! {}", *cert.view_number);

                let view = cert.view_number;
                self.vid_certs.insert(view, cert);

                // TODO Make sure we aren't voting for an arbitrarily old round for no reason
                if self.vote_if_able().await {
                    self.current_proposal = None;
                }
            }
            HotShotEvent::ViewChange(new_view) => {
                debug!("View Change event for view {}", *new_view);

                let old_view_number = self.cur_view;

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
                let vote_token = self.timeout_exchange.make_vote_token(view);

                match vote_token {
                    Err(e) => {
                        error!("Failed to generate vote token for {:?} {:?}", view, e);
                    }
                    Ok(None) => {
                        debug!("We were not chosen for consensus committee on {:?}", view);
                    }
                    Ok(Some(vote_token)) => {
                        let message = self
                            .timeout_exchange
                            .create_timeout_message::<I>(view, vote_token);

                        debug!("Sending timeout vote for view {}", *view);
                        if let GeneralConsensusMessage::TimeoutVote(vote) = message {
                            self.event_stream
                                .publish(HotShotEvent::TimeoutVoteSend(vote))
                                .await;
                        }
                    }
                }
                debug!(
                    "We did not receive evidence for view {} in time, sending timeout vote for that view!",
                    *view
                );
                let consensus = self.consensus.read().await;
                consensus.metrics.number_of_timeouts.add(1);
            }
            HotShotEvent::SendDABlockData(block) => {
                self.block = Some(block);
            }
            _ => {}
        }
    }

    /// Sends a proposal if possible from the high qc we have
    pub async fn publish_proposal_if_able(
        &mut self,
        _qc: QuorumCertificate2<TYPES, I::Leaf>,
        view: TYPES::Time,
        timeout_certificate: Option<TimeoutCertificate<TYPES>>,
    ) -> bool {
        if !self.quorum_exchange.is_leader(view) {
            error!(
                "Somehow we formed a QC but are not the leader for the next view {:?}",
                view
            );
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

        // let block_commitment = Some(self.block.commit());
        if let Some(block) = &self.block {
            let block_commitment = block.commit();

            let leaf = Leaf {
                view_number: view,
                height: parent_leaf.height + 1,
                justify_qc: consensus.high_qc.clone(),
                parent_commitment: parent_leaf.commit(),
                // Use the block commitment rather than the block, so that the replica can construct
                // the same leaf with the commitment.
                deltas: Right(block_commitment),
                rejected: vec![],
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
                proposer_id: self.api.public_key().to_bytes(),
            };

            let signature = self
                .quorum_exchange
                .sign_validating_or_commitment_proposal::<I>(&leaf.commit());
            // TODO: DA cert is sent as part of the proposal here, we should split this out so we don't have to wait for it.
            let proposal = QuorumProposal {
                block_commitment,
                view_number: leaf.view_number,
                height: leaf.height,
                justify_qc: consensus.high_qc.clone(),
                timeout_certificate: timeout_certificate.or_else(|| None),
                proposer_id: leaf.proposer_id,
                dac: None,
            };

            let message = Proposal {
                data: proposal,
                signature,
            };
            debug!(
                "Sending proposal for view {:?} \n {:?}",
                leaf.view_number, ""
            );

            self.event_stream
                .publish(HotShotEvent::QuorumProposalSend(
                    message,
                    self.quorum_exchange.public_key().clone(),
                ))
                .await;
            self.block = None;
            return true;
        }
        debug!("Self block was None");
        false
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I>,
    > TS for ConsensusTaskState<TYPES, I, A>
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockType>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
}

/// Type allias for consensus' vote collection task
pub type VoteCollectionTypes<TYPES, I> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    VoteCollectionTaskState<TYPES, I>,
>;

/// Type alias for Consensus task
pub type ConsensusTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    ConsensusTaskState<TYPES, I, A>,
>;

/// Event handle for consensus
pub async fn sequencing_consensus_handle<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>, ConsensusMessage = SequencingMessage<TYPES, I>>,
    A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
>(
    event: HotShotEvent<TYPES, I>,
    mut state: ConsensusTaskState<TYPES, I, A>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    ConsensusTaskState<TYPES, I, A>,
)
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<Leaf<TYPES>>>,
        Commitment = Commitment<Leaf<TYPES>>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockType>,
    >,
    TimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, Leaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
    if let HotShotEvent::Shutdown = event {
        (Some(HotShotTaskCompleted::ShutDown), state)
    } else {
        state.handle_event(event).await;
        (None, state)
    }
}

/// Filter for consensus, returns true for event types the consensus task subscribes to.
pub fn consensus_event_filter<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    event: &HotShotEvent<TYPES, I>,
) -> bool {
    matches!(
        event,
        HotShotEvent::QuorumProposalRecv(_, _)
            | HotShotEvent::QuorumVoteRecv(_)
            | HotShotEvent::QCFormed(_)
            | HotShotEvent::DACRecv(_)
            | HotShotEvent::VidCertRecv(_)
            | HotShotEvent::ViewChange(_)
            | HotShotEvent::SendDABlockData(_)
            | HotShotEvent::Timeout(_)
            | HotShotEvent::TimeoutVoteRecv(_)
            | HotShotEvent::Shutdown,
    )
}
