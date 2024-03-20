//! The following is the main `Marshal` binary, which just instantiates and runs
//! a `Marshal` object with the `HotShot` types.
//!
use anyhow::{Context, Result};
use cdn_broker::reexports::connection::protocols::Quic;
use cdn_marshal::{ConfigBuilder, Marshal};
use clap::Parser;
use hotshot::traits::implementations::WrappedSignatureKey;
use hotshot_example_types::node_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to.
    /// With the local discovery feature, this is a file path.
    /// With the remote (redis) discovery feature, this is a redis URL (e.g. `redis://127.0.0.1:6789`).
    #[arg(short, long)]
    discovery_endpoint: String,

    /// The port to bind to for connections (from users)
    #[arg(short, long, default_value_t = 8082)]
    bind_port: u16,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a new `Config`
    let config = ConfigBuilder::default()
        .bind_address(format!("0.0.0.0:{}", args.bind_port))
        .discovery_endpoint(args.discovery_endpoint)
        .build()
        .with_context(|| "failed to build Marshal config")?;

    // Create new `Marshal` from the config
    let marshal =
        Marshal::<WrappedSignatureKey<<TestTypes as NodeType>::SignatureKey>, Quic>::new(config)
            .await?;

    // Start the main loop, consuming it
    marshal.start().await?;

    Ok(())
}
