use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use sequencer::{api::serve, init_node, Block, ChainVariables, GenesisTransaction};
use std::net::ToSocketAddrs;
use url::Url;

#[derive(Parser)]
struct Args {
    /// Unique identifier for this instance of the sequencer network.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    chain_id: u16,

    /// Port that the sequencer API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PORT")]
    port: u16,

    /// URL of the HotShot CDN.
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_CDN_URL")]
    cdn_url: Url,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();

    // Create genesis block.
    let genesis = Block::genesis(GenesisTransaction {
        chain_variables: ChainVariables::new(
            args.chain_id,
            0, // committee_size, unused
        ),
    });

    // Initialize HotShot.
    let cdn_addr = (
        args.cdn_url.host_str().unwrap(),
        args.cdn_url.port_or_known_default().unwrap(),
    )
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let mut handle = init_node(cdn_addr, genesis).await;

    // Run consensus.
    handle.start().await;
    while let Ok(event) = handle.next_event().await {
        tracing::info!("EVENT {:?}", event);
    }
    tracing::warn!("shutting down");

    serve(handle, args.port).expect("Failed to start serving API");
}
