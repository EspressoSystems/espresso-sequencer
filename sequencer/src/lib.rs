pub mod api;
pub mod catchup;
pub mod context;
pub mod genesis;

mod external_event_handler;
pub mod hotshot_commitment;
pub mod options;
pub mod state_signature;

mod message_compat_tests;

use anyhow::Context;
use async_std::sync::RwLock;
use catchup::StatePeers;
use context::SequencerContext;
use espresso_types::{
    BackoffParams, L1Client, NodeState, PubKey, SeqTypes, SolverAuctionResultsProvider,
    ValidatedState,
};
use ethers::types::U256;
#[cfg(feature = "libp2p")]
use futures::FutureExt;
use genesis::L1Finalized;
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use libp2p::Multiaddr;
use network::libp2p::split_off_peer_id;
use options::Identity;
use state_signature::static_stake_table_commitment;
use url::Url;
pub mod persistence;
pub mod state;

#[cfg(feature = "libp2p")]
use std::time::Duration;
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData, net::SocketAddr, sync::Arc};

use derivative::Derivative;
use espresso_types::v0::traits::{PersistenceOptions, SequencerPersistence};
pub use genesis::Genesis;
#[cfg(feature = "libp2p")]
use hotshot::traits::implementations::{CombinedNetworks, GossipConfig, Libp2pNetwork};
use hotshot::{
    traits::implementations::{
        derive_libp2p_peer_id, CdnMetricsValue, CdnTopic, KeyPair, MemoryNetwork, PushCdnNetwork,
        WrappedSignatureKey,
    },
    types::SignatureKey,
    MarketplaceConfig,
};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use hotshot_types::{
    data::ViewNumber,
    light_client::{StateKeyPair, StateSignKey},
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        metrics::Metrics,
        network::{ConnectedNetwork, Topic},
        node_implementation::{NodeImplementation, NodeType, Versions},
        signature_key::{BuilderSignatureKey, StakeTableEntryType},
    },
    utils::BuilderCommitment,
    ValidatorConfig,
};
pub use options::Options;
use serde::{Deserialize, Serialize};
use vbs::version::{StaticVersion, StaticVersionType};
pub mod network;

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    Default(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Node<N: ConnectedNetwork<PubKey>, P: SequencerPersistence>(PhantomData<fn(&N, &P)>);

// Using derivative to derive Clone triggers the clippy lint
// https://rust-lang.github.io/rust-clippy/master/index.html#/incorrect_clone_impl_on_copy_type
impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence> Clone for Node<N, P> {
    fn clone(&self) -> Self {
        *self
    }
}

pub type SequencerApiVersion = StaticVersion<0, 1>;

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence> NodeImplementation<SeqTypes>
    for Node<N, P>
{
    type Network = N;
    type Storage = Arc<P>;
    type AuctionResultsProvider = SolverAuctionResultsProvider;
}

#[derive(Clone, Debug)]
pub struct NetworkParams {
    /// The address where a CDN marshal is located
    pub cdn_endpoint: String,
    pub orchestrator_url: Url,
    pub state_relay_server_url: Url,
    pub private_staking_key: BLSPrivKey,
    pub private_state_key: StateSignKey,
    pub state_peers: Vec<Url>,
    pub config_peers: Option<Vec<Url>>,
    pub catchup_backoff: BackoffParams,
    /// The address to advertise as our public API's URL
    pub public_api_url: Option<Url>,

    /// The address to send to other Libp2p nodes to contact us
    pub libp2p_advertise_address: SocketAddr,
    /// The address to bind to for Libp2p
    pub libp2p_bind_address: SocketAddr,
    /// The (optional) bootstrap node addresses for Libp2p. If supplied, these will
    /// override the bootstrap nodes specified in the config file.
    pub libp2p_bootstrap_nodes: Option<Vec<Multiaddr>>,
}

pub struct L1Params {
    pub url: Url,
    pub events_max_block_range: u64,
}

#[allow(clippy::too_many_arguments)]
pub async fn init_node<P: PersistenceOptions, V: Versions>(
    genesis: Genesis,
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    persistence_opt: P,
    l1_params: L1Params,
    seq_versions: V,
    is_da: bool,
    identity: Identity,
    marketplace_config: MarketplaceConfig<SeqTypes, Node<network::Production, P::Persistence>>,
) -> anyhow::Result<SequencerContext<network::Production, P::Persistence, V>> {
    // Expose git information via status API.
    metrics
        .text_family(
            "version".into(),
            vec!["rev".into(), "desc".into(), "timestamp".into()],
        )
        .create(vec![
            env!("VERGEN_GIT_SHA").into(),
            env!("VERGEN_GIT_DESCRIBE").into(),
            env!("VERGEN_GIT_COMMIT_TIMESTAMP").into(),
        ]);

    // Expose Node Entity Information via the status/metrics API
    metrics
        .text_family(
            "node_identity_general".into(),
            vec![
                "name".into(),
                "company_name".into(),
                "company_website".into(),
                "operating_system".into(),
                "node_type".into(),
                "network_type".into(),
            ],
        )
        .create(vec![
            identity.node_name.unwrap_or("".into()),
            identity.company_name.unwrap_or("".into()),
            identity
                .company_website
                .map(|u| u.into())
                .unwrap_or("".into()),
            identity.operating_system.unwrap_or("".into()),
            identity.node_type.unwrap_or("".into()),
            identity.network_type.unwrap_or("".into()),
        ]);

    // Expose Node Identity Location via the status/metrics API
    metrics
        .text_family(
            "node_identity_location".into(),
            vec!["country".into(), "latitude".into(), "longitude".into()],
        )
        .create(vec![
            identity.country_code.unwrap_or("".into()),
            identity
                .latitude
                .map(|l| l.to_string())
                .unwrap_or("".into()),
            identity
                .longitude
                .map(|l| l.to_string())
                .unwrap_or("".into()),
        ]);

    // Stick our public key in `metrics` so it is easily accessible via the status API.
    let pub_key = BLSPubKey::from_private(&network_params.private_staking_key);
    metrics
        .text_family("node".into(), vec!["key".into()])
        .create(vec![pub_key.to_string()]);

    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        advertise_address: Some(network_params.libp2p_advertise_address),
        builder_address: None,
        network_config_file: None,
    };
    let orchestrator_client = OrchestratorClient::new(validator_args);
    let state_key_pair = StateKeyPair::from_sign_key(network_params.private_state_key);
    let my_config = ValidatorConfig {
        public_key: pub_key,
        private_key: network_params.private_staking_key,
        stake_value: 1,
        state_key_pair,
        is_da,
    };

    // Derive our Libp2p public key from our private key
    let libp2p_public_key =
        derive_libp2p_peer_id::<<SeqTypes as NodeType>::SignatureKey>(&my_config.private_key)
            .with_context(|| "Failed to derive Libp2p peer ID")?;

    let persistence = persistence_opt.clone().create().await?;
    let (mut config, wait_for_orchestrator) = match (
        persistence.load_config().await?,
        network_params.config_peers,
    ) {
        (Some(config), _) => {
            tracing::info!("loaded network config from storage, rejoining existing network");
            (config, false)
        }
        // If we were told to fetch the config from an already-started peer, do so.
        (None, Some(peers)) => {
            tracing::info!(?peers, "loading network config from peers");
            let peers =
                StatePeers::<SequencerApiVersion>::from_urls(peers, network_params.catchup_backoff);
            let config = peers.fetch_config(my_config.clone()).await;

            tracing::info!(
                node_id = config.node_index,
                stake_table = ?config.config.known_nodes_with_stake,
                "loaded config",
            );
            persistence.save_config(&config).await?;
            (config, false)
        }
        // Otherwise, this is a fresh network; load from the orchestrator.
        (None, None) => {
            tracing::info!("loading network config from orchestrator");
            tracing::error!(
                "waiting for other nodes to connect, DO NOT RESTART until fully connected"
            );
            let config = NetworkConfig::get_complete_config(
                &orchestrator_client,
                my_config.clone(),
                // Register in our Libp2p advertise address and public key so other nodes
                // can contact us on startup
                Some(network_params.libp2p_advertise_address),
                Some(libp2p_public_key),
            )
            .await?
            .0;

            tracing::info!(
                node_id = config.node_index,
                stake_table = ?config.config.known_nodes_with_stake,
                "loaded config",
            );
            persistence.save_config(&config).await?;
            tracing::error!("all nodes connected");
            (config, true)
        }
    };

    if let Some(upgrade) = genesis.upgrades.get(&V::Upgrade::VERSION) {
        upgrade.set_hotshot_config_parameters(&mut config.config);
    }

    // If the `Libp2p` bootstrap nodes were supplied via the command line, override those
    // present in the config file.
    if let Some(bootstrap_nodes) = network_params.libp2p_bootstrap_nodes {
        if let Some(libp2p_config) = config.libp2p_config.as_mut() {
            // If the libp2p configuration is present, we can override the bootstrap nodes.

            // Split off the peer ID from the addresses
            libp2p_config.bootstrap_nodes = bootstrap_nodes
                .into_iter()
                .map(split_off_peer_id)
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| "Failed to parse peer ID from bootstrap node")?;
        } else {
            // If not, don't try launching with them. Eventually we may want to
            // provide a default configuration here instead.
            tracing::warn!("No libp2p configuration found, ignoring supplied bootstrap nodes");
        }
    }

    let node_index = config.node_index;

    // If we are a DA node, we need to subscribe to the DA topic
    let topics = {
        let mut topics = vec![CdnTopic::Global];
        if is_da {
            topics.push(CdnTopic::Da);
        }
        topics
    };

    // Initialize the push CDN network (and perform the initial connection)
    let cdn_network = PushCdnNetwork::new(
        network_params.cdn_endpoint,
        topics,
        KeyPair {
            public_key: WrappedSignatureKey(my_config.public_key),
            private_key: my_config.private_key.clone(),
        },
        CdnMetricsValue::new(metrics),
    )
    .with_context(|| "Failed to create CDN network")?;

    // Initialize the Libp2p network (if enabled)
    #[cfg(feature = "libp2p")]
    let network = {
        let p2p_network = Libp2pNetwork::from_config::<SeqTypes>(
            config.clone(),
            GossipConfig::default(),
            network_params.libp2p_bind_address,
            &my_config.public_key,
            // We need the private key so we can derive our Libp2p keypair
            // (using https://docs.rs/blake3/latest/blake3/fn.derive_key.html)
            &my_config.private_key,
            hotshot::traits::implementations::Libp2pMetricsValue::new(metrics),
        )
        .await
        .with_context(|| "Failed to create libp2p network")?;

        tracing::warn!("Waiting for at least one connection to be initialized");
        futures::select! {
            _ = cdn_network.wait_for_ready().fuse() => {
                tracing::warn!("CDN connection initialized");
            },
            _ = p2p_network.wait_for_ready().fuse() => {
                tracing::warn!("P2P connection initialized");
            },
        };

        // Combine the CDN and P2P networks
        Arc::from(CombinedNetworks::new(
            cdn_network,
            p2p_network,
            Some(Duration::from_secs(1)),
        ))
    };

    // Wait for the CDN network to be ready if we're not using the P2P network
    #[cfg(not(feature = "libp2p"))]
    let network = {
        tracing::warn!("Waiting for the CDN connection to be initialized");
        cdn_network.wait_for_ready().await;
        tracing::warn!("CDN connection initialized");
        Arc::from(cdn_network)
    };

    let mut genesis_state = ValidatedState {
        chain_config: genesis.chain_config.into(),
        ..Default::default()
    };
    for (address, amount) in genesis.accounts {
        tracing::info!(%address, %amount, "Prefunding account for demo");
        genesis_state.prefund_account(address, amount);
    }

    let l1_client = L1Client::new(l1_params.url, l1_params.events_max_block_range);
    let l1_genesis = match genesis.l1_finalized {
        L1Finalized::Block(b) => b,
        L1Finalized::Number { number } => l1_client.wait_for_finalized_block(number).await,
    };
    let instance_state = NodeState {
        chain_config: genesis.chain_config,
        l1_client,
        genesis_header: genesis.header,
        genesis_state,
        l1_genesis: Some(l1_genesis),
        peers: catchup::local_and_remote(
            persistence_opt,
            StatePeers::<SequencerApiVersion>::from_urls(
                network_params.state_peers,
                network_params.catchup_backoff,
            ),
        )
        .await,
        node_id: node_index,
        upgrades: genesis.upgrades,
        current_version: V::Base::VERSION,
    };

    let mut ctx = SequencerContext::init(
        config,
        instance_state,
        persistence,
        network,
        Some(network_params.state_relay_server_url),
        metrics,
        genesis.stake_table.capacity,
        network_params.public_api_url,
        seq_versions,
        marketplace_config,
    )
    .await?;
    if wait_for_orchestrator {
        ctx = ctx.wait_for_orchestrator(orchestrator_client);
    }
    Ok(ctx)
}

