use crate::prover::BatchProof;
use async_std::sync::RwLock;
use async_std::task::sleep;
use commit::Committable;
use contract_bindings::{
    example_rollup::{self, ExampleRollup},
    hot_shot::NewBlocksFilter,
    HotShot,
};
use ethers::prelude::*;
use hotshot_query_service::availability::BlockQueryData;
use std::sync::Arc;
use surf_disco::Url;

use sequencer::SeqTypes;

use sequencer_utils::{commitment_to_u256, connect_rpc, contract_send, u256_to_commitment};

use crate::state::State;

type HotShotClient = surf_disco::Client<hotshot_query_service::Error>;

#[derive(Clone, Debug)]
pub struct ExecutorOptions {
    pub sequencer_url: Url,
    pub l1_provider: Url,
    pub rollup_account_index: u32,
    pub rollup_mnemonic: String,
    pub hotshot_address: Address,
    pub rollup_address: Address,
}

/// Runs the executor service, which is responsible for:
/// 1) Fetching blocks of ordered transactions from HotShot and applying them to the Rollup State.
/// 2) Submitting mock proofs to the Rollup Contract.
pub async fn run_executor(opt: &ExecutorOptions, state: Arc<RwLock<State>>) {
    let ExecutorOptions {
        rollup_account_index,
        sequencer_url,
        l1_provider,
        hotshot_address,
        rollup_address,
        rollup_mnemonic,
    } = opt;

    let query_service_url = sequencer_url.join("availability").unwrap();
    let hotshot = HotShotClient::new(query_service_url.clone());
    hotshot.connect(None).await;

    // Connect to the layer one HotShot contract.
    let l1 = connect_rpc(l1_provider, rollup_mnemonic, *rollup_account_index, None)
        .await
        .expect("unable to connect to L1, hotshot commitment task exiting");

    // Create a socket connection to the L1 to subscribe to contract events
    // This assumes that the L1 node supports both HTTP and Websocket connections
    let mut ws_url = l1_provider.clone();
    ws_url.set_scheme("ws").unwrap();
    let socket_provider = Provider::<Ws>::connect(ws_url)
        .await
        .expect("Unable to make websocket connection to L1");

    let rollup_contract = ExampleRollup::new(*rollup_address, l1.clone());
    let hotshot_contract = HotShot::new(*hotshot_address, Arc::new(socket_provider));
    let filter = hotshot_contract.new_blocks_filter().from_block(0);
    let mut commits_stream = filter
        .subscribe()
        .await
        .expect("Unable to subscribe to L1 log stream");

    let mut blocks_stream = hotshot
        .socket("stream/blocks/0")
        .subscribe()
        .await
        .expect("Unable to subscribe to HotShot block stream");

    while let Some(event) = commits_stream.next().await {
        let (first_block, num_blocks) = match event {
            Ok(NewBlocksFilter {
                first_block_number,
                num_blocks,
            }) => (first_block_number, num_blocks.as_u64()),
            Err(err) => {
                tracing::error!("Error in HotShot block stream, retrying: {err}");
                continue;
            }
        };

        // When HotShot introduces optimistic DA, full block content may not be available immediately
        // so wait for all blocks to be ready before building the batch proof
        let blocks: Vec<BlockQueryData<SeqTypes>> = blocks_stream
            .by_ref()
            .take(num_blocks as usize)
            .map(|block| block.expect("Error fetching HotShot block"))
            .collect()
            .await;

        // Execute new blocks, generating proofs.
        let mut proofs = vec![];
        let mut state = state.write().await;
        tracing::info!(
            "executing blocks {}-{}, state is {}",
            first_block,
            first_block + num_blocks - 1,
            state.commit()
        );
        for i in 0..num_blocks {
            let commitment = hotshot_contract
                .commitments(first_block + i)
                .call()
                .await
                .expect("Unable to read commitment");
            let block_commitment =
                u256_to_commitment(commitment).expect("Unable to deserialize block commitment");
            let block = &blocks[i as usize];

            if block.block().commit() != block_commitment {
                panic!("Block commitment does not match hash of received block, the executor cannot continue");
            }

            proofs.push(state.execute_block(block).await);
        }

        // Compute an aggregate proof.
        let proof = BatchProof::generate(&proofs).expect("Error generating batch proof");
        let state_comm = commitment_to_u256(state.commit());

        // Send the batch proof to L1.
        tracing::info!(
            "Sending batch proof of blocks {}-{} to L1: {:?}",
            first_block,
            first_block + num_blocks - 1,
            proof,
        );
        let proof = example_rollup::BatchProof::from(proof);
        while contract_send(rollup_contract.verify_blocks(num_blocks, state_comm, proof.clone()))
            .await
            .is_none()
        {
            tracing::warn!("Failed to submit proof to contract, retrying");
            sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::transaction::{SignedTransaction, Transaction};
    use crate::utils::deploy_example_contracts;
    use crate::RollupVM;

    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::spawn;
    use ethers::providers::{Middleware, Provider};
    use ethers::signers::{LocalWallet, Signer};
    use futures::future::ready;
    use futures::FutureExt;
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::SequencerNode;
    use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
    use sequencer::testing::{init_hotshot_handles, wait_for_decide_on_handle};
    use sequencer::Vm;
    use sequencer_utils::{commitment_to_u256, AnvilOptions};
    use std::time::Duration;
    use surf_disco::{Client, Url};
    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";
    #[async_std::test]
    async fn test_execute() {
        // Create mock rollup state
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            alice.address(),
            9999,
        )])));
        let initial_state = { state.read().await.commit() };
        tracing::info!("initial state: {initial_state}");

        // Start a test HotShot and Rollup contract
        let anvil = AnvilOptions::default().spawn().await;
        let (hotshot_contract, rollup_contract, clients) =
            deploy_example_contracts(&anvil.url(), initial_state).await;

        // Setup a WS connection to the rollup contract and subscribe to state updates
        let mut ws_url = anvil.url();
        ws_url.set_scheme("ws").unwrap();
        let socket_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
        let state_update_filter = rollup_contract.state_update_filter().filter;
        let mut stream = socket_provider
            .subscribe_logs(&state_update_filter)
            .await
            .unwrap();

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        let init_handle = Box::new(move |_| (ready((api_node, 0)).boxed()));
        let SequencerNode { .. } = sequencer::api::serve(
            sequencer::api::Options {
                storage_path,
                port: sequencer_port,
                reset_store: true,
            },
            init_handle,
        )
        .await
        .unwrap();
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        // Submit transaction to sequencer
        let txn = Transaction {
            amount: 100,
            destination: bob.address(),
            nonce: 1,
        };
        let txn = SignedTransaction::new(txn, &alice).await;
        let txn = RollupVM.wrap(&txn);
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
        let hotshot_opt = CommitmentTaskOptions {
            l1_provider: anvil.url(),
            sequencer_mnemonic: TEST_MNEMONIC.to_string(),
            sequencer_account_index: clients.funded[0].index,
            hotshot_address: hotshot_contract.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone()),
        };

        let rollup_opt = ExecutorOptions {
            sequencer_url,
            rollup_account_index: clients.funded[1].index,
            l1_provider: anvil.url(),
            rollup_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: hotshot_contract.address(),
            rollup_address: rollup_contract.address(),
        };

        let state_lock = state.clone();
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });
        spawn(async move { run_executor(&rollup_opt, state_lock).await });

        // Wait for the rollup contract to process all state updates
        loop {
            // Wait for an event. This stream should not end until our events have been processed.
            stream.next().await.unwrap();
            let bob_balance = state.read().await.get_balance(&bob.address());
            if bob_balance == 100 {
                break;
            } else {
                tracing::info!("Bob's balance is {bob_balance}/100");
            }
        }

        // Ensure that the state commitments match.
        let state_comm = state.read().await.commit();
        let state_comm = commitment_to_u256(state_comm);
        let contract_state_comm = rollup_contract.state_commitment().call().await.unwrap();
        assert_eq!(state_comm, contract_state_comm);
    }

    #[async_std::test]
    async fn test_execute_batched_updates_to_slow_l1() {
        setup_logging();
        setup_backtrace();

        let num_txns = 10;

        // Create mock rollup state
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            alice.address(),
            9999,
        )])));
        let initial_state = { state.read().await.commit() };
        tracing::info!("initial state: {initial_state}");

        // Start a test HotShot and Rollup contract.
        let mut anvil = AnvilOptions::default().spawn().await;
        let (hotshot_contract, rollup_contract, clients) =
            deploy_example_contracts(&anvil.url(), initial_state).await;

        // Once the contracts have been deployed, restart the L1 with a slow block time.
        anvil
            .restart(AnvilOptions::default().block_time(Duration::from_secs(30)))
            .await;

        // Setup a WS connection to the rollup contract and subscribe to state updates
        let mut ws_url = anvil.url();
        ws_url.set_scheme("ws").unwrap();
        let socket_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
        let state_update_filter = rollup_contract.state_update_filter().filter;
        let mut stream = socket_provider
            .subscribe_logs(&state_update_filter)
            .await
            .unwrap();

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        let init_handle = Box::new(move |_| (ready((api_node, 0)).boxed()));
        let SequencerNode { .. } = sequencer::api::serve(
            sequencer::api::Options {
                storage_path,
                port: sequencer_port,
                reset_store: true,
            },
            init_handle,
        )
        .await
        .unwrap();
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        // Spawn hotshot commitment and executor tasks
        let hotshot_opt = CommitmentTaskOptions {
            l1_provider: anvil.url(),
            sequencer_mnemonic: TEST_MNEMONIC.to_string(),
            sequencer_account_index: clients.funded[0].index,
            hotshot_address: hotshot_contract.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone()),
        };

        let rollup_opt = ExecutorOptions {
            sequencer_url,
            l1_provider: anvil.url(),
            rollup_account_index: clients.funded[1].index,
            rollup_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: hotshot_contract.address(),
            rollup_address: rollup_contract.address(),
        };

        let state_lock = state.clone();
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });
        spawn(async move { run_executor(&rollup_opt, state_lock).await });

        // Submit transactions to sequencer
        for nonce in 1..=num_txns {
            let txn = Transaction {
                amount: 1,
                destination: bob.address(),
                nonce,
            };
            let txn = SignedTransaction::new(txn, &alice).await;
            let txn = RollupVM.wrap(&txn);
            nodes[0].submit_transaction(txn.clone()).await.unwrap();

            // Wait for the transaction to be sequenced, before we can sequence the next one.
            tracing::info!("Waiting for txn {nonce} to be sequenced");
            wait_for_decide_on_handle(nodes[0].clone(), txn)
                .await
                .unwrap();
        }

        // Wait for the rollup contract to process all state updates
        loop {
            // Wait for an event. This stream should not end until our events have been processed.
            stream.next().await.unwrap();
            let bob_balance = state.read().await.get_balance(&bob.address());
            if bob_balance == num_txns {
                break;
            } else {
                tracing::info!("Bob's balance is {bob_balance}/{num_txns}");
            }
        }

        // Ensure the state commitments match.
        let state_comm = state.read().await.commit();
        let state_comm = commitment_to_u256(state_comm);
        let contract_state_comm = rollup_contract.state_commitment().call().await.unwrap();
        assert_eq!(state_comm, contract_state_comm);
    }
}
