use std::{
    cmp::max,
    collections::HashSet,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256, utils::parse_ether},
    providers::{Provider, ProviderBuilder, ext::AnvilApi},
    signers::local::PrivateKeySigner,
};
use committable::{Commitment, Committable};
use espresso_contract_deployer::{
    Contract, Contracts, DEFAULT_EXIT_ESCROW_PERIOD_SECONDS, builder::DeployerArgsBuilder,
    network_config::light_client_genesis_from_stake_table,
};
use espresso_types::{
    NamespaceId, TEST_UPGRADE, ValidatedState,
    v0::traits::{NullEventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
};
use futures::{
    future::{FutureExt, join_all},
    stream::{Stream, StreamExt},
};
use hotshot::types::{Event, EventType};
use hotshot_contract_adapter::stake_table::StakeTableContractVersion;
use hotshot_types::{
    event::LeafInfo, light_client::LCV3StateSignatureRequestBody, new_protocol::CoordinatorEvent,
    traits::metrics::NoMetrics,
};
use http_client::{Client, error::ClientErr};
use itertools::izip;
use jf_merkle_tree_compat::{MerkleCommitment, MerkleTreeScheme};
use staking_cli::{
    Transaction as StakingTransaction,
    demo::{DelegationConfig, StakingTransactions},
};
use tempfile::TempDir;
use test_utils::reserve_tcp_port;
use tokio::time::sleep;
use vbs::version::StaticVersion;
use versions::{EPOCH_VERSION, NEW_PROTOCOL_VERSION, Upgrade};

use super::*;
use crate::{
    api::data_source::testing::TestableSequencerDataSource,
    catchup::{NullStateCatchup, ParallelStateCatchup, StatePeers},
    network,
    persistence::no_storage,
    testing::{
        TestConfig, TestConfigBuilder, decided_leaves, deploy_stake_table, run_test_builder,
        wait_for_decide_on_handle, wait_for_epochs,
    },
};

pub const STAKE_TABLE_CAPACITY_FOR_TEST: usize = 10;

/// One storage per node, and each node's persistence options on it.
pub async fn node_storage<D: TestableSequencerDataSource, const NUM_NODES: usize>()
-> (Vec<D::Storage>, [D::Options; NUM_NODES]) {
    let storage = join_all((0..NUM_NODES).map(|_| D::create_storage())).await;
    let persistence = std::array::from_fn(|i| D::persistence_options(&storage[i]));
    (storage, persistence)
}

pub struct TestNetwork<P: PersistenceOptions, const NUM_NODES: usize> {
    pub server: SequencerContext<network::Memory, P::Persistence>,
    pub peers: Vec<SequencerContext<network::Memory, P::Persistence>>,
    pub cfg: TestConfig<{ NUM_NODES }>,
    // todo (abdul): remove this when fs storage is removed
    pub temp_dir: Option<TempDir>,
    pub contracts: Option<Contracts>,
    /// Deferred node indices not yet started (see [`Self::start_deferred_node`]).
    deferred: Vec<usize>,
    genesis_states: Vec<ValidatedState>,
    /// Node 0's API, if nodes use it for catchup (see [`with_api_catchup`]).
    api_catchup: Option<Url>,
}

pub struct TestNetworkConfig<const NUM_NODES: usize, P, C>
where
    P: PersistenceOptions,
    C: StateCatchup + 'static,
{
    state: [ValidatedState; NUM_NODES],
    persistence: [P; NUM_NODES],
    catchup: [C; NUM_NODES],
    network_config: TestConfig<{ NUM_NODES }>,
    api_config: Options,
    contracts: Option<Contracts>,
    initial_token_supply: Option<U256>,
    deferred_start: Vec<usize>,
    api_catchup: bool,
}

impl<const NUM_NODES: usize, P, C> TestNetworkConfig<{ NUM_NODES }, P, C>
where
    P: PersistenceOptions,
    C: StateCatchup + 'static,
{
    pub fn states(&self) -> [ValidatedState; NUM_NODES] {
        self.state.clone()
    }

    fn has_stake_table(&self) -> bool {
        self.state[0]
            .chain_config
            .resolve()
            .is_some_and(|cf| cf.stake_table_contract.is_some())
    }

    /// The new protocol reads every committee after genesis from the
    /// stake table contract, so a 0.6+ network needs one even when the
    /// test doesn't care about staking.
    async fn deploy_default_stake_table(&mut self) {
        let (state, contracts) = deploy_pos(
            &self.network_config,
            self.state[0].clone(),
            self.initial_token_supply,
            DelegationConfig::MultipleDelegators,
            StakeTableContractVersion::V3,
            &(0..NUM_NODES).collect::<Vec<_>>(),
        )
        .await
        .expect("failed to deploy the stake table");
        self.state = std::array::from_fn(|_| state.clone());
        self.contracts = Some(contracts);
    }
}

#[derive(Clone)]
pub struct TestNetworkConfigBuilder<const NUM_NODES: usize, P, C>
where
    P: PersistenceOptions,
    C: StateCatchup + 'static,
{
    state: [ValidatedState; NUM_NODES],
    persistence: Option<[P; NUM_NODES]>,
    catchup: Option<[C; NUM_NODES]>,
    api_config: Option<Options>,
    network_config: Option<TestConfig<{ NUM_NODES }>>,
    contracts: Option<Contracts>,
    initial_token_supply: Option<U256>,
    deferred_start: Vec<usize>,
    api_catchup: bool,
}

impl Default for TestNetworkConfigBuilder<5, no_storage::Options, NullStateCatchup> {
    fn default() -> Self {
        TestNetworkConfigBuilder {
            state: std::array::from_fn(|_| ValidatedState::default()),
            persistence: Some([no_storage::Options; 5]),
            catchup: Some(std::array::from_fn(|_| NullStateCatchup::default())),
            network_config: None,
            api_config: None,
            contracts: None,
            initial_token_supply: None,
            deferred_start: Vec::new(),
            api_catchup: true,
        }
    }
}

impl<const NUM_NODES: usize>
    TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, NullStateCatchup>
{
    pub fn with_num_nodes()
    -> TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, NullStateCatchup> {
        TestNetworkConfigBuilder {
            state: std::array::from_fn(|_| ValidatedState::default()),
            persistence: Some([no_storage::Options; { NUM_NODES }]),
            catchup: Some(std::array::from_fn(|_| NullStateCatchup::default())),
            network_config: None,
            api_config: None,
            contracts: None,
            initial_token_supply: None,
            deferred_start: Vec::new(),
            api_catchup: true,
        }
    }
}

impl<const NUM_NODES: usize, P, C> TestNetworkConfigBuilder<{ NUM_NODES }, P, C>
where
    P: PersistenceOptions,
    C: StateCatchup + 'static,
{
    pub fn states(mut self, state: [ValidatedState; NUM_NODES]) -> Self {
        self.state = state;
        self
    }

    /// The chain config node 0 starts from.
    pub fn chain_config(&self) -> ChainConfig {
        self.state[0]
            .chain_config
            .resolve()
            .expect("node 0 starts from a full chain config")
    }

    pub fn initial_token_supply(mut self, supply: U256) -> Self {
        self.initial_token_supply = Some(supply);
        self
    }

    pub fn persistences<NP: PersistenceOptions>(
        self,
        persistence: [NP; NUM_NODES],
    ) -> TestNetworkConfigBuilder<{ NUM_NODES }, NP, C> {
        TestNetworkConfigBuilder {
            state: self.state,
            catchup: self.catchup,
            network_config: self.network_config,
            api_config: self.api_config,
            persistence: Some(persistence),
            contracts: self.contracts,
            initial_token_supply: self.initial_token_supply,
            deferred_start: self.deferred_start,
            api_catchup: self.api_catchup,
        }
    }

    pub fn api_config(mut self, api_config: Options) -> Self {
        self.api_config = Some(api_config);
        self
    }

    pub fn catchups<NC: StateCatchup + 'static>(
        self,
        catchup: [NC; NUM_NODES],
    ) -> TestNetworkConfigBuilder<{ NUM_NODES }, P, NC> {
        TestNetworkConfigBuilder {
            state: self.state,
            catchup: Some(catchup),
            network_config: self.network_config,
            api_config: self.api_config,
            persistence: self.persistence,
            contracts: self.contracts,
            initial_token_supply: self.initial_token_supply,
            deferred_start: self.deferred_start,
            api_catchup: self.api_catchup,
        }
    }

    /// Defers starting the nodes at the given (trailing) indices; they
    /// join later via [`TestNetwork::start_deferred_node`].
    pub fn deferred_start(mut self, indices: &[usize]) -> Self {
        self.deferred_start = indices.to_vec();
        self
    }

    /// Leaves node 0's API out of every node's catchup providers, for tests
    /// of a specific catchup route. A node then catches up only through
    /// its own provider, so past epoch 2 it needs one that serves leaves.
    pub fn without_api_catchup(mut self) -> Self {
        self.api_catchup = false;
        self
    }

    pub fn network_config(mut self, network_config: TestConfig<{ NUM_NODES }>) -> Self {
        self.network_config = Some(network_config);
        self
    }

    /// See [`TestConfig::set_genesis_chain_config`]. Call after `network_config`.
    pub fn genesis_chain_config(mut self, chain_config: ChainConfig) -> Self {
        self.network_config
            .as_mut()
            .expect("network_config is required")
            .set_genesis_chain_config(chain_config);
        self
    }

    pub fn contracts(mut self, contracts: Contracts) -> Self {
        self.contracts = Some(contracts);
        self
    }

    /// Setup for POS testing. Deploys contracts and adds the
    /// stake table address to state. Must be called before `build()`.
    pub async fn pos_hook(
        self,
        delegation_config: DelegationConfig,
        stake_table_version: StakeTableContractVersion,
        upgrade: Upgrade,
    ) -> anyhow::Result<Self> {
        let registered: Vec<usize> = (0..NUM_NODES).collect();
        self.pos_hook_with_registered(delegation_config, stake_table_version, upgrade, &registered)
            .await
    }

    /// Like [`Self::pos_hook`], but registers only the validators at the
    /// `registered` node indices on the stake table contract. The other
    /// nodes still run from genesis (which seeds the first two epochs)
    /// and can be registered mid-test via [`register_validators`].
    pub async fn pos_hook_with_registered(
        self,
        delegation_config: DelegationConfig,
        stake_table_version: StakeTableContractVersion,
        upgrade: Upgrade,
        registered: &[usize],
    ) -> anyhow::Result<Self> {
        if upgrade.base < EPOCH_VERSION && upgrade.target < EPOCH_VERSION {
            panic!("given version does not require pos deployment");
        };

        let network_config = self
            .network_config
            .as_ref()
            .expect("network_config is required");
        let (state, contracts) = deploy_pos(
            network_config,
            self.state[0].clone(),
            self.initial_token_supply,
            delegation_config,
            stake_table_version,
            registered,
        )
        .await?;
        Ok(self
            .states(std::array::from_fn(|_| state.clone()))
            .contracts(contracts))
    }

    pub fn build(self) -> TestNetworkConfig<{ NUM_NODES }, P, C> {
        TestNetworkConfig {
            state: self.state,
            persistence: self.persistence.unwrap(),
            catchup: self.catchup.unwrap(),
            network_config: self.network_config.unwrap(),
            api_config: self.api_config.unwrap(),
            contracts: self.contracts,
            initial_token_supply: self.initial_token_supply,
            deferred_start: self.deferred_start,
            api_catchup: self.api_catchup,
        }
    }
}

