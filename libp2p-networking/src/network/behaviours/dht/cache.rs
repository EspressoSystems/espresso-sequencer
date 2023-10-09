use std::{
    collections::{BTreeMap, HashMap},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};

use async_compatibility_layer::art::async_block_on;
use async_lock::RwLock;
use bincode::Options;
use dashmap::{mapref::one::Ref, DashMap};
use hotshot_constants::KAD_DEFAULT_REPUB_INTERVAL_SEC;
use hotshot_utils::bincode::bincode_opts;
use snafu::{ResultExt, Snafu};

/// Error wrapper type for cache
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CacheError {
    /// Failed to read or write from disk
    Disk {
        /// source of error
        source: std::io::Error,
    },

    /// Failure to serialize the cache
    Serialization {
        /// source of error
        source: Box<bincode::ErrorKind>,
    },

    /// Failure to deserialize the cache
    Deserialization {
        /// source of error
        source: Box<bincode::ErrorKind>,
    },

    /// General cache error
    GeneralCache {
        /// source of error
        source: Box<bincode::ErrorKind>,
    },
}

#[derive(Clone, derive_builder::Builder, custom_debug::Debug, Default)]
pub struct Config {
    #[builder(default = "Some(\"dht.cache\".to_string())")]
    pub filename: Option<String>,
    #[builder(default = "Duration::from_secs(KAD_DEFAULT_REPUB_INTERVAL_SEC * 16)")]
    pub expiry: Duration,
    #[builder(default = "4")]
    pub max_disk_parity_delta: u32,
}

impl Default for Cache {
    fn default() -> Self {
        async_block_on(Self::new(Config::default()))
    }
}

pub struct Cache {
    /// the cache's config
    config: Config,

    /// the cache for records (key -> value)
    cache: Arc<DashMap<Vec<u8>, Vec<u8>>>,
    /// the expiries for the dht cache, in order (expiry time -> key)
    expiries: Arc<RwLock<BTreeMap<SystemTime, Vec<u8>>>>,

    /// number of inserts since the last save
    disk_parity_delta: Arc<AtomicU32>,
}

impl Cache {
    pub async fn new(config: Config) -> Self {
        let cache = Self {
            cache: Arc::new(DashMap::new()),
            expiries: Arc::new(RwLock::new(BTreeMap::new())),
            config,
            disk_parity_delta: Arc::new(AtomicU32::new(0)),
        };

        // try loading from file
        if let Err(err) = cache.load().await {
            tracing::warn!("failed to load cache from file: {}", err);
        };

        cache
    }

    pub async fn load(&self) -> Result<(), CacheError> {
        if let Some(filename) = &self.config.filename {
            let encoded = std::fs::read(filename).context(DiskSnafu)?;

            let cache: HashMap<SystemTime, (Vec<u8>, Vec<u8>)> = bincode_opts()
                .deserialize(&encoded)
                .context(DeserializationSnafu)?;

            // inline prune and insert
            let now = SystemTime::now();
            for (expiry, (key, value)) in cache {
                if now < expiry {
                    self.cache.insert(key.clone(), value);
                    self.expiries.write().await.insert(expiry, key);
                }
            }
        }

        Ok(())
    }

    pub async fn save(&self) -> Result<(), CacheError> {
        if let Some(filename) = &self.config.filename {
            // prune first
            self.prune().await;

            // serialize
            let mut cache_to_write = HashMap::new();
            let expiries = self.expiries.read().await;
            for (expiry, key) in &*expiries {
                if let Some(entry) = self.cache.get(key) {
                    cache_to_write.insert(expiry, (key, entry.value().clone()));
                } else {
                    tracing::warn!("key not found in cache: {:?}", key);
                    Err(CacheError::GeneralCache {
                        source: Box::new(bincode::ErrorKind::Custom(
                            "key not found in cache".to_string(),
                        )),
                    })?;
                };
            }

            let encoded = bincode_opts()
                .serialize(&cache_to_write)
                .context(SerializationSnafu)?;

            std::fs::write(filename, encoded).context(DiskSnafu)?;
        }

        Ok(())
    }

