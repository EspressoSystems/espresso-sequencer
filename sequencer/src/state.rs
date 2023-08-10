use crate::{
    block::{Block, L1BlockInfo},
    chain_variables::ChainVariables,
    Error, Transaction,
};
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable};
use ethers::prelude::{Http, Middleware, Provider};
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
        let l1_block = if let Some(l1_provider) = &*L1_PROVIDER {
            block_on(async move {
                // This cannot fail, retry until we succeed.
                loop {
                    let retry_delay = Duration::from_millis(100);
                    let chain_tip_number = match l1_provider.get_block_number().await {
                        Ok(number) => number,
                        Err(err) => {
                            tracing::error!(
                                "failed to get block number from L1 provider, retrying ({err})"
                            );
                            sleep(retry_delay).await;
                            continue;
                        }
                    };
                    tracing::debug!("L1 chain tip at block {chain_tip_number}");

                    // Use a block with a few confirmations to minimize the probability of reorgs.
                    let number = chain_tip_number.saturating_sub(3.into());

                    let block = match l1_provider.get_block(number).await {
                        Ok(Some(block)) => block,
                        Ok(None) => {
                            // This can only happen if there was a reorg that changed the block
                            // height. Just retry.
                            tracing::error!("L1 block {number} does not exist, retrying");
                            sleep(retry_delay).await;
                            continue;
                        }
                        Err(err) => {
                            tracing::error!(
                                "failed to get block {number} from L1 provider, retrying ({err})"
                            );
                            sleep(retry_delay).await;

                            // Retry the whole loop, including re-fetching the block number. One
                            // reason we might have failed here is if an L1 reorg changed the chain
                            // height, and the block at our current `number` may not become
                            // available again for a while.
                            continue;
                        }
                    };

                    let Some(number) = block.number else {
                        // This can also happen if there's a reorg.
                        tracing::error!("L1 block {number} is not committed, retrying");
                        sleep(retry_delay).await;
                        continue;
                    };

                    tracing::debug!(
                        "proposing with L1 block {number} (timestamp {})",
                        block.timestamp
                    );
                    break L1BlockInfo {
                        number: number.as_u64(),
                        timestamp: block.timestamp,
                    };
                }
            })
        } else {
            // For unit testing, we may disable the L1 provider and use mock L1 blocks instead.
            L1BlockInfo::default()
        };

        Block::from_l1(l1_block)
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
            tracing::warn!("ESPRESSO_SEQUENCER_L1_PROVIDER is not set. Using mock L1 block numbers. This is suitable for testing but not production.");
            return None;
        };
        Some(
            url.try_into()
                .expect("invalid ESPRESSO_SEQUENCER_L1_PROVIDER URL"),
        )
    };
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
