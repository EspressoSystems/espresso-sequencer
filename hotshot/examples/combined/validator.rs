use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_testing::state_types::TestTypes;
use tracing::{info, instrument};
use types::VIDNetwork;

use crate::types::{DANetwork, NodeImpl, QuorumNetwork, ThisRun, ViewSyncNetwork};

use hotshot_orchestrator::client::ValidatorArgs;

pub mod types;

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
    let args = ValidatorArgs::parse();
    info!(
        "connecting to orchestrator at {:?}:{:?}",
        args.url, args.port
    );
    infra::main_entry_point::<
        TestTypes,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        VIDNetwork,
        NodeImpl,
        ThisRun,
    >(args)
    .await;
}
