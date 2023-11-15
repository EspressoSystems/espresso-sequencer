use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::{
    future::{join_all, FutureExt},
    stream::StreamExt,
};
use hotshot_types::traits::metrics::NoMetrics;
use sequencer::{
    api::{self, SequencerNode},
    hotshot_commitment::run_hotshot_commitment_task,
    init_node, init_static, Block, NetworkParams, Options,
};

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    init_static();

    tracing::info!("sequencer starting up");
    let opt = Options::parse();
    let modules = opt.modules();
    tracing::info!("modules: {:?}", modules);

    // Create genesis block.
    let genesis = Block::genesis();

    let mut tasks = vec![];
    let network_params = NetworkParams {
        da_server_url: opt.da_server_url,
        consensus_server_url: opt.consensus_server_url,
        orchestrator_url: opt.orchestrator_url,
    };

    // Inititialize HotShot. If the user requested the HTTP module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    let (mut handle, query_api_port) = match modules.http {
        Some(opt) => {
            // Add optional API modules as requested.
            let mut opt = api::Options::from(opt);
            if let Some(query_fs) = modules.query_fs {
                opt = opt.query_fs(query_fs);
            }
            if let Some(submit) = modules.submit {
                opt = opt.submit(submit);
            }

            // Save the port if we are running a query API. This can be used later when starting the
            // commitment task; otherwise the user must give us the URL of an external query API.
            let query_api_port = if opt.query_fs.is_some() {
                Some(opt.http.port)
            } else {
                None
            };
            let init_handle =
                Box::new(move |metrics| init_node(network_params, genesis, metrics).boxed());
            let SequencerNode { handle, .. } = opt
                .serve(init_handle)
                .await
                .expect("Failed to initialize API");
            (handle, query_api_port)
        }
        None => (
            init_node(network_params, genesis, Box::new(NoMetrics))
                .await
                .0,
            None,
        ),
    };
    // Register a task to run consensus.
    tasks.push(
        async move {
            // Start doing consensus.
            handle.hotshot.start_consensus().await;

            // Wait for events just to keep the process from exiting before consensus exits.
            let mut events = handle.get_event_stream(Default::default()).await.0;
            while let Some(event) = events.next().await {
                tracing::debug!(?event);
            }
            tracing::debug!("event stream ended");
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
                    query_api_port
                        .expect("Query API port is required when running commitment task")
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
