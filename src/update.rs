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
use crate::availability::{BlockQueryData, LeafQueryData, UpdateAvailabilityData};
use crate::status::UpdateStatusData;
use ark_serialize::CanonicalSerialize;
use hotshot::types::{Event, EventType};
use hotshot_types::traits::{metrics::Metrics, node_implementation::NodeTypes};
use std::error::Error;
use std::fmt::Debug;
use std::iter::once;

/// An extension trait for types which implement the update trait for each API module.
///
/// If a type implements both [UpdateAvailabilityData] and [UpdateStatusData], then it can be fully
/// kept up to date through two interfaces:
/// * [metrics](UpdateDataSource::metrics), to get a handle for populating the status metrics, which
///   should be used when initializing a [HotShotHandle](hotshot::types::HotShotHandle)
/// * [update](UpdateDataSource::update), to update the query state when a new HotShot event is
///   emitted
pub trait UpdateDataSource<Types: NodeTypes> {
    type Error: Error + Debug;

    /// Get a handle for populating status metrics.
    ///
    /// This function should be called before creating a
    /// [HotShotHandle](hotshot::types::HotShotHandle), and the resulting [Metrics] handle should be
    /// passed to HotShot, which will update it.
    fn metrics(&self) -> Box<dyn Metrics>;

    /// Update query state based on a new consensus event.
    ///
    /// The caller is responsible for authenticating `event`. This function does not perform any
    /// authentication, and if given an invalid `event` (one which does not follow from the latest
    /// known state of the ledger) it may panic or silently accept the invalid `event`. This allows
    /// the best possible performance in the case where the query service and the HotShot instance
    /// are running in the same process (and thus the event stream, directly from HotShot) is
    /// trusted.
    ///
    /// If you want to update the data source with an untrusted event, for example one received from
    /// a peer over the network, you must authenticate it first.
    fn update(&mut self, event: &Event<Types>) -> Result<(), Self::Error>;
}

impl<Types: NodeTypes, T: UpdateAvailabilityData<Types> + UpdateStatusData + Send>
    UpdateDataSource<Types> for T
where
    Types::BlockType: CanonicalSerialize,
{
    type Error = <Self as UpdateAvailabilityData<Types>>::Error;

    fn metrics(&self) -> Box<dyn Metrics> {
        UpdateStatusData::metrics(self)
    }

    fn update(&mut self, event: &Event<Types>) -> Result<(), Self::Error> {
        if let EventType::Decide { leaf_chain, qc } = &event.event {
            // `qc` justifies the first (most recent) leaf...
            let qcs = once(&**qc)
                // ...and each leaf in the chain justifies the subsequent leaf (its parent) through
                // `leaf.justify_qc`.
                .chain(leaf_chain.iter().map(|leaf| &leaf.justify_qc))
                // Put the QCs in chronological order.
                .rev()
                // The oldest QC is the `justify_qc` of the oldest leaf, which does not justify any
                // leaf in the new chain, so we don't need it.
                .skip(1);
            for (qc, leaf) in qcs.zip(leaf_chain.iter().rev()) {
                self.insert_leaf(LeafQueryData::new(leaf.clone(), qc.clone()))?;
                self.insert_block(BlockQueryData::new(leaf.clone(), qc.clone()))?;
            }
        }
        Ok(())
    }
}
