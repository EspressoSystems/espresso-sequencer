#![allow(clippy::panic)]
use super::{
    completion_task::CompletionTask,
    overall_safety_task::{OverallSafetyTask, RoundCtx},
    txn_task::TxnTask,
};
use crate::{
    spinning_task::{ChangeNode, UpDown},
    test_launcher::{Networks, TestLauncher},
};
use hotshot::{types::SystemContextHandle, Memberships};

use hotshot::{traits::TestableNodeImplementation, HotShotInitializer, SystemContext};
use hotshot_task::{
    event_stream::ChannelStream, global_registry::GlobalRegistry, task_launcher::TaskRunner,
};
use hotshot_types::traits::network::CommunicationChannel;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    traits::{election::Membership, node_implementation::NodeType, state::ConsensusTime},
    HotShotConfig, ValidatorConfig,
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

#[allow(deprecated)]
use tracing::info;

/// a node participating in a test
#[derive(Clone)]
pub struct Node<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// the unique identifier of the node
    pub node_id: u64,
    /// the networks of the node
    pub networks: Networks<TYPES, I>,
    /// the handle to the node's internals
    pub handle: SystemContextHandle<TYPES, I>,
}

/// The runner of a test network
/// spin up and down nodes, execute rounds
pub struct TestRunner<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// test launcher, contains a bunch of useful metadata and closures
    pub(crate) launcher: TestLauncher<TYPES, I>,
    /// nodes in the test
    pub(crate) nodes: Vec<Node<TYPES, I>>,
    /// nodes with a late start
    pub(crate) late_start: HashMap<u64, SystemContext<TYPES, I>>,
    /// the next node unique identifier
    pub(crate) next_node_id: u64,
    /// overarching test task
    pub(crate) task_runner: TaskRunner,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestRunner<TYPES, I>
