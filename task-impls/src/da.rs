use crate::events::SequencingHotShotEvent;
use async_compatibility_layer::{
    art::{async_spawn, async_timeout},
    async_primitives::subscribable_rwlock::ReadView,
};
use async_lock::RwLock;
use bincode::config::Options;
use bitvec::prelude::*;
use commit::Committable;
use either::{Either, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::vote::DAVoteAccumulator;
use hotshot_types::traits::election::SignedCertificate;
use hotshot_types::vote::AccumulatorPlaceholder;
use hotshot_types::vote::VoteType;
use hotshot_types::{
    certificate::DACertificate,
    consensus::{Consensus, View},
    data::{DAProposal, ProposalType, SequencingLeaf},
    message::{CommitteeConsensusMessage, Message, Proposal, SequencingMessage},
    traits::{
        consensus_api::SequencingConsensusApi,
        election::{CommitteeExchangeType, ConsensusExchange, Membership},
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{CommitteeEx, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        state::ConsensusTime,
        Block, State,
    },
    utils::ViewInner,
    vote::VoteAccumulator,
};
use hotshot_utils::bincode::bincode_opts;
use snafu::Snafu;
use std::marker::PhantomData;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Instant,
};
use tracing::{debug, error, instrument, warn};

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a DA task
pub struct DATaskState<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
> where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// The state's api
    pub api: A,
    /// Global registry task for the state
    pub registry: GlobalRegistry,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    // pub transactions: Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>>,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,

    /// This state's ID
    pub id: u64,
}

/// Struct to maintain DA Vote Collection task state
pub struct DAVoteCollectionTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = SequencingLeaf<TYPES>>,
> where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,
    /// the vote accumulator
    pub accumulator:
        Either<VoteAccumulator<TYPES::VoteTokenType, TYPES::BlockType>, DACertificate<TYPES>>,

    /// The accumulator
    #[allow(clippy::type_complexity)]
    pub accumulator2: Either<
        <DACertificate<TYPES> as SignedCertificate<
            TYPES,
            TYPES::Time,
            TYPES::VoteTokenType,
            TYPES::BlockType,
        >>::VoteAccumulator,
        DACertificate<TYPES>,
    >,
    // TODO ED Make this just "view" since it is only for this task
    /// the current view
    pub cur_view: TYPES::Time,
    /// event stream for channel events
    pub event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    /// the id of this task state
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = SequencingLeaf<TYPES>>> TS
    for DAVoteCollectionTaskState<TYPES, I>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
}

