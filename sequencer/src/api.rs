use std::{pin::Pin, sync::Arc};

use anyhow::{bail, Context};
use async_lock::RwLock;
use async_once_cell::Lazy;
use async_trait::async_trait;
use committable::Commitment;
use data_source::{CatchupDataSource, StakeTableDataSource, SubmitDataSource};
use derivative::Derivative;
use espresso_types::{
    config::PublicNetworkConfig, retain_accounts, v0::traits::SequencerPersistence,
    v0_99::ChainConfig, AccountQueryData, BlockMerkleTree, FeeAccount, FeeAccountProof,
    FeeMerkleTree, Leaf2, NodeState, PubKey, Transaction, ValidatedState,
};
use futures::{
    future::{BoxFuture, Future, FutureExt},
    stream::BoxStream,
};
use hotshot_events_service::events_source::{
    EventFilterSet, EventsSource, EventsStreamer, StartupInfo,
};
use hotshot_query_service::data_source::ExtensibleDataSource;
use hotshot_types::{
    data::ViewNumber,
    event::Event,
    light_client::StateSignatureRequestBody,
    network::NetworkConfig,
    traits::{
        network::ConnectedNetwork,
        node_implementation::{NodeType, Versions},
        ValidatedState as _,
    },
    utils::{View, ViewInner},
    vote::HasViewNumber,
    PeerConfig,
};
use itertools::Itertools;
use jf_merkle_tree::MerkleTreeScheme;

use self::data_source::{HotShotConfigDataSource, NodeStateDataSource, StateSignatureDataSource};
use crate::{
    catchup::CatchupStorage, context::Consensus, state_signature::StateSigner, SeqTypes,
    SequencerApiVersion, SequencerContext,
};

pub mod data_source;
pub mod endpoints;
pub mod fs;
pub mod options;
pub mod sql;
mod update;

pub use options::Options;

pub type BlocksFrontier = <BlockMerkleTree as MerkleTreeScheme>::MembershipProof;

type BoxLazy<T> = Pin<Arc<Lazy<T, BoxFuture<'static, T>>>>;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
struct ConsensusState<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> {
    state_signer: Arc<StateSigner<SequencerApiVersion>>,
    event_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,
    node_state: NodeState,
    network_config: NetworkConfig<PubKey>,

