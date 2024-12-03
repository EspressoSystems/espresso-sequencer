//! This file contains the `FileBackedStore` struct, which is a wrapper around a `RecordStore`
//! that occasionally saves the DHT to a file on disk.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use delegate::delegate;
use libp2p::kad::store::{RecordStore, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// A `RecordStore` wrapper that occasionally saves the DHT to a file on disk.
pub struct FileBackedStore<R: RecordStore> {
    /// The underlying store
    underlying_store: R,

    /// The path to the file
    path: String,

    /// The maximum number of records that can be added to the store before the store is saved to a file
    max_record_delta: u64,

    /// The running delta between the records in the file and the records in the underlying store
    record_delta: u64,
}

/// A serializable version of a Libp2p `Record`
#[derive(Serialize, Deserialize)]
pub struct SerializableRecord {
    /// The key of the record
    pub key: libp2p::kad::RecordKey,
    /// The value of the record
    pub value: Vec<u8>,
    /// The (original) publisher of the record.
    pub publisher: Option<libp2p::PeerId>,
    /// The record expiration time in seconds since the Unix epoch
    ///
    /// This is an approximation of the expiration time because we can't
    /// serialize an `Instant` directly.
    pub expires_unix_secs: Option<u64>,
}

/// Approximate an `Instant` to the number of seconds since the Unix epoch
fn instant_to_unix_seconds(instant: Instant) -> anyhow::Result<u64> {
    // Get the current instant and system time
    let now_instant = Instant::now();
    let now_system = SystemTime::now();

    // Get the duration of time between the instant and now
    if instant > now_instant {
        Ok(now_system
            .checked_add(instant - now_instant)
            .with_context(|| "Overflow when approximating expiration time")?
            .duration_since(UNIX_EPOCH)
            .with_context(|| "Failed to get duration since Unix epoch")?
            .as_secs())
    } else {
        Ok(now_system
            .checked_sub(now_instant - instant)
            .with_context(|| "Underflow when approximating expiration time")?
            .duration_since(UNIX_EPOCH)
            .with_context(|| "Failed to get duration since Unix epoch")?
            .as_secs())
    }
}

/// Convert a unix-second timestamp to an `Instant`
fn unix_seconds_to_instant(unix_secs: u64) -> anyhow::Result<Instant> {
    // Get the current instant and unix time
    let now_instant = Instant::now();
    let unix_secs_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .with_context(|| "Failed to get duration since Unix epoch")?
        .as_secs();

    if unix_secs > unix_secs_now {
        // If the instant is in the future, add the duration to the current time
        now_instant
            .checked_add(Duration::from_secs(unix_secs - unix_secs_now))
            .with_context(|| "Overflow when calculating future instant")
    } else {
        // If the instant is in the past, subtract the duration from the current time
        now_instant
            .checked_sub(Duration::from_secs(unix_secs_now - unix_secs))
            .with_context(|| "Underflow when calculating past instant")
    }
}

/// Allow conversion from a `libp2p::kad::Record` to a `SerializableRecord`
impl TryFrom<libp2p::kad::Record> for SerializableRecord {
    type Error = anyhow::Error;

    fn try_from(record: libp2p::kad::Record) -> anyhow::Result<Self> {
        Ok(SerializableRecord {
            key: record.key,
            value: record.value,
            publisher: record.publisher,
            expires_unix_secs: record.expires.map(instant_to_unix_seconds).transpose()?,
        })
    }
}

/// Allow conversion from a `SerializableRecord` to a `libp2p::kad::Record`
impl TryFrom<SerializableRecord> for libp2p::kad::Record {
    type Error = anyhow::Error;

    fn try_from(record: SerializableRecord) -> anyhow::Result<Self> {
        Ok(libp2p::kad::Record {
            key: record.key,
            value: record.value,
            publisher: record.publisher,
            expires: record
                .expires_unix_secs
                .map(unix_seconds_to_instant)
                .transpose()?,
        })
    }
}

impl<R: RecordStore> FileBackedStore<R> {
    /// Create a new `FileBackedStore` with the given underlying store and path.
    ///
    /// `max_record_delta` is the maximum number of records that can be added to the store before
    /// the store is saved to a file.
    pub fn new(underlying_store: R, path: String, max_record_delta: u64) -> Self {
        // Create the new store
        let mut store = FileBackedStore {
            underlying_store,
            path: path.clone(),
            max_record_delta,
            record_delta: 0,
        };

        // Try to restore the DHT from a file. If it fails, warn and start with an empty store
        if let Err(err) = store.restore_from_file(path) {
            warn!(
                "Failed to restore DHT from file: {:?}. Starting with empty store",
                err
            );
        }

        // Return the new store
        store
    }

    /// Attempt to save the DHT to the file at the given path
    ///
    /// # Errors
    /// - If we fail to serialize the DHT
    /// - If we fail to write the serialized DHT to the file
    pub fn save_to_file(&mut self) -> anyhow::Result<()> {
        debug!("Saving DHT to file");

        // Get all records and convert them to their serializable counterparts
        let serializable_records: Vec<_> = self
            .underlying_store
            .records()
            .filter_map(|record| {
                SerializableRecord::try_from(record.into_owned())
                    .map_err(|err| {
                        warn!("Failed to convert record to serializable record: {:?}", err);
                    })
                    .ok()
            })
            .collect();

        // Serialize the records
        let contents = bincode::serialize(&serializable_records)
            .with_context(|| "Failed to serialize records")?;

        // Write the contents to the file
        std::fs::write(self.path.clone(), contents)
            .with_context(|| "Failed to write DHT to file")?;

        debug!("Saved DHT to file");

        Ok(())
    }

    /// Attempt to restore the DHT to the underlying store from the file at the given path
    ///
    /// # Errors
    /// - If we fail to read the file
    /// - If we fail to deserialize the file
    pub fn restore_from_file(&mut self, path: String) -> anyhow::Result<()> {
        debug!("Restoring DHT from file");

        // Read the contents of the file as a `HashMap` of `Key` to `Vec<u8>`
        let contents = std::fs::read(path).with_context(|| "Failed to read DHT file")?;

        // Convert the contents to a `HashMap` of `RecordKey` to `Vec<u8>`
        let serializable_records: Vec<SerializableRecord> =
            bincode::deserialize(&contents).with_context(|| "Failed to parse DHT file")?;

        // Put all records into the new store
        for serializable_record in serializable_records {
            // Convert the serializable record back to a `libp2p::kad::Record`
            match libp2p::kad::Record::try_from(serializable_record) {
                Ok(record) => {
                    // Put the record into the new store
                    if let Err(err) = self.underlying_store.put(record) {
                        warn!("Failed to restore record from file: {:?}", err);
                    }
                }
                Err(err) => {
                    warn!("Failed to parse record from file: {:?}", err);
                }
            };
        }

        debug!("Restored DHT from file");

        Ok(())
    }
}

/// Implement the `RecordStore` trait for `FileBackedStore`
impl<R: RecordStore> RecordStore for FileBackedStore<R> {
    type ProvidedIter<'a>
        = R::ProvidedIter<'a>
    where
        R: 'a;
    type RecordsIter<'a>
        = R::RecordsIter<'a>
    where
        R: 'a;

    // Delegate all `RecordStore` methods except `put` to the inner store
    delegate! {
        to self.underlying_store {
            fn add_provider(&mut self, record: libp2p::kad::ProviderRecord) -> libp2p::kad::store::Result<()>;
            fn get(&self, k: &libp2p::kad::RecordKey) -> Option<std::borrow::Cow<'_, libp2p::kad::Record>>;
            fn provided(&self) -> Self::ProvidedIter<'_>;
            fn providers(&self, key: &libp2p::kad::RecordKey) -> Vec<libp2p::kad::ProviderRecord>;
            fn records(&self) -> Self::RecordsIter<'_>;
            fn remove_provider(&mut self, k: &libp2p::kad::RecordKey, p: &libp2p::PeerId);
        }
    }

    /// Overwrite the `put` method to potentially save the record to a file
    fn put(&mut self, record: libp2p::kad::Record) -> Result<()> {
        // Try to write to the underlying store
        let result = self.underlying_store.put(record);

        // If the record was successfully written, update the record delta
        if result.is_ok() {
            self.record_delta += 1;

            // If the record delta is greater than the maximum record delta, try to save the file
            if self.record_delta > self.max_record_delta {
                if let Err(e) = self.save_to_file() {
                    warn!("Failed to save DHT to file: {:?}", e);
                }
            }
        }

        result
    }

    /// Overwrite the `remove` method to potentially remove the record from a file
    fn remove(&mut self, k: &libp2p::kad::RecordKey) {
        // Remove the record from the underlying store
        self.underlying_store.remove(k);

        // Update the record delta
        self.record_delta += 1;

        // If the record delta is greater than 10, try to save the file
        if self.record_delta > 10 {
            if let Err(e) = self.save_to_file() {
                warn!("Failed to save DHT to file: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use libp2p::{
        kad::{store::MemoryStore, RecordKey},
        PeerId,
    };
    use tracing_subscriber::EnvFilter;

    use super::*;

    #[test]
    fn test_save_and_restore() {
        // Try initializing tracing
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        // Create a test store
        let mut store = FileBackedStore::new(
            MemoryStore::new(PeerId::random()),
            "/tmp/test.dht".to_string(),
            10,
        );

        // The key is a random 16-byte array
        let key = RecordKey::new(&rand::random::<[u8; 16]>().to_vec());

        // The value is a random 16-byte array
        let random_value = rand::random::<[u8; 16]>();

        // Put a record into the store
        store
            .put(libp2p::kad::Record::new(key.clone(), random_value.to_vec()))
            .expect("Failed to put record into store");

        // Save the store to a file
        store.save_to_file().expect("Failed to save store to file");

        // Create a new store from the file
        let new_store = FileBackedStore::new(
            MemoryStore::new(PeerId::random()),
            "/tmp/test.dht".to_string(),
            10,
        );

        // Check that the new store has the record
        let restored_record = new_store
            .get(&key)
            .expect("Failed to get record from store");

        // Check that the restored record has the same value as the original record
        assert_eq!(restored_record.value, random_value.to_vec());
    }

    #[test]
    fn test_record_delta() {
        // Try initializing tracing
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        // Create a test store
        let mut store = FileBackedStore::new(
            MemoryStore::new(PeerId::random()),
            "/tmp/test.dht".to_string(),
            10,
        );

        let mut keys = Vec::new();
        let mut values = Vec::new();

        // Put 10 records into the store
        for _ in 0..10 {
            // Create a random key and value
            let key = RecordKey::new(&rand::random::<[u8; 16]>().to_vec());
            let value = rand::random::<[u8; 16]>();

            keys.push(key.clone());
            values.push(value);

            store
                .put(libp2p::kad::Record::new(key, value.to_vec()))
                .expect("Failed to put record into store");
        }

        // Create a new store from the allegedly unsaved file
        let new_store = FileBackedStore::new(
            MemoryStore::new(PeerId::random()),
            "/tmp/test.dht".to_string(),
            10,
        );

        // Check that the new store has none of the records
        for key in &keys {
            assert!(new_store.get(key).is_none());
        }

        // Store one more record into the new store
        store
            .put(libp2p::kad::Record::new(
                keys[0].clone(),
                values[0].to_vec(),
            ))
            .expect("Failed to put record into store");

        // Create a new store from the allegedly saved file
        let new_store = FileBackedStore::new(
            MemoryStore::new(PeerId::random()),
            "/tmp/test.dht".to_string(),
            10,
        );

        // Check that the new store has all of the records
        for (i, key) in keys.iter().enumerate() {
            let restored_record = new_store.get(key).expect("Failed to get record from store");
            assert_eq!(restored_record.value, values[i]);
        }

        // Check that the record delta is 0
        assert_eq!(new_store.record_delta, 0);
    }

    #[test]
    fn test_approximate_instant() {
        // Create an expiry time in the future
        let expiry_future = Instant::now() + Duration::from_secs(10);

        // Approximate the expiry time
        let approximate_expiry =
            unix_seconds_to_instant(instant_to_unix_seconds(expiry_future).unwrap())
                .unwrap()
                .duration_since(Instant::now());

        // Make sure it's close to 10 seconds in the future
        assert!(approximate_expiry >= Duration::from_secs(9));
        assert!(approximate_expiry <= Duration::from_secs(11));

        // Create an expiry time in the past
        let expiry_past = Instant::now().checked_sub(Duration::from_secs(10)).unwrap();

        // Approximate the expiry time
        let approximate_expiry =
            unix_seconds_to_instant(instant_to_unix_seconds(expiry_past).unwrap()).unwrap();
        let time_difference = approximate_expiry.elapsed();

        // Make sure it's close to 10 seconds in the past
        assert!(time_difference >= Duration::from_secs(9));
        assert!(time_difference <= Duration::from_secs(11));
    }
}
