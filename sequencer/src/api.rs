use self::data_source::StateSignatureDataSource;
use crate::{
    network, persistence::SequencerPersistence, state::ValidatedState,
    state_signature::StateSigner, Node, NodeState, SeqTypes, SequencerContext, Transaction,
};
use async_once_cell::Lazy;
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use data_source::{StateDataSource, SubmitDataSource};
use derivative::Derivative;
use futures::{
    future::{BoxFuture, Future, FutureExt},
    stream::{BoxStream, Stream},
};
use hotshot::types::{Event, SystemContextHandle};
use hotshot_events_service::events_source::{BuilderEvent, EventsSource, EventsStreamer};
use hotshot_query_service::data_source::ExtensibleDataSource;
use hotshot_types::{data::ViewNumber, light_client::StateSignatureRequestBody};
use std::pin::Pin;
use vbs::version::StaticVersionType;

pub mod data_source;
pub mod endpoints;
pub mod fs;
pub mod options;
pub mod sql;
mod update;

pub use options::Options;

type BoxLazy<T> = Pin<Arc<Lazy<T, BoxFuture<'static, T>>>>;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
struct ConsensusState<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType> {
    state_signer: Arc<StateSigner<Ver>>,
    event_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,
    node_state: NodeState,

    #[derivative(Debug = "ignore")]
    handle: SystemContextHandle<SeqTypes, Node<N, P>>,
}

impl<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    From<&SequencerContext<N, P, Ver>> for ConsensusState<N, P, Ver>
{
    fn from(ctx: &SequencerContext<N, P, Ver>) -> Self {
        Self {
            state_signer: ctx.state_signer(),
            event_streamer: ctx.get_event_streamer(),
            node_state: ctx.node_state(),
            handle: ctx.consensus().clone(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
struct ApiState<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType> {
    // The consensus state is initialized lazily so we can start the API (and healthcheck endpoints)
    // before consensus has started. Any endpoint that uses consensus state will wait for
    // initialization to finish, but endpoints that do not require a consensus handle can proceed
    // without waiting.
    #[derivative(Debug = "ignore")]
    consensus: BoxLazy<ConsensusState<N, P, Ver>>,
}

impl<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    ApiState<N, P, Ver>
{
    fn new(init: impl Future<Output = ConsensusState<N, P, Ver>> + Send + 'static) -> Self {
        Self {
            consensus: Arc::pin(Lazy::from_future(init.boxed())),
        }
    }

    fn event_stream(&self) -> impl Stream<Item = Event<SeqTypes>> + Unpin {
        let state = self.clone();
        async move { state.consensus().await.get_event_stream() }
            .boxed()
            .flatten_stream()
    }

    async fn state_signer(&self) -> &StateSigner<Ver> {
        &self.consensus.as_ref().get().await.get_ref().state_signer
    }

    async fn event_streamer(&self) -> &RwLock<EventsStreamer<SeqTypes>> {
        &self.consensus.as_ref().get().await.get_ref().event_streamer
    }

    async fn consensus(&self) -> &SystemContextHandle<SeqTypes, Node<N, P>> {
        &self.consensus.as_ref().get().await.get_ref().handle
    }

    async fn node_state(&self) -> &NodeState {
        &self.consensus.as_ref().get().await.get_ref().node_state
    }
}

type StorageState<N, P, D, Ver> = ExtensibleDataSource<D, ApiState<N, P, Ver>>;

#[async_trait]
impl<N: network::Type, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    EventsSource<SeqTypes> for ApiState<N, P, Ver>
{
    type EventStream = BoxStream<'static, Arc<BuilderEvent<SeqTypes>>>;

    async fn get_event_stream(&self) -> Self::EventStream {
        self.event_streamer()
            .await
            .read()
            .await
            .get_event_stream()
            .await
    }
}

impl<
        N: network::Type,
        D: Send + Sync,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
    > SubmitDataSource<N, P> for StorageState<N, P, D, Ver>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        self.as_ref().submit(tx).await
    }
}

impl<N: network::Type, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    SubmitDataSource<N, P> for ApiState<N, P, Ver>
{
    async fn submit(&self, tx: Transaction) -> anyhow::Result<()> {
        self.consensus().await.submit_transaction(tx).await?;
        Ok(())
    }
}

impl<
        N: network::Type,
        D: Send + Sync,
        Ver: StaticVersionType + 'static,
        P: SequencerPersistence,
    > StateDataSource for StorageState<N, P, D, Ver>
{
    async fn get_decided_state(&self) -> Arc<ValidatedState> {
        self.as_ref().get_decided_state().await
    }

    async fn get_undecided_state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>> {
        self.as_ref().get_undecided_state(view).await
    }
}

impl<N: network::Type, Ver: StaticVersionType + 'static, P: SequencerPersistence> StateDataSource
    for ApiState<N, P, Ver>
{
    async fn get_decided_state(&self) -> Arc<ValidatedState> {
        self.consensus().await.get_decided_state().await
    }

    async fn get_undecided_state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>> {
        self.consensus().await.get_state(view).await
    }
}

#[async_trait]
impl<N: network::Type, D: Sync, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    StateSignatureDataSource<N> for StorageState<N, P, D, Ver>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.as_ref().get_state_signature(height).await
    }
}

