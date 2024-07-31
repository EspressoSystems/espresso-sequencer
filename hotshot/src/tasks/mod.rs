//! Provides a number of tasks that run continuously

/// Provides trait to create task states from a `SystemContextHandle`
pub mod task_state;
use std::{sync::Arc, time::Duration};

use async_broadcast::broadcast;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_task::task::Task;
#[cfg(feature = "rewind")]
use hotshot_task_impls::rewind::RewindTaskState;
use hotshot_task_impls::{
    da::DaTaskState,
    events::HotShotEvent,
    network,
    network::{NetworkEventTaskState, NetworkMessageTaskState},
    request::NetworkRequestState,
    response::{run_response_task, NetworkResponseState, RequestReceiver},
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VidTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::{
    constants::EVENT_CHANNEL_SIZE,
    message::{Messages, VersionedMessage},
    traits::{
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
};
use vbs::version::StaticVersionType;

use crate::{
    tasks::task_state::CreateTaskState, types::SystemContextHandle, ConsensusApi,
    ConsensusMetricsValue, ConsensusTaskRegistry, HotShotConfig, HotShotInitializer, Memberships,
    NetworkTaskRegistry, SignatureKey, SystemContext,
};

/// event for global event stream
#[derive(Clone, Debug)]
pub enum GlobalEvent {
    /// shut everything down
    Shutdown,
    /// dummy (TODO delete later)
    Dummy,
}

/// Add tasks for network requests and responses
pub async fn add_request_network_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    handle: &mut SystemContextHandle<TYPES, I>,
) {
    let state = NetworkRequestState::<TYPES, I>::create_from(handle).await;

    let task = Task::new(
        state,
        handle.internal_event_stream.0.clone(),
        handle.internal_event_stream.1.activate_cloned(),
    );
    handle.consensus_registry.run_task(task);
}

/// Add a task which responds to requests on the network.
pub fn add_response_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    handle: &mut SystemContextHandle<TYPES, I>,
    request_receiver: RequestReceiver,
) {
    let state = NetworkResponseState::<TYPES>::new(
        handle.hotshot.consensus(),
        request_receiver,
        handle.hotshot.memberships.quorum_membership.clone().into(),
        handle.public_key().clone(),
        handle.private_key().clone(),
        handle.hotshot.id,
    );
    handle.network_registry.register(run_response_task::<TYPES>(
        state,
        handle.internal_event_stream.1.activate_cloned(),
    ));
}

/// Add the network task to handle messages and publish events.
pub fn add_network_message_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
    channel: &Arc<NET>,
) {
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        internal_event_stream: handle.internal_event_stream.0.clone(),
        external_event_stream: handle.output_event_stream.0.clone(),
    };

    let decided_upgrade_certificate = Arc::clone(&handle.hotshot.decided_upgrade_certificate);

    let network = Arc::clone(channel);
    let mut state = network_state.clone();
    let task_handle = async_spawn(async move {
        loop {
            let decided_upgrade_certificate_lock = decided_upgrade_certificate.read().await.clone();
            let msgs = match network.recv_msgs().await {
                Ok(msgs) => {
                    let mut deserialized_messages = Vec::new();

                    for msg in msgs {
                        let deserialized_message = match VersionedMessage::deserialize(
                            &msg,
                            &decided_upgrade_certificate_lock,
                        ) {
                            Ok(deserialized) => deserialized,
                            Err(e) => {
                                tracing::error!("Failed to deserialize message: {}", e);
                                continue;
                            }
                        };

                        deserialized_messages.push(deserialized_message);
                    }

                    Messages(deserialized_messages)
                }
                Err(err) => {
                    tracing::error!("failed to receive messages: {err}");

                    // return zero messages so we sleep and try again
                    Messages(vec![])
                }
            };
            if msgs.0.is_empty() {
                // TODO: Stop sleeping here: https://github.com/EspressoSystems/HotShot/issues/2558
                async_sleep(Duration::from_millis(100)).await;
            } else {
                state.handle_messages(msgs.0).await;
            }
        }
    });
    handle.network_registry.register(task_handle);
}

/// Add the network task to handle events and send messages.
pub fn add_network_event_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
    channel: Arc<NET>,
    membership: TYPES::Membership,
    filter: fn(&Arc<HotShotEvent<TYPES>>) -> bool,
) {
    let network_state: NetworkEventTaskState<_, _, _> = NetworkEventTaskState {
        channel,
        view: TYPES::Time::genesis(),
        membership,
        filter,
        storage: Arc::clone(&handle.storage()),
        decided_upgrade_certificate: None,
    };
    let task = Task::new(
        network_state,
        handle.internal_event_stream.0.clone(),
        handle.internal_event_stream.1.activate_cloned(),
    );
    handle.consensus_registry.run_task(task);
}

