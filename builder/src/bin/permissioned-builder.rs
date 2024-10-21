use std::{collections::HashMap, num::NonZeroUsize, path::PathBuf, time::Duration};

use anyhow::{bail, Context};
use builder::permissioned::init_node;
use clap::Parser;
use espresso_types::{
    eth_signature_key::EthKeyPair, parse_duration, FeeVersion, MarketplaceVersion,
    SequencerVersions, V0_0, V0_1,
};
use ethers::types::Address;
use hotshot_types::{
    data::ViewNumber,
    light_client::StateSignKey,
    signature_key::BLSPrivKey,
    traits::{
        metrics::NoMetrics,
        node_implementation::{ConsensusTime, Versions},
    },
};
use libp2p::Multiaddr;
use sequencer::{persistence::no_storage::NoStorage, Genesis, L1Params, NetworkParams};
use sequencer_utils::logging;
use url::Url;
use vbs::version::StaticVersionType;

#[derive(Parser, Clone, Debug)]
pub struct PermissionedBuilderOptions {
    /// URL of the HotShot orchestrator.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_ORCHESTRATOR_URL",
        default_value = "http://localhost:8080"
    )]
    pub orchestrator_url: Url,

    /// The socket address of the HotShot CDN's main entry point (the marshal)
    /// in `IP:port` form
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_CDN_ENDPOINT",
        default_value = "127.0.0.1:8081"
    )]
    pub cdn_endpoint: String,

    /// The address to bind to for Libp2p (in `host:port` form)
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_BIND_ADDRESS",
        default_value = "0.0.0.0:1769"
    )]
    pub libp2p_bind_address: String,

    /// The address we advertise to other nodes as being a Libp2p endpoint.
    /// Should be supplied in `host:port` form.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_ADVERTISE_ADDRESS",
        default_value = "localhost:1769"
    )]
    pub libp2p_advertise_address: String,

    /// A comma-separated list of Libp2p multiaddresses to use as bootstrap
    /// nodes.
    ///
    /// Overrides those loaded from the `HotShot` config.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_BOOTSTRAP_NODES",
        value_delimiter = ',',
        num_args = 1..
    )]
    pub libp2p_bootstrap_nodes: Option<Vec<Multiaddr>>,

    /// URL of the Light Client State Relay Server
    #[clap(
        long,
        env = "ESPRESSO_STATE_RELAY_SERVER_URL",
        default_value = "http://localhost:8083"
    )]
    pub state_relay_server_url: Url,

    /// The amount of time to wait between each request to the HotShot
    /// consensus or DA web servers during polling.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_WEBSERVER_POLL_INTERVAL",
        default_value = "100ms",
        value_parser = parse_duration
    )]
    pub webserver_poll_interval: Duration,

    /// Path to TOML file containing genesis state.
    #[clap(long, name = "GENESIS_FILE", env = "ESPRESSO_BUILDER_GENESIS_FILE")]
    pub genesis_file: PathBuf,

    /// Path to file containing private keys.
    ///
    /// The file should follow the .env format, with two keys:
    /// * ESPRESSO_BUILDER_PRIVATE_STAKING_KEY
    /// * ESPRESSO_BUILDER_PRIVATE_STATE_KEY
    ///
    /// Appropriate key files can be generated with the `keygen` utility program.
    #[clap(long, name = "KEY_FILE", env = "ESPRESSO_BUILDER_KEY_FILE")]
    pub key_file: Option<PathBuf>,

    /// Private staking key.
    ///
    /// This can be used as an alternative to KEY_FILE.
    #[clap(
        long,
        env = "ESPRESSO_BUILDER_PRIVATE_STAKING_KEY",
        conflicts_with = "KEY_FILE"
    )]
    pub private_staking_key: Option<BLSPrivKey>,

    /// Private state signing key.
    ///
    /// This can be used as an alternative to KEY_FILE.
    #[clap(
        long,
        env = "ESPRESSO_BUILDER_PRIVATE_STATE_KEY",
        conflicts_with = "KEY_FILE"
    )]
    pub private_state_key: Option<StateSignKey>,

    /// Mnemonic phrase for builder account.
    ///
    /// This is the address fees will be charged to.
    /// It must be funded with ETH in the Espresso fee ledger
    #[clap(long, env = "ESPRESSO_BUILDER_ETH_MNEMONIC")]
    pub eth_mnemonic: String,

    /// Index of a funded account derived from eth-mnemonic.
    #[clap(long, env = "ESPRESSO_BUILDER_ETH_ACCOUNT_INDEX", default_value = "8")]
    pub eth_account_index: u32,

    /// Url we will use for RPC communication with L1.
    #[clap(long, env = "ESPRESSO_BUILDER_L1_PROVIDER")]
    pub l1_provider_url: Url,

    /// Peer nodes use to fetch missing state
    #[clap(long, env = "ESPRESSO_SEQUENCER_STATE_PEERS", value_delimiter = ',')]
    pub state_peers: Vec<Url>,

    /// Port to run the builder server on.
    #[clap(short, long, env = "ESPRESSO_BUILDER_SERVER_PORT")]
    pub port: u16,

    /// Port to run the builder server on.
    #[clap(short, long, env = "ESPRESSO_BUILDER_ADDRESS")]
    pub address: Address,

    /// Bootstrapping View number
    #[clap(short, long, env = "ESPRESSO_BUILDER_BOOTSTRAPPED_VIEW")]
    pub view_number: u64,

    /// BUILDER TRANSACTIONS CHANNEL CAPACITY
    #[clap(long, env = "ESPRESSO_BUILDER_TX_CHANNEL_CAPACITY")]
    pub tx_channel_capacity: NonZeroUsize,

    /// BUILDER HS EVENTS CHANNEL CAPACITY
    #[clap(long, env = "ESPRESSO_BUILDER_EVENT_CHANNEL_CAPACITY")]
    pub event_channel_capacity: NonZeroUsize,

    /// Url a sequencer can use to stream hotshot events
    #[clap(long, env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENTS_PROVIDER")]
    pub hotshot_events_streaming_server_url: Url,

    /// Time between each Libp2p heartbeat
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_HEARTBEAT_INTERVAL", default_value = "1s", value_parser = parse_duration)]
    pub libp2p_heartbeat_interval: Duration,

    /// Number of past heartbeats to gossip about on Libp2p
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_HISTORY_GOSSIP",
        default_value = "3"
    )]
    pub libp2p_history_gossip: usize,

    /// Number of heartbeats to keep in the Libp2p `memcache`
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_HISTORY_LENGTH",
        default_value = "5"
    )]
    pub libp2p_history_length: usize,

    /// Target number of peers for the Libp2p mesh network
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_MESH_N", default_value = "8")]
    pub libp2p_mesh_n: usize,

    /// Maximum number of peers in mesh network before removing some
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MESH_N_HIGH",
        default_value = "12"
    )]
    pub libp2p_mesh_n_high: usize,

    /// Minimum number of peers in the Libp2p mesh network before adding more
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MESH_N_LOW",
        default_value = "6"
    )]
    pub libp2p_mesh_n_low: usize,

    /// Minimum number of outbound Libp2p peers in the mesh network before adding more
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MESH_OUTBOUND_MIN",
        default_value = "2"
    )]
    pub libp2p_mesh_outbound_min: usize,

    /// The maximum number of messages to include in a Libp2p IHAVE message
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MAX_IHAVE_LENGTH",
        default_value = "5000"
    )]
    pub libp2p_max_ihave_length: usize,

    /// The maximum number of IHAVE messages to accept from a Libp2p peer within a heartbeat
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MAX_IHAVE_MESSAGES",
        default_value = "10"
    )]
    pub libp2p_max_ihave_messages: usize,

    /// Libp2p published message ids time cache duration
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_PUBLISHED_MESSAGE_IDS_CACHE_TIME", default_value = "10s", value_parser = parse_duration)]
    pub libp2p_published_message_ids_cache_time: Duration,

    /// Time to wait for a Libp2p message requested through IWANT following an IHAVE advertisement
    #[clap(
          long,
          env = "ESPRESSO_SEQUENCER_LIBP2P_MAX_IWANT_FOLLOWUP_TIME",
          default_value = "3s", value_parser = parse_duration
      )]
    pub libp2p_iwant_followup_time: Duration,

    /// The maximum number of Libp2p messages we will process in a given RPC
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_MAX_MESSAGES_PER_RPC")]
    pub libp2p_max_messages_per_rpc: Option<usize>,

    /// How many times we will allow a peer to request the same message id through IWANT gossip before we start ignoring them
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_GOSSIP_RETRANSMISSION",
        default_value = "3"
    )]
    pub libp2p_gossip_retransmission: u32,

    /// If enabled newly created messages will always be sent to all peers that are subscribed to the topic and have a good enough score
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_FLOOD_PUBLISH",
        default_value = "true"
    )]
    pub libp2p_flood_publish: bool,

    /// The time period that Libp2p message hashes are stored in the cache
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_DUPLICATE_CACHE_TIME", default_value = "20m", value_parser = parse_duration)]
    pub libp2p_duplicate_cache_time: Duration,

    /// Time to live for Libp2p fanout peers
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_FANOUT_TTL", default_value = "60s", value_parser = parse_duration)]
    pub libp2p_fanout_ttl: Duration,

    /// Initial delay in each  ibp2p heartbeat
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIBP2P_HEARTBEAT_INITIAL_DELAY", default_value = "5s", value_parser = parse_duration)]
    pub libp2p_heartbeat_initial_delay: Duration,

    /// How many Libp2p peers we will emit gossip to at each heartbeat
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_GOSSIP_FACTOR",
        default_value = "0.25"
    )]
    pub libp2p_gossip_factor: f64,

    /// Minimum number of Libp2p peers to emit gossip to during a heartbeat
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_GOSSIP_LAZY",
        default_value = "6"
    )]
    pub libp2p_gossip_lazy: usize,

    /// The maximum number of bytes we will send in a single Libp2p gossip message
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_MAX_TRANSMIT_SIZE",
        default_value = "2000000"
    )]
    pub libp2p_max_transmit_size: usize,

    /// The amount of time a builder can wait before timing out a request to the API.
    #[clap(
        short,
        long,
        env = "ESPRESSO_BUILDER_WEBSERVER_RESPONSE_TIMEOUT_DURATION",
        default_value = "1s",
        value_parser = parse_duration
    )]
    pub max_api_timeout_duration: Duration,

    /// The number of views to buffer before a builder garbage collects its state
    #[clap(
        long,
        env = "ESPRESSO_BUILDER_BUFFER_VIEW_NUM_COUNT",
        default_value = "15"
    )]
    pub buffer_view_num_count: usize,

    /// Whether or not we are a DA node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_IS_DA", action)]
    pub is_da: bool,

    #[clap(flatten)]
    logging: logging::Config,
}

