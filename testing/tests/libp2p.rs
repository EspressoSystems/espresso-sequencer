use std::time::Duration;

use hotshot_testing::{
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    node_types::{SequencingLibp2pImpl, SequencingTestTypes},
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_builder::TestMetadata,
};
use tracing::instrument;

/// libp2p network test
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
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
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<SequencingTestTypes, SequencingLibp2pImpl>(0)
        .launch()
        .run_test()
        .await
}

/// stress test for libp2p
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_libp2p_network() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata::default_stress();
    metadata
        .gen_launcher::<SequencingTestTypes, SequencingLibp2pImpl>(0)
        .launch()
        .run_test()
        .await
}
