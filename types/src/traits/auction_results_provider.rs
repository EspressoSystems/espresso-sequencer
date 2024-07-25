//! This module defines the interaction layer with the Solver via the [`AuctionResultsProvider`] trait,
//! which handles connecting to, and fetching the allocation results from, the Solver.

use anyhow::Result;
use async_compatibility_layer::art::async_timeout;
use async_trait::async_trait;
use futures::future::join_all;
use url::Url;

use super::node_implementation::NodeType;
use crate::{
    bundle::Bundle,
    constants::{AUCTION_RESULTS_FETCH_TIMEOUT, BUNDLE_FETCH_TIMEOUT},
};

/// This trait guarantees that a particular type has urls that can be extracted from it. This trait
/// essentially ensures that the results returned by the [`AuctionResultsProvider`] trait includes a
/// list of urls for the builders that HotShot must request from.
pub trait HasUrls {
    /// Returns the builer url associated with the datatype
    fn urls(&self) -> Vec<Url>;
}

/// The AuctionResultsProvider trait is the sole source of Solver-originated state and interaction,
/// and returns the results of the Solver's allocation via the associated type. The associated type,
/// `AuctionResult`, also implements the [`HasUrls`] trait, which requires that the output
/// type has the requisite fields available.
#[async_trait]
pub trait AuctionResultsProvider<TYPES: NodeType>: Send + Sync + Clone {
    /// The AuctionSolverResult is a type that holds the data associated with a particular solver
    /// run, for a particular view.
    type AuctionResult: HasUrls + Send;

    /// Fetches the auction result for a view. Does not cache the result,
    /// subsequent calls will invoke additional wasted calls.
    async fn fetch_auction_result(&self, view_number: TYPES::Time) -> Result<Self::AuctionResult>;

    /// Fetches the bundles for a view.
    async fn fetch_bundles(&self, view_number: TYPES::Time) -> Result<Vec<Bundle<TYPES>>> {
        let result = async_timeout(
            AUCTION_RESULTS_FETCH_TIMEOUT,
            self.fetch_auction_result(view_number),
        )
        .await??;

        let client = reqwest::Client::new();

        let mut futures = Vec::new();

        for url in result.urls() {
            futures.push(async_timeout(
                BUNDLE_FETCH_TIMEOUT,
                client.get(url).send().await?.json::<Bundle<TYPES>>(),
            ));
        }

        let mut bundles = Vec::new();

        for bundle in join_all(futures).await {
            match bundle {
                Ok(Ok(b)) => bundles.push(b),
                _ => continue,
            }
        }

        Ok(bundles)
    }
}
