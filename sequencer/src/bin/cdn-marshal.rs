//! The following is the main `Marshal` binary, which just instantiates and runs
//! a `Marshal` object.

use anyhow::{Context, Result};
use cdn_marshal::{ConfigBuilder, Marshal};
use clap::Parser;
use sequencer::{network::cdn::ProductionDef, SeqTypes};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to
    /// With the local discovery feature, this is a file path.
    /// With the remote (redis) discovery feature, this is a redis URL (e.g. `redis://127.0.0.1:6789`).
    #[arg(short, long, env = "ESPRESSO_CDN_MARSHAL_DISCOVERY_ENDPOINT")]
    discovery_endpoint: String,

    /// Whether or not metric collection and serving is enabled
    #[arg(
        long,
        default_value_t = false,
        env = "ESPRESSO_CDN_MARSHAL_METRICS_ENABLED"
    )]
    metrics_enabled: bool,

    /// The IP to bind to for externalizing metrics
    #[arg(
        long,
        default_value = "127.0.0.1",
        env = "ESPRESSO_CDN_MARSHAL_METRICS_IP"
    )]
    metrics_ip: String,

    /// The port to bind to for externalizing metrics
    #[arg(
        long,
        default_value_t = 9090,
        env = "ESPRESSO_CDN_MARSHAL_METRICS_PORT"
    )]
    metrics_port: u16,

    /// The port to bind to for connections (from users)
    #[arg(
        short,
        long,
        default_value_t = 1737,
        env = "ESPRESSO_CDN_MARSHAL_BIND_PORT"
    )]
    bind_port: u16,
}

#[async_std::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new `Config`
    let config = ConfigBuilder::default()
        .bind_address(format!("0.0.0.0:{}", args.bind_port))
        .metrics_enabled(args.metrics_enabled)
        .metrics_ip(args.metrics_ip)
        .metrics_port(args.metrics_port)
        .discovery_endpoint(args.discovery_endpoint)
        .build()
        .with_context(|| "failed to build Marshal config")?;

    // Create new `Marshal` from the config
    let marshal = Marshal::<ProductionDef<SeqTypes>>::new(config).await?;

    // Start the main loop, consuming it
    marshal.start().await?;

    Ok(())
}
