//! A orchestrator

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use hotshot_example_types::state_types::TestTypes;
use tracing::instrument;

use crate::infra::{read_orchestrator_init_config, run_orchestrator, OrchestratorArgs};

/// general infra used for this example
#[path = "./infra/mod.rs"]
pub mod infra;

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();
    let (config, orchestrator_url) = read_orchestrator_init_config::<TestTypes>();
    run_orchestrator::<TestTypes>(OrchestratorArgs::<TestTypes> {
        url: orchestrator_url.clone(),
        config: config.clone(),
    })
    .await;
}
