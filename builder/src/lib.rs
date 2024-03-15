#![allow(unused_imports)]
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use hotshot::{
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{MemoryStorage, NetworkingMetricsValue, WebServerNetwork},
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
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::election::Membership,
    traits::metrics::Metrics,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;

use hotshot_state_prover;
use jf_primitives::{
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::bls_over_bn254::VerKey,
};
use sequencer::{
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    l1_client::L1Client,
    //mock::MockStateCatchup,
    network,
    persistence::SequencerPersistence,
    state::FeeAccount,
    state::ValidatedState,
    state_signature::static_stake_table_commitment,
    BuilderParams,
    L1Params,
    NetworkParams,
    Node,
    NodeState,
    PrivKey,
    PubKey,
    SeqTypes,
    Storage,
};
use std::{alloc::System, any, fmt::Debug, mem, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};
use std::{net::Ipv4Addr, thread::Builder};

use tide_disco::{app, method::ReadState, App, Url};

type ElectionConfig = StaticElectionConfig;

pub struct BuilderContext<N: network::Type> {
    /// The consensus handle
    pub hotshot_handle: Consensus<N>,

    /// Index of this sequencer node
    pub node_index: u64,

    // An orchestrator to wait for before starting consensus.
    pub wait_for_orchestrator: Option<Arc<OrchestratorClient>>,
}
#[allow(unused_variables)]
pub async fn init_node(
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    builder_params: BuilderParams,
    l1_params: L1Params,
    //persistence: &mut impl SequencerPersistence,
) -> anyhow::Result<BuilderContext<network::Web>> {
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
        state_key_pair: state_key_pair,
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
        Arc::new(StatePeers::from_urls(network_params.state_peers)),
    );

    let hotshot_handle =
        init_hotshot(config.config, instance_state, networks, metrics, node_index).await;

    let builder_ctx = BuilderContext {
        hotshot_handle,
        node_index,
        wait_for_orchestrator: Some(Arc::new(orchestrator_client)),
    };
    Ok(builder_ctx)
}

