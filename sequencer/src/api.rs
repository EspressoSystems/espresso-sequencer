use self::data_source::StateSignatureDataSource;
use crate::{
    context::SequencerContext, network, state::ValidatedState, state_signature::StateSigner, Node,
    SeqTypes,
};
use async_std::sync::Arc;
use async_trait::async_trait;
use data_source::{StateDataSource, SubmitDataSource};
use hotshot::types::SystemContextHandle;
use hotshot_query_service::data_source::ExtensibleDataSource;
use hotshot_types::{data::ViewNumber, light_client::StateSignatureRequestBody};

pub mod data_source;
pub mod endpoints;
pub mod fs;
pub mod options;
pub mod sql;
mod update;

pub use options::Options;

struct State<N: network::Type> {
    state_signer: Arc<StateSigner>,
    handle: SystemContextHandle<SeqTypes, Node<N>>,
}

impl<N: network::Type> From<&SequencerContext<N>> for State<N> {
    fn from(ctx: &SequencerContext<N>) -> Self {
        Self {
            state_signer: ctx.state_signer(),
            handle: ctx.consensus().clone(),
        }
    }
}

type StorageState<N, D> = ExtensibleDataSource<D, State<N>>;

impl<N: network::Type, D> SubmitDataSource<N> for StorageState<N, D> {
    fn consensus(&self) -> &SystemContextHandle<SeqTypes, Node<N>> {
        self.as_ref().consensus()
    }
}

impl<N: network::Type> SubmitDataSource<N> for State<N> {
    fn consensus(&self) -> &SystemContextHandle<SeqTypes, Node<N>> {
        &self.handle
    }
}

impl<N: network::Type, D: Send + Sync> StateDataSource for StorageState<N, D> {
    async fn get_decided_state(&self) -> Arc<ValidatedState> {
        self.as_ref().get_decided_state().await
    }

    async fn get_undecided_state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>> {
        self.as_ref().get_undecided_state(view).await
    }
}

impl<N: network::Type> StateDataSource for State<N> {
    async fn get_decided_state(&self) -> Arc<ValidatedState> {
        self.handle.get_decided_state().await
    }

    async fn get_undecided_state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>> {
        self.handle.get_state(view).await
    }
}

#[async_trait]
impl<N: network::Type, D: Sync> StateSignatureDataSource<N> for StorageState<N, D> {
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.as_ref().get_state_signature(height).await
    }
}

#[async_trait]
impl<N: network::Type> StateSignatureDataSource<N> for State<N> {
    async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.state_signer.get_state_signature(height).await
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::{
        api::endpoints::{AccountQueryData, BlocksFrontier},
        persistence::{no_storage::NoStorage, SequencerPersistence},
        state::BlockMerkleTree,
        testing::{wait_for_decide_on_handle, TestConfig},
        Transaction, VmId,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use commit::Committable;
    use ethers::prelude::Address;
    use futures::{
        future::{join_all, FutureExt},
        stream::StreamExt,
    };
    use hotshot::types::{Event, EventType};
    use hotshot_types::traits::{metrics::NoMetrics, node_implementation::ConsensusTime};
    use jf_primitives::merkle_tree::{MerkleCommitment, MerkleTreeScheme};
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    pub struct TestNetwork {
        pub server: SequencerContext<network::Memory>,
        pub peers: Vec<SequencerContext<network::Memory>>,
    }

    impl TestNetwork {
        pub async fn with_state(opt: Options, persistence: impl SequencerPersistence) -> Self {
            let cfg = TestConfig::default();
            let server = opt
                .serve(|metrics| {
                    let cfg = cfg.clone();
                    async move { cfg.init_node(0, persistence, &*metrics).await }.boxed()
                })
                .await
                .unwrap();
            let peers =
                join_all((1..cfg.num_nodes()).map(|i| cfg.init_node(i, NoStorage, &NoMetrics)))
                    .await;

            server.start_consensus().await;
            for ctx in &peers {
                ctx.start_consensus().await;
            }

            Self { server, peers }
        }

        pub async fn new(opt: Options) -> Self {
            Self::with_state(opt, NoStorage).await
        }

        pub async fn stop_consensus(&mut self) {
            self.server.consensus_mut().shut_down().await;
            for ctx in &mut self.peers {
                ctx.consensus_mut().shut_down().await;
            }
        }
    }

    /// Test the status API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the status API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the status module or changing the port).
    pub async fn status_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let options = opt(Options::from(options::Http { port }).status(Default::default()));
        let _network = TestNetwork::new(options).await;
        client.connect(None).await;

        // The status API is well tested in the query service repo. Here we are just smoke testing
        // that we set it up correctly. Wait for a (non-genesis) block to be sequenced and then
        // check the success rate metrics.
        while client
            .get::<u64>("status/block-height")
            .send()
            .await
            .unwrap()
            <= 1
        {
            sleep(Duration::from_secs(1)).await;
        }
        let success_rate = client
            .get::<f64>("status/success-rate")
            .send()
            .await
            .unwrap();
        // If metrics are populating correctly, we should get a finite number. If not, we might get
        // NaN or infinity due to division by 0.
        assert!(success_rate.is_finite(), "{success_rate}");
        // We know at least some views have been successful, since we finalized a block.
        assert!(success_rate > 0.0, "{success_rate}");
    }

