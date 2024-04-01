//! The following is the main `Broker` binary, which just instantiates and runs
//! a `Broker` object.

use anyhow::{Context, Result};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use cdn_broker::{Broker, Config, ConfigBuilder};
use clap::Parser;

use hotshot::traits::implementations::{KeyPair, ProductionDef, WrappedSignatureKey};
use hotshot_types::traits::node_implementation::NodeType;
use hotshot_types::traits::signature_key::SignatureKey;
use sequencer::SeqTypes;
use sha2::Digest;

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
    #[arg(long, default_value_t = false)]
    metrics_enabled: bool,

    /// The IP to bind to for externalizing metrics
    #[arg(long, default_value = "127.0.0.1")]
    metrics_ip: String,

    /// The port to bind to for externalizing metrics
    #[arg(long, default_value_t = 9090)]
    metrics_port: u16,

    /// The user-facing address to bind to for connections from users
    #[arg(long, default_value = "0.0.0.0:1738")]
    public_bind_address: String,

    /// The user-facing address to advertise
    #[arg(long, default_value = "local_ip:1738")]
    public_advertise_address: String,

    /// The broker-facing address to bind to for connections from  
    /// other brokers
    #[arg(long, default_value = "0.0.0.0:1739")]
    private_bind_address: String,

    /// The broker-facing address to advertise
    #[arg(long, default_value = "local_ip:1739")]
    private_advertise_address: String,

    /// The seed for broker key generation
    #[arg(long, default_value_t = 0)]
    key_seed: u64,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    setup_logging();
    setup_backtrace();

    // Generate the broker key from the supplied seed
    let key_hash = sha2::Sha256::digest(args.key_seed.to_le_bytes());
    let (public_key, private_key) =
        <SeqTypes as NodeType>::SignatureKey::generated_from_seed_indexed(key_hash.into(), 1337);

    let broker_config: Config<WrappedSignatureKey<<SeqTypes as NodeType>::SignatureKey>> =
        ConfigBuilder::default()
            .public_advertise_address(args.public_advertise_address)
            .public_bind_address(args.public_bind_address)
            .private_advertise_address(args.private_advertise_address)
            .private_bind_address(args.private_bind_address)
            .metrics_enabled(args.metrics_enabled)
            .metrics_ip(args.metrics_ip)
            .discovery_endpoint(args.discovery_endpoint)
            .metrics_port(args.metrics_port)
            .keypair(KeyPair {
                public_key: WrappedSignatureKey(public_key),
                private_key,
            })
            .build()
            .with_context(|| "failed to build broker config")?;

    // Create new `Broker`
    // Uses TCP from broker connections and Quic for user connections.
    let broker = Broker::<ProductionDef<SeqTypes>>::new(broker_config).await?;

    // Start the main loop, consuming it
    broker.start().await?;

    Ok(())
}
