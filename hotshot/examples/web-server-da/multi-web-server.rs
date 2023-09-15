use std::sync::Arc;

use async_compatibility_layer::{art::async_spawn, channel::oneshot};
use clap::Parser;
use hotshot::demo::SDemoTypes;
use tracing::error;

#[derive(Parser, Debug)]
struct MultiWebServerArgs {
    cdn_port: u16,
    da_port: u16,
    view_sync_port: u16,
}
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::main(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    let args = MultiWebServerArgs::parse();
    let (server_shutdown_sender_cdn, server_shutdown_cdn) = oneshot();
    let (server_shutdown_sender_da, server_shutdown_da) = oneshot();
    let (server_shutdown_sender_view_sync, server_shutdown_view_sync) = oneshot();
    let _sender = Arc::new(server_shutdown_sender_cdn);
    let _sender = Arc::new(server_shutdown_sender_da);
    let _sender = Arc::new(server_shutdown_sender_view_sync);

    let cdn_server = async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <SDemoTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(Some(server_shutdown_cdn), args.cdn_port)
        .await
        {
            error!("Problem starting cdn web server: {:?}", e);
        }
        error!("cdn");
    });
    let da_server = async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <SDemoTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(Some(server_shutdown_da), args.da_port)
        .await
        {
            error!("Problem starting da web server: {:?}", e);
        }
        error!("da");
    });
    let vs_server = async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <SDemoTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(Some(server_shutdown_view_sync), args.view_sync_port)
        .await
        {
            error!("Problem starting view sync web server: {:?}", e);
        }
        error!("vs");
    });
    let _result = futures::future::join_all(vec![cdn_server, da_server, vs_server]).await;
}
