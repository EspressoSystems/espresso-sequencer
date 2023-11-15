#![allow(clippy::module_name_repetitions)]
use crate::events::HotShotEvent;
use async_compatibility_layer::art::{async_sleep, async_spawn};

use either::Either::{self, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::simple_vote::ViewSyncFinalizeData;
use hotshot_types::{
    simple_certificate::{
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        ViewSyncCommitData, ViewSyncCommitVote, ViewSyncFinalizeVote, ViewSyncPreCommitData,
        ViewSyncPreCommitVote,
    },
    traits::{network::ConsensusIntentEvent, node_implementation::ViewSyncMembership},
    vote2::{Certificate2, HasViewNumber, Vote2, VoteAccumulator2},
};

use bitvec::prelude::*;
use hotshot_task::global_registry::GlobalRegistry;
use hotshot_types::{
    data::Leaf,
    message::{GeneralConsensusMessage, Message, SequencingMessage},
    traits::{
        consensus_api::ConsensusApi,
        election::{ConsensusExchange, ViewSyncExchangeType},
        network::CommunicationChannel,
        node_implementation::{NodeImplementation, NodeType, ViewSyncEx},
        state::ConsensusTime,
    },
};
use snafu::Snafu;
use std::{collections::HashMap, marker::PhantomData, sync::Arc, time::Duration};
use tracing::{debug, error, info, instrument};
#[derive(PartialEq, PartialOrd, Clone, Debug, Eq, Hash)]
/// Phases of view sync
pub enum ViewSyncPhase {
    /// No phase; before the protocol has begun
    None,
    /// PreCommit phase
    PreCommit,
    /// Commit phase
    Commit,
    /// Finalize phase
    Finalize,
}

#[derive(Default)]
/// Information about view sync sub-tasks
pub struct ViewSyncTaskInfo {
    /// Id of the event stream of a certain task
    event_stream_id: usize,
}

#[derive(Snafu, Debug)]
/// Stub of a view sync error
pub struct ViewSyncTaskError {}

/// Main view sync task state
pub struct ViewSyncTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>, ConsensusMessage = SequencingMessage<TYPES, I>>,
    A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static + std::clone::Clone,
> where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    /// Registry to register sub tasks
    pub registry: GlobalRegistry,
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
    /// View HotShot is currently in
    pub current_view: TYPES::Time,
    /// View HotShot wishes to be in
    pub next_view: TYPES::Time,
    /// View sync exchange
    pub exchange: Arc<ViewSyncEx<TYPES, I>>,
    /// HotShot consensus API
    pub api: A,
    /// Our node id; for logging
    pub id: u64,

    /// How many timeouts we've seen in a row; is reset upon a successful view change
    pub num_timeouts_tracked: u64,

    /// Map of running replica tasks
    pub replica_task_map: HashMap<TYPES::Time, ViewSyncTaskInfo>,

    /// Map of running relay tasks
    pub relay_task_map: HashMap<TYPES::Time, ViewSyncTaskInfo>,

    /// Timeout duration for view sync rounds
    pub view_sync_timeout: Duration,

    /// Last view we garbage collected old tasks
    pub last_garbage_collected_view: TYPES::Time,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static + std::clone::Clone,
    > TS for ViewSyncTaskState<TYPES, I, A>
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
}

/// Types for the main view sync task
pub type ViewSyncTaskStateTypes<TYPES, I, A> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    ViewSyncTaskState<TYPES, I, A>,
>;

/// State of a view sync replica task
pub struct ViewSyncReplicaTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>, ConsensusMessage = SequencingMessage<TYPES, I>>,
    A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
> where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    /// Timeout for view sync rounds
    pub view_sync_timeout: Duration,
    /// Current round HotShot is in
    pub current_view: TYPES::Time,
    /// Round HotShot wishes to be in
    pub next_view: TYPES::Time,
    /// The last seen phase of the view sync protocol
    pub phase: ViewSyncPhase,
    /// The relay index we are currently on
    pub relay: u64,
    /// Whether we have seen a finalized certificate
    pub finalized: bool,
    /// Whether we have already sent a view change event for `next_view`
    pub sent_view_change_event: bool,
    /// Our node id; for logging
    pub id: u64,

    /// View sync exchange
    pub exchange: Arc<ViewSyncEx<TYPES, I>>,
    /// HotShot consensus API
    pub api: A,
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
    > TS for ViewSyncReplicaTaskState<TYPES, I, A>
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
}

