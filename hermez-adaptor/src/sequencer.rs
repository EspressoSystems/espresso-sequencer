use crate::Options;
use async_std::sync::Arc;
use contract_bindings::{
    proof_of_efficiency::{ForceBatchFilter, ForcedBatchData},
    Matic, ProofOfEfficiency,
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
use tide_disco::error::ServerError;
use zkevm::{hermez, ZkEvm};

type Middleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub async fn run(opt: &Options) {
    // Connect to the HotShot query service to stream sequenced blocks.
    let hotshot = surf_disco::Client::<ServerError>::new(opt.sequencer_url.clone());
    hotshot.connect(None).await;

    // Connect to the layer one rollup and matic contracts.
    let Some(l1) = connect_l1(opt) else {
        tracing::error!("unable to connect to L1, sequencer task exiting");
        return;
    };
    let rollup = ProofOfEfficiency::new(opt.rollup_address, l1.clone());
    let matic = Matic::new(opt.matic_address, l1);

    // The contract will need to take MATIC out of our account to collect sequencing fees. We trust
    // the contract to do this correctly so we will approve a large amount of MATIC once, to save
    // the complexity and gas cost of having to frequently re-approve.
    loop {
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

    // Get the maximum number of batches allowed to be sequenced at once.
    let max = match rollup.max_verify_batches().call().await {
        Ok(max) => max,
        Err(err) => {
            tracing::error!(
                "unable to read MAX_VERIFY_BATCHES from rollup contract: {}",
                err
            );
            tracing::error!("sequencer task will exit");
            return;
        }
    };

    sequence(&opt.zkevm(), from, max, hotshot, rollup).await;
}

async fn sequence(
    zkevm: &ZkEvm,
    from: u64,
    max_batches: u64,
    hotshot: surf_disco::Client<ServerError>,
    rollup: ProofOfEfficiency<Middleware>,
) {
    let mut blocks = match hotshot
        .socket(&format!("stream/blocks/{}", from))
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

        // It is possible that multiple blocks are already available, if HotShot is running faster
        // than we are. Collect as many blocks as are ready (up to `max_batches`) so we can send
        // them all to the contract at once to save a little gas.
        let mut to_sequence = vec![block];
        while to_sequence.len() + 1 < max_batches as usize {
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
        tracing::info!("sequencing {}/{} blocks", to_sequence.len(), max_batches);

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
    rollup: &ProofOfEfficiency<Middleware>,
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
                        continue;
                    }
                };
                tracing::info!("batch fee is {}", fee);

                let Some(receipt) = send(rollup.force_batch(transactions.clone(), fee)).await else {
                    tracing::warn!("failed to force batch, retrying");
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
                            panic!("RPC says block {} does not exist", block_number)
                        });
                    }
                    Err(err) => {
                        // If we get an error contacting the RPC, retry until successful.
                        tracing::error!(
                            "error getting block {} from RPC, retrying: {}",
                            block_number,
                            err
                        );
                        continue;
                    }
                }
            };

            ForcedBatchData {
                transactions,
                global_exit_root: event.last_global_exit_root,
                min_forced_timestamp: block.timestamp.try_into().unwrap(),
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

    // If a transaction is mined and we get a receipt for it, the block number should _always_ be
    // set. If it is not, something has gone horribly wrong with the RPC.
    let block_number = receipt
        .block_number
        .expect("transaction mined but block number not set");
    Some((receipt, block_number.as_u64()))
}

fn connect_l1(opt: &Options) -> Option<Arc<Middleware>> {
    let provider = match Provider::try_from(opt.l1_provider.to_string()) {
        Ok(provider) => provider,
        Err(err) => {
            tracing::error!("error connecting to L1 RPC: {}", err);
            return None;
        }
    };
    let wallet = match MnemonicBuilder::<English>::default()
        .phrase(opt.sequencer_mnemonic.as_str())
        .build()
    {
        Ok(wallet) => wallet,
        Err(err) => {
            tracing::error!("error opening sequencer wallet: {}", err);
            return None;
        }
    };
    let address = wallet.address();
    Some(Arc::new(NonceManagerMiddleware::new(
        SignerMiddleware::new(provider, wallet),
        address,
    )))
}
