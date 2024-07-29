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

//! Header fetching.

use super::{
    block::fetch_block_with_header, leaf::fetch_leaf_with_callbacks,
    vid::fetch_vid_common_with_header, AvailabilityProvider, Fetcher, NotifyStorage, ResultExt,
};
use crate::{
    availability::{BlockId, QueryablePayload, UpdateAvailabilityData},
    data_source::{storage::AvailabilityStorage, update::VersionedDataSource},
    Header, Payload,
};
use anyhow::Context;
use async_std::sync::Arc;
use derivative::Derivative;
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::NodeType};
use std::cmp::Ordering;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub(super) enum HeaderCallback<Types, S, P>
where
    Types: NodeType,
{
    /// Callback when fetching the leaf in order to then look up the corresponding block.
    Payload {
        #[derivative(Debug = "ignore")]
        fetcher: Arc<Fetcher<Types, S, P>>,
    },
    /// Callback when fetching the leaf in order to then look up the corresopnding VID common data.
    VidCommon {
        #[derivative(Debug = "ignore")]
        fetcher: Arc<Fetcher<Types, S, P>>,
    },
}

impl<Types: NodeType, S, P> PartialEq for HeaderCallback<Types, S, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<Types: NodeType, S, P> Eq for HeaderCallback<Types, S, P> {}

impl<Types: NodeType, S, P> PartialOrd for HeaderCallback<Types, S, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Types: NodeType, S, P> Ord for HeaderCallback<Types, S, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Arbitrarily, we choose to run payload callbacks before VID callbacks.
        match (self, other) {
            (Self::Payload { .. }, Self::VidCommon { .. }) => Ordering::Less,
            (Self::VidCommon { .. }, Self::Payload { .. }) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl<Types, S, P> HeaderCallback<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: AvailabilityStorage<Types> + VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
{
    fn fetcher(&self) -> Arc<Fetcher<Types, S, P>> {
        match self {
            Self::Payload { fetcher } => fetcher.clone(),
            Self::VidCommon { fetcher } => fetcher.clone(),
        }
    }

    pub(super) fn run(self, header: Header<Types>) {
        match self {
            Self::Payload { fetcher } => {
                tracing::info!(
                    "fetched leaf {}, will now fetch payload",
                    header.block_number()
                );
                fetch_block_with_header(fetcher, header);
            }
            Self::VidCommon { fetcher } => {
                tracing::info!(
                    "fetched leaf {}, will now fetch VID common",
                    header.block_number()
                );
                fetch_vid_common_with_header(fetcher, header);
            }
        }
    }
}

pub(super) async fn fetch_header_and_then<Types, S, P>(
    storage: &NotifyStorage<Types, S>,
    req: BlockId<Types>,
    callback: HeaderCallback<Types, S, P>,
) where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    S: AvailabilityStorage<Types> + VersionedDataSource + 'static,
    for<'a> S::Transaction<'a>: UpdateAvailabilityData<Types>,
    P: AvailabilityProvider<Types>,
{
    // Check if at least the header is available in local storage. If it is, we benefit two ways:
    // 1. We know for sure the corresponding block exists, so we can unconditionally trigger an
    //    active fetch without unnecessarily bothering our peers.
    // 2. We only need to fetch the missing data (e.g. payload or VID common), not the full block.
    //    Not only is this marginally less data to download, there are some providers that may only
    //    be able to provide certain data. For example, the HotShot DA committee members may be able
    //    to provide paylaods, but not full blocks. Or, in the case where VID recovery is needed,
    //    the VID common data may be available but the full block may not exist anywhere.
    if let Some(header) = storage
        .as_ref()
        .get_header(req)
        .await
        .context(format!("loading header for block {req}"))
        .ok_or_trace()
    {
        callback.run(header);
        return;
    }

    // If the header is _not_ present, we may still be able to fetch the request, but we need to
    // fetch the header (in fact, the entire leaf) first. This is because we have an invariant that
    // we should not store derived objects in the database unless we already have the corresponding
    // header and leaf.
    match req {
        BlockId::Number(n) => {
            fetch_leaf_with_callbacks(callback.fetcher(), n.into(), [callback.into()]);
        }
        BlockId::Hash(h) => {
            // Given only the hash, we cannot tell if the corresonding leaf actually exists, since
            // we don't have a corresponding header. Therefore, we will not spawn an active fetch.
            tracing::debug!("not fetching unknown block {h}");
        }
        BlockId::PayloadHash(h) => {
            // Same as above, we don't fetch a block with a payload that is not known to exist.
            tracing::debug!("not fetching block with unknown payload {h}");
        }
    }
}
