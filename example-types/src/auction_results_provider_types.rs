use anyhow::{bail, Result};
use async_trait::async_trait;
use hotshot_types::traits::{
    auction_results_provider::{AuctionResultsProvider, HasUrl},
    node_implementation::NodeType,
};
use serde::{Deserialize, Serialize};
use url::Url;

/// A mock result for the auction solver. This type is just a pointer to a URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAuctionResult {
    /// The URL of the builder to reach out to.
    pub url: Url,
}

impl HasUrl for TestAuctionResult {
    fn url(&self) -> Url {
        self.url.clone()
    }
}

/// The test auction results type is used to mimic the results from the Solver.
#[derive(Clone, Debug, Default)]
pub struct TestAuctionResultsProvider {
    /// We intentionally allow for the results to be pre-cooked for the unit test to gurantee a
    /// particular outcome is met.
    pub solver_results: Vec<TestAuctionResult>,

    /// A canned type to ensure that an error is thrown in absence of a true fault-injectible
    /// system for logical tests. This will guarantee that `fetch_auction_result` always throws an
    /// error.
    pub should_return_err: bool,

    /// The broadcast URL that the solver is running on. This type allows for the url to be
    /// optional, where `None` means to just return whatever `solver_results` contains, and `Some`
    /// means that we have a `FakeSolver` instance available to query.
    pub broadcast_url: Option<Url>,
}

#[async_trait]
impl<TYPES: NodeType> AuctionResultsProvider<TYPES> for TestAuctionResultsProvider {
    type AuctionResult = TestAuctionResult;

    /// Mock fetching the auction results, with optional error injection to simulate failure cases
    /// in the solver.
    async fn fetch_auction_result(
        &self,
        view_number: TYPES::Time,
    ) -> Result<Vec<Self::AuctionResult>> {
        if let Some(url) = &self.broadcast_url {
            let resp =
                reqwest::get(url.join(&format!("/v0/api/auction_results/{}", *view_number))?)
                    .await?
                    .json::<Vec<TestAuctionResult>>()
                    .await?;

            Ok(resp)
        } else {
            if self.should_return_err {
                bail!("Something went wrong")
            }

            // Otherwise, return our pre-made results
            Ok(self.solver_results.clone())
        }
    }
}
