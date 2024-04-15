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
    persistence, BuilderParams, L1Params, NetworkParams,
};
use versioned_binary_serialization::version::StaticVersionType;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    tracing::info!("sequencer starting up");
    let opt = Options::parse();
    let mut modules = opt.modules();
    tracing::info!("modules: {:?}", modules);

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
    let l1_params = L1Params {
        url: opt.l1_provider_url,
    };
    let builder_params = BuilderParams {
        mnemonic: opt.eth_mnemonic,
        prefunded_accounts: opt.prefunded_builder_accounts,
        eth_account_index: opt.eth_account_index,
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

    // Inititialize HotShot. If the user requested the HTTP module, we must initialize the handle in
    // a special way, in order to populate the API with consensus metrics. Otherwise, we initialize
    // the handle directly, with no metrics.
    let ctx = match modules.http {
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
            if let Some(state) = modules.state {
                opt = opt.state(state);
            }
            if let Some(catchup) = modules.catchup {
                opt = opt.catchup(catchup);
            }
            if let Some(hotshot_events) = modules.hotshot_events {
                opt = opt.hotshot_events(hotshot_events);
            }

            let storage = storage_opt.create().await?;
            opt.serve(
                move |metrics| {
                    async move {
                        init_node(
                            network_params,
                            &*metrics,
                            storage,
                            builder_params,
                            l1_params,
                            bind_version,
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
                storage_opt.create().await?,
                builder_params,
                l1_params,
                bind_version,
            )
            .await?
        }
    };

    // Start doing consensus.
    ctx.start_consensus().await;
    ctx.join().await;

    Ok(())
}
