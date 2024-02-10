use hotshot::HotShotConsensusApi;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::node_types::{MemoryImpl, TestTypes};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use std::collections::HashMap;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_task() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_task_impls::view_sync::ViewSyncTaskState;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::simple_vote::ViewSyncPreCommitData;
    use hotshot_types::traits::consensus_api::ConsensusApi;
    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 5.
    let handle = build_system_handle(5).await.0;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };

    let vote_data = ViewSyncPreCommitData {
        relay: 0,
        round: <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(4),
    };
    let vote = hotshot_types::simple_vote::ViewSyncPreCommitVote::<TestTypes>::create_signed_vote(
        vote_data,
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(4),
        hotshot_types::traits::consensus_api::ConsensusApi::public_key(&api),
        hotshot_types::traits::consensus_api::ConsensusApi::private_key(&api),
    )
    .expect("Failed to create a ViewSyncPreCommitVote!");

    tracing::error!("Vote in test is {:?}", vote.clone());

    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(HotShotEvent::Timeout(ViewNumber::new(2)));
    input.push(HotShotEvent::Timeout(ViewNumber::new(3)));

    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::ViewSyncPreCommitVoteSend(vote.clone()), 1);

    let view_sync_state = ViewSyncTaskState {
        current_view: ViewNumber::new(0),
        next_view: ViewNumber::new(0),
        network: api.inner.networks.quorum_network.clone().into(),
        membership: api.inner.memberships.view_sync_membership.clone().into(),
        public_key: *api.public_key(),
        private_key: api.private_key().clone(),
        api,
        num_timeouts_tracked: 0,
        replica_task_map: HashMap::default().into(),
        pre_commit_relay_map: HashMap::default().into(),
        commit_relay_map: HashMap::default().into(),
        finalize_relay_map: HashMap::default().into(),
        view_sync_timeout: Duration::new(10, 0),
        id: handle.hotshot.inner.id,
        last_garbage_collected_view: ViewNumber::new(0),
    };
    run_harness(input, output, view_sync_state, false).await;
}
