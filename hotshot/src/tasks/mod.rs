// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Provides a number of tasks that run continuously

/// Provides trait to create task states from a `SystemContextHandle`
pub mod task_state;
use std::{collections::HashSet, sync::Arc, time::Duration};

use async_broadcast::broadcast;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use async_trait::async_trait;
use futures::{
    future::{BoxFuture, FutureExt},
    stream, StreamExt,
};
use hotshot_task::task::{NetworkHandle, Task};
#[cfg(feature = "rewind")]
use hotshot_task_impls::rewind::RewindTaskState;
use hotshot_task_impls::{
    da::DaTaskState,
    events::HotShotEvent,
    health_check::HealthCheckTaskState,
    helpers::broadcast_event,
    network::{self, NetworkEventTaskState, NetworkMessageTaskState},
    request::NetworkRequestState,
    response::{run_response_task, NetworkResponseState},
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VidTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::{
    constants::EVENT_CHANNEL_SIZE,
    data::QuorumProposal,
    message::{Messages, Proposal},
    request_response::RequestReceiver,
    traits::{
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
};
use vbs::version::StaticVersionType;

use crate::{
    tasks::task_state::CreateTaskState, types::SystemContextHandle, ConsensusApi,
    ConsensusMetricsValue, ConsensusTaskRegistry, HotShotConfig, HotShotInitializer,
    MarketplaceConfig, Memberships, NetworkTaskRegistry, SignatureKey, SystemContext, Versions,
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
pub async fn add_request_network_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
) {
    handle.add_task(NetworkRequestState::<TYPES, I>::create_from(handle).await);
}

/// Add a task which responds to requests on the network.
pub fn add_response_task<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
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
    let task_name = state.get_task_name();
    handle.network_registry.register(run_response_task::<TYPES>(
        state,
        handle.internal_event_stream.0.clone(),
        handle.internal_event_stream.1.activate_cloned(),
        handle.generate_task_id(task_name),
    ));
}

/// Add the network task to handle messages and publish events.
pub fn add_network_message_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
    V: Versions,
>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
    channel: &Arc<NET>,
) {
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        internal_event_stream: handle.internal_event_stream.0.clone(),
        external_event_stream: handle.output_event_stream.0.clone(),
    };

    let upgrade_lock = handle.hotshot.upgrade_lock.clone();

    let network = Arc::clone(channel);
    let mut state = network_state.clone();
    let shutdown_signal = create_shutdown_event_monitor(handle).fuse();
    let stream = handle.internal_event_stream.0.clone();
    let task_id = handle.generate_task_id(network_state.get_task_name());
    let handle_task_id = task_id.clone();
    let task_handle = async_spawn(async move {
        let recv_stream = stream::unfold((), |()| async {
            let msgs = match network.recv_msgs().await {
                Ok(msgs) => {
                    let mut deserialized_messages = Vec::new();
                    for msg in msgs {
                        let deserialized_message = match upgrade_lock.deserialize(&msg).await {
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
                    Messages(vec![])
                }
            };
            Some((msgs, ()))
        });

        let heartbeat_interval =
            Task::<HealthCheckTaskState<TYPES>>::get_periodic_interval_in_secs();
        let fused_recv_stream = recv_stream.boxed().fuse();
        futures::pin_mut!(fused_recv_stream, heartbeat_interval, shutdown_signal);
        loop {
            futures::select! {
                () = shutdown_signal => {
                    tracing::error!("Shutting down network message task");
                    return;
                }
                msgs_option = fused_recv_stream.next() => {
                    if let Some(msgs) = msgs_option {
                        if msgs.0.is_empty() {
                            // TODO: Stop sleeping here: https://github.com/EspressoSystems/HotShot/issues/2558
                            async_sleep(Duration::from_millis(100)).await;
                        } else {
                            state.handle_messages(msgs.0).await;
                        }
                    } else {
                        // Stream has ended, which shouldn't happen in this case.
                        // You might want to handle this situation, perhaps by breaking the loop or logging an error.
                        tracing::error!("Network message stream unexpectedly ended");
                        return;
                    }
                }
                _ = Task::<HealthCheckTaskState<TYPES>>::handle_periodic_delay(&mut heartbeat_interval) => {
                    broadcast_event(Arc::new(HotShotEvent::HeartBeat(handle_task_id.clone())), &stream).await;
                }
            }
        }
    });
    handle.network_registry.register(NetworkHandle {
        handle: task_handle,
        task_id,
    });
}

/// Add the network task to handle events and send messages.
pub fn add_network_event_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
    channel: Arc<NET>,
    membership: TYPES::Membership,
    filter: fn(&Arc<HotShotEvent<TYPES>>) -> bool,
) {
    let network_state: NetworkEventTaskState<_, V, _, _> = NetworkEventTaskState {
        channel,
        view: TYPES::Time::genesis(),
        membership,
        filter,
        storage: Arc::clone(&handle.storage()),
        upgrade_lock: handle.hotshot.upgrade_lock.clone(),
    };
    handle.add_task(network_state);
}

