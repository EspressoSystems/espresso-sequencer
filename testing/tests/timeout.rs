#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
async fn test_timeout() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 1000,
        ..Default::default()
    };
    let mut metadata = TestMetadata::default();
    let dead_nodes = vec![ChangeNode {
        idx: 0,
        updown: UpDown::Down,
    }];

    metadata.timing_data = timing_data;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(Duration::new(0, 5000), dead_nodes)],
    };

    // TODO ED Add safety task, etc to confirm TCs are being formed

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(10000),
            },
        );
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}
