// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::HashMap, num::NonZeroUsize, rc::Rc, sync::Arc, time::Duration};

use anyhow::{ensure, Result};
use hotshot::{
    tasks::EventTransformerState,
    traits::{NetworkReliability, NodeImplementation, TestableNodeImplementation},
    types::SystemContextHandle,
    HotShotInitializer, MarketplaceConfig, Memberships, SystemContext, TwinsHandlerState,
};
use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResultsProvider, state_types::TestInstanceState,
    storage_types::TestStorage, testable_delay::DelayConfig,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    traits::node_implementation::{NodeType, Versions},
    ExecutionType, HotShotConfig, ValidatorConfig,
};
use tide_disco::Url;
use vec1::Vec1;

use super::{
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    txn_task::TxnTaskDescription,
};
use crate::{
    spinning_task::SpinningTaskDescription,
    test_launcher::{Network, ResourceGenerators, TestLauncher},
    view_sync_task::ViewSyncTaskDescription,
};

pub type TransactionValidator = Arc<dyn Fn(&Vec<(u64, u64)>) -> Result<()> + Send + Sync>;

/// data describing how a round should be timed.
#[derive(Clone, Debug, Copy)]
pub struct TimingData {
    /// Base duration for next-view timeout, in milliseconds
    pub next_view_timeout: u64,
    /// The exponential backoff ration for the next-view timeout
    pub timeout_ratio: (u64, u64),
    /// The delay a leader inserts before starting pre-commit, in milliseconds
    pub round_start_delay: u64,
    /// Delay after init before starting consensus, in milliseconds
    pub start_delay: u64,
    /// The maximum amount of time a leader can wait to get a block from a builder
    pub builder_timeout: Duration,
    /// time to wait until we request data associated with a proposal
    pub data_request_delay: Duration,
    /// Delay before sending through the secondary network in CombinedNetworks
    pub secondary_network_delay: Duration,
    /// view sync timeout
    pub view_sync_timeout: Duration,
}

/// metadata describing a test
#[derive(Clone)]
pub struct TestDescription<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> {
    /// Total number of staked nodes in the test
    pub num_nodes_with_stake: usize,
    /// nodes available at start
    pub start_nodes: usize,
    /// Whether to skip initializing nodes that will start late, which will catch up later with
    /// `HotShotInitializer::from_reload` in the spinning task.
    pub skip_late: bool,
    /// number of bootstrap nodes (libp2p usage only)
    pub num_bootstrap_nodes: usize,
    /// Size of the staked DA committee for the test
    pub da_staked_committee_size: usize,
    /// overall safety property description
    pub overall_safety_properties: OverallSafetyPropertiesDescription<TYPES>,
    /// spinning properties
    pub spinning_properties: SpinningTaskDescription,
    /// txns timing
    pub txn_description: TxnTaskDescription,
    /// completion task
    pub completion_task_description: CompletionTaskDescription,
    /// timing data
    pub timing_data: TimingData,
    /// unrelabile networking metadata
    pub unreliable_network: Option<Box<dyn NetworkReliability>>,
    /// view sync check task
    pub view_sync_properties: ViewSyncTaskDescription,
    /// description of builders to run
    pub builders: Vec1<BuilderDescription>,
    /// description of fallback builder to run
    pub fallback_builder: BuilderDescription,
    /// description of the solver to run
    pub solver: FakeSolverApiDescription,
    /// nodes with byzantine behaviour
    pub behaviour: Rc<dyn Fn(u64) -> Behaviour<TYPES, I, V>>,
    /// Delay config if any to add delays to asynchronous calls
    pub async_delay_config: DelayConfig,
    /// view in which to propose an upgrade
    pub upgrade_view: Option<u64>,
    /// whether to initialize the solver on startup
    pub start_solver: bool,
    /// boxed closure used to validate the resulting transactions
    pub validate_transactions: TransactionValidator,
}

