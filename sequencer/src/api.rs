use crate::{transaction::Transaction, Leaf, NMTRoot, NamespaceProofType, SeqTypes};
use async_std::{
    sync::RwLock,
    task::{spawn, JoinHandle},
};
use clap::Parser;
use futures::{future::BoxFuture, FutureExt};
use hotshot::types::SystemContextHandle;
use hotshot::{traits::NodeImplementation, types::Event};
use hotshot_query_service::{
    availability::{
        self, AvailabilityDataSource, Error as AvailabilityError, Options as AvailabilityOptions,
    },
    data_source::{QueryData, UpdateDataSource},
    status::{self, StatusDataSource},
    Error,
};
use hotshot_types::traits::metrics::{Metrics, NoMetrics};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tide_disco::{Api, App};

/// The minimal HTTP API.
///
/// The API automatically includes health and version endpoints. Additional API modules can be
/// added by including the query-api or submit-api modules.
#[derive(Parser, Clone, Debug)]
pub struct HttpOptions {
    /// Port that the HTTP API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PORT")]
    pub port: u16,
}

/// Options for the query API module.
#[derive(Parser, Clone, Debug)]
pub struct QueryOptions {
    /// Storage path for HotShot query service data.
    #[clap(long, env = "ESPRESSO_SEQUENCER_STORAGE_PATH")]
    pub storage_path: PathBuf,

    /// Create new query storage instead of opening existing one.
    #[clap(long, env = "ESPRESSO_SEQUENCER_RESET_STORE")]
    pub reset_store: bool,
}

/// Options for the submission API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct SubmitOptions;

impl SubmitOptions {
    fn init<S, I: NodeImplementation<SeqTypes>>(
        self,
        app: &mut App<S, hotshot_query_service::Error>,
    ) -> io::Result<()>
    where
        S: 'static + Send + Sync + tide_disco::method::WriteState,
        S::State: Send + Sync + Borrow<SystemContextHandle<SeqTypes, I>>,
    {
        // Include API specification in binary
        let toml = toml::from_str::<toml::value::Value>(include_str!("api.toml"))
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        // Initialize submit API
        let mut submit_api = Api::<S, hotshot_query_service::Error>::new(toml)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        // Define submit route for submit API
        submit_api
            .post("submit", |req, state| {
                async move {
                    let state = Borrow::<SystemContextHandle<SeqTypes, I>>::borrow(state);
                    state
                        .submit_transaction(
                            req.body_auto::<Transaction>()
                                .map_err(|err| Error::internal(err.to_string()))?,
                        )
                        .await
                        .map_err(|err| Error::internal(err.to_string()))
                }
                .boxed()
            })
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        // Register modules in app
        app.register_module("submit", submit_api)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub http: HttpOptions,
    pub query: Option<QueryOptions>,
    pub submit: Option<SubmitOptions>,
}

impl From<HttpOptions> for Options {
    fn from(http: HttpOptions) -> Self {
        Self {
            http,
            query: None,
            submit: None,
        }
    }
}

impl Options {
    /// Add a query API module.
    pub fn query(mut self, opt: QueryOptions) -> Self {
        self.query = Some(opt);
        self
    }

    /// Add a submit API module.
    pub fn submit(mut self, opt: SubmitOptions) -> Self {
        self.submit = Some(opt);
        self
    }

