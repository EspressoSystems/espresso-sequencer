use crate::state_signature::StateSignatureScheme;

use super::{LightClientState, StateSignatureRequestBody};
use async_compatibility_layer::channel::OneShotReceiver;
use async_std::sync::RwLock;
use clap::Args;
use ethers::types::U256;
use futures::FutureExt;
use hotshot_stake_table::vec_based::config::FieldType;
use hotshot_types::light_client::{StateSignature, StateVerKey};
use jf_primitives::signatures::SignatureScheme;
use jf_utils::to_bytes;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
};
use tide_disco::{
    api::ApiError,
    error::ServerError,
    method::{ReadState, WriteState},
    Api, App, Error as _, StatusCode,
};
use url::Url;

/// The state signatures package is a light client state and its signatures collected
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateSignaturesPackage {
    /// The state for this signatures package
    state: LightClientState,
    /// The collected signatures
    signatures: HashMap<StateVerKey, StateSignature>,

    /// Total stakes associated with the signer
    accumulated_weight: U256,
}

/// State that checks the light client state update and the signature collection
#[derive(Default)]
struct StateRelayServerState {
    /// Minimum weight to form an available state signature package
    threshold: U256,
    /// Stake table
    known_nodes: HashMap<StateVerKey, U256>,
    /// Signatures packages for each block height
    packages: HashMap<u64, StateSignaturesPackage>,

    /// The latest state signatures package whose total weight exceeds the threshold
    latest_available_package: Option<StateSignaturesPackage>,
    /// The block height of the latest available state signature package
    latest_block_height: Option<u64>,

    /// A ordered queue of block heights, used for garbage collection.
    queue: BTreeSet<u64>,

    /// shutdown signal
    shutdown: Option<OneShotReceiver<()>>,
}

impl StateRelayServerState {
    pub fn new(threshold: U256) -> Self {
        Self {
            threshold,
            ..Default::default()
        }
    }

    pub fn with_shutdown_signal(mut self, shutdown_listener: Option<OneShotReceiver<()>>) -> Self {
        if self.shutdown.is_some() {
            panic!("A shutdown signal is already registered and can not be registered twice");
        }
        self.shutdown = shutdown_listener;
        self
    }
}

// TODO(Chengyu): move this `RwLock` inside `StateRelayServerState` so that when nodes are submitting
//                signatures, it won't block the prover from fetching the available signatures.
type State = RwLock<StateRelayServerState>;
type Error = ServerError;

pub trait StateRelayServerDataSource {
    /// Get the latest available signatures package.
    /// # Errors
    /// Errors if there's no available signatures package.
    fn get_latest_signature_package(&self) -> Result<StateSignaturesPackage, Error>;

    /// Post a signature to the relay server
    /// # Errors
    /// Errors if the signature is invalid, already posted, or no longer needed.
    fn post_signature(
        &mut self,
        key: StateVerKey,
        state: LightClientState,
        signature: StateSignature,
    ) -> Result<(), Error>;
}

impl StateRelayServerDataSource for StateRelayServerState {
    fn get_latest_signature_package(&self) -> Result<StateSignaturesPackage, Error> {
        match &self.latest_available_package {
            Some(package) => Ok(package.clone()),
            None => Err(tide_disco::error::ServerError::catch_all(
                StatusCode::NotFound,
                "The light client state signatures are not ready.".to_owned(),
            )),
        }
    }

