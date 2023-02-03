use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_query_service::data_source::QueryData;
use sequencer::{
    api::{serve, HandleFromMetrics},
    init_node, Block, ChainVariables, GenesisTransaction,
};
use std::{
    net::ToSocketAddrs,
    path::{Path, PathBuf},
};
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

    /// Storage path for HotShot query service data.
    #[clap(long, env = "ESPRESSO_SEQUENCER_STORAGE_PATH")]
    storage_path: PathBuf,

    /// Create new query storage instead of opening existing one.
    #[clap(long, env = "ESPRESSO_SEQUENCER_RESET_STORE")]
    reset_store: bool,
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

    let cdn_addr = (
        args.cdn_url.host_str().unwrap(),
        args.cdn_url.port_or_known_default().unwrap(),
    )
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let init_handle: HandleFromMetrics<_> =
        Box::new(move |metrics| Box::pin(init_node(cdn_addr, genesis, metrics)));

    let storage_path = Path::new(&args.storage_path);

    let query_data = {
        if args.reset_store {
            QueryData::create(storage_path, ())
        } else {
            QueryData::open(storage_path, ())
        }
    }
    .expect("Failed to initialize query data storage");

    let (handle, task) = serve(query_data, init_handle, args.port)
        .await
        .expect("Failed to initialize API");

    // Start doing consensus.
    handle.start().await;

    // Block on the API server.
    task.await.expect("Error in API server");
}
