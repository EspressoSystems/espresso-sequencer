//! A store based on [`RollingLog`]

use async_std::sync::RwLock;
use atomic_store::{load_store::BincodeLoadStore, AtomicStoreLoader, RollingLog};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, hash::Hash};

/// A store with [`RollingLog`] as the storage system.
pub struct HashMapStore<K, V>
where
    K: Eq + Hash,
    HashMap<K, V>: Serialize + DeserializeOwned,
{
    /// Inner value
    inner: RwLock<Inner<K, V>>,
}

/// The inner value of the [`HashMapStore`]
struct Inner<K, V>
where
    K: Eq + Hash,
    HashMap<K, V>: Serialize + DeserializeOwned,
{
    /// The underlying atomic_store store
    store: RollingLog<BincodeLoadStore<HashMap<K, V>>>,
    /// Data currently loaded in the store
    data: HashMap<K, V>,
}

impl<K, V> HashMapStore<K, V>
where
    K: Eq + Hash,
    V: Clone,
    HashMap<K, V>: Serialize + DeserializeOwned + Clone,
{
    /// Load a `HashMapStore` with the given loader and name.
    ///
    /// # Errors
    ///
    /// Returns any errors that [`RollingLog`]'s `load` returns.
    pub fn load(loader: &mut AtomicStoreLoader, name: &str) -> atomic_store::Result<Self> {
        let store = RollingLog::load(loader, BincodeLoadStore::default(), name, 1024)?;
        let data = store.load_latest().unwrap_or_default();
        Ok(Self {
            inner: RwLock::new(Inner { store, data }),
        })
    }

    /// Get an entry in this store. Returning `Some(V)` if it was found.
    pub async fn get(&self, hash: &K) -> Option<V> {
        let read = self.inner.read().await;
        read.data.get(hash).cloned()
    }

    /// Insert a new key-value entry into the store. This won't be committed untill `commit` is called.
    pub async fn insert(&self, key: K, val: V) -> atomic_store::Result<()> {
        let mut lock = self.inner.write().await;
        // Make sure to commit the store first before updating the internal value
        // this makes sure that in a case of an error, the internal state is still correct
        let mut data = lock.data.clone();
        data.insert(key, val);
        lock.store.store_resource(&data)?;

        lock.data = data;
        Ok(())
    }

    /// Commit this rolling store, returning a commit lock.
    ///
    /// Once all stores are committed, you need to call `apply` on this lock, or else the commit will be reverted once the lock goes out of scope.
    ///
    /// # Errors
    ///
    /// Returns any errors that [`RollingLog`]'s `store_resource` and `commit_version` returns.
    pub async fn commit_version(&self) -> atomic_store::Result<()> {
        let mut lock = self.inner.write().await;
        lock.store.commit_version()?;
        Ok(())
    }
}

impl<K, V> HashMapStore<K, V>
where
    HashMap<K, V>: Serialize + DeserializeOwned + Clone,
    K: Eq + Hash,
{
    /// Returns all data stored in this [`HashMapStore`].
    pub async fn load_all(&self) -> HashMap<K, V> {
        self.inner.read().await.data.clone()
    }
}
