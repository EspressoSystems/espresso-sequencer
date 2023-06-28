use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::sync::RwLock;
use clap::Parser;
use commit::Committable;
use ethers::signers::{LocalWallet, Signer};
use example_l2::{
    api::{serve, APIOptions},
    executor::{run_executor, ExecutorOptions},
    seed::{SeedIdentity, INITIAL_BALANCE},
    state::State,
    utils::{deploy_example_contract, deploy_hotshot_contract},
    Options, RollupVM,
};
use futures::join;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
use std::sync::Arc;
use strum::IntoEnumIterator;

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let vm = RollupVM::new(1.into());

    let mut initial_balances = vec![];
    for identity in SeedIdentity::iter() {
        let address = LocalWallet::new(&mut ChaChaRng::seed_from_u64(identity as u64)).address();
        initial_balances.push((address, INITIAL_BALANCE))
    }
    let state = Arc::new(RwLock::new(State::from_initial_balances(
        initial_balances,
        vm,
    )));

    let api_options = APIOptions {
        api_port: opt.api_port,
        sequencer_url: opt.sequencer_url.clone(),
    };

    let executor_options = ExecutorOptions {
        hotshot_address: opt.hotshot_address,
        l1_provider: opt.l1_provider.clone(),
        rollup_account_index: opt.rollup_account_index,
        rollup_address: opt.rollup_address,
        rollup_mnemonic: opt.rollup_mnemonic.clone(),
        sequencer_url: opt.sequencer_url.clone(),
    };

    let hotshot_contract_options = CommitmentTaskOptions {
        hotshot_address: opt.hotshot_address,
        l1_chain_id: None,
        l1_provider: opt.l1_provider.clone(),
        sequencer_mnemonic: opt.rollup_mnemonic,
        sequencer_account_index: opt.hotshot_account_index,
        query_service_url: Some(opt.sequencer_url),
    };

    let serve_api = async {
        serve(&api_options, state.clone()).await.unwrap();
    };

    let initial_state = { state.read().await.commit() };

    tracing::info!("Deploying HotShot and Rollup contracts");
    let (hotshot_contract, _) = deploy_hotshot_contract(&opt.l1_provider).await;
    deploy_example_contract(&opt.l1_provider, initial_state, &hotshot_contract).await;

    tracing::info!("Launching Example Rollup API, Executor, and HotShot commitment task..");
    join!(
        run_executor(&executor_options, state.clone()),
        run_hotshot_commitment_task(&hotshot_contract_options),
        serve_api,
    );
}
