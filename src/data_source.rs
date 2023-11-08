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
        data_source::{
            AvailabilityDataSource, BlockId, LeafId, MissingSnafu, NotFoundSnafu, QueryResult,
            ResourceId, UpdateAvailabilityData,
        },
        query_data::{
            BlockHash, BlockQueryData, LeafHash, LeafQueryData, QueryableBlock, TransactionHash,
            TransactionIndex,
        },
    },
    ledger_log::LedgerLog,
    metrics::{MetricsError, PrometheusMetrics},
    status::{
        data_source::{StatusDataSource, UpdateStatusData},
        query_data::MempoolQueryData,
    },
    Block, Deltas, Resolvable,
};
use async_trait::async_trait;
use atomic_store::{AtomicStore, AtomicStoreLoader, PersistenceError};
use commit::Committable;
use futures::stream::{self, BoxStream, Stream, StreamExt, TryStreamExt};
use hotshot_types::traits::{
    metrics::Metrics,
    node_implementation::{NodeImplementation, NodeType},
    signature_key::EncodedPublicKey,
};
use serde::{de::DeserializeOwned, Serialize};
use snafu::OptionExt;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::{Bound, RangeBounds};
use std::path::Path;

pub use crate::ledger_log::Iter;
pub use crate::update::UpdateDataSource;

const CACHED_LEAVES_COUNT: usize = 100;
const CACHED_BLOCKS_COUNT: usize = 100;

/// Data used by the APIs provided in this crate, including persistent storage.
///
/// [QueryData] is designed to be both extensible (so you can add additional state to the API
/// modules defined in this crate) and composable (so you can use [QueryData] as one component of a
/// larger state type for an application with additional modules). Extending [QueryData] is possible
/// through the `UserData` type parameter -- [QueryData] implements `AsRef<UserData>` and
/// `AsMut<UserData>`, so your API extensions can always access `UserData` from [QueryData].
///
/// Composing [QueryData] with other module states is in principle simple -- just create an
/// aggregate struct containing both [QueryData] and your additional module states. A complication
/// arises from how persistent storage is managed: if other modules have their own persistent state,
/// should the storage of [QueryData] and the other modules be completely independent, or
/// synchronized under the control of a single [AtomicStore]? [QueryData] supports both patterns:
/// when you create it with [create](QueryData::create) or [open](QueryData::open), it will open its
/// own [AtomicStore] and manage the synchronization of its own storage, independent of any other
/// persistent data it might be composed with. But when you create it with
/// [create_with_store](QueryData::create_with_store) or
/// [open_with_store](QueryData::open_with_store), you may ask it to register its persistent data
/// structures with an existing [AtomicStoreLoader]. If you register other modules' persistent data
/// structures with the same loader, you can create one [AtomicStore] that synchronizes all the
/// persistent data. Note, though, that when you choose to use
/// [create_with_store](QueryData::create_with_store) or
/// [open_with_store](QueryData::open_with_store), you become responsible for ensuring that calls to
/// [AtomicStore::commit_version] alternate with calls to [QueryData::commit_version] or
/// [QueryData::skip_version].
#[derive(custom_debug::Debug)]
pub struct QueryData<Types: NodeType, I: NodeImplementation<Types>, UserData>
where
    Block<Types>: QueryableBlock,
{
    index_by_leaf_hash: HashMap<LeafHash<Types, I>, u64>,
    index_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_by_txn_hash: HashMap<TransactionHash<Types>, (u64, TransactionIndex<Types>)>,
    index_by_proposer_id: HashMap<EncodedPublicKey, Vec<u64>>,
    #[debug(skip)]
    top_storage: Option<AtomicStore>,
    leaf_storage: LedgerLog<LeafQueryData<Types, I>>,
    block_storage: LedgerLog<BlockQueryData<Types>>,
    metrics: PrometheusMetrics,
    user_data: UserData,
}

