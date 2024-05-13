use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    sync::{atomic::AtomicBool, Arc},
};

use async_trait::async_trait;
use hotshot_task_impls::{
    builder::BuilderClient, consensus::ConsensusTaskState, da::DaTaskState,
    quorum_proposal::QuorumProposalTaskState, quorum_proposal_recv::QuorumProposalRecvTaskState,
    quorum_vote::QuorumVoteTaskState, request::NetworkRequestState,
    transactions::TransactionTaskState, upgrade::UpgradeTaskState, vid::VIDTaskState,
    view_sync::ViewSyncTaskState,
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
            state: handle.hotshot.get_consensus(),
            view: handle.get_cur_view().await,
            delay: handle.hotshot.config.data_request_delay,
            da_membership: handle.hotshot.memberships.da_membership.clone(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            _phantom: PhantomData,
            id: handle.hotshot.id,
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for UpgradeTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> UpgradeTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        UpgradeTaskState {
            api: handle.clone(),
            cur_view: handle.get_cur_view().await,
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
    for VIDTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> VIDTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        VIDTaskState {
            api: handle.clone(),
            consensus: handle.hotshot.get_consensus(),
            cur_view: handle.get_cur_view().await,
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
    for DaTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> DaTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        DaTaskState {
            api: handle.clone(),
            consensus: handle.hotshot.get_consensus(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            cur_view: handle.get_cur_view().await,
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
    for ViewSyncTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> ViewSyncTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        let cur_view = handle.get_cur_view().await;
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
            api: handle.clone(),
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
    CreateTaskState<TYPES, I>
    for TransactionTaskState<TYPES, I, SystemContextHandle<TYPES, I>, Ver>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> TransactionTaskState<TYPES, I, SystemContextHandle<TYPES, I>, Ver> {
        TransactionTaskState {
            api: handle.clone(),
            consensus: handle.hotshot.get_consensus(),
            cur_view: handle.get_cur_view().await,
            network: Arc::clone(&handle.hotshot.networks.quorum_network),
            membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            instance_state: handle.hotshot.get_instance_state(),
            id: handle.hotshot.id,
            builder_client: BuilderClient::new(handle.hotshot.config.builder_url.clone()),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for ConsensusTaskState<TYPES, I>
{
    async fn create_from(handle: &SystemContextHandle<TYPES, I>) -> ConsensusTaskState<TYPES, I> {
        let consensus = handle.hotshot.get_consensus();

        ConsensusTaskState {
            consensus,
            instance_state: handle.hotshot.get_instance_state(),
            timeout: handle.hotshot.config.next_view_timeout,
            round_start_delay: handle.hotshot.config.round_start_delay,
            cur_view: handle.get_cur_view().await,
            payload_commitment_and_metadata: None,
            vote_collector: None.into(),
            timeout_vote_collector: None.into(),
            timeout_task: None,
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
        let consensus = handle.hotshot.get_consensus();

        QuorumVoteTaskState {
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            consensus,
            instance_state: handle.hotshot.get_instance_state(),
            latest_voted_view: handle.get_cur_view().await,
            vote_dependencies: HashMap::new(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            id: handle.hotshot.id,
            storage: Arc::clone(&handle.storage),
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
        let consensus = handle.hotshot.get_consensus();
        QuorumProposalTaskState {
            latest_proposed_view: handle.get_cur_view().await,
            propose_dependencies: HashMap::new(),
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            da_network: Arc::clone(&handle.hotshot.networks.da_network),
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            consensus,
            instance_state: handle.hotshot.get_instance_state(),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            storage: Arc::clone(&handle.storage),
            timeout: handle.hotshot.config.next_view_timeout,
            timeout_task: None,
            round_start_delay: handle.hotshot.config.round_start_delay,
            id: handle.hotshot.id,
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
        let consensus = handle.hotshot.get_consensus();
        QuorumProposalRecvTaskState {
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            consensus,
            cur_view: handle.get_cur_view().await,
            quorum_network: Arc::clone(&handle.hotshot.networks.quorum_network),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            timeout_task: None,
            timeout: handle.hotshot.config.next_view_timeout,
            round_start_delay: handle.hotshot.config.round_start_delay,
            output_event_stream: handle.hotshot.external_event_stream.0.clone(),
            storage: Arc::clone(&handle.storage),
            formed_upgrade_certificate: None,
            proposal_cert: None,
            decided_upgrade_cert: None,
            spawned_tasks: BTreeMap::new(),
            instance_state: handle.hotshot.get_instance_state(),
            id: handle.hotshot.id,
        }
    }
}
