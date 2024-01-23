#![allow(clippy::panic)]
use async_compatibility_layer::art::async_sleep;
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_lock::RwLock;
use async_trait::async_trait;
use clap::Parser;
use futures::StreamExt;
use hotshot::traits::implementations::{CombinedCommChannel, CombinedNetworks};
use hotshot::{
    traits::{
        implementations::{
            Libp2pCommChannel, Libp2pNetwork, MemoryStorage, NetworkingMetricsValue,
            WebCommChannel, WebServerNetwork,
        },
        NodeImplementation,
    },
    types::{SignatureKey, SystemContextHandle},
    Memberships, Networks, SystemContext,
};
use hotshot_orchestrator::config::NetworkConfigSource;
use hotshot_orchestrator::{
    self,
    client::{OrchestratorClient, ValidatorArgs},
    config::{NetworkConfig, NetworkConfigFile, WebServerConfig},
};
use hotshot_task::task::FilterEvent;
use hotshot_testing::block_types::{TestBlockHeader, TestBlockPayload, TestTransaction};
use hotshot_types::message::Message;
use hotshot_types::traits::network::ConnectedNetwork;
use hotshot_types::ValidatorConfig;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{Leaf, TestableLeaf},
    event::{Event, EventType},
    traits::{
        election::Membership,
        network::CommunicationChannel,
        node_implementation::NodeType,
        state::{ConsensusTime, TestableBlock, TestableState},
    },
    HotShotConfig,
};
use libp2p_identity::{
    ed25519::{self, SecretKey},
    Keypair,
};
use libp2p_networking::{
    network::{MeshParams, NetworkNodeConfigBuilder, NetworkNodeType},
    reexport::Multiaddr,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::marker::PhantomData;
use std::time::Duration;
use std::{collections::BTreeSet, sync::Arc};
use std::{num::NonZeroUsize, str::FromStr};
use surf_disco::Url;

use libp2p_identity::PeerId;
use std::fmt::Debug;
use std::{fs, time::Instant};
use tracing::{error, info, warn};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Multi-machine consensus",
    about = "Simulates consensus among multiple machines"
)]
/// Arguments passed to the orchestrator
pub struct OrchestratorArgs {
    /// The url the orchestrator runs on; this should be in the form of `http://localhost:5555` or `http://0.0.0.0:5555`
    pub url: Url,
    /// The configuration file to be used for this run
    pub config_file: String,
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Multi-machine consensus",
    about = "Simulates consensus among multiple machines"
)]
/// The configuration file to be used for this run
pub struct ConfigArgs {
    /// The configuration file to be used for this run
    pub config_file: String,
}

/// Reads a network configuration from a given filepath
/// # Panics
/// if unable to convert the config file into toml
#[must_use]
pub fn load_config_from_file<TYPES: NodeType>(
    config_file: &str,
) -> NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType> {
    let config_file_as_string: String = fs::read_to_string(config_file)
        .unwrap_or_else(|_| panic!("Could not read config file located at {config_file}"));
    let config_toml: NetworkConfigFile<TYPES::SignatureKey> =
        toml::from_str::<NetworkConfigFile<TYPES::SignatureKey>>(&config_file_as_string)
            .expect("Unable to convert config file to TOML");

    let mut config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType> =
        config_toml.into();

    // Generate network's public keys
    let known_nodes: Vec<_> = (0..config.config.total_nodes.get())
        .map(|node_id| {
            TYPES::SignatureKey::generated_from_seed_indexed(
                config.seed,
                node_id.try_into().unwrap(),
            )
            .0
        })
        .collect();

    config.config.known_nodes_with_stake = (0..config.config.total_nodes.get())
        .map(|node_id| known_nodes[node_id].get_stake_table_entry(1u64))
        .collect();

    config
}

/// Runs the orchestrator
pub async fn run_orchestrator<
    TYPES: NodeType,
    DACHANNEL: CommunicationChannel<TYPES> + Debug,
    QUORUMCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIEWSYNCCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIDCHANNEL: CommunicationChannel<TYPES> + Debug,
    NODE: NodeImplementation<TYPES, Storage = MemoryStorage<TYPES>>,
>(
    OrchestratorArgs { url, config_file }: OrchestratorArgs,
) {
    error!("Starting orchestrator",);
    let run_config = load_config_from_file::<TYPES>(&config_file);
    let _result = hotshot_orchestrator::run_orchestrator::<
        TYPES::SignatureKey,
        TYPES::ElectionConfigType,
    >(run_config, url)
    .await;
}

