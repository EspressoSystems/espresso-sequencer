use std::{collections::HashMap, fmt::Debug, marker::PhantomData, sync::Arc};

use crate::events::HotShotEvent;
use async_compatibility_layer::art::async_spawn;
use async_trait::async_trait;
use bitvec::prelude::*;
use either::Either::{self, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::{
    simple_certificate::{
        DACertificate, QuorumCertificate, TimeoutCertificate, VIDCertificate,
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        DAVote, QuorumVote, TimeoutVote, VIDVote, ViewSyncCommitVote, ViewSyncFinalizeVote,
        ViewSyncPreCommitVote,
    },
    traits::{election::Membership, node_implementation::NodeType},
    vote::{Certificate, HasViewNumber, Vote, VoteAccumulator},
};
use snafu::Snafu;
use tracing::{debug, error};

#[derive(Snafu, Debug)]
/// Stub of a vote error
pub struct VoteTaskError {}

/// Task state for collecting votes of one type and emiting a certificate
pub struct VoteCollectionTaskState<
    TYPES: NodeType,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment> + Debug,
> {
    /// Public key for this node.
    pub public_key: TYPES::SignatureKey,

    /// Membership for voting
    pub membership: Arc<TYPES::Membership>,

    /// accumulator handles aggregating the votes
    pub accumulator: Option<VoteAccumulator<TYPES, VOTE, CERT>>,

    /// The view which we are collecting votes for
    pub view: TYPES::Time,

    /// global event stream
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// Node id
    pub id: u64,
}

/// Describes the functions a vote must implement for it to be aggregatable by the generic vote collection task
pub trait AggregatableVote<
    TYPES: NodeType,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
>
{
    /// return the leader for this votes
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey;

    /// return the Hotshot event for the completion of this CERT
    fn make_cert_event(certificate: CERT, key: &TYPES::SignatureKey) -> HotShotEvent<TYPES>;
}

impl<
        TYPES: NodeType,
        VOTE: Vote<TYPES> + AggregatableVote<TYPES, VOTE, CERT>,
        CERT: Certificate<TYPES, Voteable = VOTE::Commitment> + Debug,
    > VoteCollectionTaskState<TYPES, VOTE, CERT>
{
    /// Take one vote and accumultate it. Returns either the cert or the updated state
    /// after the vote is accumulated
    pub async fn accumulate_vote(mut self, vote: &VOTE) -> (Option<HotShotTaskCompleted>, Self) {
        if vote.get_leader(&self.membership) != self.public_key {
            return (None, self);
        }

        if vote.get_view_number() != self.view {
            error!(
                "Vote view does not match! vote view is {} current view is {}",
                *vote.get_view_number(),
                *self.view
            );
            return (None, self);
        }
        let Some(accumulator) = self.accumulator else {
            return (None, self);
        };
        match accumulator.accumulate(vote, &self.membership) {
            Either::Left(acc) => {
                self.accumulator = Some(acc);
                (None, self)
            }
            Either::Right(qc) => {
                debug!("Certificate Formed! {:?}", qc);
                self.event_stream
                    .publish(VOTE::make_cert_event(qc, &self.public_key))
                    .await;
                self.accumulator = None;
                (Some(HotShotTaskCompleted::ShutDown), self)
            }
        }
    }
}

impl<
        TYPES: NodeType,
        VOTE: Vote<TYPES>
            + AggregatableVote<TYPES, VOTE, CERT>
            + std::marker::Send
            + std::marker::Sync
            + 'static,
        CERT: Certificate<TYPES, Voteable = VOTE::Commitment>
            + Debug
            + std::marker::Send
            + std::marker::Sync
            + 'static,
    > TS for VoteCollectionTaskState<TYPES, VOTE, CERT>
{
}

/// Types for a vote accumulator Task
pub type VoteTaskStateTypes<TYPES, VOTE, CERT> = HSTWithEvent<
    VoteTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    VoteCollectionTaskState<TYPES, VOTE, CERT>,
>;

/// Trait for types which will handle a vote event.
#[async_trait]
pub trait HandleVoteEvent<TYPES, VOTE, CERT>
where
    TYPES: NodeType,
    VOTE: Vote<TYPES> + AggregatableVote<TYPES, VOTE, CERT>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment> + Debug,
{
    /// Handle a vote event
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (
        Option<HotShotTaskCompleted>,
        VoteCollectionTaskState<TYPES, VOTE, CERT>,
    );

    /// Event filter to use for this event
    fn filter(event: &HotShotEvent<TYPES>) -> bool;
}

/// Info needed to create a vote accumulator task
#[allow(missing_docs)]
pub struct AccumulatorInfo<TYPES: NodeType> {
    pub public_key: TYPES::SignatureKey,
    pub membership: Arc<TYPES::Membership>,
    pub view: TYPES::Time,
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,
    pub id: u64,
    pub registry: GlobalRegistry,
}

/// Generic function for spawnnig a vote task.  Returns the event stream id of the spawned task if created
/// # Panics
/// Calls unwrap but should never panic.
pub async fn spawn_vote_accumulator<TYPES, VOTE, CERT>(
    info: &AccumulatorInfo<TYPES>,
    vote: VOTE,
    event: HotShotEvent<TYPES>,
    name: String,
) -> Option<(TYPES::Time, usize, usize)>
where
    TYPES: NodeType,
    VOTE: Vote<TYPES>
        + AggregatableVote<TYPES, VOTE, CERT>
        + std::marker::Send
        + std::marker::Sync
        + 'static,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>
        + Debug
        + std::marker::Send
        + std::marker::Sync
        + 'static,
    VoteCollectionTaskState<TYPES, VOTE, CERT>: HandleVoteEvent<TYPES, VOTE, CERT>,
{
    if vote.get_leader(info.membership.as_ref()) != info.public_key {
        debug!("Vote is not to the leader");
        return None;
    }

    if vote.get_view_number() != info.view {
        error!(
            "Vote view does not match! vote view is {} current view is {}",
            *vote.get_view_number(),
            *info.view
        );
        return None;
    }
    let new_accumulator = VoteAccumulator {
        vote_outcomes: HashMap::new(),
        sig_lists: Vec::new(),
        signers: bitvec![0;info. membership.total_nodes()],
        phantom: PhantomData,
    };

    let mut state = VoteCollectionTaskState::<TYPES, VOTE, CERT> {
        event_stream: info.event_stream.clone(),
        membership: info.membership.clone(),
        public_key: info.public_key.clone(),
        accumulator: Some(new_accumulator),
        view: info.view,
        id: info.id,
    };

    let result = state.handle_event(event.clone()).await;

    if result.0 == Some(HotShotTaskCompleted::ShutDown) {
        // The protocol has finished
        return None;
    }

    state = result.1;

    let relay_handle_event = HandleEvent(Arc::new(
        move |event, state: VoteCollectionTaskState<TYPES, VOTE, CERT>| {
            async move { state.handle_event(event).await }.boxed()
        },
    ));

    let filter = FilterEvent(Arc::new(
        <VoteCollectionTaskState<TYPES, VOTE, CERT> as HandleVoteEvent<TYPES, VOTE, CERT>>::filter,
    ));
    let builder = TaskBuilder::<VoteTaskStateTypes<TYPES, VOTE, CERT>>::new(name)
        .register_event_stream(state.event_stream.clone(), filter)
        .await
        .register_registry(&mut info.registry.clone())
        .await
        .register_state(state)
        .register_event_handler(relay_handle_event);

    let event_stream_id = builder.get_stream_id().unwrap();
    let id = builder.get_task_id().unwrap();

    let _task = async_spawn(async move { VoteTaskStateTypes::build(builder).launch().await });

    Some((vote.get_view_number(), id, event_stream_id))
}

/// Alias for Quorum vote accumulator
type QuorumVoteState<TYPES> =
    VoteCollectionTaskState<TYPES, QuorumVote<TYPES>, QuorumCertificate<TYPES>>;
/// Alias for DA vote accumulator
type DAVoteState<TYPES> = VoteCollectionTaskState<TYPES, DAVote<TYPES>, DACertificate<TYPES>>;
/// Alias for VID vote accumulator state
type VIDVoteState<TYPES> = VoteCollectionTaskState<TYPES, VIDVote<TYPES>, VIDCertificate<TYPES>>;
/// Alias for Timeout vote accumulator
type TimeoutVoteState<TYPES> =
    VoteCollectionTaskState<TYPES, TimeoutVote<TYPES>, TimeoutCertificate<TYPES>>;
/// Alias for View Sync Pre Commit vote accumulator
type ViewSyncPreCommitState<TYPES> = VoteCollectionTaskState<
    TYPES,
    ViewSyncPreCommitVote<TYPES>,
    ViewSyncPreCommitCertificate2<TYPES>,
>;
/// Alias for View Sync Commit vote accumulator
type ViewSyncCommitVoteState<TYPES> =
    VoteCollectionTaskState<TYPES, ViewSyncCommitVote<TYPES>, ViewSyncCommitCertificate2<TYPES>>;
/// Alias for View Sync Finalize vote accumulator
type ViewSyncFinalizeVoteState<TYPES> = VoteCollectionTaskState<
    TYPES,
    ViewSyncFinalizeVote<TYPES>,
    ViewSyncFinalizeCertificate2<TYPES>,
>;

impl<TYPES: NodeType> AggregatableVote<TYPES, QuorumVote<TYPES>, QuorumCertificate<TYPES>>
    for QuorumVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_view_number() + 1)
    }
    fn make_cert_event(
        certificate: QuorumCertificate<TYPES>,
        _key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::QCFormed(Left(certificate))
    }
}

