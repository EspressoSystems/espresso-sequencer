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
        data_source::{AvailabilityDataSource, UpdateAvailabilityData},
        query_data::{BlockHash, BlockQueryData, LeafHash, LeafQueryData, TransactionHash},
    },
    metrics::{MetricsError, PrometheusMetrics},
    status::{
        data_source::{StatusDataSource, UpdateStatusData},
        query_data::MempoolQueryData,
    },
};
use atomic_store::{
    load_store::BincodeLoadStore, AppendLog, AtomicStore, AtomicStoreLoader, PersistenceError,
};
use hotshot_types::traits::{
    metrics::Metrics, node_implementation::NodeTypes, signature_key::EncodedPublicKey, Block,
};
use std::collections::HashMap;
use std::path::Path;
use tracing::warn;

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
pub struct QueryData<Types: NodeTypes, UserData> {
    cached_leaves_start: usize,
    cached_leaves: Vec<Option<LeafQueryData<Types>>>,
    cached_blocks_start: usize,
    cached_blocks: Vec<Option<BlockQueryData<Types>>>,
    index_by_leaf_hash: HashMap<LeafHash<Types>, u64>,
    index_by_block_hash: HashMap<BlockHash<Types>, u64>,
    index_by_txn_hash: HashMap<TransactionHash<Types>, (u64, u64)>,
    index_by_proposer_id: HashMap<EncodedPublicKey, Vec<u64>>,
    #[debug(skip)]
    top_storage: Option<AtomicStore>,
    leaf_storage: AppendLog<BincodeLoadStore<Option<LeafQueryData<Types>>>>,
    block_storage: AppendLog<BincodeLoadStore<Option<BlockQueryData<Types>>>>,
    metrics: PrometheusMetrics,
    user_data: UserData,
    _marker: std::marker::PhantomData<Types>,
}

impl<Types: NodeTypes, UserData> QueryData<Types, UserData> {
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
    pub fn open(path: &Path, user_data: UserData) -> Result<Self, PersistenceError> {
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
            cached_leaves_start: 0,
            cached_leaves: vec![],
            cached_blocks_start: 0,
            cached_blocks: vec![],
            index_by_leaf_hash: Default::default(),
            index_by_block_hash: Default::default(),
            index_by_txn_hash: Default::default(),
            index_by_proposer_id: Default::default(),
            top_storage: None,
            leaf_storage: AppendLog::create(
                loader,
                Default::default(),
                "leaves",
                1u64 << 21, // 10 MB
            )?,
            block_storage: AppendLog::create(
                loader,
                Default::default(),
                "blocks",
                1u64 << 21, // 10 MB
            )?,
            metrics: Default::default(),
            user_data,
            _marker: Default::default(),
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
    ) -> Result<Self, PersistenceError> {
        let leaf_storage = AppendLog::<BincodeLoadStore<Option<LeafQueryData<Types>>>>::load(
            loader,
            Default::default(),
            "leaves",
            1u64 << 21, // 10 MB
        )?;
        let block_storage = AppendLog::<BincodeLoadStore<Option<BlockQueryData<Types>>>>::load(
            loader,
            Default::default(),
            "blocks",
            1u64 << 21, // 10 MB
        )?;

        let stored_leaves_len = leaf_storage.iter().len();
        let cached_leaves_start = if stored_leaves_len > CACHED_LEAVES_COUNT {
            stored_leaves_len - CACHED_LEAVES_COUNT
        } else {
            0
        };
        let cached_leaves = leaf_storage
            .iter()
            .skip(cached_leaves_start)
            .map(|r| {
                if let Err(e) = &r {
                    warn!("failed to load leaf. Error: {}", e);
                }
                // We treat missing leaves and failed-to-load leaves the same:
                // if we failed to load a leaf, it is now missing!
                r.ok().flatten()
            })
            .collect::<Vec<_>>();

        let stored_blocks_len = block_storage.iter().len();
        let cached_blocks_start = if stored_blocks_len > CACHED_BLOCKS_COUNT {
            stored_blocks_len - CACHED_BLOCKS_COUNT
        } else {
            0
        };
        let cached_blocks = block_storage
            .iter()
            .skip(cached_blocks_start)
            .map(|r| {
                if let Err(e) = &r {
                    warn!("failed to load block. Error: {}", e);
                }
                // We treat missing block and failed-to-load blocks the same:
                // if we failed to load a block, it is now missing!
                r.ok().flatten()
            })
            .collect::<Vec<_>>();
        let mut index_by_proposer_id = HashMap::new();
        let mut index_by_block_hash = HashMap::new();
        let index_by_leaf_hash = leaf_storage
            .iter()
            .filter_map(|res| match res {
                Err(e) => {
                    warn!("failed to load leaf. Error: {}", e);
                    None
                }
                Ok(None) => {
                    // If a leaf is missing, we can't add it to the index.
                    None
                }
                Ok(Some(leaf)) => {
                    index_by_proposer_id
                        .entry(leaf.leaf.proposer_id)
                        .or_insert_with(Vec::new)
                        .push(leaf.height);
                    index_by_block_hash.insert(leaf.leaf.justify_qc.block_commitment, leaf.height);
                    Some((leaf.hash, leaf.height))
                }
            })
            .collect();
        let index_by_txn_hash = block_storage
            .iter()
            .flat_map(|res| match res {
                Err(e) => {
                    warn!("failed to load block. Error: {}", e);
                    vec![]
                }
                Ok(None) => {
                    // If a block is missing, we can't add it to the index.
                    vec![]
                }
                Ok(Some(block)) => block
                    .block
                    .contained_transactions()
                    .into_iter()
                    .enumerate()
                    .map(|(txn_id, txn_hash)| (txn_hash, (block.height, txn_id as u64)))
                    .collect(),
            })
            .collect();

        Ok(QueryData {
            cached_leaves_start,
            cached_leaves,
            cached_blocks_start,
            cached_blocks,
            index_by_leaf_hash,
            index_by_block_hash,
            index_by_txn_hash,
            index_by_proposer_id,
            leaf_storage,
            block_storage,
            top_storage: None,
            metrics: Default::default(),
            user_data,
            _marker: Default::default(),
        })
    }

