use crate::events::HotShotEvent;
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;

use bitvec::prelude::*;
use commit::Commitment;
use either::{Either, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::traits::{network::ConsensusIntentEvent, node_implementation::VIDMembership};
use hotshot_types::{
    consensus::{Consensus, View},
    message::{Message, SequencingMessage},
    traits::{
        consensus_api::ConsensusApi,
        election::ConsensusExchange,
        node_implementation::{NodeImplementation, NodeType, VIDEx},
        signature_key::SignatureKey,
        state::ConsensusTime,
    },
    utils::ViewInner,
};
use hotshot_types::{
    simple_certificate::VIDCertificate2,
    simple_vote::{VIDData, VIDVote2},
    traits::network::CommunicationChannel,
    vote2::{HasViewNumber, VoteAccumulator2},
};

use snafu::Snafu;
use std::marker::PhantomData;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, instrument, warn};

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a VID task
pub struct VIDTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    A: ConsensusApi<TYPES, I> + 'static,
> where
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// The state's api
    pub api: A,
    /// Global registry task for the state
    pub registry: GlobalRegistry,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// the VID exchange
    pub vid_exchange: Arc<VIDEx<TYPES, I>>,

    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,

    /// This state's ID
    pub id: u64,
}

/// Struct to maintain VID Vote Collection task state
pub struct VIDVoteCollectionTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>>
where
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// the vid exchange
    pub vid_exchange: Arc<VIDEx<TYPES, I>>,
    #[allow(clippy::type_complexity)]
    /// Accumulates VID votes
    pub accumulator: Either<
        VoteAccumulator2<TYPES, VIDVote2<TYPES, VIDMembership<TYPES, I>>, VIDCertificate2<TYPES>>,
        VIDCertificate2<TYPES>,
    >,
    /// the current view
    pub cur_view: TYPES::Time,
    /// event stream for channel events
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
    /// the id of this task state
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TS for VIDVoteCollectionTaskState<TYPES, I> where
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>
{
}

#[instrument(skip_all, fields(id = state.id, view = *state.cur_view), name = "VID Vote Collection Task", level = "error")]
async fn vote_handle<TYPES, I>(
    mut state: VIDVoteCollectionTaskState<TYPES, I>,
    event: HotShotEvent<TYPES, I>,
) -> (
    Option<HotShotTaskCompleted>,
    VIDVoteCollectionTaskState<TYPES, I>,
)
where
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    match event {
        HotShotEvent::VidVoteRecv(vote) => {
            debug!(
                "VID vote recv, collection task {:?}",
                vote.get_view_number()
            );
            // panic!("Vote handle received VID vote for view {}", *vote.current_view);

            // For the case where we receive votes after we've made a certificate
            if state.accumulator.is_right() {
                debug!("VID accumulator finished view: {:?}", state.cur_view);
                return (None, state);
            }

            let accumulator = state.accumulator.left().unwrap();
            match accumulator.accumulate(&vote, state.vid_exchange.membership()) {
                Left(new_accumulator) => {
                    state.accumulator = either::Left(new_accumulator);
                }

                Right(vid_cert) => {
                    debug!("Sending VID cert! {:?}", vid_cert.view_number);
                    state
                        .event_stream
                        .publish(HotShotEvent::VidCertSend(
                            vid_cert.clone(),
                            state.vid_exchange.public_key().clone(),
                        ))
                        .await;

                    state.accumulator = Right(vid_cert.clone());
                    state
                        .vid_exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDVotes(
                            *vid_cert.view_number,
                        ))
                        .await;

                    // Return completed at this point
                    return (Some(HotShotTaskCompleted::ShutDown), state);
                }
            }
        }
        _ => {
            error!("unexpected event {:?}", event);
        }
    }
    (None, state)
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
        A: ConsensusApi<TYPES, I> + 'static,
    > VIDTaskState<TYPES, I, A>
