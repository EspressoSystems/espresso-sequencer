use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::spawn;
use clap::Parser;
use contract_bindings::hot_shot::HotShot;
use ethers::{prelude::*, providers::Provider, signers::coins_bip39::English};
use futures::FutureExt;
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
use std::io;
use std::sync::Arc;
use tide_disco::error::ServerError;
use tide_disco::Api;
use url::Url;

/// Commitment Task Command
///
/// There is an additional env var `ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG`
/// that is not handled by clap because it must be set via env var (and not via
/// CLI arguments).
///
/// Used testing with a pre-merge geth node that does not support the finalized
/// block tag.
///
/// Do not use in production.
///
/// When set to a truthy value ("y", "yes", "t", "true", "on", "1") the
/// commitment task will fetch "latest" block timestamps instead of
/// "finalized" ones.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Optional address of a predeployed HotShot contract on layer 1. If unspecified, the --deploy flag must be set
    /// to instruct the script to launch a new HotShot contract.
    #[clap(long, env = "ESPRESSO_SEQUENCER_HOTSHOT_ADDRESS")]
    pub hotshot_address: Option<Address>,

    /// Mnemonic phrase for the commitment task  wallet.
    ///
    /// This is the wallet that will be used to send commitments to the HotShot
    /// contract. It must be funded with ETH on the layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC")]
    pub eth_mnemonic: String,

    /// Index of a funded account derived from mnemonic, designating the
    /// account that will deploy the HotShot contract and/or send commitments to the HotShot contract.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_HOTSHOT_ACCOUNT_INDEX",
        default_value = "0"
    )]
    pub hotshot_account_index: u32,

    /// If true, the executable will attempt to deploy a HotShot contract instance if a HotShot address is not provided.
    #[clap(long)]
    pub deploy: bool,

    /// If provided, the service will run a basic HTTP server on the given port.
    ///
    /// The server provides healthcheck and version endpoints.
    #[clap(short, long, env = "ESPRESSO_COMMITMENT_TASK_PORT")]
    pub port: Option<u16>,
}
#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let hotshot_address;

    if let Some(address) = opt.hotshot_address {
        hotshot_address = address;
    } else if !opt.deploy {
        panic!("No HotShot address provided, use the --deploy flag if you would like the script to deploy a HotShot contract instance.")
    } else {
        tracing::info!(
            "No HotShot address provided and --deploy flag set, deploying HotShot contract."
        );

        let provider = Provider::try_from(opt.l1_provider.to_string()).unwrap();
        let chain_id = provider
            .get_chainid()
            .await
            .expect("Error fetching L1 chain id")
            .as_u64();

        let signer = Arc::new(SignerMiddleware::new(
            &provider,
            MnemonicBuilder::<English>::default()
                .phrase(&*opt.eth_mnemonic)
                .index(opt.hotshot_account_index)
                .unwrap()
                .build()
                .unwrap()
                .with_chain_id(chain_id),
        ));
        let hotshot = HotShot::deploy(signer, ())
            .expect("Error constructing deployer instance")
            .send()
            .await
            .expect("Error deploying HotShot contract");
        hotshot_address = hotshot.address();
        tracing::info!(
            "HotShot Contract launched at address: {:?}",
            hotshot_address
        );
    }

    if let Some(port) = opt.port {
        start_http_server(port, hotshot_address).unwrap();
    }

    let hotshot_contract_options = CommitmentTaskOptions {
        hotshot_address,
        l1_chain_id: None,
        l1_provider: opt.l1_provider.clone(),
        sequencer_mnemonic: opt.eth_mnemonic,
        sequencer_account_index: opt.hotshot_account_index,
        query_service_url: Some(opt.sequencer_url),
    };
    tracing::info!("Launching HotShot commitment task..");
    run_hotshot_commitment_task(&hotshot_contract_options).await;
}

fn start_http_server(port: u16, hotshot_address: Address) -> io::Result<()> {
    let mut app = tide_disco::App::<(), ServerError>::with_state(());
    let toml = toml::from_str::<toml::value::Value>(include_str!("../../api/commitment_task.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let mut api = Api::<(), ServerError>::new(toml)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    api.get("gethotshotcontract", move |_, _| {
        async move { Ok(hotshot_address) }.boxed()
    })
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    app.register_module("api", api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    spawn(app.serve(format!("0.0.0.0:{port}")));
    Ok(())
}

#[cfg(test)]
mod test {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use portpicker::pick_unused_port;
    use surf_disco::Client;

    use super::start_http_server;
    use super::Address;
    use super::ServerError;

    #[async_std::test]
    async fn test_get_hotshot_contract() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let expected_addr = "0xED15E1FE0789c524398137a066ceb2EF9884E5D8"
            .parse::<Address>()
            .unwrap();
        start_http_server(port, expected_addr).expect("Failed to start the server");

        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        let addr: Address = client.get("api/hotshot_contract").send().await.unwrap();

        assert_eq!(addr, expected_addr);
    }
}
