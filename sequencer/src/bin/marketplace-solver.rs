use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{sync::RwLock, task::spawn};
use clap::Parser;
use espresso_types::SeqTypes;
use hotshot_types::traits::node_implementation::NodeType;
use marketplace_solver::{
    define_api, handle_events,
    state::{GlobalState, SolverState, StakeTable},
    DatabaseOptions, EventsServiceClient, SolverError,
};
use tide_disco::App;
use url::Url;
use vbs::version::StaticVersionType;

type Version = <SeqTypes as NodeType>::Base;

#[derive(Parser)]
struct Args {
    /// Port to run the server on.
    #[clap(short, long, env = "ESPRESSO_MARKETPLACE_SOLVER_API_PORT")]
    solver_api_port: u16,

    /// Hotshot events service api URL
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_API_URL")]
    events_api_url: String,

    #[clap(flatten)]
    database_options: DatabaseOptions,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();
    let Args {
        solver_api_port,
        events_api_url,
        database_options,
    } = args;

    let events_api_url = Url::from_str(&format!("{events_api_url}/hotshot-events"))?;

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