where
    I: TestableNodeImplementation<TYPES, CommitteeElectionConfig = TYPES::ElectionConfigType>,
{
    /// excecute test
    /// # Panics
    /// if the test fails
    #[allow(clippy::too_many_lines)]
    pub async fn run_test(mut self) {
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

        self.add_nodes(self.launcher.metadata.total_nodes, &late_start_nodes)
            .await;

        let TestRunner {
            launcher,
            nodes,
            late_start,
            next_node_id: _,
            mut task_runner,
        } = self;
        let registry = GlobalRegistry::default();
        let test_event_stream = ChannelStream::new();

        // add transaction task
        let txn_task_state = TxnTask {
            handles: nodes.clone(),
            next_node_idx: Some(0),
        };
        let (id, task) = (launcher.txn_task_generator)(
            txn_task_state,
            registry.clone(),
            test_event_stream.clone(),
        )
        .await;
        task_runner =
            task_runner.add_task(id, "Test Transaction Submission Task".to_string(), task);

        // add completion task
        let completion_task_state = CompletionTask {
            handles: nodes.clone(),
            test_event_stream: test_event_stream.clone(),
        };
        let (id, task) = (launcher.completion_task_generator)(
            completion_task_state,
            registry.clone(),
            test_event_stream.clone(),
        )
        .await;

        task_runner = task_runner.add_task(id, "Test Completion Task".to_string(), task);

        // add spinning task
        // map spinning to view
        let mut changes: HashMap<TYPES::Time, Vec<ChangeNode>> = HashMap::new();
        for (view, mut change) in spinning_changes {
            changes
                .entry(TYPES::Time::new(view))
                .or_insert_with(Vec::new)
                .append(&mut change);
        }

        let spinning_task_state = crate::spinning_task::SpinningTask {
            handles: nodes.clone(),
            late_start,
            latest_view: None,
            changes,
        };

        let (id, task) = (launcher.spinning_task_generator)(
            spinning_task_state,
            registry.clone(),
            test_event_stream.clone(),
        )
        .await;
        task_runner = task_runner.add_task(id, "Test Spinning Task".to_string(), task);

        // add safety task
        let overall_safety_task_state = OverallSafetyTask {
            handles: nodes.clone(),
            ctx: RoundCtx::default(),
            test_event_stream: test_event_stream.clone(),
        };
        let (id, task) = (launcher.overall_safety_task_generator)(
            overall_safety_task_state,
            registry.clone(),
            test_event_stream.clone(),
        )
        .await;
        task_runner = task_runner.add_task(id, "Test Overall Safety Task".to_string(), task);

        // wait for networks to be ready
        for node in &nodes {
            node.networks.0.wait_for_ready().await;
        }

        // Start hotshot
        for node in nodes {
            if !late_start_nodes.contains(&node.node_id) {
                node.handle.hotshot.start_consensus().await;
            }
        }

        let results = task_runner.launch().await;

        let mut error_list = vec![];
        for (name, result) in results {
            match result {
                hotshot_task::task::HotShotTaskCompleted::ShutDown => {
                    info!("Task {} shut down successfully", name);
                }
                hotshot_task::task::HotShotTaskCompleted::Error(e) => error_list.push((name, e)),
                _ => {
                    panic!("Future impl for task abstraction failed! This should never happen");
                }
            }
        }
        assert!(
            error_list.is_empty(),
            "TEST FAILED! Results: {error_list:?}"
        );
    }

    /// add nodes
    /// # Panics
    /// Panics if unable to create a [`HotShotInitializer`]
    pub async fn add_nodes(&mut self, total: usize, late_start: &HashSet<u64>) -> Vec<u64> {
        let mut results = vec![];
        for i in 0..total {
            tracing::debug!("launch node {}", i);
            let node_id = self.next_node_id;
            let storage = (self.launcher.resource_generator.storage)(node_id);
            let config = self.launcher.resource_generator.config.clone();
            let initializer = HotShotInitializer::<TYPES>::from_genesis().unwrap();
            let networks = (self.launcher.resource_generator.channel_generator)(node_id);
            // We assign node's public key and stake value rather than read from config file since it's a test
            let validator_config =
                ValidatorConfig::generated_from_seed_indexed([0u8; 32], node_id, 1);
            let hotshot = self
                .add_node_with_config(
                    networks.clone(),
                    storage,
                    initializer,
                    config,
                    validator_config,
                )
                .await;
            if late_start.contains(&node_id) {
                self.late_start.insert(node_id, hotshot);
            } else {
                self.nodes.push(Node {
                    node_id,
                    networks,
                    handle: hotshot.run_tasks().await,
                });
            }
            results.push(node_id);
        }

        results
    }

    /// add a specific node with a config
    /// # Panics
    /// if unable to initialize the node's `SystemContext` based on the config
    pub async fn add_node_with_config(
        &mut self,
        networks: Networks<TYPES, I>,
        storage: I::Storage,
        initializer: HotShotInitializer<TYPES>,
        config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
        validator_config: ValidatorConfig<TYPES::SignatureKey>,
    ) -> SystemContext<TYPES, I> {
        let node_id = self.next_node_id;
        self.next_node_id += 1;
        let known_nodes_with_stake = config.known_nodes_with_stake.clone();
        // Get key pair for certificate aggregation
        let private_key = validator_config.private_key.clone();
        let public_key = validator_config.public_key.clone();
        let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
            TYPES::Membership::default_election_config(config.total_nodes.get() as u64)
        });
        let committee_election_config = I::committee_election_config_generator();
        let network_bundle = hotshot::Networks {
            quorum_network: networks.0.clone(),
            da_network: networks.1.clone(),
            _pd: PhantomData,
        };

        let memberships = Memberships {
            quorum_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config.clone(),
            ),
            da_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                committee_election_config(config.da_committee_size as u64),
            ),
            vid_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config.clone(),
            ),
            view_sync_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config,
            ),
        };

        SystemContext::new(
            public_key,
            private_key,
            node_id,
            config,
            storage,
            memberships,
            network_bundle,
            initializer,
            ConsensusMetricsValue::default(),
        )
        .await
        .expect("Could not init hotshot")
    }
}
