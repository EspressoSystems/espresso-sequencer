// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use super::ResultExt;
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionQueryData, UpdateAvailabilityData,
        VidCommonQueryData,
    },
    data_source::{
        notifier::Notifier,
        storage::{
            pruning::{PruneStorage, PrunedHeightStorage, PrunerCfg},
            AvailabilityStorage, ExplorerStorage,
        },
        update::{self, VersionedDataSource},
    },
    explorer,
    merklized_state::{
        MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence, Snapshot,
        UpdateStateData,
    },
    node::{NodeDataSource, SyncStatus, TimeWindowQueryData, WindowStart},
    types::HeightIndexed,
    Header, Payload, QueryResult, VidShare,
};
use anyhow::Context;
use async_std::sync::RwLock;
use async_trait::async_trait;
use futures::future::Future;
use hotshot_types::traits::node_implementation::NodeType;
use jf_merkle_tree::{prelude::MerkleProof, MerkleTreeScheme};
use std::{cmp::max, ops::RangeBounds};

#[derive(Debug)]
pub(super) struct NotifyStorage<Types, S>
where
    Types: NodeType,
{
    storage: S,
    notifiers: Notifiers<Types>,

    /// In-memory cache for commonly read fields.
    ///
    /// This cache keeps fields which are read frequently and which can be kept up-to-date by dead
    /// reckoning on each update. Thus, we avoid ever reading these fields from the database after
    /// startup, and we don't have to write back when the cache is modified either.
    ///
    /// Currently the cached fields include latest height and pruned height.
    ///
    /// The concurrency semantics are inherited from the underlying database. That is, any read from
    /// the cache is *always* guaranteed to be consistent with the results we would have gotten had
    /// we read from the database at that instant, and thus it is as if there is no cache at all --
    /// except performance. This is accomplished by exclusively locking the cache during any
    /// critical section where it might be out of sync with the database, for example just before
    /// committing a database transaction, until the cache is updated with the results of that
    /// transaction. While the cache is locked (hopefully only briefly) readers can bypass it and
    /// read directly from the database, getting consistent data to the extent that the databse
    /// itself maintains consistency. Thus, a reader should never block on cache updates.
    cache: RwLock<Heights>,
}

impl<Types, S> AsRef<S> for NotifyStorage<Types, S>
where
    Types: NodeType,
{
    fn as_ref(&self) -> &S {
        &self.storage
    }
}

impl<Types, S> NotifyStorage<Types, S>
where
    Types: NodeType,
    S: VersionedDataSource + Sync,
    for<'a> S::ReadOnly<'a>: NodeDataSource<Types> + PrunedHeightStorage,
{
    pub(super) async fn new(storage: S) -> anyhow::Result<Self> {
        let tx = storage.read().await?;
        let height = tx.block_height().await? as u64;
        let pruned_height = tx.load_pruned_height().await?;
        drop(tx);

        Ok(Self {
            cache: RwLock::new(Heights {
                height,
                pruned_height,
            }),
            storage,
            notifiers: Default::default(),
        })
    }
}

impl<Types, S> NotifyStorage<Types, S>
where
    Types: NodeType,
    S: PruneStorage + Sync,
{
    pub(super) async fn prune(&self) {
        // We loop until the whole run pruner run is complete
        let mut pruner = S::Pruner::default();
        loop {
            match self.storage.prune(&mut pruner).await {
                Ok(Some(height)) => {
                    tracing::warn!("Pruned to height {height}");
                    self.cache.write().await.pruned_height = Some(height);
                }
                Ok(None) => {
                    tracing::warn!("pruner run complete.");
                    break;
                }
                Err(e) => {
                    tracing::error!("pruner run failed: {e:?}");
                    break;
                }
            }
        }
    }
}

impl<Types, S> NotifyStorage<Types, S>
where
    Types: NodeType,
{
    pub(super) fn notifiers(&self) -> &Notifiers<Types> {
        &self.notifiers
    }
}

impl<Types, S> NotifyStorage<Types, S>
where
    Types: NodeType,
    S: PruneStorage + Sync,
{
    pub(super) fn get_pruning_config(&self) -> Option<PrunerCfg> {
        self.storage.get_pruning_config()
    }
}

