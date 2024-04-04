use hotshot_testing::block_builder::SimpleBuilderImplementation;
use hotshot_testing::test_builder::TimingData;
use hotshot_types::traits::network::AsynchronousNetwork;
use hotshot_types::traits::network::ChaosNetwork;
use hotshot_types::traits::network::PartiallySynchronousNetwork;
use hotshot_types::traits::network::SynchronousNetwork;
use std::time::Duration;
use std::time::Instant;

use hotshot_example_types::node_types::{Libp2pImpl, TestTypes};
use hotshot_testing::{
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_builder::TestMetadata,
};
use tracing::instrument;

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network_sync() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            check_leaf: true,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::new(360, 0),
            },
        ),
        unreliable_network: Some(Box::new(SynchronousNetwork {
            delay_high_ms: 30,
            delay_low_ms: 4,
        })),
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_sync() {
    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestMetadata,
    };
    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(240),
            },
        ),
        unreliable_network: Some(Box::new(SynchronousNetwork {
            delay_high_ms: 30,
            delay_low_ms: 4,
        })),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<MemoryImpl>>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
#[instrument]
async fn libp2p_network_async() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            check_leaf: true,
            num_failed_views: 50,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::new(240, 0),
            },
        ),
        timing_data: TimingData {
            timeout_ratio: (1, 1),
            next_view_timeout: 25000,
            ..TestMetadata::default_multiple_rounds().timing_data
        },
        unreliable_network: Some(Box::new(AsynchronousNetwork {
            keep_numerator: 9,
            keep_denominator: 10,
            delay_low_ms: 4,
            delay_high_ms: 30,
        })),
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<Libp2pImpl>>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[ignore]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_async() {
    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestMetadata,
    };

    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            check_leaf: true,
            num_failed_views: 5000,
            ..Default::default()
        },
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(240),
            },
        ),
        timing_data: TimingData {
            timeout_ratio: (1, 1),
            next_view_timeout: 1000,
            ..TestMetadata::default_multiple_rounds().timing_data
        },
        unreliable_network: Some(Box::new(AsynchronousNetwork {
            keep_numerator: 95,
            keep_denominator: 100,
            delay_low_ms: 4,
            delay_high_ms: 30,
        })),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<_>>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_partially_sync() {
    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestMetadata,
    };

    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 2,
            ..Default::default()
        },
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(240),
            },
        ),
        timing_data: TimingData {
            next_view_timeout: 25000,
            ..Default::default()
        },
        unreliable_network: Some(Box::new(PartiallySynchronousNetwork {
            asynchronous: AsynchronousNetwork {
                keep_numerator: 8,
                keep_denominator: 10,
                delay_low_ms: 4,
                delay_high_ms: 30,
            },
            synchronous: SynchronousNetwork {
                delay_high_ms: 30,
                delay_low_ms: 4,
            },
            gst: std::time::Duration::from_millis(1000),
            start: Instant::now(),
        })),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<_>>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network_partially_sync() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 2,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::new(240, 0),
            },
        ),
        unreliable_network: Some(Box::new(PartiallySynchronousNetwork {
            asynchronous: AsynchronousNetwork {
                keep_numerator: 8,
                keep_denominator: 10,
                delay_low_ms: 4,
                delay_high_ms: 30,
            },
            synchronous: SynchronousNetwork {
                delay_high_ms: 30,
                delay_low_ms: 4,
            },
            gst: std::time::Duration::from_millis(1000),
            start: Instant::now(),
        })),
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<_>>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[ignore]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_chaos() {
    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestMetadata,
    };
    use std::time::Duration;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata = TestMetadata {
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(240),
            },
        ),
        unreliable_network: Some(Box::new(ChaosNetwork {
            keep_numerator: 8,
            keep_denominator: 10,
            delay_low_ms: 4,
            delay_high_ms: 30,
            repeat_low: 1,
            repeat_high: 5,
        })),
        ..TestMetadata::default()
    };
    metadata
        .gen_launcher::<TestTypes, MemoryImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<_>>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
#[instrument]
async fn libp2p_network_chaos() {
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
        unreliable_network: Some(Box::new(ChaosNetwork {
            keep_numerator: 8,
            keep_denominator: 10,
            delay_low_ms: 4,
            delay_high_ms: 30,
            repeat_low: 1,
            repeat_high: 5,
        })),
        ..TestMetadata::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, Libp2pImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation<_>>()
        .await;
}
