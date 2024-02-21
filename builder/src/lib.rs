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
    NetworkParams, Node, NodeState, PrivKey, PubKey, SeqTypes as BuilderType, Storage,
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

    let instance_state = &NodeState {};
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
    networks: Networks<BuilderType, Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
    metrics: &dyn Metrics,
    instance_state: &NodeState,
) -> SystemContextHandle<BuilderType, Node<N>> {
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
