//! On-disk storage of node state. Based on [`atomic_store`](https://github.com/EspressoSystems/atomicstore).

mod dual_key_value_store;
mod hash_map_store;

use self::{dual_key_value_store::DualKeyValueStore, hash_map_store::HashMapStore};
use crate::{data::Leaf, traits::StateContents};
use async_std::sync::Mutex;
use async_trait::async_trait;
use atomic_store::{AtomicStore, AtomicStoreLoader};
use commit::Commitment;
use hotshot_types::traits::storage::{
        AtomicStoreSnafu, Storage, StorageError, StorageResult, StorageState, StorageUpdater,
        TestableStorage,
    };
use serde::{de::DeserializeOwned, Serialize};
use snafu::ResultExt;
use std::{path::Path, sync::Arc};
use tempfile::{tempdir, TempDir};
use tracing::{instrument, trace};

/// Inner state of an atomic storage
struct AtomicStorageInner<STATE>
where
    STATE: DeserializeOwned + Serialize + StateContents,
{
    /// Temporary directory storage might live in
    /// (we want to delete the temporary directory when storage is droppped)
    _temp_dir: Option<TempDir>,
    /// The atomic store loader
    atomic_store: Mutex<AtomicStore>,

    /// The Blocks stored by this [`AtomicStorage`]
    blocks: HashMapStore<Commitment<STATE::BlockPayload>, STATE::BlockPayload>,

    /// The [`QuorumCertificate`]s stored by this [`AtomicStorage`]
    qcs: DualKeyValueStore<QuorumCertificate2<STATE>>,

    /// The [`Leaf`s stored by this [`AtomicStorage`]
    ///
    /// In order to maintain the struct constraints, this list must be append only. Once a QC is
    /// inserted, it index _must not_ change
    leaves: DualKeyValueStore<Leaf<STATE>>,

    /// The store of states
    states: HashMapStore<Commitment<Leaf<STATE>>, STATE>,
}

/// Persistent [`Storage`] implementation, based upon [`atomic_store`].
#[derive(Clone)]
pub struct AtomicStorage<STATE>
where
    STATE: DeserializeOwned + Serialize + StateContents,
{
    /// Inner state of the atomic storage
    inner: Arc<AtomicStorageInner<STATE>>,
}

impl<S: StateContents + 'static> TestableStorage<S> for AtomicStorage<S> {
    fn construct_tmp_storage() -> StorageResult<Self> {
        let tempdir = tempdir().map_err(|e| StorageError::InconsistencyError {
            description: e.to_string(),
        })?;
        let loader = AtomicStoreLoader::create(tempdir.path(), "hotshot").map_err(|e| {
            StorageError::InconsistencyError {
                description: e.to_string(),
            }
        })?;
        Self::init_from_loader(loader, Some(tempdir))
            .map_err(|e| StorageError::AtomicStore { source: e })
    }
}

impl<STATE> AtomicStorage<STATE>
where
    STATE: StateContents,
{
    /// Creates an atomic storage at a given path. If files exist, will back up existing directory before creating.
    ///
    /// # Errors
    ///
    /// Returns the underlying errors that the following types can throw:
    /// - [`atomic_store::AtomicStoreLoader`]
    /// - [`atomic_store::AtomicStore`]
    /// - [`atomic_store::RollingLog`]
    /// - [`atomic_store::AppendLog`]
    pub fn create(path: &Path) -> atomic_store::Result<Self> {
        let loader = AtomicStoreLoader::create(path, "hotshot")?;
        Self::init_from_loader(loader, None)
    }

    /// Open an atomic storage at a given path.
    ///
    /// # Errors
    ///
    /// Returns the underlying errors that the following types can throw:
    /// - [`atomic_store::AtomicStoreLoader`]
    /// - [`atomic_store::AtomicStore`]
    /// - [`atomic_store::RollingLog`]
    /// - [`atomic_store::AppendLog`]
    pub fn open(path: &Path) -> atomic_store::Result<Self> {
        let loader = AtomicStoreLoader::load(path, "hotshot")?;
        Self::init_from_loader(loader, None)
    }

    /// Open an atomic storage with a given [`AtomicStoreLoader`]
    ///
    /// # Errors
    ///
    /// Returns the underlying errors that the following types can throw:
    /// - [`atomic_store::AtomicStore`]
    /// - [`atomic_store::RollingLog`]
    /// - [`atomic_store::AppendLog`]
    pub fn init_from_loader(
        mut loader: AtomicStoreLoader,
        dir: Option<TempDir>,
    ) -> atomic_store::Result<Self> {
        let blocks = HashMapStore::load(&mut loader, "hotshot_blocks")?;
        let qcs = DualKeyValueStore::open(&mut loader, "hotshot_qcs")?;
        let leaves = DualKeyValueStore::open(&mut loader, "hotshot_leaves")?;
        let states = HashMapStore::load(&mut loader, "hotshot_states")?;

        let atomic_store = AtomicStore::open(loader)?;

        Ok(Self {
            inner: Arc::new(AtomicStorageInner {
                _temp_dir: dir,
                atomic_store: Mutex::new(atomic_store),
                blocks,
                qcs,
                leaves,
                states,
            }),
        })
    }
}

