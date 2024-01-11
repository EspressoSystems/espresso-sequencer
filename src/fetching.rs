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
use std::{collections::HashSet, fmt::Debug};

pub mod provider;
pub mod request;

pub use provider::Provider;
pub use request::Request;

/// Management of concurrent requests to fetch resources.
#[derive(Derivative)]
#[derivative(Debug(bound = "T: Debug"), Default(bound = ""))]
pub struct Fetcher<T> {
    in_progress: Mutex<HashSet<T>>,
}

impl<T> Fetcher<T> {
    /// Fetch a resource, if it is not already being fetched.
    ///
    /// Returns the requested resource if possible. Returns [`None`] if the resource is already
    /// being fetched, or if the given provider is unable to fetch it.
    pub async fn fetch<Types>(
        &self,
        req: T,
        provider: &impl Provider<Types, T>,
    ) -> Option<T::Response>
    where
        T: Request<Types>,
    {
        // Check if the requested object is already being fetched. If not, take a lock on it so
        // we are the only ones to fetch this particular object.
        {
            let mut in_progress = self.in_progress.lock().await;
            if !in_progress.insert(req) {
                return None;
            }
        }

        // Now we are responsible for fetching the object, reach out to the provider.
        let res = provider.fetch(req).await;
        // Done fetching, remove our lock on the object.
        self.in_progress.lock().await.remove(&req);
        res
    }
}
