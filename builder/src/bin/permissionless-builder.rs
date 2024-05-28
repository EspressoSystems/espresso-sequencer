use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use builder::non_permissioned::{build_instance_state, BuilderConfig};
use clap::Parser;
use cld::ClDuration;
use es_version::SEQUENCER_VERSION;
use ethers::types::U256;
use hotshot::traits::ValidatedState;
use hotshot_types::data::ViewNumber;
use hotshot_types::traits::node_implementation::ConsensusTime;
use sequencer::{eth_signature_key::EthKeyPair, options::parse_size, ChainConfig, L1Params};
use snafu::Snafu;
use std::num::NonZeroUsize;
use std::{str::FromStr, time::Duration};
use url::Url;

#[derive(Parser, Clone, Debug)]
struct NonPermissionedBuilderOptions {
    /// URL of hotshot events API running on Espresso Sequencer DA committee node
    /// The builder will subscribe to this server to receive hotshot events
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_STREAMING_API_URL",
        default_value = "http://localhost:8081"
    )]
    hotshot_event_streaming_url: Url,

    /// Mnemonic phrase for builder account.
    ///
    /// This is the address fees will be charged to.
    /// It must be funded with ETH in the Espresso fee ledger
    #[clap(long, env = "ESPRESSO_BUILDER_ETH_MNEMONIC")]
    eth_mnemonic: String,

    /// Index of a funded account derived from eth-mnemonic.
    #[clap(long, env = "ESPRESSO_BUILDER_ETH_ACCOUNT_INDEX", default_value = "8")]
    eth_account_index: u32,

    /// Url we will use for RPC communication with L1.
    #[clap(long, env = "ESPRESSO_BUILDER_L1_PROVIDER")]
    l1_provider_url: Url,

    /// Peer nodes use to fetch missing state
    #[clap(long, env = "ESPRESSO_SEQUENCER_STATE_PEERS", value_delimiter = ',')]
    state_peers: Vec<Url>,

    /// Unique identifier for this instance of the sequencer network.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    chain_id: u64,

    /// Maximum size in bytes of a block
    #[clap(long, env = "ESPRESSO_SEQUENCER_MAX_BLOCK_SIZE", value_parser = parse_size)]
    max_block_size: u64,

    /// Minimum fee in WEI per byte of payload
    #[clap(long, env = "ESPRESSO_SEQUENCER_BASE_FEE")]
    base_fee: U256,

    /// Port to run the builder server on.
    #[clap(short, long, env = "ESPRESSO_BUILDER_SERVER_PORT")]
    port: u16,

    /// Bootstrapping View number
    #[clap(short, long, env = "ESPRESSO_BUILDER_BOOTSTRAPPED_VIEW")]
    view_number: u64,

    /// BUILDER CHANNEL CAPACITY
    #[clap(short, long, env = "ESPRESSO_BUILDER_CHANNEL_CAPACITY")]
    channel_capacity: NonZeroUsize,

    /// NETWORK INITIAL NODE COUNT
    #[clap(short, long, env = "ESPRESSO_BUILDER_INIT_NODE_COUNT")]
    node_count: NonZeroUsize,

    /// The amount of time a builder can wait before timing out a request to the API.
    #[clap(
        short,
        long,
        env = "ESPRESSO_BUILDER_WEBSERVER_RESPONSE_TIMEOUT_DURATION",
        default_value = "1s",
        value_parser = parse_duration
    )]
    max_api_timeout_duration: Duration,

    /// The number of views to buffer before a builder garbage collects its state
    #[clap(
        long,
        env = "ESPRESSO_BUILDER_BUFFER_VIEW_NUM_COUNT",
        default_value = "15"
    )]
    buffer_view_num_count: usize,
}

#[derive(Clone, Debug, Snafu)]
struct ParseDurationError {
    reason: String,
}

fn parse_duration(s: &str) -> Result<Duration, ParseDurationError> {
    ClDuration::from_str(s)
        .map(Duration::from)
        .map_err(|err| ParseDurationError {
            reason: err.to_string(),
        })
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let opt = NonPermissionedBuilderOptions::parse();

    let sequencer_version = SEQUENCER_VERSION;

    let l1_params = L1Params {
        url: opt.l1_provider_url,
        finalized_block: None,
        events_max_block_range: 10000,
    };

    let builder_key_pair = EthKeyPair::from_mnemonic(&opt.eth_mnemonic, opt.eth_account_index)?;
    let bootstrapped_view = ViewNumber::new(opt.view_number);

    let builder_server_url: Url = format!("http://0.0.0.0:{}", opt.port).parse().unwrap();

    let chain_config = ChainConfig {
        chain_id: opt.chain_id.into(),
        max_block_size: opt.max_block_size,
        base_fee: opt.base_fee.into(),
        ..Default::default()
    };
    let instance_state =
        build_instance_state(l1_params, opt.state_peers, chain_config, sequencer_version).unwrap();

    // ??

    let validated_state = ValidatedState::genesis(&instance_state).0;

    let api_response_timeout_duration = opt.max_api_timeout_duration;

    // make the txn timeout as 1/4 of the api_response_timeout_duration
    let txn_timeout_duration = api_response_timeout_duration / 4;

    let buffer_view_num_count = opt.buffer_view_num_count;

    let _builder_config = BuilderConfig::init(
        builder_key_pair,
        bootstrapped_view,
        opt.channel_capacity,
        opt.node_count,
        instance_state,
        validated_state,
        opt.hotshot_event_streaming_url,
        builder_server_url,
        api_response_timeout_duration,
        buffer_view_num_count,
        txn_timeout_duration,
    )
    .await;

    // Sleep forever
    async_std::future::pending::<()>().await;

    Ok(())
}
