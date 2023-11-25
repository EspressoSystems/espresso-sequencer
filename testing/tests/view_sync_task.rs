use hotshot::HotShotConsensusApi;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::node_types::{MemoryImpl, TestTypes};
use hotshot_types::{data::ViewNumber, traits::state::ConsensusTime};
use std::collections::HashMap;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_task() {
    use hotshot::tasks::add_view_sync_task;
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::simple_vote::ViewSyncPreCommitData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 5.
    let handle = build_system_handle(5).await.0;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };

    let vote_data = ViewSyncPreCommitData {
        relay: 0,
        round: <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(5),
    };
    let vote = hotshot_types::simple_vote::ViewSyncPreCommitVote::<TestTypes>::create_signed_vote(
        vote_data,
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::Time::new(5),
        hotshot_types::traits::consensus_api::ConsensusSharedApi::public_key(&api),
        hotshot_types::traits::consensus_api::ConsensusSharedApi::private_key(&api),
    )
    .unwrap();

    tracing::error!("Vote in test is {:?}", vote.clone());

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(HotShotEvent::Timeout(ViewNumber::new(2)));
    input.push(HotShotEvent::Timeout(ViewNumber::new(3)));
    input.push(HotShotEvent::Timeout(ViewNumber::new(4)));

    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::Timeout(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::Timeout(ViewNumber::new(3)), 1);
    output.insert(HotShotEvent::Timeout(ViewNumber::new(4)), 1);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::ViewChange(ViewNumber::new(3)), 1);
    output.insert(HotShotEvent::ViewSyncPreCommitVoteSend(vote.clone()), 1);

    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn =
        |task_runner, event_stream| add_view_sync_task(task_runner, event_stream, handle);

    run_harness(input, output, None, build_fn).await;
}
