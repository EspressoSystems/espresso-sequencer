use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::{future::FutureExt, stream::StreamExt};
use hotshot_types::traits::metrics::NoMetrics;
use sequencer::{
    api::{self, data_source::DataSourceOptions, SequencerNode},
    context::SequencerContext,
    init_node, init_static, network,
    options::{Modules, Options},
    persistence, BuilderParams, NetworkParams,
};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    init_static();

    tracing::info!("sequencer starting up");
    let opt = Options::parse();
    let mut modules = opt.modules();
    tracing::info!("modules: {:?}", modules);

    let mut context = if let Some(storage) = modules.storage_fs.take() {
        init_with_storage(modules, opt, storage).await?
    } else if let Some(storage) = modules.storage_sql.take() {
        init_with_storage(modules, opt, storage).await?
    } else {
        // Persistence is required. If none is provided, just use the local file system.
        init_with_storage(modules, opt, persistence::fs::Options::default()).await?
    };

    // Start doing consensus.
    context.start_consensus().await;

    // Wait for events just to keep the process from exiting before consensus exits.
    let mut events = context.consensus_mut().get_event_stream();
    while let Some(event) = events.next().await {
        tracing::debug!(?event);
    }
    tracing::debug!("event stream ended");
    Ok(())
}

async fn init_with_storage<S>(
    modules: Modules,
    opt: Options,
    storage_opt: S,
) -> anyhow::Result<SequencerContext<network::Web>>
where
    S: DataSourceOptions,
{
    let builder_params = BuilderParams {
        mnemonic: opt.eth_mnemonic,
        prefunded_accounts: opt.prefunded_builder_accounts,
        eth_account_index: opt.eth_account_index,
    };
    let network_params = NetworkParams {
        da_server_url: opt.da_server_url,
        consensus_server_url: opt.consensus_server_url,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        webserver_poll_interval: opt.webserver_poll_interval,
        private_staking_key: opt.private_staking_key,
        private_state_key: opt.private_state_key,
    };

    // Inititialize HotShot. If the user requested the HTTP module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    Ok(match modules.http {
        Some(opt) => {
            // Add optional API modules as requested.
            let mut opt = api::Options::from(opt);
            if let Some(query) = modules.query {
                opt = storage_opt.enable_query_module(opt, query);
            }
            if let Some(submit) = modules.submit {
                opt = opt.submit(submit);
            }
            if let Some(status) = modules.status {
                opt = opt.status(status);
            }
            let mut storage = storage_opt.create().await?;
            let SequencerNode { context, .. } = opt
                .serve(move |metrics| {
                    async move {
                        init_node(network_params, &*metrics, &mut storage, builder_params)
                            .await
                            .unwrap()
                    }
                    .boxed()
                })
                .await?;
            context
        }
        None => {
            init_node(
                network_params,
                &NoMetrics,
                &mut storage_opt.create().await?,
                builder_params,
            )
            .await?
        }
    })
}
