// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription::default();
    let catchup_node = vec![ChangeNode {
        idx: 19,
        updown: UpDown::Up,
    }];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.num_nodes_with_stake = 20;

    metadata.view_sync_properties =
        hotshot_testing::view_sync_task::ViewSyncTaskDescription::Threshold(0, 20);

    metadata.spinning_properties = SpinningTaskDescription {
        // Start the nodes before their leadership.
        node_changes: vec![(13, catchup_node)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        // Make sure we keep committing rounds after the catchup, but not the full 50.
        num_successful_views: 22,
        num_failed_views: 0,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup_cdn() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{PushCdnImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, PushCdnImpl> = TestDescription::default();
    let catchup_nodes = vec![ChangeNode {
        idx: 18,
        updown: UpDown::Up,
    }];
    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.num_nodes_with_stake = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        // Start the nodes before their leadership.
        node_changes: vec![(10, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_millis(100_000),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 0,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

/// Test that one node catches up and has successful views after coming back
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup_one_node() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription::default();
    let catchup_nodes = vec![ChangeNode {
        idx: 18,
        updown: UpDown::Up,
    }];
    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.num_nodes_with_stake = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        // Start the nodes before their leadership.
        node_changes: vec![(10, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        // Make sure we keep committing rounds after the catchup, but not the full 50.
        num_successful_views: 22,
        num_failed_views: 0,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

/// Same as `test_catchup` except we start the nodes after their leadership so they join during view sync
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup_in_view_sync() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription::default();
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
    metadata.num_nodes_with_stake = 20;
    metadata.view_sync_properties =
        hotshot_testing::view_sync_task::ViewSyncTaskDescription::Threshold(0, 20);

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(10, catchup_nodes)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        num_failed_views: 0,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

// Almost the same as `test_catchup`, but with catchup nodes reloaded from anchor leaf rather than
// initialized from genesis.
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_catchup_reload() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription::default();
    let catchup_node = vec![ChangeNode {
        idx: 19,
        updown: UpDown::Up,
    }];

    metadata.timing_data = timing_data;
    metadata.start_nodes = 19;
    metadata.skip_late = true;
    metadata.num_nodes_with_stake = 20;

    metadata.view_sync_properties =
        hotshot_testing::view_sync_task::ViewSyncTaskDescription::Threshold(0, 20);

    metadata.spinning_properties = SpinningTaskDescription {
        // Start the nodes before their leadership.
        node_changes: vec![(13, catchup_node)],
    };

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        // Make sure we keep committing rounds after the catchup, but not the full 50.
        num_successful_views: 22,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_all_restart() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{CombinedImpl, TestTypes};
    use hotshot_testing::{
        block_builder::SimpleBuilderImplementation,
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::{TestDescription, TimingData},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let timing_data = TimingData {
        next_view_timeout: 2000,
        ..Default::default()
    };
    let mut metadata: TestDescription<TestTypes, CombinedImpl> = TestDescription::default();
    let mut catchup_nodes = vec![];
    for i in 1..20 {
        catchup_nodes.push(ChangeNode {
            idx: i,
            updown: UpDown::Restart,
        })
    }

    metadata.timing_data = timing_data;
    metadata.start_nodes = 20;
    metadata.num_nodes_with_stake = 20;

    metadata.spinning_properties = SpinningTaskDescription {
        // Restart all the nodes in view 13
        node_changes: vec![(13, catchup_nodes)],
    };
    metadata.view_sync_properties =
        hotshot_testing::view_sync_task::ViewSyncTaskDescription::Threshold(0, 20);

    metadata.completion_task_description =
        CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        );
    metadata.overall_safety_properties = OverallSafetyPropertiesDescription {
        // Make sure we keep committing rounds after the catchup, but not the full 50.
        num_successful_views: 22,
        num_failed_views: 15,
        ..Default::default()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}