    /// Test the submit API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the submit API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the submit module or changing the port).
    pub async fn submit_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_logging();
        setup_backtrace();

        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let options = opt(Options::from(options::Http { port }).submit(Default::default()));
        let network = TestNetwork::new(options).await;
        let mut events = network.server.get_event_stream();

        client.connect(None).await;

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(&mut events, &txn).await.unwrap()
    }

    /// Test the state signature API.
    pub async fn state_signature_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let options = opt(Options::from(options::Http { port }));
        let network = TestNetwork::new(options).await;

        let mut height: u64;
        // Wait for block >=2 appears
        // It's waiting for an extra second to make sure that the signature is generated
        loop {
            height = network
                .server
                .consensus()
                .get_decided_leaf()
                .await
                .get_height();
            sleep(std::time::Duration::from_secs(1)).await;
            if height >= 2 {
                break;
            }
        }
        // we cannot verify the signature now, because we don't know the stake table
        assert!(client
            .get::<StateSignatureRequestBody>(&format!("state-signature/block/{}", height))
            .send()
            .await
            .is_ok());
    }

    /// Test the state API with custom options.
    ///
    /// The `opt` function can be used to modify the [`Options`] which are used to start the server.
    /// By default, the options are the minimal required to run this test (configuring a port and
    /// enabling the state API). `opt` may add additional functionality (e.g. adding a query module
    /// to test a different initialization path) but should not remove or modify the existing
    /// functionality (e.g. removing the state module or changing the port).
    pub async fn state_test_helper(opt: impl FnOnce(Options) -> Options) {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let options = opt(Options::from(options::Http { port }).state(Default::default()));
        let mut network = TestNetwork::new(options).await;
        client.connect(None).await;

        // Wait for a few blocks to be decided.
        let mut events = network.server.get_event_stream();
        loop {
            if let Event {
                event: EventType::Decide { leaf_chain, .. },
                ..
            } = events.next().await.unwrap()
            {
                if leaf_chain
                    .iter()
                    .any(|(leaf, _)| leaf.block_header.height > 2)
                {
                    break;
                }
            }
        }

        // Stop consensus running on the node so we freeze the decided and undecided states.
        network.server.consensus_mut().shut_down().await;

        // Decided fee state: absent account.
        let res = client
            .get::<AccountQueryData>(&format!("state/account/{:x}", Address::default()))
            .send()
            .await
            .unwrap();
        assert_eq!(res.balance, 0.into());
        assert_eq!(
            res.proof
                .verify(
                    &network
                        .server
                        .consensus()
                        .get_decided_state()
                        .await
                        .fee_merkle_tree
                        .commitment()
                )
                .unwrap(),
            0.into()
        );

        // Undecided fee state: absent account.
        let leaf = network.server.consensus().get_decided_leaf().await;
        let view = leaf.view_number + 1;
        let res = client
            .get::<AccountQueryData>(&format!(
                "state/catchup/{}/account/{:x}",
                view.get_u64(),
                Address::default()
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(res.balance, 0.into());
        assert_eq!(
            res.proof
                .verify(
                    &network
                        .server
                        .consensus()
                        .get_state(view)
                        .await
                        .unwrap()
                        .fee_merkle_tree
                        .commitment()
                )
                .unwrap(),
            0.into()
        );

        // Decided block state.
        let res = client
            .get::<BlocksFrontier>("state/blocks")
            .send()
            .await
            .unwrap();
        let root = &network
            .server
            .consensus()
            .get_decided_state()
            .await
            .block_merkle_tree
            .commitment();
        BlockMerkleTree::verify(root.digest(), root.size() - 1, res)
            .unwrap()
            .unwrap();

        // Undecided block state.
        let res = client
            .get::<BlocksFrontier>(&format!("state/catchup/{}/blocks", view.get_u64()))
            .send()
            .await
            .unwrap();
        let root = &network
            .server
            .consensus()
            .get_state(view)
            .await
            .unwrap()
            .block_merkle_tree
            .commitment();
        BlockMerkleTree::verify(root.digest(), root.size() - 1, res)
            .unwrap()
            .unwrap();
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod generic_tests {
    use super::*;
    use crate::{testing::wait_for_decide_on_handle, Header, Transaction, VmId};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use commit::Committable;
    use data_source::testing::TestableSequencerDataSource;
    use endpoints::{NamespaceProofQueryData, TimeWindowQueryData};
    use futures::stream::{StreamExt, TryStreamExt};
    use hotshot_query_service::availability::{BlockQueryData, LeafQueryData};
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use test_helpers::{
        state_signature_test_helper, state_test_helper, status_test_helper, submit_test_helper,
        TestNetwork,
    };
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

        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let storage = D::create_storage().await;
        let network = TestNetwork::new(
            D::options(&storage, options::Http { port }.into())
                .status(Default::default())
                .submit(Default::default()),
        )
        .await;
        let mut events = network.server.get_event_stream();

        // Connect client.
        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        let hash = client
            .post("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(txn.commit(), hash);

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(&mut events, &txn).await.unwrap();
        let mut found_txn = false;
        let mut found_empty_block = false;

        let block_height = client
            .get::<usize>("status/block-height")
            .send()
            .await
            .unwrap();

        for block_num in 0..block_height {
            let ns_query_res: NamespaceProofQueryData = client
                .get(&format!("availability/block/{block_num}/namespace/0"))
                .send()
                .await
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
    pub(crate) async fn test_restart<D: TestableSequencerDataSource>() {
        setup_logging();
        setup_backtrace();

        // Initialize nodes.
        let storage = D::create_storage().await;
        let port = pick_unused_port().unwrap();
        let mut network = TestNetwork::with_state(
            D::options(&storage, options::Http { port }.into()),
            D::connect(&storage).await,
        )
        .await;

        // Connect client.
        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

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

        // Fully shut down the API servers.
        drop(network);

        // Start up again, resuming from the last decided leaf.
        let port = pick_unused_port().expect("No ports free");
        let _network = TestNetwork::with_state(
            D::options(&storage, options::Http { port }.into()),
            D::connect(&storage).await,
        )
        .await;
        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        // Make sure we can decide new blocks after the restart.
        tracing::info!("waiting for decide, height {}", height + 1);
        let new_leaf: LeafQueryData<SeqTypes> = client
            .socket(&format!("availability/stream/leaves/{}", height + 1))
            .subscribe()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_leaf.height(), height as u64 + 1);
        assert_eq!(new_leaf.leaf().parent_commitment, chain[height - 1].hash());

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

    #[async_std::test]
    pub(crate) async fn test_timestamp_window<D: TestableSequencerDataSource>() {
        setup_logging();
        setup_backtrace();

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let storage = D::create_storage().await;
        let _network = TestNetwork::new(
            D::options(&storage, options::Http { port }.into()).status(Default::default()),
        )
        .await;

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
            loop {
                let block_height = client
                    .get::<usize>("status/block-height")
                    .send()
                    .await
                    .unwrap();
                if block_height > num_blocks {
                    break;
                }
                tracing::info!("waiting for block {num_blocks}, current height {block_height}");
                sleep(Duration::from_secs(1)).await;
            }

            let block: BlockQueryData<SeqTypes> = client
                .get(&format!("availability/block/{num_blocks}"))
                .send()
                .await
                .unwrap();
            let header = block.header().clone();
            if let Some(last_timestamp) = test_blocks.last_mut() {
                if last_timestamp[0].timestamp == header.timestamp {
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
                    assert!(prev.timestamp < start);
                }
            } else {
                // `prev` can only be `None` if the first block in the window is the genesis block.
                assert_eq!(res.from().unwrap(), 0);
            };
            for header in &res.window {
                assert!(start <= header.timestamp);
                assert!(header.timestamp < end);
                if let Some(prev) = prev {
                    assert!(prev.timestamp <= header.timestamp);
                }
                prev = Some(header);
            }
            if let Some(next) = &res.next {
                assert!(next.timestamp >= end);
                // If there is a `next`, there must be at least one previous block (either `prev`
                // itself or the last block if the window is nonempty), so we can `unwrap` here.
                assert!(next.timestamp >= prev.unwrap().timestamp);
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
        let start = test_blocks[1][0].timestamp;
        let end = start + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.window, test_blocks[1]);
        assert_eq!(res.next.unwrap(), test_blocks[2][0]);

        // Case 1: no `prev`, start of window is before genesis.
        let start = 0;
        let end = test_blocks[0][0].timestamp + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev, None);
        assert_eq!(res.window, test_blocks[0]);
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);

        // Case 2: no `next`, end of window is after the most recently sequenced block.
        let start = test_blocks[2][0].timestamp;
        let end = i64::MAX as u64;
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
        assert_eq!(more2.from().unwrap(), more.from().unwrap());
        assert_eq!(more2.prev, more.prev);
        assert_eq!(more2.next, more.next);
        assert_eq!(more2.window[..more.window.len()], more.window);

        // Case 3: the window is empty.
        let start = test_blocks[1][0].timestamp;
        let end = start;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);
        assert_eq!(res.window, vec![]);

        // Case 5: no relevant blocks are available yet.
        client
            .get::<TimeWindowQueryData>(&format!(
                "availability/headers/window/{}/{}",
                i64::MAX - 1,
                i64::MAX
            ))
            .send()
            .await
            .unwrap_err();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use test_helpers::{
        state_signature_test_helper, state_test_helper, status_test_helper, submit_test_helper,
        TestNetwork,
    };
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);
        let options = Options::from(options::Http { port });
        let _network = TestNetwork::new(options).await;

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
}
