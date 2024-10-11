use std::sync::Arc;

use anyhow::{bail, Context};
use async_trait::async_trait;
use committable::Commitment;
use committable::Committable;
use espresso_types::{
    v0::traits::{PersistenceOptions, StateCatchup},
    v0_3::ChainConfig,
    BackoffParams, BlockMerkleTree, FeeAccount, FeeAccountProof, FeeMerkleCommitment,
    FeeMerkleTree, Leaf, NodeState,
};
use futures::future::{Future, FutureExt};
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
    api::{data_source::PublicNetworkConfig, BlocksFrontier},
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
    #[tracing::instrument(skip(self, _instance))]
    async fn try_fetch_accounts(
        &self,
        _instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        for client in self.clients.iter() {
            tracing::info!("Fetching accounts from {}", client.url);
            let req = match client
                .inner
                .post::<FeeMerkleTree>(&format!("catchup/{height}/{}/accounts", view.u64(),))
                .body_binary(&accounts.to_vec())
            {
                Ok(req) => req,
                Err(err) => {
                    tracing::warn!("failed to construct accounts catchup request: {err:#}");
                    continue;
                }
            };
            let snapshot = match req.send().await {
                Ok(res) => res,
                Err(err) => {
                    tracing::warn!("Error fetching accounts from peer: {err:#}");
                    continue;
                }
            };

            // Verify proofs.
            for account in accounts {
                let Some((proof, _)) = FeeAccountProof::prove(&snapshot, (*account).into()) else {
                    tracing::warn!("response from peer missing account {account}");
                    continue;
                };
                if let Err(err) = proof.verify(&fee_merkle_tree_root) {
                    tracing::warn!("peer gave invalid proof for account {account}: {err:#}");
                    continue;
                }
            }

            return Ok(snapshot);
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

pub(crate) trait CatchupStorage: Sync {
    /// Get the state of the requested `accounts`.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    ///
    /// If successful, this function also returns the leaf from `view`, if it is available. This can
    /// be used to add the recovered state to HotShot's state map, so that future requests can get
    /// the state from memory rather than storage.
    fn get_accounts(
        &self,
        _instance: &NodeState,
        _height: u64,
        _view: ViewNumber,
        _accounts: &[FeeAccount],
    ) -> impl Send + Future<Output = anyhow::Result<(FeeMerkleTree, Leaf)>> {
        // Merklized state catchup is only supported by persistence backends that provide merklized
        // state storage. This default implementation is overridden for those that do. Otherwise,
        // catchup can still be provided by fetching undecided merklized state from consensus
        // memory.
        async {
            bail!("merklized state catchup is not supported for this data source");
        }
    }

    /// Get the blocks Merkle tree frontier.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    fn get_frontier(
        &self,
        _height: u64,
        _view: ViewNumber,
    ) -> impl Send + Future<Output = anyhow::Result<BlocksFrontier>> {
        // Merklized state catchup is only supported by persistence backends that provide merklized
        // state storage. This default implementation is overridden for those that do. Otherwise,
        // catchup can still be provided by fetching undecided merklized state from consensus
        // memory.
        async {
            bail!("merklized state catchup is not supported for this data source");
        }
    }

    fn get_chain_config(
        &self,
        _commitment: Commitment<ChainConfig>,
    ) -> impl Send + Future<Output = anyhow::Result<ChainConfig>> {
        async {
            bail!("chain config catchup is not supported for this data source");
        }
    }
}

impl CatchupStorage for hotshot_query_service::data_source::MetricsDataSource {}

impl<T, S> CatchupStorage for hotshot_query_service::data_source::ExtensibleDataSource<T, S>
where
    T: CatchupStorage,
    S: Sync,
{
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<(FeeMerkleTree, Leaf)> {
        self.inner()
            .get_accounts(instance, height, view, accounts)
            .await
    }

    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.inner().get_frontier(height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.inner().get_chain_config(commitment).await
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
    T: CatchupStorage + std::fmt::Debug + Send + Sync,
{
    // TODO: add a test for the account proof validation
    // issue # 2102 (https://github.com/EspressoSystems/espresso-sequencer/issues/2102)
    #[tracing::instrument(skip(self, instance))]
    async fn try_fetch_accounts(
        &self,
        instance: &NodeState,
        block_height: u64,
        view: ViewNumber,
        _fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        Ok(self
            .db
            .get_accounts(instance, block_height, view, accounts)
            .await?
            .0)
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
        let cf = self.db.get_chain_config(commitment).await?;

        if cf.commit() != commitment {
            panic!(
                "Critical error: Mismatched chain config detected. Expected chain config: {:?}, but got: {:?}. 
                This may indicate a compromised database",
                commitment,
                cf.commit()
            )
        }

        Ok(cf)
    }

    fn backoff(&self) -> &BackoffParams {
        &self.backoff
    }
}