    /// Commit the current state to persistent storage.
    ///
    /// If the [QueryData] is managing its own [AtomicStore] (i.e. it was created with
    /// [create](Self::create) or [open](Self::open)) it will update the global version as well.
    /// Otherwise, the caller is responsible for calling [AtomicStore::commit_version] after calling
    /// this function.
    pub fn commit_version(&mut self) -> Result<(), PersistenceError> {
        self.leaf_storage.commit_version()?;
        self.block_storage.commit_version()?;
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

impl<Types: NodeTypes, UserData> AsRef<UserData> for QueryData<Types, UserData> {
    fn as_ref(&self) -> &UserData {
        &self.user_data
    }
}

impl<Types: NodeTypes, UserData> AsMut<UserData> for QueryData<Types, UserData> {
    fn as_mut(&mut self) -> &mut UserData {
        &mut self.user_data
    }
}

impl<Types: NodeTypes, UserData> AvailabilityDataSource<Types> for QueryData<Types, UserData> {
    type LeafIterType<'a> = std::vec::IntoIter<Option<LeafQueryData<Types>>> where UserData: 'a;
    type BlockIterType<'a> = std::vec::IntoIter<Option<BlockQueryData<Types>>> where UserData: 'a;

    fn get_nth_leaf_iter(&self, _n: usize) -> Self::LeafIterType<'_> {
        unimplemented!()
    }

    fn get_nth_block_iter(&self, _n: usize) -> Self::BlockIterType<'_> {
        unimplemented!()
    }

    fn get_leaf_index_by_hash(&self, _hash: LeafHash<Types>) -> Option<u64> {
        unimplemented!()
    }

    fn get_block_index_by_hash(&self, _hash: BlockHash<Types>) -> Option<u64> {
        unimplemented!()
    }

    fn get_txn_index_by_hash(&self, _hash: TransactionHash<Types>) -> Option<(u64, u64)> {
        unimplemented!()
    }

    fn get_block_ids_by_proposer_id(&self, _id: EncodedPublicKey) -> Vec<u64> {
        unimplemented!()
    }
}

impl<Types: NodeTypes, UserData> UpdateAvailabilityData<Types> for QueryData<Types, UserData> {
    type Error = PersistenceError;

    fn append_leaves(
        &mut self,
        _leaves: Vec<Option<LeafQueryData<Types>>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn append_blocks(
        &mut self,
        _blocks: Vec<Option<BlockQueryData<Types>>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<Types: NodeTypes, UserData> StatusDataSource for QueryData<Types, UserData> {
    type Error = MetricsError;

    fn block_height(&self) -> Result<usize, Self::Error> {
        unimplemented!()
    }

    fn mempool_info(&self) -> Result<MempoolQueryData, Self::Error> {
        Ok(MempoolQueryData {
            transaction_count: self.metrics.get_gauge("outstanding_transactions")?.get() as u64,
        })
    }

    fn success_rate(&self) -> Result<f64, Self::Error> {
        let total_views = self.metrics.get_counter("currenv_view")?.get() as f64;
        // By definition, a successful view is any which committed a block.
        Ok(self.block_height()? as f64 / total_views)
    }

    fn export_metrics(&self) -> Result<String, Self::Error> {
        self.metrics.prometheus()
    }
}

impl<Types: NodeTypes, UserData> UpdateStatusData for QueryData<Types, UserData> {
    fn metrics(&self) -> Box<dyn Metrics> {
        Box::new(self.metrics.clone())
    }
}
