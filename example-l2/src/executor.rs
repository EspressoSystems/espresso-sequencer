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
use hotshot_query_service::availability::BlockHeaderQueryData;
use sequencer::{api::NamespaceProofQueryData, Block, Vm};
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
    let filter = hotshot_contract
        .new_blocks_filter()
        .from_block(0)
        // Ethers does not set the contract address on filters created via contract bindings. This
        // seems like a bug and I have reported it: https://github.com/gakonst/ethers-rs/issues/2528.
        // In the mean time we can work around by setting the address manually.
        .address(hotshot_contract.address().into());
    let mut commits_stream = filter
        .subscribe()
        .await
        .expect("Unable to subscribe to L1 log stream");

    let mut block_header_stream = hotshot
        .socket("stream/block/headers/0")
        .subscribe()
        .await
        .expect("Unable to subscribe to HotShot block header stream");

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
        let headers: Vec<BlockHeaderQueryData<SeqTypes>> = block_header_stream
            .by_ref()
            .take(num_blocks as usize)
            .map(|result| result.expect("Error fetching block header"))
            .collect()
            .await;

        // Execute new blocks, generating proofs.
        let mut proofs = vec![];
        tracing::info!(
            "executing blocks {}-{}, state is {}",
            first_block,
            first_block + num_blocks - 1,
            state.read().await.commit()
        );
        for i in 0..num_blocks {
            let commitment = hotshot_contract
                .commitments(first_block + i)
                .call()
                .await
                .expect("Unable to read commitment");
            let block_commitment =
                u256_to_commitment(commitment).expect("Unable to deserialize block commitment");

            if headers[i as usize].hash() != block_commitment {
                panic!("Block commitment does not match hash of received block, the executor cannot continue");
            }

            let vm_id: u64 = state.read().await.vm.id().into();

            let namespace_proof_query: NamespaceProofQueryData = hotshot
                .get(&format!(
                    "block/{}/namespace/{}",
                    first_block.as_u64() + i,
                    vm_id
                ))
                .send()
                .await
                .unwrap();
            let nmt_root = namespace_proof_query.nmt_root().clone();
            let namespace_proof = namespace_proof_query.proof().clone();

            // Check that the NMT root is consistent with the HotShot block committment
            let derived_block_comm = Block::commitment_from_opening(&nmt_root);

            assert_eq!(derived_block_comm, block_commitment);

            proofs.push(
                state
                    .write()
                    .await
                    .execute_block(nmt_root, namespace_proof)
                    .await,
            );
        }

        // Compute an aggregate proof.
        let proof = BatchProof::generate(&proofs).expect("Error generating batch proof");
        let state_comm = commitment_to_u256(state.read().await.commit());

        // Send the batch proof to L1.
        tracing::info!(
            "Sending batch proof of blocks {}-{} to L1: {:?}",
            first_block,
            first_block + num_blocks - 1,
            proof,
        );
        let proof = example_rollup::BatchProof::from(proof);
        let call = rollup_contract.verify_blocks(num_blocks, state_comm, proof);
        while contract_send(&call).await.is_none() {
            tracing::warn!("Failed to submit proof to contract, retrying");
            sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::state::{Amount, Nonce};
    use crate::transaction::{SignedTransaction, Transaction};
    use crate::utils::{create_provider, deploy_example_contract, ExampleRollupContract};
    use crate::RollupVM;

    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::spawn;
    use contract_bindings::TestL1System;
    use ethers::prelude::k256::ecdsa::SigningKey;
    use ethers::providers::{Middleware, Provider};
    use ethers::signers::{LocalWallet, Signer};
    use futures::future::ready;
    use futures::FutureExt;
    use hotshot::{traits::NodeImplementation, types::HotShotHandle};
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::{HttpOptions, QueryOptions};
    use sequencer::hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions};
    use sequencer::testing::{init_hotshot_handles, wait_for_decide_on_handle};
    use sequencer::{Leaf, Vm, VmId};
    use sequencer_utils::{commitment_to_u256, AnvilOptions};
    use std::path::PathBuf;
    use std::time::Duration;
    use surf_disco::{Client, Url};
    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    #[derive(Clone, Debug)]
    struct TestRollupInstance {
        contract: ExampleRollupContract,
        vm: RollupVM,
        socket_provider: Provider<Ws>,
        l1_url: Url,
        alice: Wallet<SigningKey>,
        state: Arc<RwLock<State>>,
        bob: Wallet<SigningKey>,
    }

    impl TestRollupInstance {
        pub async fn launch(
            l1_url: Url,
            vm_id: VmId,
            alice: Wallet<SigningKey>,
            bob: Wallet<SigningKey>,
            test_l1: &TestL1System,
        ) -> Self {
            // Create mock rollup state
            let vm = RollupVM::new(vm_id);
            let state = State::from_initial_balances([(alice.address(), 9999)], vm);
            let initial_state = state.commit();
            let state = Arc::new(RwLock::new(state));
            tracing::info!("initial state: {initial_state}");
            let mut ws_url = l1_url.clone();
            ws_url.set_scheme("ws").unwrap();
            let socket_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            let rollup_contract = deploy_example_contract(test_l1, initial_state).await;

            Self {
                contract: rollup_contract,
                vm,
                socket_provider,
                alice,
                l1_url,
                bob,
                state,
            }
        }

        pub async fn reset_socket_connnection(&mut self) {
            let mut ws_url = self.l1_url.clone();
            ws_url.set_scheme("ws").unwrap();
            // Occasionally the connection fails, so we retry a few times.
            for _ in 0..10 {
                match Provider::<Ws>::connect(ws_url.clone()).await {
                    Ok(provider) => {
                        self.socket_provider = provider;
                        return;
                    }
                    Err(_) => {
                        tracing::warn!("Failed to connect to websocket, retrying");
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            panic!("Failed to connect to websocket server: {:?}", ws_url);
        }

        pub async fn subscribe(&self) -> SubscriptionStream<'_, Ws, Log> {
            let state_update_filter = self
                .contract
                .state_update_filter()
                .filter
                // Ethers does not set the contract address on filters created via contract
                // bindings. This seems like a bug and I have reported it:
                // https://github.com/gakonst/ethers-rs/issues/2528. In the mean time we can work
                // around by setting the address manually.
                .address(self.contract.address());
            self.socket_provider
                .subscribe_logs(&state_update_filter)
                .await
                .unwrap()
        }

        pub async fn test_transaction(
            &self,
            amount: Amount,
            nonce: Nonce,
        ) -> sequencer::Transaction {
            let txn = Transaction {
                amount,
                destination: self.bob.address(),
                nonce,
            };
            let txn = SignedTransaction::new(txn, &self.alice).await;
            self.vm.wrap(&txn)
        }

        pub async fn get_bob_balance(&self) -> u64 {
            self.state.read().await.get_balance(&self.bob.address())
        }
    }

    async fn start_query_service<I: NodeImplementation<SeqTypes, Leaf = Leaf>>(
        port: u16,
        storage_path: PathBuf,
        node: HotShotHandle<SeqTypes, I>,
    ) {
        let init_handle = Box::new(move |_| (ready((node, 0)).boxed()));
        sequencer::api::Options::from(HttpOptions { port })
            .submit(Default::default())
            .query(QueryOptions {
                storage_path,
                reset_store: true,
            })
            .serve(init_handle)
            .await
            .unwrap();
    }

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";
    #[async_std::test]
    async fn test_execute() {
        let anvil = AnvilOptions::default().spawn().await;
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));

        // Deploy hotshot contract
        let provider = create_provider(&anvil.url());
        let test_l1 = TestL1System::deploy(provider).await.unwrap();

        // Start a test Rollup instance
        let test_rollup =
            TestRollupInstance::launch(anvil.url().clone(), 1.into(), alice, bob, &test_l1).await;

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        start_query_service(sequencer_port, storage_path, api_node).await;
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        // Submit transaction to sequencer
        let client: Client<ServerError> = Client::new(sequencer_url.clone());
        let txn = test_rollup.test_transaction(100, 1).await;
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
            sequencer_account_index: test_l1.clients.funded[0].index,
            hotshot_address: test_l1.hotshot.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone()),
        };

        let rollup_opt = ExecutorOptions {
            sequencer_url,
            rollup_account_index: test_l1.clients.funded[1].index,
            l1_provider: anvil.url(),
            rollup_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: test_l1.hotshot.address(),
            rollup_address: test_rollup.contract.address(),
        };

        let state_lock = test_rollup.state.clone();
        let mut stream = test_rollup.subscribe().await;
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });
        spawn(async move { run_executor(&rollup_opt, state_lock).await });

        // Wait for the rollup contract to process all state updates
        loop {
            // Wait for an event. This stream should not end until our events have been processed.
            let bob_balance = test_rollup.get_bob_balance().await;
            stream.next().await.unwrap();
            if bob_balance == 100 {
                break;
            } else {
                tracing::info!("Bob's balance is {bob_balance}/100");
            }
        }

        // Ensure that the state commitments match.
        let state_comm = test_rollup.state.read().await.commit();
        let state_comm = commitment_to_u256(state_comm);
        let contract_state_comm = test_rollup
            .contract
            .state_commitment()
            .call()
            .await
            .unwrap();
        assert_eq!(state_comm, contract_state_comm);
    }

    #[async_std::test]
    async fn test_execute_multi_rollup() {
        let anvil = AnvilOptions::default().spawn().await;
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));
        // Deploy hotshot contract
        let provider = create_provider(&anvil.url());
        let test_l1 = TestL1System::deploy(provider).await.unwrap();

        // Start test Rollup instances
        let num_rollups = 3;
        let mut test_rollups = Vec::new();
        for i in 1..num_rollups + 1 {
            // To keep nonces consistent for the underlying provider, we must await these iteratively
            let test_rollup = TestRollupInstance::launch(
                anvil.url().clone(),
                i.into(),
                alice.clone(),
                bob.clone(),
                &test_l1,
            )
            .await;
            test_rollups.push(test_rollup);
        }

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        start_query_service(sequencer_port, storage_path, api_node).await;
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        // Submit transactions to sequencer
        let client: Client<ServerError> = Client::new(sequencer_url.clone());
        for i in 0..num_rollups {
            let txn = test_rollups[i as usize].test_transaction(100, 1).await;
            client.connect(None).await;
            client
                .post::<()>("submit/submit")
                .body_json(&txn)
                .unwrap()
                .send()
                .await
                .unwrap();
        }

        // Spawn hotshot commitment task
        let hotshot_opt = CommitmentTaskOptions {
            l1_provider: anvil.url(),
            sequencer_mnemonic: TEST_MNEMONIC.to_string(),
            sequencer_account_index: test_l1.clients.funded[0].index,
            hotshot_address: test_l1.hotshot.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone()),
        };
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });

        // Spawn all rollup executors
        for i in 0..num_rollups {
            let test_rollup = &test_rollups[i as usize];
            let state_lock = test_rollup.state.clone();
            let rollup_opt = ExecutorOptions {
                sequencer_url: sequencer_url.clone(),
                rollup_account_index: test_l1.clients.funded[1].index,
                l1_provider: anvil.url(),
                rollup_mnemonic: TEST_MNEMONIC.to_string(),
                hotshot_address: test_l1.hotshot.address(),
                rollup_address: test_rollup.contract.address(),
            };
            spawn(async move { run_executor(&rollup_opt, state_lock).await });
        }

        // Wait for all rollup contracts to process state updates
        for i in 0..num_rollups {
            let test_rollup = &test_rollups[i as usize];
            let mut stream = test_rollup.subscribe().await;
            loop {
                // Wait for an event. This stream should not end until our events have been processed.
                let bob_balance = test_rollup.get_bob_balance().await;
                stream.next().await.unwrap();
                if bob_balance == 100 {
                    break;
                } else {
                    tracing::info!("Bob's balance is {bob_balance}/100");
                }
            }
        }

        // Ensure that the state commitments match each rollup contract's state commitment
        for i in 0..num_rollups {
            let test_rollup = &test_rollups[i as usize];
            let state_comm = test_rollup.state.read().await.commit();
            let state_comm = commitment_to_u256(state_comm);
            let contract_state_comm = test_rollup
                .contract
                .state_commitment()
                .call()
                .await
                .unwrap();
            assert_eq!(state_comm, contract_state_comm);
        }
    }

    #[async_std::test]
    async fn test_execute_batched_updates_to_slow_l1() {
        setup_logging();
        setup_backtrace();

        let num_txns = 10;

        // Create mock rollup state
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));

        // Start a test HotShot and Rollup contract.
        let mut anvil = AnvilOptions::default().spawn().await;
        let provider = create_provider(&anvil.url());
        let test_l1 = TestL1System::deploy(provider).await.unwrap();
        let mut test_rollup =
            TestRollupInstance::launch(anvil.url().clone(), 1.into(), alice, bob, &test_l1).await;

        // Once the contracts have been deployed, restart the L1 with a slow block time.
        anvil
            .restart(AnvilOptions::default().block_time(Duration::from_secs(5)))
            .await;

        test_rollup.reset_socket_connnection().await;
        let mut stream = test_rollup.subscribe().await;

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        start_query_service(sequencer_port, storage_path, api_node).await;
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
            sequencer_account_index: test_l1.clients.funded[0].index,
            hotshot_address: test_l1.hotshot.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone()),
        };

        let rollup_opt = ExecutorOptions {
            sequencer_url,
            l1_provider: anvil.url(),
            rollup_account_index: test_l1.clients.funded[1].index,
            rollup_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: test_l1.hotshot.address(),
            rollup_address: test_rollup.contract.address(),
        };

        let state_lock = test_rollup.state.clone();
        spawn(async move { run_hotshot_commitment_task(&hotshot_opt).await });
        spawn(async move { run_executor(&rollup_opt, state_lock).await });

        // Submit transactions to sequencer
        for nonce in 1..=num_txns {
            let txn = test_rollup.test_transaction(1, nonce).await;
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
            let bob_balance = test_rollup.get_bob_balance().await;
            if bob_balance == num_txns {
                break;
            } else {
                tracing::info!("Bob's balance is {bob_balance}/{num_txns}");
            }
        }

        // Ensure the state commitments match.
        let state_comm = test_rollup.state.read().await.commit();
        let state_comm = commitment_to_u256(state_comm);
        let contract_state_comm = test_rollup
            .contract
            .state_commitment()
            .call()
            .await
            .unwrap();
        assert_eq!(state_comm, contract_state_comm);
    }
}
