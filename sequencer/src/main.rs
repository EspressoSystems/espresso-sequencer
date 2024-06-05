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
    persistence, Genesis, L1Params, NetworkParams,
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
    let genesis = Genesis::from_file(&opt.genesis_file)?;
    tracing::info!(?genesis, "genesis");

    let (private_staking_key, private_state_key) = opt.private_keys()?;
    let l1_params = L1Params {
        url: opt.l1_provider_url,
        events_max_block_range: opt.l1_events_max_block_range,
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
            if let Some(config) = modules.config {
                http_opt = http_opt.config(config);
            }
            http_opt
                .serve(
                    move |metrics| {
                        async move {
                            init_node(
                                genesis,
                                network_params,
                                &*metrics,
                                storage_opt,
                                l1_params,
                                bind_version,
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
                genesis,
                network_params,
                &NoMetrics,
                storage_opt,
                l1_params,
                bind_version,
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

#[cfg(test)]
mod test {
    use super::*;
    use async_std::task::spawn;
    use es_version::SequencerVersion;
    use hotshot_types::{light_client::StateKeyPair, traits::signature_key::SignatureKey};
    use portpicker::pick_unused_port;
    use sequencer::{
        api::options::{Http, Status},
        genesis::StakeTableConfig,
        persistence::fs,
        PubKey,
    };
    use std::time::Duration;
    use surf_disco::{error::ClientError, Client, Url};
    use tempfile::TempDir;

    #[async_std::test]
    async fn test_startup_before_orchestrator() {
        setup_logging();
        setup_backtrace();

        let (pub_key, priv_key) = PubKey::generated_from_seed_indexed([0; 32], 0);
        let state_key = StateKeyPair::generate_from_seed_indexed([0; 32], 0);

        let port = pick_unused_port().unwrap();
        let tmp = TempDir::new().unwrap();

        let genesis_file = tmp.path().join("genesis.toml");
        let genesis = Genesis {
            chain_config: Default::default(),
            stake_table: StakeTableConfig { capacity: 10 },
            accounts: Default::default(),
            l1_finalized: Default::default(),
            header: Default::default(),
            network: Default::default(),
        };
        genesis.to_file(&genesis_file).unwrap();

        let modules = Modules {
            http: Some(Http::with_port(port)),
            status: Some(Status),
            ..Default::default()
        };
        let opt = Options::parse_from([
            "sequencer",
            "--private-staking-key",
            &priv_key.to_string(),
            "--private-state-key",
            &state_key.sign_key_ref().to_string(),
            "--genesis-file",
            &genesis_file.display().to_string(),
        ]);

        // Start the sequencer in a background task. This process will not complete, because it will
        // be waiting for the orchestrator, but it should at least start up the API server and
        // populate some metrics.
        tracing::info!(port, "starting sequencer");
        let task = spawn(async move {
            if let Err(err) = init_with_storage(
                modules,
                opt,
                fs::Options::new(tmp.path().into()),
                SEQUENCER_VERSION,
            )
            .await
            {
                tracing::error!("failed to start sequencer: {err:#}");
            }
        });

        // The healthcheck should eventually come up even though the node is waiting for the
        // orchestrator.
        tracing::info!("waiting for API to start");
        let url: Url = format!("http://localhost:{port}").parse().unwrap();
        let client = Client::<ClientError, SequencerVersion>::new(url.clone());
        assert!(client.connect(Some(Duration::from_secs(60))).await);
        client.get::<()>("healthcheck").send().await.unwrap();

        // The metrics should include information about the node and software version. surf-disco
        // doesn't currently support fetching a plaintext file, so we use a raw reqwest client.
        let res = reqwest::get(url.join("/status/metrics").unwrap())
            .await
            .unwrap();
        assert!(res.status().is_success(), "{}", res.status());
        let metrics = res.text().await.unwrap();
        let lines = metrics.lines().collect::<Vec<_>>();
        assert!(
            lines.contains(&format!("consensus_node{{key=\"{pub_key}\"}} 1").as_str()),
            "{lines:#?}"
        );
        assert!(
            lines.contains(
                &format!(
                    "consensus_version{{desc=\"{}\",rev=\"{}\",timestamp=\"{}\"}} 1",
                    env!("VERGEN_GIT_DESCRIBE"),
                    env!("VERGEN_GIT_SHA"),
                    env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
                )
                .as_str()
            ),
            "{lines:#?}"
        );

        task.cancel().await;
    }
}
