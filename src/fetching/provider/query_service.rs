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

use super::Provider;
use crate::{
    availability::{BlockQueryData, LeafQueryData},
    fetching::request::{BlockRequest, LeafRequest, PayloadRequest},
    Error, Payload,
};
use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::{Client, Url};

/// Data availability provider backed by another instance of this query service.
///
/// This fetcher implements the [`Provider`] interface by querying the REST API provided by another
/// instance of this query service to try and retrieve missing objects.
#[derive(Clone, Debug)]
pub struct QueryServiceFetcher {
    client: Client<Error>,
}

impl QueryServiceFetcher {
    pub async fn new(url: Url) -> Self {
        let client = Client::new(url);
        client.connect(None).await;
        Self { client }
    }
}

impl<Types> Provider<Types, BlockRequest> for QueryServiceFetcher
where
    Types: NodeType,
{
    async fn fetch(&self, req: BlockRequest) -> Option<BlockQueryData<Types>> {
        match self
            .client
            .get(&format!("availability/block/{}", usize::from(req)))
            .send()
            .await
        {
            Ok(block) => {
                // TODO we should also download a chain of QCs justifying the inclusion of `block`
                // in the chain at the requested height. However, HotShot currently lacks a good
                // light client API to verify this chain, so for now we just trust the other server.
                // https://github.com/EspressoSystems/HotShot/issues/2137
                // https://github.com/EspressoSystems/hotshot-query-service/issues/354
                Some(block)
            }
            Err(err) => {
                tracing::error!("failed to fetch block {req:?}: {err}");
                None
            }
        }
    }
}

impl<Types> Provider<Types, PayloadRequest> for QueryServiceFetcher
where
    Types: NodeType,
{
    async fn fetch(&self, req: PayloadRequest) -> Option<Payload<Types>> {
        match self
            .client
            .get(&format!("availability/payload/{}", req.0))
            .send()
            .await
        {
            Ok(payload) => {
                // TODO Verify that the data we retrieved is consistent with the request we made.
                // https://github.com/EspressoSystems/hotshot-query-service/issues/355
                Some(payload)
            }
            Err(err) => {
                tracing::error!("failed to fetch payload {req:?}: {err}");
                None
            }
        }
    }
}

impl<Types> Provider<Types, LeafRequest> for QueryServiceFetcher
where
    Types: NodeType,
{
    async fn fetch(&self, req: LeafRequest) -> Option<LeafQueryData<Types>> {
        match self
            .client
            .get(&format!("availability/leaf/{}", usize::from(req)))
            .send()
            .await
        {
            Ok(leaf) => {
                // TODO we should also download a chain of QCs justifying the inclusion of `leaf` in
                // the chain at the requested height. However, HotShot currently lacks a good light
                // client API to verify this chain, so for now we just trust the other server.
                // https://github.com/EspressoSystems/HotShot/issues/2137
                // https://github.com/EspressoSystems/hotshot-query-service/issues/354
                Some(leaf)
            }
            Err(err) => {
                tracing::error!("failed to fetch leaf {req:?}: {err}");
                None
            }
        }
    }
}
