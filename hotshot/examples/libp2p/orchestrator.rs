pub mod types;

use clap::Parser;
use hotshot::demos::vdemo::VDemoTypes;
use tracing::instrument;
use types::ThisMembership;

use crate::infra::{run_orchestrator, OrchestratorArgs};
use crate::types::{NodeImpl, ThisNetwork};

#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::main(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    let args = OrchestratorArgs::parse();

    run_orchestrator::<VDemoTypes, ThisMembership, ThisNetwork, NodeImpl>(args).await;
}