impl<Types, S> VersionedDataSource for NotifyStorage<Types, S>
where
    Types: NodeType,
    S: VersionedDataSource,
{
    type ReadOnly<'a> = Transaction<'a, Types, S::ReadOnly<'a>>
    where
        Self: 'a;

    type Transaction<'a> = Transaction<'a, Types, S::Transaction<'a>>
    where
        Self: 'a;

    async fn read(&self) -> anyhow::Result<Self::ReadOnly<'_>> {
        Transaction::new(self, S::read).await
    }

    async fn write(&self) -> anyhow::Result<Self::Transaction<'_>> {
        Transaction::new(self, S::write).await
    }
}

#[derive(Debug)]
pub struct Transaction<'a, Types, T>
where
    Types: NodeType,
{
    inner: T,
    cache: &'a RwLock<Heights>,
    notifiers: &'a Notifiers<Types>,

    // Cached values up-to-date with this transaction thus far.
    //
    // These values are initialized from the in-memory cache at the time when the transaction
    // starts. Thereafter, they are updated by mutations made within this transaction, but will not
    // reflect mutations made to the cache by other transactions until this transaction is
    // committed, thus preserving transaction isolation.
    //
    // When this transaction is committed, these will be applied (atomically) to the in-memory
    // cache, in a manner that is consistent with other transactions that may have committed in the
    // meantime. Essentially, since both of these heights grow monotonically, whichever height is
    // greater between the updated one and the one in the committed cache will be written to the
    // cache when this transaction is committed.
    //
    // Note that either of these values may be `None`, indicating an unknown value, if the cache was
    // locked when this transaction was started and the value has not been written since. This
    // preserves the property that read operations never block on the availability of the cache, and
    // thus never block on concurrent writers.
    height: Option<u64>,
    pruned_height: Option<Option<u64>>,
}

