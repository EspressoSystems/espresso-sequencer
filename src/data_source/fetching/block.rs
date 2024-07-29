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

//! [`Fetchable`] implementation for [`BlockQueryData`] and [`PayloadQueryData`].

use super::{
    header::{fetch_header_and_then, HeaderCallback},
    AvailabilityProvider, FetchRequest, Fetchable, Fetcher, Notifiers, NotifyStorage,
    RangedFetchable,
};
use crate::{
    availability::{
        BlockId, BlockQueryData, PayloadQueryData, QueryablePayload, UpdateAvailabilityData,
    },
    data_source::{storage::AvailabilityStorage, update::Transaction, VersionedDataSource},
    fetching::{
        self,
        request::{self, PayloadRequest},
        Callback,
    },
    types::HeightIndexed,
    Header, Payload, QueryResult,
};
use async_std::sync::Arc;
use async_trait::async_trait;
use derivative::Derivative;
use futures::future::{BoxFuture, FutureExt};
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::NodeType};
use std::{cmp::Ordering, future::IntoFuture, iter::once, ops::RangeBounds};

pub(super) type PayloadFetcher<Types, S, P> =
    fetching::Fetcher<request::PayloadRequest, PayloadCallback<Types, S, P>>;

impl<Types> FetchRequest for BlockId<Types>
where
    Types: NodeType,
{
    fn might_exist(self, block_height: usize, pruned_height: Option<usize>) -> bool {
        if let BlockId::Number(n) = self {
            n < block_height && pruned_height.map_or(true, |ph| n > ph)
        } else {
            true
        }
    }
}

#[async_trait]
impl<Types> Fetchable<Types> for BlockQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type Request = BlockId<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req {
            BlockId::Number(n) => self.height() == n as u64,
            BlockId::Hash(h) => self.hash() == h,
            BlockId::PayloadHash(h) => self.payload_hash() == h,
        }
    }

    async fn passive_fetch(
        notifiers: &Notifiers<Types>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>> {
        notifiers
            .block
            .wait_for(move |block| block.satisfies(req))
            .await
            .into_future()
            .boxed()
    }

    async fn active_fetch<S, P>(fetcher: Arc<Fetcher<Types, S, P>>, req: Self::Request)
    where
        S: AvailabilityStorage<Types> + VersionedDataSource + 'static,
        for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
        P: AvailabilityProvider<Types>,
    {
        fetch_header_and_then(
            &fetcher.storage,
            req,
            HeaderCallback::Payload {
                fetcher: fetcher.clone(),
            },
        )
        .await
    }

    async fn load<S>(storage: &NotifyStorage<Types, S>, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.as_ref().get_block(req).await
    }
}

#[async_trait]
impl<Types> RangedFetchable<Types> for BlockQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type RangedRequest = BlockId<Types>;

    async fn load_range<S, R>(
        storage: &NotifyStorage<Types, S>,
        range: R,
    ) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.as_ref().get_block_range(range).await
    }
}

pub(super) fn fetch_block_with_header<Types, S, P>(
    fetcher: Arc<Fetcher<Types, S, P>>,
    header: Header<Types>,
) where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
{
    // Now that we have the header, we only need to retrieve the payload.
    tracing::info!(
        "spawned active fetch for payload {:?} (height {})",
        header.payload_commitment(),
        header.block_number()
    );
    fetcher.payload_fetcher.spawn_fetch(
        PayloadRequest(header.payload_commitment()),
        fetcher.provider.clone(),
        once(PayloadCallback {
            header,
            fetcher: fetcher.clone(),
        }),
    );
}

async fn store_block<Types, S>(
    storage: &NotifyStorage<Types, S>,
    block: BlockQueryData<Types>,
) -> anyhow::Result<()>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: VersionedDataSource,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
{
    let mut tx = storage.transaction().await?;
    tx.insert_block(block).await?;
    tx.commit().await
}

#[async_trait]
impl<Types> Fetchable<Types> for PayloadQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type Request = BlockId<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req {
            BlockId::Number(n) => self.height() == n as u64,
            BlockId::Hash(h) => self.block_hash() == h,
            BlockId::PayloadHash(h) => self.hash() == h,
        }
    }

    async fn passive_fetch(
        notifiers: &Notifiers<Types>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>> {
        notifiers
            .block
            .wait_for(move |block| block.satisfies(req))
            .await
            .into_future()
            .map(|block| block.map(PayloadQueryData::from))
            .boxed()
    }

    async fn active_fetch<S, P>(fetcher: Arc<Fetcher<Types, S, P>>, req: Self::Request)
    where
        S: AvailabilityStorage<Types> + VersionedDataSource + 'static,
        for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
        P: AvailabilityProvider<Types>,
    {
        // We don't have storage for the payload alone, only the whole block. So if we need to fetch
        // the payload, we just fetch the whole block (which may end up fetching only the payload,
        // if that's all that's needed to complete the block).
        BlockQueryData::active_fetch(fetcher, req).await
    }

    async fn load<S>(storage: &NotifyStorage<Types, S>, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.as_ref().get_payload(req).await
    }
}

#[async_trait]
impl<Types> RangedFetchable<Types> for PayloadQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type RangedRequest = BlockId<Types>;

    async fn load_range<S, R>(
        storage: &NotifyStorage<Types, S>,
        range: R,
    ) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.as_ref().get_payload_range(range).await
    }
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub(super) struct PayloadCallback<Types: NodeType, S, P> {
    header: Header<Types>,
    #[derivative(Debug = "ignore")]
    fetcher: Arc<Fetcher<Types, S, P>>,
}

impl<Types: NodeType, S, P> PartialEq for PayloadCallback<Types, S, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<Types: NodeType, S, P> Eq for PayloadCallback<Types, S, P> {}

impl<Types: NodeType, S, P> Ord for PayloadCallback<Types, S, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.header.block_number().cmp(&other.header.block_number())
    }
}

impl<Types: NodeType, S, P> PartialOrd for PayloadCallback<Types, S, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Types: NodeType, S, P> Callback<Payload<Types>> for PayloadCallback<Types, S, P>
where
    Payload<Types>: QueryablePayload<Types>,
    S: 'static + VersionedDataSource,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
{
    async fn run(self, payload: Payload<Types>) {
        tracing::info!("fetched payload {:?}", self.header.payload_commitment());
        let block = BlockQueryData::new(self.header, payload);
        let height = block.height();

        // Store the block in local storage, so we can avoid fetching it in the future.
        if let Err(err) = store_block(&self.fetcher.storage, block).await {
            // It is unfortunate if this fails, but we can still proceed by returning the block that
            // we fetched, keeping it in memory. Simply log the error and move on.
            tracing::warn!("failed to store fetched block {height}: {err}");
        }
    }
}
