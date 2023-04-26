use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::{sleep, spawn_local};
use contract_bindings::PolygonZkEVM;
use ethers::{prelude::*, providers::Middleware};
use futures::{
    future::{ready, FutureExt},
    join,
    stream::StreamExt,
};
use hermez_adaptor::{Layer1Backend, ZkEvmNode};
use hotshot_query_service::{availability::BlockQueryData, data_source::QueryData};
use sequencer::hotshot_commitment::{run_hotshot_commitment_task, HotShotContractOptions};
use sequencer::SeqTypes;
use sequencer_utils::{connect_rpc, wait_for_http};
use std::time::Duration;
use tempfile::TempDir;
use zkevm::ZkEvm;

#[async_std::test]
async fn test_end_to_end() {
    setup_logging();
    setup_backtrace();

    let node = ZkEvmNode::start("test-end-to-end".to_string(), Layer1Backend::Anvil).await;

    // Create blocks periodically. This seems to be required, but we should
    // investigate how exactly the Ethereum block number drivers the
    // zkevm-node.
    node.l1().mine_blocks_periodic(Duration::from_secs(1)).await;

    // Get test setup from environment.
    let env = node.env();
    let l1_provider = env.l1_provider();
    let l2_provider = env.l2_provider();
    let mnemonic = env.funded_mnemonic();
    let rollup_address = node.l1().rollup.address();
    let hotshot_address = node.l1().hotshot.address();

    let l1 = connect_rpc(&l1_provider, mnemonic, 0, None).await.unwrap();
    let l2 = connect_rpc(&l2_provider, mnemonic, 0, None).await.unwrap();
    let zkevm = ZkEvm {
        chain_id: l2.get_chainid().await.unwrap().as_u64(),
    };
    let rollup = PolygonZkEVM::new(rollup_address, l1.clone());
    let l1_initial_block = l1.get_block_number().await.unwrap();
    let l2_initial_balance = l2.get_balance(l2.inner().address(), None).await.unwrap();
    tracing::info!(
        "address: {}, rollup address: {}, hotshot address: {}, L1 initial block: {}, \
        L2 initial balance: {}",
        l1.inner().address(),
        rollup.address(),
        hotshot_address,
        l1_initial_block,
        l2_initial_balance,
    );

    // Start a sequencer network.
    let nodes = sequencer::testing::init_hotshot_handles().await;
    let api_node = nodes[0].clone();
    let sequencer_store = TempDir::new().unwrap();
    sequencer::api::serve(
        QueryData::create(sequencer_store.path(), ()).unwrap(),
        Box::new(move |_| ready((api_node, 0)).boxed()),
        env.sequencer_port(),
    )
    .await
    .unwrap();
    for node in nodes {
        node.start().await;
    }

    // Start a Hermez adaptor.
    let adaptor_opt = hermez_adaptor::Options {
        sequencer_url: env.sequencer(),
        rpc_port: env.l2_adaptor_rpc_port(),
        l2_chain_id: zkevm.chain_id,
        query_port: env.l2_adaptor_query_port(),
    };
    let hotshot_contract_opt = HotShotContractOptions {
        l1_provider: env.l1_provider(),
        sequencer_mnemonic: mnemonic.to_string(),
        sequencer_account_index: node.l1().clients.funded[0].index,
        hotshot_address,
        l1_chain_id: None,
        query_service_url: env.sequencer(),
    };
    spawn_local(async move {
        join!(
            hermez_adaptor::json_rpc::serve(&adaptor_opt),
            hermez_adaptor::query_service::serve(&adaptor_opt),
            run_hotshot_commitment_task(&hotshot_contract_opt)
        );
    });

    // Subscribe to a block stream so we can find the blocks that end up including our transactions.
    tracing::info!("connecting to sequencer at {}", env.sequencer());
    let sequencer = surf_disco::Client::<hotshot_query_service::Error>::new(env.sequencer());
    sequencer.connect(None).await;
    let mut blocks = sequencer
        .socket("availability/stream/blocks/0")
        .subscribe()
        .await
        .unwrap();

    // Wait for the adaptor to start serving.
    tracing::info!("connecting to adaptor RPC at {}", env.l2_adaptor_rpc());
    // The adaptor is not a full RPC, therefore we can't use `wait_for_rpc`.`
    wait_for_http(&env.l2_adaptor_rpc(), Duration::from_secs(1), 100)
        .await
        .unwrap();
    tracing::info!(
        "connecting to adaptor query service at {}",
        env.l2_adaptor_query()
    );
    wait_for_http(&env.l2_adaptor_query(), Duration::from_secs(1), 100)
        .await
        .unwrap();

    // Create a few test transactions.
    let transfer_amount = 1.into();
    let num_txns = 2u64;
    let mut txn_hashes = vec![];
    let mut block_nums = vec![];
    for i in 0..num_txns {
        let hash = l2
            .send_transaction(
                TransactionRequest {
                    from: Some(l2.inner().address()),
                    to: Some(Address::zero().into()),
                    value: Some(transfer_amount),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap()
            .tx_hash();
        tracing::info!("Transaction {}: {:?}", i, hash);

        // Wait for the transaction to be included in a block. We must ensure this transaction is
        // sequenced before the next one, or both could be invalidated due to nonce misordering.
        let block_num = 'block: loop {
            let block: BlockQueryData<SeqTypes> = blocks.next().await.unwrap().unwrap();
            tracing::info!("got block {:?}", block);
            for txn in block.block().vm_transactions(&zkevm) {
                let sequenced_hash = txn.hash();
                if sequenced_hash == hash {
                    tracing::info!("transaction {} sequenced", i);
                    break 'block block.height();
                } else {
                    tracing::info!("unknown transaction {} sequenced", sequenced_hash);
                }
            }
        };

        txn_hashes.push(hash);
        block_nums.push(block_num);
    }

    // Wait for the transactions to complete on L2. Note that awaiting a [PendingTransaction]
    // will not work here -- [PendingTransaction] returns [None] if the transaction is thrown
    // out of the mempool, but since we bypassed the sequencer, our transactions were never in
    // the mempool in the first place.
    for (i, hash) in txn_hashes.into_iter().enumerate() {
        loop {
            if let Some(receipt) = l2.get_transaction_receipt(hash).await.unwrap() {
                tracing::info!("transfer {} completed: {:?}", i, receipt);
                break;
            }
            tracing::info!("Waiting for transfer {} to complete", i);
            tracing::info!(
                "L2 balance {}/{}",
                l2.get_balance(l2.inner().address(), None).await.unwrap(),
                l2_initial_balance
            );
            sleep(Duration::from_secs(1)).await;
        }
    }

    // Check the effects of the transfers.
    assert_eq!(
        l2.get_balance(l2.inner().address(), None).await.unwrap(),
        l2_initial_balance - U256::from(num_txns) * transfer_amount
    );

    // Check that blocks have been sequenced on L1 up to at least the block that included our most
    // recent transaction. The inequality is strict because batch numbers on L1 are 1-indexed but
    // HotShot block numbers are 0-indexed.
    let last_block = *block_nums.last().unwrap();

    // Wait for the batches to be verified.
    let verified_filter = rollup
        .verify_batches_trusted_aggregator_filter()
        .from_block(l1_initial_block);
    let mut events = verified_filter.stream().await.unwrap();
    loop {
        let event = events.next().await.unwrap().unwrap();
        tracing::info!("batches verified up to {}/{}", event.num_batch, last_block);
        if event.num_batch > last_block {
            break;
        }
    }
}
