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

#![cfg(feature = "file-system-data-source")]

use super::{
    ledger_log::{Iter, LedgerLog},
    AvailabilityStorage,
};
use crate::{
    availability::{
        data_source::{BlockId, LeafId, ResourceId, UpdateAvailabilityData},
        query_data::{
            BlockHash, BlockQueryData, LeafHash, LeafQueryData, QueryablePayload, TransactionHash,
            TransactionIndex,
        },
    },
    data_source::VersionedDataSource,
    node::{NodeDataSource, UpdateNodeData},
    MissingSnafu, NotFoundSnafu, Payload, QueryResult, SignatureKey,
};
use async_trait::async_trait;
use atomic_store::{AtomicStore, AtomicStoreLoader, PersistenceError};
use commit::Committable;
use futures::stream::{self, StreamExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use serde::{de::DeserializeOwned, Serialize};
use snafu::OptionExt;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::{Bound, RangeBounds};
use std::path::Path;

const CACHED_LEAVES_COUNT: usize = 100;
const CACHED_BLOCKS_COUNT: usize = 100;

/// Storage for the APIs provided in this crate, backed by a remote PostgreSQL database.
#[derive(custom_debug::Debug)]
pub struct FileSystemStorage<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    index_by_leaf_hash: HashMap<LeafHash<Types>, u64>,
    index_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_by_txn_hash: HashMap<TransactionHash<Types>, (u64, TransactionIndex<Types>)>,
    index_by_proposer_id: HashMap<SignatureKey<Types>, Vec<u64>>,
    #[debug(skip)]
    top_storage: Option<AtomicStore>,
    leaf_storage: LedgerLog<LeafQueryData<Types>>,
    block_storage: LedgerLog<BlockQueryData<Types>>,
}

impl<Types: NodeType> FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    /// Create a new [FileSystemStorage] with storage at `path`.
    ///
    /// If there is already data at `path`, it will be archived.
    ///
    /// The [FileSystemStorage] will manage its own persistence synchronization.
    pub async fn create(path: &Path) -> Result<Self, PersistenceError> {
        let mut loader = AtomicStoreLoader::create(path, "hotshot_data_source")?;
        let mut data_source = Self::create_with_store(&mut loader).await?;
        data_source.top_storage = Some(AtomicStore::open(loader)?);
        Ok(data_source)
    }

    /// Open an existing [FileSystemStorage] from storage at `path`.
    ///
    /// If there is no data at `path`, a new store will be created.
    ///
    /// The [FileSystemStorage] will manage its own persistence synchronization.
    pub async fn open(path: &Path) -> Result<Self, PersistenceError> {
        let mut loader = AtomicStoreLoader::load(path, "hotshot_data_source")?;
        let mut data_source = Self::open_with_store(&mut loader).await?;
        data_source.top_storage = Some(AtomicStore::open(loader)?);
        Ok(data_source)
    }

    /// Create a new [FileSystemStorage] using a persistent storage loader.
    ///
    /// If there is existing data corresponding to the [FileSystemStorage] data structures, it will
    /// be archived.
    ///
    /// The [FileSystemStorage] will register its persistent data structures with `loader`. The
    /// caller is responsible for creating an [AtomicStore] from `loader` and managing
    /// synchronization of the store.
    pub async fn create_with_store(
        loader: &mut AtomicStoreLoader,
    ) -> Result<Self, PersistenceError> {
        Ok(Self {
            index_by_leaf_hash: Default::default(),
            index_by_block_hash: Default::default(),
            index_by_txn_hash: Default::default(),
            index_by_proposer_id: Default::default(),
            top_storage: None,
            leaf_storage: LedgerLog::create(loader, "leaves", CACHED_LEAVES_COUNT)?,
            block_storage: LedgerLog::create(loader, "blocks", CACHED_BLOCKS_COUNT)?,
        })
    }

    /// Open an existing [FileSystemStorage] using a persistent storage loader.
    ///
    /// If there is no existing data corresponding to the [FileSystemStorage] data structures, a new
    /// store will be created.
    ///
    /// The [FileSystemStorage] will register its persistent data structures with `loader`. The
    /// caller is responsible for creating an [AtomicStore] from `loader` and managing
    /// synchronization of the store.
    pub async fn open_with_store(loader: &mut AtomicStoreLoader) -> Result<Self, PersistenceError> {
        let leaf_storage =
            LedgerLog::<LeafQueryData<Types>>::open(loader, "leaves", CACHED_LEAVES_COUNT)?;
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
            for (txn_ix, txn) in block.enumerate() {
                update_index_by_hash(&mut index_by_txn_hash, txn.commit(), (height, txn_ix));
            }
        }

        Ok(Self {
            index_by_leaf_hash,
            index_by_block_hash,
            index_by_txn_hash,
            index_by_proposer_id,
            leaf_storage,
            block_storage,
            top_storage: None,
        })
    }

    /// Advance the version of the persistent store without committing changes to persistent state.
    pub fn skip_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.skip_version()?;
        self.block_storage.skip_version()?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }
}

