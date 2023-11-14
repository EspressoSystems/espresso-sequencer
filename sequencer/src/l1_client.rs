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

use async_std::{
    sync::{Arc, RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard},
    task::{sleep, spawn},
};
use commit::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub number: u64,
    pub timestamp: U256,
    pub hash: H256,
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
pub struct L1ClientOptions {
    url: Url,
    finalized_block_tag: BlockNumber,
    retry_delay: Duration,
}

impl L1ClientOptions {
    pub fn with_url(url: Url) -> Self {
        Self {
            url,
            finalized_block_tag: BlockNumber::Finalized,
            retry_delay: Duration::from_secs(1),
        }
    }

    pub fn with_latest_block_tag(mut self) -> Self {
        // For testing with a pre-merge geth node that does not support the finalized block tag we
        // allow setting the client to use the latest block instead.
        self.finalized_block_tag = BlockNumber::Latest;
        self
    }

    pub async fn start(self) -> Result<L1Client, ProviderError> {
        let rpc = Provider::connect(self.url.clone()).await?;

        // Fetch the initial L1 snapshot now, blocking, so that we will have a reasonable baseline
        // at least until the background task starts updating the snapshot.
        let snapshot = L1Snapshot {
            head: rpc.get_block_number().await?.as_u64(),
            finalized: get_finalized_block(&rpc, self.finalized_block_tag).await?,
        };
        tracing::info!("L1 client initialized with snapshot {snapshot:?}");

        let client = L1Client {
            latest_snapshot: Arc::new(RwLock::new(snapshot)),
        };

        // Spawn a background task to update the in-memory snapshot whenever a new L1 block is
        // produced.
        spawn(update_loop(
            self.url,
            self.finalized_block_tag,
            self.retry_delay,
            client.latest_snapshot.clone(),
        ));

        Ok(client)
    }
}

#[derive(Clone, Debug)]
pub struct L1Client {
    latest_snapshot: Arc<RwLock<L1Snapshot>>,
}

impl L1Client {
    pub async fn snapshot(&self) -> L1Snapshot {
        *self.latest_snapshot.read().await
    }
}

async fn update_loop(
    url: Url,
    finalized_tag: BlockNumber,
    retry_delay: Duration,
    snapshot: Arc<RwLock<L1Snapshot>>,
) {
    'reset_connection: loop {
        // Subscribe to new blocks. This task cannot fail; retry until we succeed.
        let rpc = match Provider::connect(url.clone()).await {
            Ok(rpc) => rpc,
            Err(err) => {
                tracing::error!("error connecting to RPC {url}: {err}");
                sleep(retry_delay).await;
                continue;
            }
        };
        let mut block_stream = match rpc.subscribe_blocks().await {
            Ok(stream) => stream,
            Err(err) => {
                tracing::error!("error subscribing to L1 blocks: {err}");
                sleep(retry_delay).await;
                continue;
            }
        };

        while let Some(head) = block_stream.next().await {
            let Some(head) = head.number else {
                // This shouldn't happen, but if it does, it means the block stream has erroneously
                // given us a pending block. We are only interested in committed blocks, so just
                // skip this one.
                tracing::warn!("got block from L1 block stream with no number");
                continue;
            };
            let head = head.as_u64();

            // Take a read lock on the snapshot to check if it needs to be updated.
            let snapshot = snapshot.upgradable_read().await;
            if head <= snapshot.head {
                // We got a notification for a new block which is not newer than the current
                // snapshotted head. This can happen due to an L1 reorg. We are not allowed to
                // propose an L1 block number which is older than that of the previous Espresso
                // block, so we skip this notification.
                //
                // This does mean that we may end up proposing with a block number that is newer
                // than the current L1 head, if the height of the L1 is reduced in a reorg. This is
                // not the best for UX, as rollups will have to block until the new L1 block is
                // produced, but the L1 chain will _eventually_ (and usually quickly) catch up, and
                // this only happens in very rare cases anyways. In such rare cases we prioritize
                // maintaining our invariants (increasing L1 block numbers) over UX.
                tracing::warn!(
                    "got L1 head {head} which is not newer than previous head {}",
                    snapshot.head
                );
                continue;
            }

            // A new block has been produced. This happens fairly rarely, so it is now ok to poll to
            // see if a new block has been finalized.
            let finalized = match get_finalized_block(&rpc, finalized_tag).await {
                Ok(finalized) => Some(finalized),
                Err(err) => {
                    tracing::error!("failed to get finalized block: {err}");

                    // If we cannot get the finalized block for any reason, don't let this stop us
                    // from updating the L1 head. By returning `None` here, we will proceed to
                    // update the head but not the finalized block, and then we will reset the
                    // connection, in the hopes that this fixes whatever went wrong with
                    // `get_finalized_block`.
                    None
                }
            };

            // Update the snapshot.
            let mut snapshot = RwLockUpgradableReadGuard::upgrade(snapshot).await;
            snapshot.head = head;
            if let Some(finalized) = finalized {
                snapshot.finalized = finalized;
            }

            // Drop our exclusive lock before logging anything.
            let snapshot = RwLockWriteGuard::downgrade(snapshot);
            tracing::info!("updated L1 snapshot to {:?}", *snapshot);

            // If we encountered an error in `get_finalized_block`, reset the connection now that we
            // have updated what we can.
            if finalized.is_none() {
                tracing::warn!("resetting connection due to error in get_finalized_block");
                continue 'reset_connection;
            }
        }

        tracing::error!("L1 block stream ended unexpectedly, trying to re-establish");
    }
}

