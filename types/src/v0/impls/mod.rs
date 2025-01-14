pub use super::*;

mod auction;
mod block;
mod chain_config;
mod fee_info;
mod header;
mod instance_state;
mod l1;
mod solver;
mod state;
mod transaction;

pub use auction::SolverAuctionResultsProvider;
pub use fee_info::{retain_accounts, FeeError};
pub use instance_state::NodeState;
pub use state::ProposalValidationError;
pub use state::{get_l1_deposits, BuilderValidationError, StateValidationError, ValidatedState};

#[cfg(any(test, feature = "testing"))]
pub use instance_state::mock;

/// A helper function to generate a random address for testing
#[inline(always)]
#[cfg(test)]
fn random_address() -> alloy::primitives::Address {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 20];
    rng.fill(&mut bytes);
    alloy::primitives::Address::from_slice(&bytes)
}