/// Adds consensus-related tasks to a `SystemContextHandle`.
pub async fn add_consensus_tasks<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
) {
    handle.add_task(ViewSyncTaskState::<TYPES, I, V>::create_from(handle).await);
    handle.add_task(VidTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(DaTaskState::<TYPES, I, V>::create_from(handle).await);
    handle.add_task(TransactionTaskState::<TYPES, I, V>::create_from(handle).await);

    // only spawn the upgrade task if we are actually configured to perform an upgrade.
    if V::Base::VERSION < V::Upgrade::VERSION {
        handle.add_task(UpgradeTaskState::<TYPES, I, V>::create_from(handle).await);
    }

    {
        #![cfg(not(feature = "dependency-tasks"))]
        use hotshot_task_impls::consensus::ConsensusTaskState;

        handle.add_task(ConsensusTaskState::<TYPES, I, V>::create_from(handle).await);
    }
    {
        #![cfg(feature = "dependency-tasks")]
        use hotshot_task_impls::{
            consensus2::Consensus2TaskState, quorum_proposal::QuorumProposalTaskState,
            quorum_proposal_recv::QuorumProposalRecvTaskState, quorum_vote::QuorumVoteTaskState,
        };

        handle.add_task(QuorumProposalTaskState::<TYPES, I, V>::create_from(handle).await);
        handle.add_task(QuorumVoteTaskState::<TYPES, I, V>::create_from(handle).await);
        handle.add_task(QuorumProposalRecvTaskState::<TYPES, I, V>::create_from(handle).await);
        handle.add_task(Consensus2TaskState::<TYPES, I, V>::create_from(handle).await);
    }

    #[cfg(feature = "rewind")]
    handle.add_task(RewindTaskState::<TYPES>::create_from(&handle).await);
}

/// Creates a monitor for shutdown events.
///
/// # Returns
/// A `BoxFuture<'static, ()>` that resolves when a `HotShotEvent::Shutdown` is detected.
///
/// # Usage
/// Use in `select!` macros or similar constructs for graceful shutdowns:
#[must_use]
pub fn create_shutdown_event_monitor<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    handle: &SystemContextHandle<TYPES, I, V>,
) -> BoxFuture<'static, ()> {
    // Activate the cloned internal event stream
    let mut event_stream = handle.internal_event_stream.1.activate_cloned();

    // Create a future that completes when the `HotShotEvent::Shutdown` is received
    async move {
        loop {
            match event_stream.recv_direct().await {
                Ok(event) => {
                    if matches!(event.as_ref(), HotShotEvent::Shutdown) {
                        return;
                    }
                }
                Err(e) => {
                    tracing::error!("Shutdown event monitor channel recv error: {}", e);
                }
            }
        }
    }
    .boxed()
}

