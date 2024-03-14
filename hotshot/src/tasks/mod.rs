//! Provides a number of tasks that run continuously

/// Provides trait to create task states from a `SystemContextHandle`
pub mod task_state;

use crate::tasks::task_state::CreateTaskState;
use crate::ConsensusApi;

use crate::types::SystemContextHandle;
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};

use hotshot_task::task::{Task, TaskRegistry};
use hotshot_task_impls::{
    consensus::ConsensusTaskState,
    da::DATaskState,
    events::HotShotEvent,
    network::{NetworkEventTaskState, NetworkMessageTaskState},
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VIDTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::{
    message::Message,
    traits::{election::Membership, network::ConnectedNetwork},
};
use hotshot_types::{
    message::Messages,
    traits::{
        network::ConsensusIntentEvent,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
};
use std::{sync::Arc, time::Duration};
use tracing::error;

/// event for global event stream
#[derive(Clone, Debug)]
pub enum GlobalEvent {
    /// shut everything down
    Shutdown,
    /// dummy (TODO delete later)
    Dummy,
}

/// Add the network task to handle messages and publish events.
pub async fn add_network_message_task<
    TYPES: NodeType,
    NET: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
>(
    task_reg: Arc<TaskRegistry>,
    event_stream: Sender<HotShotEvent<TYPES>>,
    channel: Arc<NET>,
) {
    let net = channel.clone();
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        event_stream: event_stream.clone(),
    };

    // TODO we don't need two async tasks for this, we should combine the
    // by getting rid of `TransmitType`
    // https://github.com/EspressoSystems/HotShot/issues/2377
    let network = net.clone();
    let mut state = network_state.clone();
    let handle = async_spawn(async move {
        loop {
            let msgs = match network.recv_msgs().await {
                Ok(msgs) => Messages(msgs),
                Err(err) => {
                    error!("failed to receive messages: {err}");

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
    task_reg.register(handle).await;
}
/// Add the network task to handle events and send messages.
pub async fn add_network_event_task<
    TYPES: NodeType,
    NET: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    channel: Arc<NET>,
    membership: TYPES::Membership,
    filter: fn(&HotShotEvent<TYPES>) -> bool,
) {
    let network_state: NetworkEventTaskState<_, _> = NetworkEventTaskState {
        channel,
        view: TYPES::Time::genesis(),
        membership,
        filter,
    };
    let task = Task::new(tx, rx, task_reg.clone(), network_state);
    task_reg.run_task(task).await;
}

/// Setup polls for the given `consensus_state`
pub async fn inject_consensus_polls<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    API: ConsensusApi<TYPES, I>,
>(
    consensus_state: &ConsensusTaskState<TYPES, I, API>,
) {
    // Poll (forever) for the latest quorum proposal
    consensus_state
        .quorum_network
        .inject_consensus_info(ConsensusIntentEvent::PollForLatestProposal)
        .await;

    // See if we're in the DA committee
    // This will not work for epochs (because dynamic subscription
    // With the Push CDN, we are _always_ polling for latest anyway.
    let is_da = consensus_state
        .committee_membership
        .get_whole_committee(<TYPES as NodeType>::Time::new(0))
        .contains(&consensus_state.public_key);

    // If we are, poll for latest DA proposal.
    if is_da {
        consensus_state
            .committee_network
            .inject_consensus_info(ConsensusIntentEvent::PollForLatestProposal)
            .await;
    }

    // Poll (forever) for the latest view sync certificate
    consensus_state
        .quorum_network
        .inject_consensus_info(ConsensusIntentEvent::PollForLatestViewSyncCertificate)
        .await;
    // Start polling for proposals for the first view
    consensus_state
        .quorum_network
        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(1))
        .await;

    consensus_state
        .quorum_network
        .inject_consensus_info(ConsensusIntentEvent::PollForDAC(1))
        .await;

    if consensus_state
        .quorum_membership
        .get_leader(TYPES::Time::new(1))
        == consensus_state.public_key
    {
        consensus_state
            .quorum_network
            .inject_consensus_info(ConsensusIntentEvent::PollForVotes(0))
            .await;
    }
}

/// add the consensus task
pub async fn add_consensus_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let consensus_state = ConsensusTaskState::create_from(handle).await;

    inject_consensus_polls(&consensus_state).await;

    let task = Task::new(tx, rx, task_reg.clone(), consensus_state);
    task_reg.run_task(task).await;
}

/// add the VID task
pub async fn add_vid_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let vid_state = VIDTaskState::create_from(handle).await;
    let task = Task::new(tx, rx, task_reg.clone(), vid_state);
    task_reg.run_task(task).await;
}

/// add the Upgrade task.
pub async fn add_upgrade_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let upgrade_state = UpgradeTaskState::create_from(handle).await;

    let task = Task::new(tx, rx, task_reg.clone(), upgrade_state);
    task_reg.run_task(task).await;
}
/// add the Data Availability task
pub async fn add_da_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    // build the da task
    let da_state = DATaskState::create_from(handle).await;

    let task = Task::new(tx, rx, task_reg.clone(), da_state);
    task_reg.run_task(task).await;
}

/// add the Transaction Handling task
pub async fn add_transaction_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let transactions_state = TransactionTaskState::create_from(handle).await;

    let task = Task::new(tx, rx, task_reg.clone(), transactions_state);
    task_reg.run_task(task).await;
}

/// add the view sync task
pub async fn add_view_sync_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let view_sync_state = ViewSyncTaskState::create_from(handle).await;

    let task = Task::new(tx, rx, task_reg.clone(), view_sync_state);
    task_reg.run_task(task).await;
}