impl<'a, Types, T> Transaction<'a, Types, T>
where
    Types: NodeType,
{
    async fn new<S, Fut>(
        storage: &'a NotifyStorage<Types, S>,
        begin: impl FnOnce(&'a S) -> Fut,
    ) -> anyhow::Result<Self>
    where
        Fut: Future<Output = anyhow::Result<T>>,
        T: 'a,
    {
        // Snapshot cached values if possible. If we are able to acquire a lock on the cache, we
        // hold it while we initialize the transaction, so that the snapshot we get is consistent
        // with the snapshot our transaction has of the underlying database.
        let (cache_lock, height, pruned_height) = if let Some(cache_lock) = storage.cache.try_read()
        {
            let height = Some(cache_lock.height);
            let pruned_height = Some(cache_lock.pruned_height);
            (Some(cache_lock), height, pruned_height)
        } else {
            (None, None, None)
        };
        let inner = begin(&storage.storage).await?;
        drop(cache_lock);

        Ok(Self {
            inner,
            cache: &storage.cache,
            notifiers: &storage.notifiers,
            height,
            pruned_height,
        })
    }
}

impl<'a, Types, T> AsRef<T> for Transaction<'a, Types, T>
where
    Types: NodeType,
{
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<'a, Types, T> AsMut<T> for Transaction<'a, Types, T>
where
    Types: NodeType,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<'a, Types, T> update::Transaction for Transaction<'a, Types, T>
where
    Types: NodeType,
    T: update::Transaction,
{
    async fn commit(self) -> anyhow::Result<()> {
        // Lock the in-memory cache. Even though we will not write to the cache until the storage
        // update completes successfully, we must hold an exclusive lock on the cache throughout,
        // because for a brief moment between committing the storage transaction and updating the
        // cache, the cache may be out of sync with storage.
        let mut cache = self.cache.write().await;

        // Commit to storage.
        self.inner.commit().await?;

        // Update cache.
        if let Some(updated_height) = self.height {
            cache.height = max(cache.height, updated_height);
        }
        if let Some(updated_pruned_height) = self.pruned_height {
            cache.pruned_height = max(cache.pruned_height, updated_pruned_height);
        }

        Ok(())
    }

    fn revert(self) -> impl Future + Send {
        self.inner.revert()
    }
}

impl<'a, Types, T> Transaction<'a, Types, T>
where
    Types: NodeType,
    T: PrunedHeightStorage + Sync,
{
    pub(super) async fn pruned_height(&self) -> anyhow::Result<Option<u64>> {
        // Read from cache if available.
        if let Some(h) = self.pruned_height {
            return Ok(h);
        }

        // Try reading from storage if the cache is locked.
        self.inner.load_pruned_height().await
    }
}

impl<'a, Types, T> Transaction<'a, Types, T>
where
    Types: NodeType,
    T: PrunedHeightStorage + NodeDataSource<Types> + Sync,
{
    pub(super) async fn heights(&self) -> Option<Heights> {
        let height = self
            .block_height()
            .await
            .context("unable to load block height")
            .ok_or_trace()? as u64;
        let pruned_height = self
            .pruned_height()
            .await
            .context("unable to load pruned height")
            .ok_or_trace()?;
        Some(Heights {
            height,
            pruned_height,
        })
    }
}

#[async_trait]
impl<'a, Types, T, State, const ARITY: usize> MerklizedStateDataSource<Types, State, ARITY>
    for Transaction<'a, Types, T>
where
    Types: NodeType,
    T: MerklizedStateDataSource<Types, State, ARITY> + Send + Sync,
    State: MerklizedState<Types, ARITY> + 'static,
    <State as MerkleTreeScheme>::Commitment: Send,
{
    async fn get_path(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>> {
        self.as_ref().get_path(snapshot, key).await
    }
}

#[async_trait]
impl<'a, Types, T> MerklizedStateHeightPersistence for Transaction<'a, Types, T>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    T: MerklizedStateHeightPersistence + Send + Sync,
{
    async fn get_last_state_height(&self) -> QueryResult<usize> {
        self.as_ref().get_last_state_height().await
    }
}

#[async_trait]
impl<'a, Types, T, State, const ARITY: usize> UpdateStateData<Types, State, ARITY>
    for Transaction<'a, Types, T>
where
    Types: NodeType,
    State: MerklizedState<Types, ARITY>,
    T: UpdateStateData<Types, State, ARITY> + Send + Sync + 'static,
{
    async fn set_last_state_height(&mut self, height: usize) -> anyhow::Result<()> {
        self.inner.set_last_state_height(height).await
    }

    async fn insert_merkle_nodes(
        &mut self,
        path: MerkleProof<State::Entry, State::Key, State::T, ARITY>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> anyhow::Result<()> {
        self.inner
            .insert_merkle_nodes(path, traversal_path, block_number)
            .await
    }
}

#[async_trait]
impl<'a, Types, T> UpdateAvailabilityData<Types> for Transaction<'a, Types, T>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    T: UpdateAvailabilityData<Types> + Send + Sync,
{
    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> anyhow::Result<()> {
        // Send a notification about the newly received leaf.
        self.notifiers.leaf.notify(&leaf).await;

        // Record the latest height we have received in this transaction, so we can insert it in the
        // in-memory cache on commit. The known height of the chain is one more than the height of
        // its heighest object.
        self.height = max(self.height, Some(leaf.height() + 1));

        // Store the new leaf.
        self.inner.insert_leaf(leaf).await
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> anyhow::Result<()> {
        // Send a notification about the newly received block.
        self.notifiers.block.notify(&block).await;

        // Record the latest height we have received in this transaction, so we can insert it in the
        // in-memory cache on commit. The known height of the chain is one more than the height of
        // its heighest object.
        self.height = max(self.height, Some(block.height() + 1));

        // Store the new block.
        self.inner.insert_block(block).await
    }

    async fn insert_vid(
        &mut self,
        common: VidCommonQueryData<Types>,
        share: Option<VidShare>,
    ) -> anyhow::Result<()> {
        // Send a notification about the newly received data.
        self.notifiers.vid_common.notify(&common).await;

        // Record the latest height we have received in this transaction, so we can insert it in the
        // in-memory cache on commit. The known height of the chain is one more than the height of
        // its heighest object.
        self.height = max(self.height, Some(common.height() + 1));

        // Store the new data.
        self.inner.insert_vid(common, share).await
    }
}

/// [`Transaction`] implements [`AvailabilityStorage`], not the richer [`AvailabilityDataSource`].
///
/// Privding the full [`AvailabilityDataSource`] interface through a transaction would be ill
/// advised, because read operations through this interface trigger side effects (fetches) that may
/// not be rolled back if the transaction is rolled back, and may also compete for resources being
/// used by the transaction itself. Thus, we only provide [`AvailabilityStorage`], which returns
/// errors if data is not available instead of fetching.
#[async_trait]
impl<'a, Types, T> AvailabilityStorage<Types> for Transaction<'a, Types, T>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    T: AvailabilityStorage<Types>,
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        self.inner.get_leaf(id).await
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        self.inner.get_block(id).await
    }

    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        self.inner.get_header(id).await
    }

    async fn get_payload(&self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        self.inner.get_payload(id).await
    }

    async fn get_vid_common(&self, id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>> {
        self.inner.get_vid_common(id).await
    }

    async fn get_leaf_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.inner.get_leaf_range(range).await
    }

    async fn get_block_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.inner.get_block_range(range).await
    }

    async fn get_payload_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.inner.get_payload_range(range).await
    }

    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.inner.get_vid_common_range(range).await
    }

    async fn get_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<TransactionQueryData<Types>> {
        self.inner.get_transaction(hash).await
    }
}

