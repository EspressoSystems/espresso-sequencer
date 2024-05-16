//! Sequencer-specific API options and initialization.

use super::{
    data_source::{
        provider, CatchupDataSource, SequencerDataSource, StateSignatureDataSource,
        SubmitDataSource,
    },
    endpoints, fs, sql,
    update::update_loop,
    ApiState, StorageState,
};
use crate::{
    context::{SequencerContext, TaskList},
    network,
    persistence::{self, SequencerPersistence},
    state::{update_state_storage_loop, BlockMerkleTree, FeeMerkleTree},
};
use anyhow::bail;
use async_std::sync::{Arc, RwLock};
use clap::Parser;
use futures::{
    channel::oneshot,
    future::{BoxFuture, FutureExt},
};
use hotshot_query_service::{
    data_source::{ExtensibleDataSource, MetricsDataSource},
    status::{self, UpdateStatusData},
    Error,
};
use hotshot_types::traits::metrics::{Metrics, NoMetrics};
use tide_disco::{
    method::{ReadState, WriteState},
    App, Url,
};
use vbs::version::StaticVersionType;

use hotshot_events_service::events::Error as EventStreamingError;

#[derive(Clone, Debug)]
pub struct Options {
    pub http: Http,
    pub query: Option<Query>,
    pub submit: Option<Submit>,
    pub status: Option<Status>,
    pub catchup: Option<Catchup>,
    pub config: Option<Config>,
    pub state: Option<State>,
    pub hotshot_events: Option<HotshotEvents>,
    pub explorer: Option<Explorer>,
    pub storage_fs: Option<persistence::fs::Options>,
    pub storage_sql: Option<persistence::sql::Options>,
}

impl From<Http> for Options {
    fn from(http: Http) -> Self {
        Self {
            http,
            query: None,
            submit: None,
            status: None,
            catchup: None,
            config: None,
            state: None,
            hotshot_events: None,
            explorer: None,
            storage_fs: None,
            storage_sql: None,
        }
    }
}

impl Options {
    /// Add a query API module backed by a Postgres database.
    pub fn query_sql(mut self, query: Query, storage: persistence::sql::Options) -> Self {
        self.query = Some(query);
        self.storage_sql = Some(storage);
        self
    }

    /// Add a query API module backed by the file system.
    pub fn query_fs(mut self, query: Query, storage: persistence::fs::Options) -> Self {
        self.query = Some(query);
        self.storage_fs = Some(storage);
        self
    }

    /// Add a submit API module.
    pub fn submit(mut self, opt: Submit) -> Self {
        self.submit = Some(opt);
        self
    }

    /// Add a status API module.
    pub fn status(mut self, opt: Status) -> Self {
        self.status = Some(opt);
        self
    }

    /// Add a catchup API module.
    pub fn catchup(mut self, opt: Catchup) -> Self {
        self.catchup = Some(opt);
        self
    }

    /// Add a config API module.
    pub fn config(mut self, opt: Config) -> Self {
        self.config = Some(opt);
        self
    }

    /// Add a state API module.
    pub fn state(mut self, opt: State) -> Self {
        self.state = Some(opt);
        self
    }

    /// Add a Hotshot events streaming API module.
    pub fn hotshot_events(mut self, opt: HotshotEvents) -> Self {
        self.hotshot_events = Some(opt);
        self
    }

    /// Add an explorer API module.
    pub fn explorer(mut self, opt: Explorer) -> Self {
        self.explorer = Some(opt);
        self
    }

    /// Whether these options will run the query API.
    pub fn has_query_module(&self) -> bool {
        self.query.is_some() && (self.storage_fs.is_some() || self.storage_sql.is_some())
    }

