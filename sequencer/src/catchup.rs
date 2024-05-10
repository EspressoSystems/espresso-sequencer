use crate::{
    api::{data_source::CatchupDataSource, AccountQueryData, BlocksFrontier},
    persistence::PersistenceOptions,
    state::{BlockMerkleTree, FeeAccount, FeeMerkleCommitment},
};
use anyhow::{bail, Context};
use async_std::sync::RwLock;
use async_trait::async_trait;
use derive_more::From;
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_merkle_tree::{prelude::MerkleNode, ForgetableMerkleTreeScheme, MerkleTreeScheme};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc, time::Duration};
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersionType;

// This newtype is probably not worth having. It's only used to be able to log
// URLs before doing requests.
#[derive(Debug, Clone)]
struct Client<ServerError, Ver: StaticVersionType> {
    inner: surf_disco::Client<ServerError, Ver>,
    url: Url,
}

impl<Ver: StaticVersionType> Client<ServerError, Ver> {
    pub fn new(url: Url) -> Self {
        Self {
            inner: surf_disco::Client::new(url.clone()),
            url,
        }
    }

    pub fn get<T: DeserializeOwned>(&self, route: &str) -> Request<T, ServerError, Ver> {
        self.inner.get(route)
    }
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

/// A catchup implementation that falls back to a remote provider, but prefers a local provider when
/// supported.
pub(crate) async fn local_and_remote(
    local_opt: impl PersistenceOptions,
    remote: impl StateCatchup + 'static,
) -> Arc<dyn StateCatchup> {
    match local_opt.create_catchup_provider().await {
        Ok(local) => Arc::new(vec![local, Arc::new(remote)]),
        Err(err) => {
            tracing::warn!("not using local catchup: {err:#}");
            Arc::new(remote)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StatePeers<Ver: StaticVersionType> {
    clients: Vec<Client<ServerError, Ver>>,
    interval: Duration,
}

impl<Ver: StaticVersionType> StatePeers<Ver> {
    pub fn from_urls(urls: Vec<Url>) -> Self {
        if urls.is_empty() {
            panic!("Cannot create StatePeers with no peers");
        }

        Self {
            clients: urls.into_iter().map(Client::new).collect(),
            interval: Duration::from_secs(1),
        }
    }
}

#[async_trait]
impl<Ver: StaticVersionType> StateCatchup for StatePeers<Ver> {
    #[tracing::instrument(skip(self))]
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        for client in self.clients.iter() {
            tracing::info!("Fetching account {account:?} from {}", client.url);
            match client
                .get::<AccountQueryData>(&format!(
                    "catchup/{height}/{}/account/{account}",
                    view.get_u64(),
                ))
                .send()
                .await
            {
                Ok(res) => match res.proof.verify(&fee_merkle_tree_root) {
                    Ok(_) => return Ok(res),
                    Err(err) => tracing::warn!("Error verifying account proof: {}", err),
                },
                Err(err) => {
                    tracing::warn!("Error fetching account from peer: {}", err);
                }
            }
        }
        bail!("Could not fetch account from any peer");
    }

    #[tracing::instrument(skip(self, mt), height = mt.num_leaves())]
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        for client in self.clients.iter() {
            tracing::info!("Fetching frontier from {}", client.url);
            match client
                .get::<BlocksFrontier>(&format!("catchup/{height}/{}/blocks", view.get_u64()))
                .send()
                .await
            {
                Ok(frontier) => {
                    let Some(elem) = frontier.elem() else {
                        tracing::warn!("Provided frontier is missing leaf element");
                        continue;
                    };
                    match mt.remember(mt.num_leaves() - 1, *elem, &frontier) {
                        Ok(_) => return Ok(()),
                        Err(err) => {
                            tracing::warn!("Error verifying block proof: {}", err);
                            continue;
                        }
                    }
                }
                Err(err) => {
                    tracing::warn!("Error fetching blocks from peer: {}", err);
                }
            }
        }
        bail!("Could not fetch frontier from any peer");
    }

    fn retry_interval(&self) -> Duration {
        self.interval
    }
}

#[derive(Debug, From)]
pub(crate) struct SqlStateCatchup<T> {
    db: Arc<RwLock<T>>,
}

#[async_trait]
impl<T> StateCatchup for SqlStateCatchup<T>
where
    T: CatchupDataSource + Debug + Send + Sync,
{
    #[tracing::instrument(skip(self))]
    async fn try_fetch_account(
        &self,
        block_height: u64,
        view: ViewNumber,
        _fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        self.db
            .read()
            .await
            .get_account(block_height, view, account.into())
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn try_remember_blocks_merkle_tree(
        &self,
        bh: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        if bh == 0 {
            return Ok(());
        }

        let proof = self.db.read().await.get_frontier(bh, view).await?;
        match proof
            .proof
            .first()
            .context(format!("empty proof for frontier at height {bh}"))?
        {
            MerkleNode::Leaf { pos, elem, .. } => mt
                .remember(pos, elem, proof.clone())
                .context("failed to remember proof"),
            _ => bail!("invalid proof"),
        }
    }
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Box<T> {
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        (**self)
            .try_fetch_account(height, view, fee_merkle_tree_root, account)
            .await
    }

    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self)
            .try_remember_blocks_merkle_tree(height, view, mt)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(height, view, mt).await
    }
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Arc<T> {
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        (**self)
            .try_fetch_account(height, view, fee_merkle_tree_root, account)
            .await
    }

    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self)
            .try_remember_blocks_merkle_tree(height, view, mt)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(height, view, mt).await
    }
}