pub fn empty_builder_commitment() -> BuilderCommitment {
    BuilderCommitment::from_bytes([])
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use std::{collections::HashMap, time::Duration};

    use async_compatibility_layer::art::async_spawn;
    use committable::Committable;
    use espresso_types::{
        eth_signature_key::EthKeyPair,
        mock::MockStateCatchup,
        v0::traits::{PersistenceOptions, StateCatchup},
        Event, FeeAccount, Leaf, MarketplaceVersion, Payload, PubKey, SeqTypes, Transaction,
        Upgrade,
    };
    use futures::{
        future::join_all,
        stream::{Stream, StreamExt},
    };
    use hotshot::{
        traits::{
            implementations::{MasterMap, MemoryNetwork},
            BlockPayload,
        },
        types::EventType::Decide,
    };
    use hotshot_stake_table::vec_based::StakeTable;
    use hotshot_testing::block_builder::{
        BuilderTask, SimpleBuilderImplementation, TestBuilderImplementation,
    };
    use hotshot_types::{
        event::LeafInfo,
        light_client::{CircuitField, StateKeyPair, StateVerKey},
        traits::{
            block_contents::{vid_commitment, BlockHeader, EncodeBytes},
            metrics::NoMetrics,
            node_implementation::ConsensusTime,
            stake_table::StakeTableScheme,
        },
        ExecutionType, HotShotConfig, PeerConfig,
    };
    use marketplace_builder_core::{
        builder_state::{BuilderState, BuiltFromProposedBlock},
        service::{run_builder_service, BroadcastSenders, GlobalState, NoHooks, ProxyGlobalState},
    };
    use portpicker::pick_unused_port;
    use vbs::version::Version;

    use super::*;
    use crate::persistence::no_storage::{self, NoStorage};

    const STAKE_TABLE_CAPACITY_FOR_TEST: u64 = 10;
    const BUILDER_CHANNEL_CAPACITY_FOR_TEST: usize = 128;

    struct MarketplaceBuilderImplementation {
        hooks: Arc<NoHooks<SeqTypes>>,
        senders: BroadcastSenders<SeqTypes>,
    }

    impl BuilderTask<SeqTypes> for MarketplaceBuilderImplementation {
        fn start(
            self: Box<Self>,
            stream: Box<
                dyn Stream<Item = hotshot::types::Event<SeqTypes>>
                    + std::marker::Unpin
                    + Send
                    + 'static,
            >,
        ) {
            async_spawn(async move {
                let res = run_builder_service::<SeqTypes>(self.hooks, self.senders, stream).await;
                tracing::error!(?res, "Testing marketplace builder service exited");
            });
        }
    }

    pub async fn run_marketplace_builder<const NUM_NODES: usize>(
        port: Option<u16>,
        instance_state: NodeState,
        validated_state: ValidatedState,
    ) -> (Box<dyn BuilderTask<SeqTypes>>, Url) {
        let builder_key_pair = TestConfig::<0>::builder_key();
        let port = port.unwrap_or_else(|| pick_unused_port().expect("No ports available"));

        // This should never fail.
        let url: Url = format!("http://localhost:{port}")
            .parse()
            .expect("Failed to parse builder URL");

        let (senders, receivers) = marketplace_builder_core::service::broadcast_channels(
            BUILDER_CHANNEL_CAPACITY_FOR_TEST,
        );

        // builder api request channel
        let (req_sender, req_receiver) =
            async_broadcast::broadcast::<_>(BUILDER_CHANNEL_CAPACITY_FOR_TEST);

        let (genesis_payload, genesis_ns_table) =
            Payload::from_transactions([], &validated_state, &instance_state)
                .await
                .expect("genesis payload construction failed");

        let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);

        let vid_commitment = {
            let payload_bytes = genesis_payload.encode();
            vid_commitment(&payload_bytes, NUM_NODES)
        };

        // create the global state
        let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
            req_sender,
            senders.transactions.clone(),
            vid_commitment,
            ViewNumber::genesis(),
        );

        let global_state = Arc::new(RwLock::new(global_state));

        let leaf = Leaf::genesis(&validated_state, &instance_state).await;

        let builder_state = BuilderState::<SeqTypes>::new(
            BuiltFromProposedBlock {
                view_number: ViewNumber::genesis(),
                vid_commitment,
                leaf_commit: leaf.commit(),
                builder_commitment,
            },
            &receivers,
            req_receiver,
            Vec::new(), /* tx_queue */
            Arc::clone(&global_state),
            Duration::from_secs(60),
            10,
            Arc::new(instance_state),
            Duration::from_secs(60),
            Arc::new(validated_state),
        );

        builder_state.event_loop();

        let hooks = Arc::new(NoHooks(PhantomData));

        // create the proxy global state it will server the builder apis
        let app = ProxyGlobalState::new(
            global_state.clone(),
            Arc::clone(&hooks),
            (builder_key_pair.fee_account(), builder_key_pair.clone()),
            Duration::from_secs(60),
        )
        .into_app()
        .expect("Failed to create builder tide-disco app");

        async_spawn(
            app.serve(
                format!("http://0.0.0.0:{port}")
                    .parse::<Url>()
                    .expect("Failed to parse builder listener"),
                MarketplaceVersion::instance(),
            ),
        );

        (
            Box::new(MarketplaceBuilderImplementation { hooks, senders }),
            url,
        )
    }

    pub async fn run_test_builder<const NUM_NODES: usize>(
        port: Option<u16>,
    ) -> (Box<dyn BuilderTask<SeqTypes>>, Url) {
        let port = port.unwrap_or_else(|| pick_unused_port().expect("No ports available"));

        // This should never fail.
        let url: Url = format!("http://localhost:{port}")
            .parse()
            .expect("Failed to parse builder URL");

        (
            <SimpleBuilderImplementation as TestBuilderImplementation<SeqTypes>>::start(
                NUM_NODES,
                format!("http://0.0.0.0:{port}")
                    .parse()
                    .expect("Failed to parse builder listener"),
                (),
                HashMap::new(),
            )
            .await,
            url,
        )
    }

    pub struct TestConfigBuilder<const NUM_NODES: usize> {
        config: HotShotConfig<PubKey>,
        priv_keys: Vec<BLSPrivKey>,
        state_key_pairs: Vec<StateKeyPair>,
        master_map: Arc<MasterMap<PubKey>>,
        l1_url: Url,
        state_relay_url: Option<Url>,
        builder_port: Option<u16>,
        marketplace_builder_port: Option<u16>,
        upgrades: BTreeMap<Version, Upgrade>,
    }

    impl<const NUM_NODES: usize> TestConfigBuilder<NUM_NODES> {
        pub fn builder_port(mut self, builder_port: Option<u16>) -> Self {
            self.builder_port = builder_port;
            self
        }

        pub fn marketplace_builder_port(mut self, port: Option<u16>) -> Self {
            self.marketplace_builder_port = port;
            self
        }

        pub fn state_relay_url(mut self, url: Url) -> Self {
            self.state_relay_url = Some(url);
            self
        }

        pub fn l1_url(mut self, l1_url: Url) -> Self {
            self.l1_url = l1_url;
            self
        }

        pub fn upgrades<V: Versions>(mut self, upgrades: BTreeMap<Version, Upgrade>) -> Self {
            let upgrade = upgrades.get(&<V as Versions>::Upgrade::VERSION).unwrap();
            upgrade.set_hotshot_config_parameters(&mut self.config);
            self.upgrades = upgrades;
            self
        }

        pub fn build(self) -> TestConfig<NUM_NODES> {
            TestConfig {
                config: self.config,
                priv_keys: self.priv_keys,
                state_key_pairs: self.state_key_pairs,
                master_map: self.master_map,
                l1_url: self.l1_url,
                state_relay_url: self.state_relay_url,
                marketplace_builder_port: self.marketplace_builder_port,
                builder_port: self.builder_port,
                upgrades: self.upgrades,
            }
        }
    }

    impl<const NUM_NODES: usize> Default for TestConfigBuilder<NUM_NODES> {
        fn default() -> Self {
            let num_nodes = NUM_NODES;

            // Generate keys for the nodes.
            let seed = [0; 32];
            let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..num_nodes)
                .map(|i| <PubKey as SignatureKey>::generated_from_seed_indexed(seed, i as u64))
                .unzip();
            let state_key_pairs = (0..num_nodes)
                .map(|i| StateKeyPair::generate_from_seed_indexed(seed, i as u64))
                .collect::<Vec<_>>();
            let known_nodes_with_stake = pub_keys
                .iter()
                .zip(&state_key_pairs)
                .map(|(pub_key, state_key_pair)| PeerConfig::<PubKey> {
                    stake_table_entry: pub_key.stake_table_entry(1),
                    state_ver_key: state_key_pair.ver_key(),
                })
                .collect::<Vec<_>>();

            let master_map = MasterMap::new();

            let config: HotShotConfig<PubKey> = HotShotConfig {
                fixed_leader_for_gpuvid: 0,
                execution_type: ExecutionType::Continuous,
                num_nodes_with_stake: num_nodes.try_into().unwrap(),
                num_nodes_without_stake: 0,
                known_da_nodes: known_nodes_with_stake.clone(),
                known_nodes_with_stake: known_nodes_with_stake.clone(),
                known_nodes_without_stake: vec![],
                next_view_timeout: Duration::from_secs(5).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                da_staked_committee_size: num_nodes,
                da_non_staked_committee_size: 0,
                my_own_validator_config: Default::default(),
                view_sync_timeout: Duration::from_secs(1),
                data_request_delay: Duration::from_secs(1),
                builder_urls: vec1::vec1![Url::parse(&format!(
                    "http://127.0.0.1:{}",
                    pick_unused_port().unwrap()
                ))
                .unwrap()],
                builder_timeout: Duration::from_secs(1),
                start_threshold: (
                    known_nodes_with_stake.clone().len() as u64,
                    known_nodes_with_stake.clone().len() as u64,
                ),
                start_proposing_view: 0,
                stop_proposing_view: 0,
                start_voting_view: 0,
                stop_voting_view: 0,
                start_proposing_time: 0,
                start_voting_time: 0,
                stop_proposing_time: 0,
                stop_voting_time: 0,
            };

            Self {
                config,
                priv_keys,
                state_key_pairs,
                master_map,
                l1_url: "http://localhost:8545".parse().unwrap(),
                state_relay_url: None,
                builder_port: None,
                marketplace_builder_port: None,
                upgrades: Default::default(),
            }
        }
    }

    #[derive(Clone)]
    pub struct TestConfig<const NUM_NODES: usize> {
        config: HotShotConfig<PubKey>,
        priv_keys: Vec<BLSPrivKey>,
        state_key_pairs: Vec<StateKeyPair>,
        master_map: Arc<MasterMap<PubKey>>,
        l1_url: Url,
        state_relay_url: Option<Url>,
        builder_port: Option<u16>,
        marketplace_builder_port: Option<u16>,
        upgrades: BTreeMap<Version, Upgrade>,
    }

    impl<const NUM_NODES: usize> TestConfig<NUM_NODES> {
        pub fn num_nodes(&self) -> usize {
            self.priv_keys.len()
        }

        pub fn hotshot_config(&self) -> &HotShotConfig<PubKey> {
            &self.config
        }

        pub fn set_builder_urls(&mut self, builder_urls: vec1::Vec1<Url>) {
            self.config.builder_urls = builder_urls;
        }

        pub fn marketplace_builder_port(&self) -> Option<u16> {
            self.marketplace_builder_port
        }

        pub fn builder_port(&self) -> Option<u16> {
            self.builder_port
        }

        pub fn l1_url(&self) -> Url {
            self.l1_url.clone()
        }

        pub fn upgrades(&self) -> BTreeMap<Version, Upgrade> {
            self.upgrades.clone()
        }

        pub async fn init_nodes<V: Versions>(
            &self,
            bind_version: V,
        ) -> Vec<SequencerContext<network::Memory, NoStorage, V>> {
            join_all((0..self.num_nodes()).map(|i| async move {
                self.init_node(
                    i,
                    ValidatedState::default(),
                    no_storage::Options,
                    MockStateCatchup::default(),
                    &NoMetrics,
                    STAKE_TABLE_CAPACITY_FOR_TEST,
                    bind_version,
                    Default::default(),
                    Url::parse(&format!(
                        "http://localhost:{}",
                        self.marketplace_builder_port.unwrap_or_default()
                    ))
                    .unwrap(),
                )
                .await
            }))
            .await
        }

        pub fn stake_table(&self) -> StakeTable<BLSPubKey, StateVerKey, CircuitField> {
            let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(
                STAKE_TABLE_CAPACITY_FOR_TEST as usize,
            );
            self.config
                .known_nodes_with_stake
                .iter()
                .for_each(|config| {
                    st.register(
                        *config.stake_table_entry.key(),
                        config.stake_table_entry.stake(),
                        config.state_ver_key.clone(),
                    )
                    .unwrap()
                });
            st.advance();
            st.advance();
            st
        }

        #[allow(clippy::too_many_arguments)]
        pub async fn init_node<V: Versions, P: PersistenceOptions>(
            &self,
            i: usize,
            mut state: ValidatedState,
            persistence_opt: P,
            catchup: impl StateCatchup + 'static,
            metrics: &dyn Metrics,
            stake_table_capacity: u64,
            bind_version: V,
            upgrades: BTreeMap<Version, Upgrade>,
            marketplace_builder_url: Url,
        ) -> SequencerContext<network::Memory, P::Persistence, V> {
            let mut config = self.config.clone();
            let my_peer_config = &config.known_nodes_with_stake[i];
            let is_da = config.known_da_nodes.contains(my_peer_config);
            config.my_own_validator_config = ValidatorConfig {
                public_key: my_peer_config.stake_table_entry.stake_key,
                private_key: self.priv_keys[i].clone(),
                stake_value: my_peer_config.stake_table_entry.stake_amount.as_u64(),
                state_key_pair: self.state_key_pairs[i].clone(),
                is_da,
            };

            let topics = if is_da {
                vec![Topic::Global, Topic::Da]
            } else {
                vec![Topic::Global]
            };

            let network = Arc::new(MemoryNetwork::new(
                &config.my_own_validator_config.public_key,
                &self.master_map,
                &topics,
                None,
            ));

            // Make sure the builder account is funded.
            let builder_account = Self::builder_key().fee_account();
            tracing::info!(%builder_account, "prefunding builder account");
            state.prefund_account(builder_account, U256::max_value().into());
            let node_state = NodeState::new(
                i as u64,
                state.chain_config.resolve().unwrap_or_default(),
                L1Client::new(self.l1_url.clone(), 1000),
                catchup::local_and_remote(persistence_opt.clone(), catchup).await,
                V::Base::VERSION,
            )
            .with_current_version(V::Base::version())
            .with_genesis(state)
            .with_upgrades(upgrades);

            tracing::info!(
                i,
                key = %config.my_own_validator_config.public_key,
                state_key = %config.my_own_validator_config.state_key_pair.ver_key(),
                "starting node",
            );
            SequencerContext::init(
                NetworkConfig {
                    config,
                    // For testing, we use a fake network, so the rest of the network config beyond
                    // the base consensus config does not matter.
                    ..Default::default()
                },
                node_state,
                persistence_opt.create().await.unwrap(),
                network,
                self.state_relay_url.clone(),
                metrics,
                stake_table_capacity,
                None, // The public API URL
                bind_version,
                MarketplaceConfig::<SeqTypes, Node<network::Memory, P::Persistence>> {
                    auction_results_provider: Arc::new(SolverAuctionResultsProvider::default()),
                    fallback_builder_url: marketplace_builder_url,
                },
            )
            .await
            .unwrap()
        }

        pub fn builder_key() -> EthKeyPair {
            FeeAccount::generated_from_seed_indexed([1; 32], 0).1
        }
    }

    // Wait for decide event, make sure it matches submitted transaction. Return the block number
    // containing the transaction.
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> u64 {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");

            if let Decide { leaf_chain, .. } = event.event {
                if let Some(height) = leaf_chain.iter().find_map(|LeafInfo { leaf, .. }| {
                    if leaf
                        .block_payload()
                        .as_ref()?
                        .transaction_commitments(leaf.block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.block_header().block_number())
                    } else {
                        None
                    }
                }) {
                    return height;
                }
            } else {
                // Keep waiting
            }
        }
    }
}

