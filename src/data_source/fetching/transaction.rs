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

//! Transaction fetching.

use super::{AvailabilityProvider, FetchRequest, Fetchable, Fetcher, NotifyStorage};
use crate::{
    availability::{QueryablePayload, TransactionHash, TransactionQueryData},
    data_source::storage::AvailabilityStorage,
    Payload, QueryResult,
};
use async_std::sync::{Arc, RwLockReadGuard};
use async_trait::async_trait;
use derive_more::From;
use futures::future::{BoxFuture, FutureExt};
use hotshot_types::traits::node_implementation::NodeType;

#[derive(Clone, Copy, Debug, From)]
pub(super) struct TransactionRequest<Types: NodeType>(TransactionHash<Types>);

impl<Types: NodeType> FetchRequest for TransactionRequest<Types> {}

#[async_trait]
impl<Types> Fetchable<Types> for TransactionQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    type Request = TransactionRequest<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        req.0 == self.hash()
    }

    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        // Any block might satisfy the request; we won't know until we receive a new payload and
        // check if it contains the desired transaction.
        let wait_block = storage
            .block_notifier
            .wait_for(move |block| block.transaction_by_hash(req.0).is_some())
            .await;

        async move {
            let block = wait_block.await?;
            Self::with_hash(&block, req.0)
        }
        .boxed()
    }

    async fn active_fetch<S, P>(
        _fetcher: Arc<Fetcher<Types, S, P>>,
        _storage: &RwLockReadGuard<'_, NotifyStorage<Types, S>>,
        req: Self::Request,
    ) where
        S: AvailabilityStorage<Types> + 'static,
        P: AvailabilityProvider<Types>,
    {
        // We don't actively fetch transactions, because without a satisfying block payload, we have
        // no way of knowing whether a block with such a transaction actually exists, and we don't
        // want to bother peers with requests for non-existant transactions.
        tracing::debug!("not fetching unknown transaction {req:?}");
    }

    async fn load<S>(storage: &NotifyStorage<Types, S>, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.storage.get_transaction(req.0).await
    }
}
