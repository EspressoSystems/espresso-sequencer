//! Orchestrator for manipulating nodes and recording results during a run of `HotShot` tests

/// The orchestrator's clients
pub mod client;
/// Configuration for the orchestrator
pub mod config;

use std::{collections::HashSet, fs::OpenOptions, io, io::ErrorKind};

use async_lock::RwLock;
use client::{BenchResults, BenchResultsDownloadConfig};
use csv::Writer;
use futures::FutureExt;
use hotshot_types::{
    constants::Version01,
    traits::{election::ElectionConfig, signature_key::SignatureKey},
    PeerConfig,
};
use libp2p::{
    identity::{
        ed25519::{Keypair as EdKeypair, SecretKey},
        Keypair,
    },
    Multiaddr, PeerId,
};
use surf_disco::Url;
use tide_disco::{
    api::ApiError,
    error::ServerError,
    method::{ReadState, WriteState},
    Api, App, RequestError,
};
use vbs::{
    version::{StaticVersion, StaticVersionType},
    BinarySerializer,
};

use crate::config::NetworkConfig;

/// Orchestrator is not, strictly speaking, bound to the network; it can have its own versioning.
/// Orchestrator Version (major)
pub const ORCHESTRATOR_MAJOR_VERSION: u16 = 0;
/// Orchestrator Version (minor)
pub const ORCHESTRATOR_MINOR_VERSION: u16 = 1;
/// Orchestrator Version as a type
pub type OrchestratorVersion =
    StaticVersion<ORCHESTRATOR_MAJOR_VERSION, ORCHESTRATOR_MINOR_VERSION>;
/// Orchestrator Version as a type-binding instance
pub const ORCHESTRATOR_VERSION: OrchestratorVersion = StaticVersion {};

/// Generate an keypair based on a `seed` and an `index`
/// # Panics
/// This panics if libp2p is unable to generate a secret key from the seed
#[must_use]
pub fn libp2p_generate_indexed_identity(seed: [u8; 32], index: u64) -> Keypair {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let new_seed = *hasher.finalize().as_bytes();
    let sk_bytes = SecretKey::try_from_bytes(new_seed).unwrap();
    <EdKeypair as From<SecretKey>>::from(sk_bytes).into()
}

/// The state of the orchestrator
#[derive(Default, Clone)]
struct OrchestratorState<KEY: SignatureKey, ELECTION: ElectionConfig> {
    /// Tracks the latest node index we have generated a configuration for
    latest_index: u16,
    /// Tracks the latest temporary index we have generated for init validator's key pair
    tmp_latest_index: u16,
    /// The network configuration
    config: NetworkConfig<KEY, ELECTION>,
    /// The total nodes that have posted their public keys
    nodes_with_pubkey: u64,
    /// Whether the network configuration has been updated with all the peer's public keys/configs
    peer_pub_ready: bool,
    /// The set of index for nodes that have posted their public keys/configs
    pub_posted: HashSet<u64>,
    /// Whether nodes should start their HotShot instances
    /// Will be set to true once all nodes post they are ready to start
    start: bool,
    /// The total nodes that have posted they are ready to start
    nodes_connected: u64,
    /// The results of the benchmarks
    bench_results: BenchResults,
    /// The number of nodes that have posted their results
    nodes_post_results: u64,
}

