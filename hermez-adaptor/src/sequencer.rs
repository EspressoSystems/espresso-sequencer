use crate::{Options, HERMEZ_MAX_VERIFY_BATCHES};
use async_std::{sync::Arc, task::sleep};
use contract_bindings::{
    polygon_zk_evm::{ForceBatchFilter, ForcedBatchData},
    Matic, PolygonZkEVM,
};
use ethers::{
    abi::{Detokenize, RawLog},
    contract::builders::ContractCall,
    prelude::*,
    providers::Middleware as _,
    signers::coins_bip39::English,
};
use futures::{
    future::FutureExt,
    stream::{self, StreamExt},
};
use hotshot_query_service::availability::BlockQueryData;
use sequencer::{Block, SeqTypes};
use std::time::Duration;
use surf_disco::Url;
use zkevm::{hermez, ZkEvm};

const RETRY_DELAY: Duration = Duration::from_secs(1);

type Middleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;
type HotShotClient = surf_disco::Client<hotshot_query_service::Error>;

pub async fn run(opt: &Options) {
    // Connect to the HotShot query service to stream sequenced blocks.
    let hotshot = HotShotClient::new(opt.sequencer_url.join("availability").unwrap());
    hotshot.connect(None).await;

    // Connect to the layer one rollup and matic contracts.
    let Some(l1) = connect_l1(opt).await else {
        tracing::error!("unable to connect to L1, sequencer task exiting");
        return;
    };
    tracing::info!("connected to l1 at {}", opt.l1_provider);
    let rollup = PolygonZkEVM::new(opt.rollup_address, l1.clone());
    let matic = Matic::new(opt.matic_address, l1);

    // The contract will need to take MATIC out of our account to collect sequencing fees. We trust
    // the contract to do this correctly so we will approve a large amount of MATIC once, to save
    // the complexity and gas cost of having to frequently re-approve.
    loop {
        tracing::info!("approving {} MATIC for {}", U256::MAX, rollup.address());
        match matic.approve(rollup.address(), U256::MAX).send().await {
            Ok(tx) => match tx.await {
                Ok(Some(_)) => break,
                Ok(None) => {
                    tracing::warn!("MATIC allowance transaction not mined, retrying");
                }
                Err(err) => {
                    tracing::error!("MATIC allowance transaction failed: {}", err);
                    tracing::error!("sequencer task will exit");
                    return;
                }
            },
            Err(err) => {
                tracing::error!("unable to approve MATIC transfer to contract: {}", err);
                tracing::error!("sequencer task will exit");
                return;
            }
        }
    }

    // Get the last batch number sequenced.
    let from = match rollup.last_batch_sequenced().call().await {
        Ok(from) => from,
        Err(err) => {
            tracing::error!(
                "unable to read last_batch_sequenced from rollup contract: {}",
                err
            );
            tracing::error!("sequencer task will exit");
            return;
        }
    };
    tracing::info!("last batch sequenced: {}", from);

    sequence(&opt.zkevm(), from, hotshot, rollup).await;
}

