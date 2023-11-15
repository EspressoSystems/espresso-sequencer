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
use crate::status::UpdateStatusData;
use crate::{
    availability::{BlockQueryData, LeafQueryData, QueryableBlock, UpdateAvailabilityData},
    Block, Deltas, Leaf, Resolvable,
};
use async_trait::async_trait;
use hotshot::types::{Event, EventType};
use hotshot_types::{
    data::LeafType,
    traits::node_implementation::{NodeImplementation, NodeType},
};
use std::error::Error;
use std::fmt::Debug;
use std::iter::once;

/// An extension trait for types which implement the update trait for each API module.
///
/// If a type implements both [UpdateAvailabilityData] and [UpdateStatusData], then it can be fully
/// kept up to date through two interfaces:
/// * [populate_metrics](UpdateStatusData::populate_metrics), to get a handle for populating the
///   status metrics, which should be used when initializing a
///   [SystemContextHandle](hotshot::types::SystemContextHandle)
/// * [update](Self::update), to update the query state when a new HotShot event is emitted
#[async_trait]
pub trait UpdateDataSource<Types: NodeType, I: NodeImplementation<Types>>:
    UpdateAvailabilityData<Types, I> + UpdateStatusData
where
    Block<Types>: QueryableBlock,
{
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
    async fn update(&mut self, event: &Event<Types, Leaf<Types, I>>) -> Result<(), Self::Error>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>,
        Block<Types>: QueryableBlock;
}

#[async_trait]
impl<
        Types: NodeType,
        I: NodeImplementation<Types>,
        T: UpdateAvailabilityData<Types, I> + UpdateStatusData + Send,
    > UpdateDataSource<Types, I> for T
where
    Block<Types>: QueryableBlock,
{
    async fn update(&mut self, event: &Event<Types, Leaf<Types, I>>) -> Result<(), Self::Error>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>,
        Block<Types>: QueryableBlock,
    {
        if let EventType::Decide { leaf_chain, qc, .. } = &event.event {
            // `qc` justifies the first (most recent) leaf...
            let qcs = once((**qc).clone())
                // ...and each leaf in the chain justifies the subsequent leaf (its parent) through
                // `leaf.justify_qc`.
                .chain(leaf_chain.iter().map(|leaf| leaf.get_justify_qc()))
                // Put the QCs in chronological order.
                .rev()
                // The oldest QC is the `justify_qc` of the oldest leaf, which does not justify any
                // leaf in the new chain, so we don't need it.
                .skip(1);
            for (qc, leaf) in qcs.zip(leaf_chain.iter().rev()) {
                // The current version of HotShot has a guarantee that the block is available at the
                // moment the corresponding leaf is sequenced, so for the time being, we can get the
                // block by resolving the deltas and panicking if the block is not available. This
                // will change in the future, at which time this will have to be rewritten to spawn
                // a background task to resolve the block asynchronously and update the query data
                // when the block becomes available.
                let block = leaf
                    .get_deltas()
                    .try_resolve()
                    .expect("block was not available at moment leaf was sequenced");

                // `LeafQueryData::new` only fails if `qc` does not reference `leaf`. We have just
                // gotten `leaf` and `qc` directly from a consensus `Decide` event, so they are
                // guaranteed to correspond, and this should never panic.
                self.insert_leaf(
                    LeafQueryData::new(leaf.clone(), qc.clone()).expect("inconsistent leaf"),
                )
                .await?;
                // For the same reason, this will not panic either.
                self.insert_block(
                    BlockQueryData::new::<I>(leaf.clone(), qc.clone(), block)
                        .expect("inconsistent block"),
                )
                .await?;
            }
        }
        Ok(())
    }
}

/// A data source with an atomic transaction-based synchronization interface.
#[async_trait]
pub trait VersionedDataSource {
    type Error: Error + Debug;

    /// Atomically commit to all outstanding modifications to the data.
    ///
    /// If this method fails, outstanding changes are left unmodified. The caller may opt to retry
    /// or to erase outstanding changes with [`revert`](Self::revert).
    async fn commit(&mut self) -> Result<(), Self::Error>;

    /// Erase all oustanding modifications to the data.
    ///
    /// This function must not return if it has failed to revert changes. Inability to revert
    /// changes to the database is considered a fatal error, and this function may panic.
    async fn revert(&mut self);
}
