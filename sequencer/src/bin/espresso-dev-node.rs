use std::{sync::Arc, time::Duration};

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::{sleep, spawn, spawn_blocking};
use clap::Parser;
use es_version::{SequencerVersion, SEQUENCER_VERSION};
use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
};
use futures::FutureExt;
use hotshot_state_prover::service::{
    load_proving_key, one_honest_threshold, sync_state, StateProverConfig,
};
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme};
use portpicker::pick_unused_port;
use sequencer::{
    api::{
        options,
        test_helpers::{TestNetwork, STAKE_TABLE_CAPACITY_FOR_TEST},
    },
    persistence,
    state_signature::relay_server::run_relay_server,
    testing::TestConfig,
};
use sequencer_utils::{
    deployer::{deploy, Contract, Contracts},
    AnvilOptions,
};
use surf_disco::Client;
use tide_disco::error::ServerError;
use url::Url;

#[derive(Clone, Debug, Parser)]
struct Args {
    /// A JSON-RPC endpoint for the L1 to deploy to. If this is not provided, an Avil node will be
    /// launched automatically.
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    rpc_url: Option<Url>,
    /// Mnemonic for an L1 wallet.
    ///
    /// This wallet is used to deploy the contracts, so the account indicated by ACCOUNT_INDEX must
    /// be funded with with ETH.
    #[clap(
        long,
        name = "MNEMONIC",
        env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC",
        default_value = "test test test test test test test test test test test junk"
    )]
    mnemonic: String,
    /// Account index in the L1 wallet generated by MNEMONIC to use when deploying the contracts.
    #[clap(
        long,
        name = "ACCOUNT_INDEX",
        env = "ESPRESSO_DEPLOYER_ACCOUNT_INDEX",
        default_value = "0"
    )]
    account_index: u32,

    /// Port that the HTTP API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PORT")]
    sequencer_api_port: u16,

    /// Maximum concurrent connections allowed by the HTTP API server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_MAX_CONNECTIONS")]
    sequencer_api_max_connections: Option<usize>,

    /// Port for connecting to the builder.
    #[clap(short, long, env = "ESPRESSO_BUILDER_PORT")]
    builder_port: Option<u16>,

    #[clap(flatten)]
    sql: persistence::sql::Options,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let cli_params = Args::parse();
    let api_options = options::Options::from(options::Http {
        port: cli_params.sequencer_api_port,
        max_connections: cli_params.sequencer_api_max_connections,
    })
    .status(Default::default())
    .state(Default::default())
    .submit(Default::default())
    .query_sql(Default::default(), cli_params.sql);

    let (url, _anvil) = if let Some(url) = cli_params.rpc_url {
        (url, None)
    } else {
        tracing::warn!("L1 url is not provided. running an anvil node");
        let instance = AnvilOptions::default().spawn().await;
        let url = instance.url();
        tracing::info!("l1 url: {}", url);
        (url, Some(instance))
    };

    let relay_server_port = pick_unused_port().unwrap();
    let relay_server_url: Url = format!("http://localhost:{}", relay_server_port)
        .parse()
        .unwrap();

    let mut config = TestConfig::default_with_l1(url.clone());
    config.builder_port = cli_params.builder_port;
    config.state_relay_url = Some(relay_server_url.clone());

    let network = TestNetwork::new_with_config(
        api_options,
        [persistence::no_storage::Options; TestConfig::NUM_NODES],
        config,
    )
    .await;

    let config = network.cfg.hotshot_config();
    tracing::info!("Hotshot config {config:?}");

    let contracts = Contracts::new();

    tracing::info!("deploying the contracts");

    let light_client_genesis = network.light_client_genesis();

    let contracts = deploy(
        url.clone(),
        cli_params.mnemonic.clone(),
        cli_params.account_index,
        true,
        None,
        async { Ok(light_client_genesis) }.boxed(),
        contracts,
    )
    .await?;

    // Run the relay server
    let st = network.cfg.stake_table();
    let total_stake = st.total_stake(SnapshotVersion::LastEpochStart).unwrap();
    spawn(run_relay_server(
        None,
        one_honest_threshold(total_stake),
        format!("http://0.0.0.0:{relay_server_port}")
            .parse()
            .unwrap(),
        SEQUENCER_VERSION,
    ));

    // Run the prover service. These code are basically from `hotshot-state-prover`. The difference
    // is that here we don't need to fetch the `stake table` from other entities.
    // TODO: Remove the redundant code.
    let proving_key =
        spawn_blocking(move || Arc::new(load_proving_key(STAKE_TABLE_CAPACITY_FOR_TEST as usize)))
            .await;
    let relay_server_client =
        Client::<ServerError, SequencerVersion>::new(relay_server_url.clone());

    let provider = Provider::<Http>::try_from(url.to_string()).unwrap();
    let chain_id = provider.get_chainid().await.unwrap().as_u64();

    let update_interval = Duration::from_secs(20);
    let retry_interval = Duration::from_secs(2);
    let config = StateProverConfig {
        relay_server: relay_server_url,
        update_interval,
        retry_interval,
        l1_provider: url,
        light_client_address: contracts
            .get_contract_address(Contract::LightClientProxy)
            .unwrap(),
        eth_signing_key: MnemonicBuilder::<English>::default()
            .phrase(cli_params.mnemonic.as_str())
            .index(cli_params.account_index)
            .expect("error building wallet")
            .build()
            .expect("error opening wallet")
            .with_chain_id(chain_id)
            .signer()
            .clone(),
        sequencer_url: "http://localhost".parse().unwrap(), // This should not be used in dev-node
        port: None,
        stake_table_capacity: STAKE_TABLE_CAPACITY_FOR_TEST as usize,
    };

    loop {
        if let Err(err) = sync_state(&st, proving_key.clone(), &relay_server_client, &config).await
        {
            tracing::error!("Cannot sync the light client state, will retry: {}", err);
            sleep(retry_interval).await;
        } else {
            tracing::info!("Sleeping for {:?}", update_interval);
            sleep(update_interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{process::Child, sync::Arc, time::Duration};

    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::{stream::StreamExt, task::sleep};
    use committable::{Commitment, Committable};
    use contract_bindings::light_client::LightClient;
    use es_version::SequencerVersion;
    use escargot::CargoBuild;
    use espresso_types::{BlockMerkleTree, Header, SeqTypes, Transaction};
    use ethers::types::{Address, U256};
    use futures::TryStreamExt;
    use hotshot_query_service::{
        availability::{BlockQueryData, TransactionQueryData, VidCommonQueryData},
        data_source::sql::testing::TmpDb,
    };
    use jf_merkle_tree::MerkleTreeScheme;
    use portpicker::pick_unused_port;
    use sequencer::api::endpoints::NamespaceProofQueryData;
    use sequencer_utils::{init_signer, AnvilOptions};
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    pub struct BackgroundProcess(Child);

    impl Drop for BackgroundProcess {
        fn drop(&mut self) {
            self.0.kill().unwrap();
        }
    }

    // If this test failed and you are doing changes on the following stuff, please
    // sync your changes to [`espresso-sequencer-go`](https://github.com/EspressoSystems/espresso-sequencer-go)
    // and open a PR.
    // - APIs update
    // - Types (like `Header`) update
    #[async_std::test]
    async fn dev_node_test() {
        setup_logging();
        setup_backtrace();

        let builder_port = pick_unused_port().unwrap();

        let api_port = pick_unused_port().unwrap();

        let instance = AnvilOptions::default().spawn().await;
        let l1_url = instance.url();

        let db = TmpDb::init().await;
        let postgres_port = db.port();

        let process = CargoBuild::new()
            .bin("espresso-dev-node")
            .features("testing")
            .current_target()
            .run()
            .unwrap()
            .command()
            .env("ESPRESSO_SEQUENCER_L1_PROVIDER", l1_url.to_string())
            .env("ESPRESSO_BUILDER_PORT", builder_port.to_string())
            .env("ESPRESSO_SEQUENCER_API_PORT", api_port.to_string())
            .env("ESPRESSO_SEQUENCER_POSTGRES_HOST", "localhost")
            .env("ESPRESSO_SEQUENCER_ETH_MNEMONIC", TEST_MNEMONIC)
            .env("ESPRESSO_DEPLOYER_ACCOUNT_INDEX", "0")
            .env(
                "ESPRESSO_SEQUENCER_POSTGRES_PORT",
                postgres_port.to_string(),
            )
            .env("ESPRESSO_SEQUENCER_POSTGRES_USER", "postgres")
            .env("ESPRESSO_SEQUENCER_POSTGRES_PASSWORD", "password")
            .spawn()
            .unwrap();

        let _process = BackgroundProcess(process);

        let api_client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{api_port}").parse().unwrap());
        api_client.connect(None).await;

        tracing::info!("waiting for blocks");
        let _ = api_client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(5)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        let builder_api_client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{builder_port}").parse().unwrap());
        builder_api_client.connect(None).await;

        let builder_address = builder_api_client
            .get::<String>("block_info/builderaddress")
            .send()
            .await
            .unwrap();

        assert!(!builder_address.is_empty());

        let tx = Transaction::new(100_u32.into(), vec![1, 2, 3]);

        let hash: Commitment<Transaction> = api_client
            .post("submit/submit")
            .body_json(&tx)
            .unwrap()
            .send()
            .await
            .unwrap();

        let tx_hash = tx.commit();
        assert_eq!(hash, tx_hash);

        let mut tx_result = api_client
            .get::<TransactionQueryData<SeqTypes>>(&format!(
                "availability/transaction/hash/{tx_hash}",
            ))
            .send()
            .await;
        while tx_result.is_err() {
            sleep(Duration::from_secs(3)).await;

            tx_result = api_client
                .get::<TransactionQueryData<SeqTypes>>(&format!(
                    "availability/transaction/hash/{}",
                    tx_hash
                ))
                .send()
                .await;
        }

        let tx_block_height = tx_result.unwrap().block_height();

        let light_client_address = "0xdc64a140aa3e981100a9beca4e685f962f0cf6c9";

        let signer = init_signer(&l1_url, TEST_MNEMONIC, 0).await.unwrap();
        let light_client = LightClient::new(
            light_client_address.parse::<Address>().unwrap(),
            Arc::new(signer),
        );

        while light_client
            .get_hot_shot_commitment(U256::from(1))
            .call()
            .await
            .is_err()
        {
            tracing::info!("waiting for commitment");
            sleep(Duration::from_secs(3)).await;
        }

        // Check the namespace proof
        let proof = api_client
            .get::<NamespaceProofQueryData>(&format!(
                "availability/block/{tx_block_height}/namespace/100"
            ))
            .send()
            .await
            .unwrap();
        assert!(proof.proof.is_some());

        // These endpoints are currently used in `espresso-sequencer-go`. These checks
        // serve as reminders of syncing the API updates to go client repo when they change.
        {
            api_client
                .get::<u64>("status/block-height")
                .send()
                .await
                .unwrap();

            api_client
                .get::<Header>("availability/header/3")
                .send()
                .await
                .unwrap();

            api_client
                .get::<VidCommonQueryData<SeqTypes>>(&format!(
                    "availability/vid/common/{tx_block_height}"
                ))
                .send()
                .await
                .unwrap();

            while api_client
                .get::<<BlockMerkleTree as MerkleTreeScheme>::MembershipProof>("block-state/3/2")
                .send()
                .await
                .is_err()
            {
                sleep(Duration::from_secs(3)).await;
            }
        }

        drop(db);
    }
}
