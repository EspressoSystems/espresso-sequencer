use crate::{network, transaction::Transaction, Header, Leaf, NamespaceProofType, Node, SeqTypes};
use async_std::{
    sync::RwLock,
    task::{spawn, JoinHandle},
};
use clap::Parser;
use data_source::{SequencerDataSource, SubmitDataSource};
use futures::{future::BoxFuture, FutureExt, StreamExt, TryFutureExt};
use hotshot::{types::Event, types::SystemContextHandle};
use hotshot_query_service::{
    availability::{
        self, AvailabilityDataSource, BlockHash, Error as AvailabilityError,
        Options as AvailabilityOptions, QueryBlockSnafu,
    },
    data_source::{ExtensibleDataSource, UpdateDataSource, VersionedDataSource},
    status::{self, StatusDataSource, UpdateStatusData},
    Error,
};
use hotshot_types::traits::metrics::{Metrics, NoMetrics};
use jf_primitives::merkle_tree::namespaced_merkle_tree::NamespaceProof;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::sync::Arc;
use tide_disco::{Api, App};

mod data_source;
pub mod fs;

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

/// Options for the submission API module.
#[derive(Parser, Clone, Copy, Debug, Default)]
pub struct SubmitOptions;

impl SubmitOptions {
    fn init<S, N: network::Type>(self, app: &mut App<S, Error>) -> anyhow::Result<()>
    where
        S: 'static + Send + Sync + tide_disco::method::WriteState,
        S::State: Send + Sync + SubmitDataSource<N>,
    {
        // Include API specification in binary
        let toml = toml::from_str::<toml::value::Value>(include_str!("../api/submit.toml"))?;

        // Initialize submit API
        let mut submit_api = Api::<S, Error>::new(toml)?;

        // Define submit route for submit API
        submit_api.post("submit", |req, state| {
            async move {
                state
                    .handle()
                    .submit_transaction(
                        req.body_auto::<Transaction>()
                            .map_err(|err| Error::internal(err.to_string()))?,
                    )
                    .await
                    .map_err(|err| Error::internal(err.to_string()))
            }
            .boxed()
        })?;

        // Register modules in app
        app.register_module("submit", submit_api)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub http: HttpOptions,
    pub query_fs: Option<fs::Options>,
    pub submit: Option<SubmitOptions>,
}

impl From<HttpOptions> for Options {
    fn from(http: HttpOptions) -> Self {
        Self {
            http,
            query_fs: None,
            submit: None,
        }
    }
}

impl Options {
    /// Add a query API module backed by the file system.
    pub fn query_fs(mut self, opt: fs::Options) -> Self {
        self.query_fs = Some(opt);
        self
    }

    /// Add a submit API module.
    pub fn submit(mut self, opt: SubmitOptions) -> Self {
        self.submit = Some(opt);
        self
    }

