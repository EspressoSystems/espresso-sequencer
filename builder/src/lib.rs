#![allow(unused_imports)]
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use futures::{
    future::{join_all, Future},
    stream::{Stream, StreamExt},
};
use hotshot::{
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{NetworkingMetricsValue, WebServerNetwork},
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    event::Event,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::election::Membership,
    traits::metrics::Metrics,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
use std::fmt::Display;
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;

use async_std::{
    sync::Arc,
    task::{spawn, JoinHandle},
};
use hotshot_state_prover;
use jf_primitives::{
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::bls_over_bn254::VerKey,
};
use sequencer::catchup::mock::MockStateCatchup;
use sequencer::{
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    l1_client::L1Client,
    network,
    persistence::SequencerPersistence,
    state::FeeAccount,
    state::ValidatedState,
    state_signature::{static_stake_table_commitment, StateSigner},
    BuilderParams, L1Params, NetworkParams, Node, NodeState, PrivKey, PubKey, SeqTypes,
};
use std::{alloc::System, any, fmt::Debug, mem};
use std::{marker::PhantomData, net::IpAddr};
use std::{net::Ipv4Addr, thread::Builder};
use tide_disco::{app, method::ReadState, App, Url};
use versioned_binary_serialization::version::StaticVersionType;
type ElectionConfig = StaticElectionConfig;

pub struct BuilderContext<N: network::Type, Ver: StaticVersionType + 'static> {
    /// The consensus handle
    pub hotshot_handle: Consensus<N>,

    /// Index of this sequencer node
    pub node_index: u64,

    /// Context for generating state signatures.
    state_signer: Arc<StateSigner<Ver>>,

    /// An orchestrator to wait for before starting consensus.
    pub wait_for_orchestrator: Option<Arc<OrchestratorClient>>,

    /// Background tasks to shut down when the node is dropped.
    tasks: Vec<(String, JoinHandle<()>)>,
}
#[allow(unused_variables)]
pub async fn init_node<Ver: StaticVersionType + 'static>(
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    builder_params: BuilderParams,
    l1_params: L1Params,
    bind_version: Ver,
    //persistence: &mut impl SequencerPersistence,
) -> anyhow::Result<BuilderContext<network::Web, Ver>> {
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        public_ip: None,
        network_config_file: None,
    };
    // This "public" IP only applies to libp2p network configurations, so we can supply any value here
    let public_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    // Orchestrator client
    let orchestrator_client = OrchestratorClient::new(validator_args, public_ip.to_string());

    let private_staking_key = network_params.private_staking_key.clone();
    let public_staking_key = BLSPubKey::from_private(&private_staking_key);
    let state_key_pair = StateKeyPair::from_sign_key(network_params.private_state_key);

    let my_config = ValidatorConfig {
        public_key: BLSPubKey::from_private(&network_params.private_staking_key),
        private_key: network_params.private_staking_key,
        stake_value: 1,
        state_key_pair: state_key_pair.clone(),
    };

    let wait_for_orchestrator = true;

    tracing::info!("loading network config from orchestrator");
    let mut config: NetworkConfig<VerKey, StaticElectionConfig> =
        NetworkConfig::get_complete_config(&orchestrator_client, my_config.clone(), None)
            .await
            .0;
    tracing::info!(
    node_id = config.node_index,
    stake_table = ?config.config.known_nodes_with_stake,
    "loaded config",
    );

    // Get updated config from orchestrator containing all peer's public keys
    // config = orchestrator_client
    //     .post_and_wait_all_public_keys(config.node_index, public_staking_key)
    //     .await;

    // config.config.my_own_validator_config.private_key = private_staking_key.clone();
    // config.config.my_own_validator_config.public_key = public_staking_key;
    let node_index = config.node_index;

    tracing::info!("loaded config, we are node {}", config.node_index);

    // Initialize networking.
    let networks = Networks {
        da_network: Arc::new(WebServerNetwork::create(
            network_params.da_server_url,
            network_params.webserver_poll_interval,
            my_config.public_key,
            true,
        )),
        quorum_network: Arc::new(WebServerNetwork::create(
            network_params.consensus_server_url,
            network_params.webserver_poll_interval,
            my_config.public_key,
            false,
        )),
        _pd: Default::default(),
    };

    // The web server network doesn't have any metrics. By creating and dropping a
    // `NetworkingMetricsValue`, we ensure the networking metrics are created, but just not
    // populated, so that monitoring software built to work with network-related metrics doesn't
    // crash horribly just because we're not using the P2P network yet.
    let _ = NetworkingMetricsValue::new(metrics);

    // creating the instance state without any builder mnemonic
    let wallet = MnemonicBuilder::<English>::default()
        .phrase::<&str>(&builder_params.mnemonic)
        .index(builder_params.eth_account_index)?
        .build()?;
    tracing::info!("Builder account address {:?}", wallet.address());

    let mut genesis_state = ValidatedState::default();
    for address in builder_params.prefunded_accounts {
        tracing::warn!("Prefunding account {:?} for demo", address);
        genesis_state.prefund_account(address.into(), U256::max_value().into());
    }

    let l1_client = L1Client::new(l1_params.url, Address::default());

    let instance_state = NodeState::new(
        l1_client,
        wallet,
        Arc::new(StatePeers::<Ver>::from_urls(network_params.state_peers)),
    );
    let stake_table_commit =
        static_stake_table_commitment(&config.config.known_nodes_with_stake, STAKE_TABLE_CAPACITY);
    let mut state_signer = StateSigner::new(state_key_pair, stake_table_commit);
    let hotshot_handle = init_hotshot(
        config.config,
        None,
        instance_state,
        networks,
        metrics,
        node_index,
    )
    .await;

    let state_relay_server = Some(network_params.state_relay_server_url);

    if let Some(url) = state_relay_server {
        state_signer = state_signer.with_relay_server(url);
    }
    // let builder_ctx = BuilderContext {
    //     hotshot_handle,
    //     node_index,
    //     state_signer,
    //     wait_for_orchestrator: Some(Arc::new(orchestrator_client)),
    // };
    // Ok(builder_ctx)
    Ok(BuilderContext::new(
        hotshot_handle,
        node_index,
        state_signer,
    ))
}

