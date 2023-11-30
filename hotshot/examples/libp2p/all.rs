pub mod types;

use crate::infra::load_config_from_file;
use crate::types::ThisRun;
use async_compatibility_layer::art::async_spawn;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot::demo::DemoTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_orchestrator::config::NetworkConfig;
use hotshot_types::traits::node_implementation::NodeType;
use std::net::{IpAddr, Ipv4Addr};
use tracing::instrument;

use crate::{
    infra::run_orchestrator,
    infra::{ConfigArgs, OrchestratorArgs},
    types::{DANetwork, NodeImpl, QuorumNetwork, VIDNetwork, ViewSyncNetwork},
};

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

    // orchestrator
    async_spawn(run_orchestrator::<
        DemoTypes,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        VIDNetwork,
        NodeImpl,
    >(OrchestratorArgs {
        url: "http://localhost".to_string(),
        port: 4444,
        config_file: args.config_file.clone(),
    }));

    // nodes
    let config: NetworkConfig<
        <DemoTypes as NodeType>::SignatureKey,
        <DemoTypes as NodeType>::ElectionConfigType,
    > = load_config_from_file::<DemoTypes>(args.config_file);
    let mut nodes = Vec::new();
    for _ in 0..config.config.total_nodes.into() {
        let node = async_spawn(async move {
            infra::main_entry_point::<
                DemoTypes,
                DANetwork,
                QuorumNetwork,
                ViewSyncNetwork,
                VIDNetwork,
                NodeImpl,
                ThisRun,
            >(ValidatorArgs {
                url: "http://localhost".to_string(),
                port: 4444,
                public_ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            })
            .await
        });
        nodes.push(node);
    }
    futures::future::join_all(nodes).await;
}
