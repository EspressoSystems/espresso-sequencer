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

//! A generic algorithm for updating a HotShot Query Service data source with new data.
use crate::availability::UpdateAvailabilityData;
use crate::status::UpdateStatusData;
use async_trait::async_trait;
use hotshot::types::Event;
use hotshot_types::traits::{metrics::Metrics, node_implementation::NodeTypes};

/// An extension trait for types which implement the update trait for each API module.
///
/// If a type implements both [UpdateAvailabilityData] and [UpdateStatusData], then it can be fully
/// kept up to date through two interfaces:
/// * [metrics](UpdateDataSource::metrics), to get a handle for populating the status metrics, which
///   should be used when initializing a [HotShotHandle](hotshot::types::HotShotHandle)
/// * [update](UpdateDataSource::update), to update the query state when a new HotShot event is
///   emitted
#[async_trait]
pub trait UpdateDataSource<Types: NodeTypes> {
    /// Get a handle for populating status metrics.
    ///
    /// This function should be called before creating a
    /// [HotShotHandle](hotshot::types::HotShotHandle), and the resulting [Metrics] handle should be
    /// passed to HotShot, which will update it.
    fn metrics(&self) -> Box<dyn Metrics>;

    /// Update query state based on a new consensus event.
    async fn update(&mut self, event: &Event<Types>);
}

#[async_trait]
impl<Types: NodeTypes, T: UpdateAvailabilityData<Types> + UpdateStatusData + Send>
    UpdateDataSource<Types> for T
{
    fn metrics(&self) -> Box<dyn Metrics> {
        UpdateStatusData::metrics(self)
    }

    async fn update(&mut self, _event: &Event<Types>) {
        todo!()
    }
}