    #[derivative(Debug = "ignore")]
    handle: Arc<RwLock<Consensus<N, P, V>>>,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions>
    From<&SequencerContext<N, P, V>> for ConsensusState<N, P, V>
{
    fn from(ctx: &SequencerContext<N, P, V>) -> Self {
        Self {
            state_signer: ctx.state_signer(),
            event_streamer: ctx.event_streamer(),
            node_state: ctx.node_state(),
            network_config: ctx.network_config(),
            handle: ctx.consensus(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
struct ApiState<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> {
    // The consensus state is initialized lazily so we can start the API (and healthcheck endpoints)
    // before consensus has started. Any endpoint that uses consensus state will wait for
    // initialization to finish, but endpoints that do not require a consensus handle can proceed
    // without waiting.
    #[derivative(Debug = "ignore")]
    consensus: BoxLazy<ConsensusState<N, P, V>>,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> ApiState<N, P, V> {
    fn new(init: impl Future<Output = ConsensusState<N, P, V>> + Send + 'static) -> Self {
        Self {
            consensus: Arc::pin(Lazy::from_future(init.boxed())),
        }
    }

    async fn state_signer(&self) -> &StateSigner<SequencerApiVersion> {
        &self.consensus.as_ref().get().await.get_ref().state_signer
    }

    async fn event_streamer(&self) -> &RwLock<EventsStreamer<SeqTypes>> {
        &self.consensus.as_ref().get().await.get_ref().event_streamer
    }

    async fn consensus(&self) -> Arc<RwLock<Consensus<N, P, V>>> {
        Arc::clone(&self.consensus.as_ref().get().await.get_ref().handle)
    }

    async fn network_config(&self) -> NetworkConfig<PubKey> {
        self.consensus
            .as_ref()
            .get()
            .await
            .get_ref()
            .network_config
            .clone()
    }
}

type StorageState<N, P, D, V> = ExtensibleDataSource<D, ApiState<N, P, V>>;

#[async_trait]
impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> EventsSource<SeqTypes>
    for ApiState<N, P, V>
{
    type EventStream = BoxStream<'static, Arc<Event<SeqTypes>>>;

    async fn get_event_stream(
        &self,
        _filter: Option<EventFilterSet<SeqTypes>>,
    ) -> Self::EventStream {
        self.event_streamer()
            .await
            .read()
            .await
            .get_event_stream(None)
            .await
    }
    async fn get_startup_info(&self) -> StartupInfo<SeqTypes> {
        self.event_streamer()
            .await
            .read()
            .await
            .get_startup_info()
            .await
    }
}

impl<N: ConnectedNetwork<PubKey>, D: Send + Sync, V: Versions, P: SequencerPersistence>
    SubmitDataSource<N, P> for StorageState<N, P, D, V>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        self.as_ref().submit(tx).await
    }
}

impl<N: ConnectedNetwork<PubKey>, D: Sync, V: Versions, P: SequencerPersistence>
    StakeTableDataSource<SeqTypes> for StorageState<N, P, D, V>
{
    /// Get the stake table for a given epoch
    async fn get_stake_table(
        &self,
        epoch: Option<<SeqTypes as NodeType>::Epoch>,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        self.as_ref().get_stake_table(epoch).await
    }

    /// Get the stake table for a given epoch
    async fn get_da_members(
        &self,
        epoch: Option<<SeqTypes as NodeType>::Epoch>,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        self.as_ref().get_da_members(epoch).await
    }

    /// Get the stake table for the current epoch if not provided
    async fn get_stake_table_current(
        &self,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        self.as_ref().get_stake_table_current().await
    }

    /// Get the stake table for the current epoch if not provided
    async fn get_da_members_current(
        &self,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        self.as_ref().get_da_members_current().await
    }

    /// Get the stake table for the current epoch if not provided
    async fn get_current_epoch(&self) -> Option<<SeqTypes as NodeType>::Epoch> {
        self.as_ref().get_current_epoch().await
    }
}
impl<N: ConnectedNetwork<PubKey>, V: Versions, P: SequencerPersistence>
    StakeTableDataSource<SeqTypes> for ApiState<N, P, V>
{
    /// Get the stake table for a given epoch
    async fn get_stake_table(
        &self,
        epoch: Option<<SeqTypes as NodeType>::Epoch>,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        let Ok(mem) = self
            .consensus()
            .await
            .read()
            .await
            .membership_coordinator
            .membership_for_epoch(epoch)
            .await
        else {
            return vec![];
        };
        mem.stake_table().await
    }

    /// Get the stake table for the current epoch if not provided
    async fn get_stake_table_current(
        &self,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        let epoch = self.consensus().await.read().await.cur_epoch().await;

        self.get_stake_table(epoch).await
    }

    async fn get_current_epoch(&self) -> Option<<SeqTypes as NodeType>::Epoch> {
        self.consensus().await.read().await.cur_epoch().await
    }

    async fn get_da_members(
        &self,
        epoch: Option<<SeqTypes as NodeType>::Epoch>,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        let Ok(mem) = self
            .consensus()
            .await
            .read()
            .await
            .membership_coordinator
            .membership_for_epoch(epoch)
            .await
        else {
            return vec![];
        };
        mem.da_stake_table().await
    }

    /// Get the stake table for the current epoch if not provided
    async fn get_da_members_current(
        &self,
    ) -> Vec<PeerConfig<<SeqTypes as NodeType>::SignatureKey>> {
        let epoch = self.consensus().await.read().await.cur_epoch().await;

        self.get_da_members(epoch).await
    }
}

impl<N: ConnectedNetwork<PubKey>, V: Versions, P: SequencerPersistence> SubmitDataSource<N, P>
    for ApiState<N, P, V>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        let handle = self.consensus().await;

        let consensus_read_lock = handle.read().await;

        // Fetch full chain config from the validated state, if present.
        // This is necessary because we support chain config upgrades,
        // so the updated chain config is found in the validated state.
        let cf = consensus_read_lock
            .decided_state()
            .await
            .chain_config
            .resolve();

        // Use the chain config from the validated state if available,
        // otherwise, use the node state's chain config
        // The node state's chain config is the node's base version chain config
        let cf = match cf {
            Some(cf) => cf,
            None => self.node_state().await.chain_config,
        };

        let max_block_size: u64 = cf.max_block_size.into();
        let txn_size = tx.payload().len() as u64;

        // reject transaction bigger than block size
        if txn_size > max_block_size {
            bail!("transaction size ({txn_size}) is greater than max_block_size ({max_block_size})")
        }

        consensus_read_lock.submit_transaction(tx).await?;
        Ok(())
    }
}

impl<N, P, D, V> NodeStateDataSource for StorageState<N, P, D, V>
where
    N: ConnectedNetwork<PubKey>,
    V: Versions,
    P: SequencerPersistence,
    D: Sync,
{
    async fn node_state(&self) -> &NodeState {
        self.as_ref().node_state().await
    }
}

impl<
        N: ConnectedNetwork<PubKey>,
        V: Versions,
        P: SequencerPersistence,
        D: CatchupStorage + Send + Sync,
    > CatchupDataSource for StorageState<N, P, D, V>
{
    #[tracing::instrument(skip(self, instance))]
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        // Check if we have the desired state in memory.
        match self
            .as_ref()
            .get_accounts(instance, height, view, accounts)
            .await
        {
            Ok(accounts) => return Ok(accounts),
            Err(err) => {
                tracing::info!("accounts not in memory, trying storage: {err:#}");
            },
        }

        // Try storage.
        let (tree, leaf) = self
            .inner()
            .get_accounts(instance, height, view, accounts)
            .await
            .context("accounts not in memory, and could not fetch from storage")?;
        // If we successfully fetched accounts from storage, try to add them back into the in-memory
        // state.
        let handle = self.as_ref().consensus().await;
        let handle = handle.read().await;
        let consensus = handle.consensus();
        let mut consensus = consensus.write().await;
        let (state, delta) = match consensus.validated_state_map().get(&view) {
            Some(View {
                view_inner: ViewInner::Leaf { state, delta, .. },
            }) => {
                let mut state = (**state).clone();

                // Add the fetched accounts to the state.
                for account in accounts {
                    if let Some((proof, _)) = FeeAccountProof::prove(&tree, (*account).into()) {
                        if let Err(err) = proof.remember(&mut state.fee_merkle_tree) {
                            tracing::warn!(
                                ?view,
                                %account,
                                "cannot update fetched account state: {err:#}"
                            );
                        }
                    } else {
                        tracing::warn!(?view, %account, "cannot update fetched account state because account is not in the merkle tree");
                    };
                }

                (Arc::new(state), delta.clone())
            },
            _ => {
                // If we don't already have a leaf for this view, or if we don't have the view
                // at all, we can create a new view based on the recovered leaf and add it to
                // our state map. In this case, we must also add the leaf to the saved leaves
                // map to ensure consistency.
                let mut state = ValidatedState::from_header(leaf.block_header());
                state.fee_merkle_tree = tree.clone();
                (Arc::new(state), None)
            },
        };
        if let Err(err) = consensus.update_leaf(leaf, Arc::clone(&state), delta) {
            tracing::warn!(?view, "cannot update fetched account state: {err:#}");
        }
        tracing::info!(?view, "updated with fetched account state");

        Ok(tree)
    }

    #[tracing::instrument(skip(self, instance))]
    async fn get_frontier(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> anyhow::Result<BlocksFrontier> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_frontier(instance, height, view).await {
            Ok(frontier) => return Ok(frontier),
            Err(err) => {
                tracing::info!("frontier is not in memory, trying storage: {err:#}");
            },
        }

        // Try storage.
        self.inner().get_frontier(instance, height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_chain_config(commitment).await {
            Ok(cf) => return Ok(cf),
            Err(err) => {
                tracing::info!("chain config is not in memory, trying storage: {err:#}");
            },
        }

        // Try storage.
        self.inner().get_chain_config(commitment).await
    }
    async fn get_leaf_chain(&self, height: u64) -> anyhow::Result<Vec<Leaf2>> {
        // Check if we have the desired state in memory.
        match self.as_ref().get_leaf_chain(height).await {
            Ok(cf) => return Ok(cf),
            Err(err) => {
                tracing::info!("chain config is not in memory, trying storage: {err:#}");
            },
        }

        // Try storage.
        self.inner().get_leaf_chain(height).await
    }
}

// #[async_trait]
// impl<
//         N: ConnectedNetwork<PubKey>,
//         V: Versions,
//         P: SequencerPersistence,
//         D: ChainConfigPersistence + Send + Sync,
//     > ChainConfigPersistence for StorageState<N, P, D, V>
// {
//     async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
//         self.inner_mut().insert_chain_config(chain_config).await
//     }
//     async fn load_chain_config(
//         &self,
//         commitment: Commitment<ChainConfig>,
//     ) -> anyhow::Result<ChainConfig> {
//         self.inner().load_chain_config(commitment).await
//     }
// }

impl<N, V, P> NodeStateDataSource for ApiState<N, P, V>
where
    N: ConnectedNetwork<PubKey>,
    V: Versions,
    P: SequencerPersistence,
{
    async fn node_state(&self) -> &NodeState {
        &self.consensus.as_ref().get().await.get_ref().node_state
    }
}

impl<N: ConnectedNetwork<PubKey>, V: Versions, P: SequencerPersistence> CatchupDataSource
    for ApiState<N, P, V>
{
    #[tracing::instrument(skip(self, _instance))]
    async fn get_accounts(
        &self,
        _instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        let state = self
            .consensus()
            .await
            .read()
            .await
            .state(view)
            .await
            .context(format!(
                "state not available for height {height}, view {view:?}"
            ))?;
        retain_accounts(&state.fee_merkle_tree, accounts.iter().copied())
    }

    #[tracing::instrument(skip(self, _instance))]
    async fn get_frontier(
        &self,
        _instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> anyhow::Result<BlocksFrontier> {
        let state = self
            .consensus()
            .await
            .read()
            .await
            .state(view)
            .await
            .context(format!(
                "state not available for height {height}, view {view:?}"
            ))?;
        let tree = &state.block_merkle_tree;
        let frontier = tree.lookup(tree.num_leaves() - 1).expect_ok()?.1;
        Ok(frontier)
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let state = self.consensus().await.read().await.decided_state().await;
        let chain_config = state.chain_config;

        if chain_config.commit() == commitment {
            chain_config.resolve().context("chain config found")
        } else {
            bail!("chain config not found")
        }
    }

    async fn get_leaf_chain(&self, height: u64) -> anyhow::Result<Vec<Leaf2>> {
        let mut leaves = self
            .consensus()
            .await
            .read()
            .await
            .consensus()
            .read()
            .await
            .undecided_leaves();
        leaves.sort_by_key(|l| l.height());
        let (position, mut last_leaf) = leaves
            .iter()
            .find_position(|l| l.height() == height)
            .context(format!("leaf chain not available for {height}"))?;
        let mut chain = vec![last_leaf.clone()];
        for leaf in leaves.iter().skip(position + 1) {
            if leaf.justify_qc().view_number() == last_leaf.view_number() {
                chain.push(leaf.clone());
            } else {
                continue;
            }
            if leaf.view_number() == last_leaf.view_number() + 1 {
                // one away from decide
                last_leaf = leaf;
                break;
            }
            last_leaf = leaf;
        }
        // Make sure we got one more leaf to confirm the decide
        for leaf in leaves
            .iter()
            .skip_while(|l| l.height() <= last_leaf.height())
        {
            if leaf.justify_qc().view_number() == last_leaf.view_number() {
                chain.push(leaf.clone());
                return Ok(chain);
            }
        }
        bail!(format!("leaf chain not available for {height}"))
    }
}

impl<N: ConnectedNetwork<PubKey>, D: Sync, V: Versions, P: SequencerPersistence>
    HotShotConfigDataSource for StorageState<N, P, D, V>
{
    async fn get_config(&self) -> PublicNetworkConfig {
        self.as_ref().network_config().await.into()
    }
}

impl<N: ConnectedNetwork<PubKey>, V: Versions, P: SequencerPersistence> HotShotConfigDataSource
    for ApiState<N, P, V>
{
    async fn get_config(&self) -> PublicNetworkConfig {
        self.network_config().await.into()
    }
}

#[async_trait]
impl<N: ConnectedNetwork<PubKey>, D: Sync, V: Versions, P: SequencerPersistence>
    StateSignatureDataSource<N> for StorageState<N, P, D, V>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.as_ref().get_state_signature(height).await
    }
}

#[async_trait]
impl<N: ConnectedNetwork<PubKey>, V: Versions, P: SequencerPersistence> StateSignatureDataSource<N>
    for ApiState<N, P, V>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.state_signer().await.get_state_signature(height).await
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod test_helpers {
    use std::time::Duration;

    use committable::Committable;
    use espresso_types::{
        v0::traits::{NullEventConsumer, PersistenceOptions, StateCatchup},
        MarketplaceVersion, MockSequencerVersions, NamespaceId, ValidatedState,
    };
    use ethers::{prelude::Address, utils::Anvil};
    use futures::{
        future::{join_all, FutureExt},
        stream::StreamExt,
    };
    use hotshot::types::{Event, EventType};
    use hotshot_contract_adapter::light_client::{ParsedLightClientState, ParsedStakeTableState};
    use hotshot_state_prover::service::light_client_genesis_from_stake_table;
    use hotshot_types::{
        event::LeafInfo,
        traits::{metrics::NoMetrics, node_implementation::ConsensusTime},
    };
    use itertools::izip;
    use jf_merkle_tree::{MerkleCommitment, MerkleTreeScheme};
    use portpicker::pick_unused_port;
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::{error::ServerError, Api, App, Error, StatusCode};
    use tokio::{spawn, task::JoinHandle, time::sleep};
    use url::Url;
    use vbs::version::{StaticVersion, StaticVersionType};

    use super::*;
    use crate::{
        catchup::NullStateCatchup,
        network,
        persistence::no_storage,
        testing::{
            run_marketplace_builder, run_test_builder, wait_for_decide_on_handle, TestConfig,
            TestConfigBuilder,
        },
    };

    pub const STAKE_TABLE_CAPACITY_FOR_TEST: u64 = 10;

    pub struct TestNetwork<P: PersistenceOptions, const NUM_NODES: usize, V: Versions> {
        pub server: SequencerContext<network::Memory, P::Persistence, V>,
        pub peers: Vec<SequencerContext<network::Memory, P::Persistence, V>>,
        pub cfg: TestConfig<{ NUM_NODES }>,
        // todo (abdul): remove this when fs storage is removed
        pub temp_dir: Option<TempDir>,
    }

    pub struct TestNetworkConfig<const NUM_NODES: usize, P, C>
    where
        P: PersistenceOptions,
        C: StateCatchup + 'static,
    {
        state: [ValidatedState; NUM_NODES],
        persistence: [P; NUM_NODES],
        catchup: [C; NUM_NODES],
        network_config: TestConfig<{ NUM_NODES }>,
        api_config: Options,
    }

    #[derive(Clone)]
    pub struct TestNetworkConfigBuilder<const NUM_NODES: usize, P, C>
    where
        P: PersistenceOptions,
        C: StateCatchup + 'static,
    {
        state: [ValidatedState; NUM_NODES],
        persistence: Option<[P; NUM_NODES]>,
        catchup: Option<[C; NUM_NODES]>,
        api_config: Option<Options>,
        network_config: Option<TestConfig<{ NUM_NODES }>>,
    }

    impl Default for TestNetworkConfigBuilder<5, no_storage::Options, NullStateCatchup> {
        fn default() -> Self {
            TestNetworkConfigBuilder {
                state: std::array::from_fn(|_| ValidatedState::default()),
                persistence: Some([no_storage::Options; 5]),
                catchup: Some(std::array::from_fn(|_| NullStateCatchup::default())),
                network_config: None,
                api_config: None,
            }
        }
    }

    impl<const NUM_NODES: usize>
        TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, NullStateCatchup>
    {
        pub fn with_num_nodes(
        ) -> TestNetworkConfigBuilder<{ NUM_NODES }, no_storage::Options, NullStateCatchup>
        {
            TestNetworkConfigBuilder {
                state: std::array::from_fn(|_| ValidatedState::default()),
                persistence: Some([no_storage::Options; { NUM_NODES }]),
                catchup: Some(std::array::from_fn(|_| NullStateCatchup::default())),
                network_config: None,
                api_config: None,
            }
        }

        pub fn with_max_block_size(&self, max_block_size: u64) -> Self {
            let cf = ChainConfig {
                max_block_size: max_block_size.into(),
                ..Default::default()
            }
            .into();

            let mut cfg = self.clone();
            cfg.state.iter_mut().for_each(|s| s.chain_config = cf);

            cfg
        }
    }

    impl<const NUM_NODES: usize, P, C> TestNetworkConfigBuilder<{ NUM_NODES }, P, C>
    where
        P: PersistenceOptions,
        C: StateCatchup + 'static,
    {
        pub fn states(mut self, state: [ValidatedState; NUM_NODES]) -> Self {
            self.state = state;
            self
        }

        pub fn persistences<NP: PersistenceOptions>(
            self,
            persistence: [NP; NUM_NODES],
        ) -> TestNetworkConfigBuilder<{ NUM_NODES }, NP, C> {
            TestNetworkConfigBuilder {
                state: self.state,
                catchup: self.catchup,
                network_config: self.network_config,
                api_config: self.api_config,
                persistence: Some(persistence),
            }
        }

        pub fn api_config(mut self, api_config: Options) -> Self {
            self.api_config = Some(api_config);
            self
        }

        pub fn catchups<NC: StateCatchup + 'static>(
            self,
            catchup: [NC; NUM_NODES],
        ) -> TestNetworkConfigBuilder<{ NUM_NODES }, P, NC> {
            TestNetworkConfigBuilder {
                state: self.state,
                catchup: Some(catchup),
                network_config: self.network_config,
                api_config: self.api_config,
                persistence: self.persistence,
            }
        }

        pub fn network_config(mut self, network_config: TestConfig<{ NUM_NODES }>) -> Self {
            self.network_config = Some(network_config);
            self
        }

        pub fn build(self) -> TestNetworkConfig<{ NUM_NODES }, P, C> {
            TestNetworkConfig {
                state: self.state,
                persistence: self.persistence.unwrap(),
                catchup: self.catchup.unwrap(),
                network_config: self.network_config.unwrap(),
                api_config: self.api_config.unwrap(),
            }
        }
    }

