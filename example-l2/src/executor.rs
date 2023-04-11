use std::sync::Arc;

use async_std::sync::RwLock;
use contract_bindings::HotShot;
use hotshot_query_service::availability::BlockQueryData;
use sequencer::hotshot_commitment::HotShotContractOptions;
use sequencer::VmTransaction;
use sequencer::{hotshot_commitment::connect_l1, transaction::SequencerTransaction, SeqTypes};

use crate::api::VM_ID;
use crate::state::State;
use crate::transaction::SignedTransaction;

type HotShotClient = surf_disco::Client<hotshot_query_service::Error>;

async fn execute_block(block: &BlockQueryData<SeqTypes>, state: Arc<RwLock<State>>) {
    let transactions = block.block().transactions();
    for txn in transactions {
        if let SequencerTransaction::Wrapped(txn) = txn {
            if txn.vm() != VM_ID.into() {
                // Ignore transactions that do not apply to our Rollup
                continue;
            }
            let txn = SignedTransaction::decode(txn.payload());

            if let Some(txn) = txn {
                let res = state.write().await.apply_transaction(&txn);
                if let Err(err) = res {
                    // TODO: more informative logging
                    tracing::error!("Transaction invalid: {}", err)
                } else {
                    tracing::info!("Transaction applied")
                }
            } else {
                tracing::error!("Transaction encoding invalid")
            }
        }
    }
}

pub async fn execute(opt: &HotShotContractOptions, state: Arc<RwLock<State>>) {
    let query_service_url = opt.query_service_url.join("availability").unwrap();
    let hotshot = HotShotClient::new(query_service_url.clone());
    hotshot.connect(None).await;

    // Connect to the layer one HotShot contract.
    let Some(l1) = connect_l1(opt).await else {
        tracing::error!("unable to connect to L1, hotshot commitment task exiting");
        return;
    };
    let contract = HotShot::new(opt.hotshot_address, l1.clone());

    let mut block_height = 0;
    // TODO: improve polling method by waiting on contract event
    loop {
        let current_block_height = match contract.block_height().call().await {
            Ok(from) => from.as_u64(),
            Err(err) => {
                tracing::error!("Unable to read block_height from contract: {}", err);
                tracing::error!("Executor task will exit");
                return;
            }
        };
        // Get commitments
        for i in block_height..current_block_height {
            // TODO: verify commitment
            // let mut commit_bytes = [0; 32];
            // let commitment = contract.commitments(U256::from(i)).await.unwrap();
            // commitment.to_little_endian(&mut commit_bytes);
            let block = match hotshot
                .get::<BlockQueryData<SeqTypes>>(&format!("block/{}", i))
                .send()
                .await
            {
                Ok(block) => block,
                Err(err) => {
                    tracing::error!("Unable to query block from hotshot client: {}", err);
                    tracing::error!("Executor task will exit");
                    return;
                }
            };
            execute_block(&block, state.clone()).await;
        }
        block_height = current_block_height;
    }
}

#[cfg(test)]
mod test {
    use crate::transaction::Transaction;

    use super::*;
    use async_std::task::spawn;
    use contract_bindings::TestHermezContracts;
    use ethers::signers::{LocalWallet, Signer};
    use futures::future::ready;
    use futures::FutureExt;
    use hotshot_query_service::data_source::QueryData;
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::SequencerNode;
    use sequencer::hotshot_commitment::run_hotshot_commitment_task;
    use sequencer::transaction::Transaction as SequencerTransaction;
    use sequencer_utils::Anvil;
    use std::path::Path;
    use std::time::{Duration, Instant};
    use surf_disco::{Client, Url};
    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    #[async_std::test]
    async fn test_execute() {
        // Startup test contracts
        let anvil = Anvil::spawn(None).await;
        let l1 = TestHermezContracts::deploy(&anvil.url(), "http://dummy".to_string()).await;

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = sequencer::testing::init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path: &Path = &(tmp_dir.path().join("tmp_storage"));
        let init_handle = Box::new(move |_| (ready((api_node, 0)).boxed()));
        let query_data = QueryData::create(storage_path, ()).unwrap();
        let SequencerNode { .. } = sequencer::api::serve(query_data, init_handle, sequencer_port)
            .await
            .unwrap();
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        // Create mock rollup state
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            alice.address(),
            9999,
        )])));

        // Submit transaction to sequencer
        let txn = Transaction {
            amount: 100,
            destination: bob.address(),
            nonce: 1,
        };
        let txn = SignedTransaction::new(txn, &alice).await;
        let txn = SequencerTransaction::new(VM_ID.into(), txn.encode());
        let client: Client<ServerError> = Client::new(sequencer_url.clone());
        client.connect(None).await;
        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Spawn hotshot commitment and executor tasks
        let hotshot_opt = HotShotContractOptions {
            l1_provider: anvil.url(),
            sequencer_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: l1.hotshot.address(),
            l1_chain_id: None,
            query_service_url: sequencer_url,
        };
        let options = hotshot_opt.clone();
        let state_lock = state.clone();
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });
        spawn(async move { execute(&options, state_lock).await });

        // Loop until the Rollup executes our transaction
        let max_time = Duration::from_secs(75);
        let mut time_elapsed;
        let start_time = Instant::now();
        loop {
            time_elapsed = Instant::now() - start_time;
            if time_elapsed > max_time {
                panic!("Transaction not sequenced in time");
            }
            let bob_balance = state.read().await.get_balance(&bob.address());
            if bob_balance == 100 {
                break;
            }
        }
    }
}
