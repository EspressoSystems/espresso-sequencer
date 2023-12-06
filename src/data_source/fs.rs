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
    VersionedDataSource,
};
use crate::{
    availability::{
        data_source::{
            AvailabilityDataSource, BlockId, LeafId, ResourceId, UpdateAvailabilityData,
        },
        query_data::{
            BlockHash, BlockQueryData, LeafHash, LeafQueryData, QueryablePayload, TransactionHash,
            TransactionIndex,
        },
    },
    metrics::PrometheusMetrics,
    status::data_source::StatusDataSource,
    MissingSnafu, NotFoundSnafu, Payload, QueryResult,
};
use async_trait::async_trait;
use atomic_store::{AtomicStore, AtomicStoreLoader, PersistenceError};
use commit::Committable;
use futures::stream::{self, BoxStream, Stream, StreamExt, TryStreamExt};
use hotshot_types::traits::{node_implementation::NodeType, signature_key::EncodedPublicKey};
use serde::{de::DeserializeOwned, Serialize};
use snafu::OptionExt;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::{Bound, RangeBounds};
use std::path::Path;

const CACHED_LEAVES_COUNT: usize = 100;
const CACHED_BLOCKS_COUNT: usize = 100;

/// A data source for the APIs provided in this crate, backed by the local file system.
///
/// Synchronization and atomicity of persisted data structures are provided via [`atomic_store`].
/// The methods [`commit`](Self::commit), [`revert`](Self::revert), and
/// [`skip_version`](Self::skip_version) of this type can be used to control synchronization in the
/// underlying [`AtomicStore`].
///
/// # Extension and Composition
///
/// [`FileSystemDataSource`] is designed to be both extensible (so you can add additional state to
/// the API modules defined in this crate) and composable (so you can use [`FileSystemDataSource`]
/// as one component of a larger state type for an application with additional modules).
///
/// ## Extension
///
/// Adding additional, application-specific state to [`FileSystemDataSource`] is possible by
/// wrapping it in [`ExtensibleDataSource`](super::ExtensibleDataSource):
///
/// ```
/// # use atomic_store::PersistenceError;
/// # use hotshot_query_service::data_source::{ExtensibleDataSource, FileSystemDataSource};
/// # use hotshot_query_service::testing::mocks::{
/// #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
/// # };
/// # use std::path::Path;
/// # fn doc(storage_path: &Path) -> Result<(), PersistenceError> {
/// type AppState = &'static str;
///
/// let data_source: ExtensibleDataSource<FileSystemDataSource<AppTypes, AppNodeImpl>, AppState> =
///     ExtensibleDataSource::new(FileSystemDataSource::create(storage_path)?, "app state");
/// # Ok(())
/// # }
/// ```
///
/// The [`ExtensibleDataSource`](super::ExtensibleDataSource) wrapper implements all the same data
/// source traits as [`FileSystemDataSource`], and also provides access to the `AppState` parameter
/// for use in API endpoint handlers. This can be used to implement an app-specific data source
/// trait and add a new API endpoint that uses this app-specific data, as described in the
/// [extension guide](crate#extension).
///
/// ## Composition
///
/// Composing [`FileSystemDataSource`] with other module states is in principle simple -- just
/// create an aggregate struct containing both [`FileSystemDataSource`] and your additional module
/// states. A complication arises from how persistent storage is managed: if other modules have
/// their own persistent state, should the storage of [`FileSystemDataSource`] and the other modules
/// be completely independent, or synchronized under the control of a single [`AtomicStore`]?
/// [`FileSystemDataSource`] supports both patterns: when you create it with
/// [`create`](Self::create) or [`open`](Self::open), it will open its own [`AtomicStore`] and
/// manage the synchronization of its own storage, independent of any other persistent data it might
/// be composed with. But when you create it with [`create_with_store`](Self::create_with_store) or
/// [`open_with_store`](Self::open_with_store), you may ask it to register its persistent data
/// structures with an existing [`AtomicStoreLoader`]. If you register other modules' persistent
/// data structures with the same loader, you can create one [`AtomicStore`] that synchronizes all
/// the persistent data. Note, though, that when you choose to use
/// [`create_with_store`](Self::create_with_store) or [`open_with_store`](Self::open_with_store),
/// you become responsible for ensuring that calls to [`AtomicStore::commit_version`] alternate with
/// calls to [`FileSystemDataSource::commit`] or [`FileSystemDataSource::revert`].
///
/// In the following example, we compose HotShot query service modules with other application-
/// specific modules, using a single top-level [`AtomicStore`] to synchronize all persistent
/// storage.
///
/// ```
/// # use async_std::{sync::{Arc, RwLock}, task::spawn};
/// # use atomic_store::{AtomicStore, AtomicStoreLoader};
/// # use futures::StreamExt;
/// # use hotshot::types::SystemContextHandle;
/// # use hotshot_query_service::Error;
/// # use hotshot_query_service::data_source::{
/// #   FileSystemDataSource, UpdateDataSource, VersionedDataSource,
/// # };
/// # use hotshot_query_service::testing::mocks::{
/// #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
/// # };
/// # use std::path::Path;
/// # use tide_disco::App;
/// struct AppState {
///     // Top-level storage coordinator
///     store: AtomicStore,
///     hotshot_qs: FileSystemDataSource<AppTypes, AppNodeImpl>,
///     // additional state for other modules
/// }
///
/// fn init_server(
///     storage_path: &Path,
///     mut hotshot: SystemContextHandle<AppTypes, AppNodeImpl>,
/// ) -> Result<App<Arc<RwLock<AppState>>, Error>, Error> {
///     let mut loader = AtomicStoreLoader::create(storage_path, "my_app") // or `open`
///         .map_err(Error::internal)?;
///     let hotshot_qs = FileSystemDataSource::create_with_store(&mut loader)
///         .map_err(Error::internal)?;
///     // Initialize storage for other modules using the same loader.
///
///     let store = AtomicStore::open(loader).map_err(Error::internal)?;
///     let state = Arc::new(RwLock::new(AppState {
///         store,
///         hotshot_qs,
///         // additional state for other modules
///     }));
///     let mut app = App::with_state(state.clone());
///     // Register API modules.
///
///     spawn(async move {
///         let mut events = hotshot.get_event_stream(Default::default()).await.0;
///         while let Some(event) = events.next().await {
///             let mut state = state.write().await;
///             state.hotshot_qs.update(&event).await.unwrap();
///             // Update other modules' states based on `event`.
///
///             state.hotshot_qs.commit().await.unwrap();
///             // Commit or skip versions for other modules' storage.
///             state.store.commit_version().unwrap();
///         }
///     });
///
///     Ok(app)
/// }
/// ```
#[derive(custom_debug::Debug)]
pub struct FileSystemDataSource<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    index_by_leaf_hash: HashMap<LeafHash<Types>, u64>,
    index_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_by_txn_hash: HashMap<TransactionHash<Types>, (u64, TransactionIndex<Types>)>,
    index_by_proposer_id: HashMap<EncodedPublicKey, Vec<u64>>,
    #[debug(skip)]
    top_storage: Option<AtomicStore>,
    leaf_storage: LedgerLog<LeafQueryData<Types>>,
    block_storage: LedgerLog<BlockQueryData<Types>>,
    metrics: PrometheusMetrics,
}