    impl<P: PersistenceOptions, const NUM_NODES: usize, V: Versions> TestNetwork<P, { NUM_NODES }, V> {
        pub async fn new<C: StateCatchup + 'static>(
            cfg: TestNetworkConfig<{ NUM_NODES }, P, C>,
            bind_version: V,
        ) -> Self {
            let mut cfg = cfg;
            let mut builder_tasks = Vec::new();
            let mut marketplace_builder_url = "http://example.com".parse().unwrap();

            if <V as Versions>::Base::VERSION < MarketplaceVersion::VERSION {
                let (task, url) =
                    run_test_builder::<{ NUM_NODES }>(cfg.network_config.builder_port()).await;
                builder_tasks.push(task);
                cfg.network_config.set_builder_urls(vec1::vec1![url]);
            };

            if <V as Versions>::Upgrade::VERSION >= MarketplaceVersion::VERSION
                || <V as Versions>::Base::VERSION >= MarketplaceVersion::VERSION
            {
                let (task, url) = run_marketplace_builder::<{ NUM_NODES }>(
                    cfg.network_config.marketplace_builder_port(),
                )
                .await;
                builder_tasks.push(task);
                cfg.network_config
                    .set_builder_urls(vec1::vec1![url.clone()]);
                marketplace_builder_url = url;
            }

            // add default storage if none is provided as query module is now required
            let mut opt = cfg.api_config.clone();
            let temp_dir = if opt.storage_fs.is_none() && opt.storage_sql.is_none() {
                let temp_dir = tempfile::tempdir().unwrap();
                opt = opt.query_fs(
                    Default::default(),
                    crate::persistence::fs::Options::new(temp_dir.path().to_path_buf()),
                );
                Some(temp_dir)
            } else {
                None
            };

            let mut nodes = join_all(
                izip!(cfg.state, cfg.persistence, cfg.catchup)
                    .enumerate()
                    .map(|(i, (state, persistence, catchup))| {
                        let opt = opt.clone();
                        let cfg = &cfg.network_config;
                        let upgrades_map = cfg.upgrades();
                        let marketplace_builder_url = marketplace_builder_url.clone();
                        async move {
                            if i == 0 {
                                opt.serve(|metrics, consumer| {
                                    let cfg = cfg.clone();
                                    async move {
                                        Ok(cfg
                                            .init_node(
                                                0,
                                                state,
                                                persistence,
                                                catchup,
                                                &*metrics,
                                                STAKE_TABLE_CAPACITY_FOR_TEST,
                                                consumer,
                                                bind_version,
                                                upgrades_map,
                                                marketplace_builder_url,
                                            )
                                            .await)
                                    }
                                    .boxed()
                                })
                                .await
                                .unwrap()
                            } else {
                                cfg.init_node(
                                    i,
                                    state,
                                    persistence,
                                    catchup,
                                    &NoMetrics,
                                    STAKE_TABLE_CAPACITY_FOR_TEST,
                                    NullEventConsumer,
                                    bind_version,
                                    upgrades_map,
                                    marketplace_builder_url,
                                )
                                .await
                            }
                        }
                    }),
            )
            .await;

            let handle_0 = &nodes[0];

            // Hook the builder(s) up to the event stream from the first node
            for builder_task in builder_tasks {
                builder_task.start(Box::new(handle_0.event_stream().await));
            }

            for ctx in &nodes {
                ctx.start_consensus().await;
            }

            let server = nodes.remove(0);
            let peers = nodes;

            Self {
                server,
                peers,
                cfg: cfg.network_config,
                temp_dir,
            }
        }

        pub fn light_client_genesis(&self) -> (ParsedLightClientState, ParsedStakeTableState) {
            let st = self.cfg.stake_table();
            light_client_genesis_from_stake_table(st).unwrap()
        }

