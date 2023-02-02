//! Permissionless Sequencer Example
//!
//! This example demonstrates the functionality of permissionless sequencers in the Hermez zkEVM
//! system. It connects to a Hermez deployment, builds a transaction for the layer 2, submits this
//! transaction directly to the layer 1 as a forced batch, and then checks to make sure the
//! transaction executes and a proof is generated.
//!
//! # Usage
//!
//! Run a local Hermez deployment. The easiest way to do this it to run the
//! [docker-compose test](https://github.com/EspressoSystems/zkevm-node/blob/hotshot-integration/docs/running_local.md)
//! from the Espresso fork of the zkEVM repo.
//!
//! If connecting to the zkEVM docker-compose demo, the default configuration of this program works,
//! and you can simply run
//!
//! ```text
//! cargo run --release --example permissionless-sequencer
//! ```
//!
//! If connecting to a different zkEVM deployment, you will need to specify how to access the
//! deployment. You can use `cargo run --release --example permissionless-sequencer -- --help` to
//! see the configuration parameters required on the command line.
//!
//! When the demo runs, it will log at INFO level what it is doing, so you can follow along and see
//! the various steps and EVM transactions which must be executed to act as a permissionless
//! sequencer. If anything goes wrong, the demo will panic. If it completes successfully, that means
//! permissionless sequencing is working.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{sync::Arc, task::sleep};
use clap::Parser;
use contract_bindings::{
    polygon_zk_evm::{ForceBatchFilter, ForcedBatchData, SequenceForceBatchesFilter},
    Matic, PolygonZkEVM,
};
use ethers::{
    abi::RawLog,
    contract::EthEvent,
    prelude::*,
    signers::coins_bip39::English,
    utils::{format_ether, format_units},
};
use std::time::Duration;
use url::Url;
use zkevm::{hermez::encode_transactions, EvmTransaction};

/// Amount in WEI to transfer for the transfer demo.
const TRANSFER_AMOUNT: U256 = U256([1, 0, 0, 0]);

#[derive(Parser)]
struct Options {
    /// JSON-RPC endpoint for layer 1 (Ethereum).
    #[clap(
        long,
        env = "ESPRESSO_ZKEVM_L1_PROVIDER",
        default_value = "http://localhost:8545"
    )]
    l1_provider: Url,

    /// JSON-RPC endpoint for layer 2 (Hermez).
    #[clap(
        long,
        env = "ESPRESSO_ZKEVM_L2_PROVIDER",
        default_value = "http://localhost:8126"
    )]
    l2_provider: Url,

    /// Address of Hermez rollup contract on layer 1.
    #[clap(
        long,
        env = "ESPRESSO_ZKEVM_ROLLUP_ADDRESS",
        default_value = "0x2279B7A0a67DB372996a5FaB50D91eAA73d2eBe6"
    )]
    rollup_address: Address,

    /// Address of Matic token contract on layer 1.
    #[clap(
        long,
        env = "ESPRESSO_ZKEVM_MATIC_ADDRESS",
        default_value = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
    )]
    matic_address: Address,

    /// Mnemonic phrase for the deployer of the rollup contract.
    ///
    /// The first account in this wallet must be the admin of the rollup contract, and must also be
    /// funded with ETH on layers 1 and 2 and MATIC on layer 1.
    #[clap(
        long,
        env = "ESPRESSO_ZKEVM_DEPLOYER_MNEMONIC",
        default_value = "test test test test test test test test test test test junk"
    )]
    deployer_mnemonic: String,
}

