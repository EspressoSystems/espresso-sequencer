use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use sequencer::state_signature::relay_server::run_relay_server;

#[derive(Parser)]
struct Args {
    /// Port to run the server on.
    #[clap(short, long, env = "ESPRESSO_STATE_RELAY_SERVER_PORT")]
    port: u16,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();

    tracing::info!("starting state relay server on port {}", args.port);
    run_relay_server(
        None,
        format!("http://0.0.0.0:{}", args.port).parse().unwrap(),
    )
    .await
    .unwrap();
}
