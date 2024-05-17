use super::{
    fs,
    options::{Options, Query},
    sql, AccountQueryData, BlocksFrontier,
};
use crate::{
    network,
    persistence::{self, SequencerPersistence},
    PubKey, SeqTypes, Transaction,
};
use anyhow::bail;
use async_trait::async_trait;
use ethers::prelude::Address;
use futures::future::Future;
use hotshot_query_service::{
    availability::AvailabilityDataSource,
    data_source::{MetricsDataSource, UpdateDataSource, VersionedDataSource},
    fetching::provider::{AnyProvider, QueryServiceProvider},
    node::NodeDataSource,
    status::StatusDataSource,
};
use hotshot_types::{
    data::ViewNumber, light_client::StateSignatureRequestBody, HotShotConfig, ValidatorConfig,
};
use serde::ser::SerializeStruct;
use serde::Serialize;
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

pub(crate) trait SubmitDataSource<N: network::Type, P: SequencerPersistence> {
    fn submit(&self, tx: Transaction) -> impl Send + Future<Output = anyhow::Result<()>>;
}

pub(crate) trait HotShotConfigDataSource {
    fn get_config(&self) -> impl Send + Future<Output = anyhow::Result<Option<MyHotShotConfig>>>;
}

#[async_trait]
pub(crate) trait StateSignatureDataSource<N: network::Type> {
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody>;
}

pub(crate) trait CatchupDataSource {
    /// Get the state of the requested `account`.
    ///
    /// The state is fetched from a snapshot at the given height and view, which _must_ correspond!
    /// `height` is provided to simplify lookups for backends where data is not indexed by view.
    /// This function is intended to be used for catchup, so `view` should be no older than the last
    /// decided view.
    fn get_account(
        &self,
        _height: u64,
        _view: ViewNumber,
        _account: Address,
    ) -> impl Send + Future<Output = anyhow::Result<AccountQueryData>> {
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
}

impl CatchupDataSource for MetricsDataSource {}

pub struct MyHotShotConfig(HotShotConfig<PubKey>);

impl MyHotShotConfig {
    pub fn new(c: HotShotConfig<PubKey>) -> Self {
        Self(c)
    }
}
pub struct MyValidatorConfig<'a>(&'a ValidatorConfig<PubKey>);

impl<'a> Serialize for MyValidatorConfig<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("MyValidatorConfig", 3)?;

        state.serialize_field("public_key", &self.0.public_key)?;
        state.serialize_field("stake_value", &self.0.stake_value)?;
        state.serialize_field("is_da", &self.0.is_da)?;

        state.end()
    }
}

impl Serialize for MyHotShotConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("HotShotConfig", 20)?;
        let config = &self.0;
        state.serialize_field("execution_type", &config.execution_type)?;
        state.serialize_field("start_threshold", &config.start_threshold)?;
        state.serialize_field("num_nodes_with_stake", &config.num_nodes_with_stake)?;
        state.serialize_field("num_nodes_without_stake", &config.num_nodes_without_stake)?;
        state.serialize_field("known_nodes_with_stake", &config.known_nodes_with_stake)?;
        state.serialize_field("known_da_nodes", &config.known_da_nodes)?;
        state.serialize_field(
            "known_nodes_without_stake",
            &config.known_nodes_without_stake,
        )?;

        let validator_confg = MyValidatorConfig(&config.my_own_validator_config);

        state.serialize_field("my_own_validator_config", &validator_confg)?;
        state.serialize_field("da_staked_committee_size", &config.da_staked_committee_size)?;
        state.serialize_field(
            "da_non_staked_committee_size",
            &config.da_non_staked_committee_size,
        )?;
        state.serialize_field("fixed_leader_for_gpuvid", &config.fixed_leader_for_gpuvid)?;
        state.serialize_field("next_view_timeout", &config.next_view_timeout)?;
        state.serialize_field("view_sync_timeout", &config.view_sync_timeout)?;
        state.serialize_field("timeout_ratio", &config.timeout_ratio)?;
        state.serialize_field("round_start_delay", &config.round_start_delay)?;
        state.serialize_field("start_delay", &config.start_delay)?;
        state.serialize_field("num_bootstrap", &config.num_bootstrap)?;
        state.serialize_field("builder_timeout", &config.builder_timeout)?;
        state.serialize_field("data_request_delay", &config.data_request_delay)?;
        state.serialize_field("builder_url", &config.builder_url)?;

        state.end()
    }
}

#[cfg(test)]
pub(crate) mod testing {
    use super::super::Options;
    use super::*;

    #[async_trait]
    pub(crate) trait TestableSequencerDataSource: SequencerDataSource {
        type Storage: Sync;

        async fn create_storage() -> Self::Storage;
        fn persistence_options(storage: &Self::Storage) -> Self::Options;
        fn options(storage: &Self::Storage, opt: Options) -> Options;
    }
}