impl<N: network::Type> BuilderContext<N> {
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
    instance_state: NodeState,
    networks: Networks<SeqTypes, Node<N>>,
    metrics: &dyn Metrics,
    node_id: u64,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let election_config = GeneralStaticCommittee::<SeqTypes, PubKey>::default_election_config(
        config.num_nodes_with_stake.get() as u64,
        config.num_nodes_without_stake as u64,
    );
    let membership = GeneralStaticCommittee::create_election(
        config.known_nodes_with_stake.clone(),
        election_config,
    );

    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };

    SystemContext::init(
        config.my_own_validator_config.public_key,
        config.my_own_validator_config.private_key.clone(),
        node_id as u64,
        config,
        MemoryStorage::empty(),
        memberships,
        networks,
        HotShotInitializer::from_genesis(instance_state).unwrap(),
        ConsensusMetricsValue::new(metrics),
    )
    .await
    .unwrap()
    .0
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
    pub struct TestConfig {
        config: HotShotConfig<PubKey, ElectionConfig>,
        priv_keys_staking_nodes: Vec<BLSPrivKey>,
        priv_keys_non_staking_nodes: Vec<BLSPrivKey>,
        staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_stake_entries: Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
        master_map: Arc<MasterMap<Message<SeqTypes>, PubKey>>,
        anvil: Arc<AnvilInstance>,
    }

    impl Default for TestConfig {
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
    pub fn mock_node_state() -> NodeState {
        NodeState::new(
            L1Client::new("http://localhost:3331".parse().unwrap(), Address::default()),
            FeeAccount::test_wallet(),
            MockStateCatchup::default(),
        )
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

    impl TestConfig {
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

            let node_state = mock_node_state(); //NodeState::new(L1Client::Default())

            init_hotshot(config, node_state, networks, metrics, i as u64).await
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
}

#[cfg(test)]
#[cfg(any(test, feature = "testing"))]
mod test {
    use self::testing::mock_node_state;

    use super::*;
    //use super::{transaction::ApplicationTransaction, vm::TestVm, *};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    use futures::StreamExt;
    use hotshot::types::EventType::Decide;

    use hotshot_types::event::LeafInfo;
    use hotshot_types::traits::block_contents::{
        vid_commitment, BlockHeader, BlockPayload, GENESIS_VID_NUM_STORAGE_NODES,
    };
    use sequencer::block::payload::Payload;
    use sequencer::Header;
    use testing::{wait_for_decide_on_handle, TestConfig};
    #[async_std::test]
    async fn test_non_voting_builder_node() {
        setup_logging();
        setup_backtrace();

        let success_height = 5;
        // Assign `config` so it isn't dropped early.
        let config = TestConfig::default();
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
            let genesis_state = mock_node_state();
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
}

/*
    #[cfg(test)]
    mod test {
        use core::num;

        use crate::testing::{init_hotshot_handles, wait_for_decide_on_handle};
        use async_broadcast::broadcast;
        use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
        use async_compatibility_layer::{
            art::async_spawn,
            channel::{unbounded, UnboundedReceiver},
        };
        use async_lock::RwLock;
        use commit::{Commitment, CommitmentBoundsArkless};
        use futures::{Stream, StreamExt};
        use hotshot::types::EventType::Decide;
        use hotshot::types::SignatureKey;
        // use hotshot_example_types::block_types::genesis_vid_commitment;
        use hotshot_types::{
            data::{Leaf, ViewNumber},
            signature_key::BLSPubKey,
            traits::block_contents::BlockHeader,
        };
        use hs_builder_api::builder::Options as BuilderApiOptions;
        use hs_builder_core::{
            builder_state::{BuilderProgress, BuilderState, MessageType},
            service::GlobalState,
        };
        use sequencer::{
            api, network,
            options::{Modules, Options as SeqOptions},
            NetworkParams, SeqTypes,
        };
        use sequencer::{Header, NodeState};
        use std::sync::{Arc, Mutex};

        #[async_std::test]
        async fn test_active_participating_builder() {
            // TODO
            /*
            // 1. Create a hothsoht memory network and get a handle
            // 2. Setup builder state
            // 3. Setup global state
            4. Get events form the handle and call the run_standalone_builder_service function.
                possiblly spwan a task for it to run.
            5. Make a surf disco client for the sumbit_txn api and see if the txn is being submitted or not?
            6. Create a surf disco client for the hotshot api's and do request submit/response check for it.
            7. Keep the success height as a parameter for the test to exit
            8. Check if the global state is being updated or not?
            9. Check if the builder state is being updated or not?
            */
            // setup logging and bactrace
            // setup logging and backtrace
            setup_logging();
            setup_backtrace();

            tracing::info!("Starting Test: functional_builder");

            let success_height = 5;
            let num_nodes = 5;
            let num_builders = 1;

            let handles = init_hotshot_handles(num_nodes, num_builders).await;

            // trying to listen events fron the builder handle
            // Currently its same as other nodes, but later it would be different
            // let mut events = handles[num_nodes + num_builders - 1].get_event_stream();

            // start consensus for all the nodes
            for handle in handles.iter() {
                handle.hotshot.start_consensus().await;
            }

            let mut builder_context_handle = handles[num_nodes + num_builders - 1];
            // Generate Builder specific requirements
            let seed = [201_u8; 32];

            // Builder Public, Private key
            let (builder_pub_key, builder_private_key) =
                BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

            // Required Channles for the hotshot events for the builder
            // tx channel
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

            // create the global state
            let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
                (builder_pub_key, builder_private_key),
                req_sender,
                res_receiver,
                tx_sender.clone(),
            );

            // create the arc_rwlock for the global state
            let arc_rwlock_global_state: Arc<RwLock<GlobalState<SeqTypes>>> =
                Arc::new(RwLock::new(global_state));

            // clone the global state for the builder state
            let arc_rwlock_global_state_clone: Arc<RwLock<GlobalState<SeqTypes>>> =
                arc_rwlock_global_state.clone();

            // create the builder state
            let builder_state = BuilderState::<SeqTypes>::new(
                (
                    ViewNumber::new(0),
                    vid_commitment(&vec![], 8),
                    Commitment::<Leaf<SeqTypes>>::default_commitment_no_preimage(),
                ),
                tx_receiver,
                decide_receiver,
                da_receiver,
                qc_receiver,
                req_receiver,
                arc_rwlock_global_state_clone,
                res_sender,
                NonZeroUsize::new(1).unwrap(),
            );
            async_spawn(async move {
                run_standalone_builder_service(
                    tx_sender,
                    da_sender,
                    qc_sender,
                    decide_sender,
                    builder_context_handle,
                )
                .await
                .unwrap();
            })
            .await;

            let port = portpicker::pick_unused_port().expect("Could not find an open port");
            let hotshot_api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

            let port = portpicker::pick_unused_port().expect("Could not find an open port");
            let submit_txn_api = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

            // get handle to the hotshot context
            let builder_context = init_node(network_params, &NoMetrics).await?;

            // start doing consensus i.e. in this case be passive member of the consensus network
            builder_context.start_consensus().await;
        }

}
*/
