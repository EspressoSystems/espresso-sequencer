use std::pin::Pin;

use anyhow::{bail, Context};
use async_once_cell::Lazy;
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use committable::Commitment;
use data_source::{CatchupDataSource, SubmitDataSource};
use derivative::Derivative;
use espresso_types::{
    v0::traits::SequencerPersistence, AccountQueryData, BlockMerkleTree, ChainConfig,
    FeeAccountProof, NodeState, PubKey, Transaction,
};
use ethers::prelude::Address;
use futures::{
    future::{BoxFuture, Future, FutureExt},
    stream::{BoxStream, Stream},
};
use hotshot::types::{Event, SystemContextHandle};
use hotshot_events_service::events_source::{
    EventFilterSet, EventsSource, EventsStreamer, StartupInfo,
};
use hotshot_orchestrator::config::NetworkConfig;
use hotshot_query_service::data_source::ExtensibleDataSource;
use hotshot_state_prover::service::light_client_genesis_from_stake_table;
use hotshot_types::{
    data::ViewNumber, light_client::StateSignatureRequestBody, traits::network::ConnectedNetwork,
};
use jf_merkle_tree::MerkleTreeScheme;
use vbs::version::StaticVersionType;

use self::data_source::{HotShotConfigDataSource, PublicNetworkConfig, StateSignatureDataSource};
use crate::{
    network, persistence::ChainConfigPersistence, state_signature::StateSigner, Node, SeqTypes,
    SequencerContext,
};

pub mod data_source;
pub mod endpoints;
pub mod fs;
pub mod options;
pub mod sql;
mod update;

pub use options::Options;

pub type BlocksFrontier = <BlockMerkleTree as MerkleTreeScheme>::MembershipProof;

type BoxLazy<T> = Pin<Arc<Lazy<T, BoxFuture<'static, T>>>>;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
struct ConsensusState<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType>
{
    state_signer: Arc<StateSigner<Ver>>,
    event_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,
    node_state: NodeState,
    config: NetworkConfig<PubKey>,

