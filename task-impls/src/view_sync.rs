#![allow(clippy::module_name_repetitions)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    sync::Arc,
    time::Duration,
};

use async_broadcast::Sender;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    message::GeneralConsensusMessage,
    simple_certificate::{
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        ViewSyncCommitData, ViewSyncCommitVote, ViewSyncFinalizeData, ViewSyncFinalizeVote,
        ViewSyncPreCommitData, ViewSyncPreCommitVote,
    },
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vote::{Certificate, HasViewNumber, Vote},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, warn};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::{broadcast_event, cancel_task},
    vote_collection::{
        create_vote_accumulator, AccumulatorInfo, HandleVoteEvent, VoteCollectionTaskState,
    },
};
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

/// Type alias for a map from View Number to Relay to Vote Task
type RelayMap<TYPES, VOTE, CERT> =
    HashMap<<TYPES as NodeType>::Time, BTreeMap<u64, VoteCollectionTaskState<TYPES, VOTE, CERT>>>;

/// Main view sync task state
pub struct ViewSyncTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
> {
    /// View HotShot is currently in
    pub current_view: TYPES::Time,
    /// View HotShot wishes to be in
    pub next_view: TYPES::Time,
    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for the quorum
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
    pub replica_task_map: RwLock<HashMap<TYPES::Time, ViewSyncReplicaTaskState<TYPES, I, A>>>,

    /// Map of pre-commit vote accumulates for the relay
    pub pre_commit_relay_map:
        RwLock<RelayMap<TYPES, ViewSyncPreCommitVote<TYPES>, ViewSyncPreCommitCertificate2<TYPES>>>,
    /// Map of commit vote accumulates for the relay
    pub commit_relay_map:
        RwLock<RelayMap<TYPES, ViewSyncCommitVote<TYPES>, ViewSyncCommitCertificate2<TYPES>>>,
    /// Map of finalize vote accumulates for the relay
    pub finalize_relay_map:
        RwLock<RelayMap<TYPES, ViewSyncFinalizeVote<TYPES>, ViewSyncFinalizeCertificate2<TYPES>>>,

    /// Timeout duration for view sync rounds
    pub view_sync_timeout: Duration,

    /// Last view we garbage collected old tasks
    pub last_garbage_collected_view: TYPES::Time,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
    > TaskState for ViewSyncTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = ();

    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()> {
        let sender = task.clone_sender();
        task.state_mut().handle(event, sender).await;
        None
    }

    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
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

    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}

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
    /// The relay index we are currently on
    pub relay: u64,
    /// Whether we have seen a finalized certificate
    pub finalized: bool,
    /// Whether we have already sent a view change event for `next_view`
    pub sent_view_change_event: bool,
    /// Timeout task handle, when it expires we try the next relay
    pub timeout_task: Option<JoinHandle<()>>,
    /// Our node id; for logging
    pub id: u64,

    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for the quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// HotShot consensus API
    pub api: A,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TaskState
    for ViewSyncReplicaTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = ();

    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()> {
        let sender = task.clone_sender();
        task.state_mut().handle(event, sender).await;
        None
    }
    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
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

    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static + std::clone::Clone,
    > ViewSyncTaskState<TYPES, I, A>
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Main Task", level = "error")]
    #[allow(clippy::type_complexity)]
    /// Handles incoming events for the main view sync task
    pub async fn send_to_or_create_replica(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        view: TYPES::Time,
        sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        // This certificate is old, we can throw it away
        // If next view = cert round, then that means we should already have a task running for it
        if self.current_view > view {
            debug!("Already in a higher view than the view sync message");
            return;
        }

        let mut task_map = self.replica_task_map.write().await;

        if let Some(replica_task) = task_map.get_mut(&view) {
            // Forward event then return
            debug!("Forwarding message");
            let result = replica_task
                .handle(Arc::clone(&event), sender.clone())
                .await;

            if result == Some(HotShotTaskCompleted) {
                // The protocol has finished
                task_map.remove(&view);
                return;
            }

            return;
        }

        // We do not have a replica task already running, so start one
        let mut replica_state: ViewSyncReplicaTaskState<TYPES, I, A> = ViewSyncReplicaTaskState {
            current_view: view,
            next_view: view,
            relay: 0,
            finalized: false,
            sent_view_change_event: false,
            timeout_task: None,
            membership: Arc::clone(&self.membership),
            network: Arc::clone(&self.network),
            public_key: self.public_key.clone(),
            private_key: self.private_key.clone(),
            api: self.api.clone(),
            view_sync_timeout: self.view_sync_timeout,
            id: self.id,
        };

        let result = replica_state
            .handle(Arc::clone(&event), sender.clone())
            .await;

        if result == Some(HotShotTaskCompleted) {
            // The protocol has finished
            return;
        }

        task_map.insert(view, replica_state);
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Main Task", level = "error")]
    #[allow(clippy::type_complexity)]
    /// Handles incoming events for the main view sync task
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(certificate) => {
                debug!("Received view sync cert for phase {:?}", certificate);
                let view = certificate.get_view_number();
                self.send_to_or_create_replica(event, view, &event_stream)
                    .await;
            }
            HotShotEvent::ViewSyncCommitCertificate2Recv(certificate) => {
                debug!("Received view sync cert for phase {:?}", certificate);
                let view = certificate.get_view_number();
                self.send_to_or_create_replica(event, view, &event_stream)
                    .await;
            }
            HotShotEvent::ViewSyncFinalizeCertificate2Recv(certificate) => {
                debug!("Received view sync cert for phase {:?}", certificate);
                let view = certificate.get_view_number();
                self.send_to_or_create_replica(event, view, &event_stream)
                    .await;
            }
            HotShotEvent::ViewSyncTimeout(view, _, _) => {
                debug!("view sync timeout in main task {:?}", view);
                let view = *view;
                self.send_to_or_create_replica(event, view, &event_stream)
                    .await;
            }

            HotShotEvent::ViewSyncPreCommitVoteRecv(ref vote) => {
                let mut map = self.pre_commit_relay_map.write().await;
                let vote_view = vote.get_view_number();
                let relay = vote.get_data().relay;
                let relay_map = map.entry(vote_view).or_insert(BTreeMap::new());
                if let Some(relay_task) = relay_map.get_mut(&relay) {
                    debug!("Forwarding message");
                    let result = relay_task
                        .handle_event(Arc::clone(&event), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        // The protocol has finished
                        map.remove(&vote_view);
                    }
                    return;
                }

                // We do not have a relay task already running, so start one
                if self.membership.get_leader(vote_view + relay) != self.public_key {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: Arc::clone(&self.membership),
                    view: vote_view,
                    id: self.id,
                };
                let vote_collector =
                    create_vote_accumulator(&info, vote.clone(), event, &event_stream).await;
                if let Some(vote_task) = vote_collector {
                    relay_map.insert(relay, vote_task);
                }
            }

            HotShotEvent::ViewSyncCommitVoteRecv(ref vote) => {
                let mut map = self.commit_relay_map.write().await;
                let vote_view = vote.get_view_number();
                let relay = vote.get_data().relay;
                let relay_map = map.entry(vote_view).or_insert(BTreeMap::new());
                if let Some(relay_task) = relay_map.get_mut(&relay) {
                    debug!("Forwarding message");
                    let result = relay_task
                        .handle_event(Arc::clone(&event), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        // The protocol has finished
                        map.remove(&vote_view);
                    }
                    return;
                }

                // We do not have a relay task already running, so start one
                if self.membership.get_leader(vote_view + relay) != self.public_key {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: Arc::clone(&self.membership),
                    view: vote_view,
                    id: self.id,
                };
                let vote_collector =
                    create_vote_accumulator(&info, vote.clone(), event, &event_stream).await;
                if let Some(vote_task) = vote_collector {
                    relay_map.insert(relay, vote_task);
                }
            }

            HotShotEvent::ViewSyncFinalizeVoteRecv(vote) => {
                let mut map = self.finalize_relay_map.write().await;
                let vote_view = vote.get_view_number();
                let relay = vote.get_data().relay;
                let relay_map = map.entry(vote_view).or_insert(BTreeMap::new());
                if let Some(relay_task) = relay_map.get_mut(&relay) {
                    debug!("Forwarding message");
                    let result = relay_task
                        .handle_event(Arc::clone(&event), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        // The protocol has finished
                        map.remove(&vote_view);
                    }
                    return;
                }

                // We do not have a relay task already running, so start one
                if self.membership.get_leader(vote_view + relay) != self.public_key {
                    // TODO ED This will occur because everyone is pulling down votes for now. Will be fixed in `https://github.com/EspressoSystems/HotShot/issues/1471`
                    debug!("View sync vote sent to wrong leader");
                    return;
                }

                let info = AccumulatorInfo {
                    public_key: self.public_key.clone(),
                    membership: Arc::clone(&self.membership),
                    view: vote_view,
                    id: self.id,
                };
                let vote_collector =
                    create_vote_accumulator(&info, vote.clone(), event, &event_stream).await;
                if let Some(vote_task) = vote_collector {
                    relay_map.insert(relay, vote_task);
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
                    // cancel poll for votes
                    // run GC
                    for i in *self.last_garbage_collected_view..*self.current_view {
                        self.replica_task_map
                            .write()
                            .await
                            .remove_entry(&TYPES::Time::new(i));
                        self.pre_commit_relay_map
                            .write()
                            .await
                            .remove_entry(&TYPES::Time::new(i));
                        self.commit_relay_map
                            .write()
                            .await
                            .remove_entry(&TYPES::Time::new(i));
                        self.finalize_relay_map
                            .write()
                            .await
                            .remove_entry(&TYPES::Time::new(i));
                    }

                    self.last_garbage_collected_view = self.current_view - 1;
                }
            }
            &HotShotEvent::Timeout(view_number) => {
                // This is an old timeout and we can ignore it
                if view_number <= TYPES::Time::new(*self.current_view) {
                    return;
                }

                // cancel poll for votes
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view_number))
                    .await;

                self.num_timeouts_tracked += 1;
                error!(
                    "Num timeouts tracked since last view change is {}. View {} timed out",
                    self.num_timeouts_tracked, *view_number
                );

                if self.num_timeouts_tracked >= 3 {
                    error!("Too many consecutive timeouts!  This shouldn't happen");
                }

                if self.num_timeouts_tracked >= 2 {
                    error!("Starting view sync protocol for view {}", *view_number + 1);
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
                        next_view + 1
                    } else {
                        next_view
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
                    self.send_to_or_create_replica(
                        Arc::new(HotShotEvent::ViewSyncTrigger(view_number + 1)),
                        view_number + 1,
                        &event_stream,
                    )
                    .await;
                } else {
                    // If this is the first timeout we've seen advance to the next view
                    self.current_view = view_number;
                    broadcast_event(
                        Arc::new(HotShotEvent::ViewChange(TYPES::Time::new(
                            *self.current_view,
                        ))),
                        &event_stream,
                    )
                    .await;
                }
            }

            _ => {}
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    ViewSyncReplicaTaskState<TYPES, I, A>
{
    #[instrument(skip_all, fields(id = self.id, view = *self.current_view), name = "View Sync Replica Task", level = "error")]
    /// Handle incoming events for the view sync replica task
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::ViewSyncPreCommitCertificate2Recv(certificate) => {
                let last_seen_certificate = ViewSyncPhase::PreCommit;

                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    warn!("We're already in a higher round");

                    return None;
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.membership.as_ref()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return None;
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return Some(HotShotTaskCompleted);
                }

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                let Ok(vote) = ViewSyncCommitVote::<TYPES>::create_signed_vote(
                    ViewSyncCommitData {
                        relay: certificate.get_data().relay,
                        round: self.next_view,
                    },
                    self.next_view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign ViewSyncCommitData!");
                    return None;
                };
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncCommitVote(vote);

                if let GeneralConsensusMessage::ViewSyncCommitVote(vote) = message {
                    broadcast_event(
                        Arc::new(HotShotEvent::ViewSyncCommitVoteSend(vote)),
                        &event_stream,
                    )
                    .await;
                }

                if let Some(timeout_task) = self.timeout_task.take() {
                    cancel_task(timeout_task).await;
                }

                self.timeout_task = Some(async_spawn({
                    let stream = event_stream.clone();
                    let phase = last_seen_certificate;
                    let relay = self.relay;
                    let next_view = self.next_view;
                    let timeout = self.view_sync_timeout;
                    async move {
                        async_sleep(timeout).await;
                        info!("Vote sending timed out in ViewSyncPreCommitCertificateRecv, Relay = {}", relay);

                        broadcast_event(
                            Arc::new(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*next_view),
                                relay,
                                phase,
                            )),
                            &stream,
                        )
                        .await;
                    }
                }));
            }

            HotShotEvent::ViewSyncCommitCertificate2Recv(certificate) => {
                let last_seen_certificate = ViewSyncPhase::Commit;

                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    warn!("We're already in a higher round");

                    return None;
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.membership.as_ref()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return None;
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return Some(HotShotTaskCompleted);
                }

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                let Ok(vote) = ViewSyncFinalizeVote::<TYPES>::create_signed_vote(
                    ViewSyncFinalizeData {
                        relay: certificate.get_data().relay,
                        round: self.next_view,
                    },
                    self.next_view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign view sync finalized vote!");
                    return None;
                };
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncFinalizeVote(vote);

                if let GeneralConsensusMessage::ViewSyncFinalizeVote(vote) = message {
                    broadcast_event(
                        Arc::new(HotShotEvent::ViewSyncFinalizeVoteSend(vote)),
                        &event_stream,
                    )
                    .await;
                }

                info!(
                    "View sync protocol has received view sync evidence to update the view to {}",
                    *self.next_view
                );

                broadcast_event(
                    Arc::new(HotShotEvent::ViewChange(self.next_view - 1)),
                    &event_stream,
                )
                .await;

                broadcast_event(
                    Arc::new(HotShotEvent::ViewChange(self.next_view)),
                    &event_stream,
                )
                .await;

                if let Some(timeout_task) = self.timeout_task.take() {
                    cancel_task(timeout_task).await;
                }
                self.timeout_task = Some(async_spawn({
                    let stream = event_stream.clone();
                    let phase = last_seen_certificate;
                    let relay = self.relay;
                    let next_view = self.next_view;
                    let timeout = self.view_sync_timeout;
                    async move {
                        async_sleep(timeout).await;
                        info!(
                            "Vote sending timed out in ViewSyncCommitCertificateRecv, relay = {}",
                            relay
                        );
                        broadcast_event(
                            Arc::new(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*next_view),
                                relay,
                                phase,
                            )),
                            &stream,
                        )
                        .await;
                    }
                }));
            }

            HotShotEvent::ViewSyncFinalizeCertificate2Recv(certificate) => {
                // Ignore certificate if it is for an older round
                if certificate.get_view_number() < self.next_view {
                    warn!("We're already in a higher round");

                    return None;
                }

                // If certificate is not valid, return current state
                if !certificate.is_valid_cert(self.membership.as_ref()) {
                    error!("Not valid view sync cert! {:?}", certificate.get_data());

                    return None;
                }

                // If certificate is for a higher round shutdown this task
                // since another task should have been started for the higher round
                if certificate.get_view_number() > self.next_view {
                    return Some(HotShotTaskCompleted);
                }

                // cancel poll for votes
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForViewSyncVotes(
                        *certificate.view_number,
                    ))
                    .await;

                // cancel poll for view sync cert
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForViewSyncCertificate(
                        *certificate.view_number,
                    ))
                    .await;

                if certificate.get_data().relay > self.relay {
                    self.relay = certificate.get_data().relay;
                }

                if let Some(timeout_task) = self.timeout_task.take() {
                    cancel_task(timeout_task).await;
                }

                broadcast_event(
                    Arc::new(HotShotEvent::ViewChange(self.next_view)),
                    &event_stream,
                )
                .await;
                return Some(HotShotTaskCompleted);
            }

            HotShotEvent::ViewSyncTrigger(view_number) => {
                let view_number = *view_number;
                if self.next_view != TYPES::Time::new(*view_number) {
                    error!("Unexpected view number to triger view sync");
                    return None;
                }

                let Ok(vote) = ViewSyncPreCommitVote::<TYPES>::create_signed_vote(
                    ViewSyncPreCommitData {
                        relay: 0,
                        round: view_number,
                    },
                    view_number,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign pre commit vote!");
                    return None;
                };
                let message = GeneralConsensusMessage::<TYPES>::ViewSyncPreCommitVote(vote);

                if let GeneralConsensusMessage::ViewSyncPreCommitVote(vote) = message {
                    broadcast_event(
                        Arc::new(HotShotEvent::ViewSyncPreCommitVoteSend(vote)),
                        &event_stream,
                    )
                    .await;
                }

                self.timeout_task = Some(async_spawn({
                    let stream = event_stream.clone();
                    let relay = self.relay;
                    let next_view = self.next_view;
                    let timeout = self.view_sync_timeout;
                    async move {
                        async_sleep(timeout).await;
                        info!("Vote sending timed out in ViewSyncTrigger");
                        broadcast_event(
                            Arc::new(HotShotEvent::ViewSyncTimeout(
                                TYPES::Time::new(*next_view),
                                relay,
                                ViewSyncPhase::None,
                            )),
                            &stream,
                        )
                        .await;
                    }
                }));

                return None;
            }

            HotShotEvent::ViewSyncTimeout(round, relay, last_seen_certificate) => {
                let round = *round;
                // Shouldn't ever receive a timeout for a relay higher than ours
                if TYPES::Time::new(*round) == self.next_view && *relay == self.relay {
                    if let Some(timeout_task) = self.timeout_task.take() {
                        cancel_task(timeout_task).await;
                    }
                    self.relay += 1;
                    match last_seen_certificate {
                        ViewSyncPhase::None | ViewSyncPhase::PreCommit | ViewSyncPhase::Commit => {
                            let Ok(vote) = ViewSyncPreCommitVote::<TYPES>::create_signed_vote(
                                ViewSyncPreCommitData {
                                    relay: self.relay,
                                    round: self.next_view,
                                },
                                self.next_view,
                                &self.public_key,
                                &self.private_key,
                            ) else {
                                error!("Failed to sign ViewSyncPreCommitData!");
                                return None;
                            };
                            let message =
                                GeneralConsensusMessage::<TYPES>::ViewSyncPreCommitVote(vote);

                            if let GeneralConsensusMessage::ViewSyncPreCommitVote(vote) = message {
                                broadcast_event(
                                    Arc::new(HotShotEvent::ViewSyncPreCommitVoteSend(vote)),
                                    &event_stream,
                                )
                                .await;
                            }
                        }
                        ViewSyncPhase::Finalize => {
                            // This should never occur
                            unimplemented!()
                        }
                    }

                    self.timeout_task = Some(async_spawn({
                        let stream = event_stream.clone();
                        let relay = self.relay;
                        let next_view = self.next_view;
                        let timeout = self.view_sync_timeout;
                        let last_cert = last_seen_certificate.clone();
                        async move {
                            async_sleep(timeout).await;
                            info!(
                                "Vote sending timed out in ViewSyncTimeout relay = {}",
                                relay
                            );
                            broadcast_event(
                                Arc::new(HotShotEvent::ViewSyncTimeout(
                                    TYPES::Time::new(*next_view),
                                    relay,
                                    last_cert,
                                )),
                                &stream,
                            )
                            .await;
                        }
                    }));

                    return None;
                }
            }
            _ => return None,
        }
        None
    }
}