impl<TYPES: NodeType> AggregatableVote<TYPES, DAVote<TYPES>, DACertificate<TYPES>>
    for DAVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_view_number())
    }
    fn make_cert_event(
        certificate: DACertificate<TYPES>,
        key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::DACSend(certificate, key.clone())
    }
}

impl<TYPES: NodeType> AggregatableVote<TYPES, TimeoutVote<TYPES>, TimeoutCertificate<TYPES>>
    for TimeoutVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_view_number() + 1)
    }
    fn make_cert_event(
        certificate: TimeoutCertificate<TYPES>,
        _key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::QCFormed(Right(certificate))
    }
}

impl<TYPES: NodeType> AggregatableVote<TYPES, VIDVote<TYPES>, VIDCertificate<TYPES>>
    for VIDVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_view_number())
    }
    fn make_cert_event(
        certificate: VIDCertificate<TYPES>,
        key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::VidCertSend(certificate, key.clone())
    }
}

impl<TYPES: NodeType>
    AggregatableVote<TYPES, ViewSyncCommitVote<TYPES>, ViewSyncCommitCertificate2<TYPES>>
    for ViewSyncCommitVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_data().round + self.get_data().relay)
    }
    fn make_cert_event(
        certificate: ViewSyncCommitCertificate2<TYPES>,
        key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::ViewSyncCommitCertificate2Send(certificate, key.clone())
    }
}

