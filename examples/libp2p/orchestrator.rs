//! An orchestrator using libp2p

/// types used for this example
pub mod types;

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_example_types::state_types::TestTypes;
use tracing::instrument;

use crate::infra::run_orchestrator;
use crate::infra::OrchestratorArgs;
use crate::types::{DANetwork, NodeImpl, QuorumNetwork};

/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::main(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();
    let args = OrchestratorArgs::parse();

    run_orchestrator::<TestTypes, DANetwork, QuorumNetwork, NodeImpl>(args).await;
}
