use super::{
    storage::{
        pruning::{PruneStorage, PrunedHeightDataSource, PrunedHeightStorage},
        AvailabilityStorage, MerklizedStateHeightStorage, MerklizedStateStorage, NodeStorage,
        UpdateAvailabilityStorage,
    },
    Transaction, VersionedDataSource,
};
use crate::data_source::fetching::Builder;
use crate::data_source::fetching::Fetcher;
use crate::data_source::fetching::Pruner;
use crate::data_source::fetching::Storable;
use crate::data_source::AvailabilityProvider;
use crate::{
    availability::{
        AvailabilityDataSource, BlockId, BlockInfo, BlockQueryData, Fetch, FetchStream, LeafId,
        LeafQueryData, PayloadMetadata, PayloadQueryData, QueryableHeader, QueryablePayload,
        TransactionHash, TransactionQueryData, UpdateAvailabilityData, VidCommonMetadata,
        VidCommonQueryData,
    },
    merklized_state::{
        MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence, Snapshot,
    },
    metrics::PrometheusMetrics,
    node::{NodeDataSource, SyncStatus, TimeWindowQueryData, WindowStart},
    status::{HasMetrics, StatusDataSource},
    Header, Payload, QueryError, QueryResult, VidShare,
};
use async_trait::async_trait;
use backoff::backoff::Backoff;
use derivative::Derivative;
use futures::stream::{BoxStream, StreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use jf_merkle_tree::{prelude::MerkleProof, MerkleTreeScheme};
use std::sync::Arc;
use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use tokio::time::sleep;

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = "S: Debug, P: Debug"))]
pub struct LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
{
    fetcher: Arc<Fetcher<Types, S, P>>,
    pruner: Pruner<Types, S>,
}

impl<Types, S, P> LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types>,
    S: VersionedDataSource + PruneStorage + HasMetrics + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityStorage<Types>,
    for<'a> S::ReadOnly<'a>: AvailabilityStorage<Types> + NodeStorage<Types> + PrunedHeightStorage,
    P: AvailabilityProvider<Types>,
{
    pub async fn new(builder: Builder<Types, S, P>) -> anyhow::Result<Self> {
        let fetcher = Arc::new(Fetcher::new(builder.with_lightweight()).await?);
        let storage = fetcher.storage.clone();

        let pruner = Pruner::new(storage).await;

        Ok(Self { fetcher, pruner })
    }
}

impl<Types, S, P> VersionedDataSource for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + Send + Sync,
    P: Send + Sync,
{
    type Transaction<'a>
        = S::Transaction<'a>
    where
        Self: 'a;
    type ReadOnly<'a>
        = S::ReadOnly<'a>
    where
        Self: 'a;

    async fn write(&self) -> anyhow::Result<Self::Transaction<'_>> {
        self.fetcher.storage.write().await
    }

    async fn read(&self) -> anyhow::Result<Self::ReadOnly<'_>> {
        self.fetcher.storage.read().await
    }
}

#[async_trait]
impl<Types, S, P> NodeDataSource<Types> for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + 'static,
    for<'a> S::ReadOnly<'a>: NodeStorage<Types>,
    P: Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.block_height().await
    }

    async fn count_transactions_in_range(
        &self,
        _range: impl RangeBounds<usize> + Send,
    ) -> QueryResult<usize> {
        Err(QueryError::Error {
            message: "transaction count is not supported for leaf only data source".to_string(),
        })
    }

    async fn payload_size_in_range(
        &self,
        _range: impl RangeBounds<usize> + Send,
    ) -> QueryResult<usize> {
        Err(QueryError::Error {
            message: "payload size is not supported for leaf only data source".to_string(),
        })
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.vid_share(id).await
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        Err(QueryError::Error {
            message: "sync status is not available in leafonly data source".to_string(),
        })
    }

    async fn get_header_window(
        &self,
        start: impl Into<WindowStart<Types>> + Send + Sync,
        end: u64,
        limit: usize,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.get_header_window(start, end, limit).await
    }
}

