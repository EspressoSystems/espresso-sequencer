use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use cld::ClDuration;
use hotshot_state_prover::service::{key_gen, run_prover_once, run_prover_service};
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

    /// The frequency of updating the light client state
    #[clap(short, long = "freq", value_parser = parse_duration, default_value = "10m", env = "ESPRESSO_STATE_PROVER_FREQUENCY")]
    frequency: Duration,

    /// Url of the state relay server
    #[clap(
        long,
        default_value = "http://localhost:8083",
        env = "ESPRESSO_STATE_RELAY_SERVER_URL"
    )]
    relay_server: Url,

    /// Path to the proving key
    #[clap(
        short = 'k',
        long = "key",
        default_value = "key",
        env = "ESPRESSOS_STATE_PROVING_KEY"
    )]
    proving_key_path: PathBuf,
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

    if args.keygen {
        // Key gen route
        key_gen(args.proving_key_path)
    } else if args.daemon {
        // Launching the prover service daemon
        run_prover_service(args.proving_key_path, args.relay_server, args.frequency).await;
    } else {
        // Run light client state update once
        run_prover_once(args.proving_key_path, args.relay_server).await;
    }
}
