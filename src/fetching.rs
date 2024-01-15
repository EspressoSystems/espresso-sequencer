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

//! Fetching missing data from remote providers.
//!
//! This module provides a mechanism to fetch data that is missing from this query service's storage
//! from a remote data availability provider. [`Fetcher`] can be used to handle concurrent requests
//! for data, ensuring that each distinct resource is only fetched once at a time.
//!
//! Fetching is ultimately dispatched to a [`Provider`], which implements fetching for a specific
//! type of resource from a specific source. The [`provider`] module contains built-in
//! implementations of [`Provider`] for various data availability sources.
//!

use async_std::sync::Mutex;
use derivative::Derivative;
use std::collections::{hash_map::Entry, BTreeSet, HashMap};

pub mod provider;
pub mod request;

pub use provider::Provider;
pub use request::Request;

/// A callback to process the result of a request.
///
/// Sometimes, we may fetch the same object for multiple purposes, so a request may have more than
/// one callback registered. For example, we may fetch a leaf for its own sake and also to
/// reconstruct a block. Or, we may fetch the same payload for two different blocks. In both of
/// these cases, there are two objects that must be processed and stored after the fetch completes.
///
/// In these cases, we only want one task to actually fetch the resource, but there may be several
/// unrelated actions to take after the resource is fetched. This trait allows us to identify a
/// callback, so that when the task that actually fetched the resource completes, it will run one
/// instance of each distinct callback which was registered. Callbacks will run in the order
/// determined by `Ord`.
#[trait_variant::make(Callback: Send)]
pub trait LocalCallback<T>: Ord {
    async fn run(self, response: T);
}

/// Management of concurrent requests to fetch resources.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct Fetcher<T, C> {
    #[derivative(Debug = "ignore")]
    in_progress: Mutex<HashMap<T, BTreeSet<C>>>,
}

impl<T, C> Fetcher<T, C> {
    /// Fetch a resource, if it is not already being fetched.
    ///
    /// Returns the requested resource if possible. Returns [`None`] if the resource is already
    /// being fetched, or if the given provider is unable to fetch it.
    pub async fn fetch<Types>(
        &self,
        req: T,
        provider: impl Provider<Types, T>,
        callbacks: impl IntoIterator<Item = C>,
    ) where
        T: Request<Types>,
        C: Callback<T::Response>,
    {
        // Check if the requested object is already being fetched. If not, take a lock on it so
        // we are the only ones to fetch this particular object.
        {
            let mut in_progress = self.in_progress.lock().await;
            match in_progress.entry(req) {
                Entry::Occupied(mut e) => {
                    // If the object is already being fetched, add our callback for the fetching
                    // task to execute upon completion.
                    e.get_mut().extend(callbacks);
                    tracing::info!("resource {req:?} is already being fetched");
                    return;
                }
                Entry::Vacant(e) => {
                    // If the object is not being fetched, we will register our own callback and
                    // then fetch it ourselves.
                    e.insert(callbacks.into_iter().collect());
                }
            }
        }

        // Now we are responsible for fetching the object, reach out to the provider.
        let res = provider.fetch(req).await;

        // Done fetching, remove our lock on the object and execute all callbacks.
        //
        // We will keep this lock the whole time we are running the callbacks. We can't release it
        // earlier because we can't allow another task to register a callback after we have taken
        // the list of callbacks that we will execute. We also don't want to allow any new fetches
        // until we have executed the callbacks, because one of the callbacks may store some
        // resource that obviates the need for another fetch.
        let mut in_progress = self.in_progress.lock().await;
        let callbacks = in_progress.remove(&req).unwrap_or_default();
        if let Some(res) = res {
            for callback in callbacks {
                callback.run(res.clone()).await;
            }
        }
    }
}
