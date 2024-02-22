#![allow(unused_imports)]
use async_compatibility_layer::{art::async_spawn, channel::{unbounded, UnboundedReceiver}, logging::{setup_backtrace, setup_logging}};
use async_trait::async_trait;
use async_broadcast::broadcast;
use clap::Parser;
use futures::{future::FutureExt, stream::StreamExt};
use builder::{BuilderContext, init_node};
use hotshot_types::traits::{metrics::NoMetrics, node_implementation::NodeType};
use sequencer::{SeqTypes as BuilderType, NetworkParams, options::{Modules, Options}, network};
use hs_builder_core::{builder_state::MessageType, service::{run_standalone_builder_service, GlobalState}};
use std::sync::{Arc, Mutex};
use async_lock::RwLock;
use tide_disco::{method::ReadState, App, Url};


/// Construct a tide disco app that mocks the builder API.
 ///
 /// # Panics
 /// If constructing and launching the builder fails for any reason
 pub fn run_builder(url: Url) {
    let builder_api = hs_builder_api::builder::define_api::<GlobalState<BuilderType>, BuilderType>(
        &Options::default(),
    )
    .expect("Failed to construct the builder API");


    let (req_sender, req_receiver) = broadcast::<MessageType<BuilderType>>(usize::MAX);
    
    let (res_sender, res_receiver) = unbounded();

    // let global_state = Arc::new(RwLock::new(GlobalState::<BuilderType>::new(
    //     req_sender.clone(),
    //     res_receiver,
    // )));

    let mut app: App<GlobalState<BuilderType>, hs_builder_api::builder::Error> = App::with_state(GlobalState::<BuilderType>::new(
        req_sender.clone(),
        res_receiver,
    ));
    app.register_module("/", builder_api)
        .expect("Failed to register the builder API");

    async_spawn(app.serve(url));
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // setup logging and backtrace
    setup_logging();
    setup_backtrace();
    tracing::info!("Starting Builder from main.rs");
    // get options
    let opt = Options::parse(); 
    // call the init_node function and get the builder context
    
    // first create network params
    let network_params = NetworkParams {
        da_server_url: opt.da_server_url,
        consensus_server_url: opt.consensus_server_url,
        orchestrator_url: opt.orchestrator_url,
        state_relay_server_url: opt.state_relay_server_url,
        webserver_poll_interval: opt.webserver_poll_interval,
        private_staking_key: opt.private_staking_key,
    };


    let mut builder_context = init_node(network_params, &NoMetrics).await?;

    // start doing consensus i.e. in this case be passive member of the consensus network
    builder_context.start_consensus().await;

    // now run the standalone builder service defind in hs_builder_core/src/service.rs
    //run_standalone_builder_service(builder_context).await?;

    Ok(())
}