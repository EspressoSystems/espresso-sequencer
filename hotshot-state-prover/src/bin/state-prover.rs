use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use cld::ClDuration;
use hotshot_state_prover::service::run_prover_service;
use snafu::Snafu;
use std::{str::FromStr as _, time::Duration};
use url::Url;

#[derive(Parser)]
struct Args {
    /// Start the prover service daemon
    #[clap(short, long, action)]
    daemon: bool,

    /// The frequency of updating the light client state
    #[clap(short, long, value_parser = parse_duration, default_value = "30m", env = "ESPRESSO_STATE_PROVER_FREQUENCY")]
    frequency: Duration,

    /// Url of the state relay server
    #[clap(
        short,
        long,
        default_value = "http://localhost:8083",
        env = "ESPRESSO_STATE_RELAY_SERVER_URL"
    )]
    relay_server: Url,
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

    if args.daemon {
        run_prover_service(args.relay_server).await;
    }
}