pub fn nonempty_block_threshold(threshold: (u64, u64)) -> TransactionValidator {
    Arc::new(move |transactions| {
        if matches!(threshold, (0, _)) {
            return Ok(());
        }

        let blocks: Vec<_> = transactions.iter().filter(|(view, _)| *view != 0).collect();

        let num_blocks = blocks.len() as u64;
        let mut num_nonempty_blocks = 0;

        ensure!(num_blocks > 0, "Failed to commit any non-genesis blocks");

        for (_, num_transactions) in blocks {
            if *num_transactions > 0 {
                num_nonempty_blocks += 1;
            }
        }

        ensure!(
          // i.e. num_nonempty_blocks / num_blocks >= threshold.0 / threshold.1
          num_nonempty_blocks * threshold.1 >= threshold.0 * num_blocks,
          "Failed to meet nonempty block threshold of {}/{}; got {num_nonempty_blocks} nonempty blocks out of a total of {num_blocks}", threshold.0, threshold.1
        );

        Ok(())
    })
}

pub fn nonempty_block_limit(limit: (u64, u64)) -> TransactionValidator {
    Arc::new(move |transactions| {
        if matches!(limit, (_, 0)) {
            return Ok(());
        }

        let blocks: Vec<_> = transactions.iter().filter(|(view, _)| *view != 0).collect();

        let num_blocks = blocks.len() as u64;
        let mut num_nonempty_blocks = 0;

        ensure!(num_blocks > 0, "Failed to commit any non-genesis blocks");

        for (_, num_transactions) in blocks {
            if *num_transactions > 0 {
                num_nonempty_blocks += 1;
            }
        }

        ensure!(
          // i.e. num_nonempty_blocks / num_blocks <= limit.0 / limit.1
          num_nonempty_blocks * limit.1 <= limit.0 * num_blocks,
          "Exceeded nonempty block limit of {}/{}; got {num_nonempty_blocks} nonempty blocks out of a total of {num_blocks}", limit.0, limit.1
        );

        Ok(())
    })
}

#[derive(Debug)]
pub enum Behaviour<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> {
    ByzantineTwins(Box<dyn TwinsHandlerState<TYPES, I, V>>),
    Byzantine(Box<dyn EventTransformerState<TYPES, I, V>>),
    Standard,
}

pub async fn create_test_handle<
    TYPES: NodeType<InstanceState = TestInstanceState>,
    I: NodeImplementation<TYPES>,
    V: Versions,
>(
    metadata: TestDescription<TYPES, I, V>,
    node_id: u64,
    network: Network<TYPES, I>,
    memberships: Memberships<TYPES>,
    config: HotShotConfig<TYPES::SignatureKey>,
    storage: I::Storage,
    marketplace_config: MarketplaceConfig<TYPES, I>,
) -> SystemContextHandle<TYPES, I, V> {
    let initializer = HotShotInitializer::<TYPES>::from_genesis::<V>(TestInstanceState::new(
        metadata.async_delay_config,
    ))
    .await
    .unwrap();

    // See whether or not we should be DA
    let is_da = node_id < config.da_staked_committee_size as u64;

    let validator_config: ValidatorConfig<TYPES::SignatureKey> =
        ValidatorConfig::generated_from_seed_indexed([0u8; 32], node_id, 1, is_da);

    // Get key pair for certificate aggregation
    let private_key = validator_config.private_key.clone();
    let public_key = validator_config.public_key.clone();

    let behaviour = (metadata.behaviour)(node_id);
    match behaviour {
        Behaviour::ByzantineTwins(state) => {
            let state = Box::leak(state);
            let (left_handle, _right_handle) = state
                .spawn_twin_handles(
                    public_key,
                    private_key,
                    node_id,
                    config,
                    memberships,
                    network,
                    initializer,
                    ConsensusMetricsValue::default(),
                    storage,
                    marketplace_config,
                )
                .await;

            left_handle
        }
        Behaviour::Byzantine(state) => {
            let state = Box::leak(state);
            state
                .spawn_handle(
                    public_key,
                    private_key,
                    node_id,
                    config,
                    memberships,
                    network,
                    initializer,
                    ConsensusMetricsValue::default(),
                    storage,
                    marketplace_config,
                )
                .await
        }
        Behaviour::Standard => {
            let hotshot = SystemContext::<TYPES, I, V>::new(
                public_key,
                private_key,
                node_id,
                config,
                memberships,
                network,
                initializer,
                ConsensusMetricsValue::default(),
                storage,
                marketplace_config,
            )
            .await;

            hotshot.run_tasks().await
        }
    }
}

