use anyhow::{bail, Result};
use async_trait::async_trait;
use hotshot_types::traits::{
    auction_results_provider::{AuctionResultsProvider, HasUrl},
    node_implementation::NodeType,
};
use url::Url;

/// A mock result for the auction solver. This type is just a pointer to a URL.
#[derive(Debug, Clone)]
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
#[derive(Debug, Default)]
pub struct TestAuctionResultsProvider {
    /// We intentionally allow for the results to be pre-cooked for the unit test to gurantee a
    /// particular outcome is met.
    pub solver_results: Vec<TestAuctionResult>,

    /// A canned type to ensure that an error is thrown in absence of a true fault-injectible
    /// system for logical tests. This will guarantee that `fetch_auction_result` always throws an
    /// error.
    pub should_return_err: bool,
}

#[async_trait]
impl<TYPES: NodeType> AuctionResultsProvider<TYPES> for TestAuctionResultsProvider {
    type AuctionResult = TestAuctionResult;

    /// Mock fetching the auction results, with optional error injection to simulate failure cases
    /// in the solver.
    async fn fetch_auction_result(
        &self,
        _view_number: TYPES::Time,
    ) -> Result<Vec<Self::AuctionResult>> {
        if self.should_return_err {
            bail!("Something went wrong")
        }

        // Otherwise, return our pre-made results
        Ok(self.solver_results.clone())
    }
}
