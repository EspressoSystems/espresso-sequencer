use crate::{
    transaction::{SequencerTransaction, Transaction},
    Leaf, SeqTypes,
};
use async_std::{
    sync::RwLock,
    task::{spawn, JoinHandle},
};
use futures::{future::BoxFuture, FutureExt};
use hotshot::types::HotShotHandle;
use hotshot::{traits::NodeImplementation, types::Event};
use hotshot_query_service::{
    availability::{self, AvailabilityDataSource},
    data_source::{QueryData, UpdateDataSource},
    status::{self, StatusDataSource},
    Error,
};
use hotshot_types::traits::metrics::Metrics;
use std::{io, sync::Arc};
use tide_disco::{Api, App};

type NodeIndex = u64;

pub struct SequencerNode<I: NodeImplementation<SeqTypes>> {
    pub handle: HotShotHandle<SeqTypes, I>,
    pub update_task: JoinHandle<io::Result<()>>,
    pub node_index: NodeIndex,
}

pub type HandleFromMetrics<I> = Box<
    dyn FnOnce(Box<dyn Metrics>) -> BoxFuture<'static, (HotShotHandle<SeqTypes, I>, NodeIndex)>,
>;

struct AppState<I: NodeImplementation<SeqTypes>> {
    pub submit_state: HotShotHandle<SeqTypes, I>,
    pub query_state: QueryData<SeqTypes, I, ()>,
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

pub async fn serve<I: NodeImplementation<SeqTypes, Leaf = Leaf>>(
    query_state: QueryData<SeqTypes, I, ()>,
    init_handle: HandleFromMetrics<I>,
    port: u16,
) -> io::Result<SequencerNode<I>> {
    type StateType<I> = Arc<RwLock<AppState<I>>>;

    let metrics: Box<dyn Metrics> = query_state.metrics();

    // Start up handle
    let (handle, node_index) = init_handle(metrics).await;
    handle.start().await;

    let mut watch_handle = handle.clone();

    let state = Arc::new(RwLock::new(AppState::<I> {
        submit_state: handle.clone(),
        query_state,
    }));

    let mut app = App::<StateType<I>, hotshot_query_service::Error>::with_state(state.clone());

    // Include API specification in binary
    let toml = toml::from_str::<toml::value::Value>(include_str!("api.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Initialize submit API
    let mut submit_api = Api::<StateType<I>, hotshot_query_service::Error>::new(toml)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Define submit route for submit API
    submit_api
        .post("submit", |req, state| {
            async move {
                state
                    .submit_state
                    .submit_transaction(SequencerTransaction::Wrapped(
                        req.body_auto::<Transaction>()
                            .map_err(|err| Error::internal(err.to_string()))?,
                    ))
                    .await
                    .map_err(|err| Error::internal(err.to_string()))
            }
            .boxed()
        })
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Initialize availability and status APIs
    let availability_api =
        availability::define_api::<StateType<I>, SeqTypes, I>(&Default::default())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let status_api = status::define_api::<StateType<I>>(&Default::default())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Register modules in app
    app.register_module("submit", submit_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
        .register_module("availability", availability_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?
        .register_module("status", status_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?;

    let update_task = spawn(async move {
        futures::join!(app.serve(format!("0.0.0.0:{port}")), async move {
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
        })
        .0
    });
    Ok(SequencerNode {
        handle,
        update_task,
        node_index,
    })
}

#[cfg(test)]
mod test {
    use crate::{
        api::serve,
        testing::{init_hotshot_handles, wait_for_decide_on_handle},
        transaction::{SequencerTransaction, Transaction},
        vm::VmId,
    };
    use futures::FutureExt;
    use hotshot_query_service::data_source::QueryData;
    use hotshot_types::traits::metrics::Metrics;
    use portpicker::pick_unused_port;
    use std::path::Path;
    use surf_disco::Client;

    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    use super::SequencerNode;

    #[async_std::test]
    async fn submit_test() {
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

        let tmp_dir = TempDir::new().unwrap();
        let storage_path: &Path = &(tmp_dir.path().join("tmp_storage"));

        let query_data = QueryData::create(storage_path, ()).unwrap();

        let SequencerNode { handle, .. } = serve(query_data, init_handle, port).await.unwrap();

        client.connect(None).await;

        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(handle.clone(), SequencerTransaction::Wrapped(txn))
            .await
            .unwrap()
    }
}
