use async_compatibility_layer::{
    art::async_spawn,
    logging::{setup_backtrace, setup_logging},
};
use clap::Parser;
use hotshot::demo::SDemoTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use std::net::IpAddr;
use tracing::instrument;

use crate::types::{DANetwork, NodeImpl, QuorumNetwork, ThisMembership, ThisRun, ViewSyncNetwork};

pub mod types;

#[path = "../infra/mod.rs"]
pub mod infra;
#[path = "../infra/modDA.rs"]
pub mod infra_da;

#[derive(Parser, Debug, Clone)]
struct MultiValidatorArgs {
    /// Number of validators to run
    pub num_nodes: u16,
    /// The address the orchestrator runs on
    pub host: IpAddr,
    /// The port the orchestrator runs on
    pub port: u16,
    /// This node's public IP address, for libp2p
    /// If no IP address is passed in, it will default to 127.0.0.1
    pub public_ip: Option<IpAddr>,
}

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
    tracing::error!(
        "connecting to orchestrator at {:?}:{:?}",
        args.host,
        args.port
    );
    let mut nodes = Vec::new();
    for _ in 0..args.num_nodes {
        let node = async_spawn(async move {
            infra_da::main_entry_point::<
                SDemoTypes,
                ThisMembership,
                DANetwork,
                QuorumNetwork,
                ViewSyncNetwork,
                NodeImpl,
                ThisRun,
            >(ValidatorArgs {
                host: args.host.to_string(),
                port: args.port,
                public_ip: args.public_ip,
            })
            .await
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
