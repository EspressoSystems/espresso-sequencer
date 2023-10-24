//! Provides a number of tasks that run continuously on a [`HotShot`]

use crate::{
    async_spawn, types::SystemContextHandle, DACertificate, HotShotSequencingConsensusApi,
    QuorumCertificate, SequencingQuorumEx,
};
use async_compatibility_layer::art::async_sleep;
use commit::{Commitment, CommitmentBounds, Committable};
use futures::FutureExt;
use hotshot_task::{
    boxed_sync,
    event_stream::ChannelStream,
    task::{FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes},
    task_impls::TaskBuilder,
    task_launcher::TaskRunner,
    GeneratedStream, Merge,
};
use hotshot_task_impls::{
    consensus::{consensus_event_filter, ConsensusTaskTypes, SequencingConsensusTaskState},
    da::{DATaskState, DATaskTypes},
    events::SequencingHotShotEvent,
    network::{
        NetworkEventTaskState, NetworkEventTaskTypes, NetworkMessageTaskState,
        NetworkMessageTaskTypes, NetworkTaskKind,
    },
    transactions::{TransactionTaskState, TransactionsTaskTypes},
    vid::{VIDTaskState, VIDTaskTypes},
    view_sync::{ViewSyncTaskState, ViewSyncTaskStateTypes},
};
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload, VIDTransaction},
    certificate::{TimeoutCertificate, VIDCertificate, ViewSyncCertificate},
    data::{ProposalType, QuorumProposal, SequencingLeaf},
    event::Event,
    message::{Message, Messages, SequencingMessage},
    traits::{
        election::{ConsensusExchange, Membership},
        network::{CommunicationChannel, ConsensusIntentEvent, TransmitType},
        node_implementation::{
            CommitteeEx, ExchangesType, NodeImplementation, NodeType, QuorumEx,
            SequencingTimeoutEx, VIDEx, ViewSyncEx,
        },
        state::ConsensusTime,
    },
    vote::{ViewSyncData, VoteType},
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
    time::Duration,
};

/// event for global event stream
#[derive(Clone, Debug)]
pub enum GlobalEvent {
    /// shut everything down
    Shutdown,
    /// dummy (TODO delete later)
    Dummy,
}

/// Add the network task to handle messages and publish events.
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_network_message_task<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
    COMMITMENT: CommitmentBounds,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    VOTE: VoteType<TYPES, COMMITMENT>,
    MEMBERSHIP: Membership<TYPES>,
    EXCHANGE: ConsensusExchange<
            TYPES,
            Message<TYPES, I>,
            Proposal = PROPOSAL,
            Vote = VOTE,
            Membership = MEMBERSHIP,
        > + 'static,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    exchange: EXCHANGE,
) -> TaskRunner
// This bound is required so that we can call the `recv_msgs` function of `CommunicationChannel`.
where
    EXCHANGE::Networking: CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>,
{
    let channel = exchange.network().clone();
    let broadcast_stream = GeneratedStream::<Messages<TYPES, I>>::new(Arc::new(move || {
        let network = channel.clone();
        let closure = async move {
            loop {
                let msgs = Messages(
                    network
                        .recv_msgs(TransmitType::Broadcast)
                        .await
                        .expect("Failed to receive broadcast messages"),
                );
                if msgs.0.is_empty() {
                    async_sleep(Duration::from_millis(100)).await;
                } else {
                    break msgs;
                }
            }
        };
        Some(boxed_sync(closure))
    }));
    let channel = exchange.network().clone();
    let direct_stream = GeneratedStream::<Messages<TYPES, I>>::new(Arc::new(move || {
        let network = channel.clone();
        let closure = async move {
            loop {
                let msgs = Messages(
                    network
                        .recv_msgs(TransmitType::Direct)
                        .await
                        .expect("Failed to receive direct messages"),
                );
                if msgs.0.is_empty() {
                    async_sleep(Duration::from_millis(100)).await;
                } else {
                    break msgs;
                }
            }
        };
        Some(boxed_sync(closure))
    }));
    let message_stream = Merge::new(broadcast_stream, direct_stream);
    let network_state: NetworkMessageTaskState<_, _> = NetworkMessageTaskState {
        event_stream: event_stream.clone(),
    };
    let registry = task_runner.registry.clone();
    let network_message_handler = HandleMessage(Arc::new(
        move |messages: either::Either<Messages<TYPES, I>, Messages<TYPES, I>>,
              mut state: NetworkMessageTaskState<TYPES, I>| {
            let messages = match messages {
                either::Either::Left(messages) | either::Either::Right(messages) => messages,
            };
            async move {
                state.handle_messages(messages.0).await;
                (None, state)
            }
            .boxed()
        },
    ));
    let networking_name = "Networking Task";

    let networking_task_builder =
        TaskBuilder::<NetworkMessageTaskTypes<_, _>>::new(networking_name.to_string())
            .register_message_stream(message_stream)
            .register_registry(&mut registry.clone())
            .await
            .register_state(network_state)
            .register_message_handler(network_message_handler);

    // impossible for unwraps to fail
    // we *just* registered
    let networking_task_id = networking_task_builder.get_task_id().unwrap();
    let networking_task = NetworkMessageTaskTypes::build(networking_task_builder).launch();

    task_runner.add_task(
        networking_task_id,
        networking_name.to_string(),
        networking_task,
    )
}

