use std::{fmt::Debug, sync::Arc};

use anyhow::{bail, Context};
use async_std::sync::RwLock;
use async_trait::async_trait;
use committable::Commitment;
use derive_more::From;
use espresso_types::{
    traits::PersistenceOptions, v0_3::StateCatchup, AccountQueryData, BlockMerkleTree, ChainConfig,
    FeeAccount, FeeMerkleCommitment,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_merkle_tree::{prelude::MerkleNode, ForgetableMerkleTreeScheme, MerkleTreeScheme};
use serde::de::DeserializeOwned;
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersionType;

use crate::api::{data_source::CatchupDataSource, BlocksFrontier};

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
}

impl<Ver: StaticVersionType> StatePeers<Ver> {
    pub fn from_urls(urls: Vec<Url>) -> Self {
        if urls.is_empty() {
            panic!("Cannot create StatePeers with no peers");
        }

        Self {
            clients: urls.into_iter().map(Client::new).collect(),
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

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.db.read().await.get_chain_config(commitment).await
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod mock {
    use std::collections::HashMap;

    use espresso_types::{FeeAccountProof, ValidatedState};
    use jf_merkle_tree::MerkleTreeScheme;

    use super::*;

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

        async fn try_fetch_chain_config(
            &self,
            _commitment: Commitment<ChainConfig>,
        ) -> anyhow::Result<ChainConfig> {
            Ok(ChainConfig::default())
        }
    }
}