#[async_trait]
impl<'a, Types, T> NodeDataSource<Types> for Transaction<'a, Types, T>
where
    Types: NodeType,
    T: NodeDataSource<Types> + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        // Read from cache if available.
        if let Some(h) = self.height {
            return Ok(h as usize);
        }

        // Try reading from storage if the cache is locked.
        self.inner.block_height().await
    }

    async fn count_transactions(&self) -> QueryResult<usize> {
        self.inner.count_transactions().await
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        self.inner.payload_size().await
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        self.inner.vid_share(id).await
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        self.inner.sync_status().await
    }

    async fn get_header_window(
        &self,
        start: impl Into<WindowStart<Types>> + Send + Sync,
        end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        self.inner.get_header_window(start, end).await
    }
}

#[async_trait]
impl<'a, Types, T> ExplorerStorage<Types> for Transaction<'a, Types, T>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types> + explorer::traits::ExplorerHeader<Types>,
    crate::Transaction<Types>: explorer::traits::ExplorerTransaction,
    T: ExplorerStorage<Types> + Send + Sync,
{
    async fn get_block_summaries(
        &self,
        request: explorer::query_data::GetBlockSummariesRequest<Types>,
    ) -> Result<
        Vec<explorer::query_data::BlockSummary<Types>>,
        explorer::query_data::GetBlockSummariesError,
    > {
        self.as_ref().get_block_summaries(request).await
    }

    async fn get_block_detail(
        &self,
        request: explorer::query_data::BlockIdentifier<Types>,
    ) -> Result<explorer::query_data::BlockDetail<Types>, explorer::query_data::GetBlockDetailError>
    {
        self.as_ref().get_block_detail(request).await
    }

    async fn get_transaction_summaries(
        &self,
        request: explorer::query_data::GetTransactionSummariesRequest<Types>,
    ) -> Result<
        Vec<explorer::query_data::TransactionSummary<Types>>,
        explorer::query_data::GetTransactionSummariesError,
    > {
        self.as_ref().get_transaction_summaries(request).await
    }

    async fn get_transaction_detail(
        &self,
        request: explorer::query_data::TransactionIdentifier<Types>,
    ) -> Result<
        explorer::query_data::TransactionDetailResponse<Types>,
        explorer::query_data::GetTransactionDetailError,
    > {
        self.as_ref().get_transaction_detail(request).await
    }

    async fn get_explorer_summary(
        &self,
    ) -> Result<
        explorer::query_data::ExplorerSummary<Types>,
        explorer::query_data::GetExplorerSummaryError,
    > {
        self.as_ref().get_explorer_summary().await
    }

    async fn get_search_results(
        &self,
        query: String,
    ) -> Result<
        explorer::query_data::SearchResult<Types>,
        explorer::query_data::GetSearchResultsError,
    > {
        self.as_ref().get_search_results(query).await
    }
}

#[derive(Debug)]
pub(super) struct Notifiers<Types>
where
    Types: NodeType,
{
    pub(super) block: Notifier<BlockQueryData<Types>>,
    pub(super) leaf: Notifier<LeafQueryData<Types>>,
    pub(super) vid_common: Notifier<VidCommonQueryData<Types>>,
}

impl<Types> Default for Notifiers<Types>
where
    Types: NodeType,
{
    fn default() -> Self {
        Self {
            block: Notifier::new(),
            leaf: Notifier::new(),
            vid_common: Notifier::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Heights {
    pub(super) height: u64,
    pub(super) pruned_height: Option<u64>,
}

impl Heights {
    pub(super) fn might_exist(self, h: u64) -> bool {
        h < self.height && self.pruned_height.map_or(true, |ph| h > ph)
    }
}
