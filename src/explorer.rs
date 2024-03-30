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
pub(crate) mod traits;
use self::errors::InvalidLimit;
use crate::{api::load_api, Header};
pub use currency::*;
pub use data_source::*;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
pub use monetary_value::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::path::Path;
use tide_disco::StatusCode;
use tide_disco::{api::ApiError, method::ReadState, Api};
pub use traits::*;
use versioned_binary_serialization::version::StaticVersionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct BlockDetailResponse<Types: NodeType>
where
    Header<Types>: ExplorerHeader<Types>,
{
    pub block_detail: BlockDetail<Types>,
}

impl<Types: NodeType> From<BlockDetail<Types>> for BlockDetailResponse<Types>
where
    Header<Types>: ExplorerHeader<Types>,
{
    fn from(block_detail: BlockDetail<Types>) -> Self {
        Self { block_detail }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct BlockSummaryResponse<Types: NodeType>
where
    Header<Types>: ExplorerHeader<Types>,
{
    pub block_summaries: Vec<BlockSummary<Types>>,
}

impl<Types: NodeType> From<Vec<BlockSummary<Types>>> for BlockSummaryResponse<Types>
where
    Header<Types>: ExplorerHeader<Types>,
{
    fn from(block_summaries: Vec<BlockSummary<Types>>) -> Self {
        Self { block_summaries }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TransactionDetailResponse<Types: NodeType> {
    pub transaction_detail: data_source::TransactionDetailResponse<Types>,
}

impl<Types: NodeType> From<data_source::TransactionDetailResponse<Types>>
    for TransactionDetailResponse<Types>
{
    fn from(transaction_detail: data_source::TransactionDetailResponse<Types>) -> Self {
        Self { transaction_detail }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TransactionSummariesResponse<Types: NodeType>
where
    Header<Types>: ExplorerHeader<Types>,
{
    pub transaction_summaries: Vec<TransactionSummary<Types>>,
}

impl<Types: NodeType> From<Vec<TransactionSummary<Types>>> for TransactionSummariesResponse<Types>
where
    Header<Types>: ExplorerHeader<Types>,
{
    fn from(transaction_summaries: Vec<TransactionSummary<Types>>) -> Self {
        Self {
            transaction_summaries,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct ExplorerSummaryResponse<Types: NodeType>
where
    Header<Types>: ExplorerHeader<Types>,
{
    pub explorer_summary: ExplorerSummary<Types>,
}

impl<Types: NodeType> From<ExplorerSummary<Types>> for ExplorerSummaryResponse<Types>
where
    Header<Types>: ExplorerHeader<Types>,
{
    fn from(explorer_summary: ExplorerSummary<Types>) -> Self {
        Self { explorer_summary }
    }
}

impl std::error::Error for Error {}

fn validate_limit(
    limit: Result<usize, tide_disco::RequestError>,
) -> Result<NonZeroUsize, InvalidLimit> {
    let num_blocks = match limit {
        Ok(limit) => Ok(limit),
        _ => Err(InvalidLimit {}),
    }?;

    let num_blocks = match NonZeroUsize::new(num_blocks) {
        Some(num_blocks) => Ok(num_blocks),
        None => Err(InvalidLimit {}),
    }?;

    if num_blocks.get() > 100 {
        return Err(InvalidLimit {});
    }

    Ok(num_blocks)
}

pub fn define_api<State, Types: NodeType, Ver: StaticVersionType + 'static>(
    _: Ver,
) -> Result<Api<State, Error, Ver>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    Header<Types>: ExplorerHeader<Types>,
    <State as ReadState>::State: Send + Sync + ExplorerDataSource<Types>,
{
    let mut api = load_api::<State, Error, Ver>(
        Option::<Box<Path>>::None,
        include_str!("../api/explorer.toml"),
        None,
    )?;

    api.with_version("0.0.1".parse().unwrap())
        .get("get_block_detail", move |req, state| {
            async move {
                let target = match req.opt_integer_param::<str, usize>("height") {
                    Ok(Some(from)) => BlockIdentifier::Height(from),
                    _ => BlockIdentifier::Latest,
                };

                state
                    .get_block_detail(target)
                    .await
                    .map(BlockDetailResponse::from)
                    .map_err(Error::GetBlockDetail)
            }
            .boxed()
        })?
        .get("get_block_summaries", move |req, state| {
            async move {
                let num_blocks = validate_limit(req.integer_param("limit"))
                    .map_err(GetBlockSummariesError::InvalidLimit)
                    .map_err(Error::GetBlockSummaries)?;

                let target = match req.opt_integer_param::<str, usize>("from") {
                    Ok(Some(from)) => BlockIdentifier::Height(from),
                    _ => BlockIdentifier::Latest,
                };

                state
                    .get_block_summaries(GetBlockSummariesRequest(BlockRange {
                        target,
                        num_blocks,
                    }))
                    .await
                    .map(BlockSummaryResponse::from)
                    .map_err(Error::GetBlockSummaries)
            }
            .boxed()
        })?
        .get("get_transaction_detail", move |_req, state| {
            async move {
                state
                    .get_transaction_detail(TransactionIdentifier::Latest)
                    .await
                    .map(TransactionDetailResponse::from)
                    .map_err(Error::GetTransactionDetail)
            }
            .boxed()
        })?
        .get("get_transaction_summaries", move |req, state| {
            async move {
                let num_transactions = validate_limit(req.integer_param("limit"))
                    .map_err(GetTransactionSummariesError::InvalidLimit)
                    .map_err(Error::GetTransactionSummaries)?;

                let filter = match (
                    req.opt_integer_param("block"),
                    req.opt_integer_param("namespace"),
                ) {
                    (Ok(Some(block)), _) => TransactionSummaryFilter::Block(block),
                    (_, Ok(Some(namespace))) => TransactionSummaryFilter::RollUp(namespace),
                    _ => TransactionSummaryFilter::None,
                };

                let target = match (
                    req.opt_integer_param::<str, usize>("height"),
                    req.opt_integer_param::<str, usize>("offset"),
                    req.opt_blob_param("hash"),
                ) {
                    (Ok(Some(height)), Ok(Some(offset)), _) => {
                        TransactionIdentifier::HeightAndOffset(height, offset)
                    }
                    (_, _, Ok(Some(hash))) => TransactionIdentifier::Hash(hash),
                    _ => TransactionIdentifier::Latest,
                };

                state
                    .get_transaction_summaries(GetTransactionSummariesRequest {
                        range: TransactionRange {
                            target,
                            num_transactions,
                        },
                        filter,
                    })
                    .await
                    .map(TransactionSummariesResponse::from)
                    .map_err(Error::GetTransactionSummaries)
            }
            .boxed()
        })?
        .get("get_explorer_summary", move |_req, state| {
            async move {
                state
                    .get_explorer_summary()
                    .await
                    .map(ExplorerSummaryResponse::from)
                    .map_err(Error::GetExplorerSummary)
            }
            .boxed()
        })?;
    Ok(api)
}
