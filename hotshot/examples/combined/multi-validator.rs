//! A multi-validator using both the web server libp2p
use async_compatibility_layer::{
    art::async_spawn,
    logging::{setup_backtrace, setup_logging},
};
use clap::Parser;
use hotshot_orchestrator::client::{MultiValidatorArgs, ValidatorArgs};
use hotshot_testing::state_types::TestTypes;
use tracing::instrument;
use types::VIDNetwork;

use crate::types::{DANetwork, NodeImpl, QuorumNetwork, ThisRun, ViewSyncNetwork};

/// types used for this example
pub mod types;

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
    let args = MultiValidatorArgs::parse();
    tracing::error!("connecting to orchestrator at {:?}", args.url);
    let mut nodes = Vec::new();
    for node_index in 0..args.num_nodes {
        let args = args.clone();

        let node = async_spawn(async move {
            infra::main_entry_point::<
                TestTypes,
                DANetwork,
                QuorumNetwork,
                ViewSyncNetwork,
                VIDNetwork,
                NodeImpl,
                ThisRun,
            >(ValidatorArgs::from_multi_args(args, node_index))
            .await;
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
