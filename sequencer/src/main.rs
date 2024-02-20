use anyhow::bail;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use commit::Committable;
use futures::{future::FutureExt, stream::StreamExt};
use hotshot_query_service::availability::LeafQueryData;
use hotshot_types::traits::metrics::NoMetrics;
use sequencer::{
    api::{self, data_source::DataSourceOptions, SequencerNode},
    init_node, init_static, network,
    options::{Modules, Options},
    persistence, NetworkParams, SavedLeaf, SeqTypes,
};
use surf_disco::Client;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    init_static();

    tracing::info!("sequencer starting up");
    let opt = Options::parse();
    let mut modules = opt.modules();
    tracing::info!("modules: {:?}", modules);

    let mut node = if let Some(storage) = modules.storage_fs.take() {
        init_with_storage(modules, opt, storage).await?
    } else if let Some(storage) = modules.storage_sql.take() {
        init_with_storage(modules, opt, storage).await?
    } else {
        // Persistence is required. If none is provided, just use the local file system.
        init_with_storage(modules, opt, persistence::fs::Options::default()).await?
    };

    // Start doing consensus.
    node.start_consensus().await;

    // Wait for events just to keep the process from exiting before consensus exits.
    let mut events = node.context_mut().consensus_mut().get_event_stream();
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
) -> anyhow::Result<SequencerNode<network::Web>>
where
    S: DataSourceOptions,
{
    let network_params = NetworkParams {
        da_server_url: opt.da_server_url,
        consensus_server_url: opt.consensus_server_url,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        webserver_poll_interval: opt.webserver_poll_interval,
        private_staking_key: opt.private_staking_key,
    };

    // If we are starting from a saved state, load it from a query service.
    let saved_leaf = if let Some(leaf_hash) = opt.saved_leaf {
        if let Some(query_url) = &opt.query_service_url {
            tracing::info!("loading saved leaf {leaf_hash} from query service {query_url}");
            let client = Client::<hotshot_query_service::Error>::new(query_url.clone());
            client.connect(None).await;

            let leaf: LeafQueryData<SeqTypes> = client
                .get("availability/leaf/hash/{leaf_hash}")
                .send()
                .await?;
            // Check that the query service gave us the correct leaf.
            let provided_hash = leaf.leaf().commit();
            if leaf_hash != provided_hash {
                bail!(format!("query service {query_url} provided leaf with hash {provided_hash} when asked for {leaf_hash}: {leaf:?}"));
            }

            leaf.leaf().clone().into()
        } else {
            leaf_hash.into()
        }
    } else {
        SavedLeaf::default()
    };

    // Inititialize HotShot. If the user requested the HTTP module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    Ok(match modules.http {
        Some(opt) => {
            // Add optional API modules as requested.
            let mut opt = api::Options::from(opt).with_saved_leaf(saved_leaf);
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
            opt.serve(move |leaf, metrics| {
                async move {
                    init_node(network_params, leaf, &*metrics, &mut storage)
                        .await
                        .unwrap()
                }
                .boxed()
            })
            .await?
        }
        None => {
            // If we aren't running a query service locally, we cannot fetch saved leaf from our
            // local storage, so we better have fetched it from an external service if we want it at
            // all.
            let saved_leaf = saved_leaf.assert_resolved()?;
            init_node(
                network_params,
                saved_leaf,
                &NoMetrics,
                &mut storage_opt.create().await?,
            )
            .await?
            .into()
        }
    })
}