impl<P: PersistenceOptions, const NUM_NODES: usize> TestNetwork<P, { NUM_NODES }> {
    pub async fn new<C: StateCatchup + 'static>(
        cfg: TestNetworkConfig<{ NUM_NODES }, P, C>,
        upgrade: versions::Upgrade,
    ) -> Self {
        let mut cfg = cfg;
        if upgrade.base >= NEW_PROTOCOL_VERSION && !cfg.has_stake_table() {
            cfg.deploy_default_stake_table().await;
        }
        let mut builder_tasks = Vec::new();

        let (task, builder_url) =
            run_test_builder::<{ NUM_NODES }>(cfg.network_config.builder_port()).await;
        builder_tasks.push(task);
        cfg.network_config
            .set_builder_urls(vec1::vec1![builder_url.clone()]);

        // add default storage if none is provided as query module is now required
        let mut opt = cfg.api_config.clone();
        let temp_dir = if opt.storage_fs.is_none() && opt.storage_sql.is_none() {
            let temp_dir = tempfile::tempdir().unwrap();
            opt = opt.query_fs(
                Default::default(),
                crate::persistence::fs::Options::new(temp_dir.path().to_path_buf()),
            );
            Some(temp_dir)
        } else {
            None
        };

        let deferred = cfg.deferred_start.clone();
        let genesis_states = cfg.state.to_vec();
        let api_catchup: Option<Url> = cfg.api_catchup.then(|| {
            format!("http://localhost:{}", opt.http.port)
                .parse()
                .unwrap()
        });
        assert!(
            deferred.len() < NUM_NODES,
            "node 0 runs the API server and cannot be deferred"
        );
        assert_eq!(
            deferred,
            (NUM_NODES - deferred.len()..NUM_NODES).collect::<Vec<_>>(),
            "deferred_start must be the trailing indices so `node(i)` stays aligned"
        );

        let mut nodes = join_all(
            izip!(cfg.state, cfg.persistence, cfg.catchup)
                .enumerate()
                .filter(|(i, _)| !deferred.contains(i))
                .map(|(i, (state, persistence, state_peers))| {
                    let state_peers = with_api_catchup(state_peers, api_catchup.as_ref());
                    let opt = opt.clone();
                    let cfg = &cfg.network_config;
                    let upgrades_map = cfg.upgrades();
                    async move {
                        if i == 0 {
                            opt.serve(|metrics, consumer, storage| {
                                let cfg = cfg.clone();
                                async move {
                                    Ok(cfg
                                        .init_node(
                                            0,
                                            state,
                                            persistence,
                                            Some(state_peers),
                                            storage,
                                            &*metrics,
                                            STAKE_TABLE_CAPACITY_FOR_TEST,
                                            consumer,
                                            upgrade,
                                            upgrades_map,
                                        )
                                        .await)
                                }
                                .boxed()
                            })
                            .await
                            .unwrap()
                        } else {
                            cfg.init_node(
                                i,
                                state,
                                persistence,
                                Some(state_peers),
                                None,
                                &NoMetrics,
                                STAKE_TABLE_CAPACITY_FOR_TEST,
                                NullEventConsumer,
                                upgrade,
                                upgrades_map,
                            )
                            .await
                        }
                    }
                    .boxed()
                }),
        )
        .await;

        let handle_0 = &nodes[0];

        // Hook the builder(s) up to the event stream from the first node
        for builder_task in builder_tasks {
            builder_task.start(Box::new(
                handle_0
                    .consensus_handle()
                    .legacy_consensus()
                    .read()
                    .await
                    .event_stream(),
            ));
        }

        for ctx in &nodes {
            ctx.start_consensus().await;
        }

        let server = nodes.remove(0);
        let peers = nodes;

        Self {
            server,
            peers,
            cfg: cfg.network_config,
            temp_dir,
            contracts: cfg.contracts,
            deferred,
            genesis_states,
            api_catchup,
        }
    }

    /// Initializes and starts a node deferred at construction (see
    /// [`TestNetworkConfigBuilder::deferred_start`]), in ascending index
    /// order; the node is then reachable via [`Self::node`] as usual.
    pub async fn start_deferred_node<C: StateCatchup + 'static>(
        &mut self,
        i: usize,
        state: ValidatedState,
        persistence: P,
        catchup: C,
        upgrade: versions::Upgrade,
    ) -> &SequencerContext<network::Memory, P::Persistence> {
        assert_eq!(
            self.deferred.first(),
            Some(&i),
            "deferred nodes must be started in ascending index order"
        );
        self.deferred.remove(0);

        let ctx = self
            .init_and_start(i, state, persistence, catchup, upgrade)
            .await;
        self.peers.push(ctx);
        self.peers.last().unwrap()
    }

    /// Shuts the node at index `i` down and reinitializes it from the
    /// network's current configuration, picking up any rotated consensus
    /// keys or coordinator address (see [`TestConfig::set_consensus_keys`]
    /// and [`TestConfig::set_coordinator_addr`]). Node 0 hosts the query
    /// API and cannot be restarted this way.
    pub async fn restart_node<C: StateCatchup + 'static>(
        &mut self,
        i: usize,
        state: ValidatedState,
        persistence: P,
        catchup: C,
        upgrade: versions::Upgrade,
    ) -> &SequencerContext<network::Memory, P::Persistence> {
        self.stop_node(i).await;
        self.start_stopped_node(i, state, persistence, catchup, upgrade)
            .await
    }

    /// Shuts the node at index `i` down, leaving it stopped until
    /// [`Self::start_stopped_node`]. Node 0 hosts the query API and cannot
    /// be stopped this way.
    pub async fn stop_node(&mut self, i: usize) {
        assert_ne!(i, 0, "node 0 runs the API server and cannot be restarted");
        assert!(
            !self.deferred.contains(&i),
            "node {i} was deferred and has not been started yet"
        );
        self.peers[i - 1].shut_down().await;
        // The restarted node may rebind the very coordinator port it
        // just released, but `shut_down` cannot await the listener drop:
        // aborted tasks holding network senders keep it alive. Poll
        // until the address is actually bindable again.
        let addr = self.cfg.coordinator_addr(i).to_string();
        timeout(Duration::from_secs(60), async {
            while std::net::TcpListener::bind(&addr).is_err() {
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .expect("shut-down node did not release its coordinator port");
    }

    /// Reinitializes a node shut down by [`Self::stop_node`] from the
    /// network's current configuration and starts consensus on it.
    pub async fn start_stopped_node<C: StateCatchup + 'static>(
        &mut self,
        i: usize,
        state: ValidatedState,
        persistence: P,
        catchup: C,
        upgrade: versions::Upgrade,
    ) -> &SequencerContext<network::Memory, P::Persistence> {
        let ctx = self
            .init_and_start(i, state, persistence, catchup, upgrade)
            .await;
        self.peers[i - 1] = ctx;
        &self.peers[i - 1]
    }

    /// The state node `i` started from, for bringing it back from genesis.
    pub fn genesis_state(&self, i: usize) -> ValidatedState {
        self.genesis_states[i].clone()
    }

    /// Initializes node `i` from the network's current configuration and
    /// starts consensus on it, the same way construction does.
    async fn init_and_start<C: StateCatchup + 'static>(
        &self,
        i: usize,
        state: ValidatedState,
        persistence: P,
        catchup: C,
        upgrade: versions::Upgrade,
    ) -> SequencerContext<network::Memory, P::Persistence> {
        let ctx = self
            .cfg
            .init_node(
                i,
                state,
                persistence,
                Some(with_api_catchup(catchup, self.api_catchup.as_ref())),
                None,
                &NoMetrics,
                STAKE_TABLE_CAPACITY_FOR_TEST,
                NullEventConsumer,
                upgrade,
                self.cfg.upgrades(),
            )
            .await;
        ctx.start_consensus().await;
        ctx
    }

    pub async fn stop_consensus(&mut self) {
        self.server.shutdown_consensus().await;

        for ctx in &mut self.peers {
            ctx.shutdown_consensus().await;
        }
    }

    /// The context of the node at index `i` (node 0 is the API server).
    pub fn node(&self, i: usize) -> &SequencerContext<network::Memory, P::Persistence> {
        if i == 0 {
            &self.server
        } else {
            &self.peers[i - 1]
        }
    }
}