async fn force_batch<M: Middleware>(
    rollup: &PolygonZkEVM<M>,
    transactions: Bytes,
    matic_decimals: u32,
) -> u64 {
    // Get the fee we are required to pay.
    let fee = rollup.batch_fee().call().await.unwrap();
    tracing::info!(
        "forceBatch fee is {} MATIC",
        format_units(fee, matic_decimals).unwrap(),
    );

    // Send the transactions to the contract as a forced batch.
    let receipt = await_tx(
        "forcing batch",
        rollup
            .force_batch(transactions.clone(), fee)
            .send()
            .await
            .unwrap(),
    )
    .await;

    // Parse the ForceBatch event from the logs, which contains information we need to get the batch
    // sequenced, like the global exit root.
    let force_batch_event = receipt
        .logs
        .into_iter()
        .find_map(|log| {
            <ForceBatchFilter as EthEvent>::decode_log(&RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            })
            .ok()
        })
        .unwrap();

    // The forced batch was recorded with the timestamp of the block where the transaction executed.
    // In order to specify this batch for sequencing, we need the same timestamp.
    let block = rollup
        .client()
        .get_block(receipt.block_number.unwrap())
        .await
        .unwrap()
        .unwrap();

    // Specification of the batch we just forced, which we can use to sequence it.
    let forced_batch = ForcedBatchData {
        transactions,
        global_exit_root: force_batch_event.last_global_exit_root,
        min_forced_timestamp: block.timestamp.try_into().unwrap(),
    };
    tracing::info!("batch forced: {:?}", forced_batch);

    // Sequence the batch.
    let receipt = await_tx(
        "sequencing forced batch",
        rollup
            .sequence_force_batches(vec![forced_batch])
            .send()
            .await
            .unwrap(),
    )
    .await;
    tracing::info!("batch sequenced: {:?}", receipt);

    // Parse the logs which give us the batch number of the sequenced batch.
    let event = receipt
        .logs
        .into_iter()
        .find_map(|log| {
            <SequenceForceBatchesFilter as EthEvent>::decode_log(&RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            })
            .ok()
        })
        .unwrap();
    event.num_batch
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let l1 = Provider::<Http>::try_from(opt.l1_provider.to_string()).unwrap();
    let l2 = Provider::<Http>::try_from(opt.l2_provider.to_string()).unwrap();

    let l1_chain_id = l1.get_chainid().await.unwrap();
    let l2_chain_id = l2.get_chainid().await.unwrap();
    tracing::info!(
        "L1 chainid is {}, L2 chainid is {}",
        l1_chain_id,
        l2_chain_id
    );

    let l1_wallet = MnemonicBuilder::<English>::default()
        .phrase(opt.deployer_mnemonic.as_str())
        .build()
        .unwrap()
        .with_chain_id(u64::try_from(l1_chain_id).unwrap());
    let l2_wallet = MnemonicBuilder::<English>::default()
        .phrase(opt.deployer_mnemonic.as_str())
        .build()
        .unwrap()
        .with_chain_id(u64::try_from(l2_chain_id).unwrap());
    let l1_client = Arc::new(SignerMiddleware::new(l1, l1_wallet.clone()));
    let l2_client = Arc::new(SignerMiddleware::new(l2, l2_wallet.clone()));

    let rollup = PolygonZkEVM::new(opt.rollup_address, l1_client.clone());
    let matic = Matic::new(opt.matic_address, l1_client.clone());
    let matic_decimals = matic.decimals().call().await.unwrap();
    tracing::info!("Found MATIC token, {} decimals", matic_decimals);

    // Configure the rollup to allow permissionless sequencers.
    if !rollup.force_batch_allowed().call().await.unwrap() {
        await_tx(
            "enable forced batches",
            rollup.set_force_batch_allowed(true).send().await.unwrap(),
        )
        .await;
    }

    // Approve rollup to spend our MATIC.
    matic
        .approve(rollup.address(), U256::MAX)
        .send()
        .await
        .unwrap();

    // Get the initial block number. We will use this later when we search for logs that might
    // contain an event corresponding to the transaction we are going to submit.
    let l1_initial_block = l1_client.get_block_number().await.unwrap();
    tracing::info!("L1 is at block {}", l1_initial_block);

    // Build (but do not submit) a transaction for L2. We will transfer some ETH from `l2_client` to
    // a random address.
    let l2_initial_balance = l2_client
        .get_balance(l2_client.address(), None)
        .await
        .unwrap();
    tracing::info!("L2 account has {} ETH", format_ether(l2_initial_balance));
    assert!(l2_initial_balance >= TRANSFER_AMOUNT);
    let mut transfer = TransactionRequest {
        from: Some(l2_client.address()),
        to: Some(Address::random().into()),
        value: Some(TRANSFER_AMOUNT),
        ..Default::default()
    }
    .into();
    l2_client
        .fill_transaction(&mut transfer, None)
        .await
        .unwrap();
    let signature = l2_client
        .signer()
        .sign_transaction(&transfer)
        .await
        .unwrap();
    let transfer = EvmTransaction::new(transfer, signature);
    let transfer_bytes = encode_transactions([&transfer]);
    let transfer_hash = transfer.hash();
    tracing::info!("L2 transfer: {:?}", transfer);
    tracing::info!("Encoded: {}", transfer_bytes);
    tracing::info!("Hash: {:#x}", transfer_hash);

    // Force the transaction into a batch by sending it directly to the L1 rollup contract, as a
    // permissionless sequencer would.
    let forced_batch_num = force_batch(&rollup, transfer_bytes, matic_decimals as u32).await;

    // Wait for the transaction to complete on L2. Note that awaiting a [PendingTransaction] will
    // not work here -- [PendingTransaction] returns [None] if the transaction is thrown out of the
    // mempool, but since we bypassed the sequencer, our transaction was never in the mempool in the
    // first place.
    let receipt = loop {
        if let Some(receipt) = l2_client
            .get_transaction_receipt(transfer_hash)
            .await
            .unwrap()
        {
            break receipt;
        }
        tracing::info!("Waiting for transfer to complete");
        tracing::info!(
            "L2 balance {}/{}",
            l2_client
                .get_balance(l2_client.address(), None)
                .await
                .unwrap(),
            l2_initial_balance
        );
        sleep(Duration::from_secs(5)).await;
    };
    tracing::info!("L2 transfer completed: {:?}", receipt);

    // Check the effects of the transaction.
    assert_eq!(
        l2_client
            .get_balance(l2_client.address(), None)
            .await
            .unwrap(),
        l2_initial_balance - TRANSFER_AMOUNT
    );

    // Wait for the prover to verify the forced batch.
    let event = rollup
        .trusted_verify_batches_filter()
        .from_block(l1_initial_block)
        .stream()
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    tracing::info!("Batch verified: {:?}", event);
    assert!(event.num_batch >= forced_batch_num);
}

async fn await_tx<'a, P: JsonRpcClient>(
    msg: &str,
    tx: PendingTransaction<'a, P>,
) -> TransactionReceipt {
    tracing::info!("{}: {:#x}", msg, tx.tx_hash());
    let receipt = tx.await.unwrap().unwrap();
    tracing::info!("{}: {:?}", msg, receipt);
    assert_eq!(receipt.status, Some(1.into()));
    receipt
}
