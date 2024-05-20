//! L1 Client
//!
//! [`L1Client`] defines the interface that Espresso Sequencer nodes use to interact with the L1.
//! Sequencer nodes must read from the L1 to populate Espresso block metadata such as the L1 chain
//! height, which is used to facilitate bridging between the L1 and any rollups which are running on
//! the sequencer.
//!
//! This client runs asynchronously, updating an in-memory snapshot of the relevant L1 information
//! each time a new L1 block is published. This design as a few advantages:
//! * The L1 client is not synchronized with or triggered by HotShot consensus. It can run in pace
//!   with the L1, which makes it easy to use a subscription instead of polling for new blocks,
//!   vastly reducing the number of L1 RPC calls we make.
//! * HotShot block building does not interact directly with the L1; it simply reads the latest
//!   snapshot from the client's memory. This means that block production is instant and infallible.
//!   Any failures or delays in interacting with the L1 will just slow the updating of the L1
//!   snapshot, which will cause the block builder to propose with a slightly old snapshot, but they
//!   will still be able to propose on time.

use crate::state::FeeInfo;
use async_std::task::sleep;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::fee_contract::FeeContract;
use ethers::prelude::*;
use futures::{
    join,
    stream::{self, StreamExt},
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::{min, Ordering},
    sync::Arc,
    time::Duration,
};
use url::Url;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub number: u64,
    pub timestamp: U256,
    pub hash: H256,
}

impl PartialOrd for L1BlockInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for L1BlockInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1Snapshot {
    /// The relevant snapshot of the L1 includes a reference to the current head of the L1 chain.
    ///
    /// Note that the L1 head is subject to changing due to a reorg. However, no reorg will change
    /// the _number_ of this block in the chain: L1 block numbers will always be sequentially
    /// increasing. Therefore, the sequencer does not have to worry about reorgs invalidating this
    /// snapshot.
    pub head: u64,

    /// The snapshot also includes information about the latest finalized L1 block.
    ///
    /// Since this block is finalized (ie cannot be reorged) we can include specific information
    /// about the particular block, such as its hash and timestamp.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub finalized: Option<L1BlockInfo>,
}

impl Committable for L1BlockInfo {
    fn commit(&self) -> Commitment<Self> {
        let mut timestamp = [0u8; 32];
        self.timestamp.to_little_endian(&mut timestamp);

        RawCommitmentBuilder::new(&Self::tag())
            .u64_field("number", self.number)
            // `RawCommitmentBuilder` doesn't have a `u256_field` method, so we simulate it:
            .constant_str("timestamp")
            .fixed_size_bytes(&timestamp)
            .constant_str("hash")
            .fixed_size_bytes(&self.hash.0)
            .finalize()
    }

    fn tag() -> String {
        "L1BLOCK".into()
    }
}

#[derive(Clone, Debug)]
/// An Http Provider and configuration to interact with the L1.
pub struct L1Client {
    retry_delay: Duration,
    /// `Provider` from `ethers-provider`.
    provider: Arc<Provider<Http>>,
    /// Maximum number of L1 blocks that can be scanned for events in a single query.
    events_max_block_range: u64,
}

impl L1Client {
    /// Instantiate an `L1Client` for a given `Url`.
    pub fn new(url: Url, events_max_block_range: u64) -> Self {
        let provider = Arc::new(Provider::new(Http::new(url)));
        Self {
            retry_delay: Duration::from_secs(1),
            provider,
            events_max_block_range,
        }
    }
    /// Get a snapshot from the l1.
    pub async fn snapshot(&self) -> L1Snapshot {
        let (head, finalized) = join!(self.get_block_number(), self.get_finalized_block());
        L1Snapshot { head, finalized }
    }

    /// Get information about the given block.
    ///
    /// If the desired block number is not finalized yet, this function will block until it becomes
    /// finalized.
    pub async fn wait_for_finalized_block(&self, number: u64) -> L1BlockInfo {
        let interval = self.provider.get_interval();

        // Wait for the block to finalize.
        let finalized = loop {
            let Some(block) = self.get_finalized_block().await else {
                tracing::info!("waiting for finalized block");
                sleep(interval).await;
                continue;
            };
            if block.number >= number {
                break block;
            }
            tracing::info!(current_finalized = %block.number, "waiting for finalized block");
            sleep(interval).await;
            continue;
        };

        if finalized.number == number {
            return finalized;
        }

        // The finalized block may have skipped over the block of interest. In this case, our block
        // is still finalized, since it is before the finalized block. We just need to fetch it.
        loop {
            let block = match self.provider.get_block(number).await {
                Ok(Some(block)) => block,
                Ok(None) => {
                    tracing::error!(number, "no such block");
                    sleep(interval).await;
                    continue;
                }
                Err(err) => {
                    tracing::error!(%err, number, "failed to get L1 block");
                    sleep(interval).await;
                    continue;
                }
            };
            let Some(hash) = block.hash else {
                tracing::error!(number, ?block, "L1 block has no hash");
                sleep(interval).await;
                continue;
            };
            break L1BlockInfo {
                number,
                hash,
                timestamp: block.timestamp,
            };
        }
    }

