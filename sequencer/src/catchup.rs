use crate::{
    api::endpoints::{AccountQueryData, BlocksFrontier},
    state::{BlockMerkleTree, FeeAccount, FeeMerkleCommitment},
};
use async_trait::async_trait;
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_primitives::merkle_tree::{ForgetableMerkleTreeScheme, MerkleTreeScheme};
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Duration};
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
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>>;

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()>;
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

    #[tracing::instrument(skip(self))]
    async fn fetch_account(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> AccountQueryData {
        if self.clients.is_empty() {
            panic!("No peers to fetch account from");
        }
        loop {
            for client in self.clients.iter() {
                tracing::info!(
                    "Fetching account {account:?} for view {view:?} from {}",
                    client.url
                );
                match client
                    .get::<AccountQueryData>(&format!(
                        "catchup/{}/account/{account}",
                        view.get_u64(),
                    ))
                    .send()
                    .await
                {
                    Ok(res) => match res.proof.verify(&fee_merkle_tree_root) {
                        Ok(_) => return res,
                        Err(err) => tracing::warn!("Error verifying account proof: {}", err),
                    },
                    Err(err) => {
                        tracing::warn!("Error fetching account from peer: {}", err);
                    }
                }
            }
            tracing::warn!("Could not fetch account from any peer, retrying");
            async_std::task::sleep(self.interval).await;
        }
    }
}

#[async_trait]
impl<Ver: StaticVersionType> StateCatchup for StatePeers<Ver> {
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        let mut ret = vec![];
        for account in accounts {
            tracing::info!("Fetching account {account:?} for view {view:?}");
            ret.push(
                self.fetch_account(view, fee_merkle_tree_root, account)
                    .await,
            )
        }
        Ok(ret)
    }

    #[tracing::instrument(skip(self, mt), height = mt.num_leaves())]
    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        if self.clients.is_empty() {
            panic!("No peers to fetch frontier from");
        }
        loop {
            for client in self.clients.iter() {
                tracing::info!("Fetching frontier from {}", client.url);
                match client
                    .get::<BlocksFrontier>(&format!("catchup/{}/blocks", view.get_u64()))
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
            tracing::warn!("Could not fetch frontier from any peer, retrying");
            async_std::task::sleep(self.interval).await;
        }
    }
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Box<T> {
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(view, mt).await
    }
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Arc<T> {
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(view, mt).await
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod mock {
    use super::*;
    use crate::state::{FeeAccountProof, ValidatedState};
    use jf_primitives::merkle_tree::MerkleTreeScheme;
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
        async fn fetch_accounts(
            &self,
            view: ViewNumber,
            fee_merkle_tree_root: FeeMerkleCommitment,
            accounts: Vec<FeeAccount>,
        ) -> anyhow::Result<Vec<AccountQueryData>> {
            tracing::info!("catchup: fetching account data for view {view:?}");
            let src = &self.state[&view].fee_merkle_tree;
            assert_eq!(src.commitment(), fee_merkle_tree_root);

            accounts
                .into_iter()
                .map(|account| {
                    tracing::info!("catchup: fetching account {account:?} for view {view:?}");
                    Ok(FeeAccountProof::prove(src, account.into())
                        .unwrap_or_else(|| panic!("Account {account:?} not in memory"))
                        .into())
                })
                .collect::<anyhow::Result<_>>()
        }

        async fn remember_blocks_merkle_tree(
            &self,
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
