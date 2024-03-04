#![allow(unused_imports)]
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
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::metrics::Metrics,
    HotShotConfig,
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;

use jf_primitives::{
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::bls_over_bn254::VerKey,
};
use sequencer::{
    context::{Consensus, SequencerContext},
    network,
    persistence::SequencerPersistence,
    state_signature::static_stake_table_commitment,
    NetworkParams, Node, NodeState, PrivKey, PubKey, SeqTypes, Storage,
};
use std::{alloc::System, any, fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};
use std::{net::Ipv4Addr, thread::Builder};

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
    //persistence: &mut impl SequencerPersistence,
) -> anyhow::Result<BuilderContext<network::Web>> {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        public_ip: None,
        network_config_file: None,
    };
    // This "public" IP only applies to libp2p network configurations, so we can supply any value here
    let public_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    // surf disco client
    let orchestrator_client = OrchestratorClient::new(validator_args, public_ip.to_string());

    let private_staking_key = network_params.private_staking_key;
    let public_staking_key = BLSPubKey::from_private(&private_staking_key);

    let wait_for_orchestrator = true;

    tracing::info!("loading network config from orchestrator");
    let mut config: NetworkConfig<VerKey, StaticElectionConfig> =
        orchestrator_client.get_config(public_ip.to_string()).await;

    // Get updated config from orchestrator containing all peer's public keys
    config = orchestrator_client
        .post_and_wait_all_public_keys(config.node_index, public_staking_key)
        .await;

    config.config.my_own_validator_config.private_key = private_staking_key.clone();
    config.config.my_own_validator_config.public_key = public_staking_key;

    tracing::info!("loaded config, we are node {}", config.node_index);

    let node_index = config.node_index;
    let num_nodes = config.config.total_nodes.get();

    let known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry> =
        config.config.known_nodes_with_stake.clone();

    let pub_keys = known_nodes_with_stake
        .iter()
        .map(|entry| entry.stake_key)
        .collect::<Vec<_>>();

    // TODO: fetch from orchestrator?
    let state_ver_keys = (0..num_nodes)
        .map(|i| StateKeyPair::generate_from_seed_indexed(config.seed, i as u64).ver_key())
        .collect::<Vec<_>>();

    let state_key_pair = config.config.my_own_validator_config.state_key_pair.clone();

    // Initialize networking.
    let networks = Networks {
        da_network: Arc::new(WebServerNetwork::create(
            network_params.da_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
            true,
        )),
        quorum_network: Arc::new(WebServerNetwork::create(
            network_params.consensus_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
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
    let instance_state = &NodeState::default();

    let hotshot_handle = init_hotshot(
        pub_keys.clone(),
        known_nodes_with_stake.clone(),
        node_index as usize,
        private_staking_key,
        networks,
        config.config,
        metrics,
        instance_state,
    )
    .await;
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
    nodes_pub_keys: Vec<PubKey>,
    known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry>,
    node_id: usize,
    priv_key: PrivKey,
    networks: Networks<SeqTypes, Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
    metrics: &dyn Metrics,
    instance_state: &NodeState,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let membership = GeneralStaticCommittee::new(&nodes_pub_keys, known_nodes_with_stake.clone());
    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };

    SystemContext::init(
        nodes_pub_keys[node_id],
        priv_key,
        node_id as u64,
        config,
        Storage::empty(),
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
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use clap::builder;
    use commit::Committable;
    use core::num;
    use futures::{Stream, StreamExt};
    use hotshot::traits::implementations::{MasterMap, MemoryNetwork};
    use hotshot::types::EventType::Decide;
    use hotshot_types::traits::block_contents::{BlockHeader, BlockPayload};
    use hotshot_types::{
        light_client::StateKeyPair, traits::metrics::NoMetrics, ExecutionType, ValidatorConfig,
    };

    use sequencer::{transaction::Transaction, Event};
    use std::time::Duration;

    pub async fn init_hotshot_handles(
        num_nodes: usize,
        num_builders: usize,
    ) -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>> {
        init_hotshot_handles_with_metrics(num_nodes, num_builders, &NoMetrics).await
    }

    pub async fn init_hotshot_handles_with_metrics(
        num_nodes: usize,
        num_builders: usize,
        metrics: &dyn Metrics,
    ) -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>> {
        setup_logging();
        setup_backtrace();
        tracing::info!("Coming into init hotshot handles");
        let total_nodes = num_nodes + num_builders;

        // Generate the pub and the private keys for everyone
        let priv_keys = (0..total_nodes)
            .map(|_| PrivKey::generate(&mut rand::thread_rng()))
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(PubKey::from_private)
            .collect::<Vec<_>>();

        // known nodes with stake only for the hotshot nodes
        // TODO: Currently hotshot doesn't support passive nodes to receiver its events
        // TODO: therefore, we should make builder as staked node to receive events
        let known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry> = (0
            ..total_nodes)
            .map(|id| pub_keys[id].get_stake_table_entry(1u64))
            .collect();

        // TODO check total_nodes and da_committee_size

        let mut handles = vec![];
        let master_map = MasterMap::new();

        let config: HotShotConfig<_, _> = HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: total_nodes.try_into().unwrap(),
            min_transactions: 1,
            max_transactions: 10000.try_into().unwrap(),
            known_nodes_with_stake: known_nodes_with_stake.clone(),
            next_view_timeout: Duration::from_secs(5).as_millis() as u64,
            timeout_ratio: (10, 11),
            round_start_delay: Duration::from_millis(1).as_millis() as u64,
            start_delay: Duration::from_millis(1).as_millis() as u64,
            num_bootstrap: 1usize,
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(1),
            election_config: None,
            // TODO keep da commitee size as total_nodes for now, later would be num_nodes
            da_committee_size: total_nodes,
            my_own_validator_config: Default::default(),
        };

        // Create HotShot instances.
        for node_id in 0..total_nodes {
            let metrics = if node_id == 0 { metrics } else { &NoMetrics };

            let mut config = config.clone();
            config.my_own_validator_config = ValidatorConfig {
                public_key: pub_keys[node_id],
                private_key: priv_keys[node_id].clone(),
                stake_value: known_nodes_with_stake[node_id].stake_amount.as_u64(),
                state_key_pair: StateKeyPair::generate(),
            };

            let network = Arc::new(MemoryNetwork::new(
                pub_keys[node_id],
                NetworkingMetricsValue::new(metrics),
                master_map.clone(),
                None,
            ));
            let networks: Networks<SeqTypes, Node<network::Memory>> = Networks {
                da_network: network.clone(),
                quorum_network: network,
                _pd: Default::default(),
            };
            let instance_state = &NodeState::default();
            let handle = init_hotshot(
                pub_keys.clone(),
                known_nodes_with_stake.clone(),
                node_id,
                priv_keys[node_id].clone(),
                networks,
                config,
                metrics,
                instance_state,
            )
            .await;

            handles.push(handle);
        }

        handles
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
                if let Some(height) = leaf_chain.iter().find_map(|(leaf, _)| {
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
    use hotshot_example_types::block_types::genesis_vid_commitment;
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
                genesis_vid_commitment(),
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
