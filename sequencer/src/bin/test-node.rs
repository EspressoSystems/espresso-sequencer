use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::StreamExt as _;
use sequencer::{
    api::{options, test_helpers::TestNetwork, Options},
    catchup::mock::MockStateCatchup,
    persistence::no_storage::NoStorage,
    testing::TestConfig,
    ValidatedState,
};

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Port the API will server will listen on.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_API_PORT",
        default_value_t = 33000
    )]
    port: u16,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();
    let state = ValidatedState::default();
    // Prefund an arbitrary builder account.
    // TODO: why does it work without prefunding
    // state.prefund_account(Default::default(), 1000.into());
    let states = std::array::from_fn(|_| state.clone());

    // A temporary directory to store the state.
    let path = tempfile::tempdir().unwrap().path().into();
    // Start a sequencer network, using the query service for catchup.
    tracing::info!("Creating test network");
    let api_options = Options::from(options::Http { port: args.port })
        .status(Default::default())
        .state(Default::default())
        .submit(Default::default())
        .query_fs(
            Default::default(),
            sequencer::persistence::fs::Options { path },
        );
    let with_state = TestNetwork::with_state(
        api_options,
        states,
        [NoStorage; TestConfig::NUM_NODES],
        std::array::from_fn(|_| MockStateCatchup::default()),
    );
    let network = with_state.await;
    let mut events = network.server.get_event_stream();
    while let Some(event) = events.next().await {
        tracing::info!("Event: {:?}", event);
    }
}
