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

//! [`Fetchable`] implementation for [`LeafQueryData`].

use super::{
    header::HeaderCallback, AvailabilityProvider, FetchRequest, Fetchable, Fetcher, Heights,
    Notifiers, NotifyStorage, RangedFetchable,
};
use crate::{
    availability::{LeafId, LeafQueryData, QueryablePayload, UpdateAvailabilityData},
    data_source::{storage::AvailabilityStorage, update::Transaction, VersionedDataSource},
    fetching::{self, request, Callback},
    types::HeightIndexed,
    Payload, QueryResult,
};
use async_std::sync::Arc;
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::From;
use futures::future::{BoxFuture, FutureExt};
use hotshot_types::traits::node_implementation::NodeType;
use std::{cmp::Ordering, future::IntoFuture, iter::once, ops::RangeBounds};

pub(super) type LeafFetcher<Types, S, P> =
    fetching::Fetcher<request::LeafRequest, LeafCallback<Types, S, P>>;

impl<Types> FetchRequest for LeafId<Types>
where
    Types: NodeType,
{
    fn might_exist(self, heights: Heights) -> bool {
        if let LeafId::Number(n) = self {
            heights.might_exist(n as u64)
        } else {
            true
        }
    }
}

#[async_trait]
impl<Types> Fetchable<Types> for LeafQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type Request = LeafId<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req {
            LeafId::Number(n) => self.height() == n as u64,
            LeafId::Hash(h) => self.hash() == h,
        }
    }

    async fn passive_fetch(
        notifiers: &Notifiers<Types>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>> {
        notifiers
            .leaf
            .wait_for(move |leaf| leaf.satisfies(req))
            .await
            .into_future()
            .boxed()
    }

    async fn active_fetch<S, P>(
        _tx: &mut impl AvailabilityStorage<Types>,
        fetcher: Arc<Fetcher<Types, S, P>>,
        req: Self::Request,
    ) where
        S: VersionedDataSource + 'static,
        for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
        P: AvailabilityProvider<Types>,
    {
        fetch_leaf_with_callbacks(fetcher, req, None)
    }

    async fn load<S>(storage: &mut S, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.get_leaf(req).await
    }
}

pub(super) fn fetch_leaf_with_callbacks<Types, S, P, I>(
    fetcher: Arc<Fetcher<Types, S, P>>,
    req: LeafId<Types>,
    callbacks: I,
) where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
    I: IntoIterator<Item = LeafCallback<Types, S, P>> + Send + 'static,
    I::IntoIter: Send,
{
    match req {
        LeafId::Number(n) => {
            let fetcher = fetcher.clone();
            fetcher.leaf_fetcher.clone().spawn_fetch(
                n.into(),
                fetcher.provider.clone(),
                once(LeafCallback::Leaf { fetcher }).chain(callbacks),
            );
        }
        LeafId::Hash(h) => {
            // We don't actively fetch leaves when requested by hash, because we have no way of
            // knowing whether a leaf with such a hash actually exists, and we don't want to bother
            // peers with requests for non-existant leaves.
            tracing::debug!("not fetching unknown leaf {h}");
        }
    }
}

async fn store_leaf<Types, S>(
    storage: &NotifyStorage<Types, S>,
    leaf: LeafQueryData<Types>,
) -> anyhow::Result<()>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
{
    let mut tx = storage.write().await?;
    tx.insert_leaf(leaf).await?;
    tx.commit().await?;
    Ok(())
}

#[async_trait]
impl<Types> RangedFetchable<Types> for LeafQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type RangedRequest = LeafId<Types>;

    async fn load_range<S, R>(storage: &mut S, range: R) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.get_leaf_range(range).await
    }
}

#[derive(Derivative, From)]
#[derivative(Debug(bound = ""))]
pub(super) enum LeafCallback<Types: NodeType, S, P> {
    /// Callback when fetching the leaf for its own sake.
    #[from(ignore)]
    Leaf {
        #[derivative(Debug = "ignore")]
        fetcher: Arc<Fetcher<Types, S, P>>,
    },
    /// Callback when fetching the leaf in order to then look up something else.
    Continuation {
        callback: HeaderCallback<Types, S, P>,
    },
}

impl<Types: NodeType, S, P> PartialEq for LeafCallback<Types, S, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<Types: NodeType, S, P> Eq for LeafCallback<Types, S, P> {}

impl<Types: NodeType, S, P> Ord for LeafCallback<Types, S, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Store leaves in the database before storing derived objecs.
            (Self::Leaf { .. }, Self::Continuation { .. }) => Ordering::Less,
            (Self::Continuation { .. }, Self::Leaf { .. }) => Ordering::Greater,

            (Self::Continuation { callback: cb1 }, Self::Continuation { callback: cb2 }) => {
                cb1.cmp(cb2)
            }
            _ => Ordering::Equal,
        }
    }
}

impl<Types: NodeType, S, P> PartialOrd for LeafCallback<Types, S, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Types: NodeType, S, P> Callback<LeafQueryData<Types>> for LeafCallback<Types, S, P>
where
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
{
    async fn run(self, leaf: LeafQueryData<Types>) {
        match self {
            Self::Leaf { fetcher } => {
                let height = leaf.height();
                tracing::info!("fetched leaf {height}");
                if let Err(err) = store_leaf(&fetcher.storage, leaf).await {
                    // It is unfortunate if this fails, but we can still proceed by
                    // returning the leaf that we fetched, keeping it in memory.
                    // Simply log the error and move on.
                    tracing::warn!("failed to store fetched leaf {height}: {err}");
                }
            }
            Self::Continuation { callback } => callback.run(leaf.leaf.block_header().clone()),
        }
    }
}
