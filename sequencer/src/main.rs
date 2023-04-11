use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::join;
use hotshot_query_service::data_source::QueryData;
use sequencer::{
    api::{serve, HandleFromMetrics, SequencerNode},
    hotshot_commitment::run_hotshot_commitment_task,
    init_node, Block, ChainVariables, GenesisTransaction, HotShotContractCommand, Options,
};
use std::{net::ToSocketAddrs, path::Path};

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();

    // Create genesis block.
    let genesis = Block::genesis(GenesisTransaction {
        chain_variables: ChainVariables::new(
            opt.chain_id,
            0, // committee_size, unused
        ),
    });

    let cdn_addr = (
        opt.cdn_url.host_str().unwrap(),
        opt.cdn_url.port_or_known_default().unwrap(),
    )
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let init_handle: HandleFromMetrics<_> =
        Box::new(move |metrics| Box::pin(init_node(cdn_addr, genesis, metrics)));

    let storage_path = Path::new(&opt.storage_path);

    let query_data = {
        if opt.reset_store {
            QueryData::create(storage_path, ())
        } else {
            QueryData::open(storage_path, ())
        }
    }
    .expect("Failed to initialize query data storage");

    let SequencerNode {
        handle,
        update_task,
        node_index,
    } = serve(query_data, init_handle, opt.port)
        .await
        .expect("Failed to initialize API");

    let run_sequencer = async {
        // Start doing consensus.
        handle.start().await;

        // Block on the API server.
        update_task.await.expect("Error in API server");
    };

    if let (Some(HotShotContractCommand::HotShotContractOptions(hotshot_contract_options)), 0) =
        (opt.hotshot_contract_options, node_index)
    {
        join!(
            run_sequencer,
            run_hotshot_commitment_task(&hotshot_contract_options)
        );
    } else {
        run_sequencer.await;
    }
}
