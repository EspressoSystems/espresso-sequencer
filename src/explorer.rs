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

pub(crate) mod currency;
pub(crate) mod data_source;
pub(crate) mod errors;
pub(crate) mod monetary_value;
use self::data_source::{
    BlockIdentifier, BlockRange, GetBlockDetailError, GetBlockSummariesError,
    GetExplorerSummaryError, GetTransactionDetailError, GetTransactionSummariesError,
    GetTransactionSummariesRequest, TransactionIdentifier, TransactionRange,
    TransactionSummaryFilter,
};
use self::data_source::{ExplorerDataSource, GetBlockSummariesRequest};
use crate::api::load_api;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::path::Path;
use tide_disco::StatusCode;
use tide_disco::{api::ApiError, method::ReadState, Api};
use versioned_binary_serialization::version::StaticVersionType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Error {
    #[serde(untagged)]
    GetBlockDetail(GetBlockDetailError),
    #[serde(untagged)]
    GetBlockSummaries(GetBlockSummariesError),
    #[serde(untagged)]
    GetTransactionDetail(GetTransactionDetailError),
    #[serde(untagged)]
    GetTransactionSummaries(GetTransactionSummariesError),
    #[serde(untagged)]
    GetExplorerSummary(GetExplorerSummaryError),
}

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Error::GetBlockDetail(e) => e.status(),
            Error::GetBlockSummaries(e) => e.status(),
            Error::GetTransactionDetail(e) => e.status(),
            Error::GetTransactionSummaries(e) => e.status(),
            Error::GetExplorerSummary(e) => e.status(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GetBlockDetail(e) => e.fmt(f),
            Error::GetBlockSummaries(e) => e.fmt(f),
            Error::GetTransactionDetail(e) => e.fmt(f),
            Error::GetTransactionSummaries(e) => e.fmt(f),
            Error::GetExplorerSummary(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub fn define_api<State, Types: NodeType, Ver: StaticVersionType + 'static>(
    _: Ver,
) -> Result<Api<State, Error, Ver>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + ExplorerDataSource<Types>,
{
    let mut api = load_api::<State, Error, Ver>(
        Option::<Box<Path>>::None,
        include_str!("../api/explorer.toml"),
        None,
    )?;

    api.with_version("0.0.1".parse().unwrap())
        .get("get_block_detail", move |_req, state| {
            async move {
                state
                    .get_block_detail(BlockIdentifier::Latest)
                    .await
                    .map_err(Error::GetBlockDetail)
            }
            .boxed()
        })?
        .get("get_block_summaries", move |_req, state| {
            async move {
                state
                    .get_block_summaries(GetBlockSummariesRequest(BlockRange {
                        target: BlockIdentifier::Latest,
                        num_blocks: NonZeroUsize::new(20).unwrap(),
                    }))
                    .await
                    .map_err(Error::GetBlockSummaries)
            }
            .boxed()
        })?
        .get("get_transaction_detail", move |_req, state| {
            async move {
                state
                    .get_transaction_detail(TransactionIdentifier::Latest)
                    .await
                    .map_err(Error::GetTransactionDetail)
            }
            .boxed()
        })?
        .get("get_transaction_summaries", move |_req, state| {
            async move {
                state
                    .get_transaction_summaries(GetTransactionSummariesRequest {
                        range: TransactionRange {
                            target: TransactionIdentifier::Latest,
                            num_transactions: NonZeroUsize::new(20).unwrap(),
                        },
                        filter: TransactionSummaryFilter::None,
                    })
                    .await
                    .map_err(Error::GetTransactionSummaries)
            }
            .boxed()
        })?
        .get("get_explorer_summary", move |_req, state| {
            async move {
                state
                    .get_explorer_summary()
                    .await
                    .map_err(Error::GetExplorerSummary)
            }
            .boxed()
        })?;
    Ok(api)
}
