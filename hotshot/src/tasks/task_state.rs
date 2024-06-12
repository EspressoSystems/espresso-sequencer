use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    sync::{atomic::AtomicBool, Arc},
};

use async_trait::async_trait;
use chrono::Utc;
#[cfg(feature = "rewind")]
use hotshot_task_impls::rewind::RewindTaskState;
use hotshot_task_impls::{
    builder::BuilderClient, consensus::ConsensusTaskState, consensus2::Consensus2TaskState,
    da::DaTaskState, quorum_proposal::QuorumProposalTaskState,
    quorum_proposal_recv::QuorumProposalRecvTaskState, quorum_vote::QuorumVoteTaskState,
    request::NetworkRequestState, transactions::TransactionTaskState, upgrade::UpgradeTaskState,
    vid::VidTaskState, view_sync::ViewSyncTaskState,
};
use hotshot_types::traits::{
    consensus_api::ConsensusApi,
    node_implementation::{ConsensusTime, NodeImplementation, NodeType},
};
use vbs::version::StaticVersionType;

use crate::types::SystemContextHandle;

/// Trait for creating task states.
#[async_trait]
pub trait CreateTaskState<TYPES, I>
where
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
{
    /// Function to create the task state from a given `SystemContextHandle`.
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> Self;
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: StaticVersionType> CreateTaskState<TYPES, I>
    for NetworkRequestState<TYPES, I, V>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> NetworkRequestState<TYPES, I, V> {
        NetworkRequestState {
            network: Arc::clone(&handle.hotshot.networks.quorum_network),
            state: handle.hotshot.consensus(),
            view: handle.cur_view().await,
            delay: handle.hotshot.config.data_request_delay,
            da_membership: handle.hotshot.memberships.da_membership.clone(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            _phantom: PhantomData,
            id: handle.hotshot.id,
            shutdown_flag: Arc::new(AtomicBool::new(false)),
            spawned_tasks: BTreeMap::new(),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for UpgradeTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> UpgradeTaskState<TYPES, I> {
        UpgradeTaskState {
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            cur_view: handle.cur_view().await,
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            #[cfg(not(feature = "example-upgrade"))]
            should_vote: |_upgrade_proposal| false,
            #[cfg(feature = "example-upgrade")]
            should_vote: |_upgrade_proposal| true,
            vote_collector: None.into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for VidTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> VidTaskState<TYPES, I> {
        VidTaskState {
            consensus: handle.hotshot.consensus(),
            cur_view: handle.cur_view().await,
            vote_collector: None,
            network: Arc::clone(&handle.hotshot.networks.quorum_network),
            membership: handle.hotshot.memberships.vid_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for DaTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> DaTaskState<TYPES, I> {
        DaTaskState {
            consensus: handle.hotshot.consensus(),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            cur_view: handle.cur_view().await,
            vote_collector: None.into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
            storage: Arc::clone(&handle.storage),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for ViewSyncTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> ViewSyncTaskState<TYPES, I> {
        let cur_view = handle.cur_view().await;
        ViewSyncTaskState {
            current_view: cur_view,
            next_view: cur_view,
            network: Arc::clone(&handle.hotshot.networks.quorum_network),
            membership: handle
                .hotshot
                .memberships
                .view_sync_membership
                .clone()
                .into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            num_timeouts_tracked: 0,
            replica_task_map: HashMap::default().into(),
            pre_commit_relay_map: HashMap::default().into(),
            commit_relay_map: HashMap::default().into(),
            finalize_relay_map: HashMap::default().into(),
            view_sync_timeout: handle.hotshot.config.view_sync_timeout,
            id: handle.hotshot.id,
            last_garbage_collected_view: TYPES::Time::new(0),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, Ver: StaticVersionType>
    CreateTaskState<TYPES, I> for TransactionTaskState<TYPES, I, Ver>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> TransactionTaskState<TYPES, I, Ver> {
        TransactionTaskState {
            builder_timeout: handle.builder_timeout(),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            consensus: handle.hotshot.consensus(),
            cur_view: handle.cur_view().await,
            network: Arc::clone(&handle.hotshot.networks.quorum_network),
            membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            instance_state: handle.hotshot.instance_state(),
            id: handle.hotshot.id,
            builder_client: BuilderClient::new(handle.hotshot.config.builder_url.clone()),
            decided_upgrade_certificate: None,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for ConsensusTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> ConsensusTaskState<TYPES, I> {
        let consensus = handle.hotshot.consensus();
        let timeout_task = handle.spawn_initial_timeout_task();

        ConsensusTaskState {
            consensus,
            instance_state: handle.hotshot.instance_state(),
            timeout: handle.hotshot.config.next_view_timeout,
            round_start_delay: handle.hotshot.config.round_start_delay,
            cur_view: handle.cur_view().await,
            cur_view_time: Utc::now().timestamp(),
            payload_commitment_and_metadata: None,
            vote_collector: None.into(),
            timeout_vote_collector: None.into(),
            timeout_task,
            spawned_tasks: BTreeMap::new(),
            formed_upgrade_certificate: None,
            proposal_cert: None,
            decided_upgrade_cert: None,
            version: Arc::clone(&handle.hotshot.version),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            current_proposal: None,
            id: handle.hotshot.id,
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            storage: Arc::clone(&handle.storage),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for QuorumVoteTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> QuorumVoteTaskState<TYPES, I> {
        let consensus = handle.hotshot.consensus();

        QuorumVoteTaskState {
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            consensus,
            instance_state: handle.hotshot.instance_state(),
            latest_voted_view: handle.cur_view().await,
            vote_dependencies: HashMap::new(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            id: handle.hotshot.id,
            storage: Arc::clone(&handle.storage),
            version: *handle.hotshot.version.read().await,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for QuorumProposalTaskState<TYPES, I>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> QuorumProposalTaskState<TYPES, I> {
        let consensus = handle.hotshot.consensus();
        let timeout_task = handle.spawn_initial_timeout_task();

        QuorumProposalTaskState {
            latest_proposed_view: handle.cur_view().await,
            propose_dependencies: HashMap::new(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            consensus,
            instance_state: handle.hotshot.instance_state(),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            storage: Arc::clone(&handle.storage),
            timeout: handle.hotshot.config.next_view_timeout,
            timeout_task,
            round_start_delay: handle.hotshot.config.round_start_delay,
            id: handle.hotshot.id,
            version: *handle.hotshot.version.read().await,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for QuorumProposalRecvTaskState<TYPES, I>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> QuorumProposalRecvTaskState<TYPES, I> {
        let consensus = handle.hotshot.consensus();
        let timeout_task = handle.spawn_initial_timeout_task();

        QuorumProposalRecvTaskState {
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            consensus,
            cur_view: handle.cur_view().await,
            cur_view_time: Utc::now().timestamp(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            timeout_task,
            timeout: handle.hotshot.config.next_view_timeout,
            round_start_delay: handle.hotshot.config.round_start_delay,
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            storage: Arc::clone(&handle.storage),
            formed_upgrade_certificate: None,
            proposal_cert: None,
            decided_upgrade_cert: None,
            spawned_tasks: BTreeMap::new(),
            instance_state: handle.hotshot.instance_state(),
            id: handle.hotshot.id,
            version: *handle.hotshot.version.read().await,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for Consensus2TaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> Consensus2TaskState<TYPES, I> {
        let consensus = handle.hotshot.consensus();
        let timeout_task = handle.spawn_initial_timeout_task();

        Consensus2TaskState {
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            instance_state: handle.hotshot.instance_state(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            committee_membership: handle.hotshot.memberships.da_membership.clone().into(),
            vote_collector: None.into(),
            timeout_vote_collector: None.into(),
            storage: Arc::clone(&handle.storage),
            cur_view: handle.cur_view().await,
            cur_view_time: Utc::now().timestamp(),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            timeout_task,
            timeout: handle.hotshot.config.next_view_timeout,
            consensus,
            last_decided_view: handle.cur_view().await,
            id: handle.hotshot.id,
        }
    }
}

#[cfg(feature = "rewind")]
#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for RewindTaskState<TYPES>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> RewindTaskState<TYPES> {
        RewindTaskState {
            events: Vec::new(),
            id: handle.hotshot.id,
        }
    }
}
