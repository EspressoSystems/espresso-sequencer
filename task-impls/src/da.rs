use crate::events::HotShotEvent;
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;

use bitvec::prelude::*;
use commit::{Commitment, Committable};
use either::{Either, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::{
    consensus::{Consensus, View},
    data::{DAProposal, Leaf, ProposalType},
    message::{Message, Proposal},
    simple_vote::{DAData, DAVote2},
    traits::{
        consensus_api::ConsensusApi,
        election::{CommitteeExchangeType, ConsensusExchange, Membership},
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{CommitteeEx, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        state::ConsensusTime,
        BlockPayload,
    },
    utils::ViewInner,
    vote2::HasViewNumber,
    vote2::VoteAccumulator2,
};
use hotshot_types::{
    simple_certificate::DACertificate2, traits::node_implementation::CommitteeMembership,
};

use snafu::Snafu;
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tracing::{debug, error, instrument, warn};

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a DA task
pub struct DATaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>,
    A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
> where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// The state's api
    pub api: A,
    /// Global registry task for the state
    pub registry: GlobalRegistry,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, Leaf<TYPES>>>>,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,

    /// This state's ID
    pub id: u64,
}

/// Struct to maintain DA Vote Collection task state
pub struct DAVoteCollectionTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>,
> where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,
    #[allow(clippy::type_complexity)]
    /// Accumulates DA votes
    pub accumulator: Either<
        VoteAccumulator2<
            TYPES,
            DAVote2<TYPES, CommitteeMembership<TYPES, I>>,
            DACertificate2<TYPES>,
        >,
        DACertificate2<TYPES>,
    >,
    /// the current view
    pub cur_view: TYPES::Time,
    /// event stream for channel events
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
    /// the id of this task state
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>> TS
    for DAVoteCollectionTaskState<TYPES, I>
where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
}

