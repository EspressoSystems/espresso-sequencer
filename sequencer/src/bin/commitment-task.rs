use std::{io, time::Duration};

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::spawn;
use clap::Parser;
use es_version::SEQUENCER_VERSION;
use ethers::prelude::*;
use futures::FutureExt;
use sequencer::{
    hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions},
    options::parse_duration,
};
use tide_disco::{error::ServerError, Api};
use url::Url;
use vbs::version::StaticVersionType;

/// Commitment Task Command
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Address of the HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_HOTSHOT_ADDRESS")]
    pub hotshot_address: Address,

    /// Mnemonic phrase for the commitment task  wallet.
    ///
    /// This is the wallet that will be used to send commitments to the HotShot contract. It must be
    /// funded with ETH on the layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC")]
    pub eth_mnemonic: String,

    /// Index of a funded account derived from mnemonic, designating the account that will send
    /// commitments to the HotShot contract.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_HOTSHOT_ACCOUNT_INDEX",
        default_value = "0"
    )]
    pub hotshot_account_index: u32,

    /// If provided, the service will run a basic HTTP server on the given port.
    ///
    /// The server provides healthcheck and version endpoints.
    #[clap(short, long, env = "ESPRESSO_COMMITMENT_TASK_PORT")]
    pub port: Option<u16>,

    /// Client-side timeout for HTTP requests.
    #[clap(long, env = "ESPRESSO_COMMITMENT_TASK_REQUEST_TIMEOUT", value_parser = parse_duration, default_value = "5s")]
    pub request_timeout: Duration,

    /// If specified, sequencing attempts will be delayed by duration sampled from an exponential distribution with mean DELAY.
    #[clap(long, name = "DELAY", value_parser = parse_duration, env = "ESPRESSO_COMMITMENT_TASK_DELAY")]
    pub delay: Option<Duration>,
}
#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();

    if let Some(port) = opt.port {
        start_http_server(port, opt.hotshot_address, SEQUENCER_VERSION).unwrap();
    }

    let hotshot_contract_options = CommitmentTaskOptions {
        hotshot_address: opt.hotshot_address,
        l1_chain_id: None,
        l1_provider: opt.l1_provider.clone(),
        delay: opt.delay,
        sequencer_mnemonic: opt.eth_mnemonic,
        sequencer_account_index: opt.hotshot_account_index,
        request_timeout: opt.request_timeout,
        query_service_url: Some(opt.sequencer_url),
    };
    tracing::info!("Launching HotShot commitment task..");
    run_hotshot_commitment_task::<es_version::SequencerVersion>(&hotshot_contract_options).await;
}

fn start_http_server<Ver: StaticVersionType + 'static>(
    port: u16,
    hotshot_address: Address,
    bind_version: Ver,
) -> io::Result<()> {
    let mut app = tide_disco::App::<(), ServerError>::with_state(());
    let toml = toml::from_str::<toml::value::Value>(include_str!("../../api/commitment_task.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let mut api = Api::<(), ServerError, Ver>::new(toml)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    api.get("gethotshotcontract", move |_, _| {
        async move { Ok(hotshot_address) }.boxed()
    })
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    app.register_module("api", api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    spawn(app.serve(format!("0.0.0.0:{port}"), bind_version));
    Ok(())
}

#[cfg(test)]
mod test {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use es_version::{SequencerVersion, SEQUENCER_VERSION};
    use portpicker::pick_unused_port;
    use surf_disco::Client;

    use super::{start_http_server, Address, ServerError};

    #[async_std::test]
    async fn test_get_hotshot_contract() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let expected_addr = "0xED15E1FE0789c524398137a066ceb2EF9884E5D8"
            .parse::<Address>()
            .unwrap();
        start_http_server(port, expected_addr, SEQUENCER_VERSION)
            .expect("Failed to start the server");

        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        let addr: Address = client.get("api/hotshot_contract").send().await.unwrap();

        assert_eq!(addr, expected_addr);
    }
}