/// Helper function to calculate the nuymber of transactions to send per node per round
#[allow(clippy::cast_possible_truncation)]
fn calculate_num_tx_per_round(
    node_index: u64,
    total_num_nodes: usize,
    transactions_per_round: usize,
) -> usize {
    transactions_per_round / total_num_nodes
        + usize::from(
            (total_num_nodes - 1 - node_index as usize)
                < (transactions_per_round % total_num_nodes),
        )
}

/// create a web server network from a config file + public key
/// # Panics
/// Panics if the web server config doesn't exist in `config`
fn webserver_network_from_config<TYPES: NodeType>(
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    pub_key: TYPES::SignatureKey,
) -> WebServerNetwork<TYPES> {
    // Get the configuration for the web server
    let WebServerConfig {
        url,
        wait_between_polls,
    }: WebServerConfig = config.web_server_config.unwrap();

    WebServerNetwork::create(url, wait_between_polls, pub_key, false)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_lossless)]
#[allow(clippy::too_many_lines)]
/// Create a libp2p network from a config file and public key
/// # Panics
/// If unable to create bootstrap nodes multiaddres or the libp2p config is invalid
async fn libp2p_network_from_config<TYPES: NodeType>(
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    pub_key: TYPES::SignatureKey,
) -> Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> {
    let mut config = config;
    let libp2p_config = config
        .libp2p_config
        .take()
        .expect("Configuration is not for a Libp2p network");
    let bs_len = libp2p_config.bootstrap_nodes.len();
    let bootstrap_nodes: Vec<(PeerId, Multiaddr)> = libp2p_config
        .bootstrap_nodes
        .iter()
        .map(|(addr, pair)| {
            let kp = Keypair::from_protobuf_encoding(pair).unwrap();
            let peer_id = PeerId::from_public_key(&kp.public());
            let multiaddr =
                Multiaddr::from_str(&format!("/ip4/{}/udp/{}/quic-v1", addr.ip(), addr.port()))
                    .unwrap();
            (peer_id, multiaddr)
        })
        .collect();
    let identity = libp2p_generate_indexed_identity(config.seed, config.node_index);
    let node_type = if (config.node_index as usize) < bs_len {
        NetworkNodeType::Bootstrap
    } else {
        NetworkNodeType::Regular
    };
    let node_index = config.node_index;
    let port_index = if libp2p_config.index_ports {
        node_index
    } else {
        0
    };
    let bound_addr: Multiaddr = format!(
        "/{}/{}/udp/{}/quic-v1",
        if libp2p_config.public_ip.is_ipv4() {
            "ip4"
        } else {
            "ip6"
        },
        libp2p_config.public_ip,
        libp2p_config.base_port as u64 + port_index
    )
    .parse()
    .unwrap();

    // generate network
    let mut config_builder = NetworkNodeConfigBuilder::default();
    assert!(config.config.total_nodes.get() > 2);
    let replicated_nodes = NonZeroUsize::new(config.config.total_nodes.get() - 2).unwrap();
    config_builder.replication_factor(replicated_nodes);
    config_builder.identity(identity.clone());

    config_builder.bound_addr(Some(bound_addr.clone()));

    let to_connect_addrs = bootstrap_nodes
        .iter()
        .map(|(peer_id, multiaddr)| (Some(*peer_id), multiaddr.clone()))
        .collect();

    config_builder.to_connect_addrs(to_connect_addrs);

    let mesh_params =
        // NOTE I'm arbitrarily choosing these.
        match node_type {
            NetworkNodeType::Bootstrap => MeshParams {
                mesh_n_high: libp2p_config.bootstrap_mesh_n_high,
                mesh_n_low: libp2p_config.bootstrap_mesh_n_low,
                mesh_outbound_min: libp2p_config.bootstrap_mesh_outbound_min,
                mesh_n: libp2p_config.bootstrap_mesh_n,
            },
            NetworkNodeType::Regular => MeshParams {
                mesh_n_high: libp2p_config.mesh_n_high,
                mesh_n_low: libp2p_config.mesh_n_low,
                mesh_outbound_min: libp2p_config.mesh_outbound_min,
                mesh_n: libp2p_config.mesh_n,
            },
            NetworkNodeType::Conductor => unreachable!(),
        };
    config_builder.mesh_params(Some(mesh_params));

    let mut all_keys = BTreeSet::new();
    let mut da_keys = BTreeSet::new();
    for i in 0..config.config.total_nodes.get() as u64 {
        let privkey = TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], i).1;
        let pub_key = TYPES::SignatureKey::from_private(&privkey);
        if i < config.config.da_committee_size as u64 {
            da_keys.insert(pub_key.clone());
        }
        all_keys.insert(pub_key);
    }
    let node_config = config_builder.build().unwrap();

    #[allow(clippy::cast_possible_truncation)]
    Libp2pNetwork::new(
        NetworkingMetricsValue::default(),
        node_config,
        pub_key.clone(),
        Arc::new(RwLock::new(
            bootstrap_nodes
                .iter()
                .map(|(peer_id, addr)| (Some(*peer_id), addr.clone()))
                .collect(),
        )),
        bs_len,
        config.node_index as usize,
        // NOTE: this introduces an invariant that the keys are assigned using this indexed
        // function
        all_keys,
        None,
        da_keys.clone(),
        da_keys.contains(&pub_key),
    )
    .await
    .unwrap()
}