async fn get_finalized_block<P: JsonRpcClient>(
    rpc: &Provider<P>,
    tag: BlockNumber,
) -> Result<Option<L1BlockInfo>, ProviderError> {
    let Some(block) = rpc.get_block(tag).await? else {
        // This can happen in rare cases where the L1 chain is very young and has not finalized a
        // block yet. This is more common in testing and demo environments. In any case, we proceed
        // with a null L1 block rather than wait for the L1 to finalize a block, which can take a
        // long time.
        tracing::warn!("no finalized block yet");
        return Ok(None);
    };

    // The block number always exists unless the block is pending. The finalized block cannot be
    // pending, unless there has been a catastrophic reorg of the finalized prefix of the L1
    // chain, so it is OK to panic if this happens.
    let number = block.number.expect("finalized block has no number");
    // Same for the hash.
    let hash = block.hash.expect("finalized block has no hash");

    Ok(Some(L1BlockInfo {
        number: number.as_u64(),
        timestamp: block.timestamp,
        hash,
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use futures::channel::oneshot;
    use sequencer_utils::AnvilOptions;
    use std::time::Instant;

    #[async_std::test]
    async fn test_l1_block_scanning() {
        setup_logging();
        setup_backtrace();

        let test_duration = Duration::from_secs(30);
        let test_interval = Duration::from_millis(500);
        let anvil = AnvilOptions::default()
            .block_time(Duration::from_secs(1))
            .spawn()
            .await;
        let l1_client = L1ClientOptions::with_url(anvil.ws_url())
            .start()
            .await
            .unwrap();
        let anvil_client = Provider::try_from(anvil.url().to_string()).unwrap();

        let mut prev_head = 0;
        let mut prev_finalized: Option<L1BlockInfo> = None;

        let start = Instant::now();
        while start.elapsed() < test_duration {
            let snapshot = l1_client.snapshot().await;

            let expected_head = anvil_client.get_block_number().await.unwrap().as_u64();
            let expected_finalized = anvil_client
                .get_block(BlockNumber::Finalized)
                .await
                .unwrap();

            // The snapshotted head should be within one of the observed head, it may be one block
            // behind if the head just changed and the snapshot hasn't updated yet.
            assert!(
                expected_head == snapshot.head || expected_head == snapshot.head + 1,
                "snapshotted head {} does not match expected head {}",
                snapshot.head,
                expected_head
            );

            // And same for the finalized block, unless there is no finalized block.
            if let Some(expected_finalized) = expected_finalized {
                let expected_finalized_number = expected_finalized.number.unwrap().as_u64();
                let snapshot_finalized = snapshot.finalized.unwrap();
                assert!(
                    expected_finalized_number == snapshot_finalized.number
                        || expected_finalized_number == snapshot_finalized.number + 1,
                    "snapshotted finalized {} does not match expected finalized {}",
                    snapshot_finalized.number,
                    expected_finalized_number
                );
                if expected_finalized_number == snapshot_finalized.number {
                    assert_eq!(expected_finalized.hash.unwrap(), snapshot_finalized.hash);
                    assert_eq!(expected_finalized.timestamp, snapshot_finalized.timestamp);
                    tracing::info!("L1 finalized block fully synchronized");
                }
            } else {
                tracing::info!("no finalized L1 block yet");
                assert_eq!(snapshot.finalized, None);
            }

            // Check invariants: monitonically increasing L1 blocks.
            assert!(snapshot.head >= prev_head);
            if let Some(finalized) = snapshot.finalized {
                if let Some(prev_finalized) = prev_finalized {
                    assert!(finalized.number >= prev_finalized.number);
                }
            } else {
                assert_eq!(prev_finalized, None);
            }

            prev_head = snapshot.head;
            prev_finalized = snapshot.finalized;

            sleep(test_interval).await;
        }
    }

    #[async_std::test]
    async fn test_reorg_increasing_block() {
        setup_logging();
        setup_backtrace();

        let mut anvil = AnvilOptions::default()
            .block_time(Duration::from_secs(1))
            .spawn()
            .await;
        let l1_client = L1ClientOptions::with_url(anvil.ws_url())
            .start()
            .await
            .unwrap();

        // Spawn a task to check that updates to the snapshotted state are always increasing.
        let (kill_safety_task, mut killed) = oneshot::channel();
        let l1_client_clone = l1_client.clone();
        let safety_task = spawn(async move {
            let l1_client = l1_client_clone;
            let mut prev_head = l1_client.snapshot().await.head;
            loop {
                if !matches!(killed.try_recv(), Ok(None)) {
                    // The task was killed or the other end of the channel was dropped.
                    return Ok(());
                }

                // Ensure the snapshot has not decreased.
                let head = l1_client.snapshot().await.head;
                if head < prev_head {
                    return Err(format!("head decreased from {prev_head} to {head}"));
                }
                prev_head = head;
                sleep(Duration::from_millis(500)).await;
            }
        });

        // Force an L1 reorg. Make it several blocks deep to be sure the L1 client task doesn't miss
        // it while reconnecting.
        anvil.reorg(5).await;

        // Let the safety task run for a few more seconds to ensure we recover from the reorg and
        // continue to make progress.
        let snapshot = l1_client.snapshot().await.head;
        sleep(Duration::from_secs(5)).await;

        // Join the safety task and check success.
        kill_safety_task.send(()).unwrap();
        safety_task.await.unwrap();
        assert!(
            l1_client.snapshot().await.head > snapshot,
            "L1 client is not making progress"
        );
    }
}