/// Types for view sync replica state
pub type ViewSyncReplicaTaskStateTypes<TYPES, I, A> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    ViewSyncReplicaTaskState<TYPES, I, A>,
>;

/// State of a view sync relay task
pub struct ViewSyncRelayTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES, Leaf = Leaf<TYPES>, ConsensusMessage = SequencingMessage<TYPES, I>>,
    VOTE: Vote2<TYPES>,
    CERTIFICATE: Certificate2<TYPES, Voteable = VOTE::Commitment>,
> {
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES, I>>,
    /// View sync exchange
    pub exchange: Arc<ViewSyncEx<TYPES, I>>,

    /// Vote accumulator
    #[allow(clippy::type_complexity)]
    pub accumulator: Either<VoteAccumulator2<TYPES, VOTE, CERTIFICATE>, CERTIFICATE>,
    /// Our node id; for logging
    pub id: u64,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        VOTE: Vote2<TYPES> + std::marker::Send + std::marker::Sync + 'static,
        CERTIFICATE: Certificate2<TYPES, Voteable = VOTE::Commitment>
            + std::marker::Send
            + std::marker::Sync
            + 'static,
    > TS for ViewSyncRelayTaskState<TYPES, I, VOTE, CERTIFICATE>
{
}

/// Types used by the view sync relay task
pub type ViewSyncRelayTaskStateTypes<TYPES, I, VOTE, CERTIFICATE> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES, I>,
    ChannelStream<HotShotEvent<TYPES, I>>,
    ViewSyncRelayTaskState<TYPES, I, VOTE, CERTIFICATE>,
>;

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static + std::clone::Clone,
    > ViewSyncTaskState<TYPES, I, A>
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Main Task", level = "error")]
    #[allow(clippy::type_complexity)]
    /// Handles incoming events for the main view sync task
    pub async fn handle_event(&mut self, event: HotShotEvent<TYPES, I>) {
        match &event {
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(certificate) => {
                info!("Received view sync cert for phase {:?}", certificate);

                // This certificate is old, we can throw it away
                // If next view = cert round, then that means we should already have a task running for it
                if self.current_view > certificate.get_view_number() {
                    debug!("Already in a higher view than the view sync message");
                    return;
                }

                if let Some(replica_task) =
                    self.replica_task_map.get(&certificate.get_view_number())
                {
                    // Forward event then return
                    debug!("Forwarding message");
                    self.event_stream
                        .direct_message(replica_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a replica task already running, so start one
                let mut replica_state: ViewSyncReplicaTaskState<TYPES, I, A> =
                    ViewSyncReplicaTaskState {
                        current_view: certificate.get_view_number(),
                        next_view: certificate.get_view_number(),
                        relay: 0,
                        finalized: false,
                        sent_view_change_event: false,
                        phase: ViewSyncPhase::None,
                        exchange: self.exchange.clone(),
                        api: self.api.clone(),
                        event_stream: self.event_stream.clone(),
                        view_sync_timeout: self.view_sync_timeout,
                        id: self.id,
                    };

                let result = replica_state.handle_event(event.clone()).await;

                if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                    // The protocol has finished
                    return;
                }

                replica_state = result.1;

                let name = format!(
                    "View Sync Replica Task: Attempting to enter view {:?} from view {:?}",
                    self.next_view, self.current_view
                );

                let replica_handle_event = HandleEvent(Arc::new(
                    move |event, state: ViewSyncReplicaTaskState<TYPES, I, A>| {
                        async move { state.handle_event(event).await }.boxed()
                    },
                ));

                let filter = FilterEvent::default();
                let builder = TaskBuilder::<ViewSyncReplicaTaskStateTypes<TYPES, I, A>>::new(name)
                    .register_event_stream(replica_state.event_stream.clone(), filter)
                    .await
                    .register_registry(&mut self.registry.clone())
                    .await
                    .register_state(replica_state)
                    .register_event_handler(replica_handle_event);

                let event_stream_id = builder.get_stream_id().unwrap();

                self.replica_task_map.insert(
                    certificate.get_view_number(),
                    ViewSyncTaskInfo { event_stream_id },
                );

                let _view_sync_replica_task = async_spawn(async move {
                    ViewSyncReplicaTaskStateTypes::build(builder).launch().await
                });
            }

            HotShotEvent::ViewSyncPreCommitVoteRecv(vote) => {
                if let Some(relay_task) = self.relay_task_map.get(&vote.get_view_number()) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if !self
                    .exchange
                    .is_leader(vote.get_view_number() + vote.get_data().relay)
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let new_accumulator = VoteAccumulator2 {
                    vote_outcomes: HashMap::new(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.exchange.total_nodes()],
                    phantom: PhantomData,
                };

                let mut relay_state = ViewSyncRelayTaskState::<
                    TYPES,
                    I,
                    ViewSyncPreCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                    ViewSyncPreCommitCertificate2<TYPES>,
                > {
                    event_stream: self.event_stream.clone(),
                    exchange: self.exchange.clone(),
                    accumulator: either::Left(new_accumulator),
                    id: self.id,
                };

                let result = relay_state.handle_event(event.clone()).await;

                if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                    // The protocol has finished
                    return;
                }

                relay_state = result.1;

                let name = format!("View Sync Relay Task for view {:?}", vote.get_view_number());

                let relay_handle_event = HandleEvent(Arc::new(
                    move |event,
                          state: ViewSyncRelayTaskState<
                        TYPES,
                        I,
                        ViewSyncPreCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncPreCommitCertificate2<TYPES>,
                    >| {
                        async move { state.handle_event(event).await }.boxed()
                    },
                ));

                let filter = FilterEvent::default();
                let builder = TaskBuilder::<
                    ViewSyncRelayTaskStateTypes<
                        TYPES,
                        I,
                        ViewSyncPreCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncPreCommitCertificate2<TYPES>,
                    >,
                >::new(name)
                .register_event_stream(relay_state.event_stream.clone(), filter)
                .await
                .register_registry(&mut self.registry.clone())
                .await
                .register_state(relay_state)
                .register_event_handler(relay_handle_event);

                let event_stream_id = builder.get_stream_id().unwrap();

                self.relay_task_map
                    .insert(vote.get_view_number(), ViewSyncTaskInfo { event_stream_id });
                let _view_sync_relay_task = async_spawn(async move {
                    ViewSyncRelayTaskStateTypes::build(builder).launch().await
                });
            }

            HotShotEvent::ViewSyncCommitVoteRecv(vote) => {
                if let Some(relay_task) = self.relay_task_map.get(&vote.get_view_number()) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if !self
                    .exchange
                    .is_leader(vote.get_view_number() + vote.get_data().relay)
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let new_accumulator = VoteAccumulator2 {
                    vote_outcomes: HashMap::new(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.exchange.total_nodes()],
                    phantom: PhantomData,
                };

                let mut relay_state = ViewSyncRelayTaskState::<
                    TYPES,
                    I,
                    ViewSyncCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                    ViewSyncCommitCertificate2<TYPES>,
                > {
                    event_stream: self.event_stream.clone(),
                    exchange: self.exchange.clone(),
                    accumulator: either::Left(new_accumulator),
                    id: self.id,
                };

                let result = relay_state.handle_event(event.clone()).await;

                if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                    // The protocol has finished
                    return;
                }

                relay_state = result.1;

                let name = format!("View Sync Relay Task for view {:?}", vote.get_view_number());

                let relay_handle_event = HandleEvent(Arc::new(
                    move |event,
                          state: ViewSyncRelayTaskState<
                        TYPES,
                        I,
                        ViewSyncCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncCommitCertificate2<TYPES>,
                    >| {
                        async move { state.handle_event(event).await }.boxed()
                    },
                ));

                let filter = FilterEvent::default();
                let builder = TaskBuilder::<
                    ViewSyncRelayTaskStateTypes<
                        TYPES,
                        I,
                        ViewSyncCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncCommitCertificate2<TYPES>,
                    >,
                >::new(name)
                .register_event_stream(relay_state.event_stream.clone(), filter)
                .await
                .register_registry(&mut self.registry.clone())
                .await
                .register_state(relay_state)
                .register_event_handler(relay_handle_event);

                let event_stream_id = builder.get_stream_id().unwrap();

                self.relay_task_map
                    .insert(vote.get_view_number(), ViewSyncTaskInfo { event_stream_id });
                let _view_sync_relay_task = async_spawn(async move {
                    ViewSyncRelayTaskStateTypes::build(builder).launch().await
                });
            }

            HotShotEvent::ViewSyncFinalizeVoteRecv(vote) => {
                if let Some(relay_task) = self.relay_task_map.get(&vote.get_view_number()) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if !self
                    .exchange
                    .is_leader(vote.get_view_number() + vote.get_data().relay)
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let new_accumulator = VoteAccumulator2 {
                    vote_outcomes: HashMap::new(),
                    sig_lists: Vec::new(),
                    signers: bitvec![0; self.exchange.total_nodes()],
                    phantom: PhantomData,
                };

                let mut relay_state = ViewSyncRelayTaskState::<
                    TYPES,
                    I,
                    ViewSyncFinalizeVote<TYPES, ViewSyncMembership<TYPES, I>>,
                    ViewSyncFinalizeCertificate2<TYPES>,
                > {
                    event_stream: self.event_stream.clone(),
                    exchange: self.exchange.clone(),
                    accumulator: either::Left(new_accumulator),
                    id: self.id,
                };

                let result = relay_state.handle_event(event.clone()).await;

                if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                    // The protocol has finished
                    return;
                }

                relay_state = result.1;

                let name = format!("View Sync Relay Task for view {:?}", vote.get_view_number());

                let relay_handle_event = HandleEvent(Arc::new(
                    move |event,
                          state: ViewSyncRelayTaskState<
                        TYPES,
                        I,
                        ViewSyncFinalizeVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncFinalizeCertificate2<TYPES>,
                    >| {
                        async move { state.handle_event(event).await }.boxed()
                    },
                ));

                let filter = FilterEvent::default();
                let builder = TaskBuilder::<
                    ViewSyncRelayTaskStateTypes<
                        TYPES,
                        I,
                        ViewSyncFinalizeVote<TYPES, ViewSyncMembership<TYPES, I>>,
                        ViewSyncFinalizeCertificate2<TYPES>,
                    >,
                >::new(name)
                .register_event_stream(relay_state.event_stream.clone(), filter)
                .await
                .register_registry(&mut self.registry.clone())
                .await
                .register_state(relay_state)
                .register_event_handler(relay_handle_event);

                let event_stream_id = builder.get_stream_id().unwrap();

                self.relay_task_map
                    .insert(vote.get_view_number(), ViewSyncTaskInfo { event_stream_id });
                let _view_sync_relay_task = async_spawn(async move {
                    ViewSyncRelayTaskStateTypes::build(builder).launch().await
                });
            }

            &HotShotEvent::ViewChange(new_view) => {
                let new_view = TYPES::Time::new(*new_view);
                if self.current_view < new_view {
                    debug!(
                        "Change from view {} to view {} in view sync task",
                        *self.current_view, *new_view
                    );

                    self.current_view = new_view;
                    self.next_view = self.current_view;
                    self.num_timeouts_tracked = 0;

                    // Garbage collect old tasks
                    // We could put this into a separate async task, but that would require making several fields on ViewSyncTaskState thread-safe and harm readability.  In the common case this will have zero tasks to clean up.
                    for i in *self.last_garbage_collected_view..*self.current_view {
                        if let Some((_key, replica_task_info)) =
                            self.replica_task_map.remove_entry(&TYPES::Time::new(i))
                        {
                            self.event_stream
                                .direct_message(
                                    replica_task_info.event_stream_id,
                                    HotShotEvent::Shutdown,
                                )
                                .await;
                        }
                        if let Some((_key, relay_task_info)) =
                            self.relay_task_map.remove_entry(&TYPES::Time::new(i))
                        {
                            self.event_stream
                                .direct_message(
                                    relay_task_info.event_stream_id,
                                    HotShotEvent::Shutdown,
                                )
                                .await;
                        }
                    }

                    self.last_garbage_collected_view = self.current_view - 1;
                }
            }
            &HotShotEvent::Timeout(view_number) => {
                // This is an old timeout and we can ignore it
                if view_number <= TYPES::Time::new(*self.current_view) {
                    return;
                }

                self.num_timeouts_tracked += 1;
                error!(
                    "Num timeouts tracked is {}. View {} timed out",
                    self.num_timeouts_tracked, *view_number
                );

                if self.num_timeouts_tracked > 3 {
                    error!("Too many timeouts!  This shouldn't happen");
                }

                if self.num_timeouts_tracked > 2 {
                    // Start polling for view sync certificates
                    self.exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForViewSyncCertificate(
                            *view_number + 1,
                        ))
                        .await;

                    self.exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForViewSyncVotes(
                            *view_number + 1,
                        ))
                        .await;
                    // Spawn replica task
                    let next_view = *view_number + 1;
                    // Subscribe to the view after we are leader since we know we won't propose in the next view if we are leader.
                    let subscribe_view = if self.exchange.is_leader(TYPES::Time::new(next_view)) {
                        next_view + 1
                    } else {
                        next_view
                    };
                    // Subscribe to the next view just in case there is progress being made
                    self.exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(
                            subscribe_view,
                        ))
                        .await;

                    self.exchange
                        .network()
                        .inject_consensus_info(ConsensusIntentEvent::PollForDAC(subscribe_view))
                        .await;

                    let mut replica_state = ViewSyncReplicaTaskState {
                        current_view: self.current_view,
                        next_view: TYPES::Time::new(next_view),
                        relay: 0,
                        finalized: false,
                        sent_view_change_event: false,
                        phase: ViewSyncPhase::None,
                        exchange: self.exchange.clone(),
                        api: self.api.clone(),
                        event_stream: self.event_stream.clone(),
                        view_sync_timeout: self.view_sync_timeout,
                        id: self.id,
                    };

                    let result = replica_state
                        .handle_event(HotShotEvent::ViewSyncTrigger(view_number + 1))
                        .await;

                    if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                        // The protocol has finished
                        return;
                    }

                    replica_state = result.1;

                    let name = format!(
                        "View Sync Replica Task: Attempting to enter view {:?} from view {:?}",
                        *view_number + 1,
                        *view_number
                    );

                    let replica_handle_event = HandleEvent(Arc::new(
                        move |event, state: ViewSyncReplicaTaskState<TYPES, I, A>| {
                            async move { state.handle_event(event).await }.boxed()
                        },
                    ));

                    let filter = FilterEvent::default();
                    let builder =
                        TaskBuilder::<ViewSyncReplicaTaskStateTypes<TYPES, I, A>>::new(name)
                            .register_event_stream(replica_state.event_stream.clone(), filter)
                            .await
                            .register_registry(&mut self.registry.clone())
                            .await
                            .register_state(replica_state)
                            .register_event_handler(replica_handle_event);

                    let event_stream_id = builder.get_stream_id().unwrap();

                    self.replica_task_map.insert(
                        TYPES::Time::new(*view_number + 1),
                        ViewSyncTaskInfo { event_stream_id },
                    );

                    let _view_sync_replica_task = async_spawn(async move {
                        ViewSyncReplicaTaskStateTypes::build(builder).launch().await
                    });
                } else {
                    // If this is the first timeout we've seen advance to the next view
                    self.current_view = view_number;
                    self.event_stream
                        .publish(HotShotEvent::ViewChange(TYPES::Time::new(
                            *self.current_view,
                        )))
                        .await;
                }
            }

            _ => {}
        }
    }

    /// Filter view sync related events.
    pub fn filter(event: &HotShotEvent<TYPES, I>) -> bool {
        matches!(
            event,
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(_)
                | HotShotEvent::ViewSyncCommitCertificate2Recv(_)
                | HotShotEvent::ViewSyncFinalizeCertificate2Recv(_)
                | HotShotEvent::ViewSyncPreCommitVoteRecv(_)
                | HotShotEvent::ViewSyncCommitVoteRecv(_)
                | HotShotEvent::ViewSyncFinalizeVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::Timeout(_)
                | HotShotEvent::ViewSyncTimeout(_, _, _)
                | HotShotEvent::ViewChange(_)
        )
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: ConsensusApi<TYPES, Leaf<TYPES>, I> + 'static,
    > ViewSyncReplicaTaskState<TYPES, I, A>
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Replica Task", level = "error")]
    /// Handle incoming events for the view sync replica task
    pub async fn handle_event(
        mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> (
        std::option::Option<HotShotTaskCompleted>,
        ViewSyncReplicaTaskState<TYPES, I, A>,
    ) {
        match event {
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(certificate) => {
                let last_seen_certificate = ViewSyncPhase::PreCommit;

                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    error!("We're already in a higher round");

                    return (None, self);
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.exchange.membership()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return (None, self);
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return (Some(HotShotTaskCompleted::ShutDown), self);
                }

                // Ignore if the certificate is for an already seen phase
                if last_seen_certificate <= self.phase {
                    return (None, self);
                }

                self.phase = last_seen_certificate;

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                let vote =
                    ViewSyncCommitVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                        ViewSyncCommitData {
                            relay: certificate.get_data().relay,
                            round: self.next_view,
                        },
                        self.next_view,
                        self.exchange.public_key(),
                        self.exchange.private_key(),
                    );
                let message = GeneralConsensusMessage::<TYPES, I>::ViewSyncCommitVote(vote);

                if let GeneralConsensusMessage::ViewSyncCommitVote(vote) = message {
                    self.event_stream
                        .publish(HotShotEvent::ViewSyncCommitVoteSend(vote))
                        .await;
                }

                async_spawn({
                    let stream = self.event_stream.clone();
                    let phase = self.phase.clone();
                    async move {
                        async_sleep(self.view_sync_timeout).await;
                        error!("Vote sending timed out in ViewSyncCertificateRecv");
                        stream
                            .publish(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*self.next_view),
                                self.relay,
                                phase,
                            ))
                            .await;
                    }
                });
            }

            HotShotEvent::ViewSyncCommitCertificate2Recv(certificate) => {
                let last_seen_certificate = ViewSyncPhase::Commit;

                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    error!("We're already in a higher round");

                    return (None, self);
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.exchange.membership()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return (None, self);
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return (Some(HotShotTaskCompleted::ShutDown), self);
                }

                // Ignore if the certificate is for an already seen phase
                if last_seen_certificate <= self.phase {
                    return (None, self);
                }

                self.phase = last_seen_certificate;

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                let vote =
                    ViewSyncFinalizeVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                        ViewSyncFinalizeData {
                            relay: certificate.get_data().relay,
                            round: self.next_view,
                        },
                        self.next_view,
                        self.exchange.public_key(),
                        self.exchange.private_key(),
                    );
                let message = GeneralConsensusMessage::<TYPES, I>::ViewSyncFinalizeVote(vote);

                if let GeneralConsensusMessage::ViewSyncFinalizeVote(vote) = message {
                    self.event_stream
                        .publish(HotShotEvent::ViewSyncFinalizeVoteSend(vote))
                        .await;
                }
                self.event_stream
                    .publish(HotShotEvent::ViewChange(self.next_view))
                    .await;

                async_spawn({
                    let stream = self.event_stream.clone();
                    let phase = self.phase.clone();
                    async move {
                        async_sleep(self.view_sync_timeout).await;
                        error!("Vote sending timed out in ViewSyncCertificateRecv");
                        stream
                            .publish(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*self.next_view),
                                self.relay,
                                phase,
                            ))
                            .await;
                    }
                });
            }

            HotShotEvent::ViewSyncFinalizeCertificate2Recv(certificate) => {
                let last_seen_certificate = ViewSyncPhase::Finalize;

                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    error!("We're already in a higher round");

                    return (None, self);
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.exchange.membership()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return (None, self);
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return (Some(HotShotTaskCompleted::ShutDown), self);
                }

                // Ignore if the certificate is for an already seen phase
                if last_seen_certificate <= self.phase {
                    return (None, self);
                }

                self.phase = last_seen_certificate;

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                self.event_stream
                    .publish(HotShotEvent::ViewChange(self.next_view))
                    .await;
                return (Some(HotShotTaskCompleted::ShutDown), self);
            }

            HotShotEvent::ViewSyncTrigger(view_number) => {
                if self.next_view != TYPES::Time::new(*view_number) {
                    error!("Unexpected view number to triger view sync");
                    return (None, self);
                }

                let vote =
                ViewSyncPreCommitVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                    ViewSyncPreCommitData { relay: 0, round: view_number},
                    view_number,
                    self.exchange.public_key(),
                    self.exchange.private_key(),
                );
                let message = GeneralConsensusMessage::<TYPES, I>::ViewSyncPreCommitVote(vote);

                if let GeneralConsensusMessage::ViewSyncPreCommitVote(vote) = message {
                    self.event_stream
                        .publish(HotShotEvent::ViewSyncPreCommitVoteSend(vote))
                        .await;
                }

                async_spawn({
                    let stream = self.event_stream.clone();
                    async move {
                        async_sleep(self.view_sync_timeout).await;
                        error!("Vote sending timed out in ViewSyncTrigger");
                        stream
                            .publish(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*self.next_view),
                                self.relay,
                                ViewSyncPhase::None,
                            ))
                            .await;
                    }
                });

                return (None, self);
            }

            HotShotEvent::ViewSyncTimeout(round, relay, last_seen_certificate) => {
                // Shouldn't ever receive a timeout for a relay higher than ours
                if TYPES::Time::new(*round) == self.next_view
                    && relay == self.relay
                    && last_seen_certificate == self.phase
                {
                    self.relay += 1;
                    match self.phase {
                        ViewSyncPhase::None => {
                            let vote =
                            ViewSyncPreCommitVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                                ViewSyncPreCommitData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                self.exchange.public_key(),
                                self.exchange.private_key(),
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES, I>::ViewSyncPreCommitVote(vote);

                            if let GeneralConsensusMessage::ViewSyncPreCommitVote(vote) = message {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncPreCommitVoteSend(vote))
                                    .await;
                            }
                        }
                        ViewSyncPhase::PreCommit => {
                            let vote =
                            ViewSyncCommitVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                                ViewSyncCommitData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                self.exchange.public_key(),
                                self.exchange.private_key(),
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES, I>::ViewSyncCommitVote(vote);

                            if let GeneralConsensusMessage::ViewSyncCommitVote(vote) = message {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncCommitVoteSend(vote))
                                    .await;
                            }
                        }
                        ViewSyncPhase::Commit => {
                            let vote =
                            ViewSyncFinalizeVote::<TYPES, ViewSyncMembership<TYPES, I>>::create_signed_vote(
                                ViewSyncFinalizeData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                self.exchange.public_key(),
                                self.exchange.private_key(),
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES, I>::ViewSyncFinalizeVote(vote);

                            if let GeneralConsensusMessage::ViewSyncFinalizeVote(vote) = message {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncFinalizeVoteSend(vote))
                                    .await;
                            }
                        }
                        ViewSyncPhase::Finalize => {
                            // This should never occur
                            unimplemented!()
                        }
                    }

                    async_spawn({
                        let stream = self.event_stream.clone();
                        async move {
                            async_sleep(self.view_sync_timeout).await;
                            error!("Vote sending timed out in ViewSyncTimeout");
                            stream
                                .publish(HotShotEvent::ViewSyncTimeout(
                                    TYPES::Time::new(*self.next_view),
                                    self.relay,
                                    last_seen_certificate,
                                ))
                                .await;
                        }
                    });

                    return (None, self);
                }
            }
            _ => return (None, self),
        }
        (None, self)
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
    >
    ViewSyncRelayTaskState<
        TYPES,
        I,
        ViewSyncPreCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
        ViewSyncPreCommitCertificate2<TYPES>,
    >
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    /// Handles incoming events for the view sync relay task
    #[instrument(skip_all, fields(id = self.id), name = "View Sync Relay Task", level = "error")]
    #[allow(clippy::type_complexity)]
    pub async fn handle_event(
        mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> (
        std::option::Option<HotShotTaskCompleted>,
        ViewSyncRelayTaskState<
            TYPES,
            I,
            ViewSyncPreCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
            ViewSyncPreCommitCertificate2<TYPES>,
        >,
    ) {
        match event {
            HotShotEvent::ViewSyncPreCommitVoteRecv(vote) => {
                // Ignore this vote if we are not the correct relay
                // TODO ED Replace exchange with membership
                if !self
                    .exchange
                    .is_leader(vote.get_data().round + vote.get_data().relay)
                {
                    info!("We are not the correct relay for this vote; vote was intended for relay {}", vote.get_data().relay);
                    return (None, self);
                }

                debug!(
                    "Accumulating ViewSyncPreCommitVote for round {} and relay {}",
                    *vote.get_data().round,
                    vote.get_data().relay
                );

                match self.accumulator {
                    Right(_) => return (Some(HotShotTaskCompleted::ShutDown), self),
                    Left(accumulator) => {
                        match accumulator.accumulate(&vote, self.exchange.membership()) {
                            Left(new_accumulator) => {
                                self.accumulator = Either::Left(new_accumulator);
                            }
                            Right(certificate) => {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncPreCommitCertificate2Send(
                                        certificate.clone(),
                                        self.exchange.public_key().clone(),
                                    ))
                                    .await;
                                self.accumulator = Right(certificate);

                                return (Some(HotShotTaskCompleted::ShutDown), self);
                            }
                        }
                    }
                };
                (None, self)
            }

            _ => (None, self),
        }
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
    >
    ViewSyncRelayTaskState<
        TYPES,
        I,
        ViewSyncCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
        ViewSyncCommitCertificate2<TYPES>,
    >
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    /// Handles incoming events for the view sync relay task
    #[instrument(skip_all, fields(id = self.id), name = "View Sync Relay Task", level = "error")]
    #[allow(clippy::type_complexity)]
    pub async fn handle_event(
        mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> (
        std::option::Option<HotShotTaskCompleted>,
        ViewSyncRelayTaskState<
            TYPES,
            I,
            ViewSyncCommitVote<TYPES, ViewSyncMembership<TYPES, I>>,
            ViewSyncCommitCertificate2<TYPES>,
        >,
    ) {
        match event {
            HotShotEvent::ViewSyncCommitVoteRecv(vote) => {
                // Ignore this vote if we are not the correct relay
                if !self
                    .exchange
                    .is_leader(vote.get_data().round + vote.get_data().relay)
                {
                    info!("We are not the correct relay for this vote; vote was intended for relay {}", vote.get_data().relay);
                    return (None, self);
                }

                debug!(
                    "Accumulating ViewSyncCommitVote for round {} and relay {}",
                    *vote.get_data().round,
                    vote.get_data().relay
                );

                match self.accumulator {
                    Right(_) => return (Some(HotShotTaskCompleted::ShutDown), self),
                    Left(accumulator) => {
                        match accumulator.accumulate(&vote, self.exchange.membership()) {
                            Left(new_accumulator) => {
                                self.accumulator = Either::Left(new_accumulator);
                            }
                            Right(certificate) => {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncCommitCertificate2Send(
                                        certificate.clone(),
                                        self.exchange.public_key().clone(),
                                    ))
                                    .await;
                                self.accumulator = Right(certificate);

                                return (Some(HotShotTaskCompleted::ShutDown), self);
                            }
                        }
                    }
                };
                (None, self)
            }

            _ => (None, self),
        }
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = Leaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
    >
    ViewSyncRelayTaskState<
        TYPES,
        I,
        ViewSyncFinalizeVote<TYPES, ViewSyncMembership<TYPES, I>>,
        ViewSyncFinalizeCertificate2<TYPES>,
    >
where
    ViewSyncEx<TYPES, I>: ViewSyncExchangeType<TYPES, Message<TYPES, I>>,
{
    /// Handles incoming events for the view sync relay task
    #[instrument(skip_all, fields(id = self.id), name = "View Sync Relay Task", level = "error")]
    #[allow(clippy::type_complexity)]
    pub async fn handle_event(
        mut self,
        event: HotShotEvent<TYPES, I>,
    ) -> (
        std::option::Option<HotShotTaskCompleted>,
        ViewSyncRelayTaskState<
            TYPES,
            I,
            ViewSyncFinalizeVote<TYPES, ViewSyncMembership<TYPES, I>>,
            ViewSyncFinalizeCertificate2<TYPES>,
        >,
    ) {
        match event {
            HotShotEvent::ViewSyncFinalizeVoteRecv(vote) => {
                // Ignore this vote if we are not the correct relay
                // TODO ED Replace exchange with membership
                if !self
                    .exchange
                    .is_leader(vote.get_data().round + vote.get_data().relay)
                {
                    info!("We are not the correct relay for this vote; vote was intended for relay {}", vote.get_data().relay);
                    return (None, self);
                }

                debug!(
                    "Accumulating ViewSyncFinalizetVote for round {} and relay {}",
                    *vote.get_data().round,
                    vote.get_data().relay
                );

                match self.accumulator {
                    Right(_) => return (Some(HotShotTaskCompleted::ShutDown), self),
                    Left(accumulator) => {
                        match accumulator.accumulate(&vote, self.exchange.membership()) {
                            Left(new_accumulator) => {
                                self.accumulator = Either::Left(new_accumulator);
                            }
                            Right(certificate) => {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncFinalizeCertificate2Send(
                                        certificate.clone(),
                                        self.exchange.public_key().clone(),
                                    ))
                                    .await;
                                self.accumulator = Right(certificate);

                                return (Some(HotShotTaskCompleted::ShutDown), self);
                            }
                        }
                    }
                };
                (None, self)
            }

            _ => (None, self),
        }
    }
}