/// Defines the behavior of a "run" of the network with a given configuration
#[async_trait]
pub trait RunDA<
    TYPES: NodeType,
    DACHANNEL: CommunicationChannel<TYPES> + Debug,
    QUORUMCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIEWSYNCCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIDCHANNEL: CommunicationChannel<TYPES> + Debug,
    NODE: NodeImplementation<
        TYPES,
        QuorumNetwork = QUORUMCHANNEL,
        CommitteeNetwork = DACHANNEL,
        Storage = MemoryStorage<TYPES>,
    >,
> where
    <TYPES as NodeType>::StateType: TestableState,
    <TYPES as NodeType>::BlockPayload: TestableBlock,
    TYPES: NodeType<Transaction = TestTransaction>,
    Leaf<TYPES>: TestableLeaf,
    Self: Sync,
{
    /// Initializes networking, returns self
    async fn initialize_networking(
        config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    ) -> Self;

    /// Initializes the genesis state and HotShot instance; does not start HotShot consensus
    /// # Panics if it cannot generate a genesis block, fails to initialize HotShot, or cannot
    /// get the anchored view
    /// Note: sequencing leaf does not have state, so does not return state
    async fn initialize_state_and_hotshot(&self) -> SystemContextHandle<TYPES, NODE> {
        let initializer = hotshot::HotShotInitializer::<TYPES>::from_genesis()
            .expect("Couldn't generate genesis block");

        let config = self.get_config();

        // Get KeyPair for certificate Aggregation
        let pk = config.config.my_own_validator_config.public_key.clone();
        let sk = config.config.my_own_validator_config.private_key.clone();
        let known_nodes_with_stake = config.config.known_nodes_with_stake.clone();

        let da_network = self.get_da_channel();
        let quorum_network = self.get_quorum_channel();

        // Since we do not currently pass the election config type in the NetworkConfig, this will always be the default election config
        let quorum_election_config = config.config.election_config.clone().unwrap_or_else(|| {
            TYPES::Membership::default_election_config(config.config.total_nodes.get() as u64)
        });

        let committee_election_config = TYPES::Membership::default_election_config(
            config.config.da_committee_size.try_into().unwrap(),
        );
        let networks_bundle = Networks {
            quorum_network: quorum_network.clone(),
            da_network: da_network.clone(),
            _pd: PhantomData,
        };

        let memberships = Memberships {
            quorum_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config.clone(),
            ),
            da_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                committee_election_config,
            ),
            vid_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config.clone(),
            ),
            view_sync_membership: <TYPES as NodeType>::Membership::create_election(
                known_nodes_with_stake.clone(),
                quorum_election_config,
            ),
        };

        SystemContext::init(
            pk,
            sk,
            config.node_index,
            config.config,
            MemoryStorage::empty(),
            memberships,
            networks_bundle,
            initializer,
            ConsensusMetricsValue::default(),
        )
        .await
        .expect("Could not init hotshot")
        .0
    }

    /// Starts HotShot consensus, returns when consensus has finished
    async fn run_hotshot(
        &self,
        mut context: SystemContextHandle<TYPES, NODE>,
        transactions: &mut Vec<TestTransaction>,
        transactions_to_send_per_round: u64,
    ) {
        let NetworkConfig {
            rounds,
            node_index,
            start_delay_seconds,
            ..
        } = self.get_config();

        let mut total_transactions_committed = 0;
        let mut total_transactions_sent = 0;

        error!("Sleeping for {start_delay_seconds} seconds before starting hotshot!");
        async_sleep(Duration::from_secs(start_delay_seconds)).await;

        error!("Starting HotShot example!");
        let start = Instant::now();

        let (mut event_stream, _streamid) = context.get_event_stream(FilterEvent::default()).await;
        let mut anchor_view: TYPES::Time = <TYPES::Time as ConsensusTime>::genesis();
        let mut num_successful_commits = 0;

        context.hotshot.start_consensus().await;

        loop {
            match event_stream.next().await {
                None => {
                    panic!("Error! Event stream completed before consensus ended.");
                }
                Some(Event { event, .. }) => {
                    match event {
                        EventType::Error { error } => {
                            error!("Error in consensus: {:?}", error);
                            // TODO what to do here
                        }
                        EventType::Decide {
                            leaf_chain,
                            qc: _,
                            block_size,
                        } => {
                            // this might be a obob
                            if let Some(leaf) = leaf_chain.first() {
                                info!("Decide event for leaf: {}", *leaf.view_number);

                                let new_anchor = leaf.view_number;
                                if new_anchor >= anchor_view {
                                    anchor_view = leaf.view_number;
                                }

                                // send transactions
                                for _ in 0..transactions_to_send_per_round {
                                    let tx = transactions.remove(0);

                                    () = context.submit_transaction(tx).await.unwrap();
                                    total_transactions_sent += 1;
                                }
                            }

                            if let Some(size) = block_size {
                                total_transactions_committed += size;
                            }

                            num_successful_commits += leaf_chain.len();
                            if num_successful_commits >= rounds {
                                break;
                            }

                            if leaf_chain.len() > 1 {
                                warn!("Leaf chain is greater than 1 with len {}", leaf_chain.len());
                            }
                            // when we make progress, submit new events
                        }
                        EventType::ReplicaViewTimeout { view_number } => {
                            warn!("Timed out as a replicas in view {:?}", view_number);
                        }
                        EventType::NextLeaderViewTimeout { view_number } => {
                            warn!("Timed out as the next leader in view {:?}", view_number);
                        }
                        EventType::ViewFinished { view_number: _ } => {}
                        _ => unimplemented!(),
                    }
                }
            }
        }

        // Output run results
        let total_time_elapsed = start.elapsed();
        error!("[{node_index}]: {rounds} rounds completed in {total_time_elapsed:?} - Total transactions sent: {total_transactions_sent} - Total transactions committed: {total_transactions_committed} - Total commitments: {num_successful_commits}");
    }

    /// Returns the da network for this run
    fn get_da_channel(&self) -> DACHANNEL;

    /// Returns the quorum network for this run
    fn get_quorum_channel(&self) -> QUORUMCHANNEL;

    ///Returns view sync network for this run
    fn get_view_sync_channel(&self) -> VIEWSYNCCHANNEL;

    ///Returns VID network for this run
    fn get_vid_channel(&self) -> VIDCHANNEL;

    /// Returns the config for this run
    fn get_config(&self) -> NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>;
}

