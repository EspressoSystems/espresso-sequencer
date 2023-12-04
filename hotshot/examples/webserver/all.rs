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
use hotshot::demo::DemoTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_orchestrator::config::NetworkConfig;
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
            <DemoTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
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
            <DemoTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(
            Some(server_shutdown_da),
            Url::parse("http://localhost:9001").unwrap(),
        )
        .await
        {
            error!("Problem starting da web server: {:?}", e);
        }
    });

    // web server orchestrator
    async_spawn(run_orchestrator::<
        DemoTypes,
        DANetwork,
        QuorumNetwork,
        ViewSyncNetwork,
        VIDNetwork,
        NodeImpl,
    >(OrchestratorArgs {
        url: Url::parse("http://localhost:4444").unwrap(),
        config_file: args.config_file.clone(),
    }));

    // multi validator run
    let config: NetworkConfig<
        <DemoTypes as NodeType>::SignatureKey,
        <DemoTypes as NodeType>::ElectionConfigType,
    > = load_config_from_file::<DemoTypes>(args.config_file);
    let mut nodes = Vec::new();
    for _ in 0..(config.config.total_nodes.get()) {
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
    let _result = futures::future::join_all(nodes).await;
}
