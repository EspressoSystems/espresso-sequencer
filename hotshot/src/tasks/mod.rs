//! Provides a number of tasks that run continuously

/// Provides trait to create task states from a `SystemContextHandle`
pub mod task_state;

use std::{sync::Arc, time::Duration};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use hotshot_task::task::Task;
use hotshot_task_impls::{
    consensus::ConsensusTaskState,
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

#[cfg(feature = "dependency-tasks")]
use hotshot_task_impls::{
    consensus2::Consensus2TaskState, quorum_proposal::QuorumProposalTaskState,
    quorum_proposal_recv::QuorumProposalRecvTaskState, quorum_vote::QuorumVoteTaskState,
};

use hotshot_types::{
    constants::{Version01, VERSION_0_1},
    message::{Message, Messages},
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
    let state = NetworkRequestState::<TYPES, I, Version01>::create_from(handle).await;

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
    request_receiver: RequestReceiver<TYPES>,
) {
    let state = NetworkResponseState::<TYPES>::new(
        handle.hotshot.consensus(),
        request_receiver,
        handle.hotshot.memberships.quorum_membership.clone().into(),
        handle.public_key().clone(),
        handle.private_key().clone(),
    );
    handle
        .network_registry
        .register(run_response_task::<TYPES, Version01>(
            state,
            handle.internal_event_stream.1.activate_cloned(),
        ));
}
/// Add the network task to handle messages and publish events.
pub async fn add_network_message_task<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    NET: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
    channel: Arc<NET>,
) {
    let net = Arc::clone(&channel);
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        event_stream: handle.internal_event_stream.0.clone(),
    };

    let network = Arc::clone(&net);
    let mut state = network_state.clone();
    let task_handle = async_spawn(async move {
        loop {
            let msgs = match network.recv_msgs().await {
                Ok(msgs) => Messages(msgs),
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
    NET: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
    channel: Arc<NET>,
    membership: TYPES::Membership,
    filter: fn(&Arc<HotShotEvent<TYPES>>) -> bool,
) {
    let network_state: NetworkEventTaskState<_, _, _> = NetworkEventTaskState {
        channel,
        view: TYPES::Time::genesis(),
        version: VERSION_0_1,
        membership,
        filter,
        storage: Arc::clone(&handle.storage()),
    };
    let task = Task::new(
        network_state,
        handle.internal_event_stream.0.clone(),
        handle.internal_event_stream.1.activate_cloned(),
    );
    handle.consensus_registry.run_task(task);
}

/// Adds consensus-related tasks to a `SystemContextHandle`.
pub async fn add_consensus_tasks<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    VERSION: StaticVersionType + 'static,
>(
    handle: &mut SystemContextHandle<TYPES, I>,
) {
    handle.add_task(ViewSyncTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(VidTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(DaTaskState::<TYPES, I>::create_from(handle).await);
    handle.add_task(TransactionTaskState::<TYPES, I, VERSION>::create_from(handle).await);
    handle.add_task(UpgradeTaskState::<TYPES, I>::create_from(handle).await);
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
}