#[cfg(test)]
mod test {

    use espresso_types::{Header, MockSequencerVersions, NamespaceId, Payload, Transaction};
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;
    use hotshot_types::{
        event::LeafInfo,
        traits::block_contents::{
            vid_commitment, BlockHeader, BlockPayload, EncodeBytes, GENESIS_VID_NUM_STORAGE_NODES,
        },
    };
    use sequencer_utils::{test_utils::setup_test, AnvilOptions};
    use testing::{wait_for_decide_on_handle, TestConfigBuilder};

    use self::testing::run_test_builder;
    use super::*;

    #[async_std::test]
    async fn test_skeleton_instantiation() {
        setup_test();
        // Assign `config` so it isn't dropped early.
        let anvil = AnvilOptions::default().spawn().await;
        let url = anvil.url();
        const NUM_NODES: usize = 5;
        let mut config = TestConfigBuilder::<NUM_NODES>::default()
            .l1_url(url)
            .build();

        let (builder_task, builder_url) = run_test_builder::<NUM_NODES>(None).await;

        config.set_builder_urls(vec1::vec1![builder_url]);

        let handles = config.init_nodes(MockSequencerVersions::new()).await;

        let handle_0 = &handles[0];

        // Hook the builder up to the event stream from the first node
        builder_task.start(Box::new(handle_0.event_stream().await));

        let mut events = handle_0.event_stream().await;

        for handle in handles.iter() {
            handle.start_consensus().await;
        }

        // Submit target transaction to handle
        let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3]);
        handles[0]
            .submit_transaction(txn.clone())
            .await
            .expect("Failed to submit transaction");
        tracing::info!("Submitted transaction to handle: {txn:?}");

        wait_for_decide_on_handle(&mut events, &txn).await;
    }

    #[async_std::test]
    async fn test_header_invariants() {
        setup_test();

        let success_height = 30;
        // Assign `config` so it isn't dropped early.
        let anvil = AnvilOptions::default().spawn().await;
        let url = anvil.url();
        const NUM_NODES: usize = 5;
        let mut config = TestConfigBuilder::<NUM_NODES>::default()
            .l1_url(url)
            .build();

        let (builder_task, builder_url) = run_test_builder::<NUM_NODES>(None).await;

        config.set_builder_urls(vec1::vec1![builder_url]);
        let handles = config.init_nodes(MockSequencerVersions::new()).await;

        let handle_0 = &handles[0];

        let mut events = handle_0.event_stream().await;

        // Hook the builder up to the event stream from the first node
        builder_task.start(Box::new(handle_0.event_stream().await));

        for handle in handles.iter() {
            handle.start_consensus().await;
        }

        let mut parent = {
            // TODO refactor repeated code from other tests
            let (genesis_payload, genesis_ns_table) =
                Payload::from_transactions([], &ValidatedState::default(), &NodeState::mock())
                    .await
                    .unwrap();
            let genesis_commitment = {
                // TODO we should not need to collect payload bytes just to compute vid_commitment
                let payload_bytes = genesis_payload.encode();
                vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
            };
            let genesis_state = NodeState::mock();
            Header::genesis(
                &genesis_state,
                genesis_commitment,
                empty_builder_commitment(),
                genesis_ns_table,
            )
        };

        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");
            let Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            tracing::info!("Got decide {leaf_chain:?}");

            // Check that each successive header satisfies invariants relative to its parent: all
            // the fields which should be monotonic are.
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let header = leaf.block_header().clone();
                if header.height() == 0 {
                    parent = header;
                    continue;
                }
                assert_eq!(header.height(), parent.height() + 1);
                assert!(header.timestamp() >= parent.timestamp());
                assert!(header.l1_head() >= parent.l1_head());
                assert!(header.l1_finalized() >= parent.l1_finalized());
                parent = header;
            }

            if parent.height() >= success_height {
                break;
            }
        }
    }
}
