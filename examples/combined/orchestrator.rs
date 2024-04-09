//! Orchestrator using the web server
/// types used for this example
pub mod types;

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use hotshot_example_types::state_types::TestTypes;
use tracing::instrument;

use crate::{
    infra::{read_orchestrator_init_config, run_orchestrator, OrchestratorArgs},
    types::{DANetwork, NodeImpl, QuorumNetwork},
};
/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();
    let (config, orchestrator_url) = read_orchestrator_init_config::<TestTypes>();
    run_orchestrator::<TestTypes, DANetwork, QuorumNetwork, NodeImpl>(OrchestratorArgs::<
        TestTypes,
    > {
        url: orchestrator_url.clone(),
        config: config.clone(),
    })
    .await;
}
