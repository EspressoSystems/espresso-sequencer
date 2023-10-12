use async_std::{sync::Arc, task::sleep};
use async_trait::async_trait;
use clap::Parser;
use contract_bindings::hot_shot::{HotShot, Qc};
use ethers::prelude::*;
use futures::{future::try_join_all, stream::StreamExt};
use hotshot_query_service::{
    availability::{BlockHeaderQueryData, LeafQueryData},
    Block, Deltas, Resolvable,
};
use hotshot_types::traits::node_implementation::NodeImplementation;
use sequencer_utils::{commitment_to_u256, connect_rpc, contract_send, Middleware};
use std::error::Error;
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
        .expect("query service URL must be specified");

    let hotshot = HotShotClient::new(query_service_url);
    hotshot.connect(None).await;

    // Connect to the layer one HotShot contract.
    let Some(l1) = connect_l1(opt).await else {
        panic!("unable to connect to L1, hotshot commitment task exiting");
    };
    let contract = HotShot::new(opt.hotshot_address, l1.clone());

    // Get the maximum number of blocks the contract will allow at a time.
    let max = match contract.max_blocks().call().await {
        Ok(max) => max.as_u64(),
        Err(err) => {
            tracing::error!("unable to read max_blocks from contract: {}", err);
            panic!("hotshot commitment task will exit");
        }
    };
    sequence::<Node<network::Web>>(max, hotshot, contract).await;
}

async fn sequence<I: NodeImplementation<SeqTypes>>(
    max_blocks: u64,
    hotshot: HotShotClient,
    contract: HotShot<Middleware>,
) where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    loop {
        if let Err(err) = sync_with_l1::<I>(max_blocks, &hotshot, &contract).await {
            tracing::error!("error synchronizing with HotShot contract: {err}");

            // Wait a bit to avoid spam, then try again.
            sleep(RETRY_DELAY).await;
        }
    }
}

#[async_trait]
trait HotShotDataSource<I: NodeImplementation<SeqTypes>> {
    type Error: Error + Send + Sync + 'static;

    async fn block_height(&self) -> Result<u64, Self::Error>;
    async fn wait_for_block_height(&self, height: u64) -> Result<(), Self::Error>;
    async fn get_leaf(&self, height: u64) -> Result<LeafQueryData<SeqTypes, I>, Self::Error>;
}

#[async_trait]
impl<I: NodeImplementation<SeqTypes>> HotShotDataSource<I> for HotShotClient {
    type Error = hotshot_query_service::Error;

    async fn block_height(&self) -> Result<u64, Self::Error> {
        self.get("status/latest_block_height").send().await
    }

    async fn wait_for_block_height(&self, height: u64) -> Result<(), Self::Error> {
        let mut stream = self
            .socket(&format!("availability/stream/block/headers/{height}"))
            .subscribe::<BlockHeaderQueryData<SeqTypes>>()
            .await?;
        stream.next().await;
        Ok(())
    }

    async fn get_leaf(&self, height: u64) -> Result<LeafQueryData<SeqTypes, I>, Self::Error> {
        self.get(&format!("availability/leaf/{height}"))
            .send()
            .await
    }
}

async fn sync_with_l1<I: NodeImplementation<SeqTypes>>(
    max_blocks: u64,
    hotshot: &impl HotShotDataSource<I>,
    contract: &HotShot<Middleware>,
) -> Result<(), anyhow::Error>
where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    let contract_block_height = contract.block_height().call().await?.as_u64();
    let hotshot_block_height = loop {
        let height = hotshot.block_height().await?;
        if height <= contract_block_height {
            // If the contract is caught up with HotShot, wait for more blocks to be produced.
            tracing::debug!(
                "HotShot at height {height}, waiting for it to pass height {contract_block_height}"
            );
            hotshot.wait_for_block_height(contract_block_height).await?;
        } else {
            // HotShot is ahead of the contract, sequence the blocks which are currently ready.
            tracing::debug!("synchronizing blocks {contract_block_height}-{height}");
            break height;
        }
    };

    // Download leaves between `contract_block_height` and `hotshot_block_height`.
    let leaves = try_join_all(
        (contract_block_height..hotshot_block_height)
            .take(max_blocks as usize)
            .map(|height| hotshot.get_leaf(height)),
    )
    .await?;
    tracing::info!("sending {} leaves to the contract", leaves.len());

    // Send the leaves to the contract.
    let txn = build_sequence_batches_txn::<I, Middleware>(contract, leaves);
    // If the transaction fails for any reason -- not mined, reverted, etc. -- just return the
    // error. We will retry, and may end up changing the transaction we send if the contract state
    // has changed, which is one possible cause of the transaction failure. This can happen, for
    // example, if there are multiple commitment tasks racing.
    contract_send(&txn)
        .await
        .ok_or_else(|| anyhow::Error::msg("failed to send transaction"))?;

    Ok(())
}

