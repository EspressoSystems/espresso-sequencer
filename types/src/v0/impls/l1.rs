use crate::{FeeInfo, L1Client, L1Snapshot};

use super::L1BlockInfo;
use async_std::task::sleep;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::fee_contract::FeeContract;
use ethers::prelude::*;
use ethers::prelude::{H256, U256};
use futures::{
    join,
    stream::{self, StreamExt},
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{cmp::min, sync::Arc, time::Duration};
use url::Url;

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

impl L1BlockInfo {
    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn timestamp(&self) -> U256 {
        self.timestamp
    }

    pub fn hash(&self) -> H256 {
        self.hash
    }
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