    #[derivative(Debug = "ignore")]
    handle: Arc<RwLock<SystemContextHandle<SeqTypes, Node<N, P>>>>,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    From<&SequencerContext<N, P, Ver>> for ConsensusState<N, P, Ver>
{
    fn from(ctx: &SequencerContext<N, P, Ver>) -> Self {
        Self {
            state_signer: ctx.state_signer(),
            event_streamer: ctx.event_streamer(),
            node_state: ctx.node_state(),
            config: ctx.config(),
            handle: ctx.consensus(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
struct ApiState<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType> {
    // The consensus state is initialized lazily so we can start the API (and healthcheck endpoints)
    // before consensus has started. Any endpoint that uses consensus state will wait for
    // initialization to finish, but endpoints that do not require a consensus handle can proceed
    // without waiting.
    #[derivative(Debug = "ignore")]
    consensus: BoxLazy<ConsensusState<N, P, Ver>>,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    ApiState<N, P, Ver>
{
    fn new(init: impl Future<Output = ConsensusState<N, P, Ver>> + Send + 'static) -> Self {
        Self {
            consensus: Arc::pin(Lazy::from_future(init.boxed())),
        }
    }

    fn event_stream(&self) -> impl Stream<Item = Event<SeqTypes>> + Unpin {
        let state = self.clone();
        async move { state.consensus().await.read().await.event_stream() }
            .boxed()
            .flatten_stream()
    }

    async fn state_signer(&self) -> &StateSigner<Ver> {
        &self.consensus.as_ref().get().await.get_ref().state_signer
    }

    async fn event_streamer(&self) -> &RwLock<EventsStreamer<SeqTypes>> {
        &self.consensus.as_ref().get().await.get_ref().event_streamer
    }

    async fn consensus(&self) -> Arc<RwLock<SystemContextHandle<SeqTypes, Node<N, P>>>> {
        Arc::clone(&self.consensus.as_ref().get().await.get_ref().handle)
    }

    async fn node_state(&self) -> &NodeState {
        &self.consensus.as_ref().get().await.get_ref().node_state
    }

    async fn network_config(&self) -> NetworkConfig<PubKey> {
        self.consensus.as_ref().get().await.get_ref().config.clone()
    }
}

type StorageState<N, P, D, Ver> = ExtensibleDataSource<D, ApiState<N, P, Ver>>;

#[async_trait]
impl<N: ConnectedNetwork<PubKey>, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    EventsSource<SeqTypes> for ApiState<N, P, Ver>
{
    type EventStream = BoxStream<'static, Arc<Event<SeqTypes>>>;

    async fn get_event_stream(
        &self,
        _filter: Option<EventFilterSet<SeqTypes>>,
    ) -> Self::EventStream {
        self.event_streamer()
            .await
            .read()
            .await
            .get_event_stream(None)
            .await
    }
    async fn get_startup_info(&self) -> StartupInfo<SeqTypes> {
        self.event_streamer()
            .await
            .read()
            .await
            .get_startup_info()
            .await
    }
}

impl<
        N: ConnectedNetwork<PubKey>,
        D: Send + Sync,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
    > SubmitDataSource<N, P> for StorageState<N, P, D, Ver>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        self.as_ref().submit(tx).await
    }
}

impl<N: ConnectedNetwork<PubKey>, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    SubmitDataSource<N, P> for ApiState<N, P, Ver>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        self.consensus()
            .await
            .read()
            .await
            .submit_transaction(tx)
            .await?;
        Ok(())
    }
}

impl<
        N: ConnectedNetwork<PubKey>,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
        D: CatchupDataSource + Send + Sync,
    > CatchupDataSource for StorageState<N, P, D, Ver>
{
    #[tracing::instrument(skip(self))]
    async fn get_account(
        &self,
        height: u64,
        view: ViewNumber,
        account: Address,
    ) -> anyhow::Result<AccountQueryData> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_account(height, view, account).await {
            Ok(account) => return Ok(account),
            Err(err) => {
                tracing::info!("account is not in memory, trying storage: {err:#}");
            }
        }

        // Try storage.
        self.inner().get_account(height, view, account).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_frontier(height, view).await {
            Ok(frontier) => return Ok(frontier),
            Err(err) => {
                tracing::info!("frontier is not in memory, trying storage: {err:#}");
            }
        }

        // Try storage.
        self.inner().get_frontier(height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_chain_config(commitment).await {
            Ok(cf) => return Ok(cf),
            Err(err) => {
                tracing::info!("chain config is not in memory, trying storage: {err:#}");
            }
        }

        // Try storage.
        self.inner().get_chain_config(commitment).await
    }
}

#[async_trait]
impl<
        N: ConnectedNetwork<PubKey>,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
        D: ChainConfigPersistence + Send + Sync,
    > ChainConfigPersistence for StorageState<N, P, D, Ver>
{
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        self.inner_mut().insert_chain_config(chain_config).await
    }
    async fn load_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.inner().load_chain_config(commitment).await
    }
}

impl<N: ConnectedNetwork<PubKey>, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    CatchupDataSource for ApiState<N, P, Ver>
{
    #[tracing::instrument(skip(self))]
    async fn get_account(
        &self,
        height: u64,
        view: ViewNumber,
        account: Address,
    ) -> anyhow::Result<AccountQueryData> {
        let state = self
            .consensus()
            .await
            .read()
            .await
            .state(view)
            .await
            .context(format!(
                "state not available for height {height}, view {view:?}"
            ))?;
        let (proof, balance) = FeeAccountProof::prove(&state.fee_merkle_tree, account).context(
            format!("account {account} not available for height {height}, view {view:?}"),
        )?;
        Ok(AccountQueryData { balance, proof })
    }

    #[tracing::instrument(skip(self))]
    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        let state = self
            .consensus()
            .await
            .read()
            .await
            .state(view)
            .await
            .context(format!(
                "state not available for height {height}, view {view:?}"
            ))?;
        let tree = &state.block_merkle_tree;
        let frontier = tree.lookup(tree.num_leaves() - 1).expect_ok()?.1;
        Ok(frontier)
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let state = self.consensus().await.read().await.decided_state().await;
        let chain_config = state.chain_config;

        if chain_config.commit() == commitment {
            chain_config.resolve().context("chain config found")
        } else {
            bail!("chain config not found")
        }
    }
}

impl<
        N: ConnectedNetwork<PubKey>,
        D: Sync,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
    > HotShotConfigDataSource for StorageState<N, P, D, Ver>
{
    async fn get_config(&self) -> PublicNetworkConfig {
        self.as_ref().network_config().await.into()
    }
}