impl<N: network::Type, Ver: StaticVersionType + 'static> BuilderContext<N, Ver> {
    /// Constructor
    fn new(hotshot_handle: Consensus<N>, node_index: u64, state_signer: StateSigner<Ver>) -> Self {
        let events = hotshot_handle.get_event_stream();
        let mut ctx = Self {
            hotshot_handle,
            node_index,
            state_signer: Arc::new(state_signer),
            wait_for_orchestrator: None,
            tasks: vec![],
        };
        ctx.spawn("main event handler", handle_events(events));
        ctx
    }

    /// Spawn a background task attached to this context.
    ///
    /// When this context is dropped or [`shut_down`](Self::shut_down), background tasks will be
    /// cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future + Send + 'static) {
        let name = name.to_string();
        let task = {
            let name = name.clone();
            spawn(async move {
                task.await;
                tracing::info!(name, "background task exited");
            })
        };
        self.tasks.push((name, task));
    }

    /// Start participating in consensus.
    pub async fn start_consensus(&self) {
        if let Some(orchestrator_client) = &self.wait_for_orchestrator {
            tracing::info!("waiting for orchestrated start");
            orchestrator_client
                .wait_for_all_nodes_ready(self.node_index)
                .await;
        }
        self.hotshot_handle.hotshot.start_consensus().await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn init_hotshot<N: network::Type>(
    config: HotShotConfig<PubKey, ElectionConfig>,
    stake_table_entries_for_non_voting_nodes: Option<
        Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
    >,
    instance_state: NodeState,
    networks: Networks<SeqTypes, Node<N>>,
    metrics: &dyn Metrics,
    node_id: u64,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let election_config = GeneralStaticCommittee::<SeqTypes, PubKey>::default_election_config(
        config.num_nodes_with_stake.get() as u64,
        config.num_nodes_without_stake as u64,
    );
    let combined_known_nodes_with_stake = match stake_table_entries_for_non_voting_nodes {
        Some(stake_table_entries) => {
            let combined_entries = config
                .known_nodes_with_stake
                .iter()
                .cloned()
                .chain(stake_table_entries.into_iter())
                .collect();
            combined_entries
        }
        None => config.known_nodes_with_stake.clone(),
    };
    let membership =
        GeneralStaticCommittee::create_election(combined_known_nodes_with_stake, election_config);

    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };
    let da_storage = Default::default();
    SystemContext::init(
        config.my_own_validator_config.public_key,
        config.my_own_validator_config.private_key.clone(),
        node_id as u64,
        config,
        memberships,
        networks,
        HotShotInitializer::from_genesis(instance_state).unwrap(),
        ConsensusMetricsValue::new(metrics),
        da_storage,
    )
    .await
    .unwrap()
    .0
}