#[instrument(skip_all, fields(id = state.id, view = *state.cur_view), name = "DA Vote Collection Task", level = "error")]
async fn vote_handle<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = SequencingLeaf<TYPES>>>(
    mut state: DAVoteCollectionTaskState<TYPES, I>,
    event: SequencingHotShotEvent<TYPES, I>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    DAVoteCollectionTaskState<TYPES, I>,
)
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    match event {
        SequencingHotShotEvent::DAVoteRecv(vote) => {
            debug!("DA vote recv, collection task {:?}", vote.current_view);
            // panic!("Vote handle received DA vote for view {}", *vote.current_view);

            // For the case where we receive votes after we've made a certificate
            if state.accumulator2.is_right() {
                debug!("DA accumulator finished view: {:?}", state.cur_view);
                return (None, state);
            }

            let accumulator2 = state.accumulator2.left().unwrap();
            // TODO ED Maybe we don't need this to take in commitment?  Can just get it from the vote directly if it is always
            // going to be passed in as the vote.commitment
            match state.committee_exchange.accumulate_vote_2(
                accumulator2,
                &vote,
                &vote.block_commitment,
            ) {
                Left(new_accumulator) => {
                    error!("DA cert still accumulating");
                    state.accumulator2 = either::Left(new_accumulator);
                }
                Right(dac) => {
                    panic!("DA cert made!");
                    state.accumulator2 = either::Right(dac);
                }
            }

            let accumulator = state.accumulator.left().unwrap();
            match state.committee_exchange.accumulate_vote(
                &vote.signature.0,
                &vote.signature.1,
                vote.block_commitment,
                vote.vote_data,
                vote.vote_token.clone(),
                state.cur_view,
                accumulator,
                None,
            ) {
                Left(acc) => {
                    state.accumulator = Either::Left(acc);
                    // debug!("Not enough DA votes! ");
                    return (None, state);
                }
                Right(dac) => {
                    debug!("Sending DAC! {:?}", dac.view_number);
                    state
                        .event_stream
                        .publish(SequencingHotShotEvent::DACSend(
                            dac.clone(),
                            state.committee_exchange.public_key().clone(),
                        ))
                        .await;

                    state.accumulator = Right(dac.clone());
                    state
                        .committee_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *dac.view_number,
                        ))
                        .await;

                    // Return completed at this point
                    return (Some(HotShotTaskCompleted::ShutDown), state);
                }
            }
        }
        SequencingHotShotEvent::Shutdown => return (Some(HotShotTaskCompleted::ShutDown), state),
        _ => {}
    }
    (None, state)
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > DATaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]

    pub async fn handle_event(
        &mut self,
        event: SequencingHotShotEvent<TYPES, I>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            SequencingHotShotEvent::TransactionsRecv(transactions) => {
                // TODO ED Add validation checks

                let mut consensus = self.consensus.write().await;
                consensus
                    .get_transactions()
                    .modify(|txns| {
                        for transaction in transactions {
                            let size = bincode_opts().serialized_size(&transaction).unwrap_or(0);

                            // If we didn't already know about this transaction, update our mempool metrics.
                            if !consensus.seen_transactions.remove(&transaction.commit())
                                && txns.insert(transaction.commit(), transaction).is_none()
                            {
                                consensus.metrics.outstanding_transactions.update(1);
                                consensus
                                    .metrics
                                    .outstanding_transactions_memory_size
                                    .update(i64::try_from(size).unwrap_or_else(|e| {
                                        warn!("Conversion failed: {e}. Using the max value.");
                                        i64::MAX
                                    }));
                            }
                        }
                    })
                    .await;

                return None;
            }
            SequencingHotShotEvent::DAProposalRecv(proposal, sender) => {
                debug!(
                    "DA proposal received for view: {:?}",
                    proposal.data.get_view_number()
                );
                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = proposal.data.get_view_number();

                // Allow a DA proposal that is one view older, in case we have voted on a quorum
                // proposal and updated the view.
                // `self.cur_view` should be at least 1 since there is a view change before getting
                // the `DAProposalRecv` event. Otherewise, the view number subtraction below will
                // cause an overflow error.
                if view < self.cur_view - 1 {
                    warn!("Throwing away DA proposal that is more than one view older");
                    return None;
                }

                debug!(
                    "Got a DA block with {} transactions!",
                    proposal.data.deltas.contained_transactions().len()
                );
                let block_commitment = proposal.data.deltas.commit();

                // ED Is this the right leader?
                let view_leader_key = self.committee_exchange.get_leader(view);
                if view_leader_key != sender {
                    error!("DA proposal doesn't have expected leader key for view {} \n DA proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                if !view_leader_key.validate(&proposal.signature, block_commitment.as_ref()) {
                    error!("Could not verify proposal.");
                    return None;
                }

                let vote_token = self.committee_exchange.make_vote_token(view);
                match vote_token {
                    Err(e) => {
                        error!("Failed to generate vote token for {:?} {:?}", view, e);
                    }
                    Ok(None) => {
                        debug!("We were not chosen for DA committee on {:?}", view);
                    }
                    Ok(Some(vote_token)) => {
                        // Generate and send vote
                        let message = self.committee_exchange.create_da_message(
                            block_commitment,
                            view,
                            vote_token,
                        );

                        // ED Don't think this is necessary?
                        // self.cur_view = view;

                        if let CommitteeConsensusMessage::DAVote(vote) = message {
                            debug!("Sending vote to the DA leader {:?}", vote.current_view);
                            self.event_stream
                                .publish(SequencingHotShotEvent::DAVoteSend(vote))
                                .await;
                        }
                        let mut consensus = self.consensus.write().await;

                        // Ensure this view is in the view map for garbage collection, but do not overwrite if
                        // there is already a view there: the replica task may have inserted a `Leaf` view which
                        // contains strictly more information.
                        consensus.state_map.entry(view).or_insert(View {
                            view_inner: ViewInner::DA {
                                block: block_commitment,
                            },
                        });

                        // Record the block we have promised to make available.
                        consensus.saved_blocks.insert(proposal.data.deltas);
                    }
                }
            }
            SequencingHotShotEvent::DAVoteRecv(vote) => {
                // warn!(
                //     "DA vote recv, Main Task {:?}, key: {:?}",
                //     vote.current_view,
                //     self.committee_exchange.public_key()
                // );
                // Check if we are the leader and the vote is from the sender.
                let view = vote.current_view;
                if !self.committee_exchange.is_leader(view) {
                    error!("We are not the committee leader for view {} are we leader for next view? {}", *view, self.committee_exchange.is_leader(view + 1));
                    return None;
                }

                let handle_event = HandleEvent(Arc::new(move |event, state| {
                    async move { vote_handle(state, event).await }.boxed()
                }));
                let collection_view =
                    if let Some((collection_view, collection_id, _)) = &self.vote_collector {
                        // TODO: Is this correct for consecutive leaders?
                        if view > *collection_view {
                            // warn!("shutting down for view {:?}", collection_view);
                            self.registry.shutdown_task(*collection_id).await;
                        }
                        *collection_view
                    } else {
                        TYPES::Time::new(0)
                    };
                let acc = VoteAccumulator {
                    total_vote_outcomes: HashMap::new(),
                    da_vote_outcomes: HashMap::new(),
                    yes_vote_outcomes: HashMap::new(),
                    no_vote_outcomes: HashMap::new(),
                    viewsync_precommit_vote_outcomes: HashMap::new(),
                    viewsync_commit_vote_outcomes: HashMap::new(),
                    viewsync_finalize_vote_outcomes: HashMap::new(),
                    success_threshold: self.committee_exchange.success_threshold(),
                    failure_threshold: self.committee_exchange.failure_threshold(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.committee_exchange.total_nodes()],
                };
                let accumulator = self.committee_exchange.accumulate_vote(
                    &vote.clone().signature.0,
                    &vote.clone().signature.1,
                    vote.clone().block_commitment,
                    vote.clone().vote_data.clone(),
                    vote.clone().vote_token.clone(),
                    vote.clone().current_view,
                    acc,
                    None,
                );
                let accumulator2 = DAVoteAccumulator {
                    da_vote_outcomes: HashMap::new(),
                    success_threshold: self.committee_exchange.success_threshold(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.committee_exchange.total_nodes()],
                    phantom: PhantomData,
                };
                if view > collection_view {
                    let state = DAVoteCollectionTaskState {
                        committee_exchange: self.committee_exchange.clone(),
                        accumulator,
                        accumulator2: either::Left(accumulator2),
                        cur_view: view,
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                    };
                    let name = "DA Vote Collection";
                    let filter = FilterEvent(Arc::new(|event| {
                        matches!(event, SequencingHotShotEvent::DAVoteRecv(_))
                    }));
                    let builder =
                        TaskBuilder::<DAVoteCollectionTypes<TYPES, I>>::new(name.to_string())
                            .register_event_stream(self.event_stream.clone(), filter)
                            .await
                            .register_registry(&mut self.registry.clone())
                            .await
                            .register_state(state)
                            .register_event_handler(handle_event);
                    let id = builder.get_task_id().unwrap();
                    let stream_id = builder.get_stream_id().unwrap();
                    let _task =
                        async_spawn(
                            async move { DAVoteCollectionTypes::build(builder).launch().await },
                        );
                    self.vote_collector = Some((view, id, stream_id));
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, SequencingHotShotEvent::DAVoteRecv(vote))
                        .await;
                };
            }
            // TODO ED Update high QC through QCFormed event
            SequencingHotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;
                // Inject view info into network
                // ED I think it is possible that you receive a quorum proposal, vote on it and update your view before the da leader has sent their proposal, and therefore you skip polling for this view?

                // TODO ED Only poll if you are on the committee
                let is_da = self
                    .committee_exchange
                    .membership()
                    .get_committee(self.cur_view + 1)
                    .contains(self.committee_exchange.public_key());

                if is_da {
                    debug!("Polling for DA proposals for view {}", *self.cur_view + 1);
                    self.committee_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(
                            *self.cur_view + 1,
                        ))
                        .await;
                }
                if self.committee_exchange.is_leader(self.cur_view + 3) {
                    debug!("Polling for transactions for view {}", *self.cur_view + 3);
                    self.committee_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForTransactions(
                            *self.cur_view + 3,
                        ))
                        .await;
                }

                // TODO ED Make this a new task so it doesn't block main DA task

                // If we are not the next leader (DA leader for this view) immediately exit
                if !self.committee_exchange.is_leader(self.cur_view + 1) {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }
                debug!("Polling for DA votes for view {}", *self.cur_view + 1);

                // Start polling for DA votes for the "next view"
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollForVotes(*self.cur_view + 1))
                    .await;

                // ED Copy of parent_leaf() function from sequencing leader

                let consensus = self.consensus.read().await;
                let parent_view_number = &consensus.high_qc.view_number;

                let Some(parent_view) = consensus.state_map.get(parent_view_number) else {
                    error!(
                        "Couldn't find high QC parent in state map. Parent view {:?}",
                        parent_view_number
                    );
                    return None;
                };
                let Some(leaf) = parent_view.get_leaf_commitment() else {
                    error!(
                        ?parent_view_number,
                        ?parent_view,
                        "Parent of high QC points to a view without a proposal"
                    );
                    return None;
                };
                let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
                    error!("Failed to find high QC parent.");
                    return None;
                };
                let parent_leaf = leaf.clone();

                // Prepare the DA Proposal
                //         let Some(parent_leaf) = self.parent_leaf().await else {
                //     warn!("Couldn't find high QC parent in state map.");
                //     return None;
                // };

                drop(consensus);

                let mut block = <TYPES as NodeType>::StateType::next_block(None);
                let txns = self.wait_for_transactions(parent_leaf).await?;

                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForTransactions(
                        *self.cur_view + 1,
                    ))
                    .await;

                for txn in txns {
                    if let Ok(new_block) = block.add_transaction_raw(&txn) {
                        block = new_block;
                        continue;
                    }
                }

                let signature = self.committee_exchange.sign_da_proposal(&block.commit());
                let data: DAProposal<TYPES> = DAProposal {
                    deltas: block.clone(),
                    // Upon entering a new view we want to send a DA Proposal for the next view -> Is it always the case that this is cur_view + 1?
                    view_number: self.cur_view + 1,
                };
                debug!("Sending DA proposal for view {:?}", data.view_number);

                // let message = SequencingMessage::<TYPES, I>(Right(
                //     CommitteeConsensusMessage::DAProposal(Proposal { data, signature }),
                // ));
                let message = Proposal { data, signature };
                // Brodcast DA proposal
                // TODO ED We should send an event to do this, but just getting it to work for now

                self.event_stream
                    .publish(SequencingHotShotEvent::SendDABlockData(block.clone()))
                    .await;
                // if let Err(e) = self.api.send_da_broadcast(message.clone()).await {
                //     consensus.metrics.failed_to_send_messages.add(1);
                //     warn!(?message, ?e, "Could not broadcast leader proposal");
                // } else {
                //     consensus.metrics.outgoing_broadcast_messages.add(1);
                // }
                self.event_stream
                    .publish(SequencingHotShotEvent::DAProposalSend(
                        message,
                        self.committee_exchange.public_key().clone(),
                    ))
                    .await;

                return None;
            }

            SequencingHotShotEvent::Timeout(view) => {
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;
            }

            SequencingHotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {}
        }
        None
    }

    /// return None if we can't get transactions
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Vote Collection Task", level = "error")]

    async fn wait_for_transactions(
        &self,
        parent_leaf: SequencingLeaf<TYPES>,
    ) -> Option<Vec<TYPES::Transaction>> {
        let task_start_time = Instant::now();

        // let parent_leaf = self.parent_leaf().await?;
        let previous_used_txns = match parent_leaf.deltas {
            Either::Left(block) => block.contained_transactions(),
            Either::Right(_commitment) => HashSet::new(),
        };

        let consensus = self.consensus.read().await;

        let receiver = consensus.transactions.subscribe().await;

        loop {
            let all_txns = consensus.transactions.cloned().await;
            debug!("Size of transactions: {}", all_txns.len());
            let unclaimed_txns: Vec<_> = all_txns
                .iter()
                .filter(|(txn_hash, _txn)| !previous_used_txns.contains(txn_hash))
                .collect();

            let time_past = task_start_time.elapsed();
            if unclaimed_txns.len() < self.api.min_transactions()
                && (time_past < self.api.propose_max_round_time())
            {
                let duration = self.api.propose_max_round_time() - time_past;
                let result = async_timeout(duration, receiver.recv()).await;
                match result {
                    Err(_) => {
                        // Fall through below to updating new block
                        error!(
                            "propose_max_round_time passed, sending transactions we have so far"
                        );
                    }
                    Ok(Err(e)) => {
                        // Something unprecedented is wrong, and `transactions` has been dropped
                        error!("Channel receiver error for SubscribableRwLock {:?}", e);
                        return None;
                    }
                    Ok(Ok(_)) => continue,
                }
            }
            break;
        }
        let all_txns = consensus.transactions.cloned().await;
        let txns: Vec<TYPES::Transaction> = all_txns
            .iter()
            .filter_map(|(txn_hash, txn)| {
                if previous_used_txns.contains(txn_hash) {
                    None
                } else {
                    Some(txn.clone())
                }
            })
            .collect();
        Some(txns)
    }

    /// Filter the DA event.
    pub fn filter(event: &SequencingHotShotEvent<TYPES, I>) -> bool {
        matches!(
            event,
            SequencingHotShotEvent::DAProposalRecv(_, _)
                | SequencingHotShotEvent::DAVoteRecv(_)
                | SequencingHotShotEvent::Shutdown
                | SequencingHotShotEvent::TransactionsRecv(_)
                | SequencingHotShotEvent::Timeout(_)
                | SequencingHotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for DA Task
impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > TS for DATaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
}

/// Type alias for DA Vote Collection Types
pub type DAVoteCollectionTypes<TYPES, I> = HSTWithEvent<
    ConsensusTaskError,
    SequencingHotShotEvent<TYPES, I>,
    ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    DAVoteCollectionTaskState<TYPES, I>,
>;

/// Type alias for DA Task Types
pub type DATaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    SequencingHotShotEvent<TYPES, I>,
    ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    DATaskState<TYPES, I, A>,
>;
