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

use super::{
    errors::{ExplorerAPIError, Unimplemented},
    monetary_value::MonetaryValue,
};
use crate::availability::TransactionHash;
use crate::node::BlockHash;
use async_trait::async_trait;
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display},
    num::NonZeroUsize,
};
use tide_disco::StatusCode;
use time::format_description::well_known::Rfc3339;

/// BlockIdentifier is an enum that represents multiple ways of referring to
/// a specific Block.  These use cases are specific to a Block Explorer and
/// can be used to reference a specific individual block.
///
/// Any identifier specified here is not guaranteed to be valid, and may not
/// guarantee that a Block can actually be identified with the information
/// provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockIdentifier<Types: NodeType> {
    Latest,
    Height(usize),
    Hash(BlockHash<Types>),
}

impl<Types: NodeType> Display for BlockIdentifier<Types> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockIdentifier::Latest => write!(f, "latest"),
            BlockIdentifier::Height(height) => write!(f, "{}", height),
            BlockIdentifier::Hash(hash) => write!(f, "{}", hash),
        }
    }
}

/// TransactionIdentifier is an enum that represents multiple ways of of
/// identifying a specific Transaction.  These use cases are specific to a
/// Block Explorer and can be used to **ideally** uniquely identify a
/// `Transaction` within the Block Chain.
///
/// Any identified specified here is not guaranteed to actually point to a
/// transaction, and does not guarantee that a transaction with the specified
/// identification actually exists.
///
/// A TransactionHash is not actually guaranteed to point to a unique
/// transaction at the moment, however we will assume that it does for the
/// purposes of this API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionIdentifier<Types: NodeType> {
    Latest,
    HeightAndOffset(usize, usize),
    Hash(TransactionHash<Types>),
}

impl<Types: NodeType> Display for TransactionIdentifier<Types> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionIdentifier::Latest => write!(f, "latest"),
            TransactionIdentifier::HeightAndOffset(height, offset) => {
                write!(f, "{} {}", height, offset)
            }
            TransactionIdentifier::Hash(hash) => write!(f, "{}", hash),
        }
    }
}

/// BlockRange is a struct that represents a range for a specific set of
/// blocks, starting at the given `BlockIdentifier`.
///
/// This range is expected to be descending starting and the `target` and
/// descending toward `0`.
///
/// Given a stable and resolved Block Chain this should always refer to the
/// same set of blocks when the parameters themselves are the same.
///
/// If the `num_blocks` is not possible, then this should be considered as
/// referring to as many `Block`s as are possible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRange<Types: NodeType> {
    pub target: BlockIdentifier<Types>,
    pub num_blocks: NonZeroUsize,
}

/// TransactionRange is a struct that represents a range for a specific set of
/// transactions, starting at the given `TransactionIdentifier`.
///
/// This range is expected to be descending starting at the `target` and
/// descending toward the first transaction in the `Block Chain`.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionRange<Types: NodeType> {
    pub target: TransactionIdentifier<Types>,
    pub num_transactions: NonZeroUsize,
}

/// [Timestamp] represents a specific point in time that has a possible
/// offset.
///
/// This specific type is utilized in order to ensure that the timestamp is
/// always serialized in a specific format, specifically as string following
/// the RFC3339 standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub time::OffsetDateTime);

impl Serialize for Timestamp {
    /// serialize converts the timestamp into a string representation of a
    /// RFC3339 formatted date.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.format(&Rfc3339).unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    /// deserialize converts a string representation of a RFC3339 formatted
    /// date.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let dt = time::OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?;
        Ok(Timestamp(dt))
    }
}

/// [BlockDetail] is a struct that represents the details of a specific block
/// for use in a Block Explorer.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDetail {
    pub height: usize,
    pub time: Timestamp,
    pub num_transactions: usize,
    pub proposer_id: usize,
    pub fee_recipient: usize,
    pub size: usize,
    pub block_reward: Vec<MonetaryValue>,
}

/// [BlockSummary] is a struct that represents a summary overview of a specific
/// block.  It does not have all of the details of a [BlockDetail], but it is
/// useful for displaying information in a list of Blocks.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSummary {
    pub height: usize,
    pub proposer_id: usize,
    pub num_transactions: usize,
    pub size: usize,
    pub time: Timestamp,
}

/// [NamespaceId] is a specific type that represents an identified for a
/// given namespace.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamespaceId(u64);

/// [FeeAttribution] represents a specific attribution of fees for a specific
/// purpose.
///
/// The current documentation lists attribution as potentially being
/// accountable for the following entries:
/// - Sequencer
/// - DA Layer
/// - Ethereum Mainnet
#[derive(Debug, Serialize, Deserialize)]
pub struct FeeAttribution {
    pub target: String,
    pub fees: Vec<MonetaryValue>,
}