fn build_sequence_batches_txn<I: NodeImplementation<SeqTypes>, M: ethers::prelude::Middleware>(
    contract: &HotShot<M>,
    leaves: impl IntoIterator<Item = LeafQueryData<SeqTypes, I>>,
) -> ContractCall<M, ()>
where
    Deltas<SeqTypes, I>: Resolvable<Block<SeqTypes>>,
{
    let qcs = leaves
        .into_iter()
        .map(|leaf| Qc {
            height: leaf.height().into(),
            block_commitment: commitment_to_u256(leaf.block_hash()),
            ..Default::default()
        })
        .collect();
    contract.new_blocks(qcs)
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
    use async_std::task::spawn;
    use commit::Committable;
    use contract_bindings::hot_shot::{NewBlocksCall, NewBlocksFilter};
    use ethers::{abi::AbiDecode, providers::Middleware};
    use futures::FutureExt;
    use hotshot_types::{
        certificate::QuorumCertificate,
        data::{LeafType, ViewNumber},
        traits::{block_contents::Block as _, election::SignedCertificate, state::ConsensusTime},
    };
    use sequencer_utils::test_utils::TestL1System;
    use sequencer_utils::AnvilOptions;
    use surf_disco::{Error, StatusCode};

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    #[derive(Clone, Debug, Default)]
    struct MockDataSource {
        leaves: Vec<LeafQueryData<SeqTypes, Node<network::Memory>>>,
    }

    #[async_trait]
    impl HotShotDataSource<Node<network::Memory>> for MockDataSource {
        type Error = hotshot_query_service::Error;

        async fn block_height(&self) -> Result<u64, Self::Error> {
            Ok(self.leaves.len() as u64)
        }

        async fn wait_for_block_height(&self, height: u64) -> Result<(), Self::Error> {
            if height < self.block_height().await? {
                return Ok(());
            }

            // The tests don't rely on this subscription mechanism; they merely check that
            // `sync_with_l1` blocks in the case where a new block is not ready. Blocking forever
            // here is fine and much simpler than implementing a proper notification mechanism.
            futures::future::pending().await
        }

        async fn get_leaf(
            &self,
            height: u64,
        ) -> Result<LeafQueryData<SeqTypes, Node<network::Memory>>, Self::Error> {
            self.leaves.get(height as usize).cloned().ok_or_else(|| {
                Self::Error::catch_all(StatusCode::NotFound, format!("no leaf for height {height}"))
            })
        }
    }

    fn mock_leaf(height: u64) -> LeafQueryData<SeqTypes, Node<network::Memory>> {
        let txn = Transaction::new(1.into(), vec![]);
        let block = Block::new().add_transaction_raw(&txn).unwrap();

        // Fake a leaf that sequences this block.
        let mut qc = QuorumCertificate::genesis();
        let mut leaf = Leaf::new(ViewNumber::genesis(), qc.clone(), block, Default::default());
        // Mimic the behavior of HotShot, where leaf heights are indexed starting from 1, since the
        // 0-height genesis leaf is implicit.
        leaf.height = height + 1;
        qc.leaf_commitment = leaf.commit();
        LeafQueryData::new(leaf, qc).unwrap()
    }

    async fn wait_for_new_batches(
        l1: &TestL1System,
        from_block: u64,
    ) -> (NewBlocksFilter, LogMeta) {
        l1.hotshot
            .new_blocks_filter()
            .from_block(from_block)
            // Ethers does not set the contract address on filters created via contract bindings.
            // This seems like a bug and I have reported it:
            // https://github.com/gakonst/ethers-rs/issues/2528. In the mean time we can work around
            // by setting the address manually.
            .address(l1.hotshot.address().into())
            .query_with_meta()
            .await
            .unwrap()
            .remove(0)
    }

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
        let mut data = MockDataSource::default();
        for i in 0..num_batches {
            data.leaves.push(mock_leaf(i));
        }
        tracing::info!("sequencing batches: {:?}", data.leaves);

        // Connect to the HotShot contract with the expected L1 client.
        let hotshot = HotShot::new(l1.hotshot.address(), adaptor_l1_signer);

        // Ensure the transaction we're going to execute is less than the Geth RPC size limit.
        let txn = build_sequence_batches_txn(&l1.hotshot, data.leaves.clone()).tx;
        let size = txn.rlp().len();
        tracing::info!("transaction is {size} bytes");
        assert!(size < 131072);

        // Sequence them in the HotShot contract.
        sync_with_l1(num_batches, &data, &hotshot).await.unwrap();

        // Check the NewBatches event.
        let (event, meta) = wait_for_new_batches(&l1, l1_initial_block.as_u64()).await;
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
            call.qcs,
            data.leaves
                .iter()
                .map(|leaf| Qc {
                    height: leaf.height().into(),
                    block_commitment: U256::from_little_endian(&<[u8; 32]>::from(
                        leaf.block_hash()
                    )),
                    ..Default::default()
                })
                .collect::<Vec<_>>()
        );
    }

    #[async_std::test]
    async fn test_idempotency() {
        setup_logging();
        setup_backtrace();

        let anvil = AnvilOptions::default().spawn().await;
        let l1 = TestL1System::deploy(anvil.provider()).await.unwrap();
        let mut from_block = l1.provider.get_block_number().await.unwrap();
        let adaptor_l1_signer = connect_rpc(
            l1.provider.url(),
            TEST_MNEMONIC,
            l1.clients.funded[0].index,
            None,
        )
        .await
        .unwrap();

        // Create a test batch.
        let mut data = MockDataSource::default();
        data.leaves.push(mock_leaf(0));

        // Connect to the HotShot contract with the expected L1 client.
        let hotshot = HotShot::new(l1.hotshot.address(), adaptor_l1_signer);

        // Sequence them in the HotShot contract.
        sync_with_l1(1, &data, &hotshot).await.unwrap();

        // Check the NewBatches event.
        let (event, meta) = wait_for_new_batches(&l1, from_block.as_u64()).await;
        assert_eq!(event.first_block_number.as_u64(), 0);
        from_block = meta.block_number + 1;

        // Sequencing the same batch again should block until new blocks are available.
        let fut = {
            let data = data.clone();
            let hotshot = hotshot.clone();
            spawn(async move { sync_with_l1(1, &data, &hotshot).await })
        };
        // Sleep for a few seconds and make sure nothing happened.
        sleep(Duration::from_secs(3)).await;
        assert!(fut.now_or_never().is_none());
        assert_eq!(l1.hotshot.block_height().call().await.unwrap().as_u64(), 1);

        // Once a new batch is available, we can sequence it.
        data.leaves.push(mock_leaf(1));
        sync_with_l1(1, &data, &hotshot).await.unwrap();
        let (event, _) = wait_for_new_batches(&l1, from_block.as_u64()).await;
        assert_eq!(event.first_block_number.as_u64(), 1);

        // Double-check the data in the contract.
        assert_eq!(
            l1.hotshot.commitments(0.into()).call().await.unwrap(),
            commitment_to_u256(data.leaves[0].block_hash())
        );
        assert_eq!(
            l1.hotshot.commitments(1.into()).call().await.unwrap(),
            commitment_to_u256(data.leaves[1].block_hash())
        );
        assert_eq!(
            l1.hotshot.commitments(2.into()).call().await.unwrap(),
            0.into()
        );
    }
}
