use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::future::{join_all, FutureExt};
use hotshot_types::traits::metrics::NoMetrics;
use sequencer::{
    api::{serve, SequencerNode},
    hotshot_commitment::run_hotshot_commitment_task,
    init_node, Block, Options,
};
use std::net::ToSocketAddrs;

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    tracing::info!("sequencer starting up");
    let opt = Options::parse();
    let modules = opt.modules();
    tracing::info!("modules: {:?}", modules);

    // Create genesis block.
    let genesis = Block::genesis();

    let web_server_addr = (
        opt.web_server_url.host_str().unwrap(),
        opt.web_server_url.port_or_known_default().unwrap(),
    )
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let mut tasks = vec![];

    // Inititialize HotShot. If the user requested the API module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    let (mut handle, api_port) = match modules.api {
        Some(options) => {
            let port = options.port;
            let init_handle =
                Box::new(move |metrics| init_node(web_server_addr, genesis, metrics).boxed());
            let SequencerNode { handle, .. } = serve(options, init_handle)
                .await
                .expect("Failed to initialize API");
            (handle, Some(port))
        }
        None => (
            init_node(web_server_addr, genesis, Box::new(NoMetrics))
                .await
                .0,
            None,
        ),
    };
    // Register a task to run consensus.
    tasks.push(
        async move {
            // Start doing consensus.
            handle.start().await;

            // Wait for events just to keep the process from exiting before consensus exits.
            loop {
                let event = handle.next_event().await;
                tracing::debug!(?event);
            }
        }
        .boxed(),
    );

    // Register a task to run the HotShot commitment module, if requested.
    if let Some(mut options) = modules.commitment_task {
        // If no query service is specified, use the one of this node.
        if options.query_service_url.is_none() {
            options.query_service_url = Some(
                format!(
                    "http://localhost:{}",
                    api_port.expect("API port is required when running commitment task")
                )
                .parse()
                .unwrap(),
            );
        }
        tasks.push(
            async move {
                tracing::info!("Starting HotShot commitment task");
                run_hotshot_commitment_task(&options).await
            }
            .boxed(),
        )
    }

    // Run all tasks in parallel.
    join_all(tasks).await;
}
