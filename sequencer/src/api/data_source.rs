use super::{
    fs,
    options::{Options, Query},
    sql,
};
use crate::{
    network,
    persistence::{self, SequencerPersistence},
    state::{BlockMerkleTree, Delta, FeeAccount, FeeMerkleTree, ValidatedState},
    Node, SeqTypes,
};
use anyhow::Context;
use async_std::sync::Arc;
use async_trait::async_trait;
use hotshot::types::SystemContextHandle;
use hotshot_query_service::{
    availability::AvailabilityDataSource,
    data_source::{UpdateDataSource, VersionedDataSource},
    fetching::provider::{AnyProvider, QueryServiceProvider},
    merklized_state::{MerklizedState, UpdateStateStorage},
    node::NodeDataSource,
    status::StatusDataSource,
    Leaf,
};
use hotshot_types::{data::ViewNumber, light_client::StateSignatureRequestBody};
use jf_primitives::merkle_tree::{
    prelude::MerklePath, MerkleTreeScheme, ToTraversalPath, UniversalMerkleTreeScheme,
};
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
    /// Wrapper function to store merkle nodes
    async fn store_state<S: MerklizedState<SeqTypes, A>, const A: usize>(
        &mut self,
        path: MerklePath<S::Entry, S::Key, S::T>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> anyhow::Result<()>;
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
    fn consensus(&self) -> &SystemContextHandle<SeqTypes, Node<N, P>>;
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

#[async_trait]
impl<D: SequencerDataSource + Send + Sync> UpdateStateStorage<SeqTypes, D> for ValidatedState {
    async fn update_storage(
        &self,
        storage: &mut D,
        leaf: &Leaf<SeqTypes>,
        delta: Arc<Delta>,
    ) -> anyhow::Result<()> {
        let block_number = leaf.get_height();
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
        } = self;

        let Delta { fees_delta } = delta.as_ref();

        // Insert block merkle tree nodes
        let (_, proof) = block_merkle_tree
            .lookup(block_number - 1)
            .expect_ok()
            .context("Index not found in block merkle tree")?;
        let path = <u64 as ToTraversalPath<3>>::to_traversal_path(
            &(block_number - 1),
            block_merkle_tree.height(),
        );

        storage
            .store_state::<BlockMerkleTree, 3>(proof.proof, path, block_number)
            .await
            .context("failed to insert merkle nodes for block merkle tree")?;

        // Insert fee merkle tree nodes
        for delta in fees_delta {
            let (_, proof) = fee_merkle_tree
                .universal_lookup(delta)
                .expect_ok()
                .context("Index not found in fee merkle tree")?;
            let path: Vec<usize> =
                <FeeAccount as ToTraversalPath<256>>::to_traversal_path(
                    delta,
                    fee_merkle_tree.height(),
                );

            storage
                .store_state::<FeeMerkleTree, 256>(proof.proof, path, block_number)
                .await
                .context("failed to insert merkle nodes for block merkle tree")?;
        }

        Ok(())
    }
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
