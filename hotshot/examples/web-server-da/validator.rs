use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot::demo::SDemoTypes;
use tracing::{info, instrument};

use crate::types::{DANetwork, NodeImpl, QuorumNetwork, ThisMembership, ThisRun, ViewSyncNetwork};

use hotshot_orchestrator::client::ValidatorArgs;

pub mod types;

#[path = "../infra/mod.rs"]
pub mod infra;
#[path = "../infra/modDA.rs"]
pub mod infra_da;

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
        args.host, args.port
    );
    infra_da::main_entry_point::<
        SDemoTypes,
        ThisMembership,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        NodeImpl,
        ThisRun,
    >(args)
    .await;
}
