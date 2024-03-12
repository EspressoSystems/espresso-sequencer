use crate::types::SystemContextHandle;

use async_trait::async_trait;
use hotshot_task_impls::{
    consensus::{CommitmentAndMetadata, ConsensusTaskState},
    da::DATaskState,
    transactions::TransactionTaskState,
    upgrade::UpgradeTaskState,
    vid::VIDTaskState,
    view_sync::ViewSyncTaskState,
};
use hotshot_types::constants::VERSION_0_1;
use hotshot_types::traits::election::Membership;
use hotshot_types::traits::{
    block_contents::vid_commitment,
    consensus_api::ConsensusApi,
    node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    BlockPayload,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
    time::Duration,
};

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
            quorum_network: handle.hotshot.networks.quorum_network.clone(),
            should_vote: |_upgrade_proposal| false,
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
            network: handle.hotshot.networks.quorum_network.clone(),
            membership: handle.hotshot.memberships.vid_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for DATaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> DATaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        DATaskState {
            api: handle.clone(),
            consensus: handle.hotshot.get_consensus(),
            da_membership: handle.hotshot.memberships.da_membership.clone().into(),
            da_network: handle.hotshot.networks.da_network.clone(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            cur_view: handle.get_cur_view().await,
            vote_collector: None.into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
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
            network: handle.hotshot.networks.quorum_network.clone(),
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
            view_sync_timeout: Duration::new(10, 0),
            id: handle.hotshot.id,
            last_garbage_collected_view: TYPES::Time::new(0),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for TransactionTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> TransactionTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        TransactionTaskState {
            api: handle.clone(),
            consensus: handle.hotshot.get_consensus(),
            transactions: Arc::default(),
            seen_transactions: HashSet::new(),
            cur_view: handle.get_cur_view().await,
            network: handle.hotshot.networks.quorum_network.clone(),
            membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            id: handle.hotshot.id,
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> CreateTaskState<TYPES, I>
    for ConsensusTaskState<TYPES, I, SystemContextHandle<TYPES, I>>
{
    async fn create_from(
        handle: &SystemContextHandle<TYPES, I>,
    ) -> ConsensusTaskState<TYPES, I, SystemContextHandle<TYPES, I>> {
        let consensus = handle.hotshot.get_consensus();

        let (payload, metadata) = <TYPES::BlockPayload as BlockPayload>::genesis();
        // Impossible for `unwrap` to fail on the genesis payload.
        let payload_commitment = vid_commitment(
            &payload.encode().unwrap().collect(),
            handle.hotshot.memberships.quorum_membership.total_nodes(),
        );
        ConsensusTaskState {
            consensus,
            timeout: handle.hotshot.config.next_view_timeout,
            cur_view: handle.get_cur_view().await,
            payload_commitment_and_metadata: Some(CommitmentAndMetadata {
                commitment: payload_commitment,
                metadata,
                is_genesis: true,
            }),
            api: handle.clone(),
            _pd: PhantomData,
            vote_collector: None.into(),
            timeout_vote_collector: None.into(),
            timeout_task: None,
            timeout_cert: None,
            upgrade_cert: None,
            decided_upgrade_cert: None,
            current_network_version: VERSION_0_1,
            output_event_stream: handle.hotshot.output_event_stream.0.clone(),
            vid_shares: BTreeMap::new(),
            current_proposal: None,
            id: handle.hotshot.id,
            public_key: handle.public_key().clone(),
            private_key: handle.private_key().clone(),
            quorum_network: handle.hotshot.networks.quorum_network.clone(),
            committee_network: handle.hotshot.networks.da_network.clone(),
            timeout_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
            committee_membership: handle.hotshot.memberships.da_membership.clone().into(),
        }
    }
}