/// Adds node 0's API, if given, as a catchup provider alongside `catchup`. From epoch 3 the
/// new protocol fetches epoch root leaves through catchup, so a node without storage or state
/// peers would otherwise wedge at the first epoch boundary. Both providers are queried at
/// once, so a test of `catchup` itself must leave the API out.
fn with_api_catchup(
    catchup: impl StateCatchup + 'static,
    api_url: Option<&Url>,
) -> ParallelStateCatchup {
    let mut providers: Vec<Arc<dyn StateCatchup>> = vec![Arc::new(catchup)];
    if let Some(url) = api_url {
        providers.push(Arc::new(StatePeers::<SequencerApiVersion>::from_urls(
            vec![url.clone()],
            Default::default(),
            Duration::from_secs(2),
            &NoMetrics,
        )));
    }
    ParallelStateCatchup::new(&providers, Duration::from_secs(5))
}

/// Deploys the stake table (with a mock light client) sized to the
/// network's epochs, registers the validators at `registered`, and
/// returns `state` with the contract in its chain config and a zero base
/// fee, so no test pays for `FeeState` catchup.
async fn deploy_pos<const NUM_NODES: usize>(
    network_config: &TestConfig<NUM_NODES>,
    state: ValidatedState,
    initial_token_supply: Option<U256>,
    delegation_config: DelegationConfig,
    stake_table_version: StakeTableContractVersion,
    registered: &[usize],
) -> anyhow::Result<(ValidatedState, Contracts)> {
    let l1_url = network_config.l1_url();
    let signer = network_config.signer();
    let deployer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer.clone()))
        .connect_http(l1_url.clone());

    let blocks_per_epoch = network_config.hotshot_config().epoch_height;
    let epoch_start_block = network_config.hotshot_config().epoch_start_block;
    let (genesis_state, genesis_stake) = light_client_genesis_from_stake_table(
        &network_config.hotshot_config().hotshot_stake_table(),
        STAKE_TABLE_CAPACITY_FOR_TEST,
    )
    .unwrap();

    let mut contracts = Contracts::new();
    let args = DeployerArgsBuilder::default()
        .deployer(deployer.clone())
        .rpc_url(l1_url.clone())
        .mock_light_client(true)
        .genesis_lc_state(genesis_state)
        .genesis_st_state(genesis_stake)
        .blocks_per_epoch(blocks_per_epoch)
        .epoch_start_block(epoch_start_block)
        .exit_escrow_period(U256::from(max(
            blocks_per_epoch * 15 + 100,
            DEFAULT_EXIT_ESCROW_PERIOD_SECONDS,
        )))
        .multisig_pauser(signer.address())
        .token_name("Espresso".to_string())
        .token_symbol("ESP".to_string())
        .initial_token_supply(initial_token_supply.unwrap_or(U256::from(100000u64)))
        .ops_timelock_delay(U256::from(0))
        .ops_timelock_admin(signer.address())
        .ops_timelock_proposers(vec![signer.address()])
        .ops_timelock_executors(vec![signer.address()])
        .safe_exit_timelock_delay(U256::from(10))
        .safe_exit_timelock_admin(signer.address())
        .safe_exit_timelock_proposers(vec![signer.address()])
        .safe_exit_timelock_executors(vec![signer.address()])
        .build()
        .unwrap();

    deploy_stake_table(&args, stake_table_version, &mut contracts)
        .await
        .context("failed to deploy contracts")?;

    let stake_table_address = contracts
        .address(Contract::StakeTableProxy)
        .expect("StakeTableProxy address not found");

    StakingTransactions::create(
        l1_url.clone(),
        &deployer,
        stake_table_address,
        network_config.staking_key_sets(registered),
        None,
        delegation_config,
    )
    .await
    .expect("stake table setup failed")
    .apply_all()
    .await
    .expect("send all txns failed");

    // Stake table updates are read at the finalized L1 block, so keep L1
    // finalizing even when no transactions are sent.
    if let Some(anvil) = network_config.anvil() {
        anvil
            .anvil_set_interval_mining(1)
            .await
            .expect("interval mining");
    }

    let chain_config = ChainConfig {
        base_fee: 0.into(),
        stake_table_contract: Some(stake_table_address),
        ..state.chain_config.resolve().unwrap_or_default()
    };
    let state = ValidatedState {
        chain_config: chain_config.into(),
        ..state
    };
    Ok((state, contracts))
}

