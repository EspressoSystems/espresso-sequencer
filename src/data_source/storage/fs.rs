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
        data_source::{BlockId, LeafId, UpdateAvailabilityData},
        query_data::{
            BlockHash, BlockQueryData, LeafHash, LeafQueryData, PayloadQueryData, QueryablePayload,
            TransactionHash, TransactionIndex, VidCommonQueryData,
        },
    },
    data_source::VersionedDataSource,
    node::{NodeDataSource, SyncStatus},
    Header, MissingSnafu, NotFoundSnafu, Payload, QueryResult, SignatureKey, VidShare,
};
use async_trait::async_trait;
use atomic_store::{AtomicStore, AtomicStoreLoader, PersistenceError};
use commit::Committable;
use futures::stream::{self, StreamExt, TryStreamExt};
use hotshot_types::{data::VidCommitment, traits::node_implementation::NodeType};
use serde::{de::DeserializeOwned, Serialize};
use snafu::OptionExt;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::{Bound, RangeBounds};
use std::path::Path;

const CACHED_LEAVES_COUNT: usize = 100;
const CACHED_BLOCKS_COUNT: usize = 100;
const CACHED_VID_COMMON_COUNT: usize = 100;

/// Storage for the APIs provided in this crate, backed by a remote PostgreSQL database.
#[derive(custom_debug::Debug)]
pub struct FileSystemStorage<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    index_by_leaf_hash: HashMap<LeafHash<Types>, u64>,
    index_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_by_payload_hash: HashMap<VidCommitment, u64>,
    index_by_txn_hash: HashMap<TransactionHash<Types>, (u64, TransactionIndex<Types>)>,
    index_by_proposer_id: HashMap<SignatureKey<Types>, Vec<u64>>,
    // We have separate indexes for VID than for other data, because of the special case where VID
    // information does not exist for the genesis block. This means that VID data for blocks that
    // are duplicates of the genesis block may exist but not be findable through the block indexes.
    index_vid_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_vid_by_payload_hash: HashMap<VidCommitment, u64>,
    num_transactions: usize,
    payload_size: usize,
    #[debug(skip)]
    top_storage: Option<AtomicStore>,
    leaf_storage: LedgerLog<LeafQueryData<Types>>,
    block_storage: LedgerLog<BlockQueryData<Types>>,
    vid_storage: LedgerLog<(VidCommonQueryData<Types>, Option<VidShare>)>,
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
            index_by_payload_hash: Default::default(),
            index_by_txn_hash: Default::default(),
            index_by_proposer_id: Default::default(),
            index_vid_by_block_hash: Default::default(),
            index_vid_by_payload_hash: Default::default(),
            num_transactions: 0,
            payload_size: 0,
            top_storage: None,
            leaf_storage: LedgerLog::create(loader, "leaves", CACHED_LEAVES_COUNT)?,
            block_storage: LedgerLog::create(loader, "blocks", CACHED_BLOCKS_COUNT)?,
            vid_storage: LedgerLog::create(loader, "vid_common", CACHED_VID_COMMON_COUNT)?,
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
        let vid_storage = LedgerLog::<(VidCommonQueryData<Types>, Option<VidShare>)>::open(
            loader,
            "vid_common",
            CACHED_VID_COMMON_COUNT,
        )?;

        let mut index_by_proposer_id = HashMap::new();
        let mut index_by_block_hash = HashMap::new();
        let mut index_by_payload_hash = HashMap::new();
        let index_by_leaf_hash = leaf_storage
            .iter()
            .flatten()
            .map(|leaf| {
                index_by_proposer_id
                    .entry(leaf.proposer())
                    .or_insert_with(Vec::new)
                    .push(leaf.height());
                update_index_by_hash(&mut index_by_block_hash, leaf.block_hash(), leaf.height());
                update_index_by_hash(
                    &mut index_by_payload_hash,
                    leaf.payload_hash(),
                    leaf.height(),
                );
                (leaf.hash(), leaf.height())
            })
            .collect();

        let mut index_by_txn_hash = HashMap::new();
        let mut num_transactions = 0;
        let mut payload_size = 0;
        for block in block_storage.iter().flatten() {
            num_transactions += block.len();
            payload_size += block.size() as usize;

            let height = block.height();
            for (txn_ix, txn) in block.enumerate() {
                update_index_by_hash(&mut index_by_txn_hash, txn.commit(), (height, txn_ix));
            }
        }

        let mut index_vid_by_block_hash = HashMap::new();
        let mut index_vid_by_payload_hash = HashMap::new();
        for (common, _) in vid_storage.iter().flatten() {
            update_index_by_hash(
                &mut index_vid_by_block_hash,
                common.block_hash(),
                common.height(),
            );
            update_index_by_hash(
                &mut index_vid_by_payload_hash,
                common.payload_hash(),
                common.height(),
            );
        }

        Ok(Self {
            index_by_leaf_hash,
            index_by_block_hash,
            index_by_payload_hash,
            index_by_txn_hash,
            index_by_proposer_id,
            index_vid_by_block_hash,
            index_vid_by_payload_hash,
            num_transactions,
            payload_size,
            leaf_storage,
            block_storage,
            vid_storage,
            top_storage: None,
        })
    }

    /// Advance the version of the persistent store without committing changes to persistent state.
    pub fn skip_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.skip_version()?;
        self.block_storage.skip_version()?;
        self.vid_storage.skip_version()?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    fn get_block_index(&self, id: BlockId<Types>) -> QueryResult<usize> {
        match id {
            BlockId::Number(n) => Ok(n),
            BlockId::Hash(h) => {
                Ok(*self.index_by_block_hash.get(&h).context(NotFoundSnafu)? as usize)
            }
            BlockId::PayloadHash(h) => {
                Ok(*self.index_by_payload_hash.get(&h).context(NotFoundSnafu)? as usize)
            }
        }
    }

    fn get_vid_index(&self, id: BlockId<Types>) -> QueryResult<usize> {
        match id {
            BlockId::Number(n) => Ok(n),
            BlockId::Hash(h) => Ok(*self
                .index_vid_by_block_hash
                .get(&h)
                .context(NotFoundSnafu)? as usize),
            BlockId::PayloadHash(h) => Ok(*self
                .index_vid_by_payload_hash
                .get(&h)
                .context(NotFoundSnafu)? as usize),
        }
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
        self.vid_storage.commit_version().await?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    async fn revert(&mut self) {
        self.leaf_storage.revert_version().unwrap();
        self.block_storage.revert_version().unwrap();
        self.vid_storage.revert_version().unwrap();
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
            LeafId::Number(n) => n,
            LeafId::Hash(h) => *self.index_by_leaf_hash.get(&h).context(NotFoundSnafu)? as usize,
        };
        self.leaf_storage
            .iter()
            .nth(n)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        self.block_storage
            .iter()
            .nth(self.get_block_index(id)?)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)
    }

    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        self.get_block(id).await.map(|block| block.header)
    }

    async fn get_payload(&self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        self.get_block(id).await.map(PayloadQueryData::from)
    }

    async fn get_vid_common(&self, id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>> {
        Ok(self
            .vid_storage
            .iter()
            .nth(self.get_vid_index(id)?)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)?
            .0)
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

    async fn get_payload_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_iter(self.block_storage.iter(), range)
            .map(|res| res.map(PayloadQueryData::from))
            .collect())
    }

    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Ok(range_iter(self.vid_storage.iter(), range)
            .map(|res| res.map(|(common, _)| common))
            .collect())
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
        self.index_by_proposer_id
            .entry(leaf.proposer())
            .or_default()
            .push(leaf.height());
        update_index_by_hash(
            &mut self.index_by_block_hash,
            leaf.block_hash(),
            leaf.height(),
        );
        update_index_by_hash(
            &mut self.index_by_payload_hash,
            leaf.payload_hash(),
            leaf.height(),
        );
        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        if !self
            .block_storage
            .insert(block.height() as usize, block.clone())?
        {
            // The block was already present.
            return Ok(());
        }
        self.num_transactions += block.len();
        self.payload_size += block.size() as usize;
        for (txn_ix, txn) in block.enumerate() {
            update_index_by_hash(
                &mut self.index_by_txn_hash,
                txn.commit(),
                (block.height(), txn_ix),
            );
        }
        Ok(())
    }

    async fn insert_vid(
        &mut self,
        common: VidCommonQueryData<Types>,
        share: Option<VidShare>,
    ) -> Result<(), Self::Error> {
        let height = common.height();
        let block_hash = common.block_hash();
        let payload_hash = common.payload_hash();
        self.vid_storage.insert(height as usize, (common, share))?;
        update_index_by_hash(&mut self.index_vid_by_block_hash, block_hash, height);
        update_index_by_hash(&mut self.index_vid_by_payload_hash, payload_hash, height);
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

    async fn count_transactions(&self) -> QueryResult<usize> {
        Ok(self.num_transactions)
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        Ok(self.payload_size)
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        self.vid_storage
            .iter()
            .nth(self.get_vid_index(id.into())?)
            .context(NotFoundSnafu)?
            .context(MissingSnafu)?
            .1
            .context(MissingSnafu)
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        let height = self.block_height().await?;

        // The number of missing VID common is just the number of completely missing VID entries,
        // since every entry we have is guaranteed to have the common data.
        let missing_vid = self.vid_storage.missing(height);
        // Missing shares includes the completely missing VID entries, plus any entry which is _not_
        // messing but which has a null share.
        let null_vid_shares: usize = self
            .vid_storage
            .iter()
            .map(|res| if matches!(res, Some((_, None))) { 1 } else { 0 })
            .sum();
        Ok(SyncStatus {
            missing_blocks: self.block_storage.missing(height),
            missing_leaves: self.leaf_storage.missing(height),
            missing_vid_common: missing_vid,
            missing_vid_shares: missing_vid + null_vid_shares,
        })
    }
}