#[async_trait]
impl<STATE: StateContents> Storage<STATE> for AtomicStorage<STATE> {
    #[instrument(name = "AtomicStorage::get_block", skip_all)]
    async fn get_block(
        &self,
        hash: &Commitment<STATE::BlockPayload>,
    ) -> StorageResult<Option<STATE::BlockPayload>> {
        Ok(self.inner.blocks.get(hash).await)
    }

    #[instrument(name = "AtomicStorage::get_qc", skip_all)]
    async fn get_qc(
        &self,
        hash: &Commitment<STATE::BlockPayload>,
    ) -> StorageResult<Option<QuorumCertificate2<STATE>>> {
        Ok(self.inner.qcs.load_by_key_1_ref(hash).await)
    }

    #[instrument(name = "AtomicStorage::get_newest_qc", skip_all)]
    async fn get_newest_qc(&self) -> StorageResult<Option<QuorumCertificate2<STATE>>> {
        Ok(self.inner.qcs.load_latest(|qc| qc.get_view_number()).await)
    }

    #[instrument(name = "AtomicStorage::get_qc_for_view", skip_all)]
    async fn get_qc_for_view(
        &self,
        view: TYPES::Time,
    ) -> StorageResult<Option<QuorumCertificate2<STATE>>> {
        Ok(self.inner.qcs.load_by_key_2(view).await)
    }

    #[instrument(name = "AtomicStorage::get_leaf", skip_all)]
    async fn get_leaf(&self, hash: &Commitment<Leaf<STATE>>) -> StorageResult<Option<Leaf<STATE>>> {
        Ok(self.inner.leaves.load_by_key_1_ref(hash).await)
    }

    #[instrument(name = "AtomicStorage::get_leaf_by_block", skip_all)]
    async fn get_leaf_by_block(
        &self,
        hash: &Commitment<STATE::BlockPayload>,
    ) -> StorageResult<Option<Leaf<STATE>>> {
        Ok(self.inner.leaves.load_by_key_2_ref(hash).await)
    }

    #[instrument(name = "AtomicStorage::get_state", skip_all)]
    async fn get_state(&self, hash: &Commitment<Leaf<STATE>>) -> StorageResult<Option<STATE>> {
        Ok(self.inner.states.get(hash).await)
    }

    async fn get_internal_state(&self) -> StorageState<STATE> {
        let mut blocks: Vec<(Commitment<STATE::BlockPayload>, STATE::BBlockPayloadlock)> =
            self.inner.blocks.load_all().await.into_iter().collect();

        blocks.sort_by_key(|(hash, _)| *hash);
        let blocks = blocks.into_iter().map(|(_, block)| block).collect();

        let mut leafs: Vec<Leaf<STATE>> = self.inner.leaves.load_all().await;
        leafs.sort_by_cached_key(Leaf::hash);

        let mut quorum_certificates = self.inner.qcs.load_all().await;
        quorum_certificates.sort_by_key(|qc| qc.view_number());

        let mut states: Vec<(Commitment<Leaf<STATE>>, STATE)> =
            self.inner.states.load_all().await.into_iter().collect();
        states.sort_by_key(|(hash, _)| *hash);
        let states = states.into_iter().map(|(_, state)| state).collect();

        StorageState {
            blocks,
            quorum_certificates,
            leafs,
            states,
        }
    }
}

/// Implementation of [`StorageUpdater`] for the [`AtomicStorage`]
struct AtomicStorageUpdater<'a, S: StateContents> {
    /// A reference to the internals of the [`AtomicStorage`]
    inner: &'a AtomicStorageInner<S>,
}

#[async_trait]
impl<'a, STATE: StateContents + 'static> StorageUpdater<'a, STATE>
    for AtomicStorageUpdater<'a, STATE>
{
    #[instrument(name = "AtomicStorage::get_block", skip_all)]
    async fn insert_block(
        &mut self,
        hash: Commitment<STATE::BlockPayload>,
        block: STATE::BlockPayload,
    ) -> StorageResult {
        trace!(?block, "inserting block");
        self.inner
            .blocks
            .insert(hash, block)
            .await
            .context(AtomicStoreSnafu)?;
        Ok(())
    }

    #[instrument(name = "AtomicStorage::insert_leaf", skip_all)]
    async fn insert_leaf(&mut self, leaf: Leaf<STATE>) -> StorageResult {
        self.inner.leaves.insert(leaf).await
    }

    #[instrument(name = "AtomicStorage::insert_qc", skip_all)]
    async fn insert_qc(&mut self, qc: QuorumCertificate2<STATE>) -> StorageResult {
        self.inner.qcs.insert(qc).await
    }

    #[instrument(name = "AtomicStorage::insert_state", skip_all)]
    async fn insert_state(&mut self, state: STATE, hash: Commitment<Leaf<STATE>>) -> StorageResult {
        trace!(?hash, "Inserting state");
        self.inner
            .states
            .insert(hash, state)
            .await
            .context(AtomicStoreSnafu)?;
        Ok(())
    }
}