/// Registers and delegates to a batch of new validators mid-run, funding
/// their L1 accounts from the network's deployer signer.
pub async fn register_validators<const NUM_NODES: usize>(
    cfg: &TestConfig<NUM_NODES>,
    stake_table: Address,
    indices: &[usize],
    delegation_config: DelegationConfig,
) -> anyhow::Result<()> {
    let deployer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(cfg.signer()))
        .connect_http(cfg.l1_url());
    StakingTransactions::create(
        cfg.l1_url(),
        &deployer,
        stake_table,
        cfg.staking_key_sets(indices),
        None,
        delegation_config,
    )
    .await?
    .apply_all()
    .await?;
    Ok(())
}

/// Deregisters the validators at the given node indices, each exit sent
/// from that validator's own funded provider.
pub async fn deregister_validators<const NUM_NODES: usize>(
    cfg: &TestConfig<NUM_NODES>,
    stake_table: Address,
    indices: &[usize],
) -> anyhow::Result<()> {
    let providers = cfg.validator_providers();
    for &i in indices {
        let (address, provider) = &providers[i];
        let receipt = StakingTransaction::DeregisterValidator { stake_table }
            .send(provider)
            .await?
            .get_receipt()
            .await?;
        anyhow::ensure!(
            receipt.status(),
            "deregistration of validator {i} ({address}) reverted"
        );
    }
    Ok(())
}

