pub mod types;

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot::demo::DemoMembership;
use hotshot::demo::DemoTypes;
use tracing::instrument;

use crate::infra::run_orchestrator;
use crate::infra::OrchestratorArgs;
use crate::types::{DANetwork, NodeImpl, QuorumNetwork, VIDNetwork, ViewSyncNetwork};

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

    run_orchestrator::<
        DemoTypes,
        DemoMembership,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        VIDNetwork,
        NodeImpl,
    >(args)
    .await;
}
