pub mod types;

use crate::infra::load_config_from_file;
use crate::infra::{ConfigArgs, OrchestratorArgs};
use crate::types::ThisRun;
use crate::{
    infra::run_orchestrator,
    types::{DANetwork, NodeImpl, QuorumNetwork, ViewSyncNetwork},
};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

#[path = "../infra/mod.rs"]
pub mod infra;

use async_compatibility_layer::{art::async_spawn, channel::oneshot};
use clap::Parser;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_orchestrator::config::NetworkConfig;
use hotshot_testing::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::Url;
use tracing::error;
use types::VIDNetwork;

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    // use configfile args
    let args = ConfigArgs::parse();

    // spawn web servers
    let (server_shutdown_sender_cdn, server_shutdown_cdn) = oneshot();
    let (server_shutdown_sender_da, server_shutdown_da) = oneshot();
    let _sender = Arc::new(server_shutdown_sender_cdn);
    let _sender = Arc::new(server_shutdown_sender_da);

    async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(
            Some(server_shutdown_cdn),
            Url::parse("http://localhost:9000").unwrap(),
        )
        .await
        {
            error!("Problem starting cdn web server: {:?}", e);
        }
    });
    async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(
            Some(server_shutdown_da),
            Url::parse("http://localhost:9001").unwrap(),
        )
        .await
        {
            error!("Problem starting da web server: {:?}", e);
        }
    });

    let orchestrator_url = Url::parse("http://localhost:4444").unwrap();

    // web server orchestrator
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

    // multi validator run
    let config: NetworkConfig<
        <TestTypes as NodeType>::SignatureKey,
        <TestTypes as NodeType>::ElectionConfigType,
    > = load_config_from_file::<TestTypes>(args.config_file);
    let mut nodes = Vec::new();
    for _ in 0..(config.config.total_nodes.get()) {
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
            .await
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
