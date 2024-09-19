use std::{net::ToSocketAddrs, sync::Arc};

use clap::Parser;
use espresso_types::{
    FeeVersion, MarketplaceVersion, SequencerVersions, SolverAuctionResultsProvider, V0_0, V0_1,
};
use futures::future::FutureExt;
use hotshot::MarketplaceConfig;
use hotshot_types::traits::{metrics::NoMetrics, node_implementation::Versions};
use sequencer::{
    api::{self, data_source::DataSourceOptions},
    init_node,
    options::{Modules, Options},
    persistence, Genesis, L1Params, NetworkParams,
};
use vbs::version::StaticVersionType;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Options::parse();
    opt.logging.init();

    let modules = opt.modules();
    tracing::warn!(?modules, "sequencer starting up");

    let genesis = Genesis::from_file(&opt.genesis_file)?;

    // validate that the fee contract is a proxy and panic otherwise
    genesis
        .validate_fee_contract(opt.l1_provider_url.to_string())
        .await
        .unwrap();

    tracing::info!(?genesis, "genesis");

    let base = genesis.versions.base;
    let upgrade = genesis.versions.upgrade;

    match (base, upgrade) {
        (V0_1::VERSION, FeeVersion::VERSION) => {
            run(
                genesis,
                modules,
                opt,
                SequencerVersions::<V0_1, FeeVersion>::new(),
            )
            .await
        }
        (FeeVersion::VERSION, MarketplaceVersion::VERSION) => {
            run(
                genesis,
                modules,
                opt,
                SequencerVersions::<FeeVersion, MarketplaceVersion>::new(),
            )
            .await
        }
        (V0_1::VERSION, _) => {
            run(
                genesis,
                modules,
                opt,
                SequencerVersions::<V0_1, V0_0>::new(),
            )
            .await
        }
        (FeeVersion::VERSION, _) => {
            run(
                genesis,
                modules,
                opt,
                SequencerVersions::<FeeVersion, V0_0>::new(),
            )
            .await
        }
        (MarketplaceVersion::VERSION, _) => {
            run(
                genesis,
                modules,
                opt,
                SequencerVersions::<MarketplaceVersion, V0_0>::new(),
            )
            .await
        }
        _ => panic!(
            "Invalid base ({base}) and upgrade ({upgrade}) versions specified in the toml file."
        ),
    }
}

async fn run<V>(
    genesis: Genesis,
    mut modules: Modules,
    opt: Options,
    versions: V,
) -> anyhow::Result<()>
where
    V: Versions,
{
    if let Some(storage) = modules.storage_fs.take() {
        init_with_storage(genesis, modules, opt, storage, versions).await
    } else if let Some(storage) = modules.storage_sql.take() {
        init_with_storage(genesis, modules, opt, storage, versions).await
    } else {
        // Persistence is required. If none is provided, just use the local file system.
        init_with_storage(
            genesis,
            modules,
            opt,
            persistence::fs::Options::default(),
            versions,
        )
        .await
    }
}

async fn init_with_storage<S, V>(
    genesis: Genesis,
    modules: Modules,
    opt: Options,
    storage_opt: S,
    versions: V,
) -> anyhow::Result<()>
where
    S: DataSourceOptions,
    V: Versions,
{
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
        libp2p_bootstrap_nodes: opt.libp2p_bootstrap_nodes,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        public_api_url: opt.public_api_url,
        private_staking_key,
        private_state_key,
        state_peers: opt.state_peers,
        config_peers: opt.config_peers,
        catchup_backoff: opt.catchup_backoff,
    };

    let marketplace_config = MarketplaceConfig {
        auction_results_provider: Arc::new(SolverAuctionResultsProvider {
            url: opt.auction_results_solver_url,
            marketplace_path: opt.marketplace_solver_path,
            results_path: opt.auction_results_path,
        }),
        fallback_builder_url: opt.fallback_builder_url,
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
                .serve(move |metrics| {
                    async move {
                        init_node(
                            genesis,
                            network_params,
                            &*metrics,
                            storage_opt,
                            l1_params,
                            versions,
                            opt.is_da,
                            opt.identity,
                            marketplace_config,
                        )
                        .await
                        .unwrap()
                    }
                    .boxed()
                })
                .await?
        }
        None => {
            init_node(
                genesis,
                network_params,
                &NoMetrics,
                storage_opt,
                l1_params,
                versions,
                opt.is_da,
                opt.identity,
                marketplace_config,
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
    use std::time::Duration;

    use async_std::task::spawn;

    use espresso_types::{MockSequencerVersions, PubKey};
    use hotshot_types::{light_client::StateKeyPair, traits::signature_key::SignatureKey};
    use portpicker::pick_unused_port;
    use sequencer::{
        api::options::{Http, Status},
        genesis::{GenesisSequencerVersions, StakeTableConfig},
        persistence::fs,
        SequencerApiVersion,
    };
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::{error::ClientError, Client, Url};
    use tempfile::TempDir;
    use vbs::version::Version;

    use super::*;

    #[async_std::test]
    async fn test_startup_before_orchestrator() {
        setup_test();

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
            upgrades: Default::default(),
            versions: GenesisSequencerVersions {
                base: Version { major: 0, minor: 1 },
                upgrade: Version { major: 0, minor: 2 },
            },
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
                genesis,
                modules,
                opt,
                fs::Options::new(tmp.path().into()),
                MockSequencerVersions::new(),
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
        let client = Client::<ClientError, SequencerApiVersion>::new(url.clone());
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