impl<KEY: SignatureKey + 'static, ELECTION: ElectionConfig + 'static>
    OrchestratorState<KEY, ELECTION>
{
    /// create a new [`OrchestratorState`]
    pub fn new(network_config: NetworkConfig<KEY, ELECTION>) -> Self {
        OrchestratorState {
            latest_index: 0,
            tmp_latest_index: 0,
            config: network_config,
            nodes_with_pubkey: 0,
            peer_pub_ready: false,
            pub_posted: HashSet::new(),
            nodes_connected: 0,
            start: false,
            bench_results: BenchResults::default(),
            nodes_post_results: 0,
        }
    }

    /// Output the results to a csv file according to orchestrator state
    pub fn output_to_csv(&self) {
        let output_csv = BenchResultsDownloadConfig {
            commit_sha: self.config.commit_sha.clone(),
            total_nodes: self.config.config.num_nodes_with_stake.into(),
            da_committee_size: self.config.config.da_staked_committee_size,
            transactions_per_round: self.config.transactions_per_round,
            transaction_size: self.bench_results.transaction_size_in_bytes,
            rounds: self.config.rounds,
            leader_election_type: self.config.election_config_type_name.clone(),
            avg_latency_in_sec: self.bench_results.avg_latency_in_sec,
            minimum_latency_in_sec: self.bench_results.minimum_latency_in_sec,
            maximum_latency_in_sec: self.bench_results.maximum_latency_in_sec,
            throughput_bytes_per_sec: self.bench_results.throughput_bytes_per_sec,
            total_transactions_committed: self.bench_results.total_transactions_committed,
            total_time_elapsed_in_sec: self.bench_results.total_time_elapsed_in_sec,
            total_num_views: self.bench_results.total_num_views,
            failed_num_views: self.bench_results.failed_num_views,
        };
        // Open the CSV file in append mode
        let results_csv_file = OpenOptions::new()
            .create(true)
            .append(true) // Open in append mode
            .open("scripts/benchmarks_results/results.csv")
            .unwrap();
        // Open a file for writing
        let mut wtr = Writer::from_writer(results_csv_file);
        let _ = wtr.serialize(output_csv);
        let _ = wtr.flush();
        println!("Results successfully saved in scripts/benchmarks_results/results.csv");
    }
}

/// An api exposed by the orchestrator
pub trait OrchestratorApi<KEY: SignatureKey, ELECTION: ElectionConfig> {
    /// Post an identity to the orchestrator. Takes in optional
    /// arguments so others can identify us on the Libp2p network.
    /// # Errors
    /// If we were unable to serve the request
    fn post_identity(
        &mut self,
        libp2p_address: Option<Multiaddr>,
        libp2p_public_key: Option<PeerId>,
    ) -> Result<u16, ServerError>;
    /// post endpoint for each node's config
    /// # Errors
    /// if unable to serve
    fn post_getconfig(
        &mut self,
        _node_index: u16,
    ) -> Result<NetworkConfig<KEY, ELECTION>, ServerError>;
    /// get endpoint for the next available temporary node index
    /// # Errors
    /// if unable to serve
    fn get_tmp_node_index(&mut self) -> Result<u16, ServerError>;
    /// post endpoint for each node's public key
    /// # Errors
    /// if unable to serve
    fn register_public_key(
        &mut self,
        node_index: u64,
        pubkey: &mut Vec<u8>,
    ) -> Result<(), ServerError>;
    /// post endpoint for whether or not all peers public keys are ready
    /// # Errors
    /// if unable to serve
    fn peer_pub_ready(&self) -> Result<bool, ServerError>;
    /// get endpoint for the network config after all peers public keys are collected
    /// # Errors
    /// if unable to serve
    fn get_config_after_peer_collected(&self) -> Result<NetworkConfig<KEY, ELECTION>, ServerError>;
    /// get endpoint for whether or not the run has started
    /// # Errors
    /// if unable to serve
    fn get_start(&self) -> Result<bool, ServerError>;
    /// post endpoint for the results of the run
    /// # Errors
    /// if unable to serve
    fn post_run_results(&mut self, metrics: BenchResults) -> Result<(), ServerError>;
    /// post endpoint for whether or not all nodes are ready
    /// # Errors
    /// if unable to serve
    fn post_ready(&mut self) -> Result<(), ServerError>;
}