async fn sequence(
    zkevm: &ZkEvm,
    from: u64,
    hotshot: HotShotClient,
    rollup: PolygonZkEVM<Middleware>,
) {
    let mut blocks = match hotshot
        .socket(&format!("stream/blocks/{from}"))
        .subscribe()
        .await
    {
        Ok(blocks) => Box::pin(blocks.peekable()),
        Err(err) => {
            tracing::error!("unable to subscribe to HotShot query service: {}", err);
            tracing::error!("sequencer task will exit");
            return;
        }
    };

    loop {
        // Wait for HotShot to sequence a block.
        let block: BlockQueryData<SeqTypes> = match blocks.next().await {
            Some(Ok(block)) => block,
            Some(Err(err)) => {
                tracing::error!("error from HotShot, retrying: {}", err);
                continue;
            }
            None => {
                tracing::error!("HotShot block stream ended, sequencer task will exit");
                return;
            }
        };
        tracing::info!("received block from HotShot: {:?}", block);

        // It is possible that multiple blocks are already available, if HotShot
        // is running faster than we are. Collect as many blocks as are ready
        // (up to the allowed maximum) so we can send them all to the contract
        // at once to save a little gas.
        let mut to_sequence = vec![block];
        while to_sequence.len() + 1 < HERMEZ_MAX_VERIFY_BATCHES {
            if let Some(Some(Ok(block))) = blocks.as_mut().peek().now_or_never() {
                tracing::info!("an additional block is also ready: {:?}", block);
                // Since the block has been peeked, we can remove it from the stream with `next()`,
                // this should never block or return `None`.
                to_sequence.push(
                    blocks
                        .next()
                        .await
                        .expect("next() returned None after peek() returned Some")
                        .expect("next() returned Some(Err) after peek() returned Some(Ok)"),
                );
            } else {
                break;
            }
        }
        tracing::info!(
            "sequencing {}/{} blocks",
            to_sequence.len(),
            HERMEZ_MAX_VERIFY_BATCHES
        );

        // Sequence the blocks.
        sequence_batches(
            zkevm,
            &rollup,
            to_sequence.iter().map(|block| block.block()),
        )
        .await;
    }
}

async fn sequence_batches(
    zkevm: &ZkEvm,
    rollup: &PolygonZkEVM<Middleware>,
    blocks: impl IntoIterator<Item = &Block>,
) {
    // Convert the blocks to byte-encoded EVM transactions.
    let blocks = blocks
        .into_iter()
        .map(|block| hermez::encode_transactions(block.vm_transactions(zkevm)));

    // Send the batches to L1. Collect the [ForcedBatchData] that we will need to later sequence
    // each batch that we send.
    //
    // The batches will eventually be sequenced in the order that we send them. This means that if
    // any of these transactions fail for any reason, we must retry before executing any of the rest
    // of the transactions, so we have to wait for confirmation of each transaction before
    // proceeding to the next.
    //
    // When we switch to using our own rollup contract, we should add a batched version of this
    // method so that we can send many blocks in a single, atomic Ethereum transaction.
    let forced_batches = stream::iter(blocks)
        .enumerate()
        .then(|(i, transactions)| async move {
            tracing::info!("forcing batch {}: {:?}", i, transactions);

            // Try sending the batch until we succeed.
            let (receipt, block_number) = loop {
                // Get the fee we are required to pay.
                let fee = match rollup.batch_fee().call().await {
                    Ok(fee) => fee,
                    Err(err) => {
                        tracing::error!("unable to get current batch fee, retrying: {}", err);
                        sleep(RETRY_DELAY).await;
                        continue;
                    }
                };
                tracing::info!("batch fee is {}", fee);

                let Some(receipt) = send(rollup.force_batch(transactions.clone(), fee)).await else {
                    tracing::warn!("failed to force batch, retrying");
                    sleep(RETRY_DELAY).await;
                    continue;
                };
                break receipt;
            };

            // Now that we have successfully sent the batch to L1, we must not re-run that
            // operation. If anything fails below as we try to parse the ForceBatch event from the
            // logs, all we can do is retry the specific failed operation or panic.
            let event = receipt
                .logs
                .into_iter()
                .find_map(|log| {
                    <ForceBatchFilter as EthEvent>::decode_log(&RawLog {
                        topics: log.topics,
                        data: log.data.to_vec(),
                    })
                    .ok()
                })
                // The rollup contract always emits a ForceBatch event after forceBatch succeeds. If
                // we fail to find one, it can only be because we are not talking to the contract we
                // think we are; perhaps our ABI is outdated. Retrying is pointless, we should fail
                // loudly and let a human investigate.
                .expect("forceBatch succeeded but logs did not contain a ForceBatch event");

            // The forced batch was recorded with the timestamp of the block where the transaction
            // executed. In order to specify this batch for sequencing, we need the same timestamp.
            let block = loop {
                match rollup.client().get_block(block_number).await {
                    Ok(block) => {
                        // If the RPC responds successfully but tells us there is no block with this
                        // number, it is lying. At this point we should just fail loudly.
                        break block.unwrap_or_else(|| {
                            panic!("RPC says block {block_number} does not exist")
                        });
                    }
                    Err(err) => {
                        // If we get an error contacting the RPC, retry until successful.
                        tracing::error!(
                            "error getting block {} from RPC, retrying: {}",
                            block_number,
                            err
                        );
                        sleep(RETRY_DELAY).await;
                        continue;
                    }
                }
            };

            ForcedBatchData {
                transactions,
                global_exit_root: event.last_global_exit_root,
                min_forced_timestamp: block.timestamp.as_u64(),
            }
        })
        .collect::<Vec<_>>()
        .await;

    // Once all the batches have been sent to the L1, we can sequence them. This we can do in a
    // single transaction. It must succeed before we can sequence future batches from HotShot, so
    // retry until it does.
    while send(rollup.sequence_force_batches(forced_batches.clone()))
        .await
        .is_none()
    {
        tracing::warn!("failed to sequence forced batches, retrying");
        sleep(RETRY_DELAY).await;
    }
}

