use std::sync::Arc;

use anyhow::{bail, Context};
use async_trait::async_trait;
use committable::Commitment;
use committable::Committable;
use espresso_types::{
    v0::traits::{PersistenceOptions, StateCatchup},
    v0_3::ChainConfig,
    AccountQueryData, BackoffParams, BlockMerkleTree, FeeAccount, FeeMerkleCommitment,
};
use futures::future::FutureExt;
use hotshot_orchestrator::config::NetworkConfig;
use hotshot_types::{
    data::ViewNumber, traits::node_implementation::ConsensusTime as _, ValidatorConfig,
};
use jf_merkle_tree::{prelude::MerkleNode, ForgetableMerkleTreeScheme, MerkleTreeScheme};
use serde::de::DeserializeOwned;
use surf_disco::Request;
use tide_disco::error::ServerError;
use url::Url;
use vbs::version::StaticVersionType;

use crate::{
    api::{
        data_source::{CatchupDataSource, PublicNetworkConfig},
        BlocksFrontier,
    },
    PubKey,
};

// This newtype is probably not worth having. It's only used to be able to log
// URLs before doing requests.
#[derive(Debug, Clone)]
struct Client<ServerError, ApiVer: StaticVersionType> {
    inner: surf_disco::Client<ServerError, ApiVer>,
    url: Url,
}

impl<ApiVer: StaticVersionType> Client<ServerError, ApiVer> {
    pub fn new(url: Url) -> Self {
        Self {
            inner: surf_disco::Client::new(url.clone()),
            url,
        }
    }

    pub fn get<T: DeserializeOwned>(&self, route: &str) -> Request<T, ServerError, ApiVer> {
        self.inner.get(route)
    }
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
pub struct StatePeers<ApiVer: StaticVersionType> {
    clients: Vec<Client<ServerError, ApiVer>>,
    backoff: BackoffParams,
}

impl<ApiVer: StaticVersionType> StatePeers<ApiVer> {
    pub fn from_urls(urls: Vec<Url>, backoff: BackoffParams) -> Self {
        if urls.is_empty() {
            panic!("Cannot create StatePeers with no peers");
        }

        Self {
            clients: urls.into_iter().map(Client::new).collect(),
            backoff,
        }
    }

    pub async fn fetch_config(
        &self,
        my_own_validator_config: ValidatorConfig<PubKey>,
    ) -> NetworkConfig<PubKey> {
        self.backoff()
            .retry(self, move |provider| {
                let my_own_validator_config = my_own_validator_config.clone();
                async move {
                    for client in &provider.clients {
                        tracing::info!("fetching config from {}", client.url);
                        match client
                            .get::<PublicNetworkConfig>("config/hotshot")
                            .send()
                            .await
                        {
                            Ok(res) => {
                                return res.into_network_config(my_own_validator_config)
                                    .context(format!("fetched config from {}, but failed to convert to private config", client.url));
                            }
                            Err(err) => {
                                tracing::warn!("error fetching config from peer: {err:#}");
                            }
                        }
                    }
                    bail!("could not fetch config from any peer");
                }
                .boxed()
            })
            .await
    }
}

#[async_trait]
impl<ApiVer: StaticVersionType> StateCatchup for StatePeers<ApiVer> {
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
                Ok(res) => {
                    if res.proof.account == account.into() {
                        match res.proof.verify(&fee_merkle_tree_root) {
                            Ok(_) => return Ok(res),
                            Err(err) => tracing::warn!("Error verifying account proof: {}", err),
                        }
                    } else {
                        tracing::warn!("Invalid proof received from peer {:?}", client.url);
                    }
                }
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
                    if cf.commit() == commitment {
                        return Ok(cf);
                    } else {
                        tracing::error!(
                            "Received chain config with mismatched commitment from {}: expected {}, got {}",
                            client.url,
                            commitment,
                            cf.commit(),
                        );
                    }
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
    db: Arc<T>,
    backoff: BackoffParams,
}

impl<T> SqlStateCatchup<T> {
    pub(crate) fn new(db: Arc<T>, backoff: BackoffParams) -> Self {
        Self { db, backoff }
    }
}

#[async_trait]
impl<T> StateCatchup for SqlStateCatchup<T>
where
    T: CatchupDataSource + std::fmt::Debug + Send + Sync,
{
    // TODO: add a test for the account proof validation
    // issue # 2102 (https://github.com/EspressoSystems/espresso-sequencer/issues/2102)
    #[tracing::instrument(skip(self))]
    async fn try_fetch_account(
        &self,
        block_height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        let res = self
            .db
            .get_account(block_height, view, account.into())
            .await?;

        if res.proof.account != account.into() {
            panic!(
                "Critical error: Mismatched fee account proof: expected {:?}, got {:?}.
                This may indicate a compromised database",
                account, res.proof.account
            );
        }

        match res.proof.verify(&fee_merkle_tree_root) {
            Ok(_) => return Ok(res),
            Err(err) => panic!(
                "Critical error: failed to verify account proof from the database: {}",
                err
            ),
        }
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

        let proof = self.db.get_frontier(bh, view).await?;
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
        self.db.get_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        &self.backoff
    }
}