// WEB SERVER

/// Represents a web server-based run
pub struct WebServerDARun<TYPES: NodeType> {
    /// the network configuration
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    /// quorum channel
    quorum_channel: WebCommChannel<TYPES>,
    /// data availability channel
    da_channel: WebCommChannel<TYPES>,
    /// view sync channel
    view_sync_channel: WebCommChannel<TYPES>,
    /// vid channel
    vid_channel: WebCommChannel<TYPES>,
}

#[async_trait]
impl<
        TYPES: NodeType<
            Transaction = TestTransaction,
            BlockPayload = TestBlockPayload,
            BlockHeader = TestBlockHeader,
        >,
        NODE: NodeImplementation<
            TYPES,
            QuorumNetwork = WebCommChannel<TYPES>,
            CommitteeNetwork = WebCommChannel<TYPES>,
            Storage = MemoryStorage<TYPES>,
        >,
    >
    RunDA<
        TYPES,
        WebCommChannel<TYPES>,
        WebCommChannel<TYPES>,
        WebCommChannel<TYPES>,
        WebCommChannel<TYPES>,
        NODE,
    > for WebServerDARun<TYPES>
where
    <TYPES as NodeType>::StateType: TestableState,
    <TYPES as NodeType>::BlockPayload: TestableBlock,
    Leaf<TYPES>: TestableLeaf,
    Self: Sync,
{
    async fn initialize_networking(
        config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    ) -> WebServerDARun<TYPES> {
        // Get our own key
        let pub_key = config.config.my_own_validator_config.public_key.clone();

        // extract values from config (for DA network)
        let WebServerConfig {
            url,
            wait_between_polls,
        }: WebServerConfig = config.clone().da_web_server_config.unwrap();

        // create and wait for underlying network
        let underlying_quorum_network =
            webserver_network_from_config::<TYPES>(config.clone(), pub_key.clone());

        underlying_quorum_network.wait_for_ready().await;

        // create communication channels
        let quorum_channel: WebCommChannel<TYPES> =
            WebCommChannel::new(underlying_quorum_network.clone().into());

        let view_sync_channel: WebCommChannel<TYPES> =
            WebCommChannel::new(underlying_quorum_network.into());

        let da_channel: WebCommChannel<TYPES> = WebCommChannel::new(
            WebServerNetwork::create(url.clone(), wait_between_polls, pub_key.clone(), true).into(),
        );

        let vid_channel: WebCommChannel<TYPES> = WebCommChannel::new(
            WebServerNetwork::create(url, wait_between_polls, pub_key, true).into(),
        );

        WebServerDARun {
            config,
            quorum_channel,
            da_channel,
            view_sync_channel,
            vid_channel,
        }
    }

    fn get_da_channel(&self) -> WebCommChannel<TYPES> {
        self.da_channel.clone()
    }

    fn get_quorum_channel(&self) -> WebCommChannel<TYPES> {
        self.quorum_channel.clone()
    }

    fn get_view_sync_channel(&self) -> WebCommChannel<TYPES> {
        self.view_sync_channel.clone()
    }

    fn get_vid_channel(&self) -> WebCommChannel<TYPES> {
        self.vid_channel.clone()
    }

    fn get_config(&self) -> NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType> {
        self.config.clone()
    }
}

