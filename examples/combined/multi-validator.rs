// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! A multi-validator using both the web server libp2p
use async_compatibility_layer::{
    art::async_spawn,
    logging::{setup_backtrace, setup_logging},
};
use clap::Parser;
use hotshot_example_types::{node_types::TestVersions, state_types::TestTypes};
use hotshot_orchestrator::client::{MultiValidatorArgs, ValidatorArgs};
use tracing::instrument;

use crate::types::{Network, NodeImpl, ThisRun};

/// types used for this example
pub mod types;

/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();
    let args = MultiValidatorArgs::parse();
    tracing::debug!("connecting to orchestrator at {:?}", args.url);
    let mut nodes = Vec::new();
    for node_index in 0..args.num_nodes {
        let args = args.clone();

        let node = async_spawn(async move {
            infra::main_entry_point::<TestTypes, Network, NodeImpl, TestVersions, ThisRun>(
                ValidatorArgs::from_multi_args(args, node_index),
            )
            .await;
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