    pub async fn serve<N: network::Type>(
        self,
        init_handle: HandleFromMetrics<N>,
    ) -> anyhow::Result<SequencerNode<N>> {
        // The server state type depends on whether we are running a query API or not, so we handle
        // the two cases differently.
        let node = if let Some(opt) = self.query_fs {
            type WebState<N> = Arc<RwLock<AppState<N, fs::DataSource<N>>>>;

            let ds = <fs::DataSource<N> as SequencerDataSource<N>>::create(opt).await?;
            let metrics = ds.populate_metrics();

            // Start up handle
            let (mut handle, node_index) = init_handle(metrics).await;

            // Get an event stream from the handle to use for populating the query data with
            // consensus events.
            //
            // We must do this _before_ starting consensus on the handle, otherwise we could miss
            // the first events emitted by consensus.
            let mut events = handle.get_event_stream(Default::default()).await.0;

            let state: WebState<N> =
                Arc::new(RwLock::new(ExtensibleDataSource::new(ds, handle.clone())));
            let mut app = App::<_, Error>::with_state(state.clone());

            // Initialize submit API
            if let Some(submit) = self.submit {
                submit.init(&mut app)?;
            }

            // Initialize availability and status APIs
            let mut options: AvailabilityOptions = Default::default();
            let availability_extension = toml::from_str(include_str!("../api/availability.toml"))?;
            options.extensions.push(availability_extension);
            let mut availability_api =
                availability::define_api::<WebState<N>, SeqTypes, Node<N>>(&options)?;
            let status_api = status::define_api::<WebState<N>>(&Default::default())?;

            availability_api
                .get("getnamespaceproof", |req, state| {
                    async move {
                        let height: usize = req.integer_param("height")?;
                        let namespace: u64 = req.integer_param("namespace")?;
                        let block = state.get_block(height).await.context(QueryBlockSnafu {
                            resource: height.to_string(),
                        })?;

                        let proof = block.block().get_namespace_proof(namespace.into());
                        Ok(NamespaceProofQueryData {
                            transactions: proof
                                .get_namespace_leaves()
                                .into_iter()
                                .cloned()
                                .collect(),
                            proof,
                            header: block.block().header(),
                        })
                    }
                    .boxed()
                })?
                .get("getheader", |req, state| {
                    async move {
                        let height: usize = req.integer_param("height")?;
                        let block = state.get_block(height).await.context(QueryBlockSnafu {
                            resource: height.to_string(),
                        })?;
                        Ok(block.block().header())
                    }
                    .boxed()
                })?
                .get("gettimestampwindow", |req, state| {
                    async move {
                        let end = req.integer_param("end")?;
                        let res = if let Some(height) = req.opt_integer_param("height")? {
                            state.inner().window_from::<usize>(height, end).await
                        } else if let Some(hash) = req.opt_blob_param("hash")? {
                            state
                                .inner()
                                .window_from::<BlockHash<SeqTypes>>(hash, end)
                                .await
                        } else {
                            let start: u64 = req.integer_param("start")?;
                            state.inner().window(start, end).await
                        };
                        res.map_err(|err| AvailabilityError::Custom {
                            message: err.to_string(),
                            status: err.status(),
                        })
                    }
                    .boxed()
                })?;

            // Register modules in app
            app.register_module("availability", availability_api)?
                .register_module("status", status_api)?;

            let update_task = spawn(async move {
                futures::join!(
                    app.serve(format!("0.0.0.0:{}", self.http.port))
                        .map_err(anyhow::Error::from),
                    async move {
                        tracing::debug!("waiting for event");
                        while let Some(event) = events.next().await {
                            tracing::info!("got event {:?}", event);

                            // If update results in an error, program state is unrecoverable
                            if let Err(err) = update_state(&mut *state.write().await, &event).await
                            {
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

            SequencerNode {
                handle,
                node_index,
                update_task,
            }
        } else {
            let (handle, node_index) = init_handle(Box::new(NoMetrics)).await;
            let mut app = App::<_, Error>::with_state(RwLock::new(handle.clone()));

            // Initialize submit API
            if let Some(submit) = self.submit {
                submit.init::<_, N>(&mut app)?;
            }

            SequencerNode {
                handle,
                node_index,
                update_task: spawn(
                    app.serve(format!("0.0.0.0:{}", self.http.port))
                        .map_err(anyhow::Error::from),
                ),
            }
        };

        // Start consensus.
        node.handle.hotshot.start_consensus().await;
        Ok(node)
    }
}

type NodeIndex = u64;

pub struct SequencerNode<N: network::Type> {
    pub handle: SystemContextHandle<SeqTypes, Node<N>>,
    pub update_task: JoinHandle<anyhow::Result<()>>,
    pub node_index: NodeIndex,
}

pub type HandleFromMetrics<N> = Box<
    dyn FnOnce(
        Box<dyn Metrics>,
    ) -> BoxFuture<'static, (SystemContextHandle<SeqTypes, Node<N>>, NodeIndex)>,
>;

type AppState<N, D> = ExtensibleDataSource<D, SystemContextHandle<SeqTypes, Node<N>>>;

impl<N: network::Type, D> SubmitDataSource<N> for AppState<N, D> {
    fn handle(&self) -> &SystemContextHandle<SeqTypes, Node<N>> {
        self.as_ref()
    }
}

impl<N: network::Type> SubmitDataSource<N> for SystemContextHandle<SeqTypes, Node<N>> {
    fn handle(&self) -> &SystemContextHandle<SeqTypes, Node<N>> {
        self
    }
}

async fn update_state<N: network::Type, D: SequencerDataSource<N> + Send + Sync>(
    state: &mut AppState<N, D>,
    event: &Event<SeqTypes, Leaf>,
) -> anyhow::Result<()> {
    // Remember the current block height, so we can update our local index
    // based on any new blocks that get added.
    let prev_block_height = state.block_height().await?;

    state.update(event).await?;
    state.inner_mut().refresh_indices(prev_block_height).await?;
    state.commit().await?;

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    pub proof: NamespaceProofType,
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindowQueryData {
    pub from: u64,
    pub window: Vec<Header>,
    pub prev: Option<Header>,
    pub next: Option<Header>,
}

impl TimeWindowQueryData {
    fn new(from: u64) -> Self {
        Self {
            from,
            window: vec![],
            prev: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        testing::{init_hotshot_handles, wait_for_decide_on_handle},
        transaction::Transaction,
        vm::VmId,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use futures::FutureExt;
    use hotshot_query_service::availability::BlockQueryData;
    use hotshot_types::traits::metrics::Metrics;
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
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
        submit_test_helper(Some(fs::Options {
            storage_path,
            reset_store: true,
        }))
        .await
    }

    #[async_std::test]
    async fn submit_test_without_query_module() {
        submit_test_helper(None).await
    }

    async fn submit_test_helper(query_opt: Option<fs::Options>) {
        setup_logging();
        setup_backtrace();

        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        // Get list of HotShot handles, take the first one, and submit a transaction to it
        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let init_handle =
            Box::new(|_: Box<dyn Metrics>| async move { (handles[0].clone(), 0) }.boxed());

        let mut options = Options::from(HttpOptions { port }).submit(Default::default());
        if let Some(query) = query_opt {
            options = options.query_fs(query);
        }
        let SequencerNode { mut handle, .. } = options.serve(init_handle).await.unwrap();
        let mut events = handle.get_event_stream(Default::default()).await.0;

        client.connect(None).await;

        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(&mut events, txn).await.unwrap()
    }

    #[async_std::test]
    async fn test_timestamp_window() {
        setup_logging();
        setup_backtrace();

        // Start sequencer.
        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        let init_handle =
            Box::new(|_: Box<dyn Metrics>| async move { (handles[0].clone(), 0) }.boxed());
        Options::from(HttpOptions { port })
            .query_fs(fs::Options {
                storage_path,
                reset_store: true,
            })
            .serve(init_handle)
            .await
            .unwrap();

        // Connect client.
        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        // Wait for blocks with at least three different timestamps to be sequenced. This lets us
        // test all the edge cases.
        let mut test_blocks: Vec<Vec<Header>> = vec![];
        while test_blocks.len() < 3 {
            let num_blocks = test_blocks.iter().flatten().count();

            // Wait for the next block to be sequenced.
            while client
                .get::<usize>("status/latest_block_height")
                .send()
                .await
                .unwrap()
                < num_blocks + 1
            {
                sleep(Duration::from_secs(1)).await;
            }

            let block: BlockQueryData<SeqTypes> = client
                .get(&format!("availability/block/{num_blocks}"))
                .send()
                .await
                .unwrap();
            let header = block.block().header();
            if let Some(last_timestamp) = test_blocks.last_mut() {
                if last_timestamp[0].timestamp() == header.timestamp() {
                    last_timestamp.push(header);
                } else {
                    test_blocks.push(vec![header]);
                }
            } else {
                test_blocks.push(vec![header]);
            }
        }
        tracing::info!("blocks for testing: {test_blocks:#?}");

        // Define invariants that every response should satisfy.
        let check_invariants = |res: &TimeWindowQueryData, start, end, check_prev| {
            let mut prev = res.prev.as_ref();
            if let Some(prev) = prev {
                if check_prev {
                    assert!(prev.timestamp() < start);
                }
            } else {
                // `prev` can only be `None` if the first block in the window is the genesis block.
                assert_eq!(res.from, 0);
            };
            for header in &res.window {
                assert!(start <= header.timestamp());
                assert!(header.timestamp() < end);
                if let Some(prev) = prev {
                    assert!(prev.timestamp() <= header.timestamp());
                }
                prev = Some(header);
            }
            if let Some(next) = &res.next {
                assert!(next.timestamp() >= end);
                // If there is a `next`, there must be at least one previous block (either `prev`
                // itself or the last block if the window is nonempty), so we can `unwrap` here.
                assert!(next.timestamp() >= prev.unwrap().timestamp());
            }
        };

        let get_window = |start, end| {
            let client = client.clone();
            async move {
                let res = client
                    .get(&format!("availability/headers/window/{start}/{end}"))
                    .send()
                    .await
                    .unwrap();
                tracing::info!("window for timestamp range {start}-{end}: {res:#?}");
                check_invariants(&res, start, end, true);
                res
            }
        };

        // Case 0: happy path. All blocks are available, including prev and next.
        let start = test_blocks[1][0].timestamp();
        let end = start + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.window, test_blocks[1]);
        assert_eq!(res.next.unwrap(), test_blocks[2][0]);

        // Case 1: no `prev`, start of window is before genesis.
        let start = 0;
        let end = test_blocks[0][0].timestamp() + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev, None);
        assert_eq!(res.window, test_blocks[0]);
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);

        // Case 2: no `next`, end of window is after the most recently sequenced block.
        let start = test_blocks[2][0].timestamp();
        let end = u64::MAX;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[1].last().unwrap());
        // There may have been more blocks sequenced since we grabbed `test_blocks`, so just check
        // that the prefix of the window is correct.
        assert_eq!(res.window[..test_blocks[2].len()], test_blocks[2]);
        assert_eq!(res.next, None);
        // Fetch more blocks using the `from` form of the endpoint. Start from the last block we had
        // previously (ie fetch a slightly overlapping window) to ensure there is at least one block
        // in the new window.
        let from = test_blocks.iter().flatten().count() - 1;
        let more: TimeWindowQueryData = client
            .get(&format!("availability/headers/window/from/{from}/{end}",))
            .send()
            .await
            .unwrap();
        check_invariants(&more, start, end, false);
        assert_eq!(
            more.prev.as_ref().unwrap(),
            test_blocks.iter().flatten().nth(from - 1).unwrap()
        );
        assert_eq!(
            more.window[..res.window.len() - test_blocks[2].len() + 1],
            res.window[test_blocks[2].len() - 1..]
        );
        assert_eq!(res.next, None);
        // We should get the same result whether we query by block height or hash.
        let more2: TimeWindowQueryData = client
            .get(&format!(
                "availability/headers/window/from/hash/{}/{}",
                test_blocks[2].last().unwrap().commit(),
                end
            ))
            .send()
            .await
            .unwrap();
        check_invariants(&more2, start, end, false);
        assert_eq!(more2.from, more.from);
        assert_eq!(more2.prev, more.prev);
        assert_eq!(more2.next, more.next);
        assert_eq!(more2.window[..more.window.len()], more.window);

        // Case 3: the window is empty.
        let start = test_blocks[1][0].timestamp();
        let end = start;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);
        assert_eq!(res.window, vec![]);

        // Case 5: no relevant blocks are available yet.
        client
            .get::<TimeWindowQueryData>(&format!(
                "availability/headers/window/{}/{}",
                u64::MAX - 1,
                u64::MAX
            ))
            .send()
            .await
            .unwrap_err();
    }
}
