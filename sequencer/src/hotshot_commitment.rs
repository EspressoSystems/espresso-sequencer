use async_std::{sync::Arc, task::sleep};
use clap::Parser;
use contract_bindings::HotShot;
use ethers::prelude::*;
use futures::{future::FutureExt, stream::StreamExt};
use hotshot_query_service::{availability::LeafQueryData, Block, Deltas, Resolvable};
use hotshot_types::traits::node_implementation::NodeImplementation;
use sequencer_utils::{commitment_to_u256, connect_rpc, contract_send, Middleware};
use std::time::Duration;
use surf_disco::Url;

use crate::{network, Node, SeqTypes};

const RETRY_DELAY: Duration = Duration::from_secs(1);

type HotShotClient = surf_disco::Client<hotshot_query_service::Error>;

#[derive(Parser, Clone, Debug)]
pub struct CommitmentTaskOptions {
    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Chain ID for layer 1 Ethereum.
    ///
    /// This can be specified explicitly as a sanity check. No transactions will be executed if the
    /// RPC specified by `l1_provider` has a different chain ID. If not specified, the chain ID from
    /// the RPC will be used.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_CHAIN_ID")]
    pub l1_chain_id: Option<u64>,

    /// Address of HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_HOTSHOT_ADDRESS", default_value = None)]
    pub hotshot_address: Address,

    /// Mnemonic phrase for a funded wallet.
    ///
    /// This is the wallet that will be used to send blocks sequenced by HotShot to the sequencer
    /// contract. It must be funded with ETH on layer 1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC", default_value = None)]
    pub sequencer_mnemonic: String,

    /// Index of a funded account derived from sequencer-mnemonic.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_ETH_ACCOUNT_INDEX",
        default_value = "0"
    )]
    pub sequencer_account_index: u32,

    /// URL of HotShot Query Service
    ///
    /// Even though this is an Option type it *must* currently be set when
    /// passing the options to `run_hotshot_commitment_task`.
    #[clap(long, env = "ESPRESSO_SEQUENCER_QUERY_SERVICE_URL")]
    pub query_service_url: Option<Url>,
}

pub async fn run_hotshot_commitment_task(opt: &CommitmentTaskOptions) {
    let query_service_url = opt
        .query_service_url
        .clone()
        .expect("query service URL must be specified")
        .join("availability")
        .unwrap();

    let hotshot = HotShotClient::new(query_service_url);
    hotshot.connect(None).await;

    // Connect to the layer one HotShot contract.
    let Some(l1) = connect_l1(opt).await else {
        panic!("unable to connect to L1, hotshot commitment task exiting");

    };
    let contract = HotShot::new(opt.hotshot_address, l1.clone());

    // Get the last block number sequenced.
    let from = match contract.block_height().call().await {
        Ok(from) => from.as_u64(),
        Err(err) => {
            tracing::error!("unable to read block_height from contract: {}", err);
            panic!("hotshot commitment task will exit");
        }
    };
    tracing::info!("last block sequenced: {}", from);

    // Get the maximum number of blocks the contract will allow at a time.
    let max = match contract.max_blocks().call().await {
        Ok(max) => max.as_u64(),
        Err(err) => {
            tracing::error!("unable to read max_blocks from contract: {}", err);
            panic!("hotshot commitment task will exit");
        }
    };
    sequence::<Node<network::Centralized>>(from, max, hotshot, contract).await;
}

async fn sequence<I: NodeImplementation<SeqTypes>>(
    from: u64,
    max_blocks: u64,
    hotshot: HotShotClient,
    contract: HotShot<Middleware>,
) where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    let mut leaves = match hotshot
        .socket(&format!("stream/leaves/{from}"))
        .subscribe()
        .await
    {
        Ok(leaves) => Box::pin(leaves.peekable()),
        Err(err) => {
            tracing::error!("unable to subscribe to HotShot query service: {}", err);
            tracing::error!("hotshot commitment task will exit");
            return;
        }
    };

    loop {
        // Wait for HotShot to sequence a block.
        let leaf: LeafQueryData<SeqTypes, I> = match leaves.next().await {
            Some(Ok(leaf)) => leaf,
            Some(Err(err)) => {
                tracing::error!("error from HotShot, retrying: {}", err);
                continue;
            }
            None => {
                tracing::error!("HotShot leaf stream ended, hotshot commitment task will exit");
                return;
            }
        };
        tracing::info!("received leaf from HotShot: {:?}", leaf);

        // It is possible that multiple blocks are already available, if HotShot is running faster
        // than we are. Collect as many blocks as are ready (up to the allowed maximum) so we can
        // send them all to the contract at once to save a little gas.
        let mut to_sequence = vec![leaf];
        while to_sequence.len() + 1 < max_blocks as usize {
            if let Some(Some(Ok(leaf))) = leaves.as_mut().peek().now_or_never() {
                tracing::info!("an additional block is also ready: {:?}", leaf);
                // Since the block has been peeked, we can remove it from the stream with `next()`,
                // this should never block or return `None`.
                to_sequence.push(
                    leaves
                        .next()
                        .await
                        .expect("next() returned None after peek() returned Some")
                        .expect("next() returned Some(Err) after peek() returned Some(Ok)"),
                );
            } else {
                break;
            }
        }
        tracing::info!("sequencing {}/{} blocks", to_sequence.len(), max_blocks,);

        // Sequence the blocks.
        sequence_batches(&contract, to_sequence).await;
    }
}

