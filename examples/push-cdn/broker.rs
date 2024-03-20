//! The following is the main `Broker` binary, which just instantiates and runs
//! a `Broker` object.

use anyhow::{Context, Result};
use cdn_broker::reexports::connection::protocols::{Quic, Tcp};
use cdn_broker::{reexports::crypto::signature::KeyPair, Broker, Config, ConfigBuilder};
use clap::Parser;
use hotshot::traits::implementations::WrappedSignatureKey;
use hotshot::types::SignatureKey;
use hotshot_example_types::node_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeType;
use local_ip_address::local_ip;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to.
    /// With the local discovery feature, this is a file path.
    /// With the remote (redis) discovery feature, this is a redis URL (e.g. `redis://127.0.0.1:6789`).
    #[arg(short, long)]
    discovery_endpoint: String,

    /// Whether or not metric collection and serving is enabled
    #[arg(long, default_value_t = true)]
    metrics_enabled: bool,

    /// The IP to bind to for externalizing metrics
    #[arg(long, default_value = "127.0.0.1")]
    metrics_ip: String,

    /// The port to bind to for externalizing metrics
    #[arg(long, default_value_t = 9090)]
    metrics_port: u16,

    /// The port to bind to for connections from users
    #[arg(long, default_value = "127.0.0.1:1738")]
    public_advertise_address: String,

    /// The (public) port to bind to for connections from users
    #[arg(long, default_value_t = 1738)]
    public_bind_port: u16,

    /// The (private) port to bind to for connections from other brokers
    #[arg(long, default_value_t = 1739)]
    private_bind_port: u16,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get our local IP address
    let private_ip_address = local_ip().with_context(|| "failed to get local IP address")?;
    let private_address = format!("{}:{}", private_ip_address, args.private_bind_port);

    // Create deterministic keys for brokers (for now, obviously)
    let (public_key, private_key) =
        <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([0u8; 32], 1337);

    let broker_config: Config<WrappedSignatureKey<<TestTypes as NodeType>::SignatureKey>> =
        ConfigBuilder::default()
            .public_advertise_address(args.public_advertise_address)
            .public_bind_address(format!("0.0.0.0:{}", args.public_bind_port))
            .private_advertise_address(private_address.clone())
            .private_bind_address(private_address)
            .discovery_endpoint(args.discovery_endpoint)
            .metrics_port(args.metrics_port)
            .keypair(KeyPair {
                public_key: WrappedSignatureKey(public_key),
                private_key,
            })
            .build()
            .with_context(|| "failed to build broker configuration")?;

    // Create new `Broker`
    // Uses TCP from broker connections and Quic for user connections.
    let broker = Broker::<
        WrappedSignatureKey<<TestTypes as NodeType>::SignatureKey>,
        WrappedSignatureKey<<TestTypes as NodeType>::SignatureKey>,
        Tcp,
        Quic,
    >::new(broker_config)
    .await?;

    // Start the main loop, consuming it
    broker.start().await?;

    Ok(())
}
