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

use async_std::task::sleep;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::*;
use futures::join;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, time::Duration};
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
    provider: Provider<Http>,
}

impl L1Client {
    /// Instantiate an `L1Client` for a given `Url`.
    pub fn new(url: Url) -> Self {
        Self {
            retry_delay: Duration::from_secs(1),
            provider: Provider::new(Http::new(url)),
        }
    }
    /// Get a snapshot from the l1.
    pub async fn snapshot(&self) -> L1Snapshot {
        let (head, finalized) = join!(self.get_block_number(), self.get_finalized_block());
        L1Snapshot { head, finalized }
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
    use ethers::utils::Anvil;

    #[async_std::test]
    async fn test_l1_block_fetching() -> anyhow::Result<()> {
        // Test l1_client methods against `ethers::Provider`. There is
        // also some sanity testing demonstrating `Anvil` availability.
        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = L1Client::new(anvil.endpoint().parse().unwrap());
        let provider = &l1_client.provider;

        let version = provider.client_version().await.unwrap();
        assert_eq!("anvil/v0.2.0", version);

        // Test that nothing funky is happening to the provider when
        // passed along in state.
        let state = NodeState {
            l1_client: L1Client::new(anvil.endpoint().parse().unwrap()),
            ..Default::default()
        };
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

        // If we drop `anvil` the same request will fail and we will
        // get an error as below.
        drop(anvil);
        provider.client_version().await.unwrap_err();

        Ok(())
    }
}
