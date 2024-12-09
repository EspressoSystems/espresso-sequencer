use std::{num::NonZeroUsize, path::PathBuf, time::Duration};

use clap::Parser;
use espresso_types::{
    eth_signature_key::EthKeyPair, parse_duration, FeeAmount, FeeVersion, MarketplaceVersion,
    NamespaceId, SequencerVersions, V0_0,
};
use futures::future::pending;
use hotshot::helpers::initialize_logging;
use hotshot_types::traits::node_implementation::Versions;
use marketplace_builder::{
    builder::{build_instance_state, BuilderConfig},
    hooks::BidConfig,
};
use sequencer::{Genesis, L1Params};
use url::Url;
use vbs::version::StaticVersionType;

#[derive(Parser, Clone, Debug)]
struct NonPermissionedBuilderOptions {
    /// Whether this is a reserve builder.
    ///
    /// If not, it's a fallback builder that only builds for unregistered rollups.
    #[clap(short, long, env = "ESPRESSO_MARKETPLACE_BUILDER_IS_RESERVE")]
    is_reserve: bool,

    /// URL of hotshot events API running on Espresso Sequencer DA committee node
    /// The builder will subscribe to this server to receive hotshot events
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_STREAMING_API_URL",
        default_value = "http://localhost:22001"
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

    /// Port to run the builder server on.
    #[clap(short, long, env = "ESPRESSO_BUILDER_SERVER_PORT")]
    port: u16,

    /// BUILDER TRANSACTIONS CHANNEL CAPACITY
    #[clap(long, env = "ESPRESSO_BUILDER_TX_CHANNEL_CAPACITY")]
    pub tx_channel_capacity: NonZeroUsize,

    /// BUILDER HS EVENTS CHANNEL CAPACITY
    #[clap(long, env = "ESPRESSO_BUILDER_EVENT_CHANNEL_CAPACITY")]
    pub event_channel_capacity: NonZeroUsize,

    /// The amount of time a builder can wait before timing out a request to the API.
    #[clap(
        short,
        long,
        env = "ESPRESSO_BUILDER_WEBSERVER_RESPONSE_TIMEOUT_DURATION",
        default_value = "1s",
        value_parser = parse_duration
    )]
    max_api_timeout_duration: Duration,

    /// Path to TOML file containing genesis state.
    #[clap(long, name = "GENESIS_FILE", env = "ESPRESSO_BUILDER_GENESIS_FILE")]
    genesis_file: PathBuf,

    /// Namespace to build for
    #[clap(
        short,
        long,
        env = "ESPRESSO_MARKETPLACE_BUILDER_NAMESPACE",
        default_value = "1",
        value_delimiter = ','
    )]
    pub namespaces: Vec<u32>,

    /// Url we will use to communicate to solver
    #[clap(long, env = "ESPRESSO_MARKETPLACE_SOLVER_API_URL")]
    solver_url: Url,

    /// Bid amount in WEI.
    /// Builder will submit the same bid for every view
    #[clap(
        long,
        env = "ESPRESSO_MARKETPLACE_BUILDER_BID_AMOUNT",
        default_value = "1"
    )]
    bid_amount: FeeAmount,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logging();

    let opt = NonPermissionedBuilderOptions::parse();

    let genesis = Genesis::from_file(&opt.genesis_file)?;
    tracing::info!(?genesis, "genesis");

    let base = genesis.base_version;
    let upgrade = genesis.upgrade_version;

    match (base, upgrade) {
        (FeeVersion::VERSION, MarketplaceVersion::VERSION) => {
            run::<SequencerVersions<FeeVersion, MarketplaceVersion>>(genesis, opt).await
        }
        (FeeVersion::VERSION, _) => run::<SequencerVersions<FeeVersion, V0_0>>(genesis, opt).await,
        (MarketplaceVersion::VERSION, _) => {
            run::<SequencerVersions<MarketplaceVersion, V0_0>>(genesis, opt).await
        }
        _ => panic!(
            "Invalid base ({base}) and upgrade ({upgrade}) versions specified in the toml file."
        ),
    }
}

async fn run<V: Versions>(
    genesis: Genesis,
    opt: NonPermissionedBuilderOptions,
) -> anyhow::Result<()> {
    let l1_params = L1Params {
        url: opt.l1_provider_url,
        options: Default::default(),
    };

    let is_reserve = opt.is_reserve;
    let bid_config = if opt.is_reserve {
        Some(BidConfig {
            amount: opt.bid_amount,
            namespaces: opt.namespaces.into_iter().map(NamespaceId::from).collect(),
        })
    } else {
        None
    };

    let builder_key_pair = EthKeyPair::from_mnemonic(&opt.eth_mnemonic, opt.eth_account_index)?;

    let builder_server_url: Url = format!("http://0.0.0.0:{}", opt.port).parse().unwrap();

    let instance_state =
        build_instance_state::<V>(genesis.chain_config, l1_params, opt.state_peers)
            .await
            .unwrap();

    let base_fee = genesis.max_base_fee();
    tracing::info!(?base_fee, "base_fee");

    let api_response_timeout_duration = opt.max_api_timeout_duration;

    // make the txn timeout as 1/4 of the api_response_timeout_duration
    let txn_timeout_duration = api_response_timeout_duration / 4;

    let _builder_config = BuilderConfig::init(
        is_reserve,
        builder_key_pair,
        opt.tx_channel_capacity,
        opt.event_channel_capacity,
        instance_state.clone(),
        opt.hotshot_event_streaming_url,
        builder_server_url,
        api_response_timeout_duration,
        txn_timeout_duration,
        base_fee,
        bid_config,
        opt.solver_url,
    )
    .await?;

    // Sleep forever
    pending::<()>().await;

    Ok(())
}