async fn send<T: Detokenize>(
    call: ContractCall<Middleware, T>,
) -> Option<(TransactionReceipt, u64)> {
    let pending = match call.send().await {
        Ok(pending) => pending,
        Err(err) => {
            tracing::error!("error sending transaction: {}", err);
            return None;
        }
    };
    let receipt = match pending.await {
        Ok(Some(receipt)) => receipt,
        Ok(None) => {
            tracing::error!("transaction not mined");
            return None;
        }
        Err(err) => {
            tracing::error!("error waiting for transaction to be mined: {}", err);
            return None;
        }
    };
    if receipt.status != Some(1.into()) {
        tracing::error!("transaction reverted");
        return None;
    }

    // If a transaction is mined and we get a receipt for it, the block number should _always_ be
    // set. If it is not, something has gone horribly wrong with the RPC.
    let block_number = receipt
        .block_number
        .expect("transaction mined but block number not set");
    Some((receipt, block_number.as_u64()))
}

async fn connect_l1(opt: &Options) -> Option<Arc<Middleware>> {
    connect_rpc(&opt.l1_provider, &opt.sequencer_mnemonic, opt.l1_chain_id).await
}

async fn connect_rpc(
    provider: &Url,
    mnemonic: &str,
    chain_id: Option<u64>,
) -> Option<Arc<Middleware>> {
    let provider = match Provider::try_from(provider.to_string()) {
        Ok(provider) => provider,
        Err(err) => {
            tracing::error!("error connecting to RPC {}: {}", provider, err);
            return None;
        }
    };
    let chain_id = match chain_id {
        Some(id) => id,
        None => match provider.get_chainid().await {
            Ok(id) => id.as_u64(),
            Err(err) => {
                tracing::error!("error getting chain ID: {}", err);
                return None;
            }
        },
    };
    let wallet = match MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .build()
    {
        Ok(wallet) => wallet,
        Err(err) => {
            tracing::error!("error opening wallet: {}", err);
            return None;
        }
    };
    let wallet = wallet.with_chain_id(chain_id);
    let address = wallet.address();
    Some(Arc::new(NonceManagerMiddleware::new(
        SignerMiddleware::new(provider, wallet),
        address,
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Layer1Backend, ZkEvmNode};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use commit::Committable;
    use futures::future::join_all;
    use hotshot_types::traits::block_contents::Block as _;
    use sequencer::{State, Vm};
    use zkevm::EvmTransaction;

    #[async_std::test]
    async fn test_sequencer_task() {
        setup_logging();
        setup_backtrace();

        let node = ZkEvmNode::start("test-sequencer-task".to_string(), Layer1Backend::Anvil).await;

        // Get test setup from environment.
        let env = node.env();
        let l1_provider = env.l1_provider();
        let l2_provider = env.l2_provider();
        let mnemonic = env.funded_mnemonic();
        let rollup_address = node.l1().rollup.address();

        let l1 = connect_rpc(&l1_provider, mnemonic, None).await.unwrap();
        let l2 = &connect_rpc(&l2_provider, mnemonic, None).await.unwrap();
        let zkevm = ZkEvm {
            chain_id: l2.get_chainid().await.unwrap().as_u64(),
        };
        let rollup = PolygonZkEVM::new(rollup_address, l1.clone());
        let l1_initial_block = l1.get_block_number().await.unwrap();
        let initial_batch_num = rollup.last_batch_sequenced().call().await.unwrap();
        let initial_force_batch_num = rollup.last_force_batch().call().await.unwrap();
        let l2_initial_balance = l2.get_balance(l2.inner().address(), None).await.unwrap();
        tracing::info!(
            "address: {}, rollup address: {}, \
             L1 initial block: {}, initial batch num: {}, L2 initial balance: {}",
            l1.inner().address(),
            rollup.address(),
            l1_initial_block,
            initial_batch_num,
            l2_initial_balance,
        );

        // Create a few test batches.
        let transfer_amount = 1.into();
        let num_batches = 2u64;
        let nonce = l2
            .get_transaction_count(l2.inner().address(), None)
            .await
            .unwrap();
        let (batches, txn_hashes): (Vec<_>, Vec<_>) =
            join_all((0..num_batches).into_iter().map(|i| async move {
                let mut transfer = TransactionRequest {
                    from: Some(l2.inner().address()),
                    to: Some(Address::random().into()),
                    value: Some(transfer_amount),
                    nonce: Some(nonce + i),
                    ..Default::default()
                }
                .into();
                l2.fill_transaction(&mut transfer, None).await.unwrap();
                tracing::info!("transfer {}: {:?}", i, transfer);
                let signature = l2
                    .inner()
                    .signer()
                    .sign_transaction(&transfer)
                    .await
                    .unwrap();
                let txn = EvmTransaction::new(transfer, signature);
                let hash = txn.hash();
                tracing::info!("transfer hash: {}", hash);
                (
                    Block::new(State::default().commit())
                        .add_transaction_raw(&zkevm.wrap(&txn).into())
                        .unwrap(),
                    hash,
                )
            }))
            .await
            .into_iter()
            .unzip();
        tracing::info!("sequencing batches: {:?}", batches);

        // Sequence them in the rollup contract.
        sequence_batches(&zkevm, &rollup, &batches).await;

        // Check the forced batch events.
        let force_batch_events = rollup
            .force_batch_filter()
            .from_block(l1_initial_block)
            .query()
            .await
            .unwrap();
        assert_eq!(force_batch_events.len(), num_batches as usize);
        for (i, event) in force_batch_events.into_iter().enumerate() {
            assert_eq!(event.sequencer, l1.inner().address());
            assert_eq!(
                event.force_batch_num,
                initial_force_batch_num + 1 + i as u64
            );
        }

        // Check the sequence event.
        let sequence_force_batches_events = rollup
            .sequence_force_batches_filter()
            .from_block(l1_initial_block)
            .query()
            .await
            .unwrap();
        assert_eq!(sequence_force_batches_events.len(), 1);
        assert_eq!(
            sequence_force_batches_events[0].num_batch,
            initial_batch_num + num_batches
        );

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
                sleep(Duration::from_secs(5)).await;
            }
        }

        // Check the effects of the transfers.
        assert_eq!(
            l2.get_balance(l2.inner().address(), None).await.unwrap(),
            l2_initial_balance - U256::from(num_batches) * transfer_amount
        );

        // Wait for the batches to be verified.
        let event = rollup
            .verify_batches_trusted_aggregator_filter()
            .from_block(l1_initial_block)
            .stream()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(event.num_batch, initial_batch_num + num_batches);
    }
}
