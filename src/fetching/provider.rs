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

//! Remote data availability providers.

use super::Request;

/// A provider which is able to satisfy requests for data of type `T`.
#[trait_variant::make(Provider: Send)]
pub trait LocalProvider<Types, T: Request<Types>> {
    /// Fetch a resource.
    async fn fetch(&self, req: T) -> Option<T::Response>;
}

/// Trivial [`Provider`] where fetching always fails.
///
/// Useful for examples and tests which should never have need of a fetcher.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoFetching;

impl<Types, T: Send + Request<Types>> Provider<Types, T> for NoFetching {
    async fn fetch(&self, _req: T) -> Option<T::Response> {
        None
    }
}
