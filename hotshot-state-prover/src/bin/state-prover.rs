use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use cld::ClDuration;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{coins_bip39::English, MnemonicBuilder, Signer};
use ethers::types::Address;
use hotshot_state_prover::service::{
    key_gen, run_prover_once, run_prover_service, StateProverConfig,
};
use snafu::Snafu;
use std::{path::PathBuf, str::FromStr as _, time::Duration};
use url::Url;

#[derive(Parser)]
struct Args {
    /// Start the prover service daemon
    #[clap(short, long, action)]
    daemon: bool,

    /// Generate proving key and verification key from Aztec's SRS
    #[clap(long = "key-gen", action)]
    keygen: bool,

    /// Path to the proving key
    #[clap(
        short = 'k',
        long = "key",
        default_value = "key",
        env = "ESPRESSOS_STATE_PROVING_KEY"
    )]
    proving_key_path: PathBuf,

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

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Address of LightClient contract on layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_LIGHTCLIENT_ADDRESS")]
    pub light_client_address: Address,

    /// Mnemonic phrase for a funded Ethereum wallet.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC", default_value = None)]
    pub eth_mnemonic: String,

    /// Index of a funded account derived from eth-mnemonic.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_ETH_ACCOUNT_INDEX",
        default_value = "0"
    )]
    pub eth_account_index: u32,
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
        proving_key_path: args.proving_key_path.clone(),
        relay_server: args.relay_server.clone(),
        update_interval: args.update_interval,
        l1_provider: args.l1_provider.clone(),
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
    };

    if args.keygen {
        // Key gen route
        key_gen(args.proving_key_path)
    } else if args.daemon {
        // Launching the prover service daemon
        run_prover_service(config).await;
    } else {
        // Run light client state update once
        run_prover_once(config).await;
    }
}
