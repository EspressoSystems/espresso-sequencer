//! Orchestrator for manipulating nodes and recording results during a run of `HotShot` tests

/// The orchestrator's clients
pub mod client;
/// Configuration for the orchestrator
pub mod config;

use async_lock::RwLock;
use hotshot_types::traits::{election::ElectionConfig, signature_key::SignatureKey};
use std::{io, io::ErrorKind};
use tide_disco::{Api, App};

use surf_disco::Url;
use tide_disco::{
    api::ApiError,
    error::ServerError,
    method::{ReadState, WriteState},
};

use futures::FutureExt;

use crate::config::NetworkConfig;

use libp2p::identity::{
    ed25519::{Keypair as EdKeypair, SecretKey},
    Keypair,
};

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
    /// The network configuration
    config: NetworkConfig<KEY, ELECTION>,
    /// Whether nodes should start their HotShot instances
    /// Will be set to true once all nodes post they are ready to start
    start: bool,
    /// The total nodes that have posted they are ready to start
    pub nodes_connected: u64,
}

impl<KEY: SignatureKey + 'static, ELECTION: ElectionConfig + 'static>
    OrchestratorState<KEY, ELECTION>
{
    /// create a new [`OrchestratorState`]
    pub fn new(network_config: NetworkConfig<KEY, ELECTION>) -> Self {
        OrchestratorState {
            config: network_config,
            start: false,
            nodes_connected: 0,
        }
    }
}

/// An api exposed by the orchestrator
pub trait OrchestratorApi<KEY: SignatureKey, ELECTION: ElectionConfig> {
    /// post endpoint for each node's config
    /// # Errors
    /// if unable to serve
    fn post_getconfig(
        &mut self,
        node_index: u16,
    ) -> Result<NetworkConfig<KEY, ELECTION>, ServerError>;
    /// get endpoint for whether or not the run has started
    /// # Errors
    /// if unable to serve
    fn get_start(&self) -> Result<bool, ServerError>;
    /// post endpoint for whether or not all nodes are ready
    /// # Errors
    /// if unable to serve
    fn post_ready(&mut self) -> Result<(), ServerError>;
    /// post endpoint for the results of the run
    /// # Errors
    /// if unable to serve
    fn post_run_results(&mut self) -> Result<(), ServerError>;
}

impl<KEY, ELECTION> OrchestratorApi<KEY, ELECTION> for OrchestratorState<KEY, ELECTION>
where
    KEY: serde::Serialize + Clone + SignatureKey,
    ELECTION: serde::Serialize + Clone + Send + ElectionConfig,
{
    // Assumes nodes will set their own index that they received from the
    // 'identity' endpoint
    fn post_getconfig(
        &mut self,
        _node_index: u16,
    ) -> Result<NetworkConfig<KEY, ELECTION>, ServerError> {
        if self.config.libp2p_config.is_some() {
            let libp2p_config = self.config.clone().libp2p_config.unwrap();
            if libp2p_config.bootstrap_nodes.len() < libp2p_config.num_bootstrap_nodes {
                return Err(ServerError {
                    status: tide_disco::StatusCode::BadRequest,
                    message: "Not enough bootstrap nodes have registered".to_string(),
                });
            }
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
        if self.nodes_connected
            >= self
                .config
                .config
                .known_nodes_with_stake
                .len()
                .try_into()
                .unwrap()
        {
            self.start = true;
        }
        Ok(())
    }

    fn post_run_results(&mut self) -> Result<(), ServerError> {
        Ok(())
    }
}

/// Sets up all API routes
fn define_api<KEY: SignatureKey, ELECTION: ElectionConfig, State>(
) -> Result<Api<State, ServerError>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + OrchestratorApi<KEY, ELECTION>,
    KEY: serde::Serialize,
    ELECTION: serde::Serialize,
{
    let api_toml = toml::from_str::<toml::Value>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/api.toml"
    )))
    .expect("API file is not valid toml");
    let mut api = Api::<State, ServerError>::new(api_toml)?;
    api.post("post_getconfig", |req, state| {
        async move {
            let node_index = req.integer_param("node_index")?;
            state.post_getconfig(node_index)
        }
        .boxed()
    })?
    .post("postready", |_req, state| {
        async move { state.post_ready() }.boxed()
    })?
    .get("getstart", |_req, state| {
        async move { state.get_start() }.boxed()
    })?
    .post("postresults", |_req, state| {
        async move { state.post_run_results() }.boxed()
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
    app.register_module("api", web_api.unwrap())
        .expect("Error registering api");
    tracing::error!("listening on {:?}", url);
    app.serve(url).await
}
