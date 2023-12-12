#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{MemoryImpl, TestTypes},
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
    let mut metadata = TestMetadata::default();
    let catchup_nodes = vec![
        ChangeNode {
            idx: 18,
            updown: UpDown::Up,
        },
        ChangeNode {
            idx: 19,
            updown: UpDown::Up,
        },
    ];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 18;
    metadata.total_nodes = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(25, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(100000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 25,
        check_leaf: true,
        ..Default::default()
    };
    metadata.overall_safety_properties.num_failed_views = 2;

    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
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
async fn test_catchup_web() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{TestTypes, WebImpl},
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
    let mut metadata = TestMetadata::default();
    let catchup_nodes = vec![ChangeNode {
        idx: 18,
        updown: UpDown::Up,
    }];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.total_nodes = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(25, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(100000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 25,
        check_leaf: true,
        ..Default::default()
    };

    metadata
        .gen_launcher::<TestTypes, WebImpl>(0)
        .launch()
        .run_test()
        .await;
}

/// Test that one node catches up and has sucessful views after coming back
#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
async fn test_catchup_one_node() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{MemoryImpl, TestTypes},
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
    let mut metadata = TestMetadata::default();
    let catchup_nodes = vec![ChangeNode {
        idx: 18,
        updown: UpDown::Up,
    }];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.total_nodes = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(25, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(20000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 25,
        check_leaf: true,
        ..Default::default()
    };
    // only alow for the view which the catchup node hasn't started to fail
    metadata.overall_safety_properties.num_failed_views = 5;

    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}

/// Same as `test_catchup` except we start the nodes after their leadership so they join during view sync
/// This fails for the same reason as the timeout test and should work once that is fixed.
#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
async fn test_catchup_in_view_sync() {
    use std::time::Duration;

    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{MemoryImpl, TestTypes},
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
    let mut metadata = TestMetadata::default();
    let catchup_nodes = vec![
        ChangeNode {
            idx: 18,
            updown: UpDown::Up,
        },
        ChangeNode {
            idx: 19,
            updown: UpDown::Up,
        },
    ];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 18;
    metadata.total_nodes = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(25, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(10000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        check_leaf: true,
        num_failed_views: 25,
        ..Default::default()
    };

    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}
