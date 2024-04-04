#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
// TODO Add memory network tests after this issue is finished:
// https://github.com/EspressoSystems/HotShot/issues/1790
async fn test_timeout_web() {
    use std::time::Duration;

    use hotshot_example_types::node_types::WebImpl;

    use hotshot_example_types::node_types::TestTypes;
    use hotshot_testing::block_builder::SimpleBuilderImplementation;
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };

    let mut metadata = TestMetadata {
        num_nodes_with_stake: 10,
        start_nodes: 10,
        ..Default::default()
    };
    let dead_nodes = vec![ChangeNode {
        idx: 0,
        updown: UpDown::Down,
    }];

    metadata.timing_data = timing_data;

    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 25,
        num_successful_views: 25,
        ..Default::default()
    };

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, dead_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );

    // TODO ED Test with memory network once issue is resolved
    // https://github.com/EspressoSystems/HotShot/issues/1790
    metadata
        .gen_launcher::<TestTypes, WebImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<WebImpl>>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
async fn test_timeout_libp2p() {
    use std::time::Duration;

    use hotshot_example_types::node_types::Libp2pImpl;

    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestMetadata, TimingData},
    };

    use hotshot_example_types::node_types::TestTypes;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        start_delay: 2000,
        round_start_delay: 1000,
        ..Default::default()
    };

    let mut metadata = TestMetadata {
        num_nodes_with_stake: 10,
        start_nodes: 10,
        num_bootstrap_nodes: 10,
        ..Default::default()
    };
    let dead_nodes = vec![ChangeNode {
        idx: 9,
        updown: UpDown::Down,
    }];

    metadata.timing_data = timing_data;

    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 25,
        num_successful_views: 25,
        ..Default::default()
    };

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, dead_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );

    // TODO ED Test with memory network once issue is resolved
    // https://github.com/EspressoSystems/HotShot/issues/1790
    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}
