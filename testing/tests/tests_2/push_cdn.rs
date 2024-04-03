use std::time::Duration;

use async_compatibility_layer::logging::shutdown_logging;
use hotshot_example_types::node_types::{PushCdnImpl, TestTypes};
use hotshot_testing::{
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_builder::{TestMetadata, TimingData},
};
use tracing::instrument;

/// Push CDN network test
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn push_cdn_network() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        timing_data: TimingData {
            round_start_delay: 25,
            next_view_timeout: 10_000,
            start_delay: 120_000,

            ..Default::default()
        },
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 33,
            num_successful_views: 35,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(60),
            },
        ),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, PushCdnImpl>(0)
        .launch()
        .run_test()
        .await;
    shutdown_logging();
}
