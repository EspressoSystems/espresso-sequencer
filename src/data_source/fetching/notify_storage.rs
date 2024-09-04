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
    Header, Payload, QueryResult, VidShare,
};
use anyhow::Context;
use async_trait::async_trait;
use futures::future::Future;
use hotshot_types::traits::node_implementation::NodeType;
use jf_merkle_tree::{prelude::MerkleProof, MerkleTreeScheme};
use std::ops::RangeBounds;

#[derive(Debug)]
pub(super) struct NotifyStorage<Types, S>
where
    Types: NodeType,
{
    storage: S,
    notifiers: Notifiers<Types>,
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
{
    pub(super) fn new(storage: S) -> Self {
        Self {
            storage,
            notifiers: Default::default(),
        }
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
        Ok(Transaction::new(self, self.storage.read().await?))
    }

    async fn write(&self) -> anyhow::Result<Self::Transaction<'_>> {
        Ok(Transaction::new(self, self.storage.write().await?))
    }
}

#[derive(Debug)]
pub struct Transaction<'a, Types, T>
where
    Types: NodeType,
{
    inner: T,
    notifiers: &'a Notifiers<Types>,

    // Pending notifications generated during this transaction. These notifications will be sent out
    // after the transaction is committed to storage, which guarantees that anyone who subscribes to
    // notifications and then sees that the desired object is _not_ present in storage will
    // subsequently get a notification after the object is added to storage.
    inserted_leaves: Vec<LeafQueryData<Types>>,
    inserted_blocks: Vec<BlockQueryData<Types>>,
    inserted_vid: Vec<VidCommonQueryData<Types>>,
}

impl<'a, Types, T> Transaction<'a, Types, T>
where
    Types: NodeType,
{
    fn new<S>(storage: &'a NotifyStorage<Types, S>, inner: T) -> Self {
        Self {
            inner,
            notifiers: &storage.notifiers,
            inserted_leaves: Default::default(),
            inserted_blocks: Default::default(),
            inserted_vid: Default::default(),
        }
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
        self.inner.commit().await?;

        // Now that any inserted objects have been added to storage, alert any clients who were
        // waiting on these objects.
        for leaf in self.inserted_leaves {
            self.notifiers.leaf.notify(&leaf).await;
        }
        for block in self.inserted_blocks {
            self.notifiers.block.notify(&block).await;
        }
        for vid in self.inserted_vid {
            self.notifiers.vid_common.notify(&vid).await;
        }

        Ok(())
    }

    fn revert(self) -> impl Future + Send {
        self.inner.revert()
    }
}

#[async_trait]
impl<'a, Types, T> PrunedHeightStorage for Transaction<'a, Types, T>
where
    Types: NodeType,
    T: PrunedHeightStorage + Sync,
{
    async fn load_pruned_height(&self) -> anyhow::Result<Option<u64>> {
        self.inner.load_pruned_height().await
    }
}

impl<'a, Types, T> Transaction<'a, Types, T>
where
    Types: NodeType,
    T: PrunedHeightStorage + NodeDataSource<Types> + Sync,
{
    pub(super) async fn heights(&self) -> anyhow::Result<Heights> {
        let height = self.block_height().await.context("loading block height")? as u64;
        let pruned_height = self
            .load_pruned_height()
            .await
            .context("loading pruned height")?;
        Ok(Heights {
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
        // Store the new leaf.
        self.inner.insert_leaf(leaf.clone()).await?;
        // Queue a notification about the newly received leaf.
        self.inserted_leaves.push(leaf);
        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> anyhow::Result<()> {
        // Store the new block.
        self.inner.insert_block(block.clone()).await?;
        // Queue a notification about the newly received block.
        self.inserted_blocks.push(block);
        Ok(())
    }

    async fn insert_vid(
        &mut self,
        common: VidCommonQueryData<Types>,
        share: Option<VidShare>,
    ) -> anyhow::Result<()> {
        // Store the new VID.
        self.inner.insert_vid(common.clone(), share).await?;
        // Queue a notification about the newly received VID.
        self.inserted_vid.push(common);
        Ok(())
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