impl<KEY, ELECTION> OrchestratorApi<KEY, ELECTION> for OrchestratorState<KEY, ELECTION>
where
    KEY: serde::Serialize + Clone + SignatureKey + 'static,
    ELECTION: serde::Serialize + Clone + Send + ElectionConfig + 'static,
{
    /// Post an identity to the orchestrator. Takes in optional
    /// arguments so others can identify us on the Libp2p network.
    /// # Errors
    /// If we were unable to serve the request
    fn post_identity(
        &mut self,
        libp2p_address: Option<Multiaddr>,
        libp2p_public_key: Option<PeerId>,
    ) -> Result<u16, ServerError> {
        let node_index = self.latest_index;
        self.latest_index += 1;

        if usize::from(node_index) >= self.config.config.num_nodes_with_stake.get() {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Network has reached capacity".to_string(),
            });
        }

        // If the orchestrator is set up for libp2p and we have supplied the proper
        // Libp2p data, add our node to the list of bootstrap nodes.
        if self.config.libp2p_config.clone().is_some() {
            if let (Some(libp2p_public_key), Some(libp2p_address)) =
                (libp2p_public_key, libp2p_address)
            {
                // Push to our bootstrap nodes
                self.config
                    .libp2p_config
                    .as_mut()
                    .unwrap()
                    .bootstrap_nodes
                    .push((libp2p_public_key, libp2p_address));
            }
        }
        Ok(node_index)
    }

    // Assumes nodes will set their own index that they received from the
    // 'identity' endpoint
    fn post_getconfig(
        &mut self,
        _node_index: u16,
    ) -> Result<NetworkConfig<KEY, ELECTION>, ServerError> {
        Ok(self.config.clone())
    }

    // Assumes one node do not get twice
    fn get_tmp_node_index(&mut self) -> Result<u16, ServerError> {
        let tmp_node_index = self.tmp_latest_index;
        self.tmp_latest_index += 1;

        if usize::from(tmp_node_index) >= self.config.config.num_nodes_with_stake.get() {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Node index getter for key pair generation has reached capacity"
                    .to_string(),
            });
        }
        Ok(tmp_node_index)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn register_public_key(
        &mut self,
        node_index: u64,
        pubkey: &mut Vec<u8>,
    ) -> Result<(), ServerError> {
        if self.pub_posted.contains(&node_index) {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Node has already posted public key".to_string(),
            });
        }
        self.pub_posted.insert(node_index);

        // The guess is the first extra 12 bytes are from orchestrator serialization
        pubkey.drain(..12);
        let register_pub_key_with_stake = PeerConfig::<KEY>::from_bytes(pubkey).unwrap();
        self.config.config.known_nodes_with_stake[node_index as usize] =
            register_pub_key_with_stake;
        self.nodes_with_pubkey += 1;
        println!(
            "Node {:?} posted public key, now total num posted public key: {:?}",
            node_index, self.nodes_with_pubkey
        );
        if self.nodes_with_pubkey >= (self.config.config.num_nodes_with_stake.get() as u64) {
            self.peer_pub_ready = true;
        }
        Ok(())
    }

    fn peer_pub_ready(&self) -> Result<bool, ServerError> {
        if !self.peer_pub_ready {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Peer's public configs are not ready".to_string(),
            });
        }
        Ok(self.peer_pub_ready)
    }

    fn get_config_after_peer_collected(&self) -> Result<NetworkConfig<KEY, ELECTION>, ServerError> {
        if !self.peer_pub_ready {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Peer's public configs are not ready".to_string(),
            });
        }
        Ok(self.config.clone())
    }

    fn get_start(&self) -> Result<bool, ServerError> {
        // println!("{}", self.start);
        if !self.start {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Network is not ready to start".to_string(),
            });
        }
        Ok(self.start)
    }

    // Assumes nodes do not post 'ready' twice
    // TODO ED Add a map to verify which nodes have posted they're ready
    fn post_ready(&mut self) -> Result<(), ServerError> {
        self.nodes_connected += 1;
        println!("Nodes connected: {}", self.nodes_connected);
        if self.nodes_connected >= (self.config.config.num_nodes_with_stake.get() as u64) {
            self.start = true;
        }
        Ok(())
    }

    // Aggregates results of the run from all nodes
    fn post_run_results(&mut self, metrics: BenchResults) -> Result<(), ServerError> {
        if metrics.total_transactions_committed != 0 {
            // Deal with the bench results
            if self.bench_results.total_transactions_committed == 0 {
                self.bench_results = metrics;
            } else {
                // Deal with the bench results from different nodes
                let cur_metrics = self.bench_results.clone();
                self.bench_results.avg_latency_in_sec = (metrics.avg_latency_in_sec
                    * metrics.num_latency
                    + cur_metrics.avg_latency_in_sec * cur_metrics.num_latency)
                    / (metrics.num_latency + cur_metrics.num_latency);
                self.bench_results.num_latency += metrics.num_latency;
                self.bench_results.minimum_latency_in_sec = metrics
                    .minimum_latency_in_sec
                    .min(cur_metrics.minimum_latency_in_sec);
                self.bench_results.maximum_latency_in_sec = metrics
                    .maximum_latency_in_sec
                    .max(cur_metrics.maximum_latency_in_sec);
                self.bench_results.throughput_bytes_per_sec = metrics
                    .throughput_bytes_per_sec
                    .max(cur_metrics.throughput_bytes_per_sec);
                self.bench_results.total_transactions_committed = metrics
                    .total_transactions_committed
                    .max(cur_metrics.total_transactions_committed);
                assert_eq!(
                    metrics.transaction_size_in_bytes,
                    cur_metrics.transaction_size_in_bytes
                );
                self.bench_results.total_time_elapsed_in_sec = metrics
                    .total_time_elapsed_in_sec
                    .max(cur_metrics.total_time_elapsed_in_sec);
                self.bench_results.total_num_views =
                    metrics.total_num_views.min(cur_metrics.total_num_views);
                self.bench_results.failed_num_views =
                    metrics.failed_num_views.max(cur_metrics.failed_num_views);
            }
        }
        self.nodes_post_results += 1;
        if self.nodes_post_results >= (self.config.config.num_nodes_with_stake.get() as u64) {
            self.bench_results.printout();
            self.output_to_csv();
        }
        Ok(())
    }
}

