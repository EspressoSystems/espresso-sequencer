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

use atomic_store::{
    append_log, load_store::BincodeLoadStore, AppendLog, AtomicStoreLoader, PersistenceError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::VecDeque;
use std::fmt::Debug;
use tracing::warn;

/// A caching append log for ledger objects.
#[derive(Debug)]
pub(crate) struct LedgerLog<T: Serialize + DeserializeOwned> {
    cache_start: usize,
    cache_size: usize,
    cache: VecDeque<Option<T>>,
    store: AppendLog<BincodeLoadStore<Option<T>>>,
}

impl<T: Serialize + DeserializeOwned> LedgerLog<T> {
    pub(crate) fn create(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        cache_size: usize,
    ) -> Result<Self, PersistenceError> {
        Ok(Self {
            cache_start: 0,
            cache_size,
            cache: VecDeque::with_capacity(cache_size),
            store: AppendLog::create(
                loader,
                Default::default(),
                file_pattern,
                1u64 << 20, // 1 MB
            )?,
        })
    }

    pub(crate) fn open(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        cache_size: usize,
    ) -> Result<Self, PersistenceError> {
        let store = AppendLog::load(
            loader,
            Default::default(),
            file_pattern,
            1u64 << 20, // 1 MB
        )?;
        let len = store.iter().len();
        let cache_start = if len > cache_size {
            len - cache_size
        } else {
            // If the cache is large enough to contain every object in storage, we start it at index
            // 0 so that it does.
            0
        };
        let mut cache = store
            .iter()
            .skip(cache_start)
            .map(|r| {
                if let Err(e) = &r {
                    warn!("failed to load object. Error: {}", e);
                }
                // We treat missing objects and failed-to-load objects the same:
                // if we failed to load a object, it is now missing!
                r.ok().flatten()
            })
            .collect::<VecDeque<_>>();
        cache.reserve_exact(cache_size - cache.len());

        Ok(Self {
            cache_start,
            cache_size,
            cache,
            store,
        })
    }

    pub(crate) fn iter(&self) -> Iter<T> {
        Iter {
            index: 0,
            cache_start: self.cache_start,
            cache: &self.cache,
            store: self.store.iter(),
        }
    }

    pub(crate) fn store_resource(&mut self, resource: Option<T>) -> Result<(), PersistenceError> {
        self.store.store_resource(&resource)?;
        if self.cache.len() >= self.cache_size {
            self.cache.pop_front();
            self.cache_start += 1;
        }
        self.cache.push_back(resource);
        Ok(())
    }

    pub(crate) fn insert(&mut self, index: usize, object: T) -> Result<(), PersistenceError>
    where
        T: Debug,
    {
        // If there are missing objects between what we currently have and `object`, pad with
        // placeholders.
        while self.store.iter().len() < index {
            if let Err(err) = self.store_resource(None) {
                warn!("Failed to store placeholder: {}", err);
                return Err(err);
            }
        }
        let len = self.store.iter().len();
        assert!(len >= index);
        if len == index {
            // This is the next object in the chain, append it to the log.
            if let Err(err) = self.store_resource(Some(object)) {
                warn!("Failed to store object at index {}: {}", index, err);
                return Err(err);
            }
        } else {
            // This is an object earlier in the chain that we are now receiving asynchronously.
            // Update the placeholder with the actual contents of the object.
            // TODO update persistent storage once AppendLog supports updates.

            // Update the object in cache if necessary.
            if index >= self.cache_start {
                self.cache[index - self.cache_start] = Some(object);
            }
        }
        Ok(())
    }

    pub(crate) fn commit_version(&mut self) -> Result<(), PersistenceError> {
        self.store.commit_version()
    }

    pub(crate) fn skip_version(&mut self) -> Result<(), PersistenceError> {
        self.store.skip_version()
    }

    pub(crate) fn revert_version(&mut self) -> Result<(), PersistenceError> {
        self.store.revert_version()
    }
}

pub struct Iter<'a, T: Serialize + DeserializeOwned> {
    index: usize,
    cache_start: usize,
    cache: &'a VecDeque<Option<T>>,
    store: append_log::Iter<'a, BincodeLoadStore<Option<T>>>,
}

impl<'a, T: Serialize + DeserializeOwned + Clone> Iterator for Iter<'a, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // False positive: clippy suggests `self.next()` instead of `self.nth(0)`, but that would be
        // recursive.
        #[allow(clippy::iter_nth_zero)]
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.store.len();
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n;
        let res = if self.index >= self.cache_start {
            // Get the object from cache if we can.
            self.cache.get(self.index - self.cache_start).cloned()
        } else {
            // Otherwise load from storage.
            self.store.nth(n).map(|res| {
                if let Err(e) = &res {
                    warn!("failed to load object at position {}: error {}", n, e);
                }
                // Both a failed load and a successful load of `None` are treated the same: as
                // missing data, so we yield `None`. The latter case can happen if there was a
                // previous failed load and we marked this entry as explicitly missing.
                res.ok().flatten()
            })
        };

        self.index += 1;
        res
    }

    fn count(self) -> usize {
        self.store.len()
    }
}

impl<'a, T: Serialize + DeserializeOwned + Clone> ExactSizeIterator for Iter<'a, T> {}

#[cfg(test)]
mod test {
    use super::*;
    use atomic_store::AtomicStore;
    use tempdir::TempDir;

    #[test]
    fn test_ledger_log_creation() {
        let dir = TempDir::new("test_ledger_log").unwrap();

        // Create and populuate a log.
        {
            let mut loader = AtomicStoreLoader::create(dir.path(), "test_ledger_log").unwrap();
            let mut log = LedgerLog::<u64>::create(&mut loader, "ledger", 3).unwrap();
            let mut store = AtomicStore::open(loader).unwrap();
            for i in 0..5 {
                log.store_resource(Some(i)).unwrap();
                log.commit_version().unwrap();
                store.commit_version().unwrap();
            }
        }

        // Load the log from storage and check that we get the correct contents.
        {
            let mut loader = AtomicStoreLoader::load(dir.path(), "test_ledger_log").unwrap();
            let log = LedgerLog::<u64>::open(&mut loader, "ledger", 3).unwrap();
            AtomicStore::open(loader).unwrap();
            assert_eq!(
                log.iter().collect::<Vec<_>>(),
                (0..5).into_iter().map(Some).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_ledger_log_iter() {
        let dir = TempDir::new("test_ledger_log").unwrap();
        let mut loader = AtomicStoreLoader::create(dir.path(), "test_ledger_log").unwrap();
        let mut log = LedgerLog::<u64>::create(&mut loader, "ledger", 3).unwrap();
        let mut store = AtomicStore::open(loader).unwrap();
        for i in 0..5 {
            log.store_resource(Some(i)).unwrap();
            log.commit_version().unwrap();
            store.commit_version().unwrap();
        }

        assert_eq!(log.iter().len(), 5);
        for i in 0..5 {
            let mut iter = log.iter();
            assert_eq!(iter.nth(i as usize).unwrap(), Some(i), "{:?}", log);

            // `nth` should not only have returned the `n`th element, but also advanced the iterator.
            assert_eq!(
                iter.collect::<Vec<_>>(),
                (i + 1..5).into_iter().map(Some).collect::<Vec<_>>()
            );
        }
        assert_eq!(log.iter().nth(5), None);
    }
}
