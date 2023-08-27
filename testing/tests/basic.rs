#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_success() {
    use hotshot_testing::{
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata::default();
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
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
    use std::time::Duration;

    use hotshot_testing::{
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes_less_success();
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
        node_changes: vec![(Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
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
    use std::time::Duration;

    use hotshot_testing::{
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes_less_success();
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
        node_changes: vec![(Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
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
    use std::time::Duration;

    use hotshot_testing::{
        node_types::{SequencingMemoryImpl, SequencingTestTypes},
        spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
        test_builder::TestMetadata,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata::default_more_nodes_less_success();
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
        node_changes: vec![(Duration::new(4, 0), dead_nodes)],
    };
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>()
        .launch()
        .run_test()
        .await;
}
