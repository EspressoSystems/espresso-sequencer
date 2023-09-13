pub mod client;
pub mod config;

use async_lock::RwLock;
use hotshot_types::traits::{election::ElectionConfig, signature_key::SignatureKey};
use std::{
    io,
    io::ErrorKind,
    net::{IpAddr, SocketAddr},
};
use tide_disco::{Api, App};

use surf_disco::error::ClientError;
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

/// yeesh maybe we should just implement SignatureKey for this...
pub fn libp2p_generate_indexed_identity(seed: [u8; 32], index: u64) -> Keypair {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let new_seed = *hasher.finalize().as_bytes();
    let sk_bytes = SecretKey::try_from_bytes(new_seed).unwrap();
    <EdKeypair as From<SecretKey>>::from(sk_bytes).into()
}

#[derive(Default, Clone)]
struct OrchestratorState<KEY, ENTRY, ELECTION> {
    /// Tracks the latest node index we have generated a configuration for
    latest_index: u16,
    /// The network configuration
    config: NetworkConfig<KEY, ENTRY, ELECTION>,
    /// Whether nodes should start their HotShot instances
    /// Will be set to true once all nodes post they are ready to start
    start: bool,
    /// The total nodes that have posted they are ready to start
    pub nodes_connected: u64,
    /// connection to the web server
    client: Option<surf_disco::Client<ClientError>>,
}

impl<KEY: SignatureKey + 'static, ELECTION: ElectionConfig + 'static>
    OrchestratorState<KEY, KEY::StakeTableEntry, ELECTION>
{
    pub fn new(network_config: NetworkConfig<KEY, KEY::StakeTableEntry, ELECTION>) -> Self {
        let mut web_client = None;
        if network_config.web_server_config.is_some() {
            let base_url = "http://0.0.0.0/9000".to_string().parse().unwrap();
            web_client = Some(surf_disco::Client::<ClientError>::new(base_url));
        }
        OrchestratorState {
            latest_index: 0,
            config: network_config,
            start: false,
            nodes_connected: 0,
            client: web_client,
        }
    }
}

pub trait OrchestratorApi<KEY, ENTRY, ELECTION> {
    fn post_identity(&mut self, identity: IpAddr) -> Result<u16, ServerError>;
    fn post_getconfig(
        &mut self,
        node_index: u16,
    ) -> Result<NetworkConfig<KEY, ENTRY, ELECTION>, ServerError>;
    fn get_start(&self) -> Result<bool, ServerError>;
    fn post_ready(&mut self) -> Result<(), ServerError>;
    fn post_run_results(&mut self) -> Result<(), ServerError>;
}

impl<KEY, ELECTION> OrchestratorApi<KEY, KEY::StakeTableEntry, ELECTION>
    for OrchestratorState<KEY, KEY::StakeTableEntry, ELECTION>
where
    KEY: serde::Serialize + Clone + SignatureKey,
    ELECTION: serde::Serialize + Clone + Send,
{
    fn post_identity(&mut self, identity: IpAddr) -> Result<u16, ServerError> {
        let node_index = self.latest_index;
        self.latest_index += 1;

        // TODO https://github.com/EspressoSystems/HotShot/issues/850
        if usize::from(node_index) >= self.config.config.total_nodes.get() {
            return Err(ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Network has reached capacity".to_string(),
            });
        }

        //add new node's key to stake table
        if self.config.web_server_config.clone().is_some() {
            let new_key = KEY::generated_from_seed_indexed(self.config.seed, node_index.into()).0;
            let client_clone = self.client.clone().unwrap();
            async move {
                client_clone
                    .post::<()>("api/staketable")
                    .body_binary(&new_key)
                    .unwrap()
                    .send()
                    .await
            }
            .boxed();
        }

        if self.config.libp2p_config.clone().is_some() {
            let libp2p_config_clone = self.config.libp2p_config.clone().unwrap();
            // Designate node as bootstrap node and store its identity information
            if libp2p_config_clone.bootstrap_nodes.len()
                < libp2p_config_clone.num_bootstrap_nodes.try_into().unwrap()
            {
                let port_index = match libp2p_config_clone.index_ports {
                    true => node_index,
                    false => 0,
                };
                let socketaddr =
                    SocketAddr::new(identity, libp2p_config_clone.base_port + port_index);
                let keypair = libp2p_generate_indexed_identity(self.config.seed, node_index.into());
                self.config
                    .libp2p_config
                    .as_mut()
                    .unwrap()
                    .bootstrap_nodes
                    .push((socketaddr, keypair.to_protobuf_encoding().unwrap()));
            }
        }
        Ok(node_index)
    }

    // Assumes nodes will set their own index that they received from the
    // 'identity' endpoint
    fn post_getconfig(
        &mut self,
        _node_index: u16,
    ) -> Result<NetworkConfig<KEY, KEY::StakeTableEntry, ELECTION>, ServerError> {
        if self.config.libp2p_config.is_some() {
            let libp2p_config = self.config.clone().libp2p_config.unwrap();
            if libp2p_config.bootstrap_nodes.len()
                < libp2p_config.num_bootstrap_nodes.try_into().unwrap()
            {
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
        if self.nodes_connected >= self.config.config.known_nodes_with_stake.len().try_into().unwrap() {
            self.start = true;
        }
        Ok(())
    }

    fn post_run_results(&mut self) -> Result<(), ServerError> {
        Ok(())
    }
}

/// Sets up all API routes
fn define_api<KEY, ELECTION, ENTRY, State>() -> Result<Api<State, ServerError>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + OrchestratorApi<KEY, ENTRY, ELECTION>,
    KEY: serde::Serialize,
    ENTRY: serde::Serialize,
    ELECTION: serde::Serialize,
{
    let api_toml = toml::from_str::<toml::Value>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/api.toml"
    )))
    .expect("API file is not valid toml");
    let mut api = Api::<State, ServerError>::new(api_toml)?;
    api.post("postidentity", |req, state| {
        async move {
            let identity = req.string_param("identity")?.parse::<IpAddr>();
            if identity.is_err() {
                return Err(ServerError {
                    status: tide_disco::StatusCode::BadRequest,
                    message: "Identity is not a properly formed IP address".to_string(),
                });
            }
            state.post_identity(identity.unwrap())
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
pub async fn run_orchestrator<KEY, ELECTION>(
    network_config: NetworkConfig<KEY, KEY::StakeTableEntry, ELECTION>,
    host: IpAddr,
    port: u16,
) -> io::Result<()>
where
    KEY: SignatureKey + 'static + serde::Serialize,
    ELECTION: ElectionConfig + 'static + serde::Serialize,
{
    let api = define_api().map_err(|_e| io::Error::new(ErrorKind::Other, "Failed to define api"));

    let state: RwLock<OrchestratorState<KEY, KEY::StakeTableEntry, ELECTION>> =
        RwLock::new(OrchestratorState::new(network_config));

    let mut app = App::<RwLock<OrchestratorState<KEY, KEY::StakeTableEntry, ELECTION>>, ServerError>::with_state(state);
    app.register_module("api", api.unwrap())
        .expect("Error registering api");
    tracing::error!("lisening on {:?}:{:?}", host, port);
    app.serve(format!("http://{host}:{port}")).await
}
