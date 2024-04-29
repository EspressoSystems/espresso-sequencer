//! The following is the main `Marshal` binary, which just instantiates and runs
//! a `Marshal` object.

use anyhow::Result;
use cdn_marshal::{Config, Marshal};
use clap::Parser;
use sequencer::{network::cdn::ProductionDef, SeqTypes};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to
    #[arg(short, long, env = "ESPRESSO_CDN_MARSHAL_DISCOVERY_ENDPOINT")]
    discovery_endpoint: String,

    /// The port to bind to for connections (from users)
    #[arg(
        short,
        long,
        default_value_t = 1737,
        env = "ESPRESSO_CDN_MARSHAL_BIND_PORT"
    )]
    bind_port: u16,

    /// The endpoint to bind to for externalizing metrics (in `IP:port` form). If not provided,
    /// metrics are not exposed.
    #[arg(short, long, env = "ESPRESSO_CDN_MARSHAL_METRICS_BIND_ENDPOINT")]
    metrics_bind_endpoint: Option<String>,

    /// The path to the CA certificate
    /// If not provided, a local, pinned CA is used
    #[arg(long, env = "ESPRESSO_CDN_MARSHAL_CA_CERT_PATH")]
    ca_cert_path: Option<String>,

    /// The path to the CA key
    /// If not provided, a local, pinned CA is used
    #[arg(long, env = "ESPRESSO_CDN_MARSHAL_CA_KEY_PATH")]
    ca_key_path: Option<String>,
}

#[async_std::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize tracing
    if std::env::var("RUST_LOG_FORMAT") == Ok("json".to_string()) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
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
    let marshal = Marshal::<ProductionDef<SeqTypes>>::new(config).await?;

    // Start the main loop, consuming it
    marshal.start().await?;

    Ok(())
}
