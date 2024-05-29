#![allow(clippy::panic)]
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};

use async_broadcast::broadcast;
use async_lock::RwLock;
use futures::future::{
    join_all, Either,
    Either::{Left, Right},
};
use hotshot::{
    traits::TestableNodeImplementation, types::SystemContextHandle, HotShotInitializer,
    Memberships, SystemContext,
};
use hotshot_example_types::{
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    constants::EVENT_CHANNEL_SIZE,
    data::Leaf,
    message::Message,
    simple_certificate::QuorumCertificate,
    traits::{
        election::Membership,
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
    HotShotConfig, ValidatorConfig,
};
#[allow(deprecated)]
use tracing::info;

use super::{
    completion_task::CompletionTask,
    overall_safety_task::{OverallSafetyTask, RoundCtx},
    txn_task::TxnTask,
};
use crate::{
    block_builder::TestBuilderImplementation,
    completion_task::CompletionTaskDescription,
    spinning_task::{ChangeNode, SpinningTask, UpDown},
    test_launcher::{Networks, TestLauncher},
    test_task::{TestResult, TestTask},
    txn_task::TxnTaskDescription,
    view_sync_task::ViewSyncTask,
};

pub trait TaskErr: std::error::Error + Sync + Send + 'static {}
impl<T: std::error::Error + Sync + Send + 'static> TaskErr for T {}

impl<
        TYPES: NodeType<InstanceState = TestInstanceState, ValidatedState = TestValidatedState>,
        I: TestableNodeImplementation<TYPES>,
        N: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
    > TestRunner<TYPES, I, N>
where
    I: TestableNodeImplementation<TYPES>,
    I: NodeImplementation<TYPES, QuorumNetwork = N, DaNetwork = N, Storage = TestStorage<TYPES>>,
{
    /// execute test
    ///
    /// # Panics
    /// if the test fails
    #[allow(clippy::too_many_lines)]
    pub async fn run_test<B: TestBuilderImplementation<TYPES>>(mut self) {
        let (test_sender, test_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        let spinning_changes = self
            .launcher
            .metadata
            .spinning_properties
            .node_changes
            .clone();

        let mut late_start_nodes: HashSet<u64> = HashSet::new();
        for (_, changes) in &spinning_changes {
            for change in changes {
                if matches!(change.updown, UpDown::Up) {
                    late_start_nodes.insert(change.idx.try_into().unwrap());
                }
            }
        }

        self.add_nodes::<B>(
            self.launcher.metadata.num_nodes_with_stake,
            &late_start_nodes,
        )
        .await;
        let mut event_rxs = vec![];
        let mut internal_event_rxs = vec![];

        for node in &self.nodes {
            let r = node.handle.event_stream_known_impl();
            event_rxs.push(r);
        }
        for node in &self.nodes {
            let r = node.handle.internal_event_stream_known_impl();
            internal_event_rxs.push(r);
        }

        let TestRunner {
            ref launcher,
            nodes,
            late_start,
            next_node_id: _,
            _pd: _,
        } = self;

        let mut task_futs = vec![];
        let meta = launcher.metadata.clone();

        let handles = Arc::new(RwLock::new(nodes));

        let txn_task =
            if let TxnTaskDescription::RoundRobinTimeBased(duration) = meta.txn_description {
                let txn_task = TxnTask {
                    handles: Arc::clone(&handles),
                    next_node_idx: Some(0),
                    duration,
                    shutdown_chan: test_receiver.clone(),
                };
                Some(txn_task)
            } else {
                None
            };

        // add completion task
        let CompletionTaskDescription::TimeBasedCompletionTaskBuilder(time_based) =
            meta.completion_task_description;
        let completion_task = CompletionTask {
            tx: test_sender.clone(),
            rx: test_receiver.clone(),
            handles: Arc::clone(&handles),
            duration: time_based.duration,
        };

        // add spinning task
        // map spinning to view
        let mut changes: BTreeMap<TYPES::Time, Vec<ChangeNode>> = BTreeMap::new();
        for (view, mut change) in spinning_changes {
            changes
                .entry(TYPES::Time::new(view))
                .or_insert_with(Vec::new)
                .append(&mut change);
        }

        let spinning_task_state = SpinningTask {
            handles: Arc::clone(&handles),
            late_start,
            latest_view: None,
            changes,
            last_decided_leaf: Leaf::genesis(&TestValidatedState::default(), &TestInstanceState {})
                .await,
            high_qc: QuorumCertificate::genesis(
                &TestValidatedState::default(),
                &TestInstanceState {},
            )
            .await,
        };
        let spinning_task = TestTask::<SpinningTask<TYPES, I>>::new(
            spinning_task_state,
            event_rxs.clone(),
            test_receiver.clone(),
        );
        // add safety task
        let overall_safety_task_state = OverallSafetyTask {
            handles: Arc::clone(&handles),
            ctx: RoundCtx::default(),
            properties: self.launcher.metadata.overall_safety_properties,
            error: None,
            test_sender,
        };

        let safety_task = TestTask::<OverallSafetyTask<TYPES, I>>::new(
            overall_safety_task_state,
            event_rxs.clone(),
            test_receiver.clone(),
        );

        // add view sync task
        let view_sync_task_state = ViewSyncTask {
            hit_view_sync: HashSet::new(),
            description: self.launcher.metadata.view_sync_properties,
            _pd: PhantomData,
        };

        let view_sync_task = TestTask::<ViewSyncTask<TYPES, I>>::new(
            view_sync_task_state,
            internal_event_rxs,
            test_receiver.clone(),
        );

        let nodes = handles.read().await;

        // wait for networks to be ready
        for node in &*nodes {
            node.networks.0.wait_for_ready().await;
            node.networks.1.wait_for_ready().await;
        }

        // Start hotshot
        for node in &*nodes {
            if !late_start_nodes.contains(&node.node_id) {
                node.handle.hotshot.start_consensus().await;
            }
        }

        drop(nodes);

        task_futs.push(safety_task.run());
        task_futs.push(view_sync_task.run());
        task_futs.push(spinning_task.run());

        // `generator` tasks that do not process events.
        let txn_handle = txn_task.map(|txn| txn.run());
        let completion_handle = completion_task.run();

        let mut error_list = vec![];

        #[cfg(async_executor_impl = "async-std")]
        {
            let results = join_all(task_futs).await;
            tracing::error!("test tasks joined");
            for result in results {
                match result {
                    TestResult::Pass => {
                        info!("Task shut down successfully");
                    }
                    TestResult::Fail(e) => error_list.push(e),
                }
            }
            if let Some(handle) = txn_handle {
                handle.cancel().await;
            }
            completion_handle.cancel().await;
        }

        #[cfg(async_executor_impl = "tokio")]
        {
            let results = join_all(task_futs).await;

            tracing::error!("test tasks joined");
            for result in results {
                match result {
                    Ok(res) => match res {
                        TestResult::Pass => {
                            info!("Task shut down successfully");
                        }
                        TestResult::Fail(e) => error_list.push(e),
                    },
                    Err(e) => {
                        tracing::error!("Error Joining the test task {:?}", e);
                    }
                }
            }

            if let Some(handle) = txn_handle {
                handle.abort();
            }
            completion_handle.abort();
        }

        assert!(
            error_list.is_empty(),
            "TEST FAILED! Results: {error_list:?}"
        );

        let mut nodes = handles.write().await;

        for node in &mut *nodes {
            node.handle.shut_down().await;
        }
    }

    /// Add nodes.
    ///
    /// # Panics
    /// Panics if unable to create a [`HotShotInitializer`]
    pub async fn add_nodes<B: TestBuilderImplementation<TYPES>>(
        &mut self,
        total: usize,
        late_start: &HashSet<u64>,
    ) -> Vec<u64> {
        let mut results = vec![];
        let config = self.launcher.resource_generator.config.clone();
        let known_nodes_with_stake = config.known_nodes_with_stake.clone();

        let (mut builder_task, builder_url) =
            B::start(config.num_nodes_with_stake.into(), B::Config::default()).await;

        // Collect uninitialized nodes because we need to wait for all networks to be ready before starting the tasks
        let mut uninitialized_nodes = Vec::new();
        let mut networks_ready = Vec::new();

        for i in 0..total {
            let mut config = config.clone();
            let node_id = self.next_node_id;
            self.next_node_id += 1;
            tracing::debug!("launch node {}", i);

            let memberships = Memberships {
                quorum_membership: <TYPES as NodeType>::Membership::create_election(
                    known_nodes_with_stake.clone(),
                    known_nodes_with_stake.clone(),
                    config.fixed_leader_for_gpuvid,
                ),
                da_membership: <TYPES as NodeType>::Membership::create_election(
                    known_nodes_with_stake.clone(),
                    config.known_da_nodes.clone(),
                    config.fixed_leader_for_gpuvid,
                ),
                vid_membership: <TYPES as NodeType>::Membership::create_election(
                    known_nodes_with_stake.clone(),
                    known_nodes_with_stake.clone(),
                    config.fixed_leader_for_gpuvid,
                ),
                view_sync_membership: <TYPES as NodeType>::Membership::create_election(
                    known_nodes_with_stake.clone(),
                    known_nodes_with_stake.clone(),
                    config.fixed_leader_for_gpuvid,
                ),
            };
            config.builder_url = builder_url.clone();

            let networks = (self.launcher.resource_generator.channel_generator)(node_id).await;
            let storage = (self.launcher.resource_generator.storage)(node_id);

            let network0 = networks.0.clone();
            let network1 = networks.1.clone();
            let networks_ready_future = async move {
                network0.wait_for_ready().await;
                network1.wait_for_ready().await;
            };

            networks_ready.push(networks_ready_future);

            if self.launcher.metadata.skip_late && late_start.contains(&node_id) {
                self.late_start.insert(
                    node_id,
                    LateStartNode {
                        networks,
                        context: Right((storage, memberships, config)),
                    },
                );
            } else {
                let initializer = HotShotInitializer::<TYPES>::from_genesis(TestInstanceState {})
                    .await
                    .unwrap();

                // See whether or not we should be DA
                let is_da = node_id < config.da_staked_committee_size as u64;

                // We assign node's public key and stake value rather than read from config file since it's a test
                let validator_config =
                    ValidatorConfig::generated_from_seed_indexed([0u8; 32], node_id, 1, is_da);
                let hotshot = Self::add_node_with_config(
                    node_id,
                    networks.clone(),
                    memberships,
                    initializer,
                    config,
                    validator_config,
                    storage,
                )
                .await;
                if late_start.contains(&node_id) {
                    self.late_start.insert(
                        node_id,
                        LateStartNode {
                            networks,
                            context: Left(hotshot),
                        },
                    );
                } else {
                    uninitialized_nodes.push((node_id, networks, hotshot));
                }
            }

            results.push(node_id);
        }

        // Wait for all networks to be ready
        join_all(networks_ready).await;

        // Then start the necessary tasks
        for (node_id, networks, hotshot) in uninitialized_nodes {
            let handle = hotshot.run_tasks().await;
            if node_id == 1 {
                if let Some(task) = builder_task.take() {
                    task.start(Box::new(handle.event_stream()))
                }
            }

            self.nodes.push(Node {
                node_id,
                networks,
                handle,
            });
        }

        results
    }

    /// add a specific node with a config
    /// # Panics
    /// if unable to initialize the node's `SystemContext` based on the config
    pub async fn add_node_with_config(
        node_id: u64,
        networks: Networks<TYPES, I>,
        memberships: Memberships<TYPES>,
        initializer: HotShotInitializer<TYPES>,
        config: HotShotConfig<TYPES::SignatureKey>,
        validator_config: ValidatorConfig<TYPES::SignatureKey>,
        storage: I::Storage,
    ) -> Arc<SystemContext<TYPES, I>> {
        // Get key pair for certificate aggregation
        let private_key = validator_config.private_key.clone();
        let public_key = validator_config.public_key.clone();

        let network_bundle = hotshot::Networks {
            quorum_network: networks.0.clone(),
            da_network: networks.1.clone(),
            _pd: PhantomData,
        };

        SystemContext::new(
            public_key,
            private_key,
            node_id,
            config,
            memberships,
            network_bundle,
            initializer,
            ConsensusMetricsValue::default(),
            storage,
        )
        .await
        .expect("Could not init hotshot")
    }
}

/// a node participating in a test
pub struct Node<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// The node's unique identifier
    pub node_id: u64,
    /// The underlying networks belonging to the node
    pub networks: Networks<TYPES, I>,
    /// The handle to the node's internals
    pub handle: SystemContextHandle<TYPES, I>,
}

/// Either the node context or the parameters to construct the context for nodes that start late.
pub type LateNodeContext<TYPES, I> = Either<
    Arc<SystemContext<TYPES, I>>,
    (
        <I as NodeImplementation<TYPES>>::Storage,
        Memberships<TYPES>,
        HotShotConfig<<TYPES as NodeType>::SignatureKey>,
    ),
>;

/// A yet-to-be-started node that participates in tests
pub struct LateStartNode<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// The underlying networks belonging to the node
    pub networks: Networks<TYPES, I>,
    /// Either the context to which we will use to launch HotShot for initialized node when it's
    /// time, or the parameters that will be used to initialize the node and launch HotShot.
    pub context: LateNodeContext<TYPES, I>,
}

/// The runner of a test network
/// spin up and down nodes, execute rounds
pub struct TestRunner<
    TYPES: NodeType,
    I: TestableNodeImplementation<TYPES>,
    N: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
> {
    /// test launcher, contains a bunch of useful metadata and closures
    pub(crate) launcher: TestLauncher<TYPES, I>,
    /// nodes in the test
    pub(crate) nodes: Vec<Node<TYPES, I>>,
    /// nodes with a late start
    pub(crate) late_start: HashMap<u64, LateStartNode<TYPES, I>>,
    /// the next node unique identifier
    pub(crate) next_node_id: u64,
    /// Phantom for N
    pub(crate) _pd: PhantomData<N>,
}
