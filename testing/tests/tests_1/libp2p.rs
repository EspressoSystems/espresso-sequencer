use std::time::Duration;

use hotshot_example_types::node_types::{Libp2pImpl, TestTypes};
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
    test_builder::{TestMetadata, TimingData},
};
use tracing::instrument;

/// libp2p network test
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            check_leaf: true,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::new(240, 0),
            },
        ),
        timing_data: TimingData {
            next_view_timeout: 4000,
            ..Default::default()
        },
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}

/// libp2p network test with failures
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network_failures_2() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            check_leaf: true,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(240),
            },
        ),
        timing_data: TimingData {
            next_view_timeout: 4000,
            ..Default::default()
        },
        ..TestMetadata::default_multiple_rounds()
    };

    let dead_nodes = vec![ChangeNode {
        idx: 11,
        updown: UpDown::Down,
    }];

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(3, dead_nodes)],
    };
    metadata.num_nodes_with_stake = 12;
    metadata.da_staked_committee_size = 12;
    metadata.start_nodes = 12;
    // 2 nodes fail triggering view sync, expect no other timeouts
    metadata.overall_safety_properties.num_failed_views = 1;
    // Make sure we keep committing rounds after the bad leaders, but not the full 50 because of the numerous timeouts
    metadata.overall_safety_properties.num_successful_views = 15;

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}

/// stress test for libp2p
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_libp2p_network() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata::default_stress();
    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}
