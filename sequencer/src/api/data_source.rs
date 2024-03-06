use super::{
    endpoints::TimeWindowQueryData,
    fs,
    options::{Options, Query},
    sql,
};
use crate::{network, persistence, state::ValidatedState, Node, SeqTypes};
use async_std::sync::Arc;
use async_trait::async_trait;
use hotshot::types::SystemContextHandle;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId},
    data_source::{UpdateDataSource, VersionedDataSource},
    fetching::provider::{AnyProvider, QueryServiceProvider},
    node::NodeDataSource,
    status::StatusDataSource,
    QueryResult,
};
use hotshot_types::{data::ViewNumber, light_client::StateSignatureRequestBody};
use tide_disco::Url;
use versioned_binary_serialization::version::StaticVersion;

pub trait DataSourceOptions: persistence::PersistenceOptions {
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

    /// Update sequencer-specific indices when a new block is added.
    ///
    /// `from_block` should be the height of the chain the last time `refresh_indices` was called.
    /// Any blocks in the data sources with number `from_block` or greater will be incorporated into
    /// sequencer-specific data structures.
    async fn refresh_indices(&mut self, from_block: usize) -> anyhow::Result<()>;

    /// Retrieve a list of blocks whose timestamps fall within the window [start, end).
    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData>;

    /// Retrieve a list of blocks starting from `from` with timestamps less than `end`.
    async fn window_from<ID>(&self, from: ID, end: u64) -> QueryResult<TimeWindowQueryData>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync;
}

/// Provider for fetching missing data for the query service.
pub type Provider = AnyProvider<SeqTypes>;

/// Create a provider for fetching missing data from a list of peer query services.
pub fn provider<const MAJOR_VERSION: u16, const MINOR_VERSION: u16>(
    peers: impl IntoIterator<Item = Url>,
    bind_version: StaticVersion<MAJOR_VERSION, MINOR_VERSION>,
) -> Provider {
    let mut provider = Provider::default();
    for peer in peers {
        tracing::info!("will fetch missing data from {peer}");
        provider = provider.with_provider(QueryServiceProvider::new(peer, bind_version));
    }
    provider
}

pub(crate) trait SubmitDataSource<N: network::Type> {
    fn consensus(&self) -> &SystemContextHandle<SeqTypes, Node<N>>;
}

#[async_trait]
pub(crate) trait StateSignatureDataSource<N: network::Type> {
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody>;
}

#[trait_variant::make(StateDataSource: Send)]
pub(crate) trait LocalStateDataSource {
    async fn get_decided_state(&self) -> Arc<ValidatedState>;
    async fn get_undecided_state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>>;
}

#[cfg(test)]
pub(crate) mod testing {
    use super::super::Options;
    use super::*;
    use crate::persistence::SequencerPersistence;
    use std::fmt::Debug;

    #[async_trait]
    pub(crate) trait TestableSequencerDataSource: SequencerDataSource {
        type Storage;
        type Persistence: Debug + SequencerPersistence;

        async fn create_storage() -> Self::Storage;
        async fn connect(storage: &Self::Storage) -> Self::Persistence;
        fn options(storage: &Self::Storage, opt: Options) -> Options;
    }
}