/// Add the network task to handle events and send messages.
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_network_event_task<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
    COMMITMENT: CommitmentBounds,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    VOTE: VoteType<TYPES, COMMITMENT>,
    MEMBERSHIP: Membership<TYPES>,
    EXCHANGE: ConsensusExchange<
            TYPES,
            Message<TYPES, I>,
            Proposal = PROPOSAL,
            Vote = VOTE,
            Membership = MEMBERSHIP,
        > + 'static,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    exchange: EXCHANGE,
    task_kind: NetworkTaskKind,
) -> TaskRunner
// This bound is required so that we can call the `recv_msgs` function of `CommunicationChannel`.
where
    EXCHANGE::Networking: CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>,
{
    let filter = NetworkEventTaskState::<
        TYPES,
        I,
        MEMBERSHIP,
        <EXCHANGE as ConsensusExchange<_, _>>::Networking,
    >::filter(task_kind);
    let channel = exchange.network().clone();
    let network_state: NetworkEventTaskState<_, _, _, _> = NetworkEventTaskState {
        channel,
        event_stream: event_stream.clone(),
        view: TYPES::Time::genesis(),
        phantom: PhantomData,
    };
    let registry = task_runner.registry.clone();
    let network_event_handler = HandleEvent(Arc::new(
        move |event, mut state: NetworkEventTaskState<_, _, MEMBERSHIP, _>| {
            let membership = exchange.membership().clone();
            async move {
                let completion_status = state.handle_event(event, &membership).await;
                (completion_status, state)
            }
            .boxed()
        },
    ));
    let networking_name = "Networking Task";

    let networking_task_builder =
        TaskBuilder::<NetworkEventTaskTypes<_, _, _, _>>::new(networking_name.to_string())
            .register_event_stream(event_stream.clone(), filter)
            .await
            .register_registry(&mut registry.clone())
            .await
            .register_state(network_state)
            .register_event_handler(network_event_handler);

    // impossible for unwraps to fail
    // we *just* registered
    let networking_task_id = networking_task_builder.get_task_id().unwrap();
    let networking_task = NetworkEventTaskTypes::build(networking_task_builder).launch();

    task_runner.add_task(
        networking_task_id,
        networking_name.to_string(),
        networking_task,
    )
}

