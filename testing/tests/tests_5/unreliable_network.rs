use std::time::{Duration, Instant};

use hotshot_example_types::node_types::{Libp2pImpl, TestTypes};
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    test_builder::{TestDescription, TimingData},
};
use hotshot_types::traits::network::{
    AsynchronousNetwork, ChaosNetwork, PartiallySynchronousNetwork, SynchronousNetwork,
};
use tracing::instrument;

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network_sync() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, Libp2pImpl> = TestDescription {
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
        ..TestDescription::default_multiple_rounds()
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
async fn test_memory_network_sync() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription {
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
        ..TestDescription::default()
    };
    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
#[instrument]
async fn libp2p_network_async() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, Libp2pImpl> = TestDescription {
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
            ..TestDescription::<TestTypes, Libp2pImpl>::default_multiple_rounds().timing_data
        },
        unreliable_network: Some(Box::new(AsynchronousNetwork {
            keep_numerator: 9,
            keep_denominator: 10,
            delay_low_ms: 4,
            delay_high_ms: 30,
        })),
        ..TestDescription::default_multiple_rounds()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[ignore]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_async() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription {
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
            ..TestDescription::<TestTypes, MemoryImpl>::default_multiple_rounds().timing_data
        },
        unreliable_network: Some(Box::new(AsynchronousNetwork {
            keep_numerator: 95,
            keep_denominator: 100,
            delay_low_ms: 4,
            delay_high_ms: 30,
        })),
        ..TestDescription::default()
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
async fn test_memory_network_partially_sync() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 0,
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
        ..TestDescription::default()
    };
    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn libp2p_network_partially_sync() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, Libp2pImpl> = TestDescription {
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 0,
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
        ..TestDescription::default_multiple_rounds()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[ignore]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_memory_network_chaos() {
    use std::time::Duration;

    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        test_builder::TestDescription,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, MemoryImpl> = TestDescription {
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
        ..TestDescription::default()
    };
    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[ignore]
#[instrument]
async fn libp2p_network_chaos() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription<TestTypes, Libp2pImpl> = TestDescription {
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
        ..TestDescription::default_multiple_rounds()
    };

    metadata
        .gen_launcher(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}
