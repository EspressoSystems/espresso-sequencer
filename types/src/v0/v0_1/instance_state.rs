use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context};
use async_trait::async_trait;
use derive_more::From;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_merkle_tree::UniversalMerkleTreeScheme;
use jf_merkle_tree::{prelude::MerkleNode, ForgetableMerkleTreeScheme, MerkleTreeScheme};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

use vbs::version::StaticVersionType;

use crate::{
    BlockMerkleTree, ChainConfig, FeeAccount, FeeMerkleCommitment, FeeMerkleTree, L1BlockInfo,
    ValidatedState,
};

use super::state::AccountQueryData;

#[derive(Debug, Clone)]
pub struct NodeState {
    pub(crate) node_id: u64,
    pub(crate) chain_config: ChainConfig,
    pub(crate) l1_client: L1Client,
    pub(crate) peers: Arc<dyn StateCatchup>,
    pub(crate) genesis_state: ValidatedState,
    pub(crate) l1_genesis: Option<L1BlockInfo>,
}

#[derive(Clone, Debug)]
/// An Http Provider and configuration to interact with the L1.
pub struct L1Client {
    pub(crate) retry_delay: Duration,
    /// `Provider` from `ethers-provider`.
    pub(crate) provider: Arc<Provider<Http>>,
    /// Maximum number of L1 blocks that can be scanned for events in a single query.
    pub(crate) events_max_block_range: u64,
}

#[async_trait]
pub trait StateCatchup: Send + Sync + std::fmt::Debug {
    /// Try to fetch the given account state, failing without retrying if unable.
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData>;

    /// Fetch the given list of accounts, retrying on transient errors.
    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        let mut ret = vec![];
        for account in accounts {
            // Retry until we succeed.
            let account = loop {
                match self
                    .try_fetch_account(height, view, fee_merkle_tree_root, account)
                    .await
                {
                    Ok(account) => break account,
                    Err(err) => {
                        tracing::warn!(%account, "Could not fetch account, retrying: {err:#}");
                        async_std::task::sleep(self.retry_interval()).await;
                    }
                }
            };
            ret.push(account);
        }
        Ok(ret)
    }

    /// Try to fetch and remember the blocks frontier, failing without retrying if unable.
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()>;

    /// Fetch and remember the blocks frontier, retrying on transient errors.
    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        loop {
            match self.try_remember_blocks_merkle_tree(height, view, mt).await {
                Ok(()) => break,
                Err(err) => {
                    tracing::warn!("Could not fetch frontier from any peer, retrying: {err:#}");
                    async_std::task::sleep(self.retry_interval()).await;
                }
            }
        }

        Ok(())
    }

    fn retry_interval(&self) -> Duration {
        Duration::from_millis(100)
    }
}
