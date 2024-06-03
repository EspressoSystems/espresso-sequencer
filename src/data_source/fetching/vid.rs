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

//! [`Fetchable`] implementation for [`VidCommonQueryData`].

use super::{
    header::{fetch_header_and_then, HeaderCallback},
    AvailabilityProvider, FetchRequest, Fetchable, Fetcher, NotifyStorage, RangedFetchable,
};
use crate::{
    availability::{BlockId, QueryablePayload, UpdateAvailabilityData, VidCommonQueryData},
    data_source::{storage::AvailabilityStorage, VersionedDataSource},
    fetching::{self, request, Callback},
    types::HeightIndexed,
    Header, Payload, QueryResult, VidCommon,
};
use async_std::sync::{Arc, RwLockReadGuard};
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::From;
use futures::future::{BoxFuture, FutureExt};
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::NodeType};
use std::{cmp::Ordering, future::IntoFuture, iter::once, ops::RangeBounds};

pub(super) type VidCommonFetcher<Types, S, P> =
    fetching::Fetcher<request::VidCommonRequest, VidCommonCallback<Types, S, P>>;

#[derive(Clone, Copy, Debug, From)]
pub(super) struct VidCommonRequest<Types: NodeType>(BlockId<Types>);

impl<Types: NodeType> From<usize> for VidCommonRequest<Types> {
    fn from(n: usize) -> Self {
        Self(n.into())
    }
}

impl<Types> FetchRequest for VidCommonRequest<Types>
where
    Types: NodeType,
{
    fn might_exist(self, block_height: usize, pruned_height: Option<usize>) -> bool {
        self.0.might_exist(block_height, pruned_height)
    }
}

#[async_trait]
impl<Types> Fetchable<Types> for VidCommonQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type Request = VidCommonRequest<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req.0 {
            BlockId::Number(n) => self.height() == n as u64,
            BlockId::Hash(h) => self.block_hash() == h,
            BlockId::PayloadHash(h) => self.payload_hash() == h,
        }
    }

    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        storage
            .vid_common_notifier
            .wait_for(move |vid| vid.satisfies(req))
            .await
            .into_future()
            .boxed()
    }

    async fn active_fetch<S, P>(
        fetcher: Arc<Fetcher<Types, S, P>>,
        storage: &RwLockReadGuard<'_, NotifyStorage<Types, S>>,
        req: Self::Request,
    ) where
        S: AvailabilityStorage<Types> + 'static,
        P: AvailabilityProvider<Types>,
    {
        fetch_header_and_then(storage, req.0, HeaderCallback::VidCommon { fetcher }).await
    }

    async fn load<S>(storage: &NotifyStorage<Types, S>, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.storage.get_vid_common(req.0).await
    }
}

#[async_trait]
impl<Types> RangedFetchable<Types> for VidCommonQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type RangedRequest = VidCommonRequest<Types>;

    async fn load_range<S, R>(
        storage: &NotifyStorage<Types, S>,
        range: R,
    ) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.storage.get_vid_common_range(range).await
    }
}

pub(super) fn fetch_vid_common_with_header<Types, S, P>(
    fetcher: Arc<Fetcher<Types, S, P>>,
    header: Header<Types>,
) where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: AvailabilityStorage<Types> + 'static,
    P: AvailabilityProvider<Types>,
{
    // Now that we have the header, we only need to retrieve the VID common data.
    tracing::info!(
        "spawned active fetch for VID common {:?} (height {})",
        header.payload_commitment(),
        header.block_number()
    );
    fetcher.vid_common_fetcher.spawn_fetch(
        request::VidCommonRequest(header.payload_commitment()),
        fetcher.provider.clone(),
        once(VidCommonCallback {
            header,
            fetcher: fetcher.clone(),
        }),
    );
}

async fn store_vid_common<Types, S>(
    storage: &mut NotifyStorage<Types, S>,
    common: VidCommonQueryData<Types>,
) -> anyhow::Result<()>
where
    Types: NodeType,
    S: UpdateAvailabilityData<Types> + VersionedDataSource,
{
    storage.insert_vid(common, None).await?;
    storage.commit().await?;
    Ok(())
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub(super) struct VidCommonCallback<Types: NodeType, S, P> {
    header: Header<Types>,
    #[derivative(Debug = "ignore")]
    fetcher: Arc<Fetcher<Types, S, P>>,
}

impl<Types: NodeType, S, P> PartialEq for VidCommonCallback<Types, S, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<Types: NodeType, S, P> Eq for VidCommonCallback<Types, S, P> {}

impl<Types: NodeType, S, P> Ord for VidCommonCallback<Types, S, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.header.block_number().cmp(&other.header.block_number())
    }
}

impl<Types: NodeType, S, P> PartialOrd for VidCommonCallback<Types, S, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Types: NodeType, S, P> Callback<VidCommon> for VidCommonCallback<Types, S, P>
where
    Payload<Types>: QueryablePayload<Types>,
    S: AvailabilityStorage<Types>,
    P: AvailabilityProvider<Types>,
{
    async fn run(self, common: VidCommon) {
        tracing::info!("fetched VID common {:?}", self.header.payload_commitment());
        let common = VidCommonQueryData::new(self.header, common);
        let height = common.height();

        // Store the data in local storage, so we can avoid fetching it in the future.
        {
            let mut storage = self.fetcher.storage.write().await;
            if let Err(err) = store_vid_common(&mut *storage, common).await {
                // Rollback the transaction if insert fails
                // This prevents subsequent queries from failing, as they would be part of the same transaction block.
                storage.revert().await;
                // It is unfortunate if this fails, but we can still proceed by returning
                // the block that we fetched, keeping it in memory. Simply log the error and
                // move on.
                tracing::warn!("failed to store fetched VID common {height}: {err}");
            }
        }
    }
}