/// Describes a possible change to builder status during test
#[derive(Clone, Debug)]
pub enum BuilderChange {
    // Builder should start up
    Up,
    // Builder should shut down completely
    Down,
    // Toggles whether builder should always respond
    // to claim calls with errors
    FailClaims(bool),
}

/// Metadata describing builder behaviour during a test
#[derive(Clone, Debug, Default)]
pub struct BuilderDescription {
    /// view number -> change to builder status
    pub changes: HashMap<u64, BuilderChange>,
}

#[derive(Clone, Debug)]
pub struct FakeSolverApiDescription {
    /// The rate at which errors occur in the mock solver API
    pub error_pct: f32,
}

impl Default for TimingData {
    fn default() -> Self {
        Self {
            next_view_timeout: 4000,
            timeout_ratio: (11, 10),
            round_start_delay: 100,
            start_delay: 100,
            builder_timeout: Duration::from_millis(500),
            data_request_delay: Duration::from_millis(200),
            secondary_network_delay: Duration::from_millis(1000),
            view_sync_timeout: Duration::from_millis(2000),
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> TestDescription<TYPES, I, V> {
    /// the default metadata for a stress test
    #[must_use]
    #[allow(clippy::redundant_field_names)]
    pub fn default_stress() -> Self {
        let num_nodes_with_stake = 100;

        Self {
            num_bootstrap_nodes: num_nodes_with_stake,
            num_nodes_with_stake,
            start_nodes: num_nodes_with_stake,
            overall_safety_properties: OverallSafetyPropertiesDescription::<TYPES> {
                num_successful_views: 50,
                check_leaf: true,
                check_block: true,
                num_failed_views: 15,
                transaction_threshold: 0,
                threshold_calculator: Arc::new(|_active, total| (2 * total / 3 + 1)),
                expected_views_to_fail: HashMap::new(),
            },
            timing_data: TimingData {
                next_view_timeout: 2000,
                timeout_ratio: (1, 1),
                start_delay: 20000,
                round_start_delay: 25,
                ..TimingData::default()
            },
            view_sync_properties: ViewSyncTaskDescription::Threshold(0, num_nodes_with_stake),
            ..Self::default()
        }
    }

    /// the default metadata for multiple rounds
    #[must_use]
    #[allow(clippy::redundant_field_names)]
    pub fn default_multiple_rounds() -> Self {
        let num_nodes_with_stake = 10;
        TestDescription::<TYPES, I, V> {
            // TODO: remove once we have fixed the DHT timeout issue
            // https://github.com/EspressoSystems/HotShot/issues/2088
            num_bootstrap_nodes: num_nodes_with_stake,
            num_nodes_with_stake,
            start_nodes: num_nodes_with_stake,
            overall_safety_properties: OverallSafetyPropertiesDescription::<TYPES> {
                num_successful_views: 20,
                check_leaf: true,
                check_block: true,
                num_failed_views: 8,
                transaction_threshold: 0,
                threshold_calculator: Arc::new(|_active, total| (2 * total / 3 + 1)),
                expected_views_to_fail: HashMap::new(),
            },
            timing_data: TimingData {
                start_delay: 120_000,
                round_start_delay: 25,
                ..TimingData::default()
            },
            view_sync_properties: ViewSyncTaskDescription::Threshold(0, num_nodes_with_stake),
            ..TestDescription::<TYPES, I, V>::default()
        }
    }

    /// Default setting with 20 nodes and 8 views of successful views.
    #[must_use]
    #[allow(clippy::redundant_field_names)]
    pub fn default_more_nodes() -> Self {
        let num_nodes_with_stake = 20;
        Self {
            num_nodes_with_stake,
            start_nodes: num_nodes_with_stake,
            num_bootstrap_nodes: num_nodes_with_stake,
            // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
            // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
            // following issue.
            // TODO: Update message broadcasting to avoid hanging
            // <https://github.com/EspressoSystems/HotShot/issues/1567>
            da_staked_committee_size: 14,
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                TimeBasedCompletionTaskDescription {
                    // Increase the duration to get the expected number of successful views.
                    duration: Duration::new(340, 0),
                },
            ),
            overall_safety_properties: OverallSafetyPropertiesDescription {
                ..Default::default()
            },
            timing_data: TimingData {
                next_view_timeout: 5000,
                ..TimingData::default()
            },
            view_sync_properties: ViewSyncTaskDescription::Threshold(0, num_nodes_with_stake),
            ..Self::default()
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> Default
    for TestDescription<TYPES, I, V>
{
    /// by default, just a single round
    #[allow(clippy::redundant_field_names)]
    fn default() -> Self {
        let num_nodes_with_stake = 6;
        Self {
            timing_data: TimingData::default(),
            num_nodes_with_stake,
            start_nodes: num_nodes_with_stake,
            skip_late: false,
            num_bootstrap_nodes: num_nodes_with_stake,
            da_staked_committee_size: num_nodes_with_stake,
            spinning_properties: SpinningTaskDescription {
                node_changes: vec![],
            },
            overall_safety_properties: OverallSafetyPropertiesDescription::default(),
            // arbitrary, haven't done the math on this
            txn_description: TxnTaskDescription::RoundRobinTimeBased(Duration::from_millis(100)),
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                TimeBasedCompletionTaskDescription {
                    // TODO ED Put a configurable time here - 10 seconds for now
                    duration: Duration::from_millis(10000),
                },
            ),
            unreliable_network: None,
            view_sync_properties: ViewSyncTaskDescription::Threshold(0, num_nodes_with_stake),
            builders: vec1::vec1![BuilderDescription::default(), BuilderDescription::default(),],
            fallback_builder: BuilderDescription::default(),
            solver: FakeSolverApiDescription {
                // Default to a 10% error rate.
                error_pct: 0.1,
            },
            behaviour: Rc::new(|_| Behaviour::Standard),
            async_delay_config: DelayConfig::default(),
            upgrade_view: None,
            start_solver: true,
            validate_transactions: Arc::new(|_| Ok(())),
        }
    }
}

impl<
        TYPES: NodeType<InstanceState = TestInstanceState>,
        I: TestableNodeImplementation<TYPES>,
        V: Versions,
    > TestDescription<TYPES, I, V>
where
    I: NodeImplementation<TYPES, AuctionResultsProvider = TestAuctionResultsProvider<TYPES>>,
{
    /// turn a description of a test (e.g. a [`TestDescription`]) into
    /// a [`TestLauncher`] that can be used to launch the test.
    /// # Panics
    /// if some of the the configuration values are zero
    #[must_use]
    pub fn gen_launcher(self, node_id: u64) -> TestLauncher<TYPES, I, V> {
        let TestDescription {
            num_nodes_with_stake,
            num_bootstrap_nodes,
            timing_data,
            da_staked_committee_size,
            unreliable_network,
            ..
        } = self.clone();

        let mut known_da_nodes = Vec::new();

        // We assign known_nodes' public key and stake value here rather than read from config file since it's a test.
        let known_nodes_with_stake = (0..num_nodes_with_stake)
            .map(|node_id_| {
                let cur_validator_config: ValidatorConfig<TYPES::SignatureKey> =
                    ValidatorConfig::generated_from_seed_indexed(
                        [0u8; 32],
                        node_id_ as u64,
                        1,
                        node_id_ < da_staked_committee_size,
                    );

                // Add the node to the known DA nodes based on the index (for tests)
                if node_id_ < da_staked_committee_size {
                    known_da_nodes.push(cur_validator_config.public_config());
                }

                cur_validator_config.public_config()
            })
            .collect();
        // But now to test validator's config, we input the info of my_own_validator from config file when node_id == 0.
        let my_own_validator_config = ValidatorConfig::generated_from_seed_indexed(
            [0u8; 32],
            node_id,
            1,
            // This is the config for node 0
            0 < da_staked_committee_size,
        );
        // let da_committee_nodes = known_nodes[0..da_committee_size].to_vec();
        let config = HotShotConfig {
            // TODO this doesn't exist anymore
            execution_type: ExecutionType::Incremental,
            start_threshold: (1, 1),
            num_nodes_with_stake: NonZeroUsize::new(num_nodes_with_stake).unwrap(),
            // Currently making this zero for simplicity
            known_da_nodes,
            num_bootstrap: num_bootstrap_nodes,
            known_nodes_with_stake,
            known_nodes_without_stake: vec![],
            my_own_validator_config,
            da_staked_committee_size,
            fixed_leader_for_gpuvid: 1,
            next_view_timeout: 500,
            view_sync_timeout: Duration::from_millis(250),
            timeout_ratio: (11, 10),
            round_start_delay: 25,
            start_delay: 1,
            builder_timeout: Duration::from_millis(1000),
            data_request_delay: Duration::from_millis(200),
            // Placeholder until we spin up the builder
            builder_urls: vec1::vec1![Url::parse("http://localhost:9999").expect("Valid URL")],
            start_proposing_view: u64::MAX,
            stop_proposing_view: 0,
            start_voting_view: u64::MAX,
            stop_voting_view: 0,
            start_proposing_time: u64::MAX,
            stop_proposing_time: 0,
            start_voting_time: u64::MAX,
            stop_voting_time: 0,
        };
        let TimingData {
            next_view_timeout,
            timeout_ratio,
            round_start_delay,
            start_delay,
            builder_timeout,
            data_request_delay,
            secondary_network_delay,
            view_sync_timeout,
        } = timing_data;
        let mod_config =
            // TODO this should really be using the timing config struct
            |a: &mut HotShotConfig<TYPES::SignatureKey>| {
                a.next_view_timeout = next_view_timeout;
                a.timeout_ratio = timeout_ratio;
                a.round_start_delay = round_start_delay;
                a.start_delay = start_delay;
                a.builder_timeout = builder_timeout;
                a.data_request_delay = data_request_delay;
                a.view_sync_timeout = view_sync_timeout;
            };

        let metadata = self.clone();
        TestLauncher {
            resource_generator: ResourceGenerators {
                channel_generator: <I as TestableNodeImplementation<TYPES>>::gen_networks(
                    num_nodes_with_stake,
                    num_bootstrap_nodes,
                    da_staked_committee_size,
                    unreliable_network,
                    secondary_network_delay,
                ),
                storage: Box::new(move |_| {
                    let mut storage = TestStorage::<TYPES>::default();
                    // update storage impl to use settings delay option
                    storage.delay_config = metadata.async_delay_config.clone();
                    storage
                }),
                config,
                marketplace_config: Box::new(|_| MarketplaceConfig::<TYPES, I> {
                    auction_results_provider: TestAuctionResultsProvider::<TYPES>::default().into(),
                    fallback_builder_url: Url::parse("http://localhost:9999").unwrap(),
                }),
            },
            metadata: self,
        }
        .modify_default_config(mod_config)
    }
}
