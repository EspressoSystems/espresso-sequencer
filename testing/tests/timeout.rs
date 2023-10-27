#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
// TODO Add memory network tests after this issue is finished:
// https://github.com/EspressoSystems/HotShot/issues/1790
async fn test_timeout_web() {
    use std::time::Duration;

    use hotshot_testing::node_types::WebImpl;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::TestTypes,
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 1000,
        ..Default::default()
    };

    let mut metadata = TestMetadata {
        total_nodes: 10,
        start_nodes: 10,
        ..Default::default()
    };
    let dead_nodes = vec![ChangeNode {
        idx: 0,
        updown: UpDown::Down,
    }];

    metadata.timing_data = timing_data;

    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_successful_views: 25,
        ..Default::default()
    };

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(Duration::from_millis(500), dead_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(30),
            },
        );

    // TODO ED Test with memory network once issue is resolved
    // https://github.com/EspressoSystems/HotShot/issues/1790
    metadata
        .gen_launcher::<TestTypes, WebImpl>()
        .launch()
        .run_test()
        .await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_timeout_libp2p() {
    use std::time::Duration;

    use hotshot_testing::node_types::Libp2pImpl;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::TestTypes,
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 1000,
        ..Default::default()
    };

    let mut metadata = TestMetadata {
        total_nodes: 10,
        start_nodes: 10,
        ..Default::default()
    };
    let dead_nodes = vec![ChangeNode {
        idx: 0,
        updown: UpDown::Down,
    }];

    metadata.timing_data = timing_data;

    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_successful_views: 25,
        ..Default::default()
    };

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(Duration::from_millis(500), dead_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(30),
            },
        );

    // TODO ED Test with memory network once issue is resolved
    // https://github.com/EspressoSystems/HotShot/issues/1790
    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>()
        .launch()
        .run_test()
        .await;
}