    /// Proxy to `Provider.get_block_number`.
    async fn get_block_number(&self) -> u64 {
        loop {
            match self.provider.get_block_number().await {
                Ok(n) => return n.as_u64(),
                Err(e) => {
                    tracing::warn!("Blocknumber error: {}", e);
                    sleep(self.retry_delay).await;
                }
            }
        }
    }
    /// Proxy to `get_finalized_block`.
    async fn get_finalized_block(&self) -> Option<L1BlockInfo> {
        loop {
            match get_finalized_block(&self.provider).await {
                Ok(block) => return block,
                Err(e) => {
                    tracing::warn!("Finalized block error: {}", e);
                    sleep(self.retry_delay).await;
                }
            }
        }
    }
    /// Get fee info for each `Deposit` occurring between `prev`
    /// and `new`. Returns `Vec<FeeInfo>`
    pub async fn get_finalized_deposits(
        &self,
        fee_contract_address: Address,
        prev_finalized: Option<u64>,
        new_finalized: u64,
    ) -> Vec<FeeInfo> {
        // No new blocks have been finalized, therefore there are no
        // new deposits.
        if prev_finalized >= Some(new_finalized) {
            return vec![];
        }

        // `prev` should have already been processed unless we
        // haven't processed *any* blocks yet.
        let prev = prev_finalized.map(|prev| prev + 1).unwrap_or(0);

        // Divide the range `prev_finalized..=new_finalized` into chunks of size
        // `events_max_block_range`.
        let mut start = prev;
        let end = new_finalized;
        let chunk_size = self.events_max_block_range;
        let chunks = std::iter::from_fn(move || {
            let chunk_end = min(start + chunk_size - 1, end);
            if chunk_end < start {
                return None;
            }

            let chunk = (start, chunk_end);
            start = chunk_end + 1;
            Some(chunk)
        });

        // Fetch events for each chunk.
        let events = stream::iter(chunks).then(|(from, to)| {
            let retry_delay = self.retry_delay;
            let fee_contract = FeeContract::new(fee_contract_address, self.provider.clone());
            async move {
                tracing::debug!(from, to, "fetch events in range");

                // query for deposit events, loop until successful.
                loop {
                    match fee_contract
                        .deposit_filter()
                        .address(fee_contract.address().into())
                        .from_block(from)
                        .to_block(to)
                        .query()
                        .await
                    {
                        Ok(events) => break stream::iter(events),
                        Err(err) => {
                            tracing::warn!(from, to, %err, "Fee Event Error");
                            sleep(retry_delay).await;
                        }
                    }
                }
            }
        });
        events.flatten().map(FeeInfo::from).collect().await
    }
}

