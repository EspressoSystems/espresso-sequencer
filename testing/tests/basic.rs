#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_success() {
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        node_types::{MemoryImpl, TestTypes},
        test_builder::TestMetadata,
    };
    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        ),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}

/// Test one node leaving the network.
#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_with_failures_one() {
    use hotshot_testing::{
        node_types::{MemoryImpl, TestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes();
    // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
    // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
    // following issue.
    // TODO: Update message broadcasting to avoid hanging
    // <https://github.com/EspressoSystems/HotShot/issues/1567>
    let dead_nodes = vec![ChangeNode {
        idx: 19,
        updown: UpDown::Down,
    }];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, dead_nodes)],
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}

/// Test f/2 nodes leaving the network.
#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_with_failures_half_f() {
    use hotshot_testing::{
        node_types::{MemoryImpl, TestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes();
    // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
    // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
    // following issue.
    // TODO: Update message broadcasting to avoid hanging
    // <https://github.com/EspressoSystems/HotShot/issues/1567>
    let dead_nodes = vec![
        ChangeNode {
            idx: 17,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 18,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 19,
            updown: UpDown::Down,
        },
    ];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, dead_nodes)],
    };
    metadata.overall_safety_properties.num_failed_views = 6;
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}

/// Test f nodes leaving the network.
#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_with_failures_f() {
    use hotshot_testing::{
        node_types::{MemoryImpl, TestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes();
    metadata.overall_safety_properties.num_failed_views = 6;
    // Make sure we keep commiting rounds after the bad leaders, but not the full 50 because of the numerous timeouts
    metadata.overall_safety_properties.num_successful_views = 22;
    // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
    // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
    // following issue.
    // TODO: Update message broadcasting to avoid hanging
    // <https://github.com/EspressoSystems/HotShot/issues/1567>
    let dead_nodes = vec![
        ChangeNode {
            idx: 14,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 15,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 16,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 17,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 18,
            updown: UpDown::Down,
        },
        ChangeNode {
            idx: 19,
            updown: UpDown::Down,
        },
    ];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, dead_nodes)],
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test()
        .await;
}
