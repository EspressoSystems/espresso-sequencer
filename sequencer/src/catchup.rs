use crate::{
    api::endpoints::{AccountQueryData, BlocksFrontier},
    state::{BlockMerkleTree, FeeAccount, FeeAccountProof, FeeMerkleCommitment},
    Header, ValidatedState,
};
use async_trait::async_trait;
use commit::Commitment;
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime as _};
use jf_primitives::merkle_tree::{ForgetableMerkleTreeScheme, MerkleTreeScheme as _};
use serde::de::DeserializeOwned;
use std::time::Duration;
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;

// This newtype is probably not worth having. It's only used to be able to log
// URLs before doing requests.
#[derive(Debug, Clone)]
struct Client<ServerError> {
    inner: surf_disco::Client<ServerError>,
    url: Url,
}

impl Client<ServerError> {
    pub fn new(url: Url) -> Self {
        Self {
            inner: surf_disco::Client::new(url.clone()),
            url,
        }
    }

    pub fn get<T: DeserializeOwned>(&self, route: &str) -> Request<T, ServerError> {
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
    ) -> Vec<AccountQueryData>;

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
        elem: &Commitment<Header>,
    );
}

#[derive(Debug, Clone, Default)]
pub struct MockStateCatchup {
    state: ValidatedState,
}

impl From<ValidatedState> for MockStateCatchup {
    fn from(state: ValidatedState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl StateCatchup for MockStateCatchup {
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        _fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> Vec<AccountQueryData> {
        accounts
            .into_iter()
            .map(|account| {
                tracing::info!("catchup: fetching account {account:?} for view {view:?}");
                FeeAccountProof::prove(&self.state.fee_merkle_tree, account.into())
                    .unwrap_or_else(|| panic!("Account {account:?} not in memory"))
                    .into()
            })
            .collect()
    }

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
        elem: &Commitment<Header>,
    ) {
        tracing::info!("catchup: fetching frontier for view {view:?}");
        let view = view.get_u64();
        let (_, proof) = self
            .state
            .block_merkle_tree
            .lookup(view)
            .expect_ok()
            .unwrap();
        mt.remember(view, elem, proof.clone())
            .expect("Proof verifies");
    }
}

#[derive(Debug, Clone, Default)]
pub struct StatePeers {
    clients: Vec<Client<ServerError>>,
    interval: Duration,
}

impl StatePeers {
    pub fn from_urls(urls: Vec<Url>) -> Self {
        if urls.is_empty() {
            panic!("Cannot create StatePeers with no peers");
        }

        Self {
            clients: urls.into_iter().map(Client::new).collect(),
            interval: Duration::from_secs(1),
        }
    }

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
                        "state/catchup/{}/account/{}",
                        view.get_u64(),
                        account.address()
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
impl StateCatchup for StatePeers {
    async fn fetch_accounts(
        &self,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> Vec<AccountQueryData> {
        let mut ret = vec![];
        for account in accounts {
            tracing::info!("Fetching account {account:?} for view {view:?}");
            ret.push(
                self.fetch_account(view, fee_merkle_tree_root, account)
                    .await,
            )
        }
        ret
    }

    async fn remember_blocks_merkle_tree(
        &self,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
        elem: &Commitment<Header>,
    ) {
        if self.clients.is_empty() {
            panic!("No peers to fetch frontier from");
        }
        loop {
            for client in self.clients.iter() {
                tracing::info!("Fetching frontier for view {view:?} from {}", client.url);
                match client
                    .get::<BlocksFrontier>(&format!("state/catchup/{}/blocks", view.get_u64()))
                    .send()
                    .await
                {
                    // TODO: is this the right way to remember the frontier?
                    Ok(frontier) => match mt.remember(view.get_u64(), elem, &frontier) {
                        Ok(_) => return,
                        Err(err) => tracing::warn!("Error verifying block proof: {}", err),
                    },
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