impl<TYPES: NodeType>
    AggregatableVote<TYPES, ViewSyncPreCommitVote<TYPES>, ViewSyncPreCommitCertificate2<TYPES>>
    for ViewSyncPreCommitVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_data().round + self.get_data().relay)
    }
    fn make_cert_event(
        certificate: ViewSyncPreCommitCertificate2<TYPES>,
        key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::ViewSyncPreCommitCertificate2Send(certificate, key.clone())
    }
}

impl<TYPES: NodeType>
    AggregatableVote<TYPES, ViewSyncFinalizeVote<TYPES>, ViewSyncFinalizeCertificate2<TYPES>>
    for ViewSyncFinalizeVote<TYPES>
{
    fn get_leader(&self, membership: &TYPES::Membership) -> TYPES::SignatureKey {
        membership.get_leader(self.get_data().round + self.get_data().relay)
    }
    fn make_cert_event(
        certificate: ViewSyncFinalizeCertificate2<TYPES>,
        key: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        HotShotEvent::ViewSyncFinalizeCertificate2Send(certificate, key.clone())
    }
}

// Handlers for all vote accumulators
#[async_trait]
impl<TYPES: NodeType> HandleVoteEvent<TYPES, QuorumVote<TYPES>, QuorumCertificate<TYPES>>
    for QuorumVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, QuorumVoteState<TYPES>) {
        match event {
            HotShotEvent::QuorumVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::QuorumVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType> HandleVoteEvent<TYPES, DAVote<TYPES>, DACertificate<TYPES>>
    for DAVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, DAVoteState<TYPES>) {
        match event {
            HotShotEvent::DAVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::DAVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType> HandleVoteEvent<TYPES, TimeoutVote<TYPES>, TimeoutCertificate<TYPES>>
    for TimeoutVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, TimeoutVoteState<TYPES>) {
        match event {
            HotShotEvent::TimeoutVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::TimeoutVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType> HandleVoteEvent<TYPES, VIDVote<TYPES>, VIDCertificate<TYPES>>
    for VIDVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, VIDVoteState<TYPES>) {
        match event {
            HotShotEvent::VidVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::VidVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType>
    HandleVoteEvent<TYPES, ViewSyncPreCommitVote<TYPES>, ViewSyncPreCommitCertificate2<TYPES>>
    for ViewSyncPreCommitState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, ViewSyncPreCommitState<TYPES>) {
        match event {
            HotShotEvent::ViewSyncPreCommitVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::ViewSyncPreCommitVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType>
    HandleVoteEvent<TYPES, ViewSyncCommitVote<TYPES>, ViewSyncCommitCertificate2<TYPES>>
    for ViewSyncCommitVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (Option<HotShotTaskCompleted>, ViewSyncCommitVoteState<TYPES>) {
        match event {
            HotShotEvent::ViewSyncCommitVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::ViewSyncCommitVoteRecv(_))
    }
}

#[async_trait]
impl<TYPES: NodeType>
    HandleVoteEvent<TYPES, ViewSyncFinalizeVote<TYPES>, ViewSyncFinalizeCertificate2<TYPES>>
    for ViewSyncFinalizeVoteState<TYPES>
{
    async fn handle_event(
        self,
        event: HotShotEvent<TYPES>,
    ) -> (
        Option<HotShotTaskCompleted>,
        ViewSyncFinalizeVoteState<TYPES>,
    ) {
        match event {
            HotShotEvent::ViewSyncFinalizeVoteRecv(vote) => self.accumulate_vote(&vote).await,
            _ => (None, self),
        }
    }
    fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(event, HotShotEvent::ViewSyncFinalizeVoteRecv(_))
    }
}