    pub async fn serve<I: NodeImplementation<SeqTypes, Leaf = Leaf>>(
        self,
        init_handle: HandleFromMetrics<I>,
    ) -> io::Result<SequencerNode<I>> {
        // The server state type depends on whether we are running a query API or not, so we handle
        // the two cases differently.
        let (handle, node_index, update_task) = if let Some(query) = self.query {
            type StateType<I> = Arc<RwLock<AppState<I>>>;

            let storage_path = Path::new(&query.storage_path);
            let query_state = {
                if query.reset_store {
                    QueryData::create(storage_path, ())
                } else {
                    QueryData::open(storage_path, ())
                }
            }
            .expect("Failed to initialize query data storage");
            let metrics: Box<dyn Metrics> = query_state.metrics();

            // Start up handle
            let (handle, node_index) = init_handle(metrics).await;

            // Get a clone the handle to use for populating the query data with consensus events.
            //
            // We must do this _before_ starting consensus on the handle, otherwise we could miss
            // the first events emitted by consensus.
            let mut watch_handle = handle.clone();

            let state = Arc::new(RwLock::new(AppState::<I> {
                submit_state: handle.clone(),
                query_state,
            }));

            let mut app = App::<_, hotshot_query_service::Error>::with_state(state.clone());

            // Initialize submit API
            if let Some(submit) = self.submit {
                submit.init(&mut app)?;
            }

            // Initialize availability and status APIs
            let mut options: AvailabilityOptions = Default::default();
            let namespace_extension =
                toml::from_str::<toml::value::Value>(include_str!("namespace_api.toml"))
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            options.extensions.push(namespace_extension);
            let mut availability_api =
                availability::define_api::<StateType<I>, SeqTypes, I>(&options)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            let status_api = status::define_api::<StateType<I>>(&Default::default())
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            availability_api
                .get("getnamespaceproof", |req, state| {
                    async move {
                        let height: u64 = req.integer_param("height")?;
                        let namespace: u64 = req.integer_param("namespace")?;
                        let block = state
                            .get_nth_block_iter(height as usize)
                            .next()
                            .ok_or(AvailabilityError::InvalidLeafHeight { height })?
                            .ok_or(AvailabilityError::MissingBlock { height })?;

                        let proof = block.block().get_namespace_proof(namespace.into());
                        let nmt_root = block.block().get_nmt_root();
                        Ok(NamespaceProofQueryData { nmt_root, proof })
                    }
                    .boxed()
                })
                .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?;

            // Register modules in app
            app.register_module("availability", availability_api)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?
                .register_module("status", status_api)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?;

            let task = spawn(async move {
                futures::join!(
                    app.serve(format!("0.0.0.0:{}", self.http.port)),
                    async move {
                        tracing::debug!("waiting for event");
                        while let Ok(event) = watch_handle.next_event().await {
                            tracing::debug!("got event {:?}", event);
                            // If update results in an error, program state is unrecoverable
                            if let Err(err) = state.write().await.update(&event).await {
                                tracing::error!(
                                    "failed to update event {:?}: {}; updater task will exit",
                                    event,
                                    err
                                );
                                panic!();
                            }
                        }
                        tracing::warn!("end of HotShot event stream, updater task will exit");
                    }
                )
                .0
            });

            (handle, node_index, task)
        } else {
            let (handle, node_index) = init_handle(Box::new(NoMetrics)).await;
            let mut app =
                App::<_, hotshot_query_service::Error>::with_state(RwLock::new(handle.clone()));

            // Initialize submit API
            if let Some(submit) = self.submit {
                submit.init::<_, I>(&mut app)?;
            }

            (
                handle,
                node_index,
                spawn(app.serve(format!("0.0.0.0:{}", self.http.port))),
            )
        };

        // Start consensus.
        handle.start().await;

        Ok(SequencerNode {
            handle,
            update_task,
            node_index,
        })
    }
}

type NodeIndex = u64;

pub struct SequencerNode<I: NodeImplementation<SeqTypes>> {
    pub handle: SystemContextHandle<SeqTypes, I>,
    pub update_task: JoinHandle<io::Result<()>>,
    pub node_index: NodeIndex,
}

pub type HandleFromMetrics<I> = Box<
    dyn FnOnce(
        Box<dyn Metrics>,
    ) -> BoxFuture<'static, (SystemContextHandle<SeqTypes, I>, NodeIndex)>,
>;

struct AppState<I: NodeImplementation<SeqTypes>> {
    pub submit_state: SystemContextHandle<SeqTypes, I>,
    pub query_state: QueryData<SeqTypes, I, ()>,
}

impl<I: NodeImplementation<SeqTypes>> Borrow<SystemContextHandle<SeqTypes, I>> for AppState<I> {
    fn borrow(&self) -> &SystemContextHandle<SeqTypes, I> {
        &self.submit_state
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    proof: NamespaceProofType,
    nmt_root: NMTRoot,
}

impl NamespaceProofQueryData {
    pub fn proof(&self) -> &NamespaceProofType {
        &self.proof
    }

    pub fn nmt_root(&self) -> &NMTRoot {
        &self.nmt_root
    }
}

impl<I: NodeImplementation<SeqTypes, Leaf = Leaf>> AppState<I> {
    pub async fn update(
        &mut self,
        event: &Event<SeqTypes, Leaf>,
    ) -> Result<(), <QueryData<SeqTypes, I, ()> as UpdateDataSource<SeqTypes, I>>::Error> {
        self.query_state.update(event)?;
        self.query_state.commit_version().await?;
        Ok(())
    }
}

impl<I: NodeImplementation<SeqTypes>> AvailabilityDataSource<SeqTypes, I> for AppState<I> {
    type Error = <QueryData<SeqTypes, I, ()> as AvailabilityDataSource<SeqTypes, I>>::Error;

    type LeafIterType<'a> =
        <QueryData<SeqTypes, I, ()> as AvailabilityDataSource<SeqTypes, I>>::LeafIterType<'a>;

