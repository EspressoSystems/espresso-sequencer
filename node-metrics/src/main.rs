use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use node_metrics::{run_standalone_service, Options};

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    run_standalone_service(Options::parse()).await;
}