// Libp2p

/// Represents a libp2p-based run
pub struct Libp2pDARun<TYPES: NodeType> {
    /// the network configuration
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    /// quorum channel
    quorum_channel: Libp2pCommChannel<TYPES>,
    /// data availability channel
    da_channel: Libp2pCommChannel<TYPES>,
    /// view sync channel
    view_sync_channel: Libp2pCommChannel<TYPES>,
    /// vid channel
    vid_channel: Libp2pCommChannel<TYPES>,
}

#[async_trait]
impl<
        TYPES: NodeType<
            Transaction = TestTransaction,
            BlockPayload = TestBlockPayload,
            BlockHeader = TestBlockHeader,
        >,
        NODE: NodeImplementation<
            TYPES,
            QuorumNetwork = Libp2pCommChannel<TYPES>,
            CommitteeNetwork = Libp2pCommChannel<TYPES>,
            Storage = MemoryStorage<TYPES>,
        >,
    >
    RunDA<
        TYPES,
        Libp2pCommChannel<TYPES>,
        Libp2pCommChannel<TYPES>,
        Libp2pCommChannel<TYPES>,
        Libp2pCommChannel<TYPES>,
        NODE,
    > for Libp2pDARun<TYPES>
where
    <TYPES as NodeType>::StateType: TestableState,
    <TYPES as NodeType>::BlockPayload: TestableBlock,
    Leaf<TYPES>: TestableLeaf,
    Self: Sync,
{
    async fn initialize_networking(
        config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    ) -> Libp2pDARun<TYPES> {
        let pub_key = config.config.my_own_validator_config.public_key.clone();

        // create and wait for underlying network
        let underlying_quorum_network =
            libp2p_network_from_config::<TYPES>(config.clone(), pub_key).await;

        underlying_quorum_network.wait_for_ready().await;

        // create communication channels
        let quorum_channel: Libp2pCommChannel<TYPES> =
            Libp2pCommChannel::new(underlying_quorum_network.clone().into());

        let view_sync_channel: Libp2pCommChannel<TYPES> =
            Libp2pCommChannel::new(underlying_quorum_network.clone().into());

        let da_channel: Libp2pCommChannel<TYPES> =
            Libp2pCommChannel::new(underlying_quorum_network.clone().into());

        let vid_channel: Libp2pCommChannel<TYPES> =
            Libp2pCommChannel::new(underlying_quorum_network.clone().into());

        Libp2pDARun {
            config,
            quorum_channel,
            da_channel,
            view_sync_channel,
            vid_channel,
        }
    }

    fn get_da_channel(&self) -> Libp2pCommChannel<TYPES> {
        self.da_channel.clone()
    }

    fn get_quorum_channel(&self) -> Libp2pCommChannel<TYPES> {
        self.quorum_channel.clone()
    }

    fn get_view_sync_channel(&self) -> Libp2pCommChannel<TYPES> {
        self.view_sync_channel.clone()
    }

    fn get_vid_channel(&self) -> Libp2pCommChannel<TYPES> {
        self.vid_channel.clone()
    }

    fn get_config(&self) -> NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType> {
        self.config.clone()
    }
}