async fn handle_events(mut events: impl Stream<Item = Event<SeqTypes>> + Unpin) {
    // TODO: Fix this event handling
    while let Some(event) = events.next().await {
        tracing::debug!(?event, "consensus event");
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use commit::Committable;
    use core::num;
    use ethers::utils::{Anvil, AnvilInstance};
    use futures::{
        future::join_all,
        stream::{Stream, StreamExt},
    };
    use hotshot::traits::{
        implementations::{MasterMap, MemoryNetwork},
        BlockPayload,
    };
    use hotshot::types::{EventType::Decide, Message};
    use hotshot_types::{
        light_client::StateKeyPair,
        traits::{block_contents::BlockHeader, metrics::NoMetrics},
        ExecutionType, HotShotConfig, PeerConfig, ValidatorConfig,
    };
    //use sequencer::persistence::NoStorage;
    use hotshot_types::event::LeafInfo;
    use sequencer::catchup::StateCatchup;
    use sequencer::{Event, Transaction};
    use std::{num::NonZeroUsize, time::Duration};
    #[derive(Clone)]
    pub struct HotShotTestConfig {
        pub config: HotShotConfig<PubKey, ElectionConfig>,
        priv_keys_staking_nodes: Vec<BLSPrivKey>,
        priv_keys_non_staking_nodes: Vec<BLSPrivKey>,
        staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_stake_entries: Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
        master_map: Arc<MasterMap<Message<SeqTypes>, PubKey>>,
        anvil: Arc<AnvilInstance>,
    }

    impl Default for HotShotTestConfig {
        fn default() -> Self {
            let num_nodes_with_stake = Self::NUM_NODES;
            let num_nodes_without_stake = Self::BUILDER_NODES;

            // first generate stake table entries for the staking nodes
            let (priv_keys_staking_nodes, staking_nodes_state_key_pairs, known_nodes_with_stake) =
                genereate_stake_table_entries(num_nodes_with_stake as u64, 1);
            // Now generate the stake table entries for the non-staking nodes
            let (
                priv_keys_non_staking_nodes,
                non_staking_nodes_state_key_pairs,
                known_nodes_without_stake,
            ) = genereate_stake_table_entries(num_nodes_without_stake as u64, 0);

            // get the pub key out of the stake table entry for the non-staking nodes
            // Only pass the pub keys to the hotshot config
            let known_nodes_without_stake_pub_keys = known_nodes_without_stake
                .iter()
                .map(|x| <BLSPubKey as SignatureKey>::get_public_key(&x.stake_table_entry))
                .collect::<Vec<_>>();

            let master_map = MasterMap::new();

            let config: HotShotConfig<PubKey, ElectionConfig> = HotShotConfig {
                execution_type: ExecutionType::Continuous,
                num_nodes_with_stake: NonZeroUsize::new(num_nodes_with_stake).unwrap(),
                num_nodes_without_stake: num_nodes_without_stake,
                min_transactions: 1,
                max_transactions: 10000.try_into().unwrap(),
                known_nodes_with_stake,
                known_nodes_without_stake: known_nodes_without_stake_pub_keys,
                next_view_timeout: Duration::from_secs(5).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                propose_min_round_time: Duration::from_secs(0),
                propose_max_round_time: Duration::from_secs(1),
                election_config: None,
                da_staked_committee_size: num_nodes_with_stake,
                da_non_staked_committee_size: num_nodes_without_stake,
                my_own_validator_config: Default::default(),
                data_request_delay: Duration::from_millis(200),
                view_sync_timeout: Duration::from_secs(5),
            };

            Self {
                config,
                priv_keys_staking_nodes,
                priv_keys_non_staking_nodes,
                staking_nodes_state_key_pairs,
                non_staking_nodes_state_key_pairs,
                non_staking_nodes_stake_entries: known_nodes_without_stake,
                master_map,
                anvil: Arc::new(Anvil::new().spawn()),
            }
        }
    }
    pub fn genereate_stake_table_entries(
        num_nodes: u64,
        stake_value: u64,
    ) -> (Vec<BLSPrivKey>, Vec<StateKeyPair>, Vec<PeerConfig<PubKey>>) {
        // Generate keys for the nodes.
        let priv_keys = (0..num_nodes)
            .map(|_| PrivKey::generate(&mut rand::thread_rng()))
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(PubKey::from_private)
            .collect::<Vec<_>>();
        let state_key_pairs = (0..num_nodes)
            .map(|_| StateKeyPair::generate())
            .collect::<Vec<_>>();

        let nodes_with_stake = pub_keys
            .iter()
            .zip(&state_key_pairs)
            .map(|(pub_key, state_key_pair)| PeerConfig::<PubKey> {
                stake_table_entry: pub_key.get_stake_table_entry(stake_value),
                state_ver_key: state_key_pair.ver_key(),
            })
            .collect::<Vec<_>>();

        (priv_keys, state_key_pairs, nodes_with_stake)
    }

    impl HotShotTestConfig {
        pub const NUM_NODES: usize = 4;
        pub const BUILDER_NODES: usize = 2;

        pub fn num_staked_nodes(&self) -> usize {
            self.priv_keys_staking_nodes.len()
        }
        pub fn num_non_staked_nodes(&self) -> usize {
            self.priv_keys_non_staking_nodes.len()
        }
        pub fn total_staking_not_staking_nodes(&self) -> usize {
            self.num_staked_nodes() + self.num_non_staked_nodes()
        }
        pub fn total_nodes() -> usize {
            Self::NUM_NODES + Self::BUILDER_NODES
        }
        pub fn get_anvil(&self) -> Arc<AnvilInstance> {
            self.anvil.clone()
        }
        pub fn get_validator_config(
            &self,
            i: usize,
            is_staked: bool,
        ) -> ValidatorConfig<hotshot_state_prover::QCVerKey> {
            if is_staked {
                ValidatorConfig {
                    public_key: self.config.known_nodes_with_stake[i]
                        .stake_table_entry
                        .stake_key,
                    private_key: self.priv_keys_staking_nodes[i].clone(),
                    stake_value: self.config.known_nodes_with_stake[i]
                        .stake_table_entry
                        .stake_amount
                        .as_u64(),
                    state_key_pair: self.staking_nodes_state_key_pairs[i].clone(),
                }
            } else {
                ValidatorConfig {
                    public_key: self.config.known_nodes_without_stake[i],
                    private_key: self.priv_keys_non_staking_nodes[i].clone(),
                    stake_value: 0,
                    state_key_pair: self.non_staking_nodes_state_key_pairs[i].clone(),
                }
            }
        }

        pub async fn init_nodes(
            &self,
        ) -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>> {
            let num_staked_nodes = self.num_staked_nodes();
            let mut is_staked = false;
            join_all((0..self.total_staking_not_staking_nodes()).map(|i| {
                if i < num_staked_nodes {
                    is_staked = true;
                } else {
                    is_staked = false;
                }
                async move { self.init_node(i, is_staked, &NoMetrics).await }
            }))
            .await
        }

        pub async fn init_node(
            &self,
            i: usize,
            is_staked: bool,
            metrics: &dyn Metrics,
        ) -> SystemContextHandle<SeqTypes, Node<network::Memory>> {
            let mut config = self.config.clone();

            let num_staked_nodes = self.num_staked_nodes();
            if is_staked {
                config.my_own_validator_config = self.get_validator_config(i, is_staked);
            } else {
                config.my_own_validator_config =
                    self.get_validator_config(i - num_staked_nodes, is_staked);
            }

            let network = Arc::new(MemoryNetwork::new(
                config.my_own_validator_config.public_key,
                NetworkingMetricsValue::new(metrics),
                self.master_map.clone(),
                None,
            ));
            let networks = Networks {
                da_network: network.clone(),
                quorum_network: network,
                _pd: Default::default(),
            };

            //let node_state = NodeState::mock(); //NodeState::new(L1Client::Default())
            // TODO: I think not every node will have wallet, since only builder nodes
            let wallet = Self::builder_wallet(i);
            tracing::info!("node {i} is builder {:x}", wallet.address());
            let node_state = NodeState::new(
                L1Client::new(self.anvil.endpoint().parse().unwrap(), Address::default()),
                wallet,
                MockStateCatchup::default(),
            )
            .with_genesis(ValidatedState::default());

            init_hotshot(
                config,
                Some(self.non_staking_nodes_stake_entries.clone()),
                node_state,
                networks,
                metrics,
                i as u64,
            )
            .await
        }

        pub fn builder_wallet(i: usize) -> Wallet<SigningKey> {
            MnemonicBuilder::<English>::default()
                .phrase("test test test test test test test test test test test junk")
                .index(i as u32)
                .unwrap()
                .build()
                .unwrap()
        }
    }
    // Wait for decide event, make sure it matches submitted transaction. Return the block number
    // containing the transaction.
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> u64 {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");

            if let Decide { leaf_chain, .. } = event.event {
                if let Some(height) = leaf_chain.iter().find_map(|LeafInfo { leaf, .. }| {
                    if leaf
                        .block_payload
                        .as_ref()?
                        .transaction_commitments(leaf.get_block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.get_block_header().block_number())
                    } else {
                        None
                    }
                }) {
                    return height;
                }
            } else {
                // Keep waiting
            }
        }
    }

    use async_broadcast::{
        broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender,
        TryRecvError,
    };
    use async_compatibility_layer::channel::unbounded;
    use async_compatibility_layer::{
        art::{async_sleep, async_spawn},
        channel::{UnboundedReceiver, UnboundedSender},
    };
    use async_lock::RwLock;
    use hotshot_builder_core::{
        builder_state::{BuildBlockInfo, BuilderState, MessageType, ResponseMessage},
        service::GlobalState,
    };
    use hotshot_types::{
        data::{fake_commitment, Leaf, ViewNumber},
        traits::{
            block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
            node_implementation::ConsensusTime,
        },
    };

    pub struct BuilderTestConfig {
        pub global_state: Arc<RwLock<GlobalState<SeqTypes>>>,
        pub builder_state: BuilderState<SeqTypes>,
        pub hotshot_test_config: HotShotTestConfig,
        pub tx_sender: BroadcastSender<MessageType<SeqTypes>>,
        pub da_sender: BroadcastSender<MessageType<SeqTypes>>,
        pub qc_sender: BroadcastSender<MessageType<SeqTypes>>,
        pub decide_sender: BroadcastSender<MessageType<SeqTypes>>,
        pub instance_state: NodeState,
    }

    use async_trait::async_trait;
    use hotshot_builder_api::builder::Options as HotshotBuilderApiOptions;
    use hotshot_builder_api::builder::{BuildError, Error as BuilderApiError};
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptioins},
        events_source::EventsStreamer,
    };
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use serde::{Deserialize, Serialize};
    use snafu::*;

    pub fn run_builder_apis_for_hotshot(url: Url, source: Arc<RwLock<GlobalState<SeqTypes>>>) {
        let hotshot_api = hotshot_builder_api::builder::define_api::<
            Arc<RwLock<GlobalState<SeqTypes>>>,
            SeqTypes,
            Version01,
        >(&HotshotBuilderApiOptions::default())
        .expect("Failed to construct the builder APIs for hotshot");

        let mut app: App<Arc<RwLock<GlobalState<SeqTypes>>>, BuilderApiError, Version01> =
            App::with_state(source);

        app.register_module("hotshot_builder", hotshot_api)
            .expect("Failed to register the builder API for hotshot");

        async_spawn(app.serve(url, STATIC_VER_0_1));
    }

    pub fn run_builder_api_to_receive_private_txns(
        url: Url,
        source: Arc<RwLock<GlobalState<SeqTypes>>>,
    ) {
        let private_mempool_api = hotshot_builder_api::builder::submit_api::<
            Arc<RwLock<GlobalState<SeqTypes>>>,
            SeqTypes,
            Version01,
        >(&HotshotBuilderApiOptions::default())
        .expect("Failed to construct the builder API for private mempool txns");

        let mut app: App<Arc<RwLock<GlobalState<SeqTypes>>>, BuilderApiError, Version01> =
            App::with_state(source);

        app.register_module("builder_private_mempool", private_mempool_api)
            .expect("Failed to register the builder API for private mempool txns");

        async_spawn(app.serve(url, STATIC_VER_0_1));
    }

    pub fn run_hotshot_event_streaming_api(
        url: Url,
        source: Arc<RwLock<EventsStreamer<SeqTypes>>>,
    ) {
        // Start the web server.
        let hotshot_events_api = hotshot_events_service::events::define_api::<
            Arc<RwLock<EventsStreamer<SeqTypes>>>,
            SeqTypes,
            Version01,
        >(&EventStreamingApiOptioins::default())
        .expect("Failed to define hotshot eventsAPI");

        let mut app = App::<_, EventStreamApiError, Version01>::with_state(source);

        app.register_module("hotshot_events", hotshot_events_api)
            .expect("Failed to register hotshot events API");

        async_spawn(app.serve(url, STATIC_VER_0_1));
    }

    impl BuilderTestConfig {
        pub const BUILDER_ID: usize = 5;
        pub fn new() -> Self {
            let (tx_sender, tx_receiver) = broadcast::<MessageType<SeqTypes>>(15);
            // da channel
            let (da_sender, da_receiver) = broadcast::<MessageType<SeqTypes>>(15);
            // qc channel
            let (qc_sender, qc_receiver) = broadcast::<MessageType<SeqTypes>>(15);
            // decide channel
            let (decide_sender, decide_receiver) = broadcast::<MessageType<SeqTypes>>(15);

            // api request channel
            let (req_sender, req_receiver) = broadcast::<MessageType<SeqTypes>>(15);
            // response channel
            let (res_sender, res_receiver) = unbounded();

            let config = HotShotTestConfig::default();
            // Hotshot Test Config
            let wallet = HotShotTestConfig::builder_wallet(Self::BUILDER_ID);
            tracing::info!(
                "node {} is builder {:x}",
                Self::BUILDER_ID,
                wallet.address()
            );
            let node_state = NodeState::new(
                L1Client::new(
                    config.get_anvil().endpoint().parse().unwrap(),
                    Address::default(),
                ),
                wallet,
                MockStateCatchup::default(),
            )
            .with_genesis(ValidatedState::default());

            let seed = [201_u8; 32];
            // Builder Public, Private key
            let (builder_pub_key, builder_private_key) =
                BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

            // create the global state
            let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
                (builder_pub_key, builder_private_key),
                req_sender,
                res_receiver,
                tx_sender.clone(),
                node_state.clone(),
            );
            let global_state = Arc::new(RwLock::new(global_state));
            let global_state_clone = global_state.clone();

            let bootstrapped_view = ViewNumber::new(0);
            let builder_state = BuilderState::<SeqTypes>::new(
                (
                    bootstrapped_view,
                    vid_commitment(&vec![], GENESIS_VID_NUM_STORAGE_NODES),
                    fake_commitment(),
                ),
                tx_receiver,
                decide_receiver,
                da_receiver,
                qc_receiver,
                req_receiver,
                global_state_clone,
                res_sender,
                NonZeroUsize::new(1).unwrap(),
                bootstrapped_view,
            );
            Self {
                global_state: global_state,
                builder_state: builder_state,
                hotshot_test_config: config,
                tx_sender: tx_sender,
                da_sender: da_sender,
                qc_sender: qc_sender,
                decide_sender: decide_sender,
                instance_state: node_state,
            }
        }
    }
}

