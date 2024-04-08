//! web server example
use hotshot_example_types::state_types::TestTypes;
use hotshot_types::constants::WebServerVersion;
use std::sync::Arc;
use surf_disco::Url;
use vbs::version::StaticVersionType;

use async_compatibility_layer::{
    channel::oneshot,
    logging::{setup_backtrace, setup_logging},
};
use clap::Parser;

/// web server arguments
#[derive(Parser, Debug)]
struct WebServerArgs {
    /// url to run on
    url: Url,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    setup_backtrace();
    setup_logging();
    let args = WebServerArgs::parse();
    let (server_shutdown_sender, server_shutdown) = oneshot();
    let _sender = Arc::new(server_shutdown_sender);
    let _result = hotshot_web_server::run_web_server::<
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        WebServerVersion,
    >(
        Some(server_shutdown),
        args.url,
        WebServerVersion::instance(),
    )
    .await;
}