// Combined network

/// Represents a combined-network-based run
pub struct CombinedDARun<TYPES: NodeType> {
    /// the network configuration
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    /// quorum channel
    quorum_channel: CombinedCommChannel<TYPES>,
    /// data availability channel
    da_channel: CombinedCommChannel<TYPES>,
    /// view sync channel
    view_sync_channel: CombinedCommChannel<TYPES>,
    /// vid channel
    vid_channel: CombinedCommChannel<TYPES>,
}

#[async_trait]
impl<
        TYPES: NodeType<
            Transaction = TestTransaction,
            BlockPayload = TestBlockPayload,
            BlockHeader = TestBlockHeader,
        >,
        NODE: NodeImplementation<
            TYPES,
            Storage = MemoryStorage<TYPES>,
            QuorumNetwork = CombinedCommChannel<TYPES>,
            CommitteeNetwork = CombinedCommChannel<TYPES>,
        >,
    >
    RunDA<
        TYPES,
        CombinedCommChannel<TYPES>,
        CombinedCommChannel<TYPES>,
        CombinedCommChannel<TYPES>,
        CombinedCommChannel<TYPES>,
        NODE,
    > for CombinedDARun<TYPES>
where
    <TYPES as NodeType>::StateType: TestableState,
    <TYPES as NodeType>::BlockPayload: TestableBlock,
    Leaf<TYPES>: TestableLeaf,
    Self: Sync,
{
    async fn initialize_networking(
        config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    ) -> CombinedDARun<TYPES> {
        // generate our own key
        let (pub_key, _privkey) =
            <<TYPES as NodeType>::SignatureKey as SignatureKey>::generated_from_seed_indexed(
                config.seed,
                config.node_index,
            );

        // create and wait for libp2p network
        let libp2p_underlying_quorum_network =
            libp2p_network_from_config::<TYPES>(config.clone(), pub_key.clone()).await;

        libp2p_underlying_quorum_network.wait_for_ready().await;

        // extract values from config (for webserver DA network)
        let WebServerConfig {
            url,
            wait_between_polls,
        }: WebServerConfig = config.clone().da_web_server_config.unwrap();

        // create and wait for underlying webserver network
        let webserver_underlying_quorum_network =
            webserver_network_from_config::<TYPES>(config.clone(), pub_key.clone());

        let webserver_underlying_da_network =
            WebServerNetwork::create(url, wait_between_polls, pub_key, true);

        webserver_underlying_quorum_network.wait_for_ready().await;

        // combine the two communication channels
        let quorum_channel = CombinedCommChannel::new(Arc::new(CombinedNetworks(
            webserver_underlying_quorum_network.clone(),
            libp2p_underlying_quorum_network.clone(),
        )));

        let view_sync_channel = CombinedCommChannel::new(Arc::new(CombinedNetworks(
            webserver_underlying_quorum_network.clone(),
            libp2p_underlying_quorum_network.clone(),
        )));

        let da_channel: CombinedCommChannel<TYPES> =
            CombinedCommChannel::new(Arc::new(CombinedNetworks(
                webserver_underlying_da_network,
                libp2p_underlying_quorum_network.clone(),
            )));

        let vid_channel = CombinedCommChannel::new(Arc::new(CombinedNetworks(
            webserver_underlying_quorum_network,
            libp2p_underlying_quorum_network,
        )));

        CombinedDARun {
            config,
            quorum_channel,
            da_channel,
            view_sync_channel,
            vid_channel,
        }
    }

    fn get_da_channel(&self) -> CombinedCommChannel<TYPES> {
        self.da_channel.clone()
    }

    fn get_quorum_channel(&self) -> CombinedCommChannel<TYPES> {
        self.quorum_channel.clone()
    }

    fn get_view_sync_channel(&self) -> CombinedCommChannel<TYPES> {
        self.view_sync_channel.clone()
    }

    fn get_vid_channel(&self) -> CombinedCommChannel<TYPES> {
        self.vid_channel.clone()
    }

    fn get_config(&self) -> NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType> {
        self.config.clone()
    }
}

