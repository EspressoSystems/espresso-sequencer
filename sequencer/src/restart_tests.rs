#![cfg(test)]

use super::*;
use anyhow::bail;
use async_compatibility_layer::art::async_timeout;
use async_std::task::{sleep, spawn, JoinHandle};
use cdn_broker::{reexports::crypto::signature::KeyPair, Broker, Config as BrokerConfig};
use cdn_marshal::{Config as MarshalConfig, Marshal};
use derivative::Derivative;
use espresso_types::{traits::PersistenceOptions, PrivKey, PubKey, SeqTypes, SequencerVersions};
use ethers::utils::{Anvil, AnvilInstance};
use futures::{
    future::{join_all, try_join_all, BoxFuture, FutureExt},
    stream::{BoxStream, StreamExt},
};
use hotshot::traits::implementations::derive_libp2p_peer_id;
use hotshot_orchestrator::{
    config::{Libp2pConfig, NetworkConfig},
    run_orchestrator,
};
use hotshot_types::{
    event::{Event, EventType},
    light_client::StateKeyPair,
    traits::{
        node_implementation::{ConsensusTime, Versions},
        signature_key::SignatureKey,
    },
};
use itertools::Itertools;
use portpicker::pick_unused_port;
use sequencer::{
    api::{self, data_source::testing::TestableSequencerDataSource, options::Http},
    genesis::StakeTableConfig,
    network::cdn::{TestingDef, WrappedSignatureKey},
};
use sequencer_utils::test_utils::setup_test;
use std::{path::Path, time::Duration};
use tempfile::TempDir;

async fn test_restart_helper(network: (usize, usize), restart: (usize, usize), cdn: bool) {
    setup_test();

    let mut network = TestNetwork::new(network.0, network.1, cdn).await;

    // Let the network get going.
    network.check_progress().await;
    // Restart some combination of nodes and ensure progress resumes.
    network.restart(restart.0, restart.1).await;
}

#[async_std::test]
async fn test_restart_1_da_with_cdn() {
    test_restart_helper((2, 3), (1, 0), true).await;
}

#[async_std::test]
async fn test_restart_1_regular_with_cdn() {
    test_restart_helper((2, 3), (0, 1), true).await;
}

#[async_std::test]
async fn test_restart_f_with_cdn() {
    test_restart_helper((4, 6), (1, 2), true).await;
}

