use crate::{
    block::{Block, L1BlockInfo},
    chain_variables::ChainVariables,
    Error, Transaction,
};
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable};
use ethers::prelude::{BlockNumber, Http, Middleware, Provider};
use futures::future::join;
use hotshot::traits::State as HotShotState;
use hotshot_types::{data::ViewNumber, traits::state::TestableState};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Debug, Display};
use std::time::Duration;

#[derive(Default, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct State {
    chain_variables: ChainVariables,
}

impl HotShotState for State {
    type Error = Error;

    type BlockType = Block;

    type Time = ViewNumber;

    fn next_block(_prev_state: Option<Self>) -> Self::BlockType {
        // The HotShot APIs should be redesigned so that
        // * they are async and fallible
        // * new blocks being created have access to the application state, which in our case could
        //   contain an already connected ETH client.
        // For now, as a workaround, we will create a new ETH client based on environment variables
        // and use `block_on` to query it.
        let (finalized, head) = if let Some(l1_provider) = &*L1_PROVIDER {
            block_on(join(
                get_finalized_l1_block(l1_provider),
                get_l1_head(l1_provider),
            ))
        } else {
            // For unit testing, we may disable the L1 provider and use mock L1 blocks instead.
            (None, 0)
        };

        Block::from_l1(finalized, head)
    }

    fn validate_block(&self, _block: &Self::BlockType, _view_number: &Self::Time) -> bool {
        unimplemented!("Using sequencing consensus, no validation")
    }

    // This function is called exactly once, with the first block.
    fn append(
        &self,
        _block: &Self::BlockType,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }

    fn on_commit(&self) {}
}

lazy_static! {
    static ref L1_PROVIDER: Option<Provider<Http>> = {
        let Ok(url) = env::var("ESPRESSO_SEQUENCER_L1_PROVIDER") else {
            #[cfg(any(test, feature = "testing"))] 
            {
                tracing::warn!("ESPRESSO_SEQUENCER_L1_PROVIDER is not set. Using mock L1 block numbers. This is suitable for testing but not production.");
                return None;
            }
            #[cfg(not(any(test, feature = "testing")))]
            {
                panic!("ESPRESSO_SEQUENCER_L1_PROVIDER must be set.");
            }
        };
        Some(
            url.try_into()
                .expect("invalid ESPRESSO_SEQUENCER_L1_PROVIDER URL"),
        )
    };
}

async fn get_l1_head(l1_provider: &Provider<Http>) -> u64 {
    // This cannot fail, retry until we succeed.
    loop {
        let retry_delay = Duration::from_millis(100);
        let chain_tip_number = match l1_provider.get_block_number().await {
            Ok(number) => number,
            Err(err) => {
                tracing::error!("failed to get block number from L1 provider, retrying ({err})");
                sleep(retry_delay).await;
                continue;
            }
        };
        tracing::debug!("L1 chain tip at block {chain_tip_number}");
        break chain_tip_number.as_u64();
    }
}

async fn get_finalized_l1_block(l1_provider: &Provider<Http>) -> Option<L1BlockInfo> {
    // This cannot fail, retry until we succeed.
    loop {
        let retry_delay = Duration::from_millis(100);
        let block = match l1_provider.get_block(BlockNumber::Finalized).await {
            Ok(Some(block)) => block,
            Ok(None) => {
                // This can happen in rare cases where the L1 chain is very young and has not
                // finalized a block yet. This is more common in testing and demo environments. In
                // any case, we proceed with a null L1 block rather than wait for the L1 to finalize
                // a block, which can take a long time.
                return None;
            }
            Err(err) => {
                tracing::error!("failed to get finalized block from L1 provider, retrying ({err})");
                sleep(retry_delay).await;
                continue;
            }
        };

        // The block number always exists unless the block is pending. The finalized block cannot be
        // pending, unless there has been a catastrophic reorg of the finalized prefix of the L1
        // chain, so it is OK to panic if this happens.
        let number = block.number.expect("finalized block has no number");
        // Same for the hash.
        let hash = block.hash.expect("finalized block has no hash");

        tracing::debug!(
            "proposing with finalized L1 block {number} (timestamp {}, hash {hash})",
            block.timestamp,
        );
        break Some(L1BlockInfo {
            number: number.as_u64(),
            timestamp: block.timestamp,
            hash,
        });
    }
}

// Required for TestableState
#[cfg(any(test, feature = "testing"))]
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(any(test, feature = "testing"))]
impl TestableState for State {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        _padding: u64,
    ) -> <Self::BlockType as hotshot::traits::Block>::Transaction {
        Transaction::random(rng)
    }
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        unimplemented!("Not used in sequencing consensus")
    }
}