/// Funds a fresh delegator account (ETH via anvil, ESP from the deployer)
/// and delegates `amount` to `validator`. Returns the delegator's provider
/// so the test can later undelegate.
pub async fn delegate_new<const NUM_NODES: usize>(
    cfg: &TestConfig<NUM_NODES>,
    token: Address,
    stake_table: Address,
    validator: Address,
    amount: U256,
) -> anyhow::Result<impl Provider + Clone + use<NUM_NODES>> {
    let deployer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(cfg.signer()))
        .connect_http(cfg.l1_url());
    let signer = PrivateKeySigner::random();
    let delegator = signer.address();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(cfg.l1_url());

    deployer
        .anvil_set_balance(delegator, parse_ether("10").unwrap())
        .await?;
    let funding = StakingTransaction::Transfer {
        token,
        to: delegator,
        amount,
    }
    .send(&deployer)
    .await?
    .get_receipt()
    .await?;
    anyhow::ensure!(funding.status(), "ESP transfer to delegator reverted");

    for tx in [
        StakingTransaction::Approve {
            token,
            spender: stake_table,
            amount,
        },
        StakingTransaction::Delegate {
            stake_table,
            validator,
            amount,
        },
    ] {
        let receipt = tx.send(&provider).await?.get_receipt().await?;
        anyhow::ensure!(receipt.status(), "delegator transaction reverted");
    }
    Ok(provider)
}

