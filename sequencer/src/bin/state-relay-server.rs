use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use es_version::SEQUENCER_VERSION;
use sequencer::state_signature::relay_server::run_relay_server;

#[derive(Parser)]
struct Args {
    /// Port to run the server on.
    #[clap(
        short,
        long,
        env = "ESPRESSO_STATE_RELAY_SERVER_PORT",
        default_value = "8083"
    )]
    port: u16,

    /// Threshold to form an available state signature package.
    /// WARNING: this is a temporary flag, should remove after integrating with stake table.
    /// Related issue: [https://github.com/EspressoSystems/espresso-sequencer/issues/1022]
    #[clap(
        short,
        long,
        env = "ESPRESSO_STATE_SIGNATURE_WEIGHT_THRESHOLD",
        default_value = "3"
    )]
    threshold: u64,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();

    tracing::info!("starting state relay server on port {}", args.port);
    run_relay_server(
        None,
        args.threshold,
        format!("http://0.0.0.0:{}", args.port).parse().unwrap(),
        SEQUENCER_VERSION,
    )
    .await
    .unwrap();
}