#[async_trait]
impl<N: network::Type, Ver: StaticVersionType + 'static, P: SequencerPersistence>
    StateSignatureDataSource<N> for ApiState<N, P, Ver>
{
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.state_signer().await.get_state_signature(height).await
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod api_tests {
    use self::options::HotshotEvents;

    use super::*;
    use crate::test_helpers::{
        state_signature_test_helper, state_test_helper, status_test_helper, submit_test_helper,
        TestNetwork,
    };
    use crate::{
        persistence::no_storage::NoStorage,
        testing::{wait_for_decide_on_handle, TestConfig},
        Header,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use committable::Committable;
    use data_source::testing::TestableSequencerDataSource;
    use endpoints::NamespaceProofQueryData;
    use es_version::SequencerVersion;
    use ethers::utils::Anvil;
    use futures::stream::StreamExt;
    use hotshot_query_service::availability::LeafQueryData;
    use hotshot_types::vid::vid_scheme;
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    #[async_std::test]
    pub(crate) async fn submit_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        submit_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn status_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        status_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn state_signature_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        state_signature_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn test_namespace_query<D: TestableSequencerDataSource>() {
        setup_logging();
        setup_backtrace();

        let vid = vid_scheme(5);
        let txn = Transaction::new(Default::default(), vec![1, 2, 3, 4]);

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let storage = D::create_storage().await;
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network = TestNetwork::new(
            D::options(&storage, options::Http { port }.into()).submit(Default::default()),
            [NoStorage; TestConfig::NUM_NODES],
            l1,
        )
        .await;
        let mut events = network.server.get_event_stream();

        // Connect client.
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        // Wait for at least one empty block to be sequenced (after consensus starts VID).
        client
            .socket("availability/stream/leaves/0")
            .subscribe::<LeafQueryData<SeqTypes>>()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        let block_height = wait_for_decide_on_handle(&mut events, &txn).await as usize;
        tracing::info!(block_height, "transaction sequenced");
        let mut found_txn = false;
        let mut found_empty_block = false;
        for block_num in 0..=block_height {
            let header: Header = client
                .get(&format!("availability/header/{block_num}"))
                .send()
                .await
                .unwrap();
            let ns_query_res: NamespaceProofQueryData = client
                .get(&format!("availability/block/{block_num}/namespace/0"))
                .send()
                .await
                .unwrap();
            ns_query_res
                .proof
                .verify(&vid, &header.payload_commitment, &header.ns_table)
                .unwrap();

            found_empty_block = found_empty_block || ns_query_res.transactions.is_empty();

            for txn in ns_query_res.transactions {
                if txn.commit() == hash {
                    // Ensure that we validate an inclusion proof
                    found_txn = true;
                }
            }
        }
        assert!(found_txn);
        assert!(found_empty_block);
    }

    #[async_std::test]
    pub(crate) async fn state_test_with_query_module<D: TestableSequencerDataSource>() {
        let storage = D::create_storage().await;
        state_test_helper(|opt| D::options(&storage, opt)).await
    }

    #[async_std::test]
    pub(crate) async fn test_hotshot_event_streaming<D: TestableSequencerDataSource>() {
        use hotshot_events_service::events_source::BuilderEvent;
        use HotshotEvents;

        setup_logging();

        setup_backtrace();

        let hotshot_event_streaming_port =
            pick_unused_port().expect("No ports free for hotshot event streaming");
        let query_service_port = pick_unused_port().expect("No ports free for query service");

        let url = format!("http://localhost:{hotshot_event_streaming_port}")
            .parse()
            .unwrap();

        let hotshot_events = HotshotEvents {
            events_service_port: hotshot_event_streaming_port,
        };

        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        let options = Options::from(options::Http {
            port: query_service_port,
        })
        .hotshot_events(hotshot_events);

        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let _network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;

        let mut subscribed_events = client
            .socket("hotshot-events/events")
            .subscribe::<BuilderEvent<SeqTypes>>()
            .await
            .unwrap();

        let total_count = 5;
        // wait for these events to receive on client 1
        let mut receive_count = 0;
        loop {
            let event = subscribed_events.next().await.unwrap();
            tracing::info!(
                "Received event in hotshot event streaming Client 1: {:?}",
                event
            );
            receive_count += 1;
            if receive_count > total_count {
                tracing::info!("Client Received atleast desired events, exiting loop");
                break;
            }
        }
        // Offset 1 is due to the startup event info
        assert_eq!(receive_count, total_count + 1);
    }
}

#[cfg(test)]
mod test {

    use self::{
        data_source::testing::TestableSequencerDataSource, sql::DataSource as SqlDataSource,
    };
    use super::*;
    use crate::{
        catchup::{mock::MockStateCatchup, StatePeers},
        empty_builder_commitment,
        persistence::no_storage::NoStorage,
        state::{FeeAccount, FeeAmount},
        test_helpers::{
            self, state_signature_test_helper, state_test_helper, status_test_helper,
            submit_test_helper, TestNetwork,
        },
        testing::TestConfig,
        Header, NodeState,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use committable::{Commitment, Committable};
    use es_version::{SequencerVersion, SEQUENCER_VERSION};
    use ethers::utils::Anvil;
    use futures::future::{self, join_all};
    use futures::stream::{StreamExt, TryStreamExt};
    use hotshot::types::EventType;
    use hotshot_query_service::{
        availability::{BlockQueryData, LeafQueryData},
        types::HeightIndexed,
    };
    use hotshot_types::{
        event::LeafInfo,
        traits::{block_contents::BlockHeader, metrics::NoMetrics},
    };
    use jf_primitives::merkle_tree::{
        prelude::{MerkleProof, Sha3Node},
        AppendableMerkleTreeScheme,
    };
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);
        let options = Options::from(options::Http { port });
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let _network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;

        client.connect(None).await;
        let health = client.get::<AppHealth>("healthcheck").send().await.unwrap();
        assert_eq!(health.status, HealthStatus::Available);
    }

    #[async_std::test]
    async fn status_test_without_query_module() {
        status_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn submit_test_without_query_module() {
        submit_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn state_signature_test_without_query_module() {
        state_signature_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn state_test_without_query_module() {
        state_test_helper(|opt| opt).await
    }

    #[async_std::test]
    async fn test_merklized_state_api() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");

        let storage = SqlDataSource::create_storage().await;
        let options = SqlDataSource::options(
            &storage,
            Options::from(options::Http { port })
                .state(Default::default())
                .status(Default::default()),
        );

        // Populate one account so we have something to look up later. Leave the other accounts
        // unpopulated, which proves we can handle state updates even with missing accounts.
        let account = TestConfig::builder_key(0).fee_account();
        let mut state = ValidatedState::default();
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        state.prefund_account(account, 1.into());
        let mut network = TestNetwork::with_state(
            options,
            std::array::from_fn(|_| state.clone()),
            [NoStorage; TestConfig::NUM_NODES],
            std::array::from_fn(|_| MockStateCatchup::default()),
            l1,
        )
        .await;

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError, SequencerVersion> = Client::new(url);

        client.connect(None).await;

        // Wait until some blocks have been decided.
        tracing::info!("waiting for blocks");
        let blocks = client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(4)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        // sleep for few seconds so that state data is upserted
        tracing::info!("waiting for state to be inserted");
        sleep(Duration::from_secs(5)).await;
        network.stop_consensus().await;

        for block in blocks {
            let i = block.height();
            tracing::info!(i, "get block state");
            let path = client
                .get::<MerkleProof<Commitment<Header>, u64, Sha3Node, 3>>(&format!(
                    "block-state/{}/{i}",
                    i + 1
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.elem().unwrap(), block.hash());

            tracing::info!(i, "get fee state");
            let path = client
                .get::<MerkleProof<FeeAmount, FeeAccount, Sha3Node, 256>>(&format!(
                    "fee-state/{}/{}",
                    i + 1,
                    account
                ))
                .send()
                .await
                .unwrap();
            assert_eq!(*path.index(), account);
            assert!(*path.elem().unwrap() > 0.into(), "{:?}", path.elem());
        }
    }

    #[async_std::test]
    async fn test_catchup() {
        setup_logging();
        setup_backtrace();

        // Create some non-trivial initial state. We will give all the nodes in the network this
        // state, except for one, which will have a forgotten state and need to catch up.
        let mut state = ValidatedState::default();
        // Prefund an arbitrary account so the fee state has some data to forget.
        state.prefund_account(Default::default(), 1000.into());
        // Push an arbitrary header commitment so the block state has some data to forget.
        state
            .block_merkle_tree
            .push(
                Header::genesis(
                    &NodeState::mock(),
                    Default::default(),
                    empty_builder_commitment(),
                    Default::default(),
                )
                .commit(),
            )
            .unwrap();
        let states = std::array::from_fn(|i| {
            if i == TestConfig::NUM_NODES - 1 {
                state.forget()
            } else {
                state.clone()
            }
        });

        // Start a sequencer network, using the query service for catchup.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network = TestNetwork::with_state(
            Options::from(options::Http { port }).catchup(Default::default()),
            states,
            [NoStorage; TestConfig::NUM_NODES],
            std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(vec![format!("http://localhost:{port}")
                    .parse()
                    .unwrap()])
            }),
            l1,
        )
        .await;
        let mut events = network.server.get_event_stream();

        // Wait for a (non-genesis) block proposed by the lagging node, to prove that it has caught
        // up.
        let builder = TestConfig::builder_key(TestConfig::NUM_NODES - 1).fee_account();
        'outer: loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let height = leaf.get_block_header().height;
                let leaf_builder = leaf.get_block_header().fee_info.account();
                tracing::info!(
                    "waiting for block from {builder}, block {height} is from {leaf_builder}",
                );
                if height > 1 && leaf_builder == builder {
                    break 'outer;
                }
            }
        }
    }

    #[async_std::test]
    async fn test_catchup_live() {
        setup_logging();
        setup_backtrace();

        // Similar to `test_catchup`, but instead of _starting_ with a non-trivial state and a node
        // that is missing the state, we start consensus normally, wait for it to make some
        // progress, stop one node and let it fall behind, then start it again and check that it
        // catches up.

        // Start a sequencer network, using the query service for catchup.
        let port = pick_unused_port().expect("No ports free");
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let mut network = TestNetwork::with_state(
            Options::from(options::Http { port }).catchup(Default::default()),
            Default::default(),
            [NoStorage; TestConfig::NUM_NODES],
            std::array::from_fn(|_| {
                StatePeers::<SequencerVersion>::from_urls(vec![format!("http://localhost:{port}")
                    .parse()
                    .unwrap()])
            }),
            l1,
        )
        .await;

        // Wait for replica 0 to reach a (non-genesis) decide, before disconnecting it.
        let mut events = network.peers[0].get_event_stream();
        loop {
            let event = events.next().await.unwrap();
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            if leaf_chain[0].leaf.get_height() > 0 {
                break;
            }
        }

        // Shut down and restart replica 0. We don't just stop consensus and restart it; we fully
        // drop the node and recreate it so it loses all of its temporary state and starts off from
        // genesis. It should be able to catch up by listening to proposals and then rebuild its
        // state from its peers.
        tracing::info!("shutting down node");
        network.peers.remove(0);

        // Wait for a few blocks to pass while the node is down, so it falls behind.
        network
            .server
            .get_event_stream()
            .filter(|event| future::ready(matches!(event.event, EventType::Decide { .. })))
            .take(3)
            .collect::<Vec<_>>()
            .await;

        tracing::info!("restarting node");
        let node = network
            .cfg
            .init_node(
                1,
                ValidatedState::default(),
                NoStorage,
                StatePeers::<SequencerVersion>::from_urls(vec![format!("http://localhost:{port}")
                    .parse()
                    .unwrap()]),
                &NoMetrics,
                test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                SEQUENCER_VERSION,
            )
            .await;
        let mut events = node.get_event_stream();

        // Wait for a (non-genesis) block proposed by the lagging node, to prove that it has caught
        // up.
        let builder = TestConfig::builder_key(1).fee_account();
        'outer: loop {
            let event = events.next().await.unwrap();
            tracing::info!(?event, "restarted node got event");
            let EventType::Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let height = leaf.get_height();
                let leaf_builder = leaf.get_block_header().fee_info.account();
                tracing::info!(
                    "waiting for block from {builder}, block {height} is from {leaf_builder}",
                );
                if height > 1 && leaf_builder == builder {
                    break 'outer;
                }
            }
        }
    }

    #[async_std::test]
    pub(crate) async fn test_restart() {
        setup_logging();
        setup_backtrace();

        // Initialize nodes.
        let storage =
            join_all((0..TestConfig::NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
        let persistence = join_all(
            storage
                .iter()
                .map(<SqlDataSource as TestableSequencerDataSource>::connect),
        )
        .await
        .try_into()
        .unwrap();
        let port = pick_unused_port().unwrap();
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let mut network = TestNetwork::with_state(
            SqlDataSource::options(&storage[0], options::Http { port }.into())
                .state(Default::default())
                .status(Default::default()),
            Default::default(),
            persistence,
            std::array::from_fn(|_| MockStateCatchup::default()),
            l1,
        )
        .await;

        // Connect client.
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Wait until some blocks have been decided.
        client
            .socket("availability/stream/blocks/0")
            .subscribe::<BlockQueryData<SeqTypes>>()
            .await
            .unwrap()
            .take(3)
            .collect::<Vec<_>>()
            .await;

        // Shut down the consensus nodes.
        tracing::info!("shutting down nodes");
        network.stop_consensus().await;

        // Get the block height we reached.
        let height = client
            .get::<usize>("status/block-height")
            .send()
            .await
            .unwrap();
        tracing::info!("decided {height} blocks before shutting down");

        // Get the decided chain, so we can check consistency after the restart.
        let chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        let decided_view = chain.last().unwrap().leaf().get_view_number();

        // Get the most recent state, for catchup.
        let state = network.server.consensus().get_decided_state().await;
        tracing::info!(?decided_view, ?state, "consensus state");

        // Wait for merklized state storage to update.
        while let Err(err) = client
            .get::<()>(&format!("block-state/{}/{}", height - 1, height - 2))
            .send()
            .await
        {
            tracing::info!(
                height,
                "waiting for merklized state to become available ({err:#})"
            );
            sleep(Duration::from_secs(1)).await;
        }

        // Fully shut down the API servers.
        drop(network);

        // Start up again, resuming from the last decided leaf.
        let port = pick_unused_port().expect("No ports free");
        let persistence = join_all(
            storage
                .iter()
                .map(<SqlDataSource as TestableSequencerDataSource>::connect),
        )
        .await
        .try_into()
        .unwrap();
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let _network = TestNetwork::with_state(
            SqlDataSource::options(&storage[0], options::Http { port }.into())
                .catchup(Default::default()),
            Default::default(),
            persistence,
            std::array::from_fn(|_| {
                // Catchup using node 0 as a peer. Node 0 was running the archival state service
                // before the restart, so it should be able to resume without catching up by loading
                // state from storage.
                StatePeers::<SequencerVersion>::from_urls(vec![format!("http://localhost:{port}")
                    .parse()
                    .unwrap()])
            }),
            l1,
        )
        .await;
        let client: Client<ServerError, SequencerVersion> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;
        tracing::info!(port, "server running");

        // Make sure we can decide new blocks after the restart.
        tracing::info!("waiting for decide, height {height}");
        let new_leaf: LeafQueryData<SeqTypes> = client
            .socket(&format!("availability/stream/leaves/{height}"))
            .subscribe()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_leaf.height(), height as u64);
        assert_eq!(
            new_leaf.leaf().get_parent_commitment(),
            chain[height - 1].hash()
        );

        // Ensure the new chain is consistent with the old chain.
        let new_chain: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/0")
            .subscribe()
            .await
            .unwrap()
            .take(height)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(chain, new_chain);
    }
}