#[cfg(test)]
mod test {
    //use self::testing::mock_node_state;

    use super::*;
    //use super::{transaction::ApplicationTransaction, vm::TestVm, *};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    use async_std::stream::IntoStream;
    use clap::builder;
    use ethers::providers::Quorum;
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;

    use hotshot_builder_api::block_info::AvailableBlockData;
    use hotshot_builder_core::service::GlobalState;
    use hotshot_types::event::LeafInfo;
    use hotshot_types::traits::block_contents::{
        vid_commitment, BlockHeader, BlockPayload, GENESIS_VID_NUM_STORAGE_NODES,
    };
    use hotshot_types::utils::BuilderCommitment;
    use sequencer::block::payload::Payload;
    use sequencer::Header;
    use testing::{wait_for_decide_on_handle, HotShotTestConfig};

    // Test that a non-voting builder node can participate in consensus and reach a certain height.
    // It is enabled by keeping the builder(s) in the stake table, but with a stake of 0.
    // This is useful for testing that the builder can participate in consensus without voting.
    #[async_std::test]
    async fn test_non_voting_builder_node() {
        setup_logging();
        setup_backtrace();

        let success_height = 5;
        // Assign `config` so it isn't dropped early.
        let config = HotShotTestConfig::default();
        let handles = config.init_nodes().await;

        // try to listen on builder handle as it is the last handle
        let mut events = handles[5].get_event_stream();
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let mut parent = {
            // TODO refactor repeated code from other tests
            let (genesis_payload, genesis_ns_table) = Payload::genesis();
            let genesis_commitment = {
                // TODO we should not need to collect payload bytes just to compute vid_commitment
                let payload_bytes = genesis_payload
                    .encode()
                    .expect("unable to encode genesis payload")
                    .collect();
                vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
            };
            let genesis_state = NodeState::mock();
            Header::genesis(&genesis_state, genesis_commitment, genesis_ns_table)
        };

        loop {
            let event = events.next().await.unwrap();
            tracing::debug!("Received event from handle: {event:?}");
            let Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            tracing::info!("Got decide {leaf_chain:?}");

            // Check that each successive header satisfies invariants relative to its parent: all
            // the fields which should be monotonic are.
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let header = leaf.block_header.clone();
                if header.height == 0 {
                    parent = header;
                    continue;
                }
                assert_eq!(header.height, parent.height + 1);
                assert!(header.timestamp >= parent.timestamp);
                assert!(header.l1_head >= parent.l1_head);
                assert!(header.l1_finalized >= parent.l1_finalized);
                parent = header;
            }

            if parent.height >= success_height {
                break;
            }
        }
    }

    #[async_std::test]
    async fn test_permissioned_builder_core() {
        use async_compatibility_layer::art::{async_sleep, async_spawn};
        use async_std::task;
        use hotshot_builder_api::{
            block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
            builder::BuildError,
        };
        use hotshot_builder_core::builder_state::BuilderProgress;
        use hotshot_builder_core::service::run_permissioned_standalone_builder_service;
        use hotshot_types::constants::{Version01, STATIC_VER_0_1};
        use hotshot_types::traits::{
            block_contents::GENESIS_VID_NUM_STORAGE_NODES, node_implementation::NodeType,
        };
        use sequencer::transaction::Transaction;
        use std::time::Duration;
        use surf_disco::Client;
        use testing::{
            run_builder_api_to_receive_private_txns, run_builder_apis_for_hotshot,
            BuilderTestConfig,
        };

        setup_logging();
        setup_backtrace();

        let builder_test_config = BuilderTestConfig::new();
        // Get the handle for all the nodes, including both the non-builder and builder nodes
        let handles = builder_test_config.hotshot_test_config.init_nodes().await;
        // start consensus for all the nodes
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }
        let builder_context_handle = handles[BuilderTestConfig::BUILDER_ID].clone();

        //spawn the builder service
        async_spawn(async move {
            run_permissioned_standalone_builder_service(
                builder_test_config.tx_sender,
                builder_test_config.da_sender,
                builder_test_config.qc_sender,
                builder_test_config.decide_sender,
                builder_context_handle,
                builder_test_config.instance_state,
            )
            .await;
        });
        // spawn the builder event loop
        async_spawn(async move {
            builder_test_config.builder_state.event_loop();
        });

        // Run the builder apis to serve hotshot
        let port = portpicker::pick_unused_port().expect("Could not find an open port for hotshot");

        let hotshot_api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        run_builder_apis_for_hotshot(
            hotshot_api_url.clone(),
            builder_test_config.global_state.clone(),
        );

        // Run the builder apis to serve private mempool txns
        let port = portpicker::pick_unused_port()
            .expect("Could not find an open port for private mempool txns");

        let private_mempool_api_url =
            Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        // let global_state_txns_submitter = GlobalStateTxnSubmitter {
        //     global_state: builder_test_config.global_state.clone(),
        // };
        run_builder_api_to_receive_private_txns(
            private_mempool_api_url.clone(),
            builder_test_config.global_state,
        );

        // Start a hotshot client.
        let hotshot_client =
            Client::<hotshot_builder_api::builder::Error, Version01>::new(hotshot_api_url);
        assert!(hotshot_client.connect(Some(Duration::from_secs(60))).await);

        // Start a private mempool client.
        let private_mempool_client =
            Client::<hotshot_builder_api::builder::Error, Version01>::new(private_mempool_api_url);
        assert!(
            private_mempool_client
                .connect(Some(Duration::from_secs(60)))
                .await
        );

        let parent_commitment = vid_commitment(&vec![], GENESIS_VID_NUM_STORAGE_NODES);

        let response = loop {
            // Test getting available blocks
            match hotshot_client
                .get::<Vec<AvailableBlockInfo<SeqTypes>>>(&format!(
                    "hotshot_builder/availableblocks/{parent_commitment}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Available Blocks: {:?}", response);
                    assert!(!response.is_empty());
                    tracing::info!("Exiting from first loop");
                    break response;
                }
                Err(e) => {
                    tracing::warn!("Error getting available blocks {:?}", e);
                }
            };
        };

        let builder_commitment = response[0].block_hash.clone();
        let seed = [207_u8; 32];
        // Builder Public, Private key
        let (_hotshot_client_pub_key, hotshot_client_private_key) =
            BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

        // sign the builder_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            builder_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        // Test claiming blocks
        let _response = loop {
            match hotshot_client
                .get::<AvailableBlockData<SeqTypes>>(&format!(
                    "hotshot_builder/claimblock/{builder_commitment}/{encoded_signature}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Block Data: {:?}", response);
                    tracing::info!("Exiting from second loop");
                    break response;
                }
                Err(e) => {
                    panic!("Error while claiming block {:?}", e);
                }
            }
        };

        // Test claiming blocks
        let _response = loop {
            match hotshot_client
                .get::<AvailableBlockHeaderInput<SeqTypes>>(&format!(
                    "hotshot_builder/claimheaderinput/{builder_commitment}/{encoded_signature}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Block Header : {:?}", response);
                    assert!(true);
                    tracing::info!("Exiting from third loop");
                    break response;
                }
                Err(e) => {
                    panic!("Error getting claiming block header {:?}", e);
                }
            }
        };

        // Test submitting transactions
        let txn = Transaction::new(Default::default(), vec![1, 2, 3]);
        match private_mempool_client
            .post::<()>("builder_private_mempool/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                //let blocks = response.body().await.unwrap();
                println!("Received txn submitted response : {:?}", response);
                assert!(true);
                return;
            }
            Err(e) => {
                panic!("Error submitting private transaction {:?}", e);
            }
        }

        //task::sleep(std::time::Duration::from_secs(200)).await;
    }

    #[async_std::test]
    async fn test_permission_less_builder_core() {
        use async_compatibility_layer::art::{async_sleep, async_spawn};
        use async_std::task;
        use hotshot_builder_api::{
            block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
            builder::BuildError,
        };
        use hotshot_builder_core::builder_state::BuilderProgress;
        use hotshot_builder_core::service::{
            run_non_permissioned_standalone_builder_service,
            run_permissioned_standalone_builder_service,
        };
        use hotshot_types::constants::{Version01, STATIC_VER_0_1};
        use hotshot_types::traits::{
            block_contents::GENESIS_VID_NUM_STORAGE_NODES, node_implementation::NodeType,
        };
        use sequencer::transaction::Transaction;
        use std::time::Duration;
        use surf_disco::Client;
        use testing::{
            run_builder_api_to_receive_private_txns, run_builder_apis_for_hotshot,
            run_hotshot_event_streaming_api, BuilderTestConfig,
        };

        use async_lock::RwLock;
        use hotshot_events_service::{
            events::{Error as EventStreamApiError, Options as EventStreamingApiOptioins},
            events_source::{BuilderEvent, EventConsumer, EventsStreamer},
        };

        setup_logging();
        setup_backtrace();

        let builder_test_config = BuilderTestConfig::new();
        // Get the handle for all the nodes, including both the non-builder and builder nodes
        let handles = builder_test_config.hotshot_test_config.init_nodes().await;
        // start consensus for all the nodes
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }
        let builder_context_handle = handles[BuilderTestConfig::BUILDER_ID].clone();

        // get the required stuff for the election config
        let known_nodes_with_stake = builder_test_config
            .hotshot_test_config
            .config
            .known_nodes_with_stake
            .clone();

        // get count of non-staking nodes
        let num_non_staking_nodes = builder_test_config
            .hotshot_test_config
            .config
            .num_nodes_without_stake;

        // create a event streamer
        let events_streamer = Arc::new(RwLock::new(EventsStreamer::new(
            known_nodes_with_stake,
            num_non_staking_nodes,
        )));

        // spawn the event streaming api
        let port = portpicker::pick_unused_port().expect("Could not find an open port for hotshot");

        let hotshot_events_streaming_api_url =
            Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        run_hotshot_event_streaming_api(hotshot_events_streaming_api_url, events_streamer.clone());

        // create a client for it
        // Start Client for the event streaming api
        let client = Client::<EventStreamApiError, Version01>::new(
            format!("http://localhost:{}/hotshot_events", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        tracing::info!("event streaming client connected to server");

        // client subscrive to hotshot events
        let subscribed_events: surf_disco::socket::Connection<
            BuilderEvent<SeqTypes>,
            surf_disco::socket::Unsupported,
            EventStreamApiError,
            versioned_binary_serialization::version::StaticVersion<0, 1>,
        > = client
            .socket("events")
            .subscribe::<BuilderEvent<SeqTypes>>()
            .await
            .unwrap();

        tracing::info!("Client 1 Subscribed to events");

        // send the events to the event streaming api
        async_spawn({
            async move {
                let mut hotshot_event_stream = builder_context_handle.get_event_stream();
                loop {
                    let event = hotshot_event_stream.next().await.unwrap();
                    tracing::info!("Received event from handle/ before writing: {event:?}");
                    events_streamer.write().await.handle_event(event).await;
                    tracing::info!("Event written to the event streamer");
                }
            }
        });

        // spawn the builder service
        async_spawn(async move {
            run_non_permissioned_standalone_builder_service(
                builder_test_config.tx_sender,
                builder_test_config.da_sender,
                builder_test_config.qc_sender,
                builder_test_config.decide_sender,
                subscribed_events,
                builder_test_config.instance_state,
            )
            .await;
        });

        // spawn the builder event loop
        async_spawn(async move {
            builder_test_config.builder_state.event_loop();
        });

        // Run the builder apis to serve hotshot
        let port = portpicker::pick_unused_port().expect("Could not find an open port for hotshot");

        let hotshot_api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        run_builder_apis_for_hotshot(
            hotshot_api_url.clone(),
            builder_test_config.global_state.clone(),
        );

        // Run the builder apis to serve private mempool txns
        let port = portpicker::pick_unused_port()
            .expect("Could not find an open port for private mempool txns");

        let private_mempool_api_url =
            Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        run_builder_api_to_receive_private_txns(
            private_mempool_api_url.clone(),
            builder_test_config.global_state,
        );

        // Start a hotshot client.
        let hotshot_client =
            Client::<hotshot_builder_api::builder::Error, Version01>::new(hotshot_api_url);
        assert!(hotshot_client.connect(Some(Duration::from_secs(60))).await);

        // Start a private mempool client.
        let private_mempool_client =
            Client::<hotshot_builder_api::builder::Error, Version01>::new(private_mempool_api_url);
        assert!(
            private_mempool_client
                .connect(Some(Duration::from_secs(60)))
                .await
        );

        let parent_commitment = vid_commitment(&vec![], GENESIS_VID_NUM_STORAGE_NODES);

        let response = loop {
            // Test getting available blocks
            match hotshot_client
                .get::<Vec<AvailableBlockInfo<SeqTypes>>>(&format!(
                    "hotshot_builder/availableblocks/{parent_commitment}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Available Blocks: {:?}", response);
                    assert!(!response.is_empty());
                    tracing::info!("Exiting from first loop");
                    break response;
                }
                Err(e) => {
                    tracing::warn!("Error getting available blocks {:?}", e);
                }
            };
        };

        let builder_commitment = response[0].block_hash.clone();
        let seed = [207_u8; 32];
        // Builder Public, Private key
        let (_hotshot_client_pub_key, hotshot_client_private_key) =
            BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

        // sign the builder_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            builder_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        // Test claiming blocks
        let _response = loop {
            match hotshot_client
                .get::<AvailableBlockData<SeqTypes>>(&format!(
                    "hotshot_builder/claimblock/{builder_commitment}/{encoded_signature}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Block Data: {:?}", response);
                    tracing::info!("Exiting from second loop");
                    break response;
                }
                Err(e) => {
                    panic!("Error while claiming block {:?}", e);
                }
            }
        };

        // Test claiming blocks
        let _response = loop {
            match hotshot_client
                .get::<AvailableBlockHeaderInput<SeqTypes>>(&format!(
                    "hotshot_builder/claimheaderinput/{builder_commitment}/{encoded_signature}"
                ))
                .send()
                .await
            {
                Ok(response) => {
                    //let blocks = response.body().await.unwrap();
                    println!("Received Block Header : {:?}", response);
                    assert!(true);
                    tracing::info!("Exiting from third loop");
                    break response;
                }
                Err(e) => {
                    panic!("Error getting claiming block header {:?}", e);
                }
            }
        };

        let txn = Transaction::new(Default::default(), vec![1, 2, 3]);
        match private_mempool_client
            .post::<()>("builder_private_mempool/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                //let blocks = response.body().await.unwrap();
                println!("Received txn submitted response : {:?}", response);
                assert!(true);
                return;
            }
            Err(e) => {
                panic!("Error submitting private transaction {:?}", e);
            }
        }

        //task::sleep(std::time::Duration::from_secs(200)).await;
    }
}
