use crate::{
    api::{
        endpoints::{AccountQueryData, BlocksFrontier},
        options, Options,
    },
    catchup::{mock::MockStateCatchup, StateCatchup},
    context::SequencerContext,
    network,
    persistence::{no_storage::NoStorage, SequencerPersistence},
    state::{BlockMerkleTree, ValidatedState},
    testing::{run_test_builder, wait_for_decide_on_handle, TestConfig},
    Transaction,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::sleep;
use committable::Committable;
use es_version::{SequencerVersion, SEQUENCER_VERSION};
use ethers::{prelude::Address, utils::Anvil};
use futures::{
    future::{join_all, FutureExt},
    stream::StreamExt,
};
use hotshot::types::{Event, EventType};
use hotshot_contract_adapter::jellyfish::u256_to_field;
use hotshot_contract_adapter::light_client::ParsedLightClientState;

use hotshot_types::{
    event::LeafInfo,
    light_client::{GenericPublicInput, StateSignatureRequestBody},
    traits::{
        metrics::NoMetrics,
        node_implementation::ConsensusTime,
        stake_table::{SnapshotVersion, StakeTableScheme},
    },
};
use itertools::izip;
use jf_primitives::merkle_tree::{MerkleCommitment, MerkleTreeScheme};
use portpicker::pick_unused_port;
use std::time::Duration;
use surf_disco::Client;
use tide_disco::error::ServerError;
use url::Url;

type F = ark_ed_on_bn254::Fq;

pub(crate) const STAKE_TABLE_CAPACITY_FOR_TEST: usize = 10;

pub struct TestNetwork<P: SequencerPersistence> {
    pub server: SequencerContext<network::Memory, P, SequencerVersion>,
    pub peers: Vec<SequencerContext<network::Memory, P, SequencerVersion>>,
    pub cfg: TestConfig,
}

impl<P: SequencerPersistence> TestNetwork<P> {
    pub async fn with_state(
        opt: Options,
        state: [ValidatedState; TestConfig::NUM_NODES],
        persistence: [P; TestConfig::NUM_NODES],
        catchup: [impl StateCatchup + 'static; TestConfig::NUM_NODES],
        l1: Url,
    ) -> Self {
        let mut cfg = TestConfig::default_with_l1(l1);

        let (builder_task, builder_url) = run_test_builder().await;

        cfg.set_builder_url(builder_url);

        let mut nodes = join_all(izip!(state, persistence, catchup).enumerate().map(
            |(i, (state, persistence, catchup))| {
                let opt = opt.clone();
                let cfg = &cfg;
                async move {
                    if i == 0 {
                        opt.serve(
                            |metrics| {
                                let cfg = cfg.clone();
                                async move {
                                    cfg.init_node(
                                        0,
                                        state,
                                        persistence,
                                        catchup,
                                        &*metrics,
                                        STAKE_TABLE_CAPACITY_FOR_TEST,
                                        SEQUENCER_VERSION,
                                    )
                                    .await
                                }
                                .boxed()
                            },
                            SEQUENCER_VERSION,
                        )
                        .await
                        .unwrap()
                    } else {
                        cfg.init_node(
                            i,
                            state,
                            persistence,
                            catchup,
                            &NoMetrics,
                            STAKE_TABLE_CAPACITY_FOR_TEST,
                            SEQUENCER_VERSION,
                        )
                        .await
                    }
                }
            },
        ))
        .await;

        let handle_0 = &nodes[0];

        // Hook the builder up to the event stream from the first node
        if let Some(builder_task) = builder_task {
            builder_task.start(Box::new(handle_0.get_event_stream()));
        }

        for ctx in &nodes {
            ctx.start_consensus().await;
        }

        let server = nodes.remove(0);
        let peers = nodes;

        Self { server, peers, cfg }
    }

    pub async fn new(opt: Options, persistence: [P; TestConfig::NUM_NODES], l1: Url) -> Self {
        Self::with_state(
            opt,
            Default::default(),
            persistence,
            std::array::from_fn(|_| MockStateCatchup::default()),
            l1,
        )
        .await
    }

    pub fn light_client_genesis(&self) -> ParsedLightClientState {
        let st = self.cfg.stake_table(STAKE_TABLE_CAPACITY_FOR_TEST);
        let (bls_comm, schnorr_comm, stake_comm) = st
            .commitment(SnapshotVersion::LastEpochStart)
            .expect("Commitment computation shouldn't fail.");
        let threshold = st.total_stake(SnapshotVersion::LastEpochStart).unwrap() * 2 / 3;

        let pi = vec![
            u256_to_field(threshold),
            F::from(0_u64), // Arbitrary value for view number
            F::from(0_u64), // Arbitrary value for block height
            F::from(0_u64), // Arbitrary value for state commitment
            F::from(0_u64), // Arbitrary value for fee ledger commitment
            bls_comm,
            schnorr_comm,
            stake_comm,
        ];
        let pi: GenericPublicInput<F> = pi.into();
        pi.into()
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
    let client: Client<ServerError, SequencerVersion> = Client::new(url);

    let anvil = Anvil::new().spawn();
    let l1 = anvil.endpoint().parse().unwrap();
    let options = opt(Options::from(options::Http { port }).status(Default::default()));
    let _network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;
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

    let txn = Transaction::new(Default::default(), vec![1, 2, 3, 4]);

    let port = pick_unused_port().expect("No ports free");

    let url = format!("http://localhost:{port}").parse().unwrap();
    let client: Client<ServerError, SequencerVersion> = Client::new(url);

    let anvil = Anvil::new().spawn();
    let l1 = anvil.endpoint().parse().unwrap();
    let options = opt(Options::from(options::Http { port }).submit(Default::default()));
    let network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;
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
    wait_for_decide_on_handle(&mut events, &txn).await;
}

/// Test the state signature API.
pub async fn state_signature_test_helper(opt: impl FnOnce(Options) -> Options) {
    setup_logging();
    setup_backtrace();

    let port = pick_unused_port().expect("No ports free");

    let url = format!("http://localhost:{port}").parse().unwrap();
    let client: Client<ServerError, SequencerVersion> = Client::new(url);

    let anvil = Anvil::new().spawn();
    let l1 = anvil.endpoint().parse().unwrap();
    let options = opt(Options::from(options::Http { port }));
    let network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;

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
    client
        .get::<StateSignatureRequestBody>(&format!("state-signature/block/{}", height))
        .send()
        .await
        .unwrap();
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
    let client: Client<ServerError, SequencerVersion> = Client::new(url);

    let anvil = Anvil::new().spawn();
    let l1 = anvil.endpoint().parse().unwrap();
    let options = opt(Options::from(options::Http { port }).catchup(Default::default()));
    let mut network = TestNetwork::new(options, [NoStorage; TestConfig::NUM_NODES], l1).await;
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
                .any(|LeafInfo { leaf, .. }| leaf.get_block_header().height > 2)
            {
                break;
            }
        }
    }

    // Stop consensus running on the node so we freeze the decided and undecided states.
    network.server.consensus_mut().shut_down().await;

    // Decided fee state: absent account.
    let res = client
        .get::<AccountQueryData>(&format!("catchup/account/{:x}", Address::default()))
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
    let view = leaf.get_view_number() + 1;
    let res = client
        .get::<AccountQueryData>(&format!(
            "catchup/{}/account/{:x}",
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
        .get::<BlocksFrontier>("catchup/blocks")
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
        .get::<BlocksFrontier>(&format!("catchup/{}/blocks", view.get_u64()))
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
