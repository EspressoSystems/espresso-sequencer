//! The following is the main `Broker` binary, which just instantiates and runs
//! a `Broker` object.

use anyhow::{Context, Result};
use cdn_broker::{reexports::crypto::signature::KeyPair, Broker, Config, ConfigBuilder};
use clap::Parser;

use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use sequencer::{
    network::cdn::{ProductionDef, WrappedSignatureKey},
    SeqTypes,
};
use sha2::Digest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The main component of the push CDN.
struct Args {
    /// The discovery client endpoint (including scheme) to connect to.
    /// With the local discovery feature, this is a file path.
    /// With the remote (redis) discovery feature, this is a redis URL (e.g. `redis://127.0.0.1:6789`).
    #[arg(short, long, env = "ESPRESSO_CDN_BROKER_DISCOVERY_ENDPOINT")]
    discovery_endpoint: String,

    /// Whether or not metric collection and serving is enabled
    #[arg(
        long,
        default_value_t = false,
        env = "ESPRESSO_CDN_BROKER_METRICS_ENABLED"
    )]
    metrics_enabled: bool,

    /// The IP to bind to for externalizing metrics
    #[arg(
        long,
        default_value = "127.0.0.1",
        env = "ESPRESSO_CDN_BROKER_METRICS_IP"
    )]
    metrics_ip: String,

    /// The port to bind to for externalizing metrics
    #[arg(long, default_value_t = 9090, env = "ESPRESSO_CDN_BROKER_METRICS_PORT")]
    metrics_port: u16,

    /// The user-facing address to bind to for connections from users
    #[arg(
        long,
        default_value = "0.0.0.0:1738",
        env = "ESPRESSO_CDN_BROKER_PUBLIC_BIND_ADDRESS"
    )]
    public_bind_address: String,

    /// The user-facing address to advertise
    #[arg(
        long,
        default_value = "local_ip:1738",
        env = "ESPRESSO_CDN_BROKER_PUBLIC_ADVERTISE_ADDRESS"
    )]
    public_advertise_address: String,

    /// The broker-facing address to bind to for connections from  
    /// other brokers
    #[arg(
        long,
        default_value = "0.0.0.0:1739",
        env = "ESPRESSO_CDN_BROKER_PRIVATE_BIND_ADDRESS"
    )]
    private_bind_address: String,

    /// The broker-facing address to advertise
    #[arg(
        long,
        default_value = "local_ip:1739",
        env = "ESPRESSO_CDN_BROKER_PRIVATE_ADVERTISE_ADDRESS"
    )]
    private_advertise_address: String,

    /// The seed for broker key generation
    #[arg(long, default_value_t = 0, env = "ESPRESSO_CDN_BROKER_KEY_SEED")]
    key_seed: u64,
}

#[async_std::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

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