where
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "VID Main Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            HotShotEvent::VidVoteRecv(vote) => {
                // warn!(
                //     "VID vote recv, Main Task {:?}, key: {:?}",
                //     vote.current_view,
                //     self.committee_exchange.public_key()
                // );
                // Check if we are the leader and the vote is from the sender.
                let view = vote.get_view_number();
                if !self.vid_exchange.is_leader(view) {
                    error!(
                        "We are not the VID leader for view {} are we leader for next view? {}",
                        *view,
                        self.vid_exchange.is_leader(view + 1)
                    );
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
                        signers: bitvec![0; self.vid_exchange.total_nodes()],
                        phantom: PhantomData,
                    };

                    let accumulator =
                        new_accumulator.accumulate(&vote, self.vid_exchange.membership());

                    let state = VIDVoteCollectionTaskState {
                        vid_exchange: self.vid_exchange.clone(),

                        accumulator,
                        cur_view: view,
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                    };
                    let name = "VID Vote Collection";
                    let filter = FilterEvent(Arc::new(|event| {
                        matches!(event, HotShotEvent::VidVoteRecv(_))
                    }));
                    let builder =
                        TaskBuilder::<VIDVoteCollectionTypes<TYPES, I>>::new(name.to_string())
                            .register_event_stream(self.event_stream.clone(), filter)
                            .await
                            .register_registry(&mut self.registry.clone())
                            .await
                            .register_state(state)
                            .register_event_handler(handle_event);
                    let id = builder.get_task_id().unwrap();
                    let stream_id = builder.get_stream_id().unwrap();
                    let _task = async_spawn(async move {
                        VIDVoteCollectionTypes::build(builder).launch().await
                    });
                    self.vote_collector = Some((view, id, stream_id));
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::VidVoteRecv(vote))
                        .await;
                };
            }
            HotShotEvent::VidDisperseRecv(disperse, sender) => {
                // TODO copy-pasted from DAProposalRecv https://github.com/EspressoSystems/HotShot/issues/1690
                debug!(
                    "VID disperse received for view: {:?}",
                    disperse.data.get_view_number()
                );

                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = disperse.data.get_view_number();

                // Allow VID disperse date that is one view older, in case we have updated the
                // view.
                // Adding `+ 1` on the LHS rather tahn `- 1` on the RHS, to avoid the overflow
                // error due to subtracting the genesis view number.
                if view + 1 < self.cur_view {
                    warn!("Throwing away VID disperse data that is more than one view older");
                    return None;
                }

                debug!("VID disperse data is fresh.");
                let payload_commitment = disperse.data.payload_commitment;

                // ED Is this the right leader?
                let view_leader_key = self.vid_exchange.get_leader(view);
                if view_leader_key != sender {
                    error!("VID proposal doesn't have expected leader key for view {} \n DA proposal is: [N/A for VID]", *view);
                    return None;
                }

                if !view_leader_key.validate(&disperse.signature, payload_commitment.as_ref()) {
                    error!("Could not verify VID proposal sig.");
                    return None;
                }

                let vote_token = self.vid_exchange.make_vote_token(view);
                match vote_token {
                    Err(e) => {
                        error!("Failed to generate vote token for {:?} {:?}", view, e);
                    }
                    Ok(None) => {
                        debug!("We were not chosen for VID quorum on {:?}", view);
                    }
                    Ok(Some(_vote_token)) => {
                        // Generate and send vote
                        let vote = VIDVote2::create_signed_vote(
                            VIDData {
                                payload_commit: payload_commitment,
                            },
                            view,
                            self.vid_exchange.public_key(),
                            self.vid_exchange.private_key(),
                        );

                        // ED Don't think this is necessary?
                        // self.cur_view = view;

                        debug!(
                            "Sending vote to the VID leader {:?}",
                            vote.get_view_number()
                        );
                        self.event_stream
                            .publish(HotShotEvent::VidVoteSend(vote))
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

                        // Record the block we have promised to make available.
                        // TODO https://github.com/EspressoSystems/HotShot/issues/1692
                        // consensus.saved_block_payloads.insert(proposal.data.block_payload);
                    }
                }
            }
            HotShotEvent::VidCertRecv(_) => {
                // RM TODO
            }
            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // Start polling for VID disperse for the new view
                self.vid_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *self.cur_view + 1,
                    ))
                    .await;

                self.vid_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDCertificate(
                        *self.cur_view + 1,
                    ))
                    .await;

                // If we are not the next leader, we should exit
                if !self.vid_exchange.is_leader(self.cur_view + 1) {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }

                // Start polling for VID votes for the "next view"
                self.vid_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDVotes(
                        *self.cur_view + 1,
                    ))
                    .await;

                return None;
            }

            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }

    /// Filter the VID event.
    pub fn filter(event: &HotShotEvent<TYPES, I>) -> bool {
        matches!(
            event,
            HotShotEvent::Shutdown
                | HotShotEvent::VidDisperseRecv(_, _)
                | HotShotEvent::VidVoteRecv(_)
                | HotShotEvent::VidCertRecv(_)
                | HotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for VID Task
impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
        A: ConsensusApi<TYPES, I> + 'static,
    > TS for VIDTaskState<TYPES, I, A>
where
    VIDEx<TYPES, I>:
        ConsensusExchange<TYPES, Message<TYPES, I>, Commitment = Commitment<TYPES::BlockPayload>>,
{
}

/// Type alias for VID Vote Collection Types
pub type VIDVoteCollectionTypes<TYPES, I> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    VIDVoteCollectionTaskState<TYPES, I>,
>;

/// Type alias for VID Task Types
pub type VIDTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    VIDTaskState<TYPES, I, A>,
>;