impl PermissionedBuilderOptions {
    pub fn private_keys(&self) -> anyhow::Result<(BLSPrivKey, StateSignKey)> {
        if let Some(path) = &self.key_file {
            let vars = dotenvy::from_path_iter(path)?.collect::<Result<HashMap<_, _>, _>>()?;
            let staking = vars
                .get("ESPRESSO_BUILDER_PRIVATE_STAKING_KEY")
                .context("key file missing ESPRESSO_BUILDER_PRIVATE_STAKING_KEY")?
                .parse()?;
            let state = vars
                .get("ESPRESSO_BUILDER_PRIVATE_STATE_KEY")
                .context("key file missing ESPRESSO_BUILDER_PRIVATE_STATE_KEY")?
                .parse()?;
            Ok((staking, state))
        } else if let (Some(staking), Some(state)) = (
            self.private_staking_key.clone(),
            self.private_state_key.clone(),
        ) {
            Ok((staking, state))
        } else {
            bail!("neither key file nor full set of private keys was provided")
        }
    }
}

async fn run<V: Versions>(
    genesis: Genesis,
    opt: PermissionedBuilderOptions,
    versions: V,
) -> anyhow::Result<()> {
    let (private_staking_key, private_state_key) = opt.private_keys()?;

    let l1_params = L1Params {
        url: opt.l1_provider_url,
        events_max_block_range: 10000,
    };

    let builder_key_pair = EthKeyPair::from_mnemonic(&opt.eth_mnemonic, opt.eth_account_index)?;

    let network_params = NetworkParams {
        cdn_endpoint: opt.cdn_endpoint,
        libp2p_advertise_address: opt.libp2p_advertise_address,
        libp2p_bind_address: opt.libp2p_bind_address,
        libp2p_bootstrap_nodes: opt.libp2p_bootstrap_nodes,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        private_staking_key: private_staking_key.clone(),
        private_state_key,
        state_peers: opt.state_peers,
        public_api_url: None,
        libp2p_history_gossip: opt.libp2p_history_gossip,
        libp2p_history_length: opt.libp2p_history_length,
        libp2p_max_ihave_length: opt.libp2p_max_ihave_length,
        libp2p_max_ihave_messages: opt.libp2p_max_ihave_messages,
        libp2p_max_transmit_size: opt.libp2p_max_transmit_size,
        libp2p_mesh_n: opt.libp2p_mesh_n,
        libp2p_mesh_n_high: opt.libp2p_mesh_n_high,
        libp2p_heartbeat_interval: opt.libp2p_heartbeat_interval,
        libp2p_mesh_n_low: opt.libp2p_mesh_n_low,
        libp2p_mesh_outbound_min: opt.libp2p_mesh_outbound_min,
        libp2p_published_message_ids_cache_time: opt.libp2p_published_message_ids_cache_time,
        libp2p_iwant_followup_time: opt.libp2p_iwant_followup_time,
        libp2p_max_messages_per_rpc: opt.libp2p_max_messages_per_rpc,
        libp2p_gossip_retransmission: opt.libp2p_gossip_retransmission,
        libp2p_flood_publish: opt.libp2p_flood_publish,
        libp2p_duplicate_cache_time: opt.libp2p_duplicate_cache_time,
        libp2p_fanout_ttl: opt.libp2p_fanout_ttl,
        libp2p_heartbeat_initial_delay: opt.libp2p_heartbeat_initial_delay,
        libp2p_gossip_factor: opt.libp2p_gossip_factor,
        libp2p_gossip_lazy: opt.libp2p_gossip_lazy,
        config_peers: None,
        catchup_backoff: Default::default(),
    };

    let builder_server_url: Url = format!("http://0.0.0.0:{}", opt.port).parse().unwrap();

    let bootstrapped_view = ViewNumber::new(opt.view_number);

    let max_api_response_timeout_duration = opt.max_api_timeout_duration;
    // make the txn timeout as 1/4 of the api_response_timeout_duration
    let txn_timeout_duration = max_api_response_timeout_duration / 4;

    let buffer_view_num_count = opt.buffer_view_num_count;

    // it will internally spawn the builder web server
    let ctx = init_node(
        genesis,
        network_params,
        &NoMetrics,
        l1_params,
        builder_server_url.clone(),
        builder_key_pair,
        bootstrapped_view,
        opt.tx_channel_capacity,
        opt.event_channel_capacity,
        versions,
        NoStorage,
        max_api_response_timeout_duration,
        buffer_view_num_count,
        opt.is_da,
        txn_timeout_duration,
    )
    .await?;

    // Start doing consensus.
    ctx.start_consensus().await;

    Ok(())
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = PermissionedBuilderOptions::parse();
    opt.logging.init();

    let genesis = Genesis::from_file(&opt.genesis_file)?;
    tracing::info!(?genesis, "genesis");

    let base = genesis.base_version;
    let upgrade = genesis.upgrade_version;

    match (base, upgrade) {
        (V0_1::VERSION, FeeVersion::VERSION) => {
            run(genesis, opt, SequencerVersions::<V0_1, FeeVersion>::new()).await
        }
        (FeeVersion::VERSION, MarketplaceVersion::VERSION) => {
            run(
                genesis,
                opt,
                SequencerVersions::<FeeVersion, MarketplaceVersion>::new(),
            )
            .await
        }
        (V0_1::VERSION, _) => run(genesis, opt, SequencerVersions::<V0_1, V0_0>::new()).await,
        (FeeVersion::VERSION, _) => {
            run(genesis, opt, SequencerVersions::<FeeVersion, V0_0>::new()).await
        }
        (MarketplaceVersion::VERSION, _) => {
            run(
                genesis,
                opt,
                SequencerVersions::<MarketplaceVersion, V0_0>::new(),
            )
            .await
        }
        _ => panic!(
            "Invalid base ({base}) and upgrade ({upgrade}) versions specified in the toml file."
        ),
    }
}
