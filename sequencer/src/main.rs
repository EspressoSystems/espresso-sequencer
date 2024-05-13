use std::net::ToSocketAddrs;

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use es_version::SEQUENCER_VERSION;
use futures::future::FutureExt;
use hotshot_types::traits::metrics::NoMetrics;
use sequencer::{
    api::{self, data_source::DataSourceOptions},
    init_node,
    options::{Modules, Options},
    persistence, BuilderParams, ChainConfig, L1Params, NetworkParams,
};
use vbs::version::StaticVersionType;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    tracing::warn!("sequencer starting up");
    let opt = Options::parse();
    let mut modules = opt.modules();
    tracing::warn!("modules: {:?}", modules);

    if let Some(storage) = modules.storage_fs.take() {
        init_with_storage(modules, opt, storage, SEQUENCER_VERSION).await
    } else if let Some(storage) = modules.storage_sql.take() {
        init_with_storage(modules, opt, storage, SEQUENCER_VERSION).await
    } else {
        // Persistence is required. If none is provided, just use the local file system.
        init_with_storage(
            modules,
            opt,
            persistence::fs::Options::default(),
            SEQUENCER_VERSION,
        )
        .await
    }
}

async fn init_with_storage<S, Ver: StaticVersionType + 'static>(
    modules: Modules,
    opt: Options,
    storage_opt: S,
    bind_version: Ver,
) -> anyhow::Result<()>
where
    S: DataSourceOptions,
{
    let (private_staking_key, private_state_key) = opt.private_keys()?;
    let stake_table_capacity = opt.stake_table_capacity;
    let chain_config = ChainConfig::new(opt.chain_id, opt.max_block_size, opt.base_fee);
    let l1_params = L1Params {
        url: opt.l1_provider_url,
        finalized_block: opt.l1_genesis,
        events_max_block_range: opt.l1_events_max_block_range,
    };
    let builder_params = BuilderParams {
        prefunded_accounts: opt.prefunded_builder_accounts,
    };

    // Parse supplied Libp2p addresses to their socket form
    // We expect all nodes to be reachable via IPv4, so we filter out any IPv6 addresses.
    // Downstream in HotShot we pin the IP address to v4, but this can be fixed in the future.
    let libp2p_advertise_address = opt
        .libp2p_advertise_address
        .to_socket_addrs()?
        .find(|x| x.is_ipv4())
        .ok_or(anyhow::anyhow!(
            "Failed to resolve Libp2p advertise address"
        ))?;
    let libp2p_bind_address = opt
        .libp2p_bind_address
        .to_socket_addrs()?
        .find(|x| x.is_ipv4())
        .ok_or(anyhow::anyhow!("Failed to resolve Libp2p bind address"))?;

    let network_params = NetworkParams {
        cdn_endpoint: opt.cdn_endpoint,
        libp2p_advertise_address,
        libp2p_bind_address,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        private_staking_key,
        private_state_key,
        state_peers: opt.state_peers,
    };

    // Initialize HotShot. If the user requested the HTTP module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    let ctx = match modules.http {
        Some(http_opt) => {
            // Add optional API modules as requested.
            let mut http_opt = api::Options::from(http_opt);
            if let Some(query) = modules.query {
                http_opt = storage_opt.enable_query_module(http_opt, query);
            }
            if let Some(submit) = modules.submit {
                http_opt = http_opt.submit(submit);
            }
            if let Some(status) = modules.status {
                http_opt = http_opt.status(status);
            }
            if let Some(state) = modules.state {
                http_opt = http_opt.state(state);
            }
            if let Some(catchup) = modules.catchup {
                http_opt = http_opt.catchup(catchup);
            }
            if let Some(hotshot_events) = modules.hotshot_events {
                http_opt = http_opt.hotshot_events(hotshot_events);
            }
            if let Some(explorer) = modules.explorer {
                http_opt = http_opt.explorer(explorer);
            }

            http_opt
                .serve(
                    move |metrics| {
                        async move {
                            init_node(
                                network_params,
                                &*metrics,
                                storage_opt,
                                builder_params,
                                l1_params,
                                stake_table_capacity,
                                bind_version,
                                chain_config,
                                opt.is_da,
                            )
                            .await
                            .unwrap()
                        }
                        .boxed()
                    },
                    bind_version,
                )
                .await?
        }
        None => {
            init_node(
                network_params,
                &NoMetrics,
                storage_opt,
                builder_params,
                l1_params,
                stake_table_capacity,
                bind_version,
                chain_config,
                opt.is_da,
            )
            .await?
        }
    };

    // Start doing consensus.
    ctx.start_consensus().await;
    ctx.join().await;

    Ok(())
}
