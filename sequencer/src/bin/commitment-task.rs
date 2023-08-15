use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use contract_bindings::TestL1System;
use ethers::{prelude::*, providers::Provider};
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
use std::time::Duration;
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_DEMO_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Address of HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_HOTSHOT_ADDRESS")]
    pub hotshot_address: Address,

    /// Mnemonic phrase for the commitment task  wallet.
    ///
    /// This is the wallet that will be used to send commitments to the HotShot
    /// contract. It must be funded with ETH on the layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_ROLLUP_MNEMONIC")]
    pub rollup_mnemonic: String,

    /// Index of a funded account derived from mnemonic, designating the
    /// account that will send commitments to the HotShot contract
    #[clap(long, env = "ESPRESSO_DEMO_HOTSHOT_ACCOUNT_INDEX", default_value = "0")]
    pub hotshot_account_index: u32,

    /// If true, the executable will attempt to deploy a HotShot contract instance.
    #[clap(long)]
    pub deploy: bool,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let hotshot_contract_options = CommitmentTaskOptions {
        hotshot_address: opt.hotshot_address,
        l1_chain_id: None,
        l1_provider: opt.l1_provider.clone(),
        sequencer_mnemonic: opt.rollup_mnemonic,
        sequencer_account_index: opt.hotshot_account_index,
        query_service_url: Some(opt.sequencer_url),
    };

    if opt.deploy {
        tracing::info!("Deploying HotShot contract");
        let mut provider = Provider::try_from(opt.l1_provider.to_string()).unwrap();
        provider.set_interval(Duration::from_millis(10));
        let test_system = TestL1System::deploy(provider).await.unwrap();
        tracing::info!(
            "HotShot Contract launched at address: {:?}",
            test_system.hotshot.address()
        );
    }

    tracing::info!("Launching HotShot commitment task..");
    run_hotshot_commitment_task(&hotshot_contract_options).await;
}