impl<Types: NodeType, I: NodeImplementation<Types>, UserData> QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
{
    /// Create a new [QueryData] with storage at `path`.
    ///
    /// If there is already data at `path`, it will be archived.
    ///
    /// The [QueryData] will manage its own persistence synchronization.
    pub fn create(path: &Path, user_data: UserData) -> Result<Self, PersistenceError> {
        let mut loader = AtomicStoreLoader::create(path, "hotshot_query_data")?;
        let mut query_data = Self::create_with_store(&mut loader, user_data)?;
        query_data.top_storage = Some(AtomicStore::open(loader)?);
        Ok(query_data)
    }

    /// Open an existing [QueryData] from storage at `path`.
    ///
    /// If there is no data at `path`, a new store will be created.
    ///
    /// The [QueryData] will manage its own persistence synchronization.
    pub fn open(path: &Path, user_data: UserData) -> Result<Self, PersistenceError>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>,
    {
        let mut loader = AtomicStoreLoader::load(path, "hotshot_query_data")?;
        let mut query_data = Self::open_with_store(&mut loader, user_data)?;
        query_data.top_storage = Some(AtomicStore::open(loader)?);
        Ok(query_data)
    }

    /// Create a new [QueryData] using a persistent storage loader.
    ///
    /// If there is existing data corresponding to the [QueryData] data structures, it will be
    /// archived.
    ///
    /// The [QueryData] will register its persistent data structures with `loader`. The caller is
    /// responsible for creating an [AtomicStore] from `loader` and managing synchronization of the
    /// store.
    pub fn create_with_store(
        loader: &mut AtomicStoreLoader,
        user_data: UserData,
    ) -> Result<Self, PersistenceError> {
        Ok(Self {
            index_by_leaf_hash: Default::default(),
            index_by_block_hash: Default::default(),
            index_by_txn_hash: Default::default(),
            index_by_proposer_id: Default::default(),
            top_storage: None,
            leaf_storage: LedgerLog::create(loader, "leaves", CACHED_LEAVES_COUNT)?,
            block_storage: LedgerLog::create(loader, "blocks", CACHED_BLOCKS_COUNT)?,
            metrics: Default::default(),
            user_data,
        })
    }

    /// Open an existing [QueryData] using a persistent storage loader.
    ///
    /// If there is no existing data corresponding to the [QueryData] data structures, a new store
    /// will be created.
    ///
    /// The [QueryData] will register its persistent data structures with `loader`. The caller is
    /// responsible for creating an [AtomicStore] from `loader` and managing synchronization of the
    /// store.
    pub fn open_with_store(
        loader: &mut AtomicStoreLoader,
        user_data: UserData,
    ) -> Result<Self, PersistenceError>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>,
    {
        let leaf_storage =
            LedgerLog::<LeafQueryData<Types, I>>::open(loader, "leaves", CACHED_LEAVES_COUNT)?;
        let block_storage =
            LedgerLog::<BlockQueryData<Types>>::open(loader, "blocks", CACHED_BLOCKS_COUNT)?;

        let mut index_by_proposer_id = HashMap::new();
        let mut index_by_block_hash = HashMap::new();
        let index_by_leaf_hash = leaf_storage
            .iter()
            .flatten()
            .map(|leaf| {
                index_by_proposer_id
                    .entry(leaf.proposer())
                    .or_insert_with(Vec::new)
                    .push(leaf.height());
                update_index_by_hash(&mut index_by_block_hash, leaf.block_hash(), leaf.height());
                (leaf.hash(), leaf.height())
            })
            .collect();

        let mut index_by_txn_hash = HashMap::new();
        for block in block_storage.iter().flatten() {
            let height = block.height();
            for (txn_ix, txn) in block.block().enumerate() {
                update_index_by_hash(&mut index_by_txn_hash, txn.commit(), (height, txn_ix));
            }
        }

        Ok(QueryData {
            index_by_leaf_hash,
            index_by_block_hash,
            index_by_txn_hash,
            index_by_proposer_id,
            leaf_storage,
            block_storage,
            top_storage: None,
            metrics: Default::default(),
            user_data,
        })
    }