    fn post_signature(
        &mut self,
        key: StateVerKey,
        state: LightClientState,
        signature: StateSignature,
    ) -> Result<(), Error> {
        if (state.block_height as u64) <= self.latest_block_height.unwrap_or(0) {
            return Err(tide_disco::error::ServerError::catch_all(
                StatusCode::Locked,
                "The posted signature is no longer needed.".to_owned(),
            ));
        }
        let one = U256::one();
        let weight = self.known_nodes.get(&key).unwrap_or(&one);
        // TODO(Chengyu): We don't know where to fetch the stake table yet.
        // .ok_or(tide_disco::error::ServerError::catch_all(
        //     StatusCode::Unauthorized,
        //     "The posted key is not found in the stake table.".to_owned(),
        // ))?;
        let state_msg: [FieldType; 7] = (&state).into();
        if StateSignatureScheme::verify(&(), &key, state_msg, &signature).is_err() {
            return Err(tide_disco::error::ServerError::catch_all(
                StatusCode::BadRequest,
                "The posted signature is not valid.".to_owned(),
            ));
        }
        let block_height = state.block_height as u64;
        // TODO(Chengyu): this serialization should be removed once `LightClientState` implements `Eq`.
        let bytes = to_bytes!(&state).unwrap();
        let package = self.packages.entry(block_height).or_insert_with(|| {
            self.queue.insert(block_height);
            StateSignaturesPackage {
                state,
                signatures: Default::default(),
                accumulated_weight: U256::from(0),
            }
        });
        let incoming_bytes = to_bytes!(&package.state).unwrap();
        if bytes != incoming_bytes {
            return Err(tide_disco::error::ServerError::catch_all(
                StatusCode::BadRequest,
                "The posted light client is not compatible with the one previously stored."
                    .to_owned(),
            ));
        }
        match package.signatures.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(tide_disco::error::ServerError::catch_all(
                    StatusCode::BadRequest,
                    "A signature is already posted at this block height for this key.".to_owned(),
                ))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                tracing::info!("Accepting new signature for block height {}.", block_height);
                entry.insert(signature);
                package.accumulated_weight += *weight;
            }
        }

        if package.accumulated_weight >= self.threshold {
            tracing::info!(
                "State signature package at block height {} is ready to serve.",
                block_height
            );
            self.latest_block_height = Some(block_height);
            self.latest_available_package = self.packages.remove(&block_height);
            while let Some(height) = self.queue.pop_first() {
                if height == block_height {
                    break;
                }
                self.packages.remove(&height);
            }
        }
        Ok(())
    }
}

/// configurability options for the web server
#[derive(Args, Default)]
pub struct Options {
    #[arg(
        long = "state-relay-server-api-path",
        env = "STATE_RELAY_SERVER_API_PATH"
    )]
    /// path to API
    pub api_path: Option<PathBuf>,
}

/// Set up APIs for relay server
fn define_api<State>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + StateRelayServerDataSource,
{
    let mut api = match &options.api_path {
        Some(path) => Api::<State, Error>::from_file(path)?,
        None => {
            let toml: toml::Value = toml::from_str(include_str!(
                "../../api/state_relay_server.toml"
            ))
            .map_err(|err| ApiError::CannotReadToml {
                reason: err.to_string(),
            })?;
            Api::<State, Error>::new(toml)?
        }
    };

    api.get("getlateststate", |_req, state| {
        async move { state.get_latest_signature_package() }.boxed()
    })?
    .post("poststatesignature", |req, state| {
        async move {
            let StateSignatureRequestBody {
                key,
                state: lcstate,
                signature,
            } = req
                .body_auto::<StateSignatureRequestBody>()
                .map_err(Error::from_request_error)?;
            state.post_signature(key, lcstate, signature)
        }
        .boxed()
    })?;

    Ok(api)
}

pub async fn run_relay_server(
    shutdown_listener: Option<OneShotReceiver<()>>,
    url: Url,
) -> std::io::Result<()> {
    let options = Options::default();

    let api = define_api(&options).unwrap();

    // We don't have a stake table yet, putting some temporary value here.
    let threshold = U256::from(3u64);
    let state =
        State::new(StateRelayServerState::new(threshold).with_shutdown_signal(shutdown_listener));
    let mut app = App::<State, Error>::with_state(state);

    app.register_module("api", api).unwrap();

    let app_future = app.serve(url);

    app_future.await
}