#[instrument(skip_all, fields(id = state.id, view = *state.cur_view), name = "DA Vote Collection Task", level = "error")]
async fn vote_handle<TYPES: NodeType, I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>>(
    mut state: DAVoteCollectionTaskState<TYPES, I>,
    event: HotShotEvent<TYPES, I>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    DAVoteCollectionTaskState<TYPES, I>,
)
where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    match event {
        HotShotEvent::DAVoteRecv(vote) => {
            debug!("DA vote recv, collection task {:?}", vote.get_view_number());
            // panic!("Vote handle received DA vote for view {}", *vote.get_view_number());

            // For the case where we receive votes after we've made a certificate
            if state.accumulator.is_right() {
                debug!("DA accumulator finished view: {:?}", state.cur_view);
                return (None, state);
            }

            let accumulator = state.accumulator.left().unwrap();

            match accumulator.accumulate(&vote, state.committee_exchange.membership()) {
                Left(new_accumulator) => {
                    state.accumulator = either::Left(new_accumulator);
                }

                Right(dac) => {
                    debug!("Sending DAC! {:?}", dac.view_number);
                    state
                        .event_stream
                        .publish(HotShotEvent::DACSend(
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
        HotShotEvent::Shutdown => return (Some(HotShotTaskCompleted::ShutDown), state),
        _ => {
            error!("unexpected event {:?}", event);
        }
    }
    (None, state)
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
    > DATaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            HotShotEvent::DAProposalRecv(proposal, sender) => {
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
                // TODO ED Come back to this - we probably don't need this, but we should also never receive a DAC where this fails, investigate block ready so it doesn't make one for the genesis block

                // stop polling for the received proposal
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(
                        *proposal.data.view_number,
                    ))
                    .await;

                if self.cur_view != TYPES::Time::genesis() && view < self.cur_view - 1 {
                    warn!("Throwing away DA proposal that is more than one view older");
                    return None;
                }

                debug!(
                    "Got a DA block with {} transactions!",
                    proposal.data.block_payload.transaction_commitments().len()
                );
                let payload_commitment = proposal.data.block_payload.commit();

                // ED Is this the right leader?
                let view_leader_key = self.committee_exchange.get_leader(view);
                if view_leader_key != sender {
                    error!("DA proposal doesn't have expected leader key for view {} \n DA proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                if !view_leader_key.validate(&proposal.signature, payload_commitment.as_ref()) {
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
                    Ok(Some(_vote_token)) => {
                        // Generate and send vote
                        let vote = DAVote2::create_signed_vote(
                            DAData {
                                payload_commit: payload_commitment,
                            },
                            view,
                            self.committee_exchange.public_key(),
                            self.committee_exchange.private_key(),
                        );

                        // ED Don't think this is necessary?
                        // self.cur_view = view;

                        debug!("Sending vote to the DA leader {:?}", vote.get_view_number());
                        self.event_stream
                            .publish(HotShotEvent::DAVoteSend(vote))
                            .await;
                        let mut consensus = self.consensus.write().await;

                        // Ensure this view is in the view map for garbage collection, but do not overwrite if
                        // there is already a view there: the replica task may have inserted a `Leaf` view which
                        // contains strictly more information.
                        consensus.state_map.entry(view).or_insert(View {
                            view_inner: ViewInner::DA {
                                block: payload_commitment,
                            },
                        });

                        // Record the block payload we have promised to make available.
                        consensus
                            .saved_block_payloads
                            .insert(proposal.data.block_payload);
                    }
                }
            }
            HotShotEvent::DAVoteRecv(vote) => {
                debug!("DA vote recv, Main Task {:?}", vote.get_view_number(),);
                // Check if we are the leader and the vote is from the sender.
                let view = vote.get_view_number();
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

                if view > collection_view {
                    let new_accumulator = VoteAccumulator2 {
                        vote_outcomes: HashMap::new(),
                        sig_lists: Vec::new(),
                        signers: bitvec![0; self.committee_exchange.total_nodes()],
                        phantom: PhantomData,
                    };

                    let accumulator =
                        new_accumulator.accumulate(&vote, self.committee_exchange.membership());

                    let state = DAVoteCollectionTaskState {
                        committee_exchange: self.committee_exchange.clone(),

                        accumulator,
                        cur_view: view,
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                    };
                    let name = "DA Vote Collection";
                    let filter = FilterEvent(Arc::new(|event| {
                        matches!(event, HotShotEvent::DAVoteRecv(_))
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
                        .direct_message(stream_id, HotShotEvent::DAVoteRecv(vote))
                        .await;
                };
            }
            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // Inject view info into network
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

                return None;
            }
            HotShotEvent::BlockReady(block, view) => {
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForTransactions(*view))
                    .await;

                let payload_commitment = block.commit();
                let signature = self
                    .committee_exchange
                    .sign_da_proposal(&payload_commitment);
                let data: DAProposal<TYPES> = DAProposal {
                    block_payload: block.clone(),
                    // Upon entering a new view we want to send a DA Proposal for the next view -> Is it always the case that this is cur_view + 1?
                    view_number: view,
                };
                debug!("Sending DA proposal for view {:?}", data.view_number);

                let message = Proposal { data, signature };

                self.event_stream
                    .publish(HotShotEvent::SendPayloadCommitment(payload_commitment))
                    .await;
                self.event_stream
                    .publish(HotShotEvent::DAProposalSend(
                        message.clone(),
                        self.committee_exchange.public_key().clone(),
                    ))
                    .await;
            }

            HotShotEvent::Timeout(view) => {
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;
            }

            HotShotEvent::Shutdown => {
                error!("Shutting down because of shutdown signal!");
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }

    /// Filter the DA event.
    pub fn filter(event: &HotShotEvent<TYPES, I>) -> bool {
        matches!(
            event,
            HotShotEvent::DAProposalRecv(_, _)
                | HotShotEvent::DAVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::BlockReady(_, _)
                | HotShotEvent::Timeout(_)
                | HotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for DA Task
impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>>,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
    > TS for DATaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
}

/// Type alias for DA Vote Collection Types
pub type DAVoteCollectionTypes<TYPES, I> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    DAVoteCollectionTaskState<TYPES, I>,
>;

/// Type alias for DA Task Types
pub type DATaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    DATaskState<TYPES, I, A>,
>;