#[async_trait]
impl<Types, S, P> AvailabilityDataSource<Types> for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types>,
    S: VersionedDataSource + PruneStorage + HasMetrics + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityStorage<Types>,
    for<'a> S::ReadOnly<'a>: AvailabilityStorage<Types> + NodeStorage<Types> + PrunedHeightStorage,
    P: AvailabilityProvider<Types>,
{
    async fn get_leaf<ID>(&self, id: ID) -> QueryResult<Fetch<LeafQueryData<Types>>>
    where
        ID: Into<LeafId<Types>> + Send + Sync,
    {
        Ok(self.fetcher.clone().get(id.into()).await)
    }

    async fn get_header<ID>(&self, id: ID) -> QueryResult<Fetch<Header<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let mut tx = self.read().await.map_err(|e| QueryError::Error {
            message: e.to_string(),
        })?;

        tx.get_header(id.into()).await.map(Fetch::Ready)
    }

    async fn get_block<ID>(&self, _id: ID) -> QueryResult<Fetch<BlockQueryData<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        Err(QueryError::Error {
            message: "block is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload<ID>(&self, _id: ID) -> QueryResult<Fetch<PayloadQueryData<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        Err(QueryError::Error {
            message: "payload is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload_metadata<ID>(&self, _id: ID) -> QueryResult<Fetch<PayloadMetadata<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        Err(QueryError::Error {
            message: "payload metadata is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_vid_common<ID>(&self, id: ID) -> QueryResult<Fetch<VidCommonQueryData<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let mut tx = self.read().await.map_err(|e| QueryError::Error {
            message: e.to_string(),
        })?;
        tx.get_vid_common(id.into()).await.map(Fetch::Ready)
    }

    async fn get_vid_common_metadata<ID>(
        &self,
        id: ID,
    ) -> QueryResult<Fetch<VidCommonMetadata<Types>>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let mut tx = self.read().await.map_err(|e| QueryError::Error {
            message: e.to_string(),
        })?;
        tx.get_vid_common_metadata(id.into())
            .await
            .map(Fetch::Ready)
    }

    async fn get_leaf_range<R>(&self, range: R) -> QueryResult<FetchStream<LeafQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Ok(self.fetcher.clone().get_range(range).boxed())
    }

    async fn get_header_range<R>(&self, range: R) -> QueryResult<FetchStream<Header<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        let leaves = self.get_leaf_range(range).await?;
        Ok(leaves
            .map(|fetch| fetch.map(|leaf| leaf.leaf.block_header().clone()))
            .boxed())
    }

    async fn get_block_range<R>(&self, _range: R) -> QueryResult<FetchStream<BlockQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Err(QueryError::Error {
            message: "block range is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<FetchStream<PayloadQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Err(QueryError::Error {
            message: "payload is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload_metadata_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<FetchStream<PayloadMetadata<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Err(QueryError::Error {
            message: "payload metadata is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<FetchStream<VidCommonQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Ok(self.fetcher.clone().get_range(range).boxed())
    }

    async fn get_vid_common_metadata_range<R>(
        &self,
        range: R,
    ) -> QueryResult<FetchStream<VidCommonMetadata<Types>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        Ok(self.fetcher.clone().get_range(range).boxed())
    }

    async fn get_leaf_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> QueryResult<FetchStream<LeafQueryData<Types>>> {
        Ok(self.fetcher.clone().get_range_rev(start, end))
    }

    async fn get_block_range_rev(
        &self,
        _start: Bound<usize>,
        _end: usize,
    ) -> QueryResult<FetchStream<BlockQueryData<Types>>> {
        Err(QueryError::Error {
            message: "block range is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload_range_rev(
        &self,
        _start: Bound<usize>,
        _end: usize,
    ) -> QueryResult<FetchStream<PayloadQueryData<Types>>> {
        Err(QueryError::Error {
            message: "payload range is not supported for leaf only data source".to_string(),
        })
    }

    async fn get_payload_metadata_range_rev(
        &self,
        _start: Bound<usize>,
        _end: usize,
    ) -> QueryResult<FetchStream<PayloadMetadata<Types>>> {
        Err(QueryError::Error {
            message: "payload metadata range is not supported for leaf only data source"
                .to_string(),
        })
    }

    async fn get_vid_common_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> QueryResult<FetchStream<VidCommonQueryData<Types>>> {
        Ok(self.fetcher.clone().get_range_rev(start, end))
    }

    async fn get_vid_common_metadata_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> QueryResult<FetchStream<VidCommonMetadata<Types>>> {
        Ok(self.fetcher.clone().get_range_rev(start, end))
    }

    async fn get_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<Fetch<TransactionQueryData<Types>>> {
        let mut tx = self.read().await.map_err(|e| QueryError::Error {
            message: e.to_string(),
        })?;
        tx.get_transaction(hash).await.map(Fetch::Ready)
    }

    async fn subscribe_blocks(
        &self,
        _from: usize,
    ) -> QueryResult<BoxStream<'static, BlockQueryData<Types>>> {
        Err(QueryError::Error {
            message: "block subscription is not supported for leaf only data source".to_string(),
        })
    }

    async fn subscribe_payloads(
        &self,
        _from: usize,
    ) -> QueryResult<BoxStream<'static, PayloadQueryData<Types>>> {
        Err(QueryError::Error {
            message: "payload subscription is not supported for leaf only data source".to_string(),
        })
    }

    async fn subscribe_payload_metadata(
        &self,
        _from: usize,
    ) -> QueryResult<BoxStream<'static, PayloadMetadata<Types>>> {
        Err(QueryError::Error {
            message: "payload metadata subscription is not supported for leaf only data source"
                .to_string(),
        })
    }

    async fn subscribe_leaves(
        &self,
        from: usize,
    ) -> QueryResult<BoxStream<'static, LeafQueryData<Types>>> {
        Ok(self
            .get_leaf_range(from..)
            .await?
            .then(Fetch::resolve)
            .boxed())
    }

    async fn subscribe_vid_common(
        &self,
        from: usize,
    ) -> QueryResult<BoxStream<'static, VidCommonQueryData<Types>>> {
        Ok(self
            .get_vid_common_range(from..)
            .await?
            .then(Fetch::resolve)
            .boxed())
    }

    async fn subscribe_vid_common_metadata(
        &self,
        from: usize,
    ) -> QueryResult<BoxStream<'static, VidCommonMetadata<Types>>> {
        Ok(self
            .get_vid_common_metadata_range(from..)
            .await?
            .then(Fetch::resolve)
            .boxed())
    }
}