    /// Commit the current state to persistent storage.
    ///
    /// If the [QueryData] is managing its own [AtomicStore] (i.e. it was created with
    /// [create](Self::create) or [open](Self::open)) it will update the global version as well.
    /// Otherwise, the caller is responsible for calling [AtomicStore::commit_version] after calling
    /// this function.
    pub async fn commit_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.commit_version().await?;
        self.block_storage.commit_version().await?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    /// Advance the version of the persistent store without committing changes to persistent state.
    ///
    /// This function is useful when the [AtomicStore] synchronizing storage for this [QueryData] is
    /// being managed by the caller. The caller may want to persist some changes to other modules
    /// whose state is managed by the same [AtomicStore]. In order to call
    /// [AtomicStore::commit_version], the version of this [QueryData] must be advanced, either by
    /// [commit_version](Self::commit_version) or, if there are no outstanding changes,
    /// [skip_version](Self::skip_version).
    pub fn skip_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.skip_version()?;
        self.block_storage.skip_version()?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    /// Revert changes made to persistent storage since the last call to [commit_version](Self::commit_version).
    pub fn revert_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.revert_version()?;
        self.block_storage.revert_version()?;
        Ok(())
    }
}

impl<Types: NodeType, I: NodeImplementation<Types>, UserData> AsRef<UserData>
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
{
    fn as_ref(&self) -> &UserData {
        &self.user_data
    }
}

impl<Types: NodeType, I: NodeImplementation<Types>, UserData> AsMut<UserData>
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
{
    fn as_mut(&mut self) -> &mut UserData {
        &mut self.user_data
    }
}

async fn range_stream<T>(
    mut iter: Iter<'_, T>,
    range: impl RangeBounds<usize>,
) -> impl '_ + Stream<Item = QueryResult<T>>
where
    T: Clone + Serialize + DeserializeOwned + Sync,
{
    let start = range.start_bound().cloned();
    let end = range.end_bound().cloned();

    // Advance the underlying iterator to the start of the range.
    let pos = match start {
        Bound::Included(n) => {
            if n > 0 {
                iter.nth(n - 1);
            }
            n
        }
        Bound::Excluded(n) => {
            iter.nth(n);
            n + 1
        }
        Bound::Unbounded => 0,
    };

    stream::unfold((iter, end, pos), |(mut iter, end, pos)| async move {
        // Check if we have reached the end of the range.
        let reached_end = match end {
            Bound::Included(n) => pos > n,
            Bound::Excluded(n) => pos >= n,
            Bound::Unbounded => false,
        };
        if reached_end {
            return None;
        }
        let opt = iter.next()?;
        Some((opt.context(MissingSnafu), (iter, end, pos + 1)))
    })
}