#[async_std::test]
async fn test_restart_f_minus_1_with_cdn() {
    test_restart_helper((4, 6), (1, 1), true).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_f_plus_1_with_cdn() {
    test_restart_helper((4, 6), (1, 3), true).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_with_cdn() {
    test_restart_helper((4, 6), (1, 5), true).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_minus_1_with_cdn() {
    test_restart_helper((4, 6), (1, 4), true).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_plus_1_with_cdn() {
    test_restart_helper((4, 6), (2, 5), true).await;
}

#[async_std::test]
async fn test_restart_all_with_cdn() {
    test_restart_helper((2, 8), (2, 8), true).await;
}

#[async_std::test]
async fn test_restart_1_da_without_cdn() {
    test_restart_helper((2, 3), (1, 0), false).await;
}

#[async_std::test]
async fn test_restart_1_regular_without_cdn() {
    test_restart_helper((2, 3), (0, 1), false).await;
}

#[async_std::test]
async fn test_restart_f_without_cdn() {
    test_restart_helper((4, 6), (1, 2), false).await;
}

#[async_std::test]
async fn test_restart_f_minus_1_without_cdn() {
    test_restart_helper((4, 6), (1, 1), false).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_f_plus_1_without_cdn() {
    test_restart_helper((4, 6), (1, 3), false).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_without_cdn() {
    test_restart_helper((4, 6), (1, 5), false).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_minus_1_without_cdn() {
    test_restart_helper((4, 6), (1, 4), false).await;
}

#[ignore]
#[async_std::test]
async fn test_restart_2f_plus_1_without_cdn() {
    test_restart_helper((4, 6), (2, 5), false).await;
}

#[async_std::test]
async fn test_restart_all_without_cdn() {
    test_restart_helper((2, 8), (2, 8), false).await;
}

#[derive(Clone, Copy, Debug)]
struct NetworkParams<'a> {
    genesis_file: &'a Path,
    orchestrator_port: u16,
    cdn_port: u16,
    l1_provider: &'a str,
    peer_ports: &'a [u16],
}

#[derive(Clone, Debug)]
struct NodeParams {
    api_port: u16,
    libp2p_port: u16,
    staking_key: PrivKey,
    state_key: StateKeyPair,
    is_da: bool,
}

impl NodeParams {
    fn new(i: u64, is_da: bool) -> Self {
        Self {
            api_port: pick_unused_port().unwrap(),
            libp2p_port: pick_unused_port().unwrap(),
            staking_key: PubKey::generated_from_seed_indexed([0; 32], i).1,
            state_key: StateKeyPair::generate_from_seed_indexed([0; 32], i),
            is_da,
        }
    }
}

#[derive(Debug)]
struct TestNode<S: TestableSequencerDataSource> {
    storage: S::Storage,
    context: Option<
        SequencerContext<
            network::Production,
            <S::Options as PersistenceOptions>::Persistence,
            <SequencerVersions as Versions>::Base,
        >,
    >,
    modules: Modules,
    opt: Options,
    num_nodes: usize,
}

impl<S: TestableSequencerDataSource> TestNode<S> {
    #[tracing::instrument]
    async fn new(network: NetworkParams<'_>, node: &NodeParams) -> Self {
        tracing::info!(?network, ?node, "creating node",);

        let storage = S::create_storage().await;
        let mut modules = Modules {
            http: Some(Http::with_port(node.api_port)),
            status: Some(Default::default()),
            catchup: Some(Default::default()),
            ..Default::default()
        };
        if node.is_da {
            modules.query = Some(Default::default());
            modules.state = Some(Default::default());
        }

        let mut opt = Options::parse_from([
            "sequencer",
            "--private-staking-key",
            &node.staking_key.to_string(),
            "--private-state-key",
            &node.state_key.sign_key_ref().to_string(),
            "--genesis-file",
            &network.genesis_file.display().to_string(),
            "--orchestrator-url",
            &format!("http://localhost:{}", network.orchestrator_port),
            "--libp2p-bind-address",
            &format!("0.0.0.0:{}", node.libp2p_port),
            "--libp2p-advertise-address",
            &format!("127.0.0.1:{}", node.libp2p_port),
            "--cdn-endpoint",
            &format!("127.0.0.1:{}", network.cdn_port),
            "--state-peers",
            &network
                .peer_ports
                .iter()
                .map(|port| format!("http://127.0.0.1:{port}"))
                .join(","),
            "--l1-provider-url",
            network.l1_provider,
        ]);
        opt.is_da = node.is_da;
        Self {
            storage,
            modules,
            opt,
            num_nodes: network.peer_ports.len(),
            context: None,
        }
    }

    fn stop(&mut self) -> BoxFuture<()> {
        async {
            if let Some(mut context) = self.context.take() {
                tracing::info!(node_id = context.node_id(), "stopping node");
                context.shut_down().await;
            }
        }
        .boxed()
    }

    fn start(&mut self) -> BoxFuture<()>
    where
        S::Storage: Send,
    {
        async {
            tracing::info!("starting node");
            let ctx = init_with_storage(
                self.modules.clone(),
                self.opt.clone(),
                S::persistence_options(&self.storage),
                <SequencerVersions as Versions>::Base::instance(),
            )
            .await
            .unwrap();
            tracing::info!(node_id = ctx.node_id(), "starting consensus");
            ctx.start_consensus().await;
            self.context = Some(ctx);
        }
        .boxed()
    }

    async fn event_stream(&self) -> Option<BoxStream<Event<SeqTypes>>> {
        if let Some(ctx) = &self.context {
            Some(ctx.event_stream().await.boxed())
        } else {
            None
        }
    }

    fn check_progress_with_timeout(&self) -> BoxFuture<anyhow::Result<()>> {
        async {
            let Some(context) = &self.context else {
                tracing::info!("skipping progress check on stopped node");
                return Ok(());
            };
            let node_id = context.node_id();
            let next_view_timeout = {
                context
                    .consensus()
                    .read()
                    .await
                    .hotshot
                    .config
                    .next_view_timeout
            };
            // Give enough time for every node to propose, with every view timing out. This is
            // conservative: of course if we actually make progress, not every view will time out,
            // and we will take less than this amount of time.
            let timeout = Duration::from_millis(next_view_timeout) * (self.num_nodes as u32);
            match async_timeout(timeout, self.check_progress()).await {
                Ok(res) => res,
                Err(_) => bail!("timed out waiting for progress on node {node_id}"),
            }
        }
        .boxed()
    }

    async fn check_progress(&self) -> anyhow::Result<()> {
        let Some(context) = &self.context else {
            tracing::info!("skipping progress check on stopped node");
            return Ok(());
        };

        let num_nodes = {
            context
                .consensus()
                .read()
                .await
                .hotshot
                .config
                .num_nodes_with_stake
        };
        let node_id = context.node_id();
        tracing::info!(node_id, num_nodes, "waiting for progress from node");

        // Wait for a block proposed by this node. This proves that the node is tracking consensus
        // (getting Decide events) and participating (able to propose).
        let mut events = context.event_stream().await;
        while let Some(event) = events.next().await {
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for leaf in leaf_chain.iter() {
                if leaf.leaf.view_number().u64() % (num_nodes.get() as u64) == node_id {
                    tracing::info!(
                        node_id,
                        height = leaf.leaf.height(),
                        "got leaf proposed by this node"
                    );
                    return Ok(());
                }
                tracing::info!(
                    node_id,
                    height = leaf.leaf.height(),
                    view = leaf.leaf.view_number().u64(),
                    "leaf not proposed by this node"
                );
            }
        }

        bail!("node {node_id} event stream ended unexpectedly");
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct TestNetwork {
    da_nodes: Vec<TestNode<api::sql::DataSource>>,
    regular_nodes: Vec<TestNode<api::fs::DataSource>>,
    tmp: TempDir,
    orchestrator_task: Option<JoinHandle<()>>,
    broker_task: Option<JoinHandle<()>>,
    marshal_task: Option<JoinHandle<()>>,
    #[derivative(Debug = "ignore")]
    _anvil: AnvilInstance,
}

impl Drop for TestNetwork {
    fn drop(&mut self) {
        if let Some(task) = self.orchestrator_task.take() {
            async_std::task::block_on(task.cancel());
        }
        if let Some(task) = self.broker_task.take() {
            async_std::task::block_on(task.cancel());
        }
        if let Some(task) = self.marshal_task.take() {
            async_std::task::block_on(task.cancel());
        }
    }
}

impl TestNetwork {
    async fn new(da_nodes: usize, regular_nodes: usize, cdn: bool) -> Self {
        let tmp = TempDir::new().unwrap();
        let genesis_file = tmp.path().join("genesis.toml");
        let genesis = Genesis {
            chain_config: Default::default(),
            stake_table: StakeTableConfig { capacity: 10 },
            accounts: Default::default(),
            l1_finalized: Default::default(),
            header: Default::default(),
            upgrades: Default::default(),
        };
        genesis.to_file(&genesis_file).unwrap();

        let node_params = (0..da_nodes + regular_nodes)
            .map(|i| NodeParams::new(i as u64, i < da_nodes))
            .collect::<Vec<_>>();

        let orchestrator_port = pick_unused_port().unwrap();
        let orchestrator_task = Some(start_orchestrator(orchestrator_port, &node_params));

        let cdn_dir = tmp.path().join("cdn");
        let cdn_port = pick_unused_port().unwrap();
        let broker_task = if cdn {
            Some(start_broker(&cdn_dir).await)
        } else {
            None
        };
        let marshal_task = if cdn {
            Some(start_marshal(&cdn_dir, cdn_port).await)
        } else {
            None
        };

        let anvil_port = pick_unused_port().unwrap();
        let anvil = Anvil::new().port(anvil_port).spawn();
        let anvil_endpoint = anvil.endpoint();

        let peer_ports = node_params
            .iter()
            .map(|node| node.api_port)
            .collect::<Vec<_>>();
        let network_params = NetworkParams {
            genesis_file: &genesis_file,
            orchestrator_port,
            cdn_port,
            l1_provider: &anvil_endpoint,
            peer_ports: &peer_ports,
        };

        let mut network = Self {
            da_nodes: join_all(
                (0..da_nodes).map(|i| TestNode::new(network_params, &node_params[i])),
            )
            .await,
            regular_nodes: join_all(
                (0..regular_nodes)
                    .map(|i| TestNode::new(network_params, &node_params[i + da_nodes])),
            )
            .await,
            tmp,
            orchestrator_task,
            broker_task,
            marshal_task,
            _anvil: anvil,
        };

        join_all(
            network
                .da_nodes
                .iter_mut()
                .map(TestNode::start)
                .chain(network.regular_nodes.iter_mut().map(TestNode::start)),
        )
        .await;

        network
    }

    async fn check_progress(&self) {
        try_join_all(
            self.da_nodes
                .iter()
                .map(TestNode::check_progress_with_timeout)
                .chain(
                    self.regular_nodes
                        .iter()
                        .map(TestNode::check_progress_with_timeout),
                ),
        )
        .await
        .unwrap();
    }

    async fn restart(&mut self, da_nodes: usize, regular_nodes: usize) {
        tracing::info!(da_nodes, regular_nodes, "shutting down some nodes");
        join_all(
            self.da_nodes[..da_nodes]
                .iter_mut()
                .map(TestNode::stop)
                .chain(
                    self.regular_nodes[..regular_nodes]
                        .iter_mut()
                        .map(TestNode::stop),
                ),
        )
        .await;

        let quorum_threshold = 2 * self.num_nodes() / 3 + 1;
        let da_threshold = 2 * self.da_nodes.len() / 3 + 1;
        if self.num_nodes() - da_nodes - regular_nodes >= quorum_threshold
            && self.da_nodes.len() - da_nodes >= da_threshold
        {
            // If we are shutting down less than f nodes, the remaining nodes should be able to make
            // progress, and we will check that that is the case.
            //
            // Note that not every node will be able to commit leaves, because a node requires the
            // cooperation of the node after it to commit its proposal. But, as long as we have shut
            // down fewer than the fault tolerance, at least *some* node will have a correct node
            // after it and will be able to commit. Thus, we just grab an event stream and look for
            // any decide.
            tracing::info!("waiting for remaining nodes to progress");
            let mut events = if da_nodes < self.da_nodes.len() {
                self.da_nodes[da_nodes].event_stream().await.unwrap()
            } else {
                self.regular_nodes[regular_nodes]
                    .event_stream()
                    .await
                    .unwrap()
            };
            // Wait for a few decides, the first couple may be from before the restart.
            for _ in 0..self.num_nodes() {
                let timeout = Duration::from_secs((2 * self.num_nodes()) as u64);
                async_timeout(timeout, async {
                    loop {
                        let event = events
                            .next()
                            .await
                            .expect("event stream terminated unexpectedly");
                        let EventType::Decide { leaf_chain, .. } = event.event else {
                            continue;
                        };
                        tracing::info!(?leaf_chain, "got decide, chain is progressing");
                        break;
                    }
                })
                .await
                .expect("timed out waiting for progress with nodes down");
            }
        } else {
            // Make sure there is a brief delay before restarting the nodes; we need the OS to
            // have time to clean up the ports they were using.
            tracing::info!(
                "shut down too many nodes to make progress; will continue after a brief delay"
            );
            sleep(Duration::from_secs(2)).await;
        }

        join_all(
            self.da_nodes[..da_nodes]
                .iter_mut()
                .map(TestNode::start)
                .chain(
                    self.regular_nodes[..regular_nodes]
                        .iter_mut()
                        .map(TestNode::start),
                ),
        )
        .await;
        self.check_progress().await;
    }

    fn num_nodes(&self) -> usize {
        self.da_nodes.len() + self.regular_nodes.len()
    }
}

fn start_orchestrator(port: u16, nodes: &[NodeParams]) -> JoinHandle<()> {
    // We don't run a builder in these tests, so use a very short timeout before nodes decide to
    // build an empty block on their own.
    let builder_timeout = Duration::from_millis(10);
    // These tests frequently have nodes down and views failing, so we use a fairly short view
    // timeout.
    let view_timeout = Duration::from_secs(2);

    let num_nodes = nodes.len();
    let bootstrap_nodes = nodes
        .iter()
        .map(|node| {
            let port = node.libp2p_port;
            let peer_id = derive_libp2p_peer_id::<PubKey>(&node.staking_key).unwrap();
            let addr = format!("/ip4/127.0.0.1/udp/{port}/quic-v1")
                .parse()
                .unwrap();
            (peer_id, addr)
        })
        .collect();

    let mut config = NetworkConfig::<PubKey> {
        indexed_da: false,
        libp2p_config: Some(Libp2pConfig {
            bootstrap_nodes,
            node_index: 0,
            bootstrap_mesh_n_high: 4,
            bootstrap_mesh_n_low: 4,
            bootstrap_mesh_outbound_min: 4 / 2,
            bootstrap_mesh_n: 4,
            mesh_n_high: 4,
            mesh_n_low: 4,
            mesh_outbound_min: 4 / 2,
            mesh_n: 4,
            next_view_timeout: view_timeout.as_millis() as u64,
            online_time: 10,
            num_txn_per_round: 0,
            server_mode: true,
            builder_timeout,
        }),
        ..Default::default()
    };
    config.config.num_nodes_with_stake = num_nodes.try_into().unwrap();
    config.config.da_staked_committee_size = num_nodes;
    config.config.known_nodes_with_stake = vec![];
    config.config.known_da_nodes = vec![];
    config.config.known_nodes_without_stake = vec![];
    config.config.next_view_timeout = view_timeout.as_millis() as u64;
    config.config.builder_timeout = builder_timeout;

    let bind = format!("http://0.0.0.0:{port}").parse().unwrap();
    spawn(async move {
        match run_orchestrator(config, bind).await {
            Ok(()) => tracing::warn!("orchestrator exited"),
            Err(err) => tracing::error!(%err, "orchestrator failed"),
        }
    })
}

async fn start_broker(dir: &Path) -> JoinHandle<()> {
    let (public_key, private_key) = PubKey::generated_from_seed_indexed([0; 32], 1337);
    let public_port = pick_unused_port().expect("failed to find free port for broker");
    let private_port = pick_unused_port().expect("failed to find free port for broker");
    let broker_config: BrokerConfig<TestingDef<SeqTypes>> = BrokerConfig {
        public_advertise_endpoint: format!("127.0.0.1:{}", public_port),
        public_bind_endpoint: format!("127.0.0.1:{}", public_port),
        private_advertise_endpoint: format!("127.0.0.1:{}", private_port),
        private_bind_endpoint: format!("127.0.0.1:{}", private_port),

        metrics_bind_endpoint: None,
        discovery_endpoint: dir.display().to_string(),
        keypair: KeyPair {
            public_key: WrappedSignatureKey(public_key),
            private_key,
        },

        ca_cert_path: None,
        ca_key_path: None,
        global_memory_pool_size: Some(1024 * 1024 * 1024),
    };

    spawn(async move {
        match Broker::new(broker_config).await.unwrap().start().await {
            Ok(()) => tracing::warn!("broker exited"),
            Err(err) => tracing::error!("broker failed: {err:#}"),
        }
    })
}

async fn start_marshal(dir: &Path, port: u16) -> JoinHandle<()> {
    let marshal_config = MarshalConfig {
        bind_endpoint: format!("0.0.0.0:{port}"),
        metrics_bind_endpoint: None,
        discovery_endpoint: dir.display().to_string(),
        ca_cert_path: None,
        ca_key_path: None,
        global_memory_pool_size: Some(1024 * 1024 * 1024),
    };

    spawn(async move {
        match Marshal::<TestingDef<SeqTypes>>::new(marshal_config)
            .await
            .unwrap()
            .start()
            .await
        {
            Ok(()) => tracing::warn!("marshal exited"),
            Err(err) => tracing::error!("marshal failed: {err:#}"),
        }
    })
}
