use crate::{
    block::Block,
    chain_variables::ChainVariables,
    l1_client::{L1Client, L1ClientOptions, L1Snapshot},
    Error, Transaction,
};
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable};
use ethers::prelude::BlockNumber;
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
    ethereum_block_tag: BlockNumber,
}

impl HotShotState for State {
    type Error = Error;

    type BlockType = Block;

    type Time = ViewNumber;

    fn next_block(_prev_state: Option<Self>) -> Self::BlockType {
        // The HotShot APIs should be redesigned so that
        // * they are async
        // * new blocks being created have access to the application state, which in our case could
        //   contain an already connected L1 client.
        // For now, as a workaround, we will create a new L1 client based on environment variables
        // and use `block_on` to query it.
        let L1Snapshot { finalized, head } = if let Some(l1_client) = &*L1_CLIENT {
            block_on(l1_client.snapshot())
        } else {
            // For unit testing, we may disable the L1 client and use mock L1 blocks instead.
            L1Snapshot {
                finalized: None,
                head: 0,
            }
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
    pub(crate) static ref L1_CLIENT: Option<L1Client> = {
        let Ok(url) = env::var("ESPRESSO_SEQUENCER_L1_WS_PROVIDER") else {
            #[cfg(any(test, feature = "testing"))]
            {
                tracing::warn!("ESPRESSO_SEQUENCER_L1_WS_PROVIDER is not set. Using mock L1 block numbers. This is suitable for testing but not production.");
                return None;
            }
            #[cfg(not(any(test, feature = "testing")))]
            {
                panic!("ESPRESSO_SEQUENCER_L1_WS_PROVIDER must be set.");
            }
        };

        let mut opt = L1ClientOptions::with_url(url.parse().unwrap());
        // For testing with a pre-merge geth node that does not support the finalized block tag we
        // allow setting an environment variable to use the latest block instead. This feature is
        // used in the OP devnet which uses the docker images built in this repo. Therefore it's not
        // hidden behind the testing flag.
        if let Ok(val) = env::var("ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG") {
            match val.as_str() {
               "y" | "yes" | "t"|  "true" | "on" | "1"  => {
                    tracing::warn!(
                        "ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG is set. Using latest block tag\
                         instead of finalized block tag. This is suitable for testing but not production."
                    );
                    opt = opt.with_latest_block_tag();
                },
                "n" | "no" | "f" | "false" | "off" | "0" => {}
                _ => panic!("invalid ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG value: {}", val)
            }
        }

        block_on(async move {
            // Starting the client can fail due to transient errors in the L1 RPC. This could make
            // it very annoying to start up the sequencer node, so retry until we succeed.
            loop {
                match opt.clone().start().await {
                    Ok(client) => break Some(client),
                    Err(err) => {
                        tracing::error!("failed to start L1 client, retrying: {err}");
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        })
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