/// Waits epoch by epoch, starting at `start_epoch`, until the committee
/// reported by `node/validators/{epoch}` satisfies `pred`. Returns the
/// first matching epoch and its committee; panics after `max_epochs`
/// epochs without a match.
pub async fn wait_for_committee(
    client: &Client<ClientErr, SequencerApiVersion>,
    events: &mut (impl Stream<Item = CoordinatorEvent<SeqTypes>> + Unpin),
    epoch_height: u64,
    start_epoch: u64,
    max_epochs: u64,
    pred: impl Fn(&AuthenticatedValidatorMap) -> bool,
) -> (u64, AuthenticatedValidatorMap) {
    let mut last = None;
    for epoch in start_epoch..start_epoch + max_epochs {
        wait_for_epochs(events, epoch_height, epoch).await;
        let validators = client
            .get::<AuthenticatedValidatorMap>(&format!("node/validators/{epoch}"))
            .send()
            .await
            .expect("validators for a decided epoch");
        if pred(&validators) {
            return (epoch, validators);
        }
        last = Some((epoch, validators));
    }
    let last =
        last.map(|(epoch, validators)| (epoch, validators.keys().copied().collect::<Vec<_>>()));
    panic!(
        "committee predicate not satisfied within {max_epochs} epochs starting at {start_epoch}; \
         last committee: {last:?}"
    );
}

/// The L1 accounts of the validators at the given node indices.
pub fn staking_addresses<const NUM_NODES: usize>(
    cfg: &TestConfig<NUM_NODES>,
    indices: &[usize],
) -> HashSet<Address> {
    cfg.staking_key_sets(indices)
        .iter()
        .map(|keys| keys.signer.address())
        .collect()
}

/// Predicate for [`wait_for_committee`]: the committee is exactly the
/// expected set of validator accounts.
pub fn committee_is(expected: HashSet<Address>) -> impl Fn(&AuthenticatedValidatorMap) -> bool {
    move |validators| validators.keys().copied().collect::<HashSet<_>>() == expected
}

/// Asserts the node is live: it must advance `epochs_ahead` epochs (at
/// least 1) past its current decided epoch, and, when the chain runs the
/// self-building new protocol, sequence a newly submitted transaction.
/// Inclusion is not asserted on legacy versions because the test-only
/// legacy builder stops producing non-empty blocks after roughly a
/// hundred views, independent of any stake table activity.
pub async fn assert_node_live<P: SequencerPersistence>(
    node: &SequencerContext<network::Memory, P>,
    epoch_height: u64,
    epochs_ahead: u64,
) {
    assert!(epochs_ahead > 0, "epochs_ahead must be at least 1");
    let mut events = node.event_stream();
    let leaf = node.decided_leaf().await;
    let current = leaf
        .epoch(epoch_height)
        .map(|epoch| epoch.u64())
        .unwrap_or_default();
    // `wait_for_epochs` returns on the first epoch strictly greater than
    // its target.
    wait_for_epochs(&mut events, epoch_height, current + epochs_ahead - 1).await;

    if node.decided_leaf().await.block_header().version() < versions::NEW_PROTOCOL_VERSION {
        tracing::info!("legacy version: skipping transaction-inclusion liveness check");
        return;
    }
    // Detect inclusion via the header's namespace table: the namespace is
    // unique to this call, and decide events at 0.6 do not always carry
    // payloads.
    static NAMESPACE_COUNTER: AtomicU32 = AtomicU32::new(10_101);
    let namespace = NamespaceId::from(NAMESPACE_COUNTER.fetch_add(1, Ordering::Relaxed));
    let tx = Transaction::new(namespace, vec![7; 8]);
    node.submit_transaction(tx)
        .await
        .expect("live node accepts transactions");
    tokio::time::timeout(Duration::from_secs(120), async {
        loop {
            let leaf = match events.next().await.unwrap() {
                CoordinatorEvent::LegacyEvent(Event {
                    event: EventType::Decide { leaf_chain, .. },
                    ..
                }) => leaf_chain[0].leaf.clone(),
                CoordinatorEvent::NewDecide { leaf_infos, .. } => leaf_infos[0].leaf.clone(),
                _ => continue,
            };
            if leaf
                .block_header()
                .ns_table()
                .find_ns_id(&namespace)
                .is_some()
            {
                tracing::info!(height = leaf.height(), "transaction namespace sequenced");
                return;
            }
        }
    })
    .await
    .expect("submitted transaction was not sequenced in time");
}

