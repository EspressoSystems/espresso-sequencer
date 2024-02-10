//! Provides a number of tasks that run continuously

use crate::{types::SystemContextHandle, HotShotConsensusApi};
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};

use hotshot_task::task::{Task, TaskRegistry};
use hotshot_task_impls::{
    consensus::{CommitmentAndMetadata, ConsensusTaskState},
    da::DATaskState,
    events::HotShotEvent,
    network::{NetworkEventTaskState, NetworkMessageTaskState},
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VIDTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::traits::election::Membership;
use hotshot_types::{
    event::Event,
    message::Messages,
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        network::{CommunicationChannel, ConsensusIntentEvent, TransmitType},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        BlockPayload,
    },
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
    time::Duration,
};
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
pub async fn add_network_message_task<TYPES: NodeType, NET: CommunicationChannel<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    event_stream: Sender<HotShotEvent<TYPES>>,
    channel: NET,
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
    let broadcast_handle = async_spawn(async move {
        loop {
            let msgs = match network.recv_msgs(TransmitType::Broadcast).await {
                Ok(msgs) => Messages(msgs),
                Err(err) => {
                    error!("failed to receive broadcast messages: {err}");

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
    let network = net.clone();
    let mut state = network_state.clone();
    let direct_handle = async_spawn(async move {
        loop {
            let msgs = match network.recv_msgs(TransmitType::Direct).await {
                Ok(msgs) => Messages(msgs),
                Err(err) => {
                    error!("failed to receive direct messages: {err}");

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
    task_reg.register(direct_handle).await;
    task_reg.register(broadcast_handle).await;
}

/// Add the network task to handle events and send messages.
pub async fn add_network_event_task<TYPES: NodeType, NET: CommunicationChannel<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    channel: NET,
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

/// Create the consensus task state
/// # Panics
/// If genesis payload can't be encoded.  This should not be possible
pub async fn create_consensus_state<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    output_stream: Sender<Event<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) -> ConsensusTaskState<TYPES, I, HotShotConsensusApi<TYPES, I>> {
    let consensus = handle.hotshot.get_consensus();
    let c_api: HotShotConsensusApi<TYPES, I> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };

    let (payload, metadata) = <TYPES::BlockPayload as BlockPayload>::genesis();
    // Impossible for `unwrap` to fail on the genesis payload.
    let payload_commitment = vid_commitment(
        &payload.encode().unwrap().collect(),
        handle
            .hotshot
            .inner
            .memberships
            .quorum_membership
            .total_nodes(),
    );
    // build the consensus task
    let consensus_state = ConsensusTaskState {
        consensus,
        timeout: handle.hotshot.inner.config.next_view_timeout,
        cur_view: TYPES::Time::new(0),
        payload_commitment_and_metadata: Some(CommitmentAndMetadata {
            commitment: payload_commitment,
            metadata,
            is_genesis: true,
        }),
        api: c_api.clone(),
        _pd: PhantomData,
        vote_collector: None.into(),
        timeout_vote_collector: None.into(),
        timeout_task: None,
        timeout_cert: None,
        output_event_stream: output_stream,
        vid_shares: HashMap::new(),
        current_proposal: None,
        id: handle.hotshot.inner.id,
        public_key: c_api.public_key().clone(),
        private_key: c_api.private_key().clone(),
        quorum_network: c_api.inner.networks.quorum_network.clone().into(),
        committee_network: c_api.inner.networks.da_network.clone().into(),
        timeout_membership: c_api.inner.memberships.quorum_membership.clone().into(),
        quorum_membership: c_api.inner.memberships.quorum_membership.clone().into(),
        committee_membership: c_api.inner.memberships.da_membership.clone().into(),
    };
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
        .get_committee(<TYPES as NodeType>::Time::new(0))
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
    consensus_state
}

/// add the consensus task
pub async fn add_consensus_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let state =
        create_consensus_state(handle.hotshot.inner.output_event_stream.0.clone(), handle).await;
    let task = Task::new(tx, rx, task_reg.clone(), state);
    task_reg.run_task(task).await;
}

/// add the VID task
pub async fn add_vid_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    // build the vid task
    let c_api: HotShotConsensusApi<TYPES, I> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let vid_state = VIDTaskState {
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        cur_view: TYPES::Time::new(0),
        vote_collector: None,
        network: c_api.inner.networks.quorum_network.clone().into(),
        membership: c_api.inner.memberships.vid_membership.clone().into(),
        public_key: c_api.public_key().clone(),
        private_key: c_api.private_key().clone(),
        id: handle.hotshot.inner.id,
    };

    let task = Task::new(tx, rx, task_reg.clone(), vid_state);
    task_reg.run_task(task).await;
}

/// add the Upgrade task.
///
/// # Panics
///
/// Uses .`unwrap()`, though this should never panic.
pub async fn add_upgrade_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    task_reg: Arc<TaskRegistry>,
    tx: Sender<HotShotEvent<TYPES>>,
    rx: Receiver<HotShotEvent<TYPES>>,
    handle: &SystemContextHandle<TYPES, I>,
) {
    let c_api: HotShotConsensusApi<TYPES, I> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let upgrade_state = UpgradeTaskState {
        api: c_api.clone(),
        cur_view: TYPES::Time::new(0),
        quorum_membership: c_api.inner.memberships.quorum_membership.clone().into(),
        quorum_network: c_api.inner.networks.quorum_network.clone().into(),
        should_vote: |_upgrade_proposal| false,
        vote_collector: None.into(),
        public_key: c_api.public_key().clone(),
        private_key: c_api.private_key().clone(),
        id: handle.hotshot.inner.id,
    };
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
    let c_api: HotShotConsensusApi<TYPES, I> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let da_state = DATaskState {
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        da_membership: c_api.inner.memberships.da_membership.clone().into(),
        da_network: c_api.inner.networks.da_network.clone().into(),
        quorum_membership: c_api.inner.memberships.quorum_membership.clone().into(),
        cur_view: TYPES::Time::new(0),
        vote_collector: None.into(),
        public_key: c_api.public_key().clone(),
        private_key: c_api.private_key().clone(),
        id: handle.hotshot.inner.id,
    };

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
    // build the transactions task
    let c_api: HotShotConsensusApi<TYPES, I> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let transactions_state = TransactionTaskState {
        api: c_api.clone(),
        consensus: handle.hotshot.get_consensus(),
        transactions: Arc::default(),
        seen_transactions: HashSet::new(),
        cur_view: TYPES::Time::new(0),
        network: c_api.inner.networks.quorum_network.clone().into(),
        membership: c_api.inner.memberships.quorum_membership.clone().into(),
        public_key: c_api.public_key().clone(),
        private_key: c_api.private_key().clone(),
        id: handle.hotshot.inner.id,
    };

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
    let api = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    // build the view sync task
    let view_sync_state = ViewSyncTaskState {
        current_view: TYPES::Time::new(0),
        next_view: TYPES::Time::new(0),
        network: api.inner.networks.quorum_network.clone().into(),
        membership: api.inner.memberships.view_sync_membership.clone().into(),
        public_key: api.public_key().clone(),
        private_key: api.private_key().clone(),
        api,
        num_timeouts_tracked: 0,
        replica_task_map: HashMap::default().into(),
        pre_commit_relay_map: HashMap::default().into(),
        commit_relay_map: HashMap::default().into(),
        finalize_relay_map: HashMap::default().into(),
        view_sync_timeout: Duration::new(10, 0),
        id: handle.hotshot.inner.id,
        last_garbage_collected_view: TYPES::Time::new(0),
    };

    let task = Task::new(tx, rx, task_reg.clone(), view_sync_state);
    task_reg.run_task(task).await;
}