    /// Start the server.
    ///
    /// The function `init_context` is used to create a sequencer context from a metrics object and
    /// optional saved consensus state. The metrics object is created from the API data source, so
    /// that consensus will populuate metrics that can then be read and served by the API.
    pub async fn serve<N, P, F, Ver: StaticVersionType + 'static>(
        mut self,
        init_context: F,
        bind_version: Ver,
    ) -> anyhow::Result<SequencerContext<N, P, Ver>>
    where
        N: network::Type,
        P: SequencerPersistence,
        F: FnOnce(Box<dyn Metrics>) -> BoxFuture<'static, SequencerContext<N, P, Ver>>,
    {
        // Create a channel to send the context to the web server after it is initialized. This
        // allows the web server to start before initialization can complete, since initialization
        // can take a long time (and is dependent on other nodes).
        let (send_ctx, recv_ctx) = oneshot::channel();
        let state = ApiState::new(async move {
            recv_ctx
                .await
                .expect("context initialized and sent over channel")
        });
        let init_context = move |metrics| {
            let fut = init_context(metrics);
            async move {
                let ctx = fut.await;
                if send_ctx.send(super::ConsensusState::from(&ctx)).is_err() {
                    tracing::warn!("API server exited without receiving context");
                }
                ctx
            }
            .boxed()
        };
        let mut tasks = TaskList::default();

        // The server state type depends on whether we are running a query or status API or not, so
        // we handle the two cases differently.
        let metrics = if let Some(query_opt) = self.query.take() {
            if let Some(opt) = self.storage_sql.take() {
                self.init_with_query_module_sql::<N, P, Ver>(
                    query_opt,
                    opt,
                    state,
                    &mut tasks,
                    bind_version,
                )
                .await?
            } else if let Some(opt) = self.storage_fs.take() {
                self.init_with_query_module_fs::<N, P, Ver>(
                    query_opt,
                    opt,
                    state,
                    &mut tasks,
                    bind_version,
                )
                .await?
            } else {
                bail!("query module requested but not storage provided");
            }
        } else if self.status.is_some() {
            // If a status API is requested but no availability API, we use the `MetricsDataSource`,
            // which allows us to run the status API with no persistent storage.
            let ds = MetricsDataSource::default();
            let metrics = ds.populate_metrics();
            let mut app = App::<_, Error>::with_state(Arc::new(RwLock::new(
                ExtensibleDataSource::new(ds, state.clone()),
            )));

            // Initialize status API.
            let status_api = status::define_api(&Default::default(), bind_version)?;
            app.register_module("status", status_api)?;

            self.init_hotshot_modules::<_, _, _, Ver>(&mut app)?;

            if self.hotshot_events.is_some() {
                self.init_and_spawn_hotshot_event_streaming_module(
                    state,
                    &mut tasks,
                    bind_version,
                )?;
            }

            tasks.spawn(
                "API server",
                app.serve(format!("0.0.0.0:{}", self.http.port), bind_version),
            );

            metrics
        } else {
            // If no status or availability API is requested, we don't need metrics or a query
            // service data source. The only app state is the HotShot handle, which we use to submit
            // transactions.
            //
            // If we have no availability API, we cannot load a saved leaf from local storage, so we
            // better have been provided the leaf ahead of time if we want it at all.
            let mut app = App::<_, Error>::with_state(RwLock::new(state.clone()));

            self.init_hotshot_modules::<_, _, _, Ver>(&mut app)?;

            if self.hotshot_events.is_some() {
                self.init_and_spawn_hotshot_event_streaming_module(
                    state,
                    &mut tasks,
                    bind_version,
                )?;
            }

            tasks.spawn(
                "API server",
                app.serve(format!("0.0.0.0:{}", self.http.port), bind_version),
            );

            Box::new(NoMetrics)
        };

        Ok(init_context(metrics).await.with_task_list(tasks))
    }

    async fn init_app_modules<N, P, D, Ver: StaticVersionType + 'static>(
        &self,
        ds: D,
        state: ApiState<N, P, Ver>,
        tasks: &mut TaskList,
        bind_version: Ver,
    ) -> anyhow::Result<(
        Box<dyn Metrics>,
        Arc<RwLock<StorageState<N, P, D, Ver>>>,
        App<Arc<RwLock<StorageState<N, P, D, Ver>>>, Error>,
    )>
    where
        N: network::Type,
        P: SequencerPersistence,
        D: SequencerDataSource + CatchupDataSource + Send + Sync + 'static,
    {
        let metrics = ds.populate_metrics();
        let ds: endpoints::AvailState<N, P, D, Ver> =
            Arc::new(RwLock::new(ExtensibleDataSource::new(ds, state.clone())));
        let mut app = App::<_, Error>::with_state(ds.clone());

        // Initialize status API
        if self.status.is_some() {
            let status_api = status::define_api::<endpoints::AvailState<N, P, D, Ver>, Ver>(
                &Default::default(),
                bind_version,
            )?;
            app.register_module("status", status_api)?;
        }

        // Initialize availability and node APIs (these both use the same data source).
        app.register_module("availability", endpoints::availability(bind_version)?)?;
        app.register_module("node", endpoints::node(bind_version)?)?;

        self.init_hotshot_modules::<_, _, _, Ver>(&mut app)?;

        tasks.spawn(
            "query storage updater",
            update_loop(ds.clone(), state.event_stream()),
        );

        Ok((metrics, ds, app))
    }

    async fn init_with_query_module_fs<N, P, Ver: StaticVersionType + 'static>(
        &self,
        query_opt: Query,
        mod_opt: persistence::fs::Options,
        state: ApiState<N, P, Ver>,
        tasks: &mut TaskList,
        bind_version: Ver,
    ) -> anyhow::Result<Box<dyn Metrics>>
    where
        N: network::Type,
        P: SequencerPersistence,
    {
        let ds = <fs::DataSource as SequencerDataSource>::create(
            mod_opt,
            provider(query_opt.peers, bind_version),
            false,
        )
        .await?;

        let (metrics, _, app) = self
            .init_app_modules(ds, state.clone(), tasks, bind_version)
            .await?;

        if self.hotshot_events.is_some() {
            self.init_and_spawn_hotshot_event_streaming_module(state, tasks, bind_version)?;
        }

        tasks.spawn(
            "API server",
            app.serve(format!("0.0.0.0:{}", self.http.port), Ver::instance()),
        );
        Ok(metrics)
    }

    async fn init_with_query_module_sql<N, P, Ver: StaticVersionType + 'static>(
        self,
        query_opt: Query,
        mod_opt: persistence::sql::Options,
        state: ApiState<N, P, Ver>,
        tasks: &mut TaskList,
        bind_version: Ver,
    ) -> anyhow::Result<Box<dyn Metrics>>
    where
        N: network::Type,
        P: SequencerPersistence,
    {
        let ds = sql::DataSource::create(
            mod_opt.clone(),
            provider(query_opt.peers.clone(), bind_version),
            false,
        )
        .await?;
        let (metrics, ds, mut app) = self
            .init_app_modules(ds, state.clone(), tasks, bind_version)
            .await?;

        if self.explorer.is_some() {
            app.register_module("explorer", endpoints::explorer(bind_version)?)?;
        }

        if self.state.is_some() {
            // Initialize merklized state module for block merkle tree
            app.register_module(
                "block-state",
                endpoints::merklized_state::<N, P, _, BlockMerkleTree, _, 3>(bind_version)?,
            )?;
            // Initialize merklized state module for fee merkle tree
            app.register_module(
                "fee-state",
                endpoints::merklized_state::<N, P, _, FeeMerkleTree, _, 256>(bind_version)?,
            )?;

            let state = state.clone();
            let get_node_state = async move { state.node_state().await.clone() };
            tasks.spawn(
                "merklized state storage update loop",
                update_state_storage_loop(ds, get_node_state),
            );
        }

        if self.hotshot_events.is_some() {
            self.init_and_spawn_hotshot_event_streaming_module(state, tasks, bind_version)?;
        }

        if self.config.is_some() {
            app.register_module("config", endpoints::config(bind_version)?)?;
        }

        tasks.spawn(
            "API server",
            app.serve(format!("0.0.0.0:{}", self.http.port), Ver::instance()),
        );
        Ok(metrics)
    }

    /// Initialize the modules for interacting with HotShot.
    ///
    /// This function adds the `submit`, `state`, and `state_signature` API modules to the given
    /// app. These modules only require a HotShot handle as state, and thus they work with any data
    /// source, so initialization is the same no matter what mode the service is running in.
    fn init_hotshot_modules<N, P, S, Ver: StaticVersionType + 'static>(
        &self,
        app: &mut App<S, Error>,
    ) -> anyhow::Result<()>
    where
        S: 'static + Send + Sync + ReadState + WriteState,
        P: SequencerPersistence,
        S::State:
            Send + Sync + SubmitDataSource<N, P> + StateSignatureDataSource<N> + CatchupDataSource,
        N: network::Type,
    {
        let bind_version = Ver::instance();
        // Initialize submit API
        if self.submit.is_some() {
            let submit_api = endpoints::submit::<_, _, _, Ver>()?;
            app.register_module("submit", submit_api)?;
        }

        // Initialize state API.
        if self.catchup.is_some() {
            tracing::info!("initializing state API");
            let catchup_api = endpoints::catchup(bind_version)?;
            app.register_module("catchup", catchup_api)?;
        }

        let state_signature_api = endpoints::state_signature(bind_version)?;
        app.register_module("state-signature", state_signature_api)?;

        Ok(())
    }

    // Enable the events streaming api module
    fn init_and_spawn_hotshot_event_streaming_module<
        N,
        P: SequencerPersistence,
        Ver: StaticVersionType + 'static,
    >(
        &self,
        state: ApiState<N, P, Ver>,
        tasks: &mut TaskList,
        bind_version: Ver,
    ) -> anyhow::Result<()>
    where
        N: network::Type,
    {
        // Start the event streaming API server if it is enabled.
        // It runs to different port and app because State and Extensible Data source needs to support required
        // EventsSource trait, which is currently intended not to implement to separate hotshot-query-service crate, and
        // hotshot-events-service crate.

        let mut app = App::<_, EventStreamingError>::with_state(RwLock::new(state));

        tracing::info!("initializing hotshot events API");
        let hotshot_events_api = hotshot_events_service::events::define_api(
            &hotshot_events_service::events::Options::default(),
        )?;

        app.register_module::<_, Ver>("hotshot-events", hotshot_events_api)?;

        tasks.spawn(
            "Hotshot Events Streaming API server",
            app.serve(
                format!(
                    "0.0.0.0:{}",
                    self.hotshot_events.unwrap().events_service_port
                ),
                bind_version,
            ),
        );

        Ok(())
    }
}

/// The minimal HTTP API.
///
/// The API automatically includes health and version endpoints. Additional API modules can be
/// added by including the query-api or submit-api modules.
#[derive(Parser, Clone, Debug)]
pub struct Http {
    /// Port that the HTTP API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PORT")]
    pub port: u16,
}

/// Options for the submission API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct Submit;

/// Options for the status API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct Status;

/// Options for the catchup API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct Catchup;

/// Options for the config API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct Config;

/// Options for the query API module.
#[derive(Parser, Clone, Debug, Default)]
pub struct Query {
    /// Peers for fetching missing data for the query service.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PEERS")]
    pub peers: Vec<Url>,
}

/// Options for the state API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct State;

/// Options for the Hotshot events streaming API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct HotshotEvents {
    /// Port that the HTTP Hotshot Event streaming API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_STREAMING_API_PORT")]
    pub events_service_port: u16,
}

/// Options for the explorer API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct Explorer;