/// Main entry point for validators
/// # Panics
/// if unable to get the local ip address
pub async fn main_entry_point<
    TYPES: NodeType<
        Transaction = TestTransaction,
        BlockPayload = TestBlockPayload,
        BlockHeader = TestBlockHeader,
    >,
    DACHANNEL: CommunicationChannel<TYPES> + Debug,
    QUORUMCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIEWSYNCCHANNEL: CommunicationChannel<TYPES> + Debug,
    VIDCHANNEL: CommunicationChannel<TYPES> + Debug,
    NODE: NodeImplementation<
        TYPES,
        QuorumNetwork = QUORUMCHANNEL,
        CommitteeNetwork = DACHANNEL,
        Storage = MemoryStorage<TYPES>,
    >,
    RUNDA: RunDA<TYPES, DACHANNEL, QUORUMCHANNEL, VIEWSYNCCHANNEL, VIDCHANNEL, NODE>,
>(
    args: ValidatorArgs,
) where
    <TYPES as NodeType>::StateType: TestableState,
    <TYPES as NodeType>::BlockPayload: TestableBlock,
    Leaf<TYPES>: TestableLeaf,
{
    setup_logging();
    setup_backtrace();

    error!("Starting validator");

    // see what our public identity will be
    let public_ip = match args.public_ip {
        Some(ip) => ip,
        None => local_ip_address::local_ip().unwrap(),
    };

    let orchestrator_client: OrchestratorClient =
        OrchestratorClient::new(args.clone(), public_ip.to_string());

    // conditionally save/load config from file or orchestrator
    let (mut run_config, source) =
        NetworkConfig::from_file_or_orchestrator(&orchestrator_client, args.network_config_file)
            .await;

    let node_index = run_config.node_index;
    error!("Retrieved config; our node index is {node_index}");

    run_config.config.my_own_validator_config =
        ValidatorConfig::<<TYPES as NodeType>::SignatureKey>::generated_from_seed_indexed(
            run_config.seed,
            node_index,
            1,
        );
    //run_config.libp2p_config.as_mut().unwrap().public_ip = args.public_ip.unwrap();

    error!("Initializing networking");
    let run = RUNDA::initialize_networking(run_config.clone()).await;
    let hotshot = run.initialize_state_and_hotshot().await;

    // pre-generate transactions
    let NetworkConfig {
        transaction_size,
        rounds,
        transactions_per_round,
        node_index,
        config: HotShotConfig { total_nodes, .. },
        ..
    } = run_config;

    let mut txn_rng = StdRng::seed_from_u64(node_index);
    let transactions_to_send_per_round =
        calculate_num_tx_per_round(node_index, total_nodes.get(), transactions_per_round);
    let mut transactions = Vec::new();

    for round in 0..rounds {
        for _ in 0..transactions_to_send_per_round {
            let mut txn = <TYPES::StateType>::create_random_transaction(
                None,
                &mut txn_rng,
                transaction_size as u64,
            );

            // prepend destined view number to transaction
            let view_execute_number: u64 = round as u64 + 4;
            txn.0[0..8].copy_from_slice(&view_execute_number.to_be_bytes());

            transactions.push(txn);
        }
    }

    if let NetworkConfigSource::Orchestrator = source {
        error!("Waiting for the start command from orchestrator");
        orchestrator_client
            .wait_for_all_nodes_ready(run_config.clone().node_index)
            .await;
    }

    error!("Starting HotShot");
    run.run_hotshot(
        hotshot,
        &mut transactions,
        transactions_to_send_per_round as u64,
    )
    .await;
}

/// generate a libp2p identity based on a seed and idx
/// # Panics
/// if unable to create a secret key out of bytes
#[must_use]
pub fn libp2p_generate_indexed_identity(seed: [u8; 32], index: u64) -> Keypair {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let new_seed = *hasher.finalize().as_bytes();
    let sk_bytes = SecretKey::try_from_bytes(new_seed).unwrap();
    <ed25519::Keypair as From<SecretKey>>::from(sk_bytes).into()
}
