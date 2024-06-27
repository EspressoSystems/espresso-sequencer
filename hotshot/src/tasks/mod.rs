//! Provides a number of tasks that run continuously

/// Provides trait to create task states from a `SystemContextHandle`
pub mod task_state;

use std::{sync::Arc, time::Duration};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use hotshot_task::task::Task;
#[cfg(not(feature = "dependency-tasks"))]
use hotshot_task_impls::consensus::ConsensusTaskState;
#[cfg(feature = "rewind")]
use hotshot_task_impls::rewind::RewindTaskState;
#[cfg(feature = "dependency-tasks")]
use hotshot_task_impls::{
    consensus2::Consensus2TaskState, quorum_proposal::QuorumProposalTaskState,
    quorum_proposal_recv::QuorumProposalRecvTaskState, quorum_vote::QuorumVoteTaskState,
};
use hotshot_task_impls::{
    da::DaTaskState,
    events::HotShotEvent,
    network::{NetworkEventTaskState, NetworkMessageTaskState},
    request::NetworkRequestState,
    response::{run_response_task, NetworkResponseState, RequestReceiver},
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VidTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::{
    message::{Messages, VersionedMessage},
    traits::{
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
};
use vbs::version::StaticVersionType;

use crate::{tasks::task_state::CreateTaskState, types::SystemContextHandle, ConsensusApi};

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
pub async fn add_response_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    handle: &mut SystemContextHandle<TYPES, I>,
    request_receiver: RequestReceiver,
) {
    let state = NetworkResponseState::<TYPES>::new(
        handle.hotshot.consensus(),
        request_receiver,
        handle.hotshot.memberships.quorum_membership.clone().into(),
        handle.public_key().clone(),
        handle.private_key().clone(),
    );
    handle.network_registry.register(run_response_task::<TYPES>(
        state,
        handle.internal_event_stream.1.activate_cloned(),
    ));
}
/// Add the network task to handle messages and publish events.
pub async fn add_network_message_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
    channel: Arc<NET>,
) {
    let net = Arc::clone(&channel);
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        event_stream: handle.internal_event_stream.0.clone(),
    };

    let decided_upgrade_certificate = Arc::clone(&handle.hotshot.decided_upgrade_certificate);

    let network = Arc::clone(&net);
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
                                return;
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
pub async fn add_network_event_task<
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
        handle.add_task(ConsensusTaskState::<TYPES, I>::create_from(handle).await);
    }
    {
        #![cfg(feature = "dependency-tasks")]
        handle.add_task(QuorumProposalTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(QuorumVoteTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(QuorumProposalRecvTaskState::<TYPES, I>::create_from(handle).await);
        handle.add_task(Consensus2TaskState::<TYPES, I>::create_from(handle).await);
    }

    #[cfg(feature = "rewind")]
    handle.add_task(RewindTaskState::<TYPES>::create_from(&handle).await);
}
