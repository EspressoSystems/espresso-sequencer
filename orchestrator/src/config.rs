use hotshot_types::{
    traits::{election::ElectionConfig, signature_key::SignatureKey},
    ExecutionType, HotShotConfig, ValidatorConfig,
};
use std::fs;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
    path::PathBuf,
    time::Duration,
};
use toml;
use tracing::error;
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
#[serde(bound(deserialize = ""))]
pub struct NetworkConfig<KEY: SignatureKey, ELECTIONCONFIG: ElectionConfig> {
    pub rounds: usize,
    pub transactions_per_round: usize,
    pub num_bootrap: usize,
    pub next_view_timeout: u64,
    pub propose_min_round_time: Duration,
    pub propose_max_round_time: Duration,
    pub node_index: u64,
    pub seed: [u8; 32],
    pub transaction_size: usize,
    pub start_delay_seconds: u64,
    pub key_type_name: String,
    pub election_config_type_name: String,
    pub libp2p_config: Option<Libp2pConfig>,
    pub config: HotShotConfig<KEY, ELECTIONCONFIG>,
    pub web_server_config: Option<WebServerConfig>,
    pub da_web_server_config: Option<WebServerConfig>,
}

impl<K: SignatureKey, E: ElectionConfig> Default for NetworkConfig<K, E> {
    fn default() -> Self {
        Self {
            rounds: default_rounds(),
            transactions_per_round: default_transactions_per_round(),
            node_index: 0,
            seed: [0u8; 32],
            transaction_size: default_transaction_size(),
            libp2p_config: None,
            config: HotShotConfigFile::default().into(),
            start_delay_seconds: 60,
            key_type_name: std::any::type_name::<K>().to_string(),
            election_config_type_name: std::any::type_name::<E>().to_string(),
            web_server_config: None,
            da_web_server_config: None,
            next_view_timeout: 10,
            num_bootrap: 5,
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(10),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(bound(deserialize = ""))]
pub struct NetworkConfigFile<KEY: SignatureKey> {
    #[serde(default = "default_rounds")]
    pub rounds: usize,
    #[serde(default = "default_transactions_per_round")]
    pub transactions_per_round: usize,
    #[serde(default)]
    pub node_index: u64,
    #[serde(default)]
    pub seed: [u8; 32],
    #[serde(default = "default_transaction_size")]
    pub transaction_size: usize,
    #[serde(default = "default_start_delay_seconds")]
    pub start_delay_seconds: u64,
    #[serde(default)]
    pub libp2p_config: Option<Libp2pConfigFile>,
    #[serde(default)]
    pub config: HotShotConfigFile<KEY>,
    #[serde(default = "default_web_server_config")]
    pub web_server_config: Option<WebServerConfig>,
    #[serde(default = "default_web_server_config")]
    pub da_web_server_config: Option<WebServerConfig>,
}

fn default_web_server_config() -> Option<WebServerConfig> {
    None
}

impl<K: SignatureKey, E: ElectionConfig> From<NetworkConfigFile<K>> for NetworkConfig<K, E> {
    fn from(val: NetworkConfigFile<K>) -> Self {
        NetworkConfig {
            rounds: val.rounds,
            transactions_per_round: val.transactions_per_round,
            node_index: 0,
            num_bootrap: val.config.num_bootstrap,
            next_view_timeout: val.config.next_view_timeout,
            propose_max_round_time: val.config.propose_max_round_time,
            propose_min_round_time: val.config.propose_min_round_time,
            seed: val.seed,
            transaction_size: val.transaction_size,
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
        }
    }
}

/// Holds configuration for a `HotShot`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct HotShotConfigFile<KEY: SignatureKey> {
    /// Total number of nodes in the network
    pub total_nodes: NonZeroUsize,
    #[serde(skip)]
    /// My own public key, secret key, stake value
    pub my_own_validator_config: ValidatorConfig<KEY>,
    #[serde(skip)]
    /// The known nodes' public key and stake value
    pub known_nodes_with_stake: Vec<KEY::StakeTableEntry>,
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

/// Holds configuration for a validator node
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(bound(deserialize = ""))]
pub struct ValidatorConfigFile {
    /// The validator's seed
    pub seed: [u8; 32],
    /// The validator's index, which can be treated as another input to the seed
    pub node_id: u64,
    // The validator's stake, commented for now
    // pub stake_value: u64,
}

impl ValidatorConfigFile {
    pub fn from_file(dir_str: &str) -> Self {
        let current_working_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                error!("get_current_working_dir error: {:?}", e);
                PathBuf::from("")
            }
        };
        let filename =
            current_working_dir.into_os_string().into_string().unwrap() + "/../../" + dir_str;
        match fs::read_to_string(filename.clone()) {
            // If successful return the files text as `contents`.
            Ok(contents) => {
                let data: ValidatorConfigFile = match toml::from_str(&contents) {
                    // If successful, return data as `Data` struct.
                    // `d` is a local variable.
                    Ok(d) => d,
                    // Handle the `error` case.
                    Err(e) => {
                        // Write `msg` to `stderr`.
                        error!("Unable to load data from `{}`: {}", filename, e);
                        ValidatorConfigFile::default()
                    }
                };
                data
            }
            // Handle the `error` case.
            Err(e) => {
                // Write `msg` to `stderr`.
                error!("Could not read file `{}`: {}", filename, e);
                ValidatorConfigFile::default()
            }
        }
    }
}

impl<KEY: SignatureKey, E: ElectionConfig> From<HotShotConfigFile<KEY>> for HotShotConfig<KEY, E> {
    fn from(val: HotShotConfigFile<KEY>) -> Self {
        HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: val.total_nodes,
            max_transactions: val.max_transactions,
            min_transactions: val.min_transactions,
            known_nodes_with_stake: val.known_nodes_with_stake,
            my_own_validator_config: val.my_own_validator_config,
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
fn default_transaction_size() -> usize {
    100
}

impl<K: SignatureKey> From<ValidatorConfigFile> for ValidatorConfig<K> {
    fn from(val: ValidatorConfigFile) -> Self {
        // here stake_value is set to 1, since we don't input stake_value from ValidatorConfigFile for now
        let validator_config =
            ValidatorConfig::generated_from_seed_indexed(val.seed, val.node_id, 1);
        ValidatorConfig {
            public_key: validator_config.public_key,
            private_key: validator_config.private_key,
            stake_value: validator_config.stake_value,
        }
    }
}
impl<KEY: SignatureKey, E: ElectionConfig> From<ValidatorConfigFile> for HotShotConfig<KEY, E> {
    fn from(value: ValidatorConfigFile) -> Self {
        let mut config: HotShotConfig<KEY, E> = HotShotConfigFile::default().into();
        config.my_own_validator_config = value.into();
        config
    }
}

impl<KEY: SignatureKey> Default for HotShotConfigFile<KEY> {
    fn default() -> Self {
        let gen_known_nodes_with_stake = (0..10)
            .map(|node_id| {
                let cur_validator_config: ValidatorConfig<KEY> =
                    ValidatorConfig::generated_from_seed_indexed([0u8; 32], node_id, 1);

                cur_validator_config
                    .public_key
                    .get_stake_table_entry(cur_validator_config.stake_value)
            })
            .collect();
        Self {
            total_nodes: NonZeroUsize::new(10).unwrap(),
            my_own_validator_config: ValidatorConfig::default(),
            known_nodes_with_stake: gen_known_nodes_with_stake,
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
}

fn default_start_delay_seconds() -> u64 {
    60
}