/// Catchup from multiple providers tries each provider in a round robin fashion until it succeeds.
#[async_trait]
impl<T: StateCatchup> StateCatchup for Vec<T> {
    #[tracing::instrument(skip(self))]
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        for provider in self {
            match provider
                .try_fetch_account(height, view, fee_merkle_tree_root, account)
                .await
            {
                Ok(account) => return Ok(account),
                Err(err) => {
                    tracing::warn!(%account, ?provider, "failed to fetch account: {err:#}");
                }
            }
        }

        bail!("could not fetch account from any provider");
    }

    #[tracing::instrument(skip(self, mt))]
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        for provider in self {
            match provider
                .try_remember_blocks_merkle_tree(height, view, mt)
                .await
            {
                Ok(()) => return Ok(()),
                Err(err) => {
                    tracing::warn!(?provider, "failed to fetch frontier: {err:#}");
                }
            }
        }

        bail!("could not fetch account from any provider");
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod mock {
    use super::*;
    use crate::state::{FeeAccountProof, ValidatedState};
    use jf_merkle_tree::MerkleTreeScheme;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Default)]
    pub struct MockStateCatchup {
        state: HashMap<ViewNumber, Arc<ValidatedState>>,
    }

    impl FromIterator<(ViewNumber, Arc<ValidatedState>)> for MockStateCatchup {
        fn from_iter<I: IntoIterator<Item = (ViewNumber, Arc<ValidatedState>)>>(iter: I) -> Self {
            Self {
                state: iter.into_iter().collect(),
            }
        }
    }

    #[async_trait]
    impl StateCatchup for MockStateCatchup {
        async fn try_fetch_account(
            &self,
            _height: u64,
            view: ViewNumber,
            fee_merkle_tree_root: FeeMerkleCommitment,
            account: FeeAccount,
        ) -> anyhow::Result<AccountQueryData> {
            let src = &self.state[&view].fee_merkle_tree;
            assert_eq!(src.commitment(), fee_merkle_tree_root);

            tracing::info!("catchup: fetching account {account:?} for view {view:?}");
            Ok(FeeAccountProof::prove(src, account.into())
                .unwrap_or_else(|| panic!("Account {account:?} not in memory"))
                .into())
        }

        async fn try_remember_blocks_merkle_tree(
            &self,
            _height: u64,
            view: ViewNumber,
            mt: &mut BlockMerkleTree,
        ) -> anyhow::Result<()> {
            tracing::info!("catchup: fetching frontier for view {view:?}");
            let src = &self.state[&view].block_merkle_tree;

            assert_eq!(src.commitment(), mt.commitment());
            assert!(
                src.num_leaves() > 0,
                "catchup should not be triggered when blocks tree is empty"
            );

            let index = src.num_leaves() - 1;
            let (elem, proof) = src.lookup(index).expect_ok().unwrap();
            mt.remember(index, elem, proof.clone())
                .expect("Proof verifies");

            Ok(())
        }
    }
}