/// [TransactionDetail] is a struct that represents the details of a specific
/// transaction / payload contained within a Block.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub height: usize,
    pub block_confirmed: bool,
    pub index: usize,
    pub num_transactions: usize,
    pub size: usize,
    // pub hash: TransactionHash,
    pub time: Timestamp,
    // pub sender: ProposerID,
    pub sequencing_fees: Vec<MonetaryValue>,
    pub fee_details: Vec<FeeAttribution>,
}

/// [TransactionSummary] is a struct that represents a summary overview of a
/// specific transaction / payload contained within a Block. It does not have
/// all of the details of a [TransactionDetail], but it is useful for displaying
/// information in a list of Transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSummary {
    // pub hash: TransactionHash,
    pub rollups: Vec<NamespaceId>,
    pub height: usize,
    pub time: Timestamp,
}

/// GetBlockSummariesRequest is a struct that represents an incoming request
/// for Block Summaries.  This isn't sent on the line, but an endpoint will
/// be mapped to this struct in order for the request to be processed.
#[derive(Debug, PartialEq, Eq)]
pub struct GetBlockSummariesRequest<Types: NodeType>(pub BlockRange<Types>);

/// [TransactionSummaryFilter] represents the various filters that can be
/// applied when retrieving a list of [TransactionSummary] entries.
#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionSummaryFilter {
    None,
    RollUp(NamespaceId),
    Block(usize),
}

/// GetTransactionSummariesRequest is a struct that represents an incoming
/// request for Transaction Summaries.  This isn't sent on the line, but an
/// endpoint will be mapped to this struct in order for the request to be
/// processed.
#[derive(Debug)]
pub struct GetTransactionSummariesRequest<Types: NodeType> {
    pub range: TransactionRange<Types>,
    pub filter: TransactionSummaryFilter,
}

impl<Types: NodeType> Default for GetTransactionSummariesRequest<Types> {
    fn default() -> Self {
        Self {
            range: TransactionRange {
                target: TransactionIdentifier::Latest,
                num_transactions: NonZeroUsize::new(20).unwrap(),
            },
            filter: TransactionSummaryFilter::None,
        }
    }
}

/// [GetBlockDetailError] represents an error that has occurred in response to
/// the [GetBlockDetail] request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBlockDetailError {
    #[serde(untagged)]
    Unimplemented(Unimplemented),
    #[serde(untagged)]
    BlockNotFound(String),
}

impl GetBlockDetailError {
    pub fn status(&self) -> StatusCode {
        match self {
            GetBlockDetailError::Unimplemented(err) => err.status(),
            GetBlockDetailError::BlockNotFound(_) => StatusCode::NotFound,
        }
    }
}

impl Display for GetBlockDetailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBlockDetailError::Unimplemented(err) => write!(f, "{err}"),
            GetBlockDetailError::BlockNotFound(_) => write!(f, "block not found"),
        }
    }
}

impl ExplorerAPIError for GetBlockDetailError {
    fn code(&self) -> &str {
        match self {
            GetBlockDetailError::Unimplemented(err) => err.code(),
            GetBlockDetailError::BlockNotFound(_) => "BLOCK_NOT_FOUND",
        }
    }
}

/// [GetBlockSummariesError] represents an error that has occurred in response
/// to the [GetBlockSummaries] request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBlockSummariesError {
    #[serde(untagged)]
    Unimplemented(Unimplemented),
    #[serde(untagged)]
    TargetNotFound(String),
}

impl GetBlockSummariesError {
    pub fn status(&self) -> StatusCode {
        match self {
            GetBlockSummariesError::Unimplemented(err) => err.status(),
            GetBlockSummariesError::TargetNotFound(_) => StatusCode::NotFound,
        }
    }
}

impl Display for GetBlockSummariesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBlockSummariesError::Unimplemented(err) => write!(f, "{err}"),
            GetBlockSummariesError::TargetNotFound(identifier) => {
                write!(f, "target not found: {}", identifier)
            }
        }
    }
}

impl ExplorerAPIError for GetBlockSummariesError {
    fn code(&self) -> &str {
        match self {
            GetBlockSummariesError::Unimplemented(err) => err.code(),
            GetBlockSummariesError::TargetNotFound(_) => "TARGET_NOT_FOUND",
        }
    }
}

/// [GetTransactionDetailError] represents an error that has occurred in
/// response to the [GetTransactionDetail] request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTransactionDetailError {
    #[serde(untagged)]
    Unimplemented(Unimplemented),
    #[serde(untagged)]
    TransactionNotFound(String),
}