/// Asserts every node has decided at least `min_height`, and that nodes
/// which have decided the same height agree on the leaf.
pub async fn assert_nodes_agree<P: SequencerPersistence>(
    nodes: &[&SequencerContext<network::Memory, P>],
    min_height: u64,
) {
    let leaves = join_all(nodes.iter().map(|node| node.decided_leaf())).await;
    let mut by_height: std::collections::BTreeMap<u64, Commitment<Leaf2>> = Default::default();
    for (i, leaf) in leaves.iter().enumerate() {
        assert!(
            leaf.height() >= min_height,
            "node {i} decided height {} is below {min_height}",
            leaf.height()
        );
        if let Some(other) = by_height.insert(leaf.height(), leaf.commit()) {
            assert_eq!(
                other,
                leaf.commit(),
                "decided-leaf divergence at height {}",
                leaf.height()
            );
        }
    }
}

/// Test the status API with custom options.
///
/// The `opt` function can be used to modify the [`Options`] which are used to start the server.
/// By default, the options are the minimal required to run this test (configuring a port and
/// enabling the status API). `opt` may add additional functionality (e.g. adding a query module
/// to test a different initialization path) but should not remove or modify the existing
/// functionality (e.g. removing the status module or changing the port).
pub async fn status_test_helper(opt: impl FnOnce(Options) -> Options) {
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let url = format!("http://localhost:{port}").parse().unwrap();
    let client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(url);

    let options = opt(Options::with_port(port));
    let network_config = TestConfigBuilder::default().build();
    let config = TestNetworkConfigBuilder::default()
        .api_config(options)
        .network_config(network_config)
        .build();
    let network = TestNetwork::new(config, TEST_UPGRADE).await;
    client.connect(None).await;

    // The status API is well tested in the query service repo. Here we are just smoke testing
    // that we set it up correctly. Wait for a (non-genesis) block to be sequenced and then
    // check the success rate metrics.
    while client
        .get::<u64>("status/block-height")
        .send()
        .await
        .unwrap()
        <= 1
    {
        sleep(Duration::from_secs(1)).await;
    }
    let success_rate = client
        .get::<f64>("status/success-rate")
        .send()
        .await
        .unwrap();
    // If metrics are populating correctly, we should get a finite number. If not, we might get
    // NaN or infinity due to division by 0.
    assert!(success_rate.is_finite(), "{success_rate}");
    // We know at least some views have been successful, since we finalized a block.
    assert!(success_rate > 0.0, "{success_rate}");

    let keys: NodePublicKeys = client.get("status/keys").send().await.unwrap();
    let expected = network.server.validator_config();
    assert_eq!(keys.consensus_key, expected.public_key);
    assert_eq!(keys.state_ver_key, expected.state_public_key);
    assert_eq!(
        keys.x25519_key,
        expected.x25519_keypair.as_ref().map(|kp| kp.public_key())
    );
    assert_eq!(keys.p2p_addr, expected.p2p_addr);
    assert!(keys.p2p_addr.is_some());
    assert_eq!(keys.eth_account, None);

    let json: serde_json::Value = client.get("status/keys").send().await.unwrap();
    let bls = json["consensus_key"].as_str().unwrap();
    assert!(bls.starts_with("BLS_VER_KEY~"), "{bls}");
    let schnorr = json["state_ver_key"].as_str().unwrap();
    assert!(schnorr.starts_with("SCHNORR_VER_KEY~"), "{schnorr}");
    let x25519 = json["x25519_key"].as_str().unwrap();
    assert!(x25519.starts_with("X25519_PK~"), "{x25519}");
    assert_eq!(
        json["p2p_addr"].as_str().unwrap(),
        expected.p2p_addr.as_ref().unwrap().to_string()
    );
}

/// Test the submit API with custom options.
///
/// The `opt` function can be used to modify the [`Options`] which are used to start the server.
/// By default, the options are the minimal required to run this test (configuring a port and
/// enabling the submit API). `opt` may add additional functionality (e.g. adding a query module
/// to test a different initialization path) but should not remove or modify the existing
/// functionality (e.g. removing the submit module or changing the port).
pub async fn submit_test_helper(opt: impl FnOnce(Options) -> Options) {
    let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3, 4]);

    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    let url = format!("http://localhost:{port}").parse().unwrap();
    let client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(url);

    let options = opt(Options::with_port(port).submit(Default::default()));
    let network_config = TestConfigBuilder::default().build();
    let config = TestNetworkConfigBuilder::default()
        .api_config(options)
        .network_config(network_config)
        .build();
    let network = TestNetwork::new(config, TEST_UPGRADE).await;
    let mut events = network.server.event_stream();

    client.connect(None).await;

    let hash = client
        .post("submit/submit")
        .body_json(&txn)
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(txn.commit(), hash);

    // Wait for a Decide event containing transaction matching the one we sent
    wait_for_decide_on_handle(&mut events, &txn).await;
}

/// Test the state signature API.
pub async fn state_signature_test_helper(opt: impl FnOnce(Options) -> Options) {
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    let url = format!("http://localhost:{port}").parse().unwrap();

    let client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(url);

    let options = opt(Options::with_port(port));
    let network_config = TestConfigBuilder::default().build();
    let config = TestNetworkConfigBuilder::default()
        .api_config(options)
        .network_config(network_config)
        .build();
    let network = TestNetwork::new(config, TEST_UPGRADE).await;

    let mut height: u64;
    // Wait for block >=2 appears
    // It's waiting for an extra second to make sure that the signature is generated
    loop {
        height = network.server.decided_leaf().await.height();
        sleep(std::time::Duration::from_secs(1)).await;
        if height >= 2 {
            break;
        }
    }
    // we cannot verify the signature now, because we don't know the stake table
    client
        .get::<LCV3StateSignatureRequestBody>(&format!("state-signature/block/{height}"))
        .send()
        .await
        .unwrap();
}

/// Test the catchup API with custom options.
///
/// The `opt` function can be used to modify the [`Options`] which are used to start the server.
/// By default, the options are the minimal required to run this test (configuring a port and
/// enabling the catchup API). `opt` may add additional functionality (e.g. adding a query module
/// to test a different initialization path) but should not remove or modify the existing
/// functionality (e.g. removing the catchup module or changing the port).
pub async fn catchup_test_helper(opt: impl FnOnce(Options) -> Options) {
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let url = format!("http://localhost:{port}").parse().unwrap();
    let client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(url);

    let options = opt(Options::with_port(port));
    let network_config = TestConfigBuilder::default().build();
    let config = TestNetworkConfigBuilder::default()
        .api_config(options)
        .network_config(network_config)
        .build();
    let network = TestNetwork::new(config, TEST_UPGRADE).await;
    client.connect(None).await;

    // Wait for a few blocks to be decided.
    let mut events = network.server.event_stream();
    loop {
        if let Some(leaf_chain) = decided_leaves(&events.next().await.unwrap())
            && leaf_chain
                .iter()
                .any(|LeafInfo { leaf, .. }| leaf.block_header().height() > 2)
        {
            break;
        }
    }

    // Consensus keeps running: at 0.6 stopping it drops every in-memory state. So the
    // leaf can be decided and garbage collected between reading its state and serving the
    // request; retry on a fresh undecided leaf when that happens.
    for attempt in 0.. {
        assert!(attempt < 50, "never served catchup for an undecided state");
        if try_undecided_catchup(&network, &client).await {
            return;
        }
        sleep(Duration::from_millis(100)).await;
    }
}

/// Checks the catchup API against the newest undecided state, or returns `false` if there
/// is none or it went out of memory mid-check.
async fn try_undecided_catchup<P: PersistenceOptions, const NUM_NODES: usize>(
    network: &TestNetwork<P, NUM_NODES>,
    client: &Client<ClientErr, StaticVersion<0, 1>>,
) -> bool {
    let undecided = network.server.consensus_handle().undecided_leaves().await;
    let Some(leaf) = undecided.iter().max_by_key(|leaf| leaf.height()) else {
        return false;
    };
    let (height, view) = (leaf.height(), leaf.view_number());
    let Some(state) = network.server.state(view).await else {
        return false;
    };

    // Undecided fee state: absent account.
    let Ok(res) = client
        .get::<AccountQueryData>(&format!(
            "catchup/{height}/{}/account/{:x}",
            view.u64(),
            Address::default()
        ))
        .send()
        .await
    else {
        return false;
    };
    assert_eq!(res.balance, U256::ZERO);
    assert_eq!(
        res.proof
            .verify(&state.fee_merkle_tree.commitment())
            .unwrap(),
        U256::ZERO,
    );

    // Undecided block state.
    let Ok(res) = client
        .get::<BlocksFrontier>(&format!("catchup/{height}/{}/blocks", view.u64()))
        .send()
        .await
    else {
        return false;
    };
    let root = &state.block_merkle_tree.commitment();
    BlockMerkleTree::verify(root, root.size() - 1, res)
        .unwrap()
        .unwrap();
    true
}