/// Sets up all API routes
fn define_api<KEY: SignatureKey, ELECTION: ElectionConfig, State, VER: StaticVersionType>(
) -> Result<Api<State, ServerError, VER>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + OrchestratorApi<KEY, ELECTION>,
    KEY: serde::Serialize,
    ELECTION: serde::Serialize,
    VER: 'static,
{
    let api_toml = toml::from_str::<toml::Value>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/api.toml"
    )))
    .expect("API file is not valid toml");
    let mut api = Api::<State, ServerError, VER>::new(api_toml)?;
    api.post("post_identity", |req, state| {
        async move {
            // Read the bytes from the body
            let mut body_bytes = req.body_bytes();
            body_bytes.drain(..12);

            // Decode the libp2p data so we can add to our bootstrap nodes (if supplied)
            let Ok((libp2p_address, libp2p_public_key)) =
                vbs::Serializer::<Version01>::deserialize(&body_bytes)
            else {
                return Err(ServerError {
                    status: tide_disco::StatusCode::BadRequest,
                    message: "Malformed body".to_string(),
                });
            };

            // Call our state function to process the request
            state.post_identity(libp2p_address, libp2p_public_key)
        }
        .boxed()
    })?
    .post("post_getconfig", |req, state| {
        async move {
            let node_index = req.integer_param("node_index")?;
            state.post_getconfig(node_index)
        }
        .boxed()
    })?
    .post("get_tmp_node_index", |_req, state| {
        async move { state.get_tmp_node_index() }.boxed()
    })?
    .post("post_pubkey", |req, state| {
        async move {
            let node_index = req.integer_param("node_index")?;
            let mut pubkey = req.body_bytes();
            state.register_public_key(node_index, &mut pubkey)
        }
        .boxed()
    })?
    .get("peer_pubconfig_ready", |_req, state| {
        async move { state.peer_pub_ready() }.boxed()
    })?
    .get("get_config_after_peer_collected", |_req, state| {
        async move { state.get_config_after_peer_collected() }.boxed()
    })?
    .post(
        "post_ready",
        |_req, state: &mut <State as ReadState>::State| async move { state.post_ready() }.boxed(),
    )?
    .get("get_start", |_req, state| {
        async move { state.get_start() }.boxed()
    })?
    .post("post_results", |req, state| {
        async move {
            let metrics: Result<BenchResults, RequestError> = req.body_json();
            state.post_run_results(metrics.unwrap())
        }
        .boxed()
    })?;
    Ok(api)
}

/// Runs the orchestrator
/// # Errors
/// This errors if tide disco runs into an issue during serving
/// # Panics
/// This panics if unable to register the api with tide disco
pub async fn run_orchestrator<KEY, ELECTION>(
    network_config: NetworkConfig<KEY, ELECTION>,
    url: Url,
) -> io::Result<()>
where
    KEY: SignatureKey + 'static + serde::Serialize,
    ELECTION: ElectionConfig + 'static + serde::Serialize,
{
    let web_api =
        define_api().map_err(|_e| io::Error::new(ErrorKind::Other, "Failed to define api"));

    let state: RwLock<OrchestratorState<KEY, ELECTION>> =
        RwLock::new(OrchestratorState::new(network_config));

    let mut app = App::<RwLock<OrchestratorState<KEY, ELECTION>>, ServerError>::with_state(state);
    app.register_module::<ServerError, OrchestratorVersion>("api", web_api.unwrap())
        .expect("Error registering api");
    tracing::error!("listening on {:?}", url);
    app.serve(url, ORCHESTRATOR_VERSION).await
}