#[async_trait]
impl<Types: NodeType, I: NodeImplementation<Types>, UserData> AvailabilityDataSource<Types, I>
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
    UserData: Send + Sync,
{
    type LeafStream = BoxStream<'static, LeafQueryData<Types, I>>;
    type BlockStream = BoxStream<'static, BlockQueryData<Types>>;

    type LeafRange<'a, R> = BoxStream<'a, QueryResult<LeafQueryData<Types, I>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;
    type BlockRange<'a, R> = BoxStream<'a, QueryResult<BlockQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;

    async fn get_leaf(&self, id: LeafId<Types, I>) -> QueryResult<LeafQueryData<Types, I>> {
        let n = match id {
            ResourceId::Number(n) => n,
            ResourceId::Hash(h) => {
                *self.index_by_leaf_hash.get(&h).context(NotFoundSnafu)? as usize
            }
        };
        self.leaf_storage
            .iter()
            .nth(n)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        let n = match id {
            ResourceId::Number(n) => n,
            ResourceId::Hash(h) => {
                *self.index_by_block_hash.get(&h).context(NotFoundSnafu)? as usize
            }
        };
        self.block_storage
            .iter()
            .nth(n)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)
    }

    async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_stream(self.leaf_storage.iter(), range).await.boxed())
    }

    async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_stream(self.block_storage.iter(), range).await.boxed())
    }

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        let (height, ix) = self.index_by_txn_hash.get(&hash).context(NotFoundSnafu)?;
        let block = self.get_block(ResourceId::Number(*height as usize)).await?;
        Ok((block, ix.clone()))
    }

    async fn get_proposals(
        &self,
        id: &EncodedPublicKey,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types, I>>> {
        let all_ids = self
            .index_by_proposer_id
            .get(id)
            .cloned()
            .unwrap_or_default();
        let start_from = match limit {
            Some(count) => all_ids.len().saturating_sub(count),
            None => 0,
        };
        stream::iter(all_ids)
            .skip(start_from)
            .then(|height| self.get_leaf(ResourceId::Number(height as usize)))
            .try_collect()
            .await
    }

    async fn count_proposals(&self, id: &EncodedPublicKey) -> QueryResult<usize> {
        Ok(match self.index_by_proposer_id.get(id) {
            Some(ids) => ids.len(),
            None => 0,
        })
    }

    async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream> {
        self.leaf_storage.subscribe(height).context(MissingSnafu)
    }

    async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> {
        self.block_storage.subscribe(height).context(MissingSnafu)
    }
}

#[async_trait]
impl<Types: NodeType, I: NodeImplementation<Types>, UserData> UpdateAvailabilityData<Types, I>
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
    UserData: Send,
{
    type Error = PersistenceError;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types, I>) -> Result<(), Self::Error>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>,
    {
        self.leaf_storage
            .insert(leaf.height() as usize, leaf.clone())?;
        self.index_by_leaf_hash.insert(leaf.hash(), leaf.height());
        update_index_by_hash(
            &mut self.index_by_block_hash,
            leaf.block_hash(),
            leaf.height(),
        );
        self.index_by_proposer_id
            .entry(leaf.proposer())
            .or_default()
            .push(leaf.height());
        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        self.block_storage
            .insert(block.height() as usize, block.clone())?;
        for (txn_ix, txn) in block.block().enumerate() {
            update_index_by_hash(
                &mut self.index_by_txn_hash,
                txn.commit(),
                (block.height(), txn_ix),
            );
        }
        Ok(())
    }
}

/// Update an index mapping hashes of objects to their positions in the ledger.
///
/// This function will insert the mapping from `hash` to `pos` into `index`, _unless_ there is
/// already an entry for `hash` at an earlier position in the ledger.
fn update_index_by_hash<H: Eq + Hash, P: Ord>(index: &mut HashMap<H, P>, hash: H, pos: P) {
    match index.entry(hash) {
        Entry::Occupied(mut e) => {
            if &pos < e.get() {
                // Overwrite the existing entry if the new object was sequenced first.
                e.insert(pos);
            }
        }
        Entry::Vacant(e) => {
            e.insert(pos);
        }
    }
}

/// Metric-related functions.
impl<Types: NodeType, I: NodeImplementation<Types>, UserData> QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
{
    fn consensus_metrics(&self) -> Result<PrometheusMetrics, MetricsError> {
        self.metrics.get_subgroup(["consensus"])
    }
}

#[async_trait]
impl<Types: NodeType, I: NodeImplementation<Types>, UserData> StatusDataSource
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
    UserData: Sync,
{
    type Error = MetricsError;

    async fn block_height(&self) -> Result<usize, Self::Error> {
        Ok(self.leaf_storage.iter().len())
    }

    async fn mempool_info(&self) -> Result<MempoolQueryData, Self::Error> {
        Ok(MempoolQueryData {
            transaction_count: self
                .consensus_metrics()?
                .get_gauge("outstanding_transactions")?
                .get() as u64,
            memory_footprint: self
                .consensus_metrics()?
                .get_gauge("outstanding_transactions_memory_size")?
                .get() as u64,
        })
    }

    async fn success_rate(&self) -> Result<f64, Self::Error> {
        let total_views = self.consensus_metrics()?.get_gauge("current_view")?.get() as f64;
        // By definition, a successful view is any which committed a block.
        Ok(self.block_height().await? as f64 / total_views)
    }

    async fn export_metrics(&self) -> Result<String, Self::Error> {
        self.metrics.prometheus()
    }
}

