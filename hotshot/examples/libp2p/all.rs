#![allow(clippy::panic)]
//! A example program using libp2p
/// types used for this example
pub mod types;

use crate::infra::load_config_from_file;
use crate::types::ThisRun;
use async_compatibility_layer::art::async_spawn;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_orchestrator::config::NetworkConfig;
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeType;
use std::net::{IpAddr, Ipv4Addr};
use surf_disco::Url;
use tracing::instrument;

use crate::{
    infra::run_orchestrator,
    infra::{ConfigArgs, OrchestratorArgs},
    types::{DANetwork, NodeImpl, QuorumNetwork, VIDNetwork, ViewSyncNetwork},
};

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

    // use configfile args
    let args = ConfigArgs::parse();
    let orchestrator_url = Url::parse("http://localhost:4444").unwrap();

    // orchestrator
    async_spawn(run_orchestrator::<
        TestTypes,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        VIDNetwork,
        NodeImpl,
    >(OrchestratorArgs {
        url: orchestrator_url.clone(),
        config_file: args.config_file.clone(),
    }));

    // nodes
    let config: NetworkConfig<
        <TestTypes as NodeType>::SignatureKey,
        <TestTypes as NodeType>::ElectionConfigType,
    > = load_config_from_file::<TestTypes>(&args.config_file);
    let mut nodes = Vec::new();
    for _ in 0..config.config.total_nodes.into() {
        let orchestrator_url = orchestrator_url.clone();
        let node = async_spawn(async move {
            infra::main_entry_point::<
                TestTypes,
                DANetwork,
                QuorumNetwork,
                ViewSyncNetwork,
                VIDNetwork,
                NodeImpl,
                ThisRun,
            >(ValidatorArgs {
                url: orchestrator_url,
                public_ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                network_config_file: None,
            })
            .await;
        });
        nodes.push(node);
    }
    futures::future::join_all(nodes).await;
}
