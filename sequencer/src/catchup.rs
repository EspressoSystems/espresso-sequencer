use crate::{
    api::{data_source::CatchupDataSource, AccountQueryData, BlocksFrontier},
    options::{parse_duration, Ratio},
    persistence::PersistenceOptions,
    state::{BlockMerkleTree, FeeAccount, FeeMerkleCommitment},
    ChainConfig,
};
use anyhow::{bail, Context};
use async_std::{sync::RwLock, task::sleep};
use async_trait::async_trait;
use clap::Parser;
use committable::Commitment;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_merkle_tree::{prelude::MerkleNode, ForgetableMerkleTreeScheme, MerkleTreeScheme};
use rand::Rng;
use serde::de::DeserializeOwned;
use std::{cmp::min, fmt::Debug, sync::Arc, time::Duration};
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersionType;

#[derive(Clone, Copy, Debug, Parser, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackoffParams {
    /// Exponential backoff exponent.
    #[clap(
        long = "catchup-backoff-factor",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BACKOFF_FACTOR",
        default_value = "4"
    )]
    factor: u32,

    /// Exponential backoff base delay.
    #[clap(
        long = "catchup-base-retry-delay",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BASE_RETRY_DELAY",
        default_value = "20ms",
        value_parser = parse_duration
    )]
    base: Duration,

    /// Exponential max delay.
    #[clap(
        long = "catchup-max-retry-delay",
        env = "ESPRESSO_SEQUENCER_CATCHUP_MAX_RETRY_DELAY",
        default_value = "5s",
        value_parser = parse_duration
    )]
    max: Duration,

    /// Exponential backoff jitter as a ratio of the backoff delay, numerator:denominator.
    #[clap(
        long = "catchup-backoff-jitter",
        env = "ESPRESSO_SEQUENCER_CATCHUP_BACKOFF_JITTER",
        default_value = "1:10"
    )]
    jitter: Ratio,
}

impl Default for BackoffParams {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

impl BackoffParams {
    async fn retry<S, T>(
        &self,
        mut state: S,
        f: impl for<'a> Fn(&'a mut S) -> BoxFuture<'a, anyhow::Result<T>>,
    ) -> T {
        let mut delay = self.base;
        loop {
            match f(&mut state).await {
                Ok(res) => break res,
                Err(err) => {
                    tracing::warn!(
                        "Retryable operation failed, will retry after {delay:?}: {err:#}"
                    );
                    sleep(delay).await;
                    delay = self.backoff(delay);
                }
            }
        }
    }

    #[must_use]
    fn backoff(&self, delay: Duration) -> Duration {
        if delay >= self.max {
            return self.max;
        }

        let mut rng = rand::thread_rng();

        // Increase the backoff by the backoff factor.
        let ms = (delay * self.factor).as_millis() as u64;

        // Sample a random jitter factor in the range [0, self.jitter].
        let jitter_num = rng.gen_range(0..self.jitter.numerator);
        let jitter_den = self.jitter.denominator;

        // Increase the delay by the jitter factor.
        let jitter = ms * jitter_num / jitter_den;
        let delay = Duration::from_millis(ms + jitter);

        // Bound the delay by the maximum.
        min(delay, self.max)
    }
}

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
            let account = self
                .backoff()
                .retry(self, |provider| {
                    provider
                        .try_fetch_account(height, view, fee_merkle_tree_root, account)
                        .map_err(|err| err.context("fetching account {account}"))
                        .boxed()
                })
                .await;
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
        self.backoff()
            .retry(mt, |mt| {
                self.try_remember_blocks_merkle_tree(height, view, mt)
                    .map_err(|err| err.context("fetching frontier"))
                    .boxed()
            })
            .await;
        Ok(())
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig>;

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        self.backoff()
            .retry(self, |provider| {
                provider
                    .try_fetch_chain_config(commitment)
                    .map_err(|err| err.context("fetching chain config"))
                    .boxed()
            })
            .await
    }

    fn backoff(&self) -> &BackoffParams;
}

/// A catchup implementation that falls back to a remote provider, but prefers a local provider when
/// supported.
pub(crate) async fn local_and_remote(
    local_opt: impl PersistenceOptions,
    remote: impl StateCatchup + 'static,
) -> Arc<dyn StateCatchup> {
    match local_opt.create_catchup_provider(*remote.backoff()).await {
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
    backoff: BackoffParams,
}

impl<Ver: StaticVersionType> StatePeers<Ver> {
    pub fn from_urls(urls: Vec<Url>, backoff: BackoffParams) -> Self {
        if urls.is_empty() {
            panic!("Cannot create StatePeers with no peers");
        }

        Self {
            clients: urls.into_iter().map(Client::new).collect(),
            backoff,
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
                    view.u64(),
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
                .get::<BlocksFrontier>(&format!("catchup/{height}/{}/blocks", view.u64()))
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        for client in self.clients.iter() {
            tracing::info!("Fetching chain config from {}", client.url);
            match client
                .get::<ChainConfig>(&format!("catchup/chain-config/{}", commitment))
                .send()
                .await
            {
                Ok(cf) => {
                    return Ok(cf);
                }
                Err(err) => {
                    tracing::warn!("Error fetching chain config from peer: {}", err);
                }
            }
        }
        bail!("Could not fetch chain config from any peer");
    }

    fn backoff(&self) -> &BackoffParams {
        &self.backoff
    }
}

#[derive(Debug)]
pub(crate) struct SqlStateCatchup<T> {
    db: Arc<RwLock<T>>,
    backoff: BackoffParams,
}

impl<T> SqlStateCatchup<T> {
    pub(crate) fn new(db: Arc<RwLock<T>>, backoff: BackoffParams) -> Self {
        Self { db, backoff }
    }
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.db.read().await.get_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        &self.backoff
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        (**self).try_fetch_chain_config(commitment).await
    }

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        (**self).fetch_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        (**self).backoff()
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        (**self).try_fetch_chain_config(commitment).await
    }

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        (**self).fetch_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        (**self).backoff()
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        for provider in self {
            match provider.try_fetch_chain_config(commitment).await {
                Ok(cf) => return Ok(cf),
                Err(err) => {
                    tracing::warn!(?provider, "failed to fetch chain config: {err:#}");
                }
            }
        }

        bail!("could not fetch chain config from any provider");
    }

    fn backoff(&self) -> &BackoffParams {
        // Use whichever provider's backoff is most conservative.
        self.iter()
            .map(|p| p.backoff())
            .max()
            .expect("provider list not empty")
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
        backoff: BackoffParams,
        state: HashMap<ViewNumber, Arc<ValidatedState>>,
    }

    impl FromIterator<(ViewNumber, Arc<ValidatedState>)> for MockStateCatchup {
        fn from_iter<I: IntoIterator<Item = (ViewNumber, Arc<ValidatedState>)>>(iter: I) -> Self {
            Self {
                backoff: Default::default(),
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

        async fn try_fetch_chain_config(
            &self,
            _commitment: Commitment<ChainConfig>,
        ) -> anyhow::Result<ChainConfig> {
            Ok(ChainConfig::default())
        }

        fn backoff(&self) -> &BackoffParams {
            &self.backoff
        }
    }
}
