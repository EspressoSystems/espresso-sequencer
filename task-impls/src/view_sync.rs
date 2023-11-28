#![allow(clippy::module_name_repetitions)]
use crate::{
    events::HotShotEvent,
    vote::{spawn_vote_accumulator, AccumulatorInfo},
};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use either::Either;
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::{simple_vote::ViewSyncFinalizeData, traits::signature_key::SignatureKey};
use hotshot_types::{
    simple_vote::{
        ViewSyncCommitData, ViewSyncCommitVote, ViewSyncFinalizeVote, ViewSyncPreCommitData,
        ViewSyncPreCommitVote,
    },
    traits::network::ConsensusIntentEvent,
    vote::{Certificate, HasViewNumber, Vote, VoteAccumulator},
};

use hotshot_task::global_registry::GlobalRegistry;
use hotshot_types::{
    message::GeneralConsensusMessage,
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        network::CommunicationChannel,
        node_implementation::{NodeImplementation, NodeType},
        state::ConsensusTime,
    },
};
use snafu::Snafu;
use std::{collections::HashMap, sync::Arc, time::Duration};
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
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
> {
    /// Registry to register sub tasks
    pub registry: GlobalRegistry,
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,
    /// View HotShot is currently in
    pub current_view: TYPES::Time,
    /// View HotShot wishes to be in
    pub next_view: TYPES::Time,
    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for teh quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
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
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
    > TS for ViewSyncTaskState<TYPES, I, A>
{
}

/// Types for the main view sync task
pub type ViewSyncTaskStateTypes<TYPES, I, A> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    ViewSyncTaskState<TYPES, I, A>,
>;

/// State of a view sync replica task
pub struct ViewSyncReplicaTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
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

    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for teh quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// HotShot consensus API
    pub api: A,
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TS
    for ViewSyncReplicaTaskState<TYPES, I, A>
{
}

/// Types for view sync replica state
pub type ViewSyncReplicaTaskStateTypes<TYPES, I, A> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    ViewSyncReplicaTaskState<TYPES, I, A>,
>;

/// State of a view sync relay task
pub struct ViewSyncRelayTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    VOTE: Vote<TYPES>,
    CERTIFICATE: Certificate<TYPES, Voteable = VOTE::Commitment>,
> {
    /// Event stream to publish events to
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,
    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for teh quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Vote accumulator
    #[allow(clippy::type_complexity)]
    pub accumulator: Either<VoteAccumulator<TYPES, VOTE, CERTIFICATE>, CERTIFICATE>,
    /// Our node id; for logging
    pub id: u64,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        VOTE: Vote<TYPES> + std::marker::Send + std::marker::Sync + 'static,
        CERTIFICATE: Certificate<TYPES, Voteable = VOTE::Commitment>
            + std::marker::Send
            + std::marker::Sync
            + 'static,
    > TS for ViewSyncRelayTaskState<TYPES, I, VOTE, CERTIFICATE>
{
}

/// Types used by the view sync relay task
pub type ViewSyncRelayTaskStateTypes<TYPES, I, VOTE, CERTIFICATE> = HSTWithEvent<
    ViewSyncTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    ViewSyncRelayTaskState<TYPES, I, VOTE, CERTIFICATE>,
>;

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
    > ViewSyncTaskState<TYPES, I, A>
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Main Task", level = "error")]
    #[allow(clippy::type_complexity)]
    /// Handles incoming events for the main view sync task
    pub async fn handle_event(&mut self, event: HotShotEvent<TYPES>) {
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
                        membership: self.membership.clone(),
                        network: self.network.clone(),
                        public_key: self.public_key.clone(),
                        private_key: self.private_key.clone(),
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

            HotShotEvent::ViewSyncPreCommitVoteRecv(ref vote) => {
                let vote_view = vote.get_view_number();
                if let Some(relay_task) = self.relay_task_map.get(&vote_view) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if self
                    .membership
                    .get_leader(vote_view + vote.get_data().relay)
                    != self.public_key
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let name = format!("View Sync Relay Task for view {vote_view:?}");
                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: self.membership.clone(),
                    view: vote_view,
                    event_stream: self.event_stream.clone(),
                    id: self.id,
                    registry: self.registry.clone(),
                };
                let vote_collector =
                    spawn_vote_accumulator(&info, vote.clone(), event, name.to_string()).await;
                if let Some((_, _, event_stream_id)) = vote_collector {
                    self.relay_task_map
                        .insert(vote_view, ViewSyncTaskInfo { event_stream_id });
                }
            }

            HotShotEvent::ViewSyncCommitVoteRecv(ref vote) => {
                let vote_view = vote.get_view_number();
                if let Some(relay_task) = self.relay_task_map.get(&vote_view) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if self
                    .membership
                    .get_leader(vote_view + vote.get_data().relay)
                    != self.public_key
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let name = format!("View Sync Relay Task for view {vote_view:?}");
                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: self.membership.clone(),
                    view: vote_view,
                    event_stream: self.event_stream.clone(),
                    id: self.id,
                    registry: self.registry.clone(),
                };
                let vote_collector =
                    spawn_vote_accumulator(&info, vote.clone(), event, name.to_string()).await;
                if let Some((_, _, event_stream_id)) = vote_collector {
                    self.relay_task_map
                        .insert(vote_view, ViewSyncTaskInfo { event_stream_id });
                }
            }

            HotShotEvent::ViewSyncFinalizeVoteRecv(ref vote) => {
                let vote_view = vote.get_view_number();
                if let Some(relay_task) = self.relay_task_map.get(&vote_view) {
                    // Forward event then return
                    self.event_stream
                        .direct_message(relay_task.event_stream_id, event)
                        .await;
                    return;
                }

                // We do not have a relay task already running, so start one
                if self
                    .membership
                    .get_leader(vote_view + vote.get_data().relay)
                    != self.public_key
                {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let name = format!("View Sync Relay Task for view {vote_view:?}");
                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: self.membership.clone(),
                    view: vote.get_view_number(),
                    event_stream: self.event_stream.clone(),
                    id: self.id,
                    registry: self.registry.clone(),
                };
                let vote_collector =
                    spawn_vote_accumulator(&info, vote.clone(), event, name.to_string()).await;
                if let Some((_, _, event_stream_id)) = vote_collector {
                    self.relay_task_map
                        .insert(vote_view, ViewSyncTaskInfo { event_stream_id });
                }
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
                    "Num timeouts tracked since last view change is {}. View {} timed out",
                    self.num_timeouts_tracked, *view_number
                );

                if self.num_timeouts_tracked > 3 {
                    error!("Too many consecutive timeouts!  This shouldn't happen");
                }

                if self.num_timeouts_tracked > 2 {
                    // Start polling for view sync certificates
                    self.network
                        .inject_consensus_info(ConsensusIntentEvent::PollForViewSyncCertificate(
                            *view_number + 1,
                        ))
                        .await;

                    self.network
                        .inject_consensus_info(ConsensusIntentEvent::PollForViewSyncVotes(
                            *view_number + 1,
                        ))
                        .await;
                    // Spawn replica task
                    let next_view = *view_number + 1;
                    // Subscribe to the view after we are leader since we know we won't propose in the next view if we are leader.
                    let subscribe_view = if self.membership.get_leader(TYPES::Time::new(next_view))
                        == self.public_key
                    {
                        next_view
                    } else {
                        next_view + 1
                    };
                    // Subscribe to the next view just in case there is progress being made
                    self.network
                        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(
                            subscribe_view,
                        ))
                        .await;

                    self.network
                        .inject_consensus_info(ConsensusIntentEvent::PollForDAC(subscribe_view))
                        .await;

                    let mut replica_state = ViewSyncReplicaTaskState {
                        current_view: self.current_view,
                        next_view: TYPES::Time::new(next_view),
                        relay: 0,
                        finalized: false,
                        sent_view_change_event: false,
                        phase: ViewSyncPhase::None,
                        membership: self.membership.clone(),
                        network: self.network.clone(),
                        public_key: self.public_key.clone(),
                        private_key: self.private_key.clone(),
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
    pub fn filter(event: &HotShotEvent<TYPES>) -> bool {
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    ViewSyncReplicaTaskState<TYPES, I, A>
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Replica Task", level = "error")]
    /// Handle incoming events for the view sync replica task
    pub async fn handle_event(
        mut self,
        event: HotShotEvent<TYPES>,
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
                if !certificate.is_valid_cert(self.membership.as_ref()) {
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

                let vote = ViewSyncCommitVote::<TYPES>::create_signed_vote(
                    ViewSyncCommitData {
                        relay: certificate.get_data().relay,
                        round: self.next_view,
                    },
                    self.next_view,
                    &self.public_key,
                    &self.private_key,
                );
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncCommitVote(vote);

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
                if !certificate.is_valid_cert(self.membership.as_ref()) {
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

                let vote = ViewSyncFinalizeVote::<TYPES>::create_signed_vote(
                    ViewSyncFinalizeData {
                        relay: certificate.get_data().relay,
                        round: self.next_view,
                    },
                    self.next_view,
                    &self.public_key,
                    &self.private_key,
                );
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncFinalizeVote(vote);

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
                if !certificate.is_valid_cert(self.membership.as_ref()) {
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

                let vote = ViewSyncPreCommitVote::<TYPES>::create_signed_vote(
                    ViewSyncPreCommitData {
                        relay: 0,
                        round: view_number,
                    },
                    view_number,
                    &self.public_key,
                    &self.private_key,
                );
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncPreCommitVote(vote);

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
                            let vote = ViewSyncPreCommitVote::<TYPES>::create_signed_vote(
                                ViewSyncPreCommitData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                &self.public_key,
                                &self.private_key,
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES>::ViewSyncPreCommitVote(vote);

                            if let GeneralConsensusMessage::ViewSyncPreCommitVote(vote) = message {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncPreCommitVoteSend(vote))
                                    .await;
                            }
                        }
                        ViewSyncPhase::PreCommit => {
                            let vote = ViewSyncCommitVote::<TYPES>::create_signed_vote(
                                ViewSyncCommitData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                &self.public_key,
                                &self.private_key,
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES>::ViewSyncCommitVote(vote);

                            if let GeneralConsensusMessage::ViewSyncCommitVote(vote) = message {
                                self.event_stream
                                    .publish(HotShotEvent::ViewSyncCommitVoteSend(vote))
                                    .await;
                            }
                        }
                        ViewSyncPhase::Commit => {
                            let vote = ViewSyncFinalizeVote::<TYPES>::create_signed_vote(
                                ViewSyncFinalizeData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                &self.public_key,
                                &self.private_key,
                            );
                            let message =
                                GeneralConsensusMessage::<TYPES>::ViewSyncFinalizeVote(vote);

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
