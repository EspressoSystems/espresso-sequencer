use std::{str::FromStr as _, time::Duration};

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use cld::ClDuration;
use es_version::SEQUENCER_VERSION;
use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
    types::Address,
};
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_state_prover::service::{run_prover_once, run_prover_service, StateProverConfig};
use snafu::Snafu;
use url::Url;

#[derive(Parser)]
struct Args {
    /// Start the prover service daemon
    #[clap(short, long, action)]
    daemon: bool,

    /// Url of the state relay server
    #[clap(
        long,
        default_value = "http://localhost:8083",
        env = "ESPRESSO_STATE_RELAY_SERVER_URL"
    )]
    relay_server: Url,

    /// The frequency of updating the light client state, expressed in update interval
    #[clap(short, long = "freq", value_parser = parse_duration, default_value = "10m", env = "ESPRESSO_STATE_PROVER_UPDATE_INTERVAL")]
    update_interval: Duration,

    /// Interval between retries if a state update fails
    #[clap(long = "retry-freq", value_parser = parse_duration, default_value = "2s", env = "ESPRESSO_STATE_PROVER_RETRY_INTERVAL")]
    retry_interval: Duration,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_PROVIDER",
        default_value = "http://localhost:8545"
    )]
    l1_provider: Url,

    /// Address of LightClient contract on layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIGHTCLIENT_ADDRESS")]
    light_client_address: Address,

    /// Mnemonic phrase for a funded Ethereum wallet.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC", default_value = None)]
    eth_mnemonic: String,

    /// Index of a funded account derived from eth-mnemonic.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_STATE_PROVER_ACCOUNT_INDEX",
        default_value = "0"
    )]
    eth_account_index: u32,

    /// URL of a sequencer node that is currently providing the HotShot config.
    /// This is used to initialize the stake table.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_URL",
        default_value = "http://localhost:24000"
    )]
    pub sequencer_url: Url,

    /// If daemon and provided, the service will run a basic HTTP server on the given port.
    ///
    /// The server provides healthcheck and version endpoints.
    #[clap(short, long, env = "ESPRESSO_PROVER_SERVICE_PORT")]
    pub port: Option<u16>,

    /// Stake table capacity for the prover circuit
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_STAKE_TABLE_CAPACITY", default_value_t = STAKE_TABLE_CAPACITY)]
    pub stake_table_capacity: usize,
}

#[derive(Clone, Debug, Snafu)]
pub struct ParseDurationError {
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
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();

    // prepare config for state prover from user options
    let provider = Provider::<Http>::try_from(args.l1_provider.to_string()).unwrap();
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let config = StateProverConfig {
        relay_server: args.relay_server,
        update_interval: args.update_interval,
        retry_interval: args.retry_interval,
        l1_provider: args.l1_provider,
        light_client_address: args.light_client_address,
        eth_signing_key: MnemonicBuilder::<English>::default()
            .phrase(args.eth_mnemonic.as_str())
            .index(args.eth_account_index)
            .expect("error building wallet")
            .build()
            .expect("error opening wallet")
            .with_chain_id(chain_id)
            .signer()
            .clone(),
        sequencer_url: args.sequencer_url,
        port: args.port,
        stake_table_capacity: args.stake_table_capacity,
    };

    if args.daemon {
        // Launching the prover service daemon
        if let Err(err) = run_prover_service(config, SEQUENCER_VERSION).await {
            tracing::error!("Error running prover service: {:?}", err);
        };
    } else {
        // Run light client state update once
        if let Err(err) = run_prover_once(config, SEQUENCER_VERSION).await {
            tracing::error!("Error running prover once: {:?}", err);
        };
    }
}
