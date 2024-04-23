use std::time::Duration;

use hotshot_example_types::node_types::{CombinedImpl, TestTypes};
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
    test_builder::{TestDescription, TimingData},
};
use rand::Rng;
use tracing::instrument;

/// A run with both the CDN and libp2p functioning properly
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_combined_network() {
    use hotshot_testing::block_builder::SimpleBuilderImplementation;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let metadata: TestDescription = TestDescription {
        timing_data: TimingData {
            round_start_delay: 25,
            next_view_timeout: 10_000,
            start_delay: 120_000,

            ..Default::default()
        },
        overall_safety_properties: OverallSafetyPropertiesDescription {
            num_failed_views: 33,
            num_successful_views: 25,
            ..Default::default()
        },
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(120),
            },
        ),
        ..TestDescription::default_multiple_rounds()
    };

    metadata
        .gen_launcher::<TestTypes, CombinedImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

// A run where the CDN crashes part-way through
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_combined_network_cdn_crash() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata: TestDescription = TestDescription {
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
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(120),
            },
        ),
        ..TestDescription::default_multiple_rounds()
    };

    let mut all_nodes = vec![];
    for node in 0..metadata.num_nodes_with_stake {
        all_nodes.push(ChangeNode {
            idx: node,
            updown: UpDown::NetworkDown,
        });
    }

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, all_nodes)],
    };

    metadata
        .gen_launcher::<TestTypes, CombinedImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

// A run where the CDN crashes partway through
// and then comes back up
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_combined_network_reup() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata: TestDescription = TestDescription {
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
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(120),
            },
        ),
        ..TestDescription::default_multiple_rounds()
    };

    let mut all_down = vec![];
    let mut all_up = vec![];
    for node in 0..metadata.num_nodes_with_stake {
        all_down.push(ChangeNode {
            idx: node,
            updown: UpDown::NetworkDown,
        });
        all_up.push(ChangeNode {
            idx: node,
            updown: UpDown::NetworkUp,
        });
    }

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(13, all_up), (5, all_down)],
    };

    metadata
        .gen_launcher::<TestTypes, CombinedImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

// A run where half of the nodes disconnect from the CDN
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_combined_network_half_dc() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata: TestDescription = TestDescription {
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
        // allow more time to pass in CI
        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(120),
            },
        ),
        ..TestDescription::default_multiple_rounds()
    };

    let mut half = vec![];
    for node in 0..metadata.num_nodes_with_stake / 2 {
        half.push(ChangeNode {
            idx: node,
            updown: UpDown::NetworkDown,
        });
    }

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: vec![(5, half)],
    };

    metadata
        .gen_launcher::<TestTypes, CombinedImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}

fn generate_random_node_changes(
    total_nodes: usize,
    total_num_rounds: usize,
) -> Vec<(u64, Vec<ChangeNode>)> {
    let mut rng = rand::thread_rng();
    let mut node_changes = vec![];

    for _ in 0..total_nodes * 2 {
        let updown = if rng.gen::<bool>() {
            UpDown::NetworkUp
        } else {
            UpDown::NetworkDown
        };

        let node_change = ChangeNode {
            idx: rng.gen_range(0..total_nodes),
            updown,
        };

        let round = rng.gen_range(1..total_num_rounds) as u64;

        node_changes.push((round, vec![node_change]));
    }

    node_changes
}

// A fuzz test, where random network events take place on all nodes
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_combined_network_fuzzy() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    let mut metadata = TestDescription {
        num_bootstrap_nodes: 10,
        num_nodes_with_stake: 20,
        start_nodes: 20,

        timing_data: TimingData {
            round_start_delay: 25,
            next_view_timeout: 10_000,
            start_delay: 120_000,

            ..Default::default()
        },

        completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
            TimeBasedCompletionTaskDescription {
                duration: Duration::from_secs(120),
            },
        ),
        ..TestDescription::default_stress()
    };

    metadata.spinning_properties = SpinningTaskDescription {
        node_changes: generate_random_node_changes(
            metadata.num_nodes_with_stake,
            metadata.overall_safety_properties.num_successful_views * 2,
        ),
    };

    metadata
        .gen_launcher::<TestTypes, CombinedImpl>(0)
        .launch()
        .run_test::<SimpleBuilderImplementation>()
        .await;
}
