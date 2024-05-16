use std::collections::HashMap;

use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_task() {
    use hotshot_task_impls::{harness::run_harness, view_sync::ViewSyncTaskState};
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::simple_vote::ViewSyncPreCommitData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 5.
    let handle = build_system_handle(5).await.0;

    let vote_data = ViewSyncPreCommitData {
        relay: 0,
        round: <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(4),
    };
    let vote = hotshot_types::simple_vote::ViewSyncPreCommitVote::<TestTypes>::create_signed_vote(
        vote_data,
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(4),
        hotshot_types::traits::consensus_api::ConsensusApi::public_key(&handle),
        hotshot_types::traits::consensus_api::ConsensusApi::private_key(&handle),
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

    let view_sync_state = ViewSyncTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;
    run_harness(input, output, view_sync_state, false).await;
}