/// Adds consensus-related tasks to a `SystemContextHandle`.
pub async fn add_consensus_tasks<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    handle: &mut SystemContextHandle<TYPES, I>,
) {
    handle.add_task(ViewSyncTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(VidTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(DaTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(TransactionTaskState::<TYPES, I>::create_from(handle).await);

    // only spawn the upgrade task if we are actually configured to perform an upgrade.
    if TYPES::Base::VERSION < TYPES::Upgrade::VERSION {
        handle.add_task(UpgradeTaskState::<TYPES, I>::create_from(handle).await);
    }

    {
        #![cfg(not(feature = "dependency-tasks"))]
        use hotshot_task_impls::consensus::ConsensusTaskState;

        handle.add_task(ConsensusTaskState::<TYPES, I>::create_from(handle).await);
    }
    {
        #![cfg(feature = "dependency-tasks")]
        use hotshot_task_impls::{
            consensus2::Consensus2TaskState, quorum_proposal::QuorumProposalTaskState,
            quorum_proposal_recv::QuorumProposalRecvTaskState, quorum_vote::QuorumVoteTaskState,
        };

        handle.add_task(QuorumProposalTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(QuorumVoteTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(QuorumProposalRecvTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(Consensus2TaskState::<TYPES, I>::create_from(handle).await);
    }

    #[cfg(feature = "rewind")]
    handle.add_task(RewindTaskState::<TYPES>::create_from(&handle).await);
}

#[async_trait]
/// Trait for intercepting and modifying messages between the network and consensus layers.
///
/// Consensus <-> [Byzantine logic layer] <-> Network
pub trait EventTransformerState<TYPES: NodeType, I: NodeImplementation<TYPES>>
where
    Self: std::fmt::Debug + Send + Sync + 'static,
{
    /// modify incoming messages from the network
    async fn recv_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>>;

    /// modify outgoing messages from the network
    async fn send_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>>;

    #[allow(clippy::too_many_arguments)]
    /// Creates a `SystemContextHandle` with the given even transformer
    async fn spawn_handle(
        &'static mut self,
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        nonce: u64,
        config: HotShotConfig<TYPES::SignatureKey>,
        memberships: Memberships<TYPES>,
        network: Arc<I::Network>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
        storage: I::Storage,
        auction_results_provider: I::AuctionResultsProvider,
    ) -> SystemContextHandle<TYPES, I> {
        let hotshot = SystemContext::new(
            public_key,
            private_key,
            nonce,
            config,
            memberships,
            network,
            initializer,
            metrics,
            storage,
            auction_results_provider,
        );
        let consensus_registry = ConsensusTaskRegistry::new();
        let network_registry = NetworkTaskRegistry::new();

        let output_event_stream = hotshot.external_event_stream.clone();
        let internal_event_stream = hotshot.internal_event_stream.clone();

        let mut handle = SystemContextHandle {
            consensus_registry,
            network_registry,
            output_event_stream: output_event_stream.clone(),
            internal_event_stream: internal_event_stream.clone(),
            hotshot: Arc::clone(&hotshot),
            storage: Arc::clone(&hotshot.storage),
            network: Arc::clone(&hotshot.network),
            memberships: Arc::clone(&hotshot.memberships),
        };

        add_consensus_tasks::<TYPES, I>(&mut handle).await;
        self.add_network_tasks(&mut handle).await;

        handle
    }

    /// Add byzantine network tasks with the trait
    async fn add_network_tasks(&'static mut self, handle: &mut SystemContextHandle<TYPES, I>) {
        let state_in = Arc::new(RwLock::new(self));
        let state_out = Arc::clone(&state_in);
        // channels between the task spawned in this function and the network tasks.
        // with this, we can control exactly what events the network tasks see.

        // channel to the network task
        let (sender_to_network, network_task_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        // channel from the network task
        let (network_task_sender, mut receiver_from_network) = broadcast(EVENT_CHANNEL_SIZE);
        // create a copy of the original receiver
        let (original_sender, mut original_receiver) = (
            handle.internal_event_stream.0.clone(),
            handle.internal_event_stream.1.activate_cloned(),
        );

        // replace the internal event stream with the one we just created,
        // so that the network tasks are spawned with our channel.
        let mut internal_event_stream = (
            network_task_sender.clone(),
            network_task_receiver.clone().deactivate(),
        );
        std::mem::swap(
            &mut internal_event_stream,
            &mut handle.internal_event_stream,
        );

        // spawn the network tasks with our newly-created channel
        add_network_tasks::<TYPES, I>(handle).await;

        std::mem::swap(
            &mut internal_event_stream,
            &mut handle.internal_event_stream,
        );

        // spawn a task to listen on the (original) internal event stream,
        // and broadcast the transformed events to the replacement event stream we just created.
        let send_handle = async_spawn(async move {
            loop {
                if let Ok(msg) = original_receiver.recv().await {
                    let mut state = state_out.write().await;

                    let mut results = state.send_handler(&msg).await;

                    results.reverse();

                    while let Some(event) = results.pop() {
                        let _ = sender_to_network.broadcast(event.into()).await;
                    }
                }
            }
        });

        // spawn a task to listen on the newly created event stream,
        // and broadcast the transformed events to the original internal event stream
        let recv_handle = async_spawn(async move {
            loop {
                if let Ok(msg) = receiver_from_network.recv().await {
                    let mut state = state_in.write().await;

                    let mut results = state.recv_handler(&msg).await;

                    results.reverse();

                    while let Some(event) = results.pop() {
                        let _ = original_sender.broadcast(event.into()).await;
                    }
                }
            }
        });

        handle.network_registry.register(send_handle);
        handle.network_registry.register(recv_handle);
    }
}

#[derive(Debug)]
/// An `EventTransformerState` that multiplies `QuorumProposalSend` events, incrementing the view number of the proposal
pub struct BadProposalViewDos {
    /// The number of times to duplicate a `QuorumProposalSend` event
    pub multiplier: u64,
    /// The view number increment each time it's duplicated
    pub increment: u64,
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> EventTransformerState<TYPES, I>
    for BadProposalViewDos
{
    async fn recv_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        vec![event.clone()]
    }

    async fn send_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        match event {
            HotShotEvent::QuorumProposalSend(proposal, signature) => {
                let mut result = Vec::new();

                for n in 0..self.multiplier {
                    let mut modified_proposal = proposal.clone();

                    modified_proposal.data.view_number += n * self.increment;

                    result.push(HotShotEvent::QuorumProposalSend(
                        modified_proposal,
                        signature.clone(),
                    ));
                }

                result
            }
            _ => vec![event.clone()],
        }
    }
}

#[derive(Debug)]
/// An `EventHandlerState` that doubles the `QuorumVoteSend` and `QuorumProposalSend` events
pub struct DoubleProposeVote;

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> EventTransformerState<TYPES, I>
    for DoubleProposeVote
{
    async fn recv_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        vec![event.clone()]
    }

    async fn send_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        match event {
            HotShotEvent::QuorumProposalSend(_, _) | HotShotEvent::QuorumVoteSend(_) => {
                vec![event.clone(), event.clone()]
            }
            _ => vec![event.clone()],
        }
    }
}

/// adds tasks for sending/receiving messages to/from the network.
pub async fn add_network_tasks<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    handle: &mut SystemContextHandle<TYPES, I>,
) {
    let network = Arc::clone(&handle.network);
    let quorum_membership = handle.memberships.quorum_membership.clone();
    let da_membership = handle.memberships.da_membership.clone();
    let vid_membership = handle.memberships.vid_membership.clone();
    let view_sync_membership = handle.memberships.view_sync_membership.clone();

    add_network_message_task(handle, &network);
    add_network_message_task(handle, &network);

    if let Some(request_receiver) = network.spawn_request_receiver_task().await {
        add_request_network_task(handle).await;
        add_response_task(handle, request_receiver);
    }

    add_network_event_task(
        handle,
        Arc::clone(&network),
        quorum_membership.clone(),
        network::quorum_filter,
    );
    add_network_event_task(
        handle,
        Arc::clone(&network),
        quorum_membership,
        network::upgrade_filter,
    );
    add_network_event_task(
        handle,
        Arc::clone(&network),
        da_membership,
        network::da_filter,
    );
    add_network_event_task(
        handle,
        Arc::clone(&network),
        view_sync_membership,
        network::view_sync_filter,
    );
    add_network_event_task(
        handle,
        Arc::clone(&network),
        vid_membership,
        network::vid_filter,
    );
}