    async fn prune(&self) {
        let now = SystemTime::now();
        let mut expiries = self.expiries.write().await;
        let mut removed: u32 = 0;

        while let Some((expires, key)) = expiries.pop_first() {
            if now > expires {
                self.cache.remove(&key);
                removed += 1;
            } else {
                expiries.insert(expires, key);
                break;
            }
        }

        if removed > 0 {
            self.disk_parity_delta.fetch_add(removed, Ordering::Relaxed);
        }
    }

    pub async fn get(&self, key: &Vec<u8>) -> Option<Ref<'_, Vec<u8>, Vec<u8>>> {
        // prune, save if necessary
        self.prune().await;
        self.save_if_necessary().await;

        // get
        self.cache.get(key)
    }

    pub async fn insert(&self, key: Vec<u8>, value: Vec<u8>) {
        // insert into cache and expiries
        self.cache.insert(key.clone(), value);
        self.expiries
            .write()
            .await
            .insert(SystemTime::now() + self.config.expiry, key);

        // save if reached max disk parity delta
        self.disk_parity_delta.fetch_add(1, Ordering::Relaxed);
        self.save_if_necessary().await;
    }

    async fn save_if_necessary(&self) {
        let cur_disk_parity_delta = self.disk_parity_delta.load(Ordering::Relaxed);
        if cur_disk_parity_delta >= self.config.max_disk_parity_delta {
            if let Err(err) = self.save().await {
                tracing::error!("failed to save cache to file: {}", err);
            };
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use async_compatibility_layer::art::async_sleep;
    use libp2p_identity::PeerId;
    use tracing::instrument;

    /// cache eviction test
    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_dht_cache_eviction() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();

        // cache with 1s eviction
        let cache = Cache::new(Config {
            filename: None,
            expiry: Duration::from_secs(1),
            max_disk_parity_delta: 4,
        })
        .await;

        let (key, value) = (PeerId::random(), PeerId::random());

        // insert
        cache.insert(key.to_bytes(), value.to_bytes()).await;

        // check that it is in the cache and expiries
        assert_eq!(
            cache.get(&key.to_bytes()).await.unwrap().value(),
            &value.to_bytes()
        );
        assert_eq!(cache.expiries.read().await.len(), 1);

        // sleep for 1s
        async_sleep(Duration::from_secs(1)).await;

        // check that now is evicted
        assert!(cache.get(&key.to_bytes()).await.is_none());

        // check that the cache and expiries are empty
        assert!(cache.expiries.read().await.is_empty());
        assert!(cache.cache.is_empty());
    }

    /// cache add test
    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_dht_cache_save_load() {
        let _ = std::fs::remove_file("test.cache");

        let cache = Cache::new(Config {
            filename: Some("test.cache".to_string()),
            expiry: Duration::from_secs(600),
            max_disk_parity_delta: 4,
        })
        .await;

        // add 10 key-value pairs to the cache
        for i in 0u8..10u8 {
            let (key, value) = (vec![i; 1], vec![i + 1; 1]);
            cache.insert(key, value).await;
        }

        // save the cache
        cache.save().await.unwrap();

        // load the cache
        let cache = Cache::new(Config {
            filename: Some("test.cache".to_string()),
            expiry: Duration::from_secs(600),
            max_disk_parity_delta: 4,
        })
        .await;

        // check that the cache has the 10 key-value pairs
        for i in 0u8..10u8 {
            let (key, value) = (vec![i; 1], vec![i + 1; 1]);
            assert_eq!(cache.get(&key).await.unwrap().value(), &value);
        }

        // delete the cache file
        let _ = std::fs::remove_file("test.cache");
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_dht_disk_parity() {
        let _ = std::fs::remove_file("test.cache");

        let cache = Cache::new(Config {
            // tests run sequentially, shouldn't matter
            filename: Some("test.cache".to_string()),
            expiry: Duration::from_secs(600),
            max_disk_parity_delta: 4,
        })
        .await;

        // insert into cache
        for i in 0..3 {
            cache.insert(vec![i; 1], vec![i + 1; 1]).await;
        }

        // check that file is not saved
        assert!(!std::path::Path::new("test.cache").exists());

        // insert into cache
        cache.insert(vec![0; 1], vec![1; 1]).await;

        // check that file is saved
        assert!(std::path::Path::new("test.cache").exists());

        // delete the cache file
        _ = std::fs::remove_file("test.cache");
    }
}
