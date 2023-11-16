use super::{
    completion_task::CompletionTask,
    overall_safety_task::{OverallSafetyTask, RoundCtx},
    txn_task::TxnTask,
};
use crate::{
    spinning_task::UpDown,
    test_launcher::{Networks, TestLauncher},
};
use hotshot::{traits::NodeImplementation, types::SystemContextHandle};

use hotshot::{traits::TestableNodeImplementation, HotShotInitializer, HotShotType, SystemContext};
use hotshot_task::{
    event_stream::ChannelStream, global_registry::GlobalRegistry, task_launcher::TaskRunner,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    message::Message,
    traits::{
        election::{ConsensusExchange, Membership},
        network::CommunicationChannel,
        node_implementation::{
            CommitteeCommChannel, ExchangesType, NodeType, QuorumCommChannel, QuorumEx,
        },
        signature_key::SignatureKey,
    },
    HotShotConfig, ValidatorConfig,
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

#[allow(deprecated)]
use tracing::info;

#[derive(Clone)]
pub struct Node<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    pub node_id: u64,
    pub handle: SystemContextHandle<TYPES, I>,
}

/// The runner of a test network
/// spin up and down nodes, execute rounds
pub struct TestRunner<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>
where
    QuorumCommChannel<TYPES, I>: CommunicationChannel<TYPES>,
{
    pub(crate) launcher: TestLauncher<TYPES, I>,
    pub(crate) nodes: Vec<Node<TYPES, I>>,
    pub(crate) late_start: HashMap<u64, SystemContext<TYPES, I>>,
    pub(crate) next_node_id: u64,
    pub(crate) task_runner: TaskRunner,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestRunner<TYPES, I>
where
    SystemContext<TYPES, I>: HotShotType<TYPES, I>,
    QuorumCommChannel<TYPES, I>: CommunicationChannel<TYPES>,
    I: NodeImplementation<
        TYPES,
        QuorumNetwork = QuorumCommChannel<TYPES, I>,
        CommitteeNetwork = CommitteeCommChannel<TYPES, I>,
    >,
{
    /// excecute test
    pub async fn run_test(mut self)
    where
        I::Exchanges: ExchangesType<
            TYPES,
            Message<TYPES>,
            ElectionConfigs = (TYPES::ElectionConfigType, I::CommitteeElectionConfig),
        >,
    {
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
        assert!(
            late_start_nodes.len()
                <= self.launcher.metadata.total_nodes - self.launcher.metadata.start_nodes,
            "Test wants to late start too many nodes."
        );

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
        let spinning_task_state = crate::spinning_task::SpinningTask {
            handles: nodes.clone(),
            late_start,
            changes: spinning_changes.into_iter().map(|(_, b)| b).collect(),
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
                    info!("Task {} shut down successfully", name)
                }
                hotshot_task::task::HotShotTaskCompleted::Error(e) => error_list.push((name, e)),
                _ => {
                    panic!("Future impl for task abstraction failed! This should never happen");
                }
            }
        }
        if !error_list.is_empty() {
            panic!("TEST FAILED! Results: {:?}", error_list);
        }
    }

    /// add nodes
    pub async fn add_nodes(&mut self, total: usize, late_start: &HashSet<u64>) -> Vec<u64>
    where
        I::Exchanges: ExchangesType<
            TYPES,
            Message<TYPES>,
            ElectionConfigs = (TYPES::ElectionConfigType, I::CommitteeElectionConfig),
        >,
        I: NodeImplementation<
            TYPES,
            QuorumNetwork = QuorumCommChannel<TYPES, I>,
            CommitteeNetwork = CommitteeCommChannel<TYPES, I>,
        >,
    {
        let mut results = vec![];
        for i in 0..total {
            tracing::debug!("launch node {}", i);
            let node_id = self.next_node_id;
            let storage = (self.launcher.resource_generator.storage)(node_id);
            let config = self.launcher.resource_generator.config.clone();
            let initializer =
                HotShotInitializer::<TYPES>::from_genesis(I::block_genesis()).unwrap();
            let networks = (self.launcher.resource_generator.channel_generator)(node_id);
            // We assign node's public key and stake value rather than read from config file since it's a test
            let validator_config =
                ValidatorConfig::generated_from_seed_indexed([0u8; 32], node_id, 1);
            let hotshot = self
                .add_node_with_config(networks, storage, initializer, config, validator_config)
                .await;
            if late_start.contains(&node_id) {
                self.late_start.insert(node_id, hotshot);
            } else {
                self.nodes.push(Node {
                    node_id,
                    handle: hotshot.run_tasks().await,
                });
            }
            results.push(node_id);
        }

        results
    }

    /// add a specific node with a config
    pub async fn add_node_with_config(
        &mut self,
        networks: Networks<TYPES, I>,
        storage: I::Storage,
        initializer: HotShotInitializer<TYPES>,
        config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
        validator_config: ValidatorConfig<TYPES::SignatureKey>,
    ) -> SystemContext<TYPES, I>
    where
        I::Exchanges: ExchangesType<
            TYPES,
            Message<TYPES>,
            ElectionConfigs = (TYPES::ElectionConfigType, I::CommitteeElectionConfig),
        >,
        I: NodeImplementation<
            TYPES,
            QuorumNetwork = QuorumCommChannel<TYPES, I>,
            CommitteeNetwork = CommitteeCommChannel<TYPES, I>,
        >,
    {
        let node_id = self.next_node_id;
        self.next_node_id += 1;
        let known_nodes_with_stake = config.known_nodes_with_stake.clone();
        // Get key pair for certificate aggregation
        let private_key = validator_config.private_key.clone();
        let public_key = validator_config.public_key.clone();
        let entry = public_key.get_stake_table_entry(validator_config.stake_value);
        let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
            <QuorumEx<TYPES,I> as ConsensusExchange<
                TYPES,
                Message<TYPES>,
            >>::Membership::default_election_config(config.total_nodes.get() as u64)
        });
        let committee_election_config = I::committee_election_config_generator();
        let network_bundle = hotshot::Networks {
            quorum_network: networks.0.clone(),
            da_network: networks.1.clone(),
            _pd: PhantomData,
        };
        let exchanges = I::Exchanges::create(
            known_nodes_with_stake.clone(),
            (
                quorum_election_config,
                committee_election_config(config.da_committee_size as u64),
            ),
            networks,
            public_key.clone(),
            entry.clone(),
            private_key.clone(),
        );
        SystemContext::new(
            public_key,
            private_key,
            node_id,
            config,
            storage,
            exchanges,
            network_bundle,
            initializer,
            ConsensusMetricsValue::new(),
        )
        .await
        .expect("Could not init hotshot")
    }
}
