use clap::Parser;
use hotshot::helpers::initialize_logging;
use node_metrics::{run_standalone_service, Options};

#[tokio::main]
async fn main() {
    initialize_logging();

    run_standalone_service(Options::parse()).await;
}