        pub async fn stop_consensus(&mut self) {
            self.server.shutdown_consensus().await;

            for ctx in &mut self.peers {
                ctx.shutdown_consensus().await;
            }
        }
    }

    /// Test the status API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the status API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the status module or changing the port).
    pub async fn status_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url);

        let options = opt(Options::with_port(port));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        client.connect(None).await;

        // The status API is well tested in the query service repo. Here we are just smoke testing
        // that we set it up correctly. Wait for a (non-genesis) block to be sequenced and then
        // check the success rate metrics.
        while client
            .get::<u64>("status/block-height")
            .send()
            .await
            .unwrap()
            <= 1
        {
            sleep(Duration::from_secs(1)).await;
        }
        let success_rate = client
            .get::<f64>("status/success-rate")
            .send()
            .await
            .unwrap();
        // If metrics are populating correctly, we should get a finite number. If not, we might get
        // NaN or infinity due to division by 0.
        assert!(success_rate.is_finite(), "{success_rate}");
        // We know at least some views have been successful, since we finalized a block.
        assert!(success_rate > 0.0, "{success_rate}");
    }

    /// Test the submit API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the submit API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the submit module or changing the port).
    pub async fn submit_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_test();

        let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url);

        let options = opt(Options::with_port(port).submit(Default::default()));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let mut events = network.server.event_stream().await;

        client.connect(None).await;

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(&mut events, &txn).await;
    }

    /// Test the state signature API.
    pub async fn state_signature_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_test();

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();

        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url);

        let options = opt(Options::with_port(port));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        let mut height: u64;
        // Wait for block >=2 appears
        // It's waiting for an extra second to make sure that the signature is generated
        loop {
            height = network.server.decided_leaf().await.height();
            sleep(std::time::Duration::from_secs(1)).await;
            if height >= 2 {
                break;
            }
        }
        // we cannot verify the signature now, because we don't know the stake table
        client
            .get::<StateSignatureRequestBody>(&format!("state-signature/block/{}", height))
            .send()
            .await
            .unwrap();
    }

    /// Test the catchup API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the catchup API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the catchup module or changing the port).
    pub async fn catchup_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url);

        let options = opt(Options::with_port(port));
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        client.connect(None).await;

        // Wait for a few blocks to be decided.
        let mut events = network.server.event_stream().await;
        loop {
            if let Event {
                event: EventType::Decide { leaf_chain, .. },
                ..
            } = events.next().await.unwrap()
            {
                if leaf_chain
                    .iter()
                    .any(|LeafInfo { leaf, .. }| leaf.block_header().height() > 2)
                {
                    break;
                }
            }
        }

        // Stop consensus running on the node so we freeze the decided and undecided states.
        // We'll let it go out of scope here since it's a write lock.
        {
            network.server.shutdown_consensus().await;
        }

        // Undecided fee state: absent account.
        let leaf = network.server.decided_leaf().await;
        let height = leaf.height() + 1;
        let view = leaf.view_number() + 1;
        let res = client
            .get::<AccountQueryData>(&format!(
                "catchup/{height}/{}/account/{:x}",
                view.u64(),
                Address::default()
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(res.balance, 0.into());
        assert_eq!(
            res.proof
                .verify(
                    &network
                        .server
                        .state(view)
                        .await
                        .unwrap()
                        .fee_merkle_tree
                        .commitment()
                )
                .unwrap(),
            0.into()
        );

        // Undecided block state.
        let res = client
            .get::<BlocksFrontier>(&format!("catchup/{height}/{}/blocks", view.u64()))
            .send()
            .await
            .unwrap();
        let root = &network
            .server
            .state(view)
            .await
            .unwrap()
            .block_merkle_tree
            .commitment();
        BlockMerkleTree::verify(root.digest(), root.size() - 1, res)
            .unwrap()
            .unwrap();
    }

    pub async fn spawn_dishonest_peer_catchup_api() -> anyhow::Result<(Url, JoinHandle<()>)> {
        let toml = toml::from_str::<toml::Value>(include_str!("../api/catchup.toml")).unwrap();
        let mut api =
            Api::<(), hotshot_query_service::Error, SequencerApiVersion>::new(toml).unwrap();

        api.get("account", |_req, _state: &()| {
            async move {
                Result::<AccountQueryData, _>::Err(hotshot_query_service::Error::catch_all(
                    StatusCode::BAD_REQUEST,
                    "no account found".to_string(),
                ))
            }
            .boxed()
        })?
        .get("blocks", |_req, _state| {
            async move {
                Result::<BlocksFrontier, _>::Err(hotshot_query_service::Error::catch_all(
                    StatusCode::BAD_REQUEST,
                    "no block found".to_string(),
                ))
            }
            .boxed()
        })?
        .get("chainconfig", |_req, _state| {
            async move {
                Result::<ChainConfig, _>::Ok(ChainConfig {
                    max_block_size: 300.into(),
                    base_fee: 1.into(),
                    fee_recipient: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                        .parse()
                        .unwrap(),
                    ..Default::default()
                })
            }
            .boxed()
        })?
        .get("leafchain", |_req, _state| {
            async move {
                Result::<Vec<Leaf2>, _>::Err(hotshot_query_service::Error::catch_all(
                    StatusCode::BAD_REQUEST,
                    "No leafchain found".to_string(),
                ))
            }
            .boxed()
        })?;

        let mut app = App::<_, hotshot_query_service::Error>::with_state(());
        app.with_version(env!("CARGO_PKG_VERSION").parse().unwrap());

        app.register_module::<_, _>("catchup", api).unwrap();

        let port = pick_unused_port().expect("no free port");
        let url: Url = Url::parse(&format!("http://localhost:{port}")).unwrap();

        let handle = spawn({
            let url = url.clone();
            async move {
                let _ = app.serve(url, SequencerApiVersion::instance()).await;
            }
        });

        Ok((url, handle))
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod api_tests {
    use std::fmt::Debug;

    use committable::Committable;
    use data_source::testing::TestableSequencerDataSource;
    use endpoints::NamespaceProofQueryData;
    use espresso_types::{
        traits::{EventConsumer, PersistenceOptions},
        Header, Leaf2, MockSequencerVersions, NamespaceId,
    };
    use ethers::utils::Anvil;
    use futures::{future, stream::StreamExt};
    use hotshot_example_types::node_types::TestVersions;
    use hotshot_query_service::availability::{
        AvailabilityDataSource, BlockQueryData, VidCommonQueryData,
    };
    use hotshot_types::{
        data::{
            ns_table::parse_ns_table, vid_disperse::VidDisperseShare2, DaProposal2, EpochNumber,
            QuorumProposal2, QuorumProposalWrapper, VidCommitment,
        },
        event::LeafInfo,
        message::Proposal,
        simple_certificate::QuorumCertificate2,
        traits::{node_implementation::ConsensusTime, signature_key::SignatureKey, EncodeBytes},
        vid::avidm::{init_avidm_param, AvidMScheme},
    };
    use portpicker::pick_unused_port;
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::Client;
    use test_helpers::{
        catchup_test_helper, state_signature_test_helper, status_test_helper, submit_test_helper,
        TestNetwork, TestNetworkConfigBuilder,
    };
    use tide_disco::error::ServerError;
    use vbs::version::StaticVersion;

    use super::{update::ApiEventConsumer, *};
    use crate::{
        network,
        persistence::no_storage::NoStorage,
        testing::{wait_for_decide_on_handle, TestConfigBuilder},
    };

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn submit_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        submit_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn status_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        status_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn state_signature_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        state_signature_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn test_namespace_query<D: TestableSequencerDataSource>() {
        setup_test();

        // Arbitrary transaction, arbitrary namespace ID
        let ns_id = NamespaceId::from(42_u32);
        let txn = Transaction::new(ns_id, vec![1, 2, 3, 4]);

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let storage = D::create_storage().await;
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(D::options(&storage, Options::with_port(port)).submit(Default::default()))
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let mut events = network.server.event_stream().await;

        // Connect client.
        let client: Client<ServerError, StaticVersion<0, 1>> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        let block_height = wait_for_decide_on_handle(&mut events, &txn).await as usize;
        tracing::info!(block_height, "transaction sequenced");

        // Wait for the query service to update to this block height.
        client
            .socket(&format!("availability/stream/blocks/{block_height}"))
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();

        let mut found_txn = false;
        let mut found_empty_block = false;
        for block_num in 0..=block_height {
            let header: Header = client
                .get(&format!("availability/header/{block_num}"))
                .send()
                .await
                .unwrap();
            let ns_query_res: NamespaceProofQueryData = client
                .get(&format!("availability/block/{block_num}/namespace/{ns_id}"))
                .send()
                .await
                .unwrap();

            // Verify namespace proof if present
            if let Some(ns_proof) = ns_query_res.proof {
                let vid_common: VidCommonQueryData<SeqTypes> = client
                    .get(&format!("availability/vid/common/{block_num}"))
                    .send()
                    .await
                    .unwrap();

                ns_proof
                    .verify(
                        header.ns_table(),
                        &header.payload_commitment(),
                        &vid_common.common().clone().unwrap(),
                    )
                    .unwrap();
            } else {
                // Namespace proof should be present if ns_id exists in ns_table
                assert!(header.ns_table().find_ns_id(&ns_id).is_none());
                assert!(ns_query_res.transactions.is_empty());
            }

            found_empty_block = found_empty_block || ns_query_res.transactions.is_empty();

            for txn in ns_query_res.transactions {
                if txn.commit() == hash {
                    // Ensure that we validate an inclusion proof
                    found_txn = true;
                }
            }
        }
        assert!(found_txn);
        assert!(found_empty_block);
    }

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn catchup_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        catchup_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[tokio::test(flavor = "multi_thread")]
    pub async fn test_non_consecutive_decide_with_failing_event_consumer<D>()
    where
        D: TestableSequencerDataSource + Debug + 'static,
    {
        #[derive(Clone, Copy, Debug)]
        struct FailConsumer;

        #[async_trait]
        impl EventConsumer for FailConsumer {
            async fn handle_event(&self, _: &Event<SeqTypes>) -> anyhow::Result<()> {
                bail!("mock error injection");
            }
        }

        setup_test();
        let (pubkey, privkey) = PubKey::generated_from_seed_indexed([0; 32], 1);

        let storage = D::create_storage().await;
        let persistence = D::persistence_options(&storage).create().await.unwrap();
        let data_source: Arc<StorageState<network::Memory, NoStorage, _, MockSequencerVersions>> =
            Arc::new(StorageState::new(
                D::create(D::persistence_options(&storage), Default::default(), false)
                    .await
                    .unwrap(),
                ApiState::new(future::pending()),
            ));

        // Create two non-consecutive leaf chains.
        let mut chain1 = vec![];

        let genesis = Leaf2::genesis::<TestVersions>(&Default::default(), &NodeState::mock()).await;
        let payload = genesis.block_payload().unwrap();
        let payload_bytes_arc = payload.encode();

        let avidm_param = init_avidm_param(2).unwrap();
        let weights = vec![1u32; 2];

        let ns_table = parse_ns_table(payload.byte_len().as_usize(), &payload.encode());
        let (payload_commitment, shares) =
            AvidMScheme::ns_disperse(&avidm_param, &weights, &payload_bytes_arc, ns_table).unwrap();

        let mut quorum_proposal = QuorumProposalWrapper::<SeqTypes> {
            proposal: QuorumProposal2::<SeqTypes> {
                block_header: genesis.block_header().clone(),
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate2::genesis::<MockSequencerVersions>(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await,
                upgrade_certificate: None,
                view_change_evidence: None,
                next_drb_result: None,
                next_epoch_justify_qc: None,
                epoch: None,
            },
        };
        let mut qc = QuorumCertificate2::genesis::<MockSequencerVersions>(
            &ValidatedState::default(),
            &NodeState::mock(),
        )
        .await;

        let mut justify_qc = qc.clone();
        for i in 0..5 {
            *quorum_proposal.proposal.block_header.height_mut() = i;
            quorum_proposal.proposal.view_number = ViewNumber::new(i);
            quorum_proposal.proposal.justify_qc = justify_qc;
            let leaf = Leaf2::from_quorum_proposal(&quorum_proposal);
            qc.view_number = leaf.view_number();
            qc.data.leaf_commit = Committable::commit(&leaf);
            justify_qc = qc.clone();
            chain1.push((leaf.clone(), qc.clone()));

            // Include a quorum proposal for each leaf.
            let quorum_proposal_signature =
                PubKey::sign(&privkey, &bincode::serialize(&quorum_proposal).unwrap())
                    .expect("Failed to sign quorum_proposal");
            persistence
                .append_quorum_proposal2(&Proposal {
                    data: quorum_proposal.clone(),
                    signature: quorum_proposal_signature,
                    _pd: Default::default(),
                })
                .await
                .unwrap();

            // Include VID information for each leaf.
            let share = VidDisperseShare2::<SeqTypes> {
                view_number: leaf.view_number(),
                payload_commitment,
                share: shares[0].clone(),
                recipient_key: pubkey,
                epoch: Some(EpochNumber::new(0)),
                target_epoch: Some(EpochNumber::new(0)),
            };
            persistence
                .append_vid2(&share.to_proposal(&privkey).unwrap())
                .await
                .unwrap();

            // Include payload information for each leaf.
            let block_payload_signature =
                PubKey::sign(&privkey, &payload_bytes_arc).expect("Failed to sign block payload");
            let da_proposal_inner = DaProposal2::<SeqTypes> {
                encoded_transactions: payload_bytes_arc.clone(),
                metadata: payload.ns_table().clone(),
                view_number: leaf.view_number(),
                epoch: Some(EpochNumber::new(0)),
            };
            let da_proposal = Proposal {
                data: da_proposal_inner,
                signature: block_payload_signature,
                _pd: Default::default(),
            };
            persistence
                .append_da2(&da_proposal, VidCommitment::V1(payload_commitment))
                .await
                .unwrap();
        }
        // Split into two chains.
        let mut chain2 = chain1.split_off(2);
        // Make non-consecutive (i.e. we skip a leaf).
        chain2.remove(0);

        // Decide 2 leaves, but fail in event processing.
        let leaf_chain = chain1
            .iter()
            .map(|(leaf, qc)| (leaf_info(leaf.clone()), qc.clone()))
            .collect::<Vec<_>>();
        tracing::info!("decide with event handling failure");
        persistence
            .append_decided_leaves(
                ViewNumber::new(1),
                leaf_chain.iter().map(|(leaf, qc)| (leaf, qc.clone())),
                &FailConsumer,
            )
            .await
            .unwrap();

        // Now decide remaining leaves successfully. We should now process a decide event for all
        // the leaves.
        let consumer = ApiEventConsumer::from(data_source.clone());
        let leaf_chain = chain2
            .iter()
            .map(|(leaf, qc)| (leaf_info(leaf.clone()), qc.clone()))
            .collect::<Vec<_>>();
        tracing::info!("decide successfully");
        persistence
            .append_decided_leaves(
                ViewNumber::new(4),
                leaf_chain.iter().map(|(leaf, qc)| (leaf, qc.clone())),
                &consumer,
            )
            .await
            .unwrap();

        // Check that the leaves were moved to archive storage, along with payload and VID
        // information.
        for (leaf, qc) in chain1.iter().chain(&chain2) {
            tracing::info!(height = leaf.height(), "check archive");
            let qd = data_source.get_leaf(leaf.height() as usize).await.await;
            let stored_leaf: Leaf2 = qd.leaf().clone();
            let stored_qc = qd.qc().clone();
            assert_eq!(&stored_leaf, leaf);
            assert_eq!(&stored_qc, qc);

            data_source
                .get_block(leaf.height() as usize)
                .await
                .try_resolve()
                .ok()
                .unwrap();
            data_source
                .get_vid_common(leaf.height() as usize)
                .await
                .try_resolve()
                .ok()
                .unwrap();

            // Check that all data has been garbage collected for the decided views.
            assert!(persistence
                .load_da_proposal(leaf.view_number())
                .await
                .unwrap()
                .is_none());
            assert!(persistence
                .load_vid_share(leaf.view_number())
                .await
                .unwrap()
                .is_none());
            assert!(persistence
                .load_quorum_proposal(leaf.view_number())
                .await
                .is_err());
        }

        // Check that data has _not_ been garbage collected for the missing view.
        assert!(persistence
            .load_da_proposal(ViewNumber::new(2))
            .await
            .unwrap()
            .is_some());
        assert!(persistence
            .load_vid_share(ViewNumber::new(2))
            .await
            .unwrap()
            .is_some());
        persistence
            .load_quorum_proposal(ViewNumber::new(2))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    pub async fn test_decide_missing_data<D>()
    where
        D: TestableSequencerDataSource + Debug + 'static,
    {
        setup_test();

        let storage = D::create_storage().await;
        let persistence = D::persistence_options(&storage).create().await.unwrap();
        let data_source: Arc<StorageState<network::Memory, NoStorage, _, MockSequencerVersions>> =
            Arc::new(StorageState::new(
                D::create(D::persistence_options(&storage), Default::default(), false)
                    .await
                    .unwrap(),
                ApiState::new(future::pending()),
            ));
        let consumer = ApiEventConsumer::from(data_source.clone());

        let mut qc = QuorumCertificate2::genesis::<MockSequencerVersions>(
            &ValidatedState::default(),
            &NodeState::mock(),
        )
        .await;
        let leaf =
            Leaf2::genesis::<TestVersions>(&ValidatedState::default(), &NodeState::mock()).await;

        // Append the genesis leaf. We don't use this for the test, because the update function will
        // automatically fill in the missing data for genesis. We just append this to get into a
        // consistent state to then append the leaf from view 1, which will have missing data.
        tracing::info!(?leaf, ?qc, "decide genesis leaf");
        persistence
            .append_decided_leaves(
                leaf.view_number(),
                [(&leaf_info(leaf.clone()), qc.clone())],
                &consumer,
            )
            .await
            .unwrap();

        // Create another leaf, with missing data.
        let mut block_header = leaf.block_header().clone();
        *block_header.height_mut() += 1;
        let qp = QuorumProposalWrapper {
            proposal: QuorumProposal2 {
                block_header,
                view_number: leaf.view_number() + 1,
                justify_qc: qc.clone(),
                upgrade_certificate: None,
                view_change_evidence: None,
                next_drb_result: None,
                next_epoch_justify_qc: None,
                epoch: None,
            },
        };

        let leaf = Leaf2::from_quorum_proposal(&qp);
        qc.view_number = leaf.view_number();
        qc.data.leaf_commit = Committable::commit(&leaf);

        // Decide a leaf without the corresponding payload or VID.
        tracing::info!(?leaf, ?qc, "append leaf 1");
        persistence
            .append_decided_leaves(
                leaf.view_number(),
                [(&leaf_info(leaf.clone()), qc)],
                &consumer,
            )
            .await
            .unwrap();

        // Check that we still processed the leaf.
        assert_eq!(leaf, data_source.get_leaf(1).await.await.leaf().clone());
        assert!(data_source.get_vid_common(1).await.is_pending());
        assert!(data_source.get_block(1).await.is_pending());
    }

    fn leaf_info(leaf: Leaf2) -> LeafInfo<SeqTypes> {
        LeafInfo {
            leaf,
            vid_share: None,
            state: Default::default(),
            delta: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::{BTreeMap, HashSet},
        time::Duration,
    };

    use committable::{Commitment, Committable};
    use espresso_types::{
        config::PublicHotShotConfig,
        traits::NullEventConsumer,
        v0_1::{UpgradeMode, ViewBasedUpgrade},
        BackoffParams, EpochVersion, FeeAccount, FeeAmount, Header, MarketplaceVersion,
        MockSequencerVersions, SequencerVersions, TimeBasedUpgrade, Timestamp, Upgrade,
        UpgradeType, ValidatedState,
    };
    use ethers::utils::Anvil;
    use futures::{
        future::{self, join_all},
        stream::{StreamExt, TryStreamExt},
    };
    use hotshot::types::EventType;
    use hotshot_query_service::{
        availability::{BlockQueryData, LeafQueryData, VidCommonQueryData},
        types::HeightIndexed,
    };
    use hotshot_types::{
        event::LeafInfo,
        traits::{metrics::NoMetrics, node_implementation::ConsensusTime},
        ValidatorConfig,
    };
    use jf_merkle_tree::prelude::{MerkleProof, Sha3Node};
    use portpicker::pick_unused_port;
    use sequencer_utils::{ser::FromStringOrInteger, test_utils::setup_test};
    use surf_disco::Client;
    use test_helpers::{
        catchup_test_helper, spawn_dishonest_peer_catchup_api, state_signature_test_helper,
        status_test_helper, submit_test_helper, TestNetwork, TestNetworkConfigBuilder,
    };
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};
    use time::OffsetDateTime;
    use tokio::time::sleep;
    use vbs::version::{StaticVersion, StaticVersionType, Version};

    use self::{
        data_source::testing::TestableSequencerDataSource, options::HotshotEvents,
        sql::DataSource as SqlDataSource,
    };
    use super::*;
    use crate::{
        catchup::{NullStateCatchup, StatePeers},
        persistence::no_storage,
        testing::{TestConfig, TestConfigBuilder},
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn test_healthcheck() {
        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url);
        let options = Options::with_port(port);
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::<5, _, NullStateCatchup>::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        client.connect(None).await;
        let health = client.get::<AppHealth>("healthcheck").send().await.unwrap();
        assert_eq!(health.status, HealthStatus::Available);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn status_test_without_query_module() {
        status_test_helper(|opt| opt).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn submit_test_without_query_module() {
        submit_test_helper(|opt| opt).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn state_signature_test_without_query_module() {
        state_signature_test_helper(|opt| opt).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn catchup_test_without_query_module() {
        catchup_test_helper(|opt| opt).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn slow_test_merklized_state_api() {
        setup_test();

        let port = pick_unused_port().expect("No ports free");

        let storage = SqlDataSource::create_storage().await;

        let options = SqlDataSource::options(&storage, Options::with_port(port));

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerApiVersion> = Client::new(url);

        client.connect(Some(Duration::from_secs(15))).await;

        // Wait until some blocks have been decided.
        tracing::info!("waiting for blocks");
        let blocks = client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(4)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        // sleep for few seconds so that state data is upserted
        tracing::info!("waiting for state to be inserted");
        sleep(Duration::from_secs(5)).await;
        network.stop_consensus().await;

        for block in blocks {
            let i = block.height();
            tracing::info!(i, "get block state");
            let path = client
                .get::<MerkleProof<Commitment<Header>, u64, Sha3Node, 3>>(&format!(
                    "block-state/{}/{i}",
                    i + 1
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.elem().unwrap(), block.hash());

            tracing::info!(i, "get fee state");
            let account = TestConfig::<5>::builder_key().fee_account();
            let path = client
                .get::<MerkleProof<FeeAmount, FeeAccount, Sha3Node, 256>>(&format!(
                    "fee-state/{}/{}",
                    i + 1,
                    account
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.index(), account);
            assert!(*path.elem().unwrap() > 0.into(), "{:?}", path.elem());
        }

        // testing fee_balance api
        let account = TestConfig::<5>::builder_key().fee_account();
        let amount = client
            .get::<Option<FeeAmount>>(&format!("fee-state/fee-balance/latest/{}", account))
            .send()
            .await
            .unwrap()
            .unwrap();
        let expected = ethers::types::U256::max_value();
        assert_eq!(expected, amount.0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_leaf_only_data_source() {
        setup_test();

        let port = pick_unused_port().expect("No ports free");

        let storage = SqlDataSource::create_storage().await;
        let options =
            SqlDataSource::leaf_only_ds_options(&storage, Options::with_port(port)).unwrap();

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerApiVersion> = Client::new(url);

        tracing::info!("waiting for blocks");
        client.connect(Some(Duration::from_secs(15))).await;
        // Wait until some blocks have been decided.

        let account = TestConfig::<5>::builder_key().fee_account();

        let _headers = client
            .socket("availability/stream/headers/0")
            .subscribe::<Header>()
            .await
            .unwrap()
            .take(10)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        for i in 1..5 {
            let leaf = client
                .get::<LeafQueryData<SeqTypes>>(&format!("availability/leaf/{i}"))
                .send()
                .await
                .unwrap();

            assert_eq!(leaf.height(), i);

            let header = client
                .get::<Header>(&format!("availability/header/{i}"))
                .send()
                .await
                .unwrap();

            assert_eq!(header.height(), i);

            let vid = client
                .get::<VidCommonQueryData<SeqTypes>>(&format!("availability/vid/common/{i}"))
                .send()
                .await
                .unwrap();

            assert_eq!(vid.height(), i);

            client
                .get::<MerkleProof<Commitment<Header>, u64, Sha3Node, 3>>(&format!(
                    "block-state/{i}/{}",
                    i - 1
                ))
                .send()
                .await
                .unwrap();

            client
                .get::<MerkleProof<FeeAmount, FeeAccount, Sha3Node, 256>>(&format!(
                    "fee-state/{}/{}",
                    i + 1,
                    account
                ))
                .send()
                .await
                .unwrap();
        }

        // This would fail even though we have processed atleast 10 leaves
        // this is because light weight nodes only support leaves, headers and VID
        client
            .get::<BlockQueryData<SeqTypes>>("availability/block/1")
            .send()
            .await
            .unwrap_err();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_catchup() {
        setup_test();

        // Start a sequencer network, using the query service for catchup.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::with_port(port))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .catchups(std::array::from_fn(|_| {
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .build();
        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        // Wait for replica 0 to reach a (non-genesis) decide, before disconnecting it.
        let mut events = network.peers[0].event_stream().await;
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            if leaf_chain[0].leaf.height() > 0 {
                break;
            }
        }

        // Shut down and restart replica 0. We don't just stop consensus and restart it; we fully
        // drop the node and recreate it so it loses all of its temporary state and starts off from
        // genesis. It should be able to catch up by listening to proposals and then rebuild its
        // state from its peers.
        tracing::info!("shutting down node");
        network.peers.remove(0);

        // Wait for a few blocks to pass while the node is down, so it falls behind.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        tracing::info!("restarting node");
        let node = network
            .cfg
            .init_node(
                1,
                ValidatedState::default(),
                no_storage::Options,
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                ),
                &NoMetrics,
                test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                NullEventConsumer,
                MockSequencerVersions::new(),
                Default::default(),
                "http://localhost".parse().unwrap(),
            )
            .await;
        let mut events = node.event_stream().await;

        // Wait for a (non-genesis) block proposed by each node, to prove that the lagging node has
        // caught up and all nodes are in sync.
        let mut proposers = [false; NUM_NODES];
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let height = leaf.height();
                let leaf_builder = (leaf.view_number().u64() as usize) % NUM_NODES;
                if height == 0 {
                    continue;
                }

                tracing::info!(
                    "waiting for blocks from {proposers:?}, block {height} is from {leaf_builder}",
                );
                proposers[leaf_builder] = true;
            }

            if proposers.iter().all(|has_proposed| *has_proposed) {
                break;
            }
        }
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_catchup_epochs() {
        setup_test();

        // Start a sequencer network, using the query service for catchup.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        const EPOCH_HEIGHT: u64 = 5;
        let network_config = TestConfigBuilder::default()
            .l1_url(l1)
            .epoch_height(EPOCH_HEIGHT)
            .build();
        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::with_port(port))
            .network_config(network_config)
            .catchups(std::array::from_fn(|_| {
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .build();
        let mut network = TestNetwork::new(
            config,
            SequencerVersions::<EpochVersion, EpochVersion>::new(),
        )
        .await;

        // Wait for replica 0 to decide in the third epoch.
        let mut events = network.peers[0].event_stream().await;
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            if leaf_chain[0].leaf.height() > EPOCH_HEIGHT * 3 {
                break;
            }
        }

        // Shut down and restart replica 0. We don't just stop consensus and restart it; we fully
        // drop the node and recreate it so it loses all of its temporary state and starts off from
        // genesis. It should be able to catch up by listening to proposals and then rebuild its
        // state from its peers.
        tracing::info!("shutting down node");
        network.peers.remove(0);

        // Wait for a few blocks to pass while the node is down, so it falls behind.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        tracing::info!("restarting node");
        let node = network
            .cfg
            .init_node(
                1,
                ValidatedState::default(),
                no_storage::Options,
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                ),
                &NoMetrics,
                test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                NullEventConsumer,
                MockSequencerVersions::new(),
                Default::default(),
                "http://localhost".parse().unwrap(),
            )
            .await;
        let mut events = node.event_stream().await;

        // Wait for a (non-genesis) block proposed by each node, to prove that the lagging node has
        // caught up and all nodes are in sync.
        let mut proposers = [false; NUM_NODES];
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let height = leaf.height();
                let leaf_builder = (leaf.view_number().u64() as usize) % NUM_NODES;
                if height == 0 {
                    continue;
                }

                tracing::info!(
                    "waiting for blocks from {proposers:?}, block {height} is from {leaf_builder}",
                );
                proposers[leaf_builder] = true;
            }

            if proposers.iter().all(|has_proposed| *has_proposed) {
                break;
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_chain_config_from_instance() {
        // This test uses a ValidatedState which only has the default chain config commitment.
        // The NodeState has the full chain config.
        // Both chain config commitments will match, so the ValidatedState should have the full chain config after a non-genesis block is decided.
        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let chain_config: ChainConfig = ChainConfig::default();

        let state = ValidatedState {
            chain_config: chain_config.commit().into(),
            ..Default::default()
        };

        let states = std::array::from_fn(|_| state.clone());

        let config = TestNetworkConfigBuilder::default()
            .api_config(Options::with_port(port))
            .states(states)
            .catchups(std::array::from_fn(|_| {
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();

        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        // Wait for few blocks to be decided.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        for peer in &network.peers {
            let state = peer.consensus().read().await.decided_state().await;

            assert_eq!(state.chain_config.resolve().unwrap(), chain_config)
        }

        network.server.shut_down().await;
        drop(network);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_chain_config_catchup() {
        // This test uses a ValidatedState with a non-default chain config
        // so it will be different from the NodeState chain config used by the TestNetwork.
        // However, for this test to work, at least one node should have a full chain config
        // to allow other nodes to catch up.

        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let cf = ChainConfig {
            max_block_size: 300.into(),
            base_fee: 1.into(),
            ..Default::default()
        };

        // State1 contains only the chain config commitment
        let state1 = ValidatedState {
            chain_config: cf.commit().into(),
            ..Default::default()
        };

        //state 2 contains the full chain config
        let state2 = ValidatedState {
            chain_config: cf.into(),
            ..Default::default()
        };

        let mut states = std::array::from_fn(|_| state1.clone());
        // only one node has the full chain config
        // all the other nodes should do a catchup to get the full chain config from peer 0
        states[0] = state2;

        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::from(options::Http {
                port,
                max_connections: None,
            }))
            .states(states)
            .catchups(std::array::from_fn(|_| {
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();

        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        // Wait for a few blocks to be decided.
        network
            .server
            .event_stream()
            .await
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        for peer in &network.peers {
            let state = peer.consensus().read().await.decided_state().await;

            assert_eq!(state.chain_config.resolve().unwrap(), cf)
        }

        network.server.shut_down().await;
        drop(network);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_chain_config_catchup_dishonest_peer() {
        // This test sets up a network of three nodes, each with the full chain config.
        // One of the nodes is connected to a dishonest peer.
        // When this node makes a chain config catchup request, it will result in an error due to the peer's malicious response.
        // The test also makes a catchup request for another node with an honest peer, which succeeds.
        // The requested chain config is based on the commitment from the validated state's chain config.
        // The dishonest peer responds with an invalid (malicious) chain config
        setup_test();

        const NUM_NODES: usize = 3;

        let (url, handle) = spawn_dishonest_peer_catchup_api().await.unwrap();

        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let cf = ChainConfig {
            max_block_size: 300.into(),
            base_fee: 1.into(),
            ..Default::default()
        };

        let state = ValidatedState {
            chain_config: cf.into(),
            ..Default::default()
        };

        let mut peers = std::array::from_fn(|_| {
            StatePeers::<SequencerApiVersion>::from_urls(
                vec![format!("http://localhost:{port}").parse().unwrap()],
                BackoffParams::default(),
                &NoMetrics,
            )
        });

        // one of the node has dishonest peer. This list of peers is for node#1
        peers[2] = StatePeers::<SequencerApiVersion>::from_urls(
            vec![url.clone()],
            BackoffParams::default(),
            &NoMetrics,
        );

        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::from(options::Http {
                port,
                max_connections: None,
            }))
            .states(std::array::from_fn(|_| state.clone()))
            .catchups(peers)
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();

        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        // Test a catchup request for node #0, which is connected to an honest peer.
        // The catchup should successfully retrieve the correct chain config.
        let node = &network.peers[0];
        let peers = node.node_state().peers;
        peers.try_fetch_chain_config(0, cf.commit()).await.unwrap();

        // Test a catchup request for node #1, which is connected to a dishonest peer.
        // This request will result in an error due to the malicious chain config provided by the peer.
        let node = &network.peers[1];
        let peers = node.node_state().peers;
        peers
            .try_fetch_chain_config(0, cf.commit())
            .await
            .unwrap_err();

        network.server.shut_down().await;
        handle.abort();
    }

    // #[tokio::test(flavor = "multi_thread")]
    // async fn test_pos_upgrade_view_based() {
    //     setup_test();

    //     let mut upgrades = std::collections::BTreeMap::new();
    //     type MySequencerVersions = SequencerVersions<FeeVersion, EpochVersion>;

    //     let mode = UpgradeMode::View(ViewBasedUpgrade {
    //         start_voting_view: None,
    //         stop_voting_view: None,
    //         start_proposing_view: 1,
    //         stop_proposing_view: 10,
    //     });

    //     let upgrade_type = UpgradeType::Epoch {
    //         chain_config: ChainConfig {
    //             max_block_size: 500.into(),
    //             base_fee: 2.into(),
    //             stake_table_contract: Some(Default::default()),
    //             ..Default::default()
    //         },
    //     };

    //     upgrades.insert(
    //         <MySequencerVersions as Versions>::Upgrade::VERSION,
    //         Upgrade { mode, upgrade_type },
    //     );
    //     test_upgrade_helper::<MySequencerVersions>(upgrades, MySequencerVersions::new()).await;
    // }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_marketplace_upgrade_view_based() {
        setup_test();

        let mut upgrades = std::collections::BTreeMap::new();
        type MySequencerVersions = SequencerVersions<EpochVersion, MarketplaceVersion>;

        let mode = UpgradeMode::View(ViewBasedUpgrade {
            start_voting_view: None,
            stop_voting_view: None,
            start_proposing_view: 1,
            stop_proposing_view: 10,
        });

        let upgrade_type = UpgradeType::Marketplace {
            chain_config: ChainConfig {
                max_block_size: 400.into(),
                base_fee: 2.into(),
                bid_recipient: Some(Default::default()),
                ..Default::default()
            },
        };

        upgrades.insert(
            <MySequencerVersions as Versions>::Upgrade::VERSION,
            Upgrade { mode, upgrade_type },
        );
        test_upgrade_helper::<MySequencerVersions>(upgrades, MySequencerVersions::new()).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_marketplace_upgrade_time_based() {
        setup_test();

        let now = OffsetDateTime::now_utc().unix_timestamp() as u64;

        let mut upgrades = std::collections::BTreeMap::new();
        type MySequencerVersions = SequencerVersions<EpochVersion, MarketplaceVersion>;

        let mode = UpgradeMode::Time(TimeBasedUpgrade {
            start_proposing_time: Timestamp::from_integer(now).unwrap(),
            stop_proposing_time: Timestamp::from_integer(now + 500).unwrap(),
            start_voting_time: None,
            stop_voting_time: None,
        });

        let upgrade_type = UpgradeType::Marketplace {
            chain_config: ChainConfig {
                max_block_size: 400.into(),
                base_fee: 2.into(),
                bid_recipient: Some(Default::default()),
                ..Default::default()
            },
        };

        upgrades.insert(
            <MySequencerVersions as Versions>::Upgrade::VERSION,
            Upgrade { mode, upgrade_type },
        );
        test_upgrade_helper::<MySequencerVersions>(upgrades, MySequencerVersions::new()).await;
    }

    async fn test_upgrade_helper<MockSeqVersions: Versions>(
        upgrades: BTreeMap<Version, Upgrade>,
        bind_version: MockSeqVersions,
    ) {
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let chain_config_upgrade = upgrades
            .get(&<MockSeqVersions as Versions>::Upgrade::VERSION)
            .unwrap()
            .upgrade_type
            .chain_config()
            .unwrap();

        const NUM_NODES: usize = 5;
        let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
            .api_config(Options::from(options::Http {
                port,
                max_connections: None,
            }))
            .catchups(std::array::from_fn(|_| {
                StatePeers::<SequencerApiVersion>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .network_config(
                TestConfigBuilder::default()
                    .l1_url(l1)
                    .upgrades::<MockSeqVersions>(upgrades)
                    .build(),
            )
            .build();

        let mut network = TestNetwork::new(config, bind_version).await;

        let mut events = network.server.event_stream().await;

        // First loop to get an `UpgradeProposal`. Note that the
        // actual upgrade will take several subsequent views for
        // voting and finally the actual upgrade.
        let new_version_first_view = loop {
            let event = events.next().await.unwrap();
            match event.event {
                EventType::UpgradeProposal { proposal, .. } => {
                    let upgrade = proposal.data.upgrade_proposal;
                    let new_version = upgrade.new_version;
                    assert_eq!(new_version, <MockSeqVersions as Versions>::Upgrade::VERSION);
                    break upgrade.new_version_first_view;
                },
                _ => continue,
            }
        };
        tracing::info!(?new_version_first_view, "seen upgrade proposal");

        let client: Client<ServerError, SequencerApiVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Loop to wait on the upgrade itself.
        loop {
            // Get height as a proxy for view number. Height is always
            // >= to view. Especially when using Anvil, there should be little
            // difference. As a possible alternative we might loop on
            // hotshot events here again and pull the view number off
            // the event.
            let height = client
                .get::<ViewNumber>("status/block-height")
                .send()
                .await
                .unwrap();

            let states: Vec<_> = network
                .peers
                .iter()
                .map(|peer| async { peer.consensus().read().await.decided_state().await })
                .collect();

            let configs: Option<Vec<ChainConfig>> = join_all(states)
                .await
                .iter()
                .map(|state| state.chain_config.resolve())
                .collect();

            tracing::info!(?height, ?new_version_first_view, "checking config");

            // ChainConfigs will eventually be resolved
            if let Some(configs) = configs {
                tracing::info!(?configs, "configs");
                if height > new_version_first_view + 10 {
                    for config in configs {
                        assert_eq!(config, chain_config_upgrade);
                    }
                    break; // if assertion did not panic, we need to exit the loop
                }
            }
            sleep(Duration::from_millis(200)).await;
        }

        network.server.shut_down().await;
    }

    #[tokio::test(flavor = "multi_thread")]
    pub(crate) async fn test_restart() {
        setup_test();

        const NUM_NODES: usize = 5;
        // Initialize nodes.
        let storage = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
        let persistence: [_; NUM_NODES] = storage
            .iter()
            .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let port = pick_unused_port().unwrap();
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let config = TestNetworkConfigBuilder::default()
            .api_config(SqlDataSource::options(
                &storage[0],
                Options::with_port(port),
            ))
            .persistences(persistence.clone())
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();
        let mut network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        // Connect client.
        let client: Client<ServerError, SequencerApiVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Wait until some blocks have been decided.
        client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(3)
            .collect::<Vec<_>>()
            .await;

        // Shut down the consensus nodes.
        tracing::info!("shutting down nodes");
        network.stop_consensus().await;

        // Get the block height we reached.
        let height = client
            .get::<usize>("status/block-height")
            .send()
            .await
            .unwrap();
        tracing::info!("decided {height} blocks before shutting down");

        // Get the decided chain, so we can check consistency after the restart.
        let chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        let decided_view = chain.last().unwrap().leaf().view_number();

        // Get the most recent state, for catchup.

        let state = network.server.decided_state().await;
        tracing::info!(?decided_view, ?state, "consensus state");

        // Fully shut down the API servers.
        drop(network);

        // Start up again, resuming from the last decided leaf.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();

        let config = TestNetworkConfigBuilder::default()
            .api_config(SqlDataSource::options(
                &storage[0],
                Options::with_port(port),
            ))
            .persistences(persistence)
            .catchups(std::array::from_fn(|_| {
                // Catchup using node 0 as a peer. Node 0 was running the archival state service
                // before the restart, so it should be able to resume without catching up by loading
                // state from storage.
                StatePeers::<StaticVersion<0, 1>>::from_urls(
                    vec![format!("http://localhost:{port}").parse().unwrap()],
                    Default::default(),
                    &NoMetrics,
                )
            }))
            .network_config(TestConfigBuilder::default().l1_url(l1).build())
            .build();
        let _network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let client: Client<ServerError, StaticVersion<0, 1>> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Make sure we can decide new blocks after the restart.
        tracing::info!("waiting for decide, height {height}");
        let new_leaf: LeafQueryData<SeqTypes> = client
            .socket(&format!("availability/stream/leaves/{height}"))
            .subscribe()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_leaf.height(), height as u64);
        assert_eq!(
            new_leaf.leaf().parent_commitment(),
            chain[height - 1].hash()
        );

        // Ensure the new chain is consistent with the old chain.
        let new_chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(chain, new_chain);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_fetch_config() {
        setup_test();

        let port = pick_unused_port().expect("No ports free");
        let url: surf_disco::Url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, StaticVersion<0, 1>> = Client::new(url.clone());

        let options = Options::with_port(port).config(Default::default());
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        client.connect(None).await;

        // Fetch a network config from the API server. The first peer URL is bogus, to test the
        // failure/retry case.
        let peers = StatePeers::<StaticVersion<0, 1>>::from_urls(
            vec!["https://notarealnode.network".parse().unwrap(), url],
            Default::default(),
            &NoMetrics,
        );

        // Fetch the config from node 1, a different node than the one running the service.
        let validator = ValidatorConfig::generated_from_seed_indexed([0; 32], 1, 1, false);
        let config = peers.fetch_config(validator.clone()).await.unwrap();

        // Check the node-specific information in the recovered config is correct.
        assert_eq!(config.node_index, 1);

        // Check the public information is also correct (with respect to the node that actually
        // served the config, for public keys).
        pretty_assertions::assert_eq!(
            serde_json::to_value(PublicHotShotConfig::from(config.config)).unwrap(),
            serde_json::to_value(PublicHotShotConfig::from(
                network.cfg.hotshot_config().clone()
            ))
            .unwrap()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_hotshot_event_streaming() {
        setup_test();

        let hotshot_event_streaming_port =
            pick_unused_port().expect("No ports free for hotshot event streaming");
        let query_service_port = pick_unused_port().expect("No ports free for query service");

        let url = format!("http://localhost:{hotshot_event_streaming_port}")
            .parse()
            .unwrap();

        let hotshot_events = HotshotEvents {
            events_service_port: hotshot_event_streaming_port,
        };

        let client: Client<ServerError, SequencerApiVersion> = Client::new(url);

        let options = Options::with_port(query_service_port).hotshot_events(hotshot_events);

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        let mut subscribed_events = client
            .socket("hotshot-events/events")
            .subscribe::<Event<SeqTypes>>()
            .await
            .unwrap();

        let total_count = 5;
        // wait for these events to receive on client 1
        let mut receive_count = 0;
        loop {
            let event = subscribed_events.next().await.unwrap();
            dbg!(&event);
            tracing::info!(
                "Received event in hotshot event streaming Client 1: {:?}",
                event
            );
            receive_count += 1;
            if receive_count > total_count {
                tracing::info!("Client Received at least desired events, exiting loop");
                break;
            }
        }
        assert_eq!(receive_count, total_count + 1);
    }
    // TODO unfinished test. the idea is to observe epochs and views
    // are progressing in a sane way
    #[tokio::test(flavor = "multi_thread")]
    async fn test_hotshot_event_streaming_epoch_progression() {
        setup_test();

        let epoch_height = 5;
        type PosVersion = SequencerVersions<StaticVersion<0, 3>, StaticVersion<0, 0>>;

        let hotshot_event_streaming_port =
            pick_unused_port().expect("No ports free for hotshot event streaming");
        let query_service_port = pick_unused_port().expect("No ports free for query service");

        let url = format!("http://localhost:{hotshot_event_streaming_port}")
            .parse()
            .unwrap();

        let hotshot_events = HotshotEvents {
            events_service_port: hotshot_event_streaming_port,
        };

        let client: Client<ServerError, SequencerApiVersion> = Client::new(url);

        let options = Options::with_port(query_service_port).hotshot_events(hotshot_events);

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default()
            .l1_url(l1)
            .with_epoch_height(epoch_height)
            .build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let _network = TestNetwork::new(config, PosVersion::new()).await;

        let mut subscribed_events = client
            .socket("hotshot-events/events")
            .subscribe::<Event<SeqTypes>>()
            .await
            .unwrap();

        // wanted views
        let total_count = epoch_height * 2;
        // wait for these events to receive on client 1
        let mut views = HashSet::new();
        let mut i = 0;
        loop {
            let event = subscribed_events.next().await.unwrap();
            let event = event.unwrap();
            let view_number = event.view_number;
            views.insert(view_number.u64());

            if let hotshot::types::EventType::Decide { .. } = event.event {
                dbg!("got decide");
            }
            if views.contains(&total_count) {
                tracing::info!("Client Received at least desired views, exiting loop");
                break;
            }
            if i > 100 {
                // Timeout
                panic!("Views are not progressing");
            }
            i += 1;
        }
        assert!(views.contains(&total_count));
    }
}