/// add the consensus task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_consensus_task<
    TYPES: NodeType<BlockHeader = VIDBlockHeader, BlockPayload = VIDBlockPayload>,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    output_stream: ChannelStream<Event<TYPES, I::Leaf>>,
    handle: SystemContextHandle<TYPES, I>,
) -> TaskRunner
where
    SequencingQuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, SequencingLeaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, Commitment<SequencingLeaf<TYPES>>>,
        Commitment = Commitment<SequencingLeaf<TYPES>>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockPayload>,
    >,
    SequencingTimeoutEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, SequencingLeaf<TYPES>>,
        Certificate = TimeoutCertificate<TYPES>,
        Commitment = Commitment<TYPES::Time>,
    >,
{
    let consensus = handle.hotshot.get_consensus();
    let c_api: HotShotSequencingConsensusApi<TYPES, I> = HotShotSequencingConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let registry = task_runner.registry.clone();
    // build the consensus task
    let consensus_state = SequencingConsensusTaskState {
        registry: registry.clone(),
        consensus,
        timeout: handle.hotshot.inner.config.next_view_timeout,
        cur_view: TYPES::Time::new(0),
        block_commitment: Some(VIDBlockPayload::genesis().commit()),
        quorum_exchange: c_api.inner.exchanges.quorum_exchange().clone().into(),
        timeout_exchange: c_api.inner.exchanges.timeout_exchange().clone().into(),
        api: c_api.clone(),
        committee_exchange: c_api.inner.exchanges.committee_exchange().clone().into(),
        _pd: PhantomData,
        vote_collector: None,
        timeout_task: async_spawn(async move {}),
        event_stream: event_stream.clone(),
        output_event_stream: output_stream,
        da_certs: HashMap::new(),
        vid_certs: HashMap::new(),
        current_proposal: None,
        id: handle.hotshot.inner.id,
        qc: None,
    };
    consensus_state
        .quorum_exchange
        .network()
        .inject_consensus_info(ConsensusIntentEvent::PollForCurrentProposal)
        .await;
    consensus_state
        .quorum_exchange
        .network()
        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(1))
        .await;
    let filter = FilterEvent(Arc::new(consensus_event_filter));
    let consensus_name = "Consensus Task";
    let consensus_event_handler = HandleEvent(Arc::new(
        move |event,
              mut state: SequencingConsensusTaskState<
            TYPES,
            I,
            HotShotSequencingConsensusApi<TYPES, I>,
        >| {
            async move {
                if let SequencingHotShotEvent::Shutdown = event {
                    (Some(HotShotTaskCompleted::ShutDown), state)
                } else {
                    state.handle_event(event).await;
                    (None, state)
                }
            }
            .boxed()
        },
    ));
    let consensus_task_builder = TaskBuilder::<
        ConsensusTaskTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
    >::new(consensus_name.to_string())
    .register_event_stream(event_stream.clone(), filter)
    .await
    .register_registry(&mut registry.clone())
    .await
    .register_state(consensus_state)
    .register_event_handler(consensus_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let consensus_task_id = consensus_task_builder.get_task_id().unwrap();
    let consensus_task = ConsensusTaskTypes::build(consensus_task_builder).launch();

    task_runner.add_task(
        consensus_task_id,
        consensus_name.to_string(),
        consensus_task,
    )
}

