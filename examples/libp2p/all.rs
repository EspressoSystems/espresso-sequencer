//! A example program using libp2p
/// types used for this example
pub mod types;

use crate::infra::read_orchestrator_init_config;
use crate::infra::OrchestratorArgs;
use crate::types::ThisRun;
use crate::{
    infra::run_orchestrator,
    types::{DANetwork, NodeImpl, QuorumNetwork},
};
use async_compatibility_layer::art::async_spawn;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use hotshot_example_types::state_types::TestTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use tracing::instrument;

/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();

    // use configfile args
    let (config, orchestrator_url) = read_orchestrator_init_config::<TestTypes>();

    // orchestrator
    async_spawn(run_orchestrator::<
        TestTypes,
        DANetwork,
        QuorumNetwork,
        NodeImpl,
    >(OrchestratorArgs {
        url: orchestrator_url.clone(),
        config: config.clone(),
    }));

    // nodes
    let mut nodes = Vec::new();
    for i in 0..config.config.num_nodes_with_stake.into() {
        // Calculate our libp2p advertise address, which we will later derive the
        // bind address from for example purposes.
        let advertise_address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            8000 + (u16::try_from(i).expect("failed to create advertise address")),
        );
        let orchestrator_url = orchestrator_url.clone();
        let node = async_spawn(async move {
            infra::main_entry_point::<TestTypes, DANetwork, QuorumNetwork, NodeImpl, ThisRun>(
                ValidatorArgs {
                    url: orchestrator_url,
                    advertise_address: Some(advertise_address),
                    network_config_file: None,
                },
            )
            .await;
        });
        nodes.push(node);
    }
    futures::future::join_all(nodes).await;
}