async fn get_finalized_block<P: JsonRpcClient>(
    rpc: &Provider<P>,
) -> Result<Option<L1BlockInfo>, ProviderError> {
    let Some(block) = rpc.get_block(BlockNumber::Finalized).await? else {
        // This can happen in rare cases where the L1 chain is very young and has not finalized a
        // block yet. This is more common in testing and demo environments. In any case, we proceed
        // with a null L1 block rather than wait for the L1 to finalize a block, which can take a
        // long time.
        tracing::warn!("no finalized block yet");
        return Ok(None);
    };

    // The number and hash _should_ both exists: they exist unless the block is pending, and the
    // finalized block cannot be pending, unless there has been a catastrophic reorg of the
    // finalized prefix of the L1 chain.
    let number = block
        .number
        .ok_or_else(|| ProviderError::CustomError("finalized block has no number".into()))?;
    let hash = block
        .hash
        .ok_or_else(|| ProviderError::CustomError("finalized block has no hash".into()))?;

    Ok(Some(L1BlockInfo {
        number: number.as_u64(),
        timestamp: block.timestamp,
        hash,
    }))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::NodeState;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use contract_bindings::fee_contract::FeeContract;
    use ethers::utils::{parse_ether, Anvil};
    use std::ops::Add;

    #[async_std::test]
    async fn test_l1_block_fetching() -> anyhow::Result<()> {
        setup_logging();
        setup_backtrace();

        // Test l1_client methods against `ethers::Provider`. There is
        // also some sanity testing demonstrating `Anvil` availability.
        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = L1Client::new(anvil.endpoint().parse().unwrap(), 1);
        let provider = &l1_client.provider;

        let version = provider.client_version().await.unwrap();
        assert_eq!("anvil/v0.2.0", version);

        // Test that nothing funky is happening to the provider when
        // passed along in state.
        let state = NodeState::mock().with_l1(L1Client::new(anvil.endpoint().parse().unwrap(), 1));
        let version = state.l1_client().provider.client_version().await.unwrap();
        assert_eq!("anvil/v0.2.0", version);

        // compare response of underlying provider w/ `get_block_number`
        let expected_head = provider.get_block_number().await.unwrap().as_u64();
        let head = l1_client.get_block_number().await;
        assert_eq!(expected_head, head);

        // compare response of underlying provider w/ `get_finalized_block`
        let expected_finalized = provider.get_block(BlockNumber::Finalized).await.unwrap();
        let finalized = l1_client.get_finalized_block().await.unwrap();

        assert_eq!(expected_finalized.unwrap().hash.unwrap(), finalized.hash);

        // If we drop `anvil` the same request will fail.
        drop(anvil);
        provider.client_version().await.unwrap_err();

        Ok(())
    }

    #[async_std::test]
    async fn test_get_finalized_deposits() -> anyhow::Result<()> {
        setup_logging();
        setup_backtrace();

        // how many deposits will we make
        let deposits = 5;
        let deploy_txn_count = 2;

        let anvil = Anvil::new().spawn();
        let wallet_address = anvil.addresses().first().cloned().unwrap();
        let l1_client = L1Client::new(anvil.endpoint().parse().unwrap(), 1);
        let wallet: LocalWallet = anvil.keys()[0].clone().into();

        // In order to deposit we need a provider that can sign.
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
        let client = Arc::new(client);

        // Initialize a contract with some deposits

        // deploy the fee contract
        let fee_contract =
            contract_bindings::fee_contract::FeeContract::deploy(Arc::new(client.clone()), ())
                .unwrap()
                .send()
                .await?;

        // prepare the initialization data to be sent with the proxy when the proxy is deployed
        let initialize_data = fee_contract
            .initialize(wallet_address) // Here, you simulate the call to get the transaction data without actually sending it.
            .calldata()
            .expect("Failed to encode initialization data");

        // deploy the proxy contract and set the implementation address as the address of the fee contract and send the initialization data
        let proxy_contract = contract_bindings::erc1967_proxy::ERC1967Proxy::deploy(
            client.clone(),
            (fee_contract.address(), initialize_data),
        )
        .unwrap()
        .send()
        .await?;

        // cast the proxy to be of type fee contract so that we can interact with the implementation methods via the proxy
        let fee_contract_proxy = FeeContract::new(proxy_contract.address(), client.clone());

        // confirm that the owner of the contract is the address that was sent as part of the initialization data
        let owner = fee_contract_proxy.owner().await;
        assert_eq!(owner.unwrap(), wallet_address.clone());

        // Anvil will produce a bock for every transaction.
        let head = l1_client.get_block_number().await;
        // there are two transactions, deploying the implementation contract, FeeContract, and deploying the proxy contract
        assert_eq!(deploy_txn_count, head);

        // make some deposits.
        for n in 1..=deposits {
            // Varied amounts are less boring.
            let amount = n as f32 / 10.0;
            let receipt = fee_contract_proxy
                .deposit(wallet_address)
                .value(parse_ether(amount).unwrap())
                .send()
                .await?
                .await?;

            // Successful transactions have `status` of `1`.
            assert_eq!(Some(U64::from(1)), receipt.clone().unwrap().status);
        }

        let head = l1_client.get_block_number().await;
        // Anvil will produce a block for every transaction.
        assert_eq!(deposits + deploy_txn_count, head);

        // Use non-signing `L1Client` to retrieve data.
        let l1_client = L1Client::new(anvil.endpoint().parse().unwrap(), 1);
        // Set prev deposits to `None` so `Filter` will start at block
        // 0. The test would also succeed if we pass `0` (b/c first
        // block did not deposit).
        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                None,
                deposits + deploy_txn_count,
            )
            .await;

        assert_eq!(deposits as usize, pending.len(), "{pending:?}");
        assert_eq!(&wallet_address, &pending[0].account().into());
        assert_eq!(
            U256::from(1500000000000000000u64),
            pending.iter().fold(U256::from(0), |total, info| total
                .add(U256::from(info.amount())))
        );

        // check a few more cases
        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(0),
                deposits + deploy_txn_count,
            )
            .await;
        assert_eq!(deposits as usize, pending.len());

        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(0), 0)
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(0), 1)
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(deploy_txn_count),
                deploy_txn_count,
            )
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(deploy_txn_count),
                deploy_txn_count + 1,
            )
            .await;
        assert_eq!(1, pending.len());

        // what happens if `new_finalized` is `0`?
        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(deploy_txn_count), 0)
            .await;
        assert_eq!(0, pending.len());

        Ok(())
    }

    #[async_std::test]
    async fn test_wait_for_finalized_block() {
        setup_logging();
        setup_backtrace();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = L1Client::new(anvil.endpoint().parse().unwrap(), 1);
        let provider = &l1_client.provider;

        // Wait for a block 10 blocks in the future.
        let block_height = provider.get_block_number().await.unwrap().as_u64();
        let block = l1_client.wait_for_finalized_block(block_height + 10).await;
        assert_eq!(block.number, block_height + 10);

        // Compare against underlying provider.
        let true_block = provider
            .get_block(block_height + 10)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(block.timestamp, true_block.timestamp);
        assert_eq!(block.hash, true_block.hash.unwrap());
    }
}