#[async_trait]
/// Trait for intercepting and modifying messages between the network and consensus layers.
///
/// Consensus <-> [Byzantine logic layer] <-> Network
pub trait EventTransformerState<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>
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
        marketplace_config: MarketplaceConfig<TYPES, I>,
    ) -> SystemContextHandle<TYPES, I, V> {
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
            marketplace_config,
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

        add_consensus_tasks::<TYPES, I, V>(&mut handle).await;
        self.add_network_tasks(&mut handle).await;
        add_health_check_task(&mut handle).await;

        handle
    }

    /// Add byzantine network tasks with the trait
    #[allow(clippy::too_many_lines)]
    async fn add_network_tasks(&'static mut self, handle: &mut SystemContextHandle<TYPES, I, V>) {
        let task_id = self.get_task_name();
        let state_in = Arc::new(RwLock::new(self));
        let state_out = Arc::clone(&state_in);
        // channels between the task spawned in this function and the network tasks.
        // with this, we can control exactly what events the network tasks see.

        // channel to the network task
        let (sender_to_network, network_task_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        // channel from the network task
        let (network_task_sender, receiver_from_network) = broadcast(EVENT_CHANNEL_SIZE);
        // create a copy of the original receiver
        let (original_sender, original_receiver) = (
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
        add_network_tasks::<TYPES, I, V>(handle).await;

        std::mem::swap(
            &mut internal_event_stream,
            &mut handle.internal_event_stream,
        );

        // spawn a task to listen on the (original) internal event stream,
        // and broadcast the transformed events to the replacement event stream we just created.
        let shutdown_signal = create_shutdown_event_monitor(handle).fuse();
        let send_handle = async_spawn(async move {
            let recv_stream = stream::unfold(original_receiver, |mut recv| async move {
                match recv.recv().await {
                    Ok(event) => Some((Ok(event), recv)),
                    Err(async_broadcast::RecvError::Closed) => None,
                    Err(e) => Some((Err(e), recv)),
                }
            })
            .boxed();

            let fused_recv_stream = recv_stream.fuse();
            futures::pin_mut!(fused_recv_stream, shutdown_signal);

            loop {
                futures::select! {
                    () = shutdown_signal => {
                        tracing::error!("Shutting down relay send task");
                        let _ = sender_to_network.broadcast(HotShotEvent::<TYPES>::Shutdown.into()).await;
                        return;
                    }
                    event = fused_recv_stream.next() => {
                        match event {
                            Some(Ok(msg)) => {
                                let mut state = state_out.write().await;
                                let mut results = state.send_handler(&msg).await;
                                results.reverse();
                                while let Some(event) = results.pop() {
                                    let _ = sender_to_network.broadcast(event.into()).await;
                                }
                            }
                            Some(Err(e)) => {
                                tracing::error!("Relay Task, send_handle, Error receiving event: {:?}", e);
                            }
                            None => {
                                tracing::info!("Relay Task, send_handle, Event stream closed");
                                return;
                            }
                        }
                    }
                }
            }
        });

        // spawn a task to listen on the newly created event stream,
        // and broadcast the transformed events to the original internal event stream
        let shutdown_signal = create_shutdown_event_monitor(handle).fuse();
        let recv_handle = async_spawn(async move {
            let network_recv_stream =
                stream::unfold(receiver_from_network, |mut recv| async move {
                    match recv.recv().await {
                        Ok(event) => Some((Ok(event), recv)),
                        Err(async_broadcast::RecvError::Closed) => None,
                        Err(e) => Some((Err(e), recv)),
                    }
                });

            let fused_network_recv_stream = network_recv_stream.boxed().fuse();
            futures::pin_mut!(fused_network_recv_stream, shutdown_signal);

            loop {
                futures::select! {
                    () = shutdown_signal => {
                        tracing::error!("Shutting down relay receive task");
                        return;
                    }
                    event = fused_network_recv_stream.next() => {
                        match event {
                            Some(Ok(msg)) => {
                                let mut state = state_in.write().await;
                                let mut results = state.recv_handler(&msg).await;
                                results.reverse();
                                while let Some(event) = results.pop() {
                                    let _ = original_sender.broadcast(event.into()).await;
                                }
                            }
                            Some(Err(e)) => {
                                tracing::error!("Relay Task, recv_handle, Error receiving event from network: {:?}", e);
                            }
                            None => {
                                tracing::info!("Relay Task, recv_handle, Network event stream closed");
                                return;
                            }
                        }
                    }
                }
            }
        });

        handle.network_registry.register(NetworkHandle {
            handle: send_handle,
            task_id: handle.generate_task_id(task_id),
        });
        handle.network_registry.register(NetworkHandle {
            handle: recv_handle,
            task_id: handle.generate_task_id(task_id),
        });
    }

    /// Gets the name of the current task
    fn get_task_name(&self) -> &'static str {
        std::any::type_name::<dyn EventTransformerState<TYPES, I, V>>()
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
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> EventTransformerState<TYPES, I, V>
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
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> EventTransformerState<TYPES, I, V>
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

#[derive(Debug)]
/// An `EventHandlerState` that modifies justify_qc on `QuorumProposalSend` to that of a previous view to mock dishonest leader
pub struct DishonestLeader<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Store events from previous views
    pub validated_proposals: Vec<QuorumProposal<TYPES>>,
    /// How many times current node has been elected leader and sent proposal
    pub total_proposals_from_node: u64,
    /// Which proposals to be dishonest at
    pub dishonest_at_proposal_numbers: HashSet<u64>,
    /// How far back to look for a QC
    pub view_look_back: usize,
    /// Phantom
    pub _phantom: std::marker::PhantomData<I>,
}

/// Add method that will handle `QuorumProposalSend` events
/// If we have previous proposals stored and the total_proposals_from_node matches a value specified in dishonest_at_proposal_numbers
/// Then send out the event with the modified proposal that has an older QC
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> DishonestLeader<TYPES, I> {
    /// When a leader is sending a proposal this method will mock a dishonest leader
    /// We accomplish this by looking back a number of specified views and using that cached proposals QC
    fn handle_proposal_send_event(
        &self,
        event: &HotShotEvent<TYPES>,
        proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
        sender: &TYPES::SignatureKey,
    ) -> HotShotEvent<TYPES> {
        let length = self.validated_proposals.len();
        if !self
            .dishonest_at_proposal_numbers
            .contains(&self.total_proposals_from_node)
            || length == 0
        {
            return event.clone();
        }

        // Grab proposal from specified view look back
        let proposal_from_look_back = if length - 1 < self.view_look_back {
            // If look back is too far just take the first proposal
            self.validated_proposals[0].clone()
        } else {
            let index = (self.validated_proposals.len() - 1) - self.view_look_back;
            self.validated_proposals[index].clone()
        };

        // Create a dishonest proposal by using the old proposals qc
        let mut dishonest_proposal = proposal.clone();
        dishonest_proposal.data.justify_qc = proposal_from_look_back.justify_qc;

        HotShotEvent::QuorumProposalSend(dishonest_proposal, sender.clone())
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES> + std::fmt::Debug, V: Versions>
    EventTransformerState<TYPES, I, V> for DishonestLeader<TYPES, I>
{
    async fn recv_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        vec![event.clone()]
    }

    async fn send_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        match event {
            HotShotEvent::QuorumProposalSend(proposal, sender) => {
                self.total_proposals_from_node += 1;
                return vec![self.handle_proposal_send_event(event, proposal, sender)];
            }
            HotShotEvent::QuorumProposalValidated(proposal, _) => {
                self.validated_proposals.push(proposal.clone());
            }
            _ => {}
        }
        vec![event.clone()]
    }
}

#[derive(Debug)]
/// An `EventHandlerState` that modifies view number on the certificate of `DacSend` event to that of a future view
pub struct DishonestDa {
    /// How many times current node has been elected leader and sent Da Cert
    pub total_da_certs_sent_from_node: u64,
    /// Which proposals to be dishonest at
    pub dishonest_at_da_cert_sent_numbers: HashSet<u64>,
    /// When leader how many times we will send DacSend and increment view number
    pub total_views_add_to_cert: u64,
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES> + std::fmt::Debug, V: Versions>
    EventTransformerState<TYPES, I, V> for DishonestDa
{
    async fn recv_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        vec![event.clone()]
    }

    async fn send_handler(&mut self, event: &HotShotEvent<TYPES>) -> Vec<HotShotEvent<TYPES>> {
        if let HotShotEvent::DacSend(cert, sender) = event {
            self.total_da_certs_sent_from_node += 1;
            if self
                .dishonest_at_da_cert_sent_numbers
                .contains(&self.total_da_certs_sent_from_node)
            {
                let mut result = vec![HotShotEvent::DacSend(cert.clone(), sender.clone())];
                for i in 1..=self.total_views_add_to_cert {
                    let mut bad_cert = cert.clone();
                    bad_cert.view_number = cert.view_number + i;
                    result.push(HotShotEvent::DacSend(bad_cert, sender.clone()));
                }
                return result;
            }
        }
        vec![event.clone()]
    }
}

/// adds tasks for sending/receiving messages to/from the network.
pub async fn add_network_tasks<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
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

/// Add the health check task
pub async fn add_health_check_task<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions>(
    handle: &mut SystemContextHandle<TYPES, I, V>,
) {
    handle.add_task(HealthCheckTaskState::<TYPES>::create_from(handle).await);
}
