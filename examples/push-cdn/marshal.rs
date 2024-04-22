//! The following is the main `Marshal` binary, which just instantiates and runs
//! a `Marshal` object.

use anyhow::Result;
use cdn_marshal::{Config, Marshal};
use clap::Parser;
use hotshot::traits::implementations::ProductionDef;
use hotshot_example_types::node_types::TestTypes;

// TODO: forall, add logging where we need it

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to
    #[arg(short, long)]
    discovery_endpoint: String,

    /// The port to bind to for connections (from users)
    #[arg(short, long, default_value_t = 1737)]
    bind_port: u16,

    /// The endpoint to bind to for externalizing metrics (in `IP:port` form). If not provided,
    /// metrics are not exposed.
    #[arg(short, long)]
    metrics_bind_endpoint: Option<String>,

    /// The path to the CA certificate
    /// If not provided, a local, pinned CA is used
    #[arg(long)]
    ca_cert_path: Option<String>,

    /// The path to the CA key
    /// If not provided, a local, pinned CA is used
    #[arg(long)]
    ca_key_path: Option<String>,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize tracing
    if std::env::var("RUST_LOG_FORMAT") == Ok("json".to_string()) {
        tracing_subscriber::fmt().json().init();
    } else {
        tracing_subscriber::fmt().init();
    }

    // Create a new `Config`
    let config = Config {
        discovery_endpoint: args.discovery_endpoint,
        bind_endpoint: format!("0.0.0.0:{}", args.bind_port),
        metrics_bind_endpoint: args.metrics_bind_endpoint,
        ca_cert_path: args.ca_cert_path,
        ca_key_path: args.ca_key_path,
    };

    // Create new `Marshal` from the config
    let marshal = Marshal::<ProductionDef<TestTypes>>::new(config).await?;

    // Start the main loop, consuming it
    marshal.start().await?;

    Ok(())
}