    type BlockIterType<'a> =
        <QueryData<SeqTypes, I, ()> as AvailabilityDataSource<SeqTypes, I>>::BlockIterType<'a>;

    type LeafStreamType =
        <QueryData<SeqTypes, I, ()> as AvailabilityDataSource<SeqTypes, I>>::LeafStreamType;
    type BlockStreamType =
        <QueryData<SeqTypes, I, ()> as AvailabilityDataSource<SeqTypes, I>>::BlockStreamType;

    fn get_nth_leaf_iter(&self, n: usize) -> Self::LeafIterType<'_> {
        self.query_state.get_nth_leaf_iter(n)
    }

    fn get_nth_block_iter(&self, n: usize) -> Self::BlockIterType<'_> {
        self.query_state.get_nth_block_iter(n)
    }

    fn get_leaf_index_by_hash(
        &self,
        hash: hotshot_query_service::availability::LeafHash<SeqTypes, I>,
    ) -> Option<u64> {
        self.query_state.get_leaf_index_by_hash(hash)
    }

    fn get_block_index_by_hash(
        &self,
        hash: hotshot_query_service::availability::BlockHash<SeqTypes>,
    ) -> Option<u64> {
        self.query_state.get_block_index_by_hash(hash)
    }

    fn get_txn_index_by_hash(
        &self,
        hash: hotshot_query_service::availability::TransactionHash<SeqTypes>,
    ) -> Option<(u64, u64)> {
        self.query_state.get_txn_index_by_hash(hash)
    }

    fn get_block_ids_by_proposer_id(
        &self,
        id: &hotshot_types::traits::signature_key::EncodedPublicKey,
    ) -> Vec<u64> {
        self.query_state.get_block_ids_by_proposer_id(id)
    }

    fn subscribe_leaves(&self, height: usize) -> Result<Self::LeafStreamType, Self::Error> {
        self.query_state.subscribe_leaves(height)
    }

    fn subscribe_blocks(&self, height: usize) -> Result<Self::BlockStreamType, Self::Error> {
        self.query_state.subscribe_blocks(height)
    }
}

impl<I: NodeImplementation<SeqTypes>> StatusDataSource for AppState<I> {
    type Error = <QueryData<SeqTypes, I, ()> as StatusDataSource>::Error;

    fn block_height(&self) -> Result<usize, Self::Error> {
        self.query_state.block_height()
    }

    fn mempool_info(&self) -> Result<hotshot_query_service::status::MempoolQueryData, Self::Error> {
        self.query_state.mempool_info()
    }

    fn success_rate(&self) -> Result<f64, Self::Error> {
        self.query_state.success_rate()
    }

    fn export_metrics(&self) -> Result<String, Self::Error> {
        self.query_state.export_metrics()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        testing::{init_hotshot_handles, wait_for_decide_on_handle},
        transaction::Transaction,
        vm::VmId,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use futures::FutureExt;
    use hotshot_types::traits::metrics::Metrics;
    use portpicker::pick_unused_port;
    use surf_disco::Client;

    use tempfile::TempDir;
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};

    use super::{HttpOptions, Options, QueryOptions, SequencerNode};

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.start().await;
        }
        let init_handle =
            Box::new(|_: Box<dyn Metrics>| async move { (handles[0].clone(), 0) }.boxed());

        let options = Options::from(HttpOptions { port });
        options.serve(init_handle).await.unwrap();

        client.connect(None).await;
        let health = client.get::<AppHealth>("healthcheck").send().await.unwrap();
        assert_eq!(health.status, HealthStatus::Available);
    }

    #[async_std::test]
    async fn submit_test_with_query_module() {
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        submit_test_helper(Some(QueryOptions {
            storage_path,
            reset_store: true,
        }))
        .await
    }

    #[async_std::test]
    async fn submit_test_without_query_module() {
        submit_test_helper(None).await
    }

    async fn submit_test_helper(query_opt: Option<QueryOptions>) {
        setup_logging();
        setup_backtrace();

        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        // Get list of HotShot handles, take the first one, and submit a transaction to it
        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.start().await;
        }

        let init_handle =
            Box::new(|_: Box<dyn Metrics>| async move { (handles[0].clone(), 0) }.boxed());

        let mut options = Options::from(HttpOptions { port }).submit(Default::default());
        if let Some(query) = query_opt {
            options = options.query(query);
        }
        let SequencerNode { handle, .. } = options.serve(init_handle).await.unwrap();

        client.connect(None).await;

        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(handle.clone(), txn)
            .await
            .unwrap()
    }
}
