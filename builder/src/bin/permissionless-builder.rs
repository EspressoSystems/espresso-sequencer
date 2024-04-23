use anyhow::{bail, Context};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use builder::non_permissioned::{build_instance_state, BuilderConfig};
use clap::Parser;
use cld::ClDuration;
use es_version::SEQUENCER_VERSION;
use hotshot_types::data::ViewNumber;
use hotshot_types::light_client::StateSignKey;
use hotshot_types::signature_key::BLSPrivKey;
use hotshot_types::traits::node_implementation::ConsensusTime;
use sequencer::eth_signature_key::EthKeyPair;
use sequencer::{BuilderParams, L1Params};
use snafu::Snafu;
use std::num::NonZeroUsize;
use std::{collections::HashMap, path::PathBuf, str::FromStr, time::Duration};
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct NonPermissionedBuilderOptions {
    /// URL of hotshot events API running on Espresso Sequencer DA committee node
    /// The builder will subscribe to this server to receive hotshot events
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_STREAMING_API_URL",
        default_value = "http://localhost:8081"
    )]
    pub hotshot_event_streaming_url: Url,

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
        conflicts_with = "key_file"
    )]
    pub private_staking_key: Option<BLSPrivKey>,

    /// Private state signing key.
    ///
    /// This can be used as an alternative to KEY_FILE.
    #[clap(
        long,
        env = "ESPRESSO_BUILDER_PRIVATE_STATE_KEY",
        conflicts_with = "key_file"
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

    /// Bootstrapping View number
    #[clap(short, long, env = "ESPRESSO_BUILDER_BOOTSTRAPPED_VIEW")]
    pub view_number: u64,

    /// BUILDER CHANNEL CAPACITY
    #[clap(short, long, env = "ESPRESSO_BUILDER_CHANNEL_CAPACITY")]
    pub channel_capacity: NonZeroUsize,

    /// The amount of time a builder can wait before timing out a request to the API.
    #[clap(
        short,
        long,
        env = "WEBSERVER_RESPONSE_TIMEOUT_DURATION",
        default_value = "1s",
        value_parser = parse_duration
    )]
    pub max_api_timeout_duration: Duration,

    /// The number of views to buffer before a builder garbage collects its state
    #[clap(
        short,
        long,
        env = "ESPRESSO_BUILDER_BUFFER_VIEW_NUM_COUNT",
        default_value = "15"
    )]
    pub buffer_view_num_count: usize,
}

#[derive(Clone, Debug, Snafu)]
pub struct ParseDurationError {
    reason: String,
}

pub fn parse_duration(s: &str) -> Result<Duration, ParseDurationError> {
    ClDuration::from_str(s)
        .map(Duration::from)
        .map_err(|err| ParseDurationError {
            reason: err.to_string(),
        })
}
impl NonPermissionedBuilderOptions {
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

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let opt = NonPermissionedBuilderOptions::parse();

    let sequencer_version = SEQUENCER_VERSION;

    let l1_params = L1Params {
        url: opt.l1_provider_url,
    };

    let builder_key_pair = EthKeyPair::from_mnemonic(&opt.eth_mnemonic, opt.eth_account_index)?;

    let builder_params = BuilderParams {
        mnemonic: opt.eth_mnemonic,
        prefunded_accounts: vec![],
        eth_account_index: opt.eth_account_index,
    };

    let bootstrapped_view = ViewNumber::new(opt.view_number);

    let builder_server_url: Url = format!("http://0.0.0.0:{}", opt.port).parse().unwrap();

    let instance_state = build_instance_state(
        l1_params,
        builder_params,
        opt.state_peers,
        sequencer_version,
    )
    .unwrap();

    let api_response_timeout_duration = opt.max_api_timeout_duration;

    let buffer_view_num_count = opt.buffer_view_num_count;

    let _builder_config = BuilderConfig::init(
        builder_key_pair,
        bootstrapped_view,
        opt.channel_capacity,
        instance_state,
        opt.hotshot_event_streaming_url,
        builder_server_url,
        api_response_timeout_duration,
        buffer_view_num_count,
    )
    .await;

    // Sleep forever
    async_std::future::pending::<()>().await;

    Ok(())
}