impl<Types: NodeType, I: NodeImplementation<Types>, UserData> UpdateStatusData
    for QueryData<Types, I, UserData>
where
    Block<Types>: QueryableBlock,
{
    fn metrics(&self) -> Box<dyn Metrics> {
        Box::new(self.metrics.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::{
        consensus::MockNetwork,
        mocks::{MockNodeImpl, MockTransaction, MockTypes},
        setup_test, sleep,
    };
    use async_std::sync::RwLock;
    use bincode::Options;
    use futures::StreamExt;
    use hotshot_utils::bincode::bincode_opts;
    use std::collections::HashSet;
    use std::time::Duration;

    async fn get_non_empty_blocks<UserData: Send + Sync>(
        qd: &RwLock<QueryData<MockTypes, MockNodeImpl, UserData>>,
    ) -> Vec<(
        LeafQueryData<MockTypes, MockNodeImpl>,
        BlockQueryData<MockTypes>,
    )> {
        let qd = qd.read().await;
        qd.get_leaf_range(..)
            .await
            .unwrap()
            .zip(qd.get_block_range(..).await.unwrap())
            .filter_map(|entry| async move {
                match entry {
                    (Ok(leaf), Ok(block)) if !block.is_empty() => Some((leaf, block)),
                    _ => None,
                }
            })
            .collect()
            .await
    }

    async fn validate<UserData: Send + Sync>(
        qd: &RwLock<QueryData<MockTypes, MockNodeImpl, UserData>>,
    ) {
        let qd = qd.read().await;

        // Check the consistency of every block/leaf pair. Keep track of blocks and transactions
        // we've seen so we can detect duplicates.
        let mut seen_blocks = HashMap::new();
        let mut seen_transactions = HashMap::new();
        let mut leaves = qd.get_leaf_range(..).await.unwrap().enumerate();
        while let Some((i, leaf)) = leaves.next().await {
            let leaf = leaf.unwrap();
            assert_eq!(leaf.height(), i as u64);
            assert_eq!(leaf.hash(), leaf.leaf().commit());

            // Check indices.
            assert_eq!(leaf, qd.get_leaf(ResourceId::Number(i)).await.unwrap());
            assert_eq!(
                leaf,
                qd.get_leaf(ResourceId::Hash(leaf.hash())).await.unwrap()
            );
            assert!(qd
                .get_proposals(&leaf.proposer(), None)
                .await
                .unwrap()
                .contains(&leaf));

            let Ok(block) = qd.get_block(ResourceId::Number(i)).await else {
                continue;
            };
            assert_eq!(leaf.block_hash(), block.hash());
            assert_eq!(block.height(), i as u64);
            assert_eq!(block.hash(), block.block().commit());
            assert_eq!(
                block.size(),
                bincode_opts().serialized_size(block.block()).unwrap()
            );

            // Check indices.
            assert_eq!(block, qd.get_block(ResourceId::Number(i)).await.unwrap());
            // We should be able to look up the block by hash unless it is a duplicate. For
            // duplicate blocks, this function returns the index of the first duplicate.
            let ix = seen_blocks.entry(block.hash()).or_insert(i as u64);
            assert_eq!(
                qd.get_block(ResourceId::Hash(block.hash()))
                    .await
                    .unwrap()
                    .height(),
                *ix
            );

            for (j, txn) in block.block().iter().enumerate() {
                // We should be able to look up the transaction by hash unless it is a duplicate.
                // For duplicate transactions, this function returns the index of the first
                // duplicate.
                let ix = seen_transactions
                    .entry(txn.commit())
                    .or_insert((i as u64, j));
                let (block, pos) = qd.get_block_with_transaction(txn.commit()).await.unwrap();
                assert_eq!((block.height(), pos), *ix);
            }
        }

        // Check that the proposer ID of every leaf indexed by a given proposer ID is that proposer
        // ID.
        for proposer in qd
            .get_leaf_range(..)
            .await
            .unwrap()
            .filter_map(|res| async move { res.ok().map(|leaf| leaf.proposer()) })
            .collect::<HashSet<_>>()
            .await
        {
            for leaf in qd.get_proposals(&proposer, None).await.unwrap() {
                assert_eq!(proposer, leaf.proposer());
            }
        }
    }

    #[async_std::test]
    async fn test_update() {
        setup_test();

        let mut network = MockNetwork::init(()).await;
        let qd = network.query_data();

        network.start().await;
        assert_eq!(get_non_empty_blocks(&qd).await, vec![]);

        // Submit a few blocks and make sure each one gets reflected in the query service and
        // preserves the consistency of the data and indices.
        let mut blocks = {
            qd.read()
                .await
                .subscribe_blocks(0)
                .await
                .unwrap()
                .enumerate()
        };
        for nonce in 0..3 {
            let txn = MockTransaction { nonce };
            network.submit_transaction(txn).await;

            // Wait for the transaction to be finalized.
            let (i, block) = loop {
                tracing::info!("waiting for block {nonce}");
                let (i, block) = blocks.next().await.unwrap();
                if !block.is_empty() {
                    break (i, block);
                }
                tracing::info!("block {i} is empty");
            };

            assert_eq!(
                qd.read()
                    .await
                    .get_block(ResourceId::Number(i))
                    .await
                    .unwrap(),
                block
            );
            validate(&qd).await;
        }

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_metrics() {
        setup_test();

        let mut network = MockNetwork::init(()).await;
        let qd = network.query_data();

        {
            // With consensus paused, check that the success rate returns NaN (since the block
            // height, the numerator, and view number, the denominator, are both 0).
            assert!(qd.read().await.success_rate().await.unwrap().is_nan());
            // Check that block height is initially zero.
            assert_eq!(qd.read().await.block_height().await.unwrap(), 0);
        }

        // Submit a transaction, and check that it is reflected in the mempool.
        let txn = MockTransaction { nonce: 0 };
        network.submit_transaction(txn.clone()).await;
        loop {
            let mempool = { qd.read().await.mempool_info().await.unwrap() };
            let expected = MempoolQueryData {
                transaction_count: 1,
                memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
            };
            if mempool == expected {
                break;
            }
            tracing::info!(
                "waiting for mempool to reflect transaction (currently {:?})",
                mempool
            );
            sleep(Duration::from_secs(1)).await;
        }
        {
            assert_eq!(
                qd.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 1,
                    memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
                }
            );
        }

        // Submitting the same transaction should not affect the mempool.
        network.submit_transaction(txn.clone()).await;
        sleep(Duration::from_secs(3)).await;
        {
            assert_eq!(
                qd.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 1,
                    memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
                }
            );
        }

        // Start consensus and wait for the transaction to be finalized.
        let mut blocks = { qd.read().await.subscribe_blocks(0).await.unwrap() };
        network.start().await;
        loop {
            if !blocks.next().await.unwrap().is_empty() {
                break;
            }
        }

        {
            // Check that block height and success rate have been updated. Note that we can only
            // check if success rate is positive. We don't know exactly what it is because we can't
            // know how many views have elapsed without race conditions. Similarly, we can only
            // check that block height is at least 2, because we know that the genesis block and our
            // transaction's block have both been committed, but we can't know how many empty blocks
            // were committed.
            assert!(qd.read().await.success_rate().await.unwrap() > 0.0);
            assert!(qd.read().await.block_height().await.unwrap() >= 2);
        }

        {
            // Check that the transaction is no longer reflected in the mempool.
            assert_eq!(
                qd.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 0,
                    memory_footprint: 0,
                }
            );
        }
    }
}
