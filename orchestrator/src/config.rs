use hotshot_types::{ExecutionType, HotShotConfig};
use std::marker::PhantomData;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
    time::Duration,
};
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Libp2pConfig {
    pub bootstrap_nodes: Vec<(SocketAddr, Vec<u8>)>,
    pub num_bootstrap_nodes: usize,
    pub public_ip: IpAddr,
    pub base_port: u16,
    pub node_index: u64,
    pub index_ports: bool,
    pub bootstrap_mesh_n_high: usize,
    pub bootstrap_mesh_n_low: usize,
    pub bootstrap_mesh_outbound_min: usize,
    pub bootstrap_mesh_n: usize,
    pub mesh_n_high: usize,
    pub mesh_n_low: usize,
    pub mesh_outbound_min: usize,
    pub mesh_n: usize,
    pub next_view_timeout: u64,
    pub propose_min_round_time: Duration,
    pub propose_max_round_time: Duration,
    pub online_time: u64,
    pub num_txn_per_round: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Libp2pConfigFile {
    pub index_ports: bool,
    pub bootstrap_mesh_n_high: usize,
    pub bootstrap_mesh_n_low: usize,
    pub bootstrap_mesh_outbound_min: usize,
    pub bootstrap_mesh_n: usize,
    pub mesh_n_high: usize,
    pub mesh_n_low: usize,
    pub mesh_outbound_min: usize,
    pub mesh_n: usize,
    pub online_time: u64,
    pub base_port: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct WebServerConfig {
    pub host: IpAddr,
    pub port: u16,
    pub wait_between_polls: Duration,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NetworkConfig<KEY, ENTRY, ELECTIONCONFIG> {
    pub rounds: usize,
    pub transactions_per_round: usize,
    pub num_bootrap: usize,
    pub next_view_timeout: u64,
    pub propose_min_round_time: Duration,
    pub propose_max_round_time: Duration,
    pub node_index: u64,
    pub seed: [u8; 32],
    pub padding: usize,
    pub start_delay_seconds: u64,
    pub key_type_name: String,
    pub election_config_type_name: String,
    pub libp2p_config: Option<Libp2pConfig>,
    pub config: HotShotConfig<ENTRY, ELECTIONCONFIG>,
    pub web_server_config: Option<WebServerConfig>,
    pub da_web_server_config: Option<WebServerConfig>,
    _key_type_phantom: PhantomData<KEY>,
}

impl<K, ENTRY, E> Default for NetworkConfig<K, ENTRY, E> {
    fn default() -> Self {
        Self {
            rounds: default_rounds(),
            transactions_per_round: default_transactions_per_round(),
            node_index: 0,
            seed: [0u8; 32],
            padding: default_padding(),
            libp2p_config: None,
            config: default_config().into(),
            start_delay_seconds: 60,
            key_type_name: std::any::type_name::<K>().to_string(),
            election_config_type_name: std::any::type_name::<E>().to_string(),
            web_server_config: None,
            da_web_server_config: None,
            _key_type_phantom: PhantomData,
            next_view_timeout: 10,
            num_bootrap: 5,
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(10),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NetworkConfigFile {
    #[serde(default = "default_rounds")]
    pub rounds: usize,
    #[serde(default = "default_transactions_per_round")]
    pub transactions_per_round: usize,
    #[serde(default)]
    pub node_index: u64,
    #[serde(default)]
    pub seed: [u8; 32],
    #[serde(default = "default_padding")]
    pub padding: usize,
    #[serde(default = "default_start_delay_seconds")]
    pub start_delay_seconds: u64,
    #[serde(default)]
    pub libp2p_config: Option<Libp2pConfigFile>,
    #[serde(default = "default_config")]
    pub config: HotShotConfigFile,
    #[serde(default = "default_web_server_config")]
    pub web_server_config: Option<WebServerConfig>,
    #[serde(default = "default_web_server_config")]
    pub da_web_server_config: Option<WebServerConfig>,
}

fn default_web_server_config() -> Option<WebServerConfig> {
    None
}

impl<K, ENTRY, E> From<NetworkConfigFile> for NetworkConfig<K, ENTRY, E> {
    fn from(val: NetworkConfigFile) -> Self {
        NetworkConfig {
            rounds: val.rounds,
            transactions_per_round: val.transactions_per_round,
            node_index: 0,
            num_bootrap: val.config.num_bootstrap,
            next_view_timeout: val.config.next_view_timeout,
            propose_max_round_time: val.config.propose_max_round_time,
            propose_min_round_time: val.config.propose_min_round_time,
            seed: val.seed,
            padding: val.padding,
            libp2p_config: val.libp2p_config.map(|libp2p_config| Libp2pConfig {
                num_bootstrap_nodes: val.config.num_bootstrap,
                index_ports: libp2p_config.index_ports,
                bootstrap_nodes: Vec::new(),
                public_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                base_port: libp2p_config.base_port,
                node_index: 0,
                bootstrap_mesh_n_high: libp2p_config.bootstrap_mesh_n_high,
                bootstrap_mesh_n_low: libp2p_config.bootstrap_mesh_n_low,
                bootstrap_mesh_outbound_min: libp2p_config.bootstrap_mesh_outbound_min,
                bootstrap_mesh_n: libp2p_config.bootstrap_mesh_n,
                mesh_n_high: libp2p_config.mesh_n_high,
                mesh_n_low: libp2p_config.mesh_n_low,
                mesh_outbound_min: libp2p_config.mesh_outbound_min,
                mesh_n: libp2p_config.mesh_n,
                next_view_timeout: val.config.next_view_timeout,
                propose_min_round_time: val.config.propose_min_round_time,
                propose_max_round_time: val.config.propose_max_round_time,
                online_time: libp2p_config.online_time,
                num_txn_per_round: val.transactions_per_round,
            }),
            config: val.config.into(),
            key_type_name: std::any::type_name::<K>().to_string(),
            election_config_type_name: std::any::type_name::<E>().to_string(),
            start_delay_seconds: val.start_delay_seconds,
            web_server_config: val.web_server_config,
            da_web_server_config: val.da_web_server_config,
            _key_type_phantom: PhantomData,
        }
    }
}

/// Holds configuration for a `HotShot`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HotShotConfigFile {
    /// Total number of nodes in the network
    pub total_nodes: NonZeroUsize,
    /// Number of committee nodes
    pub committee_nodes: usize,
    /// Maximum transactions per block
    pub max_transactions: NonZeroUsize,
    /// Minimum transactions per block
    pub min_transactions: usize,
    /// Base duration for next-view timeout, in milliseconds
    pub next_view_timeout: u64,
    /// The exponential backoff ration for the next-view timeout
    pub timeout_ratio: (u64, u64),
    /// The delay a leader inserts before starting pre-commit, in milliseconds
    pub round_start_delay: u64,
    /// Delay after init before starting consensus, in milliseconds
    pub start_delay: u64,
    /// Number of network bootstrap nodes
    pub num_bootstrap: usize,
    /// The minimum amount of time a leader has to wait to start a round
    pub propose_min_round_time: Duration,
    /// The maximum amount of time a leader can wait to start a round
    pub propose_max_round_time: Duration,
}

impl<ENTRY, E> From<HotShotConfigFile> for HotShotConfig<ENTRY, E> {
    fn from(val: HotShotConfigFile) -> Self {
        HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: val.total_nodes,
            max_transactions: val.max_transactions,
            min_transactions: val.min_transactions,
            known_nodes_with_stake: Vec::new(),
            da_committee_size: val.committee_nodes,
            next_view_timeout: val.next_view_timeout,
            timeout_ratio: val.timeout_ratio,
            round_start_delay: val.round_start_delay,
            start_delay: val.start_delay,
            num_bootstrap: val.num_bootstrap,
            propose_min_round_time: val.propose_min_round_time,
            propose_max_round_time: val.propose_max_round_time,
            election_config: None,
        }
    }
}

// This is hacky, blame serde for not having something like `default_value = "10"`
fn default_rounds() -> usize {
    10
}
fn default_transactions_per_round() -> usize {
    10
}
fn default_padding() -> usize {
    100
}
fn default_config() -> HotShotConfigFile {
    HotShotConfigFile {
        total_nodes: NonZeroUsize::new(10).unwrap(),
        committee_nodes: 5,
        max_transactions: NonZeroUsize::new(100).unwrap(),
        min_transactions: 1,
        next_view_timeout: 10000,
        timeout_ratio: (11, 10),
        round_start_delay: 1,
        start_delay: 1,
        propose_min_round_time: Duration::from_secs(0),
        propose_max_round_time: Duration::from_secs(10),
        num_bootstrap: 5,
    }
}

fn default_start_delay_seconds() -> u64 {
    60
}
