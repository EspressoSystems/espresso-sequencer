use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::sync::RwLock;
use clap::Parser;
use commit::Committable;
use ethers::signers::{LocalWallet, Signer};
use example_l2::{
    api::{serve, APIOptions},
    executor::{run_executor, ExecutorOptions},
    state::State,
    utils::deploy_example_contracts,
    Options,
};
use futures::join;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, HotShotContractOptions};
use std::sync::Arc;

const GENESIS_BALANCE: u64 = 9999;

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();

    let genesis = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
    let state = Arc::new(RwLock::new(State::from_initial_balances([(
        genesis.address(),
        GENESIS_BALANCE,
    )])));

    let api_options = APIOptions {
        sequencer_url: opt.sequencer_url.clone(),
        api_port: opt.api_port,
    };

    let executor_options = ExecutorOptions {
        rollup_account_index: 0, // TODO
        sequencer_url: opt.sequencer_url.clone(),
        l1_provider: opt.l1_provider.clone(),
        hotshot_address: opt.hotshot_address,
        rollup_address: opt.rollup_address,
        rollup_mnemonic: opt.rollup_mnemonic.clone(),
    };

    let hotshot_contract_options = HotShotContractOptions {
        sequencer_mnemonic: opt.rollup_mnemonic,
        sequencer_account_index: 0,
        l1_provider: opt.l1_provider.clone(),
        l1_chain_id: None,
        hotshot_address: opt.hotshot_address,
        query_service_url: opt.sequencer_url,
    };

    let serve_api = async {
        serve(&api_options, state.clone()).await.unwrap();
    };

    let initial_state = { state.read().await.commit() };

    tracing::info!("Deploying HotShot and Rollup contracts");
    deploy_example_contracts(&opt.l1_provider, initial_state).await;

    tracing::info!("Launching Example Rollup API, Executor, and HotShot commitment task..");
    join!(
        run_executor(&executor_options, state.clone()),
        run_hotshot_commitment_task(&hotshot_contract_options),
        serve_api,
    );
}