impl<Types, S, P> UpdateAvailabilityData<Types> for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityStorage<Types>,
    for<'a> S::ReadOnly<'a>: AvailabilityStorage<Types> + NodeStorage<Types> + PrunedHeightStorage,
    P: AvailabilityProvider<Types>,
{
    async fn append(&self, mut info: BlockInfo<Types>) -> anyhow::Result<()> {
        info.block = None;

        let try_store = || async {
            let mut tx = self.fetcher.storage.write().await?;
            info.clone().store(&mut tx).await?;
            tracing::info!(?info, "appended !!!");
            tx.commit().await
        };

        let mut backoff = self.fetcher.backoff.clone();
        backoff.reset();
        loop {
            let Err(_) = try_store().await else {
                break;
            };

            let Some(delay) = backoff.next_backoff() else {
                break;
            };
            tracing::info!(?delay, "retrying failed operation");
            sleep(delay).await;
        }
        info.notify(&self.fetcher.notifiers).await;
        Ok(())
    }
}

#[async_trait]
impl<Types, S, P> StatusDataSource for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + HasMetrics + Send + Sync + 'static,
    for<'a> S::ReadOnly<'a>: NodeStorage<Types>,
    P: AvailabilityProvider<Types>,
{
    async fn block_height(&self) -> QueryResult<usize> {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.block_height().await
    }
}

#[async_trait]
impl<Types, S, P> PrunedHeightDataSource for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + HasMetrics + Send + Sync + 'static,
    for<'a> S::ReadOnly<'a>: PrunedHeightStorage,
    P: Send + Sync,
{
    async fn load_pruned_height(&self) -> anyhow::Result<Option<u64>> {
        let mut tx = self.read().await?;
        tx.load_pruned_height().await
    }
}

impl<Types, S, P> AsRef<S> for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    P: Send + Sync,
{
    fn as_ref(&self) -> &S {
        &self.fetcher.storage
    }
}

impl<Types, S, P> HasMetrics for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: HasMetrics,
    P: Send + Sync,
{
    fn metrics(&self) -> &PrometheusMetrics {
        self.as_ref().metrics()
    }
}

#[async_trait]
impl<Types, S, P, State, const ARITY: usize> MerklizedStateDataSource<Types, State, ARITY>
    for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + 'static,
    for<'a> S::ReadOnly<'a>: MerklizedStateStorage<Types, State, ARITY>,
    State: MerklizedState<Types, ARITY> + 'static,
    <State as MerkleTreeScheme>::Commitment: Send,
    P: AvailabilityProvider<Types>,
{
    async fn get_path(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>> {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.get_path(snapshot, key).await
    }
}

#[async_trait]
impl<Types, S, P> MerklizedStateHeightPersistence for LeafOnlyDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource + 'static,
    for<'a> S::ReadOnly<'a>: MerklizedStateHeightStorage,
    P: AvailabilityProvider<Types>,
{
    async fn get_last_state_height(&self) -> QueryResult<usize> {
        let mut tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.get_last_state_height().await
    }
}
