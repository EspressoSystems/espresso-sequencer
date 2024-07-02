use super::{
    fs,
    options::{Options, Query},
    sql,
};
use crate::{
    network,
    persistence::{self, SequencerPersistence},
    state::ValidatedState,
    SeqTypes, Transaction,
};
use async_std::sync::Arc;
use async_trait::async_trait;
use hotshot_query_service::{
    availability::AvailabilityDataSource,
    data_source::{UpdateDataSource, VersionedDataSource},
    fetching::provider::{AnyProvider, QueryServiceProvider},
    node::NodeDataSource,
    status::StatusDataSource,
};
use hotshot_types::{data::ViewNumber, light_client::StateSignatureRequestBody};
use tide_disco::Url;
use vbs::version::StaticVersionType;

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
}

/// Provider for fetching missing data for the query service.
pub type Provider = AnyProvider<SeqTypes>;

/// Create a provider for fetching missing data from a list of peer query services.
pub fn provider<Ver: StaticVersionType + 'static>(
    peers: impl IntoIterator<Item = Url>,
    bind_version: Ver,
) -> Provider {
    let mut provider = Provider::default();
    for peer in peers {
        tracing::info!("will fetch missing data from {peer}");
        provider = provider.with_provider(QueryServiceProvider::new(peer, bind_version));
    }
    provider
}

#[trait_variant::make(SubmitDataSource: Send)]
pub(crate) trait LocalSubmitDataSource<N: network::Type, P: SequencerPersistence> {
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()>;
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
