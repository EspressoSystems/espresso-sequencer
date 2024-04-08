//! A example program using the web server
/// types used for this example
pub mod types;

use crate::infra::read_orchestrator_init_config;
use crate::infra::OrchestratorArgs;
use crate::types::ThisRun;
use crate::{
    infra::run_orchestrator,
    types::{DANetwork, NodeImpl, QuorumNetwork},
};
use std::sync::Arc;

/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

use async_compatibility_layer::{art::async_spawn, channel::oneshot};
use hotshot_example_types::state_types::TestTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_types::constants::WebServerVersion;
use surf_disco::Url;
use tracing::error;
use vbs::version::StaticVersionType;

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    setup_logging();
    setup_backtrace();

    let (config, orchestrator_url) = read_orchestrator_init_config::<TestTypes>();

    // spawn web servers
    let (server_shutdown_sender_cdn, server_shutdown_cdn) = oneshot();
    let (server_shutdown_sender_da, server_shutdown_da) = oneshot();
    let _sender = Arc::new(server_shutdown_sender_cdn);
    let _sender = Arc::new(server_shutdown_sender_da);

    async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
            WebServerVersion,
        >(
            Some(server_shutdown_cdn),
            Url::parse("http://localhost:9000").unwrap(),
            WebServerVersion::instance(),
        )
        .await
        {
            error!("Problem starting cdn web server: {:?}", e);
        }
    });
    async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
            WebServerVersion,
        >(
            Some(server_shutdown_da),
            Url::parse("http://localhost:9001").unwrap(),
            WebServerVersion::instance(),
        )
        .await
        {
            error!("Problem starting da web server: {:?}", e);
        }
    });

    // web server orchestrator
    async_spawn(run_orchestrator::<
        TestTypes,
        DANetwork,
        QuorumNetwork,
        NodeImpl,
    >(OrchestratorArgs::<TestTypes> {
        url: orchestrator_url.clone(),
        config: config.clone(),
    }));

    // multi validator run
    let mut nodes = Vec::new();
    for _ in 0..(config.config.num_nodes_with_stake.get()) {
        let orchestrator_url = orchestrator_url.clone();
        let node = async_spawn(async move {
            infra::main_entry_point::<TestTypes, DANetwork, QuorumNetwork, NodeImpl, ThisRun>(
                ValidatorArgs {
                    url: orchestrator_url,
                    advertise_address: None,
                    network_config_file: None,
                },
            )
            .await;
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
