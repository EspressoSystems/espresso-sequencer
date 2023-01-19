use crate::{
    transaction::{SequencerTransaction, Transaction},
    SeqTypes,
};
use async_std::{
    sync::Mutex,
    task::{spawn, JoinHandle},
};
use futures::{future::BoxFuture, FutureExt};
use hotshot::traits::NodeImplementation;
use hotshot::types::HotShotHandle;
use hotshot_query_service::{
    availability::{self, AvailabilityDataSource},
    data_source::{QueryData, UpdateDataSource},
    status::{self, StatusDataSource},
    Error,
};
use hotshot_types::traits::metrics::Metrics;
use std::io;
use tide_disco::{Api, App};

pub type HandleFromMetrics<I> =
    Box<dyn FnOnce(Box<dyn Metrics>) -> BoxFuture<'static, HotShotHandle<SeqTypes, I>>>;

struct AppState<I: NodeImplementation<SeqTypes>> {
    pub submit_state: HotShotHandle<SeqTypes, I>,
    pub query_state: QueryData<SeqTypes, ()>,
}

impl<I: NodeImplementation<SeqTypes>> AvailabilityDataSource<SeqTypes> for AppState<I> {
    type LeafIterType<'a> =
        <QueryData<SeqTypes, ()> as AvailabilityDataSource<SeqTypes>>::LeafIterType<'a>;

    type BlockIterType<'a> =
        <QueryData<SeqTypes, ()> as AvailabilityDataSource<SeqTypes>>::BlockIterType<'a>;

    fn get_nth_leaf_iter(&self, n: usize) -> Self::LeafIterType<'_> {
        self.query_state.get_nth_leaf_iter(n)
    }

    fn get_nth_block_iter(&self, n: usize) -> Self::BlockIterType<'_> {
        self.query_state.get_nth_block_iter(n)
    }

    fn get_leaf_index_by_hash(
        &self,
        hash: hotshot_query_service::availability::LeafHash<SeqTypes>,
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
}

impl<I: NodeImplementation<SeqTypes>> StatusDataSource for AppState<I> {
    type Error = io::Error;

    fn block_height(&self) -> Result<usize, Self::Error> {
        self.query_state
            .block_height()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    fn mempool_info(&self) -> Result<hotshot_query_service::status::MempoolQueryData, Self::Error> {
        self.query_state
            .mempool_info()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    fn success_rate(&self) -> Result<f64, Self::Error> {
        self.query_state
            .success_rate()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    fn export_metrics(&self) -> Result<String, Self::Error> {
        self.query_state
            .export_metrics()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }
}

pub async fn serve<I: NodeImplementation<SeqTypes>>(
    query_state: QueryData<SeqTypes, ()>,
    init_handle: HandleFromMetrics<I>,
    port: u16,
) -> io::Result<JoinHandle<io::Result<()>>> {
    type StateType<I> = Mutex<AppState<I>>;

    let metrics: Box<dyn Metrics> = query_state.metrics();

    // Start up handle
    let handle = init_handle(metrics).await.clone();
    handle.start().await;

    let state = Mutex::new(AppState::<I> {
        submit_state: handle,
        query_state,
    });

    let mut app = App::<StateType<I>, hotshot_query_service::Error>::with_state(state);

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
    let availability_api = availability::define_api::<StateType<I>, SeqTypes>(&Default::default())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let status_api = status::define_api::<StateType<I>>(&Default::default())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Register modules in app
    app.register_module("api", submit_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
        .register_module("availability", availability_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?
        .register_module("status", status_api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, Error::internal(err)))?;

    Ok(spawn(app.serve(format!("0.0.0.0:{}", port))))
}

#[cfg(test)]
mod test {
    use crate::{
        api::serve,
        test::{init_hotshot_handles, wait_for_decide_on_handle},
        transaction::{SequencerTransaction, Transaction},
        vm::VmId,
        Node, SeqTypes,
    };
    use hotshot::traits::implementations::MemoryNetwork;
    use hotshot_query_service::data_source::QueryData;
    use hotshot_types::traits::metrics::Metrics;
    use portpicker::pick_unused_port;
    use std::path::Path;
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    use super::HandleFromMetrics;

    #[async_std::test]
    async fn submit_test() {
        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{}", port).parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        // Get list of HotShot handles, take the first one, and submit a transaction to it
        let handles = init_hotshot_handles().await;

        let watch_handle = handles[0].clone();

        let init_handle: HandleFromMetrics<Node<MemoryNetwork<SeqTypes>>> =
            Box::new(|_: Box<dyn Metrics>| Box::pin(async move { handles[0].clone() }));

        // TODO: obvious placeholder
        let storage_path = Path::new("obvious placeholder");

        let query_data = QueryData::create(storage_path, ()).unwrap();

        serve(query_data, init_handle, port).await.unwrap();

        client.connect(None).await;

        client
            .post::<()>("api/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(watch_handle.clone(), SequencerTransaction::Wrapped(txn))
            .await
            .unwrap()
    }
}
