use std::sync::Arc;

use anyhow::Context;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{sync::RwLock, task::spawn};
use clap::Parser;
use espresso_types::{SeqTypes, SequencerVersions};
use hotshot_types::traits::node_implementation::{NodeType, Versions};
use marketplace_solver::{
    define_api, handle_events,
    state::{GlobalState, SolverState, StakeTable},
    EventsServiceClient, Options, SolverError,
};
use tide_disco::App;
use vbs::version::StaticVersionType;

type Version = <SequencerVersions as Versions>::Base;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let args = Options::parse();
    let Options {
        solver_api_port,
        events_api_url,
        database_options,
    } = args;

    let events_api_url = events_api_url.join("hotshot-events").unwrap();

    let events_client = EventsServiceClient::new(events_api_url.clone()).await;

    let startup_info = events_client
        .get_startup_info()
        .await
        .context("failed to get startup info ")?;

    let event_stream = events_client
        .get_event_stream()
        .await
        .context("failed to get event stream")?;

    let database = database_options
        .connect()
        .await
        .context("failed to connect to database")?;

    let solver_state = SolverState {
        stake_table: StakeTable {
            known_nodes_with_stake: startup_info.known_node_with_stake,
        },
        bid_txs: Default::default(),
    };

    let global_state = Arc::new(RwLock::new(GlobalState::new(database, solver_state)?));

    let event_handler = spawn(handle_events(event_stream, global_state.clone()));

    let mut app = App::<_, SolverError>::with_state(global_state);

    let mut api = define_api(Default::default())?;
    api.with_version(env!("CARGO_PKG_VERSION").parse()?);

    app.register_module::<SolverError, Version>("marketplace-solver", api)?;

    app.serve(format!("0.0.0.0:{}", solver_api_port), Version::instance())
        .await
        .unwrap();

    event_handler.cancel().await;

    Ok(())
}