impl<N: ConnectedNetwork<PubKey>, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    HotShotConfigDataSource for ApiState<N, P, Ver>
{
    async fn get_config(&self) -> PublicNetworkConfig {
        self.network_config().await.into()
    }
}

#[async_trait]
impl<
        N: ConnectedNetwork<PubKey>,
        D: Sync,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
    > StateSignatureDataSource<N> for StorageState<N, P, D, Ver>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.as_ref().get_state_signature(height).await
    }
}

#[async_trait]
impl<N: ConnectedNetwork<PubKey>, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    StateSignatureDataSource<N> for ApiState<N, P, Ver>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.state_signer().await.get_state_signature(height).await
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod test_helpers {
    use std::time::Duration;

    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use committable::Committable;
    use es_version::{SequencerVersion, SEQUENCER_VERSION};
    use espresso_types::{
        mock::MockStateCatchup,
        v0::traits::{PersistenceOptions, StateCatchup},
        NamespaceId, ValidatedState,
    };
    use ethers::{prelude::Address, utils::Anvil};
    use futures::{
        future::{join_all, FutureExt},
        stream::StreamExt,
    };
    use hotshot::types::{Event, EventType};
    use hotshot_contract_adapter::light_client::ParsedLightClientState;
    use hotshot_types::{
        event::LeafInfo,
        traits::{metrics::NoMetrics, node_implementation::ConsensusTime},
    };
    use itertools::izip;
    use jf_merkle_tree::{MerkleCommitment, MerkleTreeScheme};
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    use super::*;
    use crate::{
        persistence::no_storage,
        testing::{run_test_builder, wait_for_decide_on_handle, TestConfig, TestConfigBuilder},
    };

    pub const STAKE_TABLE_CAPACITY_FOR_TEST: u64 = 10;

    pub struct TestNetwork<P: PersistenceOptions, const NUM_NODES: usize> {
        pub server: SequencerContext<network::Memory, P::Persistence, SequencerVersion>,
        pub peers: Vec<SequencerContext<network::Memory, P::Persistence, SequencerVersion>>,
        pub cfg: TestConfig<{ NUM_NODES }>,
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
    }

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
    }

    impl Default for TestNetworkConfigBuilder<5, no_storage::Options, MockStateCatchup> {
        fn default() -> Self {
            TestNetworkConfigBuilder {
                state: std::array::from_fn(|_| ValidatedState::default()),
                persistence: Some([no_storage::Options; 5]),
                catchup: Some(std::array::from_fn(|_| MockStateCatchup::default())),
                network_config: None,
                api_config: None,
            }
        }
    }

    impl<const NUM_NODES: usize>
        TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, MockStateCatchup>
    {
        pub fn with_num_nodes(
        ) -> TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, MockStateCatchup>
        {
            TestNetworkConfigBuilder {
                state: std::array::from_fn(|_| ValidatedState::default()),
                persistence: Some([no_storage::Options; { NUM_NODES }]),
                catchup: Some(std::array::from_fn(|_| MockStateCatchup::default())),
                network_config: None,
                api_config: None,
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
            }
        }

        pub fn network_config(mut self, network_config: TestConfig<{ NUM_NODES }>) -> Self {
            self.network_config = Some(network_config);
            self
        }

        pub fn build(self) -> TestNetworkConfig<{ NUM_NODES }, P, C> {
            TestNetworkConfig {
                state: self.state,
                persistence: self.persistence.unwrap(),
                catchup: self.catchup.unwrap(),
                network_config: self.network_config.unwrap(),
                api_config: self.api_config.unwrap(),
            }
        }
    }

    impl<P: PersistenceOptions, const NUM_NODES: usize> TestNetwork<P, { NUM_NODES }> {
        pub async fn new<C: StateCatchup + 'static>(
            cfg: TestNetworkConfig<{ NUM_NODES }, P, C>,
        ) -> Self {
            let mut cfg = cfg;
            let (builder_task, builder_url) =
                run_test_builder::<{ NUM_NODES }>(cfg.network_config.builder_port()).await;

            cfg.network_config
                .set_builder_urls(vec1::vec1![builder_url]);

            let mut nodes = join_all(
                izip!(cfg.state, cfg.persistence, cfg.catchup)
                    .enumerate()
                    .map(|(i, (state, persistence, catchup))| {
                        let opt = cfg.api_config.clone();
                        let cfg = &cfg.network_config;
                        let upgrades_map = cfg.upgrades();
                        async move {
                            if i == 0 {
                                opt.serve(
                                    |metrics| {
                                        let cfg = cfg.clone();
                                        async move {
                                            cfg.init_node(
                                                0,
                                                state,
                                                persistence,
                                                catchup,
                                                &*metrics,
                                                STAKE_TABLE_CAPACITY_FOR_TEST,
                                                SEQUENCER_VERSION,
                                                upgrades_map,
                                            )
                                            .await
                                        }
                                        .boxed()
                                    },
                                    SEQUENCER_VERSION,
                                )
                                .await
                                .unwrap()
                            } else {
                                cfg.init_node(
                                    i,
                                    state,
                                    persistence,
                                    catchup,
                                    &NoMetrics,
                                    STAKE_TABLE_CAPACITY_FOR_TEST,
                                    SEQUENCER_VERSION,
                                    upgrades_map,
                                )
                                .await
                            }
                        }
                    }),
            )
            .await;

            let handle_0 = &nodes[0];

            // Hook the builder up to the event stream from the first node
            builder_task.start(Box::new(handle_0.event_stream().await));

            for ctx in &nodes {
                ctx.start_consensus().await;
            }

            let server = nodes.remove(0);
            let peers = nodes;

            Self {
                server,
                peers,
                cfg: cfg.network_config,
            }
        }

        pub fn light_client_genesis(&self) -> ParsedLightClientState {
            let st = self.cfg.stake_table();
            light_client_genesis_from_stake_table(st).unwrap()
        }

        pub async fn stop_consensus(&mut self) {
            self.server.shutdown_consensus().await;

            for ctx in &mut self.peers {
                ctx.shutdown_consensus().await;
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
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = opt(Options::with_port(port).status(Default::default()));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config).await;
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
    }

    /// Test the submit API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the submit API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the submit module or changing the port).
    pub async fn submit_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_logging();
        setup_backtrace();

        let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = opt(Options::with_port(port).submit(Default::default()));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config).await;
        let mut events = network.server.event_stream().await;

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
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = opt(Options::with_port(port));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config).await;

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
            .get::<StateSignatureRequestBody>(&format!("state-signature/block/{}", height))
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
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = opt(Options::with_port(port).catchup(Default::default()));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config).await;
        client.connect(None).await;

        // Wait for a few blocks to be decided.
        let mut events = network.server.event_stream().await;
        loop {
            if let Event {
                event: EventType::Decide { leaf_chain, .. },
                ..
            } = events.next().await.unwrap()
            {
                if leaf_chain
                    .iter()
                    .any(|LeafInfo { leaf, .. }| leaf.block_header().height() > 2)
                {
                    break;
                }
            }
        }

        // Stop consensus running on the node so we freeze the decided and undecided states.
        // We'll let it go out of scope here since it's a write lock.
        {
            network.server.shutdown_consensus().await;
        }

        // Undecided fee state: absent account.
        let leaf = network.server.decided_leaf().await;
        let height = leaf.height() + 1;
        let view = leaf.view_number() + 1;
        let res = client
            .get::<AccountQueryData>(&format!(
                "catchup/{height}/{}/account/{:x}",
                view.u64(),
                Address::default()
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(res.balance, 0.into());
        assert_eq!(
            res.proof
                .verify(
                    &network
                        .server
                        .state(view)
                        .await
                        .unwrap()
                        .fee_merkle_tree
                        .commitment()
                )
                .unwrap(),
            0.into()
        );

        // Undecided block state.
        let res = client
            .get::<BlocksFrontier>(&format!("catchup/{height}/{}/blocks", view.u64()))
            .send()
            .await
            .unwrap();
        let root = &network
            .server
            .state(view)
            .await
            .unwrap()
            .block_merkle_tree
            .commitment();
        BlockMerkleTree::verify(root.digest(), root.size() - 1, res)
            .unwrap()
            .unwrap();
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod api_tests {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use committable::Committable;
    use data_source::testing::TestableSequencerDataSource;
    use endpoints::NamespaceProofQueryData;
    use es_version::SequencerVersion;
    use espresso_types::{Header, NamespaceId};
    use ethers::utils::Anvil;
    use futures::stream::StreamExt;
    use hotshot_query_service::availability::{LeafQueryData, VidCommonQueryData};
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use test_helpers::{
        catchup_test_helper, state_signature_test_helper, status_test_helper, submit_test_helper,
        TestNetwork, TestNetworkConfigBuilder,
    };
    use tide_disco::error::ServerError;

    use self::options::HotshotEvents;
    use super::*;
    use crate::testing::{wait_for_decide_on_handle, TestConfigBuilder};

    #[async_std::test]
    pub(crate) async fn submit_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        submit_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn status_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        status_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn state_signature_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        state_signature_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn test_namespace_query<D: TestableSequencerDataSource>() {
        setup_logging();
        setup_backtrace();

        // Arbitrary transaction, arbitrary namespace ID
        let ns_id = NamespaceId::from(42_u32);
        let txn = Transaction::new(ns_id, vec![1, 2, 3, 4]);

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let storage = D::create_storage().await;
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(D::options(&storage, Options::with_port(port)).submit(Default::default()))
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config).await;
        let mut events = network.server.event_stream().await;

        // Connect client.
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        // Wait for at least one empty block to be sequenced (after consensus starts VID).
        client
            .socket("availability/stream/leaves/0")
            .subscribe::<LeafQueryData<SeqTypes>>()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        let block_height = wait_for_decide_on_handle(&mut events, &txn).await as usize;
        tracing::info!(block_height, "transaction sequenced");
        let mut found_txn = false;
        let mut found_empty_block = false;
        for block_num in 0..=block_height {
            let header: Header = client
                .get(&format!("availability/header/{block_num}"))
                .send()
                .await
                .unwrap();
            let ns_query_res: NamespaceProofQueryData = client
                .get(&format!("availability/block/{block_num}/namespace/{ns_id}"))
                .send()
                .await
                .unwrap();

            // Verify namespace proof if present
            if let Some(ns_proof) = ns_query_res.proof {
                let vid_common: VidCommonQueryData<SeqTypes> = client
                    .get(&format!("availability/vid/common/{block_num}"))
                    .send()
                    .await
                    .unwrap();

                ns_proof
                    .verify(
                        header.ns_table(),
                        &header.payload_commitment(),
                        vid_common.common(),
                    )
                    .unwrap();
            } else {
                // Namespace proof should be present if ns_id exists in ns_table
                assert!(header.ns_table().find_ns_id(&ns_id).is_none());
                assert!(ns_query_res.transactions.is_empty());
            }

            found_empty_block = found_empty_block || ns_query_res.transactions.is_empty();

            for txn in ns_query_res.transactions {
                if txn.commit() == hash {
                    // Ensure that we validate an inclusion proof
                    found_txn = true;
                }
            }
        }
        assert!(found_txn);
        assert!(found_empty_block);
    }

    #[async_std::test]
    pub(crate) async fn catchup_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        catchup_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn test_hotshot_event_streaming<D: TestableSequencerDataSource>() {
        use HotshotEvents;

        setup_logging();

        setup_backtrace();

        let hotshot_event_streaming_port =
            pick_unused_port().expect("No ports free for hotshot event streaming");
        let query_service_port = pick_unused_port().expect("No ports free for query service");

        let url = format!("http://localhost:{hotshot_event_streaming_port}")
            .parse()
            .unwrap();

        let hotshot_events = HotshotEvents {
            events_service_port: hotshot_event_streaming_port,
        };

        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = Options::with_port(query_service_port).hotshot_events(hotshot_events);

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config).await;

        let mut subscribed_events = client
            .socket("hotshot-events/events")
            .subscribe::<Event<SeqTypes>>()
            .await
            .unwrap();

        let total_count = 5;
        // wait for these events to receive on client 1
        let mut receive_count = 0;
        loop {
            let event = subscribed_events.next().await.unwrap();
            tracing::info!(
                "Received event in hotshot event streaming Client 1: {:?}",
                event
            );
            receive_count += 1;
            if receive_count > total_count {
                tracing::info!("Client Received atleast desired events, exiting loop");
                break;
            }
        }
        assert_eq!(receive_count, total_count);
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use committable::{Commitment, Committable};
    use es_version::{SequencerVersion, SEQUENCER_VERSION};
    use espresso_types::{
        mock::MockStateCatchup,
        v0_1::{UpgradeMode, ViewBasedUpgrade},
        FeeAccount, FeeAmount, Header, Upgrade, UpgradeType, ValidatedState,
    };
    use ethers::utils::Anvil;
    use futures::{
        future::{self, join_all},
        stream::{StreamExt, TryStreamExt},
    };
    use hotshot::types::EventType;
    use hotshot_query_service::{
        availability::{BlockQueryData, LeafQueryData},
        types::HeightIndexed,
    };
    use hotshot_types::{
        event::LeafInfo,
        traits::{
            metrics::NoMetrics,
            node_implementation::{ConsensusTime, NodeType},
        },
        ValidatorConfig,
    };
    use jf_merkle_tree::prelude::{MerkleProof, Sha3Node};
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use test_helpers::{
        catchup_test_helper, state_signature_test_helper, status_test_helper, submit_test_helper,
        TestNetwork, TestNetworkConfigBuilder,
    };
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};
    use vbs::version::Version;

    use self::{
        data_source::{testing::TestableSequencerDataSource, PublicHotShotConfig},
        sql::DataSource as SqlDataSource,
    };
    use super::*;
    use crate::{
        catchup::StatePeers,
        persistence::no_storage,
        testing::{TestConfig, TestConfigBuilder},
    };

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);
        let options = Options::with_port(port);
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::<5, _, MockStateCatchup>::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config).await;

        client.connect(None).await;
        let health = client.get::<AppHealth>("healthcheck").send().await.unwrap();
        assert_eq!(health.status, HealthStatus::Available);
    }

    #[async_std::test]
    async fn status_test_without_query_module() {
        status_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn submit_test_without_query_module() {
        submit_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn state_signature_test_without_query_module() {
        state_signature_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn catchup_test_without_query_module() {
        catchup_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn test_merklized_state_api() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");

        let storage = SqlDataSource::create_storage().await;
        let options = SqlDataSource::options(
            &storage,
            Options::with_port(port)
                .state(Default::default())
                .status(Default::default()),
        );

        let anvil: ethers::utils::AnvilInstance = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let mut network = TestNetwork::new(config).await;
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        client.connect(None).await;

        // Wait until some blocks have been decided.
        tracing::info!("waiting for blocks");
        let blocks = client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(4)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        // sleep for few seconds so that state data is upserted
        tracing::info!("waiting for state to be inserted");
        sleep(Duration::from_secs(5)).await;
        network.stop_consensus().await;

        for block in blocks {
            let i = block.height();
            tracing::info!(i, "get block state");
            let path = client
                .get::<MerkleProof<Commitment<Header>, u64, Sha3Node, 3>>(&format!(
                    "block-state/{}/{i}",
                    i + 1
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.elem().unwrap(), block.hash());

            tracing::info!(i, "get fee state");
            let account = TestConfig::<5>::builder_key().fee_account();
            let path = client
                .get::<MerkleProof<FeeAmount, FeeAccount, Sha3Node, 256>>(&format!(
                    "fee-state/{}/{}",
                    i + 1,
                    account
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.index(), account);
            assert!(*path.elem().unwrap() > 0.into(), "{:?}", path.elem());
        }
    }

    #[async_std::test]
    async fn test_catchup() {
        setup_logging();
        setup_backtrace();

        // Start a sequencer network, using the query service for catchup.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::with_port(port).catchup(Default::default()))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .catchups(std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                )
            }))
            .build();
        let mut network = TestNetwork::new(config).await;

        // Wait for replica 0 to reach a (non-genesis) decide, before disconnecting it.
        let mut events = network.peers[0].event_stream().await;
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            if leaf_chain[0].leaf.height() > 0 {
                break;
            }
        }

        // Shut down and restart replica 0. We don't just stop consensus and restart it; we fully
        // drop the node and recreate it so it loses all of its temporary state and starts off from
        // genesis. It should be able to catch up by listening to proposals and then rebuild its
        // state from its peers.
        tracing::info!("shutting down node");
        network.peers.remove(0);

        // Wait for a few blocks to pass while the node is down, so it falls behind.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        tracing::info!("restarting node");
        let node = network
            .cfg
            .init_node(
                1,
                ValidatedState::default(),
                no_storage::Options,
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                ),
                &NoMetrics,
                test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                SEQUENCER_VERSION,
                Default::default(),
            )
            .await;
        let mut events = node.event_stream().await;

        // Wait for a (non-genesis) block proposed by each node, to prove that the lagging node has
        // caught up and all nodes are in sync.
        let mut proposers = [false; NUM_NODES];
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let height = leaf.height();
                let leaf_builder = (leaf.view_number().u64() as usize) % NUM_NODES;
                if height == 0 {
                    continue;
                }

                tracing::info!(
                    "waiting for blocks from {proposers:?}, block {height} is from {leaf_builder}",
                );
                proposers[leaf_builder] = true;
            }

            if proposers.iter().all(|has_proposed| *has_proposed) {
                break;
            }
        }
    }

    #[async_std::test]
    async fn test_chain_config_from_instance() {
        // This test uses a ValidatedState which only has the default chain config commitment.
        // The NodeState has the full chain config.
        // Both chain config commitments will match, so the ValidatedState should have the full chain config after a non-genesis block is decided.
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let chain_config: ChainConfig = ChainConfig::default();

        let state = ValidatedState {
            chain_config: chain_config.commit().into(),
            ..Default::default()
        };

        let states = std::array::from_fn(|_| state.clone());

        let config = TestNetworkConfigBuilder::default()
            .api_config(Options::with_port(port).catchup(Default::default()))
            .states(states)
            .catchups(std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();

        let mut network = TestNetwork::new(config).await;

        // Wait for few blocks to be decided.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        for peer in &network.peers {
            let state = peer.consensus().read().await.decided_state().await;

            assert_eq!(state.chain_config.resolve().unwrap(), chain_config)
        }

        network.server.shut_down().await;
        drop(network);
    }

    #[async_std::test]
    async fn test_chain_config_catchup() {
        // This test uses a ValidatedState with a non-default chain config
        // so it will be different from the NodeState chain config used by the TestNetwork.
        // However, for this test to work, at least one node should have a full chain config
        // to allow other nodes to catch up.

        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let cf = ChainConfig {
            max_block_size: 300.into(),
            base_fee: 1.into(),
            ..Default::default()
        };

        // State1 contains only the chain config commitment
        let state1 = ValidatedState {
            chain_config: cf.commit().into(),
            ..Default::default()
        };

        //state 2 contains the full chain config
        let state2 = ValidatedState {
            chain_config: cf.into(),
            ..Default::default()
        };

        let mut states = std::array::from_fn(|_| state1.clone());
        // only one node has the full chain config
        // all the other nodes should do a catchup to get the full chain config from peer 0
        states[0] = state2;

        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(
                Options::from(options::Http {
                    port,
                    max_connections: None,
                })
                .catchup(Default::default()),
            )
            .states(states)
            .catchups(std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();

        let mut network = TestNetwork::new(config).await;

        // Wait for a few blocks to be decided.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        for peer in &network.peers {
            let state = peer.consensus().read().await.decided_state().await;

            assert_eq!(state.chain_config.resolve().unwrap(), cf)
        }

        network.server.shut_down().await;
        drop(network);
    }

    #[async_std::test]
    async fn test_chain_config_upgrade() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let chain_config_upgrade = ChainConfig {
            max_block_size: 300.into(),
            base_fee: 1.into(),
            ..Default::default()
        };
        let mut upgrades = std::collections::BTreeMap::new();

        upgrades.insert(
            <SeqTypes as NodeType>::Upgrade::VERSION,
            Upgrade {
                mode: UpgradeMode::View(ViewBasedUpgrade {
                    start_voting_view: None,
                    stop_voting_view: None,
                    start_proposing_view: 1,
                    stop_proposing_view: 10,
                }),
                upgrade_type: UpgradeType::ChainConfig {
                    chain_config: chain_config_upgrade,
                },
            },
        );

        let stop_voting_view = u64::MAX;

        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(
                Options::from(options::Http {
                    port,
                    max_connections: None,
                })
                .catchup(Default::default())
                .status(Default::default()),
            )
            .catchups(std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                )
            }))
            .network_config(
                TestConfigBuilder::default()
                    .l1_url(l1)
                    .upgrades(upgrades)
                    .build(),
            )
            .build();

        let mut network = TestNetwork::new(config).await;

        let mut events = network.server.event_stream().await;
        loop {
            let event = events.next().await.unwrap();

            match event.event {
                EventType::UpgradeProposal { proposal, .. } => {
                    let upgrade = proposal.data.upgrade_proposal;
                    let new_version = upgrade.new_version;
                    assert_eq!(new_version, Version { major: 0, minor: 2 });
                    break;
                }
                _ => continue,
            }
        }

        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        'outer: loop {
            let height = client
                .get::<usize>("status/block-height")
                .send()
                .await
                .unwrap();

            for peer in &network.peers {
                let state = peer.consensus().read().await.decided_state().await;

                match state.chain_config.resolve() {
                    Some(cf) => {
                        if cf != chain_config_upgrade && height as u64 > stop_voting_view {
                            panic!("failed to upgrade chain config");
                        }
                    }
                    None => continue 'outer,
                }
            }

            break;
        }

        network.server.shut_down().await;
        drop(network);
    }

    #[async_std::test]
    pub(crate) async fn test_restart() {
        setup_logging();
        setup_backtrace();

        const NUM_NODES: usize = 5;
        // Initialize nodes.
        let storage = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
        let persistence: [_; NUM_NODES] = storage
            .iter()
            .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let port = pick_unused_port().unwrap();
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let config = TestNetworkConfigBuilder::default()
            .api_config(
                SqlDataSource::options(&storage[0], Options::with_port(port))
                    .state(Default::default())
                    .status(Default::default()),
            )
            .persistences(persistence)
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();
        let mut network = TestNetwork::new(config).await;

        // Connect client.
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Wait until some blocks have been decided.
        client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(3)
            .collect::<Vec<_>>()
            .await;

        // Shut down the consensus nodes.
        tracing::info!("shutting down nodes");
        network.stop_consensus().await;

        // Get the block height we reached.
        let height = client
            .get::<usize>("status/block-height")
            .send()
            .await
            .unwrap();
        tracing::info!("decided {height} blocks before shutting down");

        // Get the decided chain, so we can check consistency after the restart.
        let chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        let decided_view = chain.last().unwrap().leaf().view_number();

        // Get the most recent state, for catchup.

        let state = network.server.decided_state().await;
        tracing::info!(?decided_view, ?state, "consensus state");

        // Fully shut down the API servers.
        drop(network);

        // Start up again, resuming from the last decided leaf.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let persistence: [_; NUM_NODES] = storage
            .iter()
            .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let config = TestNetworkConfigBuilder::default()
            .api_config(
                SqlDataSource::options(&storage[0], Options::with_port(port))
                    .catchup(Default::default()),
            )
            .persistences(persistence)
            .catchups(std::array::from_fn(|_| {
                // Catchup using node 0 as a peer. Node 0 was running the archival state service
                // before the restart, so it should be able to resume without catching up by loading
                // state from storage.
                StatePeers::<SequencerVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();
        let _network = TestNetwork::new(config).await;
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Make sure we can decide new blocks after the restart.
        tracing::info!("waiting for decide, height {height}");
        let new_leaf: LeafQueryData<SeqTypes> = client
            .socket(&format!("availability/stream/leaves/{height}"))
            .subscribe()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_leaf.height(), height as u64);
        assert_eq!(
            new_leaf.leaf().parent_commitment(),
            chain[height - 1].hash()
        );

        // Ensure the new chain is consistent with the old chain.
        let new_chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(chain, new_chain);
    }

    #[async_std::test]
    async fn test_fetch_config() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url: surf_disco::Url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url.clone());

        let options = Options::with_port(port).config(Default::default());
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config).await;
        client.connect(None).await;

        // Fetch a network config from the API server. The first peer URL is bogus, to test the
        // failure/retry case.
        let peers = StatePeers::<SequencerVersion>::from_urls(
            vec!["https://notarealnode.network".parse().unwrap(), url],
            Default::default(),
        );

        // Fetch the config from node 1, a different node than the one running the service.
        let validator = ValidatorConfig::generated_from_seed_indexed([0; 32], 1, 1, false);
        let mut config = peers.fetch_config(validator.clone()).await;

        // Check the node-specific information in the recovered config is correct.
        assert_eq!(config.node_index, 1);
        assert_eq!(
            config.config.my_own_validator_config.public_key,
            validator.public_key
        );
        assert_eq!(
            config.config.my_own_validator_config.private_key,
            validator.private_key
        );

        // Check the public information is also correct (with respect to the node that actually
        // served the config, for public keys).
        config.config.my_own_validator_config =
            ValidatorConfig::generated_from_seed_indexed([0; 32], 0, 1, true);
        pretty_assertions::assert_eq!(
            serde_json::to_value(PublicHotShotConfig::from(config.config)).unwrap(),
            serde_json::to_value(PublicHotShotConfig::from(
                network.cfg.hotshot_config().clone()
            ))
            .unwrap()
        );
    }
}
