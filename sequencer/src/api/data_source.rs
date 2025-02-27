use anyhow::Context;
use async_trait::async_trait;
use committable::Commitment;
use espresso_types::{
    v0::traits::{PersistenceOptions, SequencerPersistence},
    v0_99::ChainConfig,
    FeeAccount, FeeAccountProof, FeeMerkleTree, NodeState, PubKey, PublicNetworkConfig,
    Transaction,
};
use futures::future::Future;
use hotshot_query_service::{
    availability::AvailabilityDataSource,
    data_source::{UpdateDataSource, VersionedDataSource},
    fetching::provider::{AnyProvider, QueryServiceProvider},
    node::NodeDataSource,
    status::StatusDataSource,
};
use hotshot_types::traits::node_implementation::NodeType;
use hotshot_types::{
    data::ViewNumber,
    light_client::StateSignatureRequestBody,
    stake_table::StakeTableEntry,
    traits::{network::ConnectedNetwork, node_implementation::Versions},
};

use tide_disco::Url;

use super::{
    fs,
    options::{Options, Query},
    sql, AccountQueryData, BlocksFrontier,
};
use crate::{
    persistence::{self},
    SeqTypes, SequencerApiVersion,
};

pub trait DataSourceOptions: PersistenceOptions {
    type DataSource: SequencerDataSource<Options = Self>;

    fn enable_query_module(&self, opt: Options, query: Query) -> Options;
}

impl DataSourceOptions for persistence::sql::Options {
    type DataSource = sql::DataSource;

    fn enable_query_module(&self, opt: Options, query: Query) -> Options {
        opt.query_sql(query, self.clone())
    }
}

impl DataSourceOptions for persistence::fs::Options {
    type DataSource = fs::DataSource;

    fn enable_query_module(&self, opt: Options, query: Query) -> Options {
        opt.query_fs(query, self.clone())
    }
}

/// A data source with sequencer-specific functionality.
///
/// This trait extends the generic [`AvailabilityDataSource`] with some additional data needed to
/// provided sequencer-specific endpoints.
#[async_trait]
pub trait SequencerDataSource:
    AvailabilityDataSource<SeqTypes>
    + NodeDataSource<SeqTypes>
    + StatusDataSource
    + UpdateDataSource<SeqTypes>
    + VersionedDataSource
    + Sized
{
    type Options: DataSourceOptions<DataSource = Self>;

    /// Instantiate a data source from command line options.
    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self>;
}

/// Provider for fetching missing data for the query service.
pub type Provider = AnyProvider<SeqTypes>;

/// Create a provider for fetching missing data from a list of peer query services.
pub fn provider<V: Versions>(
    peers: impl IntoIterator<Item = Url>,
    bind_version: SequencerApiVersion,
) -> Provider {
    let mut provider = Provider::default();
    for peer in peers {
        tracing::info!("will fetch missing data from {peer}");
        provider = provider.with_provider(QueryServiceProvider::new(peer, bind_version));
    }
    provider
}

pub(crate) trait SubmitDataSource<N: ConnectedNetwork<PubKey>, P: SequencerPersistence> {
    fn submit(&self, tx: Transaction) -> impl Send + Future<Output = anyhow::Result<()>>;
}

pub(crate) trait HotShotConfigDataSource {
    fn get_config(&self) -> impl Send + Future<Output = PublicNetworkConfig>;
}

#[async_trait]
pub(crate) trait StateSignatureDataSource<N: ConnectedNetwork<PubKey>> {
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody>;
}

pub(crate) trait NodeStateDataSource {
    fn node_state(&self) -> impl Send + Future<Output = &NodeState>;
}

pub(crate) trait StakeTableDataSource<T: NodeType> {
    /// Get the stake table for a given epoch
    fn get_stake_table(
        &self,
        epoch: Option<<T as NodeType>::Epoch>,
    ) -> impl Send + Future<Output = Vec<StakeTableEntry<T::SignatureKey>>>;

    /// Get the stake table for  the current epoch if not provided
    fn get_stake_table_current(
        &self,
    ) -> impl Send + Future<Output = Vec<StakeTableEntry<T::SignatureKey>>>;

    fn get_current_epoch(&self) -> impl Send + Future<Output = Option<T::Epoch>>;

    fn get_da_members(
        &self,
        epoch: Option<<SeqTypes as NodeType>::Epoch>,
    ) -> impl Send + Future<Output = Vec<StakeTableEntry<<SeqTypes as NodeType>::SignatureKey>>>;
    /// Get the stake table for the current epoch if not provided
    fn get_da_members_current(
        &self,
    ) -> impl Send + Future<Output = Vec<StakeTableEntry<<SeqTypes as NodeType>::SignatureKey>>>;
}

pub(crate) trait CatchupDataSource: Sync {
    /// Get the state of the requested `account`.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    fn get_account(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        account: FeeAccount,
    ) -> impl Send + Future<Output = anyhow::Result<AccountQueryData>> {
        async move {
            let tree = self
                .get_accounts(instance, height, view, &[account])
                .await?;
            let (proof, balance) = FeeAccountProof::prove(&tree, account.into()).context(
                format!("account {account} not available for height {height}, view {view:?}"),
            )?;
            Ok(AccountQueryData { balance, proof })
        }
    }

    /// Get the state of the requested `accounts`.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> impl Send + Future<Output = anyhow::Result<FeeMerkleTree>>;

    /// Get the blocks Merkle tree frontier.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    fn get_frontier(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> impl Send + Future<Output = anyhow::Result<BlocksFrontier>>;

    fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> impl Send + Future<Output = anyhow::Result<ChainConfig>>;
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::{super::Options, *};

    #[async_trait]
    pub trait TestableSequencerDataSource: SequencerDataSource {
        type Storage: Sync;

        async fn create_storage() -> Self::Storage;
        fn persistence_options(storage: &Self::Storage) -> Self::Options;
        fn leaf_only_ds_options(
            _storage: &Self::Storage,
            _opt: Options,
        ) -> anyhow::Result<Options> {
            anyhow::bail!("not supported")
        }
        fn options(storage: &Self::Storage, opt: Options) -> Options;
    }
}