async fn sequence_batches<I: NodeImplementation<SeqTypes>>(
    contract: &HotShot<Middleware>,
    leaves: impl IntoIterator<Item = LeafQueryData<SeqTypes, I>>,
) where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    let txn = build_sequence_batches_txn(contract, leaves);

    // Send the block commitments and QCs to L1. This operation must succeed before we go any
    // further, because sequencing the next batch will depend on having successfully sequenced this
    // one. Thus we will retry until it succeeds.
    while contract_send(&txn).await.is_none() {
        tracing::warn!("failed to sequence batches, retrying");
        sleep(RETRY_DELAY).await;
    }
}

fn build_sequence_batches_txn<I: NodeImplementation<SeqTypes>, M: ethers::prelude::Middleware>(
    contract: &HotShot<M>,
    leaves: impl IntoIterator<Item = LeafQueryData<SeqTypes, I>>,
) -> ContractCall<M, ()>
where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    let (block_comms, qcs) = leaves
        .into_iter()
        .map(|leaf| {
            (
                commitment_to_u256(leaf.block_hash()),
                // The QC validation part of the contract is currently mocked out, so it doesn't
                // matter what we send here. For realism of gas usage, we want to send something of
                // the correct size. The plan for on-chain QC validation is for the contract to only
                // take a few 32-byte words of the QC, with the rest replaced by a short commitment,
                // since the contract doesn't need all the fields of the QC and storing the whole
                // QC in calldata can be expensive (or even run into RPC size limits).
                [0; 32 * 3].into(),
            )
        })
        .unzip();
    contract.new_blocks(block_comms, qcs)
}

pub async fn connect_l1(opt: &CommitmentTaskOptions) -> Option<Arc<Middleware>> {
    connect_rpc(
        &opt.l1_provider,
        &opt.sequencer_mnemonic,
        opt.sequencer_account_index,
        opt.l1_chain_id,
    )
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Block, Leaf, Transaction};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use commit::Committable;
    use contract_bindings::{hot_shot::NewBlocksCall, TestL1System};
    use ethers::{abi::AbiDecode, providers::Middleware};
    use hotshot_types::{
        certificate::QuorumCertificate,
        data::{LeafType, ViewNumber},
        traits::{block_contents::Block as _, election::SignedCertificate, state::ConsensusTime},
    };
    use sequencer_utils::AnvilOptions;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    #[async_std::test]
    async fn test_sequencer_task() {
        setup_logging();
        setup_backtrace();

        let anvil = AnvilOptions::default().spawn().await;
        let l1 = TestL1System::deploy(anvil.provider()).await.unwrap();

        let l1_initial_block = l1.provider.get_block_number().await.unwrap();
        let initial_batch_num = l1.hotshot.block_height().call().await.unwrap();

        let adaptor_l1_signer = connect_rpc(
            l1.provider.url(),
            TEST_MNEMONIC,
            l1.clients.funded[0].index,
            None,
        )
        .await
        .unwrap();

        // Create a few test batches.
        let num_batches = l1.hotshot.max_blocks().call().await.unwrap().as_u64();
        let mut leaves: Vec<LeafQueryData<SeqTypes, Node<network::Memory>>> = vec![];
        for _ in 0..num_batches {
            let txn = Transaction::new(1.into(), vec![]);
            let block = Block::new().add_transaction_raw(&txn).unwrap();

            // Fake a leaf that sequences this block.
            let mut qc = QuorumCertificate::genesis();
            let leaf = Leaf::new(ViewNumber::genesis(), qc.clone(), block, Default::default());
            qc.leaf_commitment = leaf.commit();
            leaves.push(LeafQueryData::new(leaf, qc).unwrap());
        }
        tracing::info!("sequencing batches: {:?}", leaves);

        // Connect to the HotShot contract with the expected L1 client.
        let hotshot = HotShot::new(l1.hotshot.address(), adaptor_l1_signer);

        // Ensure the transaction we're going to execute is less than the Geth RPC size limit.
        let txn = build_sequence_batches_txn(&l1.hotshot, leaves.clone()).tx;
        let size = txn.rlp().len();
        tracing::info!("transaction is {size} bytes");
        assert!(size < 131072);

        // Sequence them in the HotShot contract.
        sequence_batches(&hotshot, leaves.clone()).await;

        // Check the NewBatches event.
        let (event, meta) = l1
            .hotshot
            .new_blocks_filter()
            .from_block(l1_initial_block)
            .query_with_meta()
            .await
            .unwrap()
            .remove(0);
        assert_eq!(event.first_block_number, initial_batch_num);

        let calldata = l1
            .provider
            .get_transaction(meta.transaction_hash)
            .await
            .unwrap()
            .unwrap()
            .input;
        let call = NewBlocksCall::decode(calldata).unwrap();
        assert_eq!(
            call.new_commitments,
            leaves
                .iter()
                .map(|leaf| U256::from_little_endian(&<[u8; 32]>::from(leaf.block_hash())))
                .collect::<Vec<_>>()
        );
    }
}
