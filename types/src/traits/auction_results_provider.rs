//! This module defines the interaction layer with the Solver via the [`AuctionResultsProvider`] trait,
//! which handles connecting to, and fetching the allocation results from, the Solver.

use anyhow::Result;
use async_trait::async_trait;
use url::Url;

use super::node_implementation::NodeType;

/// This trait guarantees that a particular type has a url associated with it. This trait
/// essentially ensures that the results returned by the [`AuctionResultsProvider`] trait includes a URL
/// for the builder that HotShot must request from.
pub trait HasUrl {
    /// Returns the builer url associated with the datatype
    fn url(&self) -> Url;
}

/// The AuctionResultsProvider trait is the sole source of Solver-originated state and interaction,
/// and returns the results of the Solver's allocation via the associated type. The associated type,
/// `AuctionResult`, also implements the [`HasUrl`] trait, which requires that the output
/// type has the requisite fields available.
#[async_trait]
pub trait AuctionResultsProvider<TYPES: NodeType>: Send + Sync {
    /// The AuctionSolverResult is a type that holds the data associated with a particular solver
    /// run, for a particular view.
    type AuctionResult: HasUrl;

    /// Fetches the auction result for a view. Does not cache the result,
    /// subsequent calls will invoke additional wasted calls.
    async fn fetch_auction_result(
        &self,
        view_number: TYPES::Time,
    ) -> Result<Vec<Self::AuctionResult>>;
}