#[async_trait]
impl<Types: NodeType> VersionedDataSource for FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    type Error = PersistenceError;

    async fn commit(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.commit_version().await?;
        self.block_storage.commit_version().await?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    async fn revert(&mut self) {
        self.leaf_storage.revert_version().unwrap();
        self.block_storage.revert_version().unwrap();
    }
}

fn range_iter<T>(
    mut iter: Iter<'_, T>,
    range: impl RangeBounds<usize>,
) -> impl '_ + Iterator<Item = QueryResult<T>>
where
    T: Clone + Serialize + DeserializeOwned,
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

    itertools::unfold((iter, end, pos), |(iter, end, pos)| {
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
        *pos += 1;
        Some(opt.context(MissingSnafu))
    })
}

#[async_trait]
impl<Types: NodeType> AvailabilityStorage<Types> for FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
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

    async fn get_leaf_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_iter(self.leaf_storage.iter(), range).collect())
    }

    async fn get_block_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_iter(self.block_storage.iter(), range).collect())
    }

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        let (height, ix) = self.index_by_txn_hash.get(&hash).context(NotFoundSnafu)?;
        let block = self.get_block((*height as usize).into()).await?;
        Ok((block, ix.clone()))
    }
}

#[async_trait]
impl<Types: NodeType> UpdateAvailabilityData<Types> for FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    type Error = PersistenceError;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        self.leaf_storage
            .insert(leaf.height() as usize, leaf.clone())?;
        self.index_by_leaf_hash.insert(leaf.hash(), leaf.height());
        update_index_by_hash(
            &mut self.index_by_block_hash,
            leaf.block_hash(),
            leaf.height(),
        );
        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        self.block_storage
            .insert(block.height() as usize, block.clone())?;
        for (txn_ix, txn) in block.enumerate() {
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

#[async_trait]
impl<Types: NodeType> NodeDataSource<Types> for FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    async fn block_height(&self) -> QueryResult<usize> {
        Ok(self.leaf_storage.iter().len())
    }

    async fn get_proposals(
        &self,
        id: &SignatureKey<Types>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
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
            .then(|height| self.get_leaf((height as usize).into()))
            .try_collect()
            .await
    }

    async fn count_proposals(&self, id: &SignatureKey<Types>) -> QueryResult<usize> {
        Ok(match self.index_by_proposer_id.get(id) {
            Some(ids) => ids.len(),
            None => 0,
        })
    }
}

#[async_trait]
impl<Types: NodeType> UpdateNodeData<Types> for FileSystemStorage<Types>
where
    Payload<Types>: QueryablePayload,
{
    type Error = PersistenceError;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        self.index_by_proposer_id
            .entry(leaf.proposer())
            .or_default()
            .push(leaf.height());
        Ok(())
    }
}