impl<Types: NodeType> FileSystemDataSource<Types>
where
    Payload<Types>: QueryablePayload,
{
    /// Create a new [FileSystemDataSource] with storage at `path`.
    ///
    /// If there is already data at `path`, it will be archived.
    ///
    /// The [FileSystemDataSource] will manage its own persistence synchronization.
    pub fn create(path: &Path) -> Result<Self, PersistenceError> {
        let mut loader = AtomicStoreLoader::create(path, "hotshot_data_source")?;
        let mut data_source = Self::create_with_store(&mut loader)?;
        data_source.top_storage = Some(AtomicStore::open(loader)?);
        Ok(data_source)
    }

    /// Open an existing [FileSystemDataSource] from storage at `path`.
    ///
    /// If there is no data at `path`, a new store will be created.
    ///
    /// The [FileSystemDataSource] will manage its own persistence synchronization.
    pub fn open(path: &Path) -> Result<Self, PersistenceError> {
        let mut loader = AtomicStoreLoader::load(path, "hotshot_data_source")?;
        let mut data_source = Self::open_with_store(&mut loader)?;
        data_source.top_storage = Some(AtomicStore::open(loader)?);
        Ok(data_source)
    }

    /// Create a new [FileSystemDataSource] using a persistent storage loader.
    ///
    /// If there is existing data corresponding to the [FileSystemDataSource] data structures, it
    /// will be archived.
    ///
    /// The [FileSystemDataSource] will register its persistent data structures with `loader`. The
    /// caller is responsible for creating an [AtomicStore] from `loader` and managing
    /// synchronization of the store.
    pub fn create_with_store(loader: &mut AtomicStoreLoader) -> Result<Self, PersistenceError> {
        Ok(Self {
            index_by_leaf_hash: Default::default(),
            index_by_block_hash: Default::default(),
            index_by_txn_hash: Default::default(),
            index_by_proposer_id: Default::default(),
            top_storage: None,
            leaf_storage: LedgerLog::create(loader, "leaves", CACHED_LEAVES_COUNT)?,
            block_storage: LedgerLog::create(loader, "blocks", CACHED_BLOCKS_COUNT)?,
            metrics: Default::default(),
        })
    }

    /// Open an existing [FileSystemDataSource] using a persistent storage loader.
    ///
    /// If there is no existing data corresponding to the [FileSystemDataSource] data structures, a
    /// new store will be created.
    ///
    /// The [FileSystemDataSource] will register its persistent data structures with `loader`. The
    /// caller is responsible for creating an [AtomicStore] from `loader` and managing
    /// synchronization of the store.
    pub fn open_with_store(loader: &mut AtomicStoreLoader) -> Result<Self, PersistenceError> {
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
            for (txn_ix, txn) in block.payload().enumerate() {
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
            metrics: Default::default(),
        })
    }

    /// Advance the version of the persistent store without committing changes to persistent state.
    ///
    /// This function is useful when the [AtomicStore] synchronizing storage for this
    /// [FileSystemDataSource] is being managed by the caller. The caller may want to persist some
    /// changes to other modules whose state is managed by the same [AtomicStore]. In order to call
    /// [AtomicStore::commit_version], the version of this [FileSystemDataSource] must be advanced,
    /// either by [commit](Self::commit) or, if there are no outstanding changes,
    /// [skip_version](Self::skip_version).
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
impl<Types: NodeType> VersionedDataSource for FileSystemDataSource<Types>
where
    Payload<Types>: QueryablePayload,
{
    type Error = PersistenceError;

    /// Commit the current state to persistent storage.
    ///
    /// If the [FileSystemDataSource] is managing its own [AtomicStore] (i.e. it was created with
    /// [create](Self::create) or [open](Self::open)) it will update the global version as well.
    /// Otherwise, the caller is responsible for calling [AtomicStore::commit_version] after calling
    /// this function.
    async fn commit(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.commit_version().await?;
        self.block_storage.commit_version().await?;
        if let Some(store) = &mut self.top_storage {
            store.commit_version()?;
        }
        Ok(())
    }

    /// Revert changes made to persistent storage since the last call to
    /// [commit](Self::commit).
    async fn revert(&mut self) {
        self.leaf_storage.revert_version().unwrap();
        self.block_storage.revert_version().unwrap();
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
impl<Types: NodeType> AvailabilityDataSource<Types> for FileSystemDataSource<Types>
where
    Payload<Types>: QueryablePayload,
{
    type LeafStream = BoxStream<'static, QueryResult<LeafQueryData<Types>>>;
    type BlockStream = BoxStream<'static, QueryResult<BlockQueryData<Types>>>;

    type LeafRange<'a, R> = BoxStream<'a, QueryResult<LeafQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;
    type BlockRange<'a, R> = BoxStream<'a, QueryResult<BlockQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync,
    {
        let n = match id.into() {
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

    async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let n = match id.into() {
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
        let block = self.get_block(*height as usize).await?;
        Ok((block, ix.clone()))
    }

    async fn get_proposals(
        &self,
        id: &EncodedPublicKey,
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
            .then(|height| self.get_leaf(height as usize))
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
        Ok(self
            .leaf_storage
            .subscribe(height)
            .context(MissingSnafu)?
            .map(Ok)
            .boxed())
    }

    async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> {
        Ok(self
            .block_storage
            .subscribe(height)
            .context(MissingSnafu)?
            .map(Ok)
            .boxed())
    }
}

#[async_trait]
impl<Types: NodeType> UpdateAvailabilityData<Types> for FileSystemDataSource<Types>
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
        self.index_by_proposer_id
            .entry(leaf.proposer())
            .or_default()
            .push(leaf.height());
        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        self.block_storage
            .insert(block.height() as usize, block.clone())?;
        for (txn_ix, txn) in block.payload().enumerate() {
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
impl<Types: NodeType> StatusDataSource for FileSystemDataSource<Types>
where
    Payload<Types>: QueryablePayload,
{
    async fn block_height(&self) -> QueryResult<usize> {
        Ok(self.leaf_storage.iter().len())
    }

    fn metrics(&self) -> &PrometheusMetrics {
        &self.metrics
    }
}

#[cfg(any(test, feature = "testing"))]
mod impl_testable_data_source {
    use super::*;
    use crate::{
        data_source::UpdateDataSource,
        testing::mocks::{DataSourceLifeCycle, MockTypes},
    };
    use hotshot::types::Event;
    use tempdir::TempDir;

    #[async_trait]
    impl DataSourceLifeCycle for FileSystemDataSource<MockTypes> {
        type Storage = TempDir;

        async fn create(node_id: usize) -> Self::Storage {
            TempDir::new(&format!("file_system_data_source_{node_id}")).unwrap()
        }

        async fn connect(storage: &Self::Storage) -> Self {
            Self::open(storage.path()).unwrap()
        }

        async fn handle_event(&mut self, event: &Event<MockTypes>) {
            self.update(event).await.unwrap();
            self.commit().await.unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{availability_tests, status_tests};
    use super::FileSystemDataSource;
    use crate::testing::mocks::MockTypes;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_data_source_tests!(FileSystemDataSource<MockTypes>);
}