/// add the VID task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_vid_task<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    vid_exchange: VIDEx<TYPES, I>,
    handle: SystemContextHandle<TYPES, I>,
) -> TaskRunner
where
    VIDEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = VIDCertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockPayload>,
    >,
{
    // build the vid task
    let c_api: HotShotSequencingConsensusApi<TYPES, I> = HotShotSequencingConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let registry = task_runner.registry.clone();
    let vid_state = VIDTaskState {
        registry: registry.clone(),
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        cur_view: TYPES::Time::new(0),
        vid_exchange: vid_exchange.into(),
        vote_collector: None,
        event_stream: event_stream.clone(),
        id: handle.hotshot.inner.id,
    };
    let vid_event_handler = HandleEvent(Arc::new(
        move |event, mut state: VIDTaskState<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>| {
            async move {
                let completion_status = state.handle_event(event).await;
                (completion_status, state)
            }
            .boxed()
        },
    ));
    let vid_name = "VID Task";
    let vid_event_filter = FilterEvent(Arc::new(
        VIDTaskState::<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>::filter,
    ));

    let vid_task_builder = TaskBuilder::<
        VIDTaskTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
    >::new(vid_name.to_string())
    .register_event_stream(event_stream.clone(), vid_event_filter)
    .await
    .register_registry(&mut registry.clone())
    .await
    .register_state(vid_state)
    .register_event_handler(vid_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let vid_task_id = vid_task_builder.get_task_id().unwrap();
    let vid_task = VIDTaskTypes::build(vid_task_builder).launch();
    task_runner.add_task(vid_task_id, vid_name.to_string(), vid_task)
}

/// add the Data Availability task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_da_task<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    committee_exchange: CommitteeEx<TYPES, I>,
    handle: SystemContextHandle<TYPES, I>,
) -> TaskRunner
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = Commitment<TYPES::BlockPayload>,
    >,
{
    // build the da task
    let c_api: HotShotSequencingConsensusApi<TYPES, I> = HotShotSequencingConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let registry = task_runner.registry.clone();
    let da_state = DATaskState {
        registry: registry.clone(),
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        cur_view: TYPES::Time::new(0),
        committee_exchange: committee_exchange.into(),
        vote_collector: None,
        event_stream: event_stream.clone(),
        id: handle.hotshot.inner.id,
    };
    let da_event_handler = HandleEvent(Arc::new(
        move |event, mut state: DATaskState<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>| {
            async move {
                let completion_status = state.handle_event(event).await;
                (completion_status, state)
            }
            .boxed()
        },
    ));
    let da_name = "DA Task";
    let da_event_filter = FilterEvent(Arc::new(
        DATaskState::<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>::filter,
    ));

    let da_task_builder = TaskBuilder::<
        DATaskTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
    >::new(da_name.to_string())
    .register_event_stream(event_stream.clone(), da_event_filter)
    .await
    .register_registry(&mut registry.clone())
    .await
    .register_state(da_state)
    .register_event_handler(da_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let da_task_id = da_task_builder.get_task_id().unwrap();
    let da_task = DATaskTypes::build(da_task_builder).launch();
    task_runner.add_task(da_task_id, da_name.to_string(), da_task)
}

/// add the Transaction Handling task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_transaction_task<
    TYPES: NodeType<Transaction = VIDTransaction, BlockPayload = VIDBlockPayload>,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    quorum_exchange: QuorumEx<TYPES, I>,
    handle: SystemContextHandle<TYPES, I>,
) -> TaskRunner
where
    QuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = QuorumCertificate<TYPES, Commitment<I::Leaf>>,
    >,
{
    // build the transactions task
    let c_api: HotShotSequencingConsensusApi<TYPES, I> = HotShotSequencingConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let registry = task_runner.registry.clone();
    let transactions_state = TransactionTaskState {
        registry: registry.clone(),
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        transactions: Arc::default(),
        seen_transactions: HashSet::new(),
        cur_view: TYPES::Time::new(0),
        quorum_exchange: quorum_exchange.into(),
        event_stream: event_stream.clone(),
        id: handle.hotshot.inner.id,
    };
    let transactions_event_handler = HandleEvent(Arc::new(
        move |event,
              mut state: TransactionTaskState<
            TYPES,
            I,
            HotShotSequencingConsensusApi<TYPES, I>,
        >| {
            async move {
                let completion_status = state.handle_event(event).await;
                (completion_status, state)
            }
            .boxed()
        },
    ));
    let transactions_name = "Transactions Task";
    let transactions_event_filter = FilterEvent(Arc::new(
        TransactionTaskState::<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>::filter,
    ));

    let transactions_task_builder = TaskBuilder::<
        TransactionsTaskTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
    >::new(transactions_name.to_string())
    .register_event_stream(event_stream.clone(), transactions_event_filter)
    .await
    .register_registry(&mut registry.clone())
    .await
    .register_state(transactions_state)
    .register_event_handler(transactions_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let da_task_id = transactions_task_builder.get_task_id().unwrap();
    let da_task = TransactionsTaskTypes::build(transactions_task_builder).launch();
    task_runner.add_task(da_task_id, transactions_name.to_string(), da_task)
}
/// add the view sync task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_view_sync_task<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    handle: SystemContextHandle<TYPES, I>,
) -> TaskRunner
where
    ViewSyncEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = ViewSyncCertificate<TYPES>,
        Certificate = ViewSyncCertificate<TYPES>,
        Commitment = Commitment<ViewSyncData<TYPES>>,
    >,
{
    let api = HotShotSequencingConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    // build the view sync task
    let view_sync_state = ViewSyncTaskState {
        registry: task_runner.registry.clone(),
        event_stream: event_stream.clone(),
        current_view: TYPES::Time::new(0),
        next_view: TYPES::Time::new(0),
        exchange: (*api.inner.exchanges.view_sync_exchange()).clone().into(),
        api,
        num_timeouts_tracked: 0,
        replica_task_map: HashMap::default(),
        relay_task_map: HashMap::default(),
        view_sync_timeout: Duration::new(5, 0),
        id: handle.hotshot.inner.id,
        last_garbage_collected_view: TYPES::Time::new(0),
    };
    let registry = task_runner.registry.clone();
    let view_sync_event_handler =
        HandleEvent(Arc::new(
            move |event,
                  mut state: ViewSyncTaskState<
                TYPES,
                I,
                HotShotSequencingConsensusApi<TYPES, I>,
            >| {
                async move {
                    if let SequencingHotShotEvent::Shutdown = event {
                        (Some(HotShotTaskCompleted::ShutDown), state)
                    } else {
                        state.handle_event(event).await;
                        (None, state)
                    }
                }
                .boxed()
            },
        ));
    let view_sync_name = "ViewSync Task";
    let view_sync_event_filter = FilterEvent(Arc::new(
        ViewSyncTaskState::<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>::filter,
    ));

    let view_sync_task_builder = TaskBuilder::<
        ViewSyncTaskStateTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
    >::new(view_sync_name.to_string())
    .register_event_stream(event_stream.clone(), view_sync_event_filter)
    .await
    .register_registry(&mut registry.clone())
    .await
    .register_state(view_sync_state)
    .register_event_handler(view_sync_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let view_sync_task_id = view_sync_task_builder.get_task_id().unwrap();

    let view_sync_task = ViewSyncTaskStateTypes::build(view_sync_task_builder).launch();
    task_runner.add_task(
        view_sync_task_id,
        view_sync_name.to_string(),
        view_sync_task,
    )
}
