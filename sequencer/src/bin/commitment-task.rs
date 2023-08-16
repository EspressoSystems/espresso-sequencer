use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use contract_bindings::{create_signer, HotShot};
use ethers::{prelude::*, providers::Provider};
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Optional address of a predeployed HotShot contract on layer 1. If unspecified, the task will deploy
    /// a HotShot instance before launching the commitment task.
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
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let hotshot_address;

    if let Some(address) = opt.hotshot_address {
        hotshot_address = address;
    } else {
        tracing::info!("HotShot contract address unspecified, deploying HotShot contract.");

        let provider = Provider::try_from(opt.l1_provider.to_string()).unwrap();
        let chain_id = provider
            .get_chainid()
            .await
            .expect("Error fetching L1 chain id")
            .as_u64();

        let signer = create_signer(
            opt.hotshot_account_index,
            &provider,
            chain_id,
            &opt.eth_mnemonic,
        );
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