impl GetTransactionDetailError {
    pub fn status(&self) -> StatusCode {
        match self {
            GetTransactionDetailError::Unimplemented(err) => err.status(),
            GetTransactionDetailError::TransactionNotFound(_) => StatusCode::NotFound,
        }
    }
}

impl Display for GetTransactionDetailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetTransactionDetailError::Unimplemented(err) => write!(f, "{err}"),
            GetTransactionDetailError::TransactionNotFound(identifier) => {
                write!(f, "transaction not found: {}", identifier)
            }
        }
    }
}

impl ExplorerAPIError for GetTransactionDetailError {
    fn code(&self) -> &str {
        match self {
            GetTransactionDetailError::Unimplemented(err) => err.code(),
            GetTransactionDetailError::TransactionNotFound(_) => "TRANSACTION_NOT_FOUND",
        }
    }
}

/// [GetTransactionSummariesError] represents an error that has occurred in
/// response to the [GetTransactionSummaries] request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTransactionSummariesError {
    #[serde(untagged)]
    Unimplemented(Unimplemented),
    #[serde(untagged)]
    TargetNotFound(String),
}

impl GetTransactionSummariesError {
    pub fn status(&self) -> StatusCode {
        match self {
            GetTransactionSummariesError::Unimplemented(err) => err.status(),
            GetTransactionSummariesError::TargetNotFound(_) => StatusCode::NotFound,
        }
    }
}

impl Display for GetTransactionSummariesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetTransactionSummariesError::Unimplemented(err) => write!(f, "{err}"),
            GetTransactionSummariesError::TargetNotFound(identifier) => {
                write!(f, "target not found: {}", identifier)
            }
        }
    }
}

impl ExplorerAPIError for GetTransactionSummariesError {
    fn code(&self) -> &str {
        match self {
            GetTransactionSummariesError::Unimplemented(err) => err.code(),
            GetTransactionSummariesError::TargetNotFound(_) => "TARGET_NOT_FOUND",
        }
    }
}

/// [GetExplorerSummaryError] represents an error that has occurred in response
/// to the [GetExplorerSummary] request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetExplorerSummaryError {
    #[serde(untagged)]
    Unimplemented(Unimplemented),
}

impl GetExplorerSummaryError {
    pub fn status(&self) -> StatusCode {
        match self {
            GetExplorerSummaryError::Unimplemented(err) => err.status(),
        }
    }
}

impl Display for GetExplorerSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetExplorerSummaryError::Unimplemented(err) => write!(f, "{err}"),
        }
    }
}

impl ExplorerAPIError for GetExplorerSummaryError {
    fn code(&self) -> &str {
        match self {
            GetExplorerSummaryError::Unimplemented(err) => err.code(),
        }
    }
}

/// An interface for querying Data and Statistics from the HotShot Blockchain.
///
/// This interface provides methods that allows the enabling of querying data
/// concerning the blockchain from the stored data for use with a
/// block explorer.  It does not provide the same guarantees as the
/// Availability data source with data fetching.  It is not concerned with
/// being up-to-date or having all of the data required, but rather it is
/// concerned with providing the requested data as quickly as possible, and in
/// a way that can be easily cached.
#[async_trait]
pub trait ExplorerDataSource<Types: NodeType> {
    async fn get_block_detail(
        &self,
        request: BlockIdentifier<Types>,
    ) -> Result<BlockDetail, GetBlockDetailError> {
        let _ = request;
        Err(GetBlockDetailError::Unimplemented(Unimplemented {}))
    }

    async fn get_block_summaries(
        &self,
        request: GetBlockSummariesRequest<Types>,
    ) -> Result<Vec<BlockSummary>, GetBlockSummariesError> {
        let _ = request;
        Err(GetBlockSummariesError::Unimplemented(Unimplemented {}))
    }

    async fn get_transaction_detail(
        &self,
        request: TransactionIdentifier<Types>,
    ) -> Result<TransactionDetail, GetTransactionDetailError> {
        let _ = request;
        Err(GetTransactionDetailError::Unimplemented(Unimplemented {}))
    }

    async fn get_transaction_summaries(
        &self,
        request: GetTransactionSummariesRequest<Types>,
    ) -> Result<Vec<TransactionSummary>, GetTransactionSummariesError> {
        let _ = request;
        Err(GetTransactionSummariesError::Unimplemented(
            Unimplemented {},
        ))
    }

    async fn get_explorer_summary(&self) -> Result<(), GetExplorerSummaryError> {
        Err(GetExplorerSummaryError::Unimplemented(Unimplemented {}))
    }
}

#[async_trait]
pub trait UpdateExplorerData<Types: NodeType> {
    type Error: Error + Debug + Send + Sync + 'static;
}
