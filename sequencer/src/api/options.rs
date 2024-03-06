//! Sequencer-specific API options and initialization.

use super::{
    data_source::{
        provider, SequencerDataSource, StateDataSource, StateSignatureDataSource, SubmitDataSource,
    },
    endpoints, fs, sql,
    update::update_loop,
};
use crate::{context::SequencerContext, network, persistence};
use anyhow::bail;
use async_std::sync::{Arc, RwLock};
use clap::Parser;
use futures::future::BoxFuture;
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
use versioned_binary_serialization::version::StaticVersion;

#[derive(Clone, Debug)]
pub struct Options {
    pub http: Http,
    pub query: Option<Query>,
    pub submit: Option<Submit>,
    pub status: Option<Status>,
    pub state: Option<State>,
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
            state: None,
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

    /// Add a state API module.
    pub fn state(mut self, opt: State) -> Self {
        self.state = Some(opt);
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
    pub async fn serve<N, F, const MAJOR_VERSION: u16, const MINOR_VERSION: u16>(
        mut self,
        init_context: F,
        bind_version: StaticVersion<MAJOR_VERSION, MINOR_VERSION>,
    ) -> anyhow::Result<SequencerContext<N, MAJOR_VERSION, MINOR_VERSION>>
    where
        N: network::Type,
        F: FnOnce(
            Box<dyn Metrics>,
        ) -> BoxFuture<'static, SequencerContext<N, MAJOR_VERSION, MINOR_VERSION>>,
    {
        // The server state type depends on whether we are running a query or status API or not, so
        // we handle the two cases differently.
        if let Some(query_opt) = self.query.take() {
            if let Some(opt) = self.storage_sql.take() {
                self.init_with_query_module::<N, sql::DataSource, MAJOR_VERSION, MINOR_VERSION>(
                    query_opt,
                    opt,
                    init_context,
                    bind_version,
                )
                .await
            } else if let Some(opt) = self.storage_fs.take() {
                self.init_with_query_module::<N, fs::DataSource, MAJOR_VERSION, MINOR_VERSION>(
                    query_opt,
                    opt,
                    init_context,
                    bind_version,
                )
                .await
            } else {
                bail!("query module requested but not storage provided");
            }
        } else if self.status.is_some() {
            // If a status API is requested but no availability API, we use the `MetricsDataSource`,
            // which allows us to run the status API with no persistent storage.
            let ds = MetricsDataSource::default();
            let mut context = init_context(ds.populate_metrics()).await;
            let mut app = App::<_, Error, MAJOR_VERSION, MINOR_VERSION>::with_state(Arc::new(
                RwLock::new(ExtensibleDataSource::new(ds, super::State::from(&context))),
            ));

            // Initialize status API.
            let status_api = status::define_api(&Default::default(), bind_version)?;
            app.register_module("status", status_api)?;

            self.init_hotshot_modules(&mut app)?;
            context.spawn(
                "API server",
                app.serve(format!("0.0.0.0:{}", self.http.port)),
            );
            Ok(context)
        } else {
            // If no status or availability API is requested, we don't need metrics or a query
            // service data source. The only app state is the HotShot handle, which we use to submit
            // transactions.
            //
            // If we have no availability API, we cannot load a saved leaf from local storage, so we
            // better have been provided the leaf ahead of time if we want it at all.
            let mut context = init_context(Box::new(NoMetrics)).await;
            let mut app = App::<_, Error, MAJOR_VERSION, MINOR_VERSION>::with_state(RwLock::new(
                super::State::from(&context),
            ));

            self.init_hotshot_modules(&mut app)?;
            context.spawn(
                "API server",
                app.serve(format!("0.0.0.0:{}", self.http.port)),
            );
            Ok(context)
        }
    }

    async fn init_with_query_module<N, D, const MAJOR_VERSION: u16, const MINOR_VERSION: u16>(
        self,
        query_opt: Query,
        mod_opt: D::Options,
        init_context: impl FnOnce(
            Box<dyn Metrics>,
        ) -> BoxFuture<
            'static,
            SequencerContext<N, MAJOR_VERSION, MINOR_VERSION>,
        >,
        bind_version: StaticVersion<MAJOR_VERSION, MINOR_VERSION>,
    ) -> anyhow::Result<SequencerContext<N, MAJOR_VERSION, MINOR_VERSION>>
    where
        N: network::Type,
        D: SequencerDataSource + Send + Sync + 'static,
    {
        let ds = D::create(mod_opt, provider(query_opt.peers, bind_version), false).await?;
        let metrics = ds.populate_metrics();

        // Start up handle
        let mut context = init_context(metrics).await;

        // Get an event stream from the handle to use for populating the query data with
        // consensus events.
        //
        // We must do this _before_ starting consensus on the handle, otherwise we could miss
        // the first events emitted by consensus.
        let events = context.get_event_stream();

        let state: endpoints::AvailState<N, D, MAJOR_VERSION, MINOR_VERSION> = Arc::new(
            RwLock::new(ExtensibleDataSource::new(ds, (&context).into())),
        );
        let mut app = App::<_, Error, MAJOR_VERSION, MINOR_VERSION>::with_state(state.clone());

        // Initialize status API
        if self.status.is_some() {
            let status_api = status::define_api::<
                endpoints::AvailState<N, D, MAJOR_VERSION, MINOR_VERSION>,
                MAJOR_VERSION,
                MINOR_VERSION,
            >(&Default::default(), bind_version)?;
            app.register_module("status", status_api)?;
        }

        // Initialize availability and node APIs (these both use the same data source).
        app.register_module("availability", endpoints::availability(bind_version)?)?;
        app.register_module("node", endpoints::node(bind_version)?)?;

        self.init_hotshot_modules(&mut app)?;
        context.spawn("query storage updater", update_loop(state, events));
        context.spawn(
            "API server",
            app.serve(format!("0.0.0.0:{}", self.http.port)),
        );

        Ok(context)
    }

    /// Initialize the modules for interacting with HotShot.
    ///
    /// This function adds the `submit`, `state`, and `state_signature` API modules to the given
    /// app. These modules only require a HotShot handle as state, and thus they work with any data
    /// source, so initialization is the same no matter what mode the service is running in.
    fn init_hotshot_modules<N, S, const MAJOR_VERSION: u16, const MINOR_VERSION: u16>(
        &self,
        app: &mut App<S, Error, MAJOR_VERSION, MINOR_VERSION>,
    ) -> anyhow::Result<()>
    where
        S: 'static + Send + Sync + ReadState + WriteState,
        S::State: Send + Sync + SubmitDataSource<N> + StateSignatureDataSource<N> + StateDataSource,
        N: network::Type,
    {
        let bind_version: StaticVersion<MAJOR_VERSION, MINOR_VERSION> = StaticVersion {};
        // Initialize submit API
        if self.submit.is_some() {
            let submit_api = endpoints::submit()?;
            app.register_module("submit", submit_api)?;
        }

        // Initialize state API.
        if self.state.is_some() {
            tracing::info!("initializing state API");
            let state_api = endpoints::state(bind_version)?;
            app.register_module("state", state_api)?;
        }

        let state_signature_api = endpoints::state_signature(bind_version)?;
        app.register_module("state-signature", state_signature_api)?;

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

/// Options for the state API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct State;

/// Options for the query API module.
#[derive(Parser, Clone, Debug, Default)]
pub struct Query {
    /// Peers for fetching missing data for the query service.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PEERS")]
    pub peers: Vec<Url>,
}
