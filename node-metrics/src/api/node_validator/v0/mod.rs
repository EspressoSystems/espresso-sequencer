use crate::service::client_message::{ClientMessage, InternalClientMessage};
use crate::service::data_state::{LocationDetails, NodeIdentity};
use crate::service::server_message::ServerMessage;
use futures::future::Either;
use futures::{
    channel::mpsc::{self, Sender},
    FutureExt, SinkExt, StreamExt,
};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::light_client::{CircuitField, StateVerKey};
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::signature_key::SignatureKey;
use hotshot_types::traits::{signature_key::StakeTableEntryType, stake_table::StakeTableScheme};
use hotshot_types::PeerConfig;
use prometheus_parse::Scrape;
use sequencer::state::FeeAccount;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::BufRead;
use std::str::FromStr;
use tide_disco::socket::Connection;
use tide_disco::{api::ApiError, Api};
use vbs::version::{StaticVersion, StaticVersionType, Version};

/// CONSTANT for protocol major version
pub const VERSION_MAJ: u16 = 0;

/// CONSTANT for protocol minor version
pub const VERSION_MIN: u16 = 1;

pub const VERSION_0_1: Version = Version {
    major: VERSION_MAJ,
    minor: VERSION_MIN,
};

/// Constant for the version of this API.
pub const BASE_VERSION: Version = VERSION_0_1;

/// Specific type for version 0.1
pub type Version01 = StaticVersion<VERSION_MAJ, VERSION_MIN>;

// Static instance of the Version01 type
pub const STATIC_VER_0_1: Version01 = StaticVersion {};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    UnhandledTideDisco(tide_disco::StatusCode, String),
    UnhandledSurfDisco(surf_disco::StatusCode, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnhandledSurfDisco(status, msg) => {
                write!(f, "Unhandled Surf Disco Error: {} - {}", status, msg)
            }

            Self::UnhandledTideDisco(status, msg) => {
                write!(f, "Unhandled Tide Disco Error: {} - {}", status, msg)
            }
        }
    }
}

impl std::error::Error for Error {}

impl tide_disco::Error for Error {
    fn catch_all(status: tide_disco::StatusCode, msg: String) -> Self {
        Self::UnhandledTideDisco(status, msg)
    }

    fn status(&self) -> tide_disco::StatusCode {
        tide_disco::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl surf_disco::Error for Error {
    fn catch_all(status: surf_disco::StatusCode, msg: String) -> Self {
        Self::UnhandledSurfDisco(status, msg)
    }

    fn status(&self) -> surf_disco::StatusCode {
        surf_disco::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug)]
pub enum LoadApiError {
    Toml(toml::de::Error),
    Api(ApiError),
}

impl From<toml::de::Error> for LoadApiError {
    fn from(err: toml::de::Error) -> Self {
        LoadApiError::Toml(err)
    }
}

impl From<ApiError> for LoadApiError {
    fn from(err: ApiError) -> Self {
        LoadApiError::Api(err)
    }
}

pub(crate) fn load_api<State: 'static, Ver: StaticVersionType + 'static>(
    default: &str,
) -> Result<Api<State, Error, Ver>, LoadApiError> {
    let toml: toml::Value = toml::from_str(default)?;
    Ok(Api::new(toml)?)
}

#[derive(Debug)]
pub enum LoadTomlError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for LoadTomlError {
    fn from(err: std::io::Error) -> Self {
        LoadTomlError::Io(err)
    }
}

impl From<toml::de::Error> for LoadTomlError {
    fn from(err: toml::de::Error) -> Self {
        LoadTomlError::Toml(err)
    }
}

impl From<std::str::Utf8Error> for LoadTomlError {
    fn from(err: std::str::Utf8Error) -> Self {
        LoadTomlError::Utf8(err)
    }
}

#[derive(Debug)]
pub enum DefineApiError {
    LoadApiError(LoadApiError),
    LoadTomlError(LoadTomlError),
    ApiError(ApiError),
}

impl From<LoadApiError> for DefineApiError {
    fn from(err: LoadApiError) -> Self {
        DefineApiError::LoadApiError(err)
    }
}

impl From<LoadTomlError> for DefineApiError {
    fn from(err: LoadTomlError) -> Self {
        DefineApiError::LoadTomlError(err)
    }
}

impl From<ApiError> for DefineApiError {
    fn from(err: ApiError) -> Self {
        DefineApiError::ApiError(err)
    }
}

/// [StateClientMessageSender] allows for the retrieval of a [Sender] for sending
/// messages received from the client to the Server for request processing.
pub trait StateClientMessageSender {
    fn sender(&self) -> Sender<InternalClientMessage>;
}

#[derive(Debug)]
pub enum EndpointError {}

pub fn define_api<State>() -> Result<Api<State, Error, Version01>, DefineApiError>
where
    State: StateClientMessageSender + Send + Sync + 'static,
{
    let mut api = load_api::<State, Version01>(include_str!("./node_validator.toml"))?;

    api.with_version("0.0.1".parse().unwrap()).socket(
        "details",
        move |_req, socket: Connection<ServerMessage, ClientMessage, Error, Version01>, state| {
            async move {
                let mut socket_stream = socket.clone();
                let mut socket_sink = socket;

                let mut internal_client_message_sender = state.sender();
                let (server_message_sender, mut server_message_receiver) = mpsc::channel(32);

                // Let's register ourselves with the Server
                if let Err(err) = internal_client_message_sender
                    .send(InternalClientMessage::Connected(server_message_sender))
                    .await
                {
                    // This means that the client_message_sender is closed
                    // we need to exit the stream.
                    tracing::info!(
                        "client message sender is closed before first message: {}",
                        err
                    );
                    return Ok(());
                }

                // We should receive a response from the server that identifies us
                // uniquely.
                let client_id = if let Some(ServerMessage::YouAre(client_id)) =
                    server_message_receiver.next().await
                {
                    client_id
                } else {
                    // The channel is closed, and this client should be removed
                    // we need to exit the stream
                    tracing::info!("server message receiver closed before first message",);
                    return Ok(());
                };

                // We want to start these futures outside of the loop.  If we
                // don't do this then every iteration will not be guaranteed
                // to not skip a message.
                let mut next_client_message = socket_stream.next();
                let mut next_server_message = server_message_receiver.next();

                loop {
                    match futures::future::select(next_client_message, next_server_message).await {
                        Either::Left((client_request, remaining_server_message)) => {
                            let client_request = if let Some(client_request) = client_request {
                                client_request
                            } else {
                                // The client has disconnected, we need to exit the stream
                                tracing::info!("client message has disconnected");
                                break;
                            };

                            let client_request = if let Ok(client_request) = client_request {
                                client_request
                            } else {
                                // This indicates that there was a more
                                // specific error with the socket message.
                                // This error can be various, and may be
                                // recoverable depending on the actual nature
                                // of the error.  We will treat it as
                                // unrecoverable for now.
                                break;
                            };

                            let internal_client_message =
                                client_request.to_internal_with_client_id(client_id);
                            if let Err(err) = internal_client_message_sender
                                .send(internal_client_message)
                                .await
                            {
                                // This means that the client_message_sender is closed
                                tracing::info!("client message sender is closed: {}", err);
                                break;
                            }

                            // let's queue up the next client message to receive
                            next_client_message = socket_stream.next();
                            next_server_message = remaining_server_message;
                        }
                        Either::Right((server_message, remaining_client_message)) => {
                            // Alright, we have a server message, we want to forward it
                            // to the down-stream client.

                            let server_message = if let Some(server_message) = server_message {
                                server_message
                            } else {
                                // The server has disconnected, we need to exit the stream
                                break;
                            };

                            // We want to forward the message to the client
                            if let Err(err) = socket_sink.send(&server_message).await {
                                // This means that the socket is closed
                                tracing::info!("socket is closed: {}", err);
                                break;
                            }

                            // let's queue up the next server message to receive
                            next_server_message = server_message_receiver.next();
                            next_client_message = remaining_client_message;
                        }
                    }
                }

                // We don't actually care if this fails or not, as we're exiting
                // this function anyway, and these Senders and Receivers will
                // automatically be dropped.
                _ = internal_client_message_sender
                    .send(InternalClientMessage::Disconnected(client_id))
                    .await;

                Ok(())
            }
            .boxed()
        },
    )?;
    Ok(api)
}

#[derive(Debug, Deserialize)]
pub struct PublishHotShotConfig {
    pub known_nodes_with_stake: Vec<PeerConfig<BLSPubKey>>,
}

/// [get_stake_table_from_sequencer] retrieves the stake table from the
/// Sequencer.  It expects a [surf_disco::Client] to be provided so that it can
/// make the request to the Hotshot Query Service.  It will return a
/// [StakeTable] that is populated with the data retrieved from the Hotshot
/// Query Service.
pub async fn get_stake_table_from_sequencer(
    client: surf_disco::Client<hotshot_query_service::Error, Version01>,
) -> Result<StakeTable<BLSPubKey, StateVerKey, CircuitField>, hotshot_query_service::Error> {
    let request = client
        .get("config/hotshot")
        // We need to set the Accept header, otherwise the Content-Type
        // will be application/octet-stream, and we won't be able to
        // deserialize the response.
        .header("Accept", "application/json");
    let stake_table_result = request.send().await;

    let public_hot_shot_config: PublishHotShotConfig = match stake_table_result {
        Ok(public_hot_shot_config) => public_hot_shot_config,
        Err(err) => {
            tracing::info!("retrieve stake table request failed: {}", err);
            return Err(err);
        }
    };

    let mut stake_table = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(
        public_hot_shot_config.known_nodes_with_stake.len(),
    );

    for node in public_hot_shot_config.known_nodes_with_stake.into_iter() {
        stake_table
            .register(
                *node.stake_table_entry.key(),
                node.stake_table_entry.stake(),
                node.state_ver_key,
            )
            .expect("registering stake table entry");
    }

    stake_table.advance();
    stake_table.advance();

    Ok(stake_table)
}

pub enum GetNodeIdentityFromUrlError {
    Url(url::ParseError),
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    NoNodeIdentity,
}

impl From<url::ParseError> for GetNodeIdentityFromUrlError {
    fn from(err: url::ParseError) -> Self {
        GetNodeIdentityFromUrlError::Url(err)
    }
}

impl From<reqwest::Error> for GetNodeIdentityFromUrlError {
    fn from(err: reqwest::Error) -> Self {
        GetNodeIdentityFromUrlError::Reqwest(err)
    }
}

impl From<std::io::Error> for GetNodeIdentityFromUrlError {
    fn from(err: std::io::Error) -> Self {
        GetNodeIdentityFromUrlError::Io(err)
    }
}

pub async fn get_node_identity_from_url(
    url: url::Url,
) -> Result<NodeIdentity, GetNodeIdentityFromUrlError> {
    let client = reqwest::Client::new();

    let completed_url = url.join("v0/status/metrics")?;
    let request = client.get(completed_url).build()?;
    let response = client.execute(request).await?;
    let response_bytes = response.bytes().await?;

    let buffered_response = std::io::BufReader::new(&*response_bytes);
    let scrape = prometheus_parse::Scrape::parse(buffered_response.lines())?;

    if let Some(node_identity) = node_identity_from_scrape(scrape) {
        Ok(node_identity)
    } else {
        Err(GetNodeIdentityFromUrlError::NoNodeIdentity)
    }
}

/// [stream_leaves_from_hotshot_query_service] retrieves a stream of
/// [sequencer::Leaf]s from the Hotshot Query Service.  It expects a
/// [current_block_height] to be provided so that it can determine the starting
/// block height to begin streaming from.  No matter what the value of
/// [current_block_height] is the stream will always check what the latest
/// block height is on the hotshot query service.  It will then attempt to
/// pull as few Leafs as it needs from the stream.
pub async fn stream_leaves_from_hotshot_query_service(
    current_block_height: Option<u64>,
    client: surf_disco::Client<hotshot_query_service::Error, Version01>,
) -> Result<
    impl futures::Stream<Item = Result<sequencer::Leaf, hotshot_query_service::Error>> + Unpin,
    hotshot_query_service::Error,
> {
    let block_height_result = client.get("status/block-height").send().await;
    let block_height: u64 = match block_height_result {
        Ok(block_height) => block_height,
        Err(err) => {
            tracing::info!("retrieve block height request failed: {}", err);
            return Err(err);
        }
    };

    let latest_block_start = block_height.saturating_sub(50);
    let start_block_height = if let Some(known_height) = current_block_height {
        std::cmp::min(known_height, latest_block_start)
    } else {
        latest_block_start
    };

    let leaves_stream_result = client
        .socket(&format!(
            "availability/stream/leaves/{}",
            start_block_height
        ))
        .subscribe::<sequencer::Leaf>()
        .await;

    let leaves_stream = match leaves_stream_result {
        Ok(leaves_stream) => leaves_stream,
        Err(err) => {
            tracing::info!("retrieve leaves stream failed: {}", err);
            return Err(err);
        }
    };

    Ok(leaves_stream)
}

pub fn populate_node_identity_from_scrape(node_identity: &mut NodeIdentity, scrape: Scrape) {
    // Handle General Information Population

    // Determine the key for the "consensus_node_identity_general" sample
    let node_identity_general_key = scrape
        .docs
        .iter()
        .find(|(_, key)| key == &"node_identity_general")
        .map(|(key, _)| key);

    if let Some(node_identity_general_key) = node_identity_general_key {
        let node_identity_general_sample = scrape
            .samples
            .iter()
            .find(|sample| &sample.metric == node_identity_general_key);

        if let Some(node_identity_general_sample) = node_identity_general_sample {
            node_identity.name = node_identity_general_sample
                .labels
                .get("name")
                .map(|s| s.into());
            node_identity.company = node_identity_general_sample
                .labels
                .get("company_name")
                .map(|s| s.into());
            node_identity.network_type = node_identity_general_sample
                .labels
                .get("network_type")
                .map(|s| s.into());
            node_identity.node_type = node_identity_general_sample
                .labels
                .get("node_type")
                .map(|s| s.into());
            node_identity.operating_system = node_identity_general_sample
                .labels
                .get("operating_system")
                .map(|s| s.into());
            // Wallet Address
            let parsed_wallet_address_result = node_identity_general_sample
                .labels
                .get("wallet")
                .map(FeeAccount::from_str);

            match parsed_wallet_address_result {
                Some(Ok(parsed_wallet_address)) => {
                    node_identity.wallet_address = Some(parsed_wallet_address);
                }
                Some(Err(err)) => {
                    tracing::info!("parsing wallet address failed: {}", err);
                }
                None => {}
            }
        }
    }

    let node_identity_location_key = scrape
        .docs
        .iter()
        .find(|(_, key)| key == &"node_identity_location")
        .map(|(key, _)| key);
    if let Some(node_identity_location_key) = node_identity_location_key {
        let node_identity_location_sample = scrape
            .samples
            .iter()
            .find(|sample| &sample.metric == node_identity_location_key);

        // We either have an existing location, or we'd potentially like to create
        // one.

        if let Some(node_identity_location_sample) = node_identity_location_sample {
            let mut location = node_identity
                .location
                .take()
                .unwrap_or(LocationDetails::new(None, None));
            location.country = node_identity_location_sample
                .labels
                .get("country")
                .map(|s| s.into());

            let latitude = node_identity_location_sample
                .labels
                .get("latitude")
                .map(|s| s.parse::<f64>());
            let longitude = node_identity_location_sample
                .labels
                .get("latitude")
                .map(|s| s.parse::<f64>());

            if let (Some(Ok(latitude)), Some(Ok(longitude))) = (latitude, longitude) {
                location.coords = Some((latitude, longitude));
            }

            // Are there any details populated?
            if location.country.is_some() || location.coords.is_some() {
                node_identity.location = Some(location);
            } else {
                node_identity.location = None;
            }
        }
    }
}

pub fn node_identity_from_scrape(scrape: Scrape) -> Option<NodeIdentity> {
    let node_key = scrape
        .docs
        .iter()
        .find(|(_, key)| key == &"node")
        .map(|(key, _)| key);

    let node_key = node_key?;

    let node_sample = scrape
        .samples
        .iter()
        .find(|sample| &sample.metric == node_key);

    let node_sample = node_sample?;

    let public_key_string = node_sample.labels.get("key")?;

    //  create the Tagged Base 64 Public Key representation
    let tagged_base64 =
        if let Ok(tagged_base64) = tagged_base64::TaggedBase64::parse(public_key_string) {
            tagged_base64
        } else {
            return None;
        };

    // Now we can take those bytes and we can create a Public Key from them.
    let public_key = match BLSPubKey::from_bytes(tagged_base64.value().as_ref()) {
        Ok(public_key) => public_key,
        Err(err) => {
            // We couldn't parse the public key, so we can't create a NodeIdentity.
            tracing::info!("parsing public key failed: {}", err);
            return None;
        }
    };

    let mut node_identity = NodeIdentity::from_public_key(public_key);
    populate_node_identity_from_scrape(&mut node_identity, scrape);

    Some(node_identity)
}

#[cfg(test)]
mod tests {
    use super::{
        get_stake_table_from_sequencer, stream_leaves_from_hotshot_query_service, Error,
        StateClientMessageSender, STATIC_VER_0_1,
    };
    use crate::service::{
        client_id::ClientId,
        client_message::InternalClientMessage,
        client_state::{
            process_distribute_block_detail_handling_stream,
            process_distribute_node_identity_handling_stream,
            process_distribute_voters_handling_stream, process_internal_client_message_stream,
            ClientThreadState,
        },
        data_state::{process_leaf_stream, DataState},
    };
    use async_std::sync::RwLock;
    use futures::{
        channel::mpsc::{self, Sender},
        SinkExt, StreamExt,
    };
    use sequencer::state::FeeAccount;
    use std::{
        io::{BufRead, BufReader},
        str::FromStr,
        sync::Arc,
    };
    use tide_disco::App;

    struct TestState(Sender<InternalClientMessage>);

    impl StateClientMessageSender for TestState {
        fn sender(&self) -> Sender<InternalClientMessage> {
            self.0.clone()
        }
    }

    #[async_std::test]
    #[ignore]
    async fn test_api_creation() {
        let node_validator_api_result = super::define_api::<TestState>();

        let node_validator_api = match node_validator_api_result {
            Ok(api) => api,
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        };

        let (sender, receiver) = mpsc::channel(32);
        let mut app: App<TestState, Error> = App::with_state(TestState(sender));
        let register_module_result = app.register_module("node-validator", node_validator_api);

        if let Err(e) = register_module_result {
            panic!("Error: {:?}", e);
        }

        let mut data_state = DataState::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        let client_thread_state = ClientThreadState::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            ClientId::from_count(1),
        );

        let client = surf_disco::Client::new(
            "https://query.cappuccino.testnet.espresso.network/v0"
                .parse()
                .unwrap(),
        );

        let get_stake_table_result = get_stake_table_from_sequencer(client.clone()).await;
        let stake_table = get_stake_table_result.unwrap();
        data_state.replace_stake_table(stake_table);

        let data_state = Arc::new(RwLock::new(data_state));
        let client_thread_state = Arc::new(RwLock::new(client_thread_state));
        let (block_detail_sender, block_detail_receiver) = mpsc::channel(32);
        let (leaf_sender, leaf_receiver) = mpsc::channel(32);
        let (_node_identity_sender, node_identity_receiver) = mpsc::channel(32);
        let (voters_sender, voters_receiver) = mpsc::channel(32);

        let _process_internal_client_message_handle =
            async_std::task::spawn(process_internal_client_message_stream(
                receiver,
                data_state.clone(),
                client_thread_state.clone(),
            ));

        let _process_distribute_block_detail_handle =
            async_std::task::spawn(process_distribute_block_detail_handling_stream(
                client_thread_state.clone(),
                block_detail_receiver,
            ));

        let _process_distribute_node_identity_handle =
            async_std::task::spawn(process_distribute_node_identity_handling_stream(
                client_thread_state.clone(),
                node_identity_receiver,
            ));

        let _process_distribute_voters_handle = async_std::task::spawn(
            process_distribute_voters_handling_stream(client_thread_state.clone(), voters_receiver),
        );

        let _process_leaf_stream_handle = async_std::task::spawn(process_leaf_stream(
            leaf_receiver,
            data_state.clone(),
            block_detail_sender,
            voters_sender,
        ));

        let _leaf_retriever_handle = async_std::task::spawn(async move {
            // Alright, let's get some leaves, bro

            let client = client;

            let mut leaf_stream = stream_leaves_from_hotshot_query_service(None, client)
                .await
                .unwrap();

            let mut leaf_sender = leaf_sender;

            loop {
                let leaf_result = leaf_stream.next().await;
                let leaf = if let Some(Ok(leaf)) = leaf_result {
                    leaf
                } else {
                    tracing::info!("leaf stream closed");
                    break;
                };

                let leaf_send_result = leaf_sender.send(leaf).await;
                if let Err(err) = leaf_send_result {
                    tracing::info!("leaf sender closed: {}", err);
                    break;
                }
            }
        });

        let _app_serve_result = app.serve("0.0.0.0:9000", STATIC_VER_0_1).await;
    }

    fn example_prometheus_output() -> &'static str {
        "# HELP consensus_cdn_num_failed_messages num_failed_messages
# TYPE consensus_cdn_num_failed_messages counter
consensus_cdn_num_failed_messages 0
# HELP consensus_current_view current_view
# TYPE consensus_current_view gauge
consensus_current_view 7
# HELP consensus_invalid_qc invalid_qc
# TYPE consensus_invalid_qc gauge
consensus_invalid_qc 0
# HELP consensus_last_decided_time last_decided_time
# TYPE consensus_last_decided_time gauge
consensus_last_decided_time 1720537017
# HELP consensus_last_decided_view last_decided_view
# TYPE consensus_last_decided_view gauge
consensus_last_decided_view 4
# HELP consensus_last_synced_block_height last_synced_block_height
# TYPE consensus_last_synced_block_height gauge
consensus_last_synced_block_height 4
# HELP consensus_libp2p_num_connected_peers num_connected_peers
# TYPE consensus_libp2p_num_connected_peers gauge
consensus_libp2p_num_connected_peers 4
# HELP consensus_libp2p_num_failed_messages num_failed_messages
# TYPE consensus_libp2p_num_failed_messages counter
consensus_libp2p_num_failed_messages 0
# HELP consensus_node node
# TYPE consensus_node gauge
consensus_node{key=\"BLS_VER_KEY~bQszS-QKYvUij2g20VqS8asttGSb95NrTu2PUj0uMh1CBUxNy1FqyPDjZqB29M7ZbjWqj79QkEOWkpga84AmDYUeTuWmy-0P1AdKHD3ehc-dKvei78BDj5USwXPJiDUlCxvYs_9rWYhagaq-5_LXENr78xel17spftNd5MA1Mw5U\"} 1
# HELP consensus_node_identity_general node_identity_general
# TYPE consensus_node_identity_general gauge
consensus_node_identity_general{company_name=\"Espresso Systems\",name=\"sequencer0\",network_type=\"local\",node_type=\"espresso-sequencer 0.1\",operating_system=\"Linux 5.15.153.1\",wallet=\"0x00000000000000000000000000000000\"} 1
# HELP consensus_node_identity_location node_identity_location
# TYPE consensus_node_identity_location gauge
consensus_node_identity_location{country=\"US\",latitude=\"-40.7128\",longitude=\"-74.0060\"} 1
# HELP consensus_node_index node_index
# TYPE consensus_node_index gauge
consensus_node_index 4
# HELP consensus_number_of_empty_blocks_proposed number_of_empty_blocks_proposed
# TYPE consensus_number_of_empty_blocks_proposed counter
consensus_number_of_empty_blocks_proposed 1
# HELP consensus_number_of_timeouts number_of_timeouts
# TYPE consensus_number_of_timeouts counter
consensus_number_of_timeouts 0
# HELP consensus_number_of_timeouts_as_leader number_of_timeouts_as_leader
# TYPE consensus_number_of_timeouts_as_leader counter
consensus_number_of_timeouts_as_leader 0
# HELP consensus_number_of_views_per_decide_event number_of_views_per_decide_event
# TYPE consensus_number_of_views_per_decide_event histogram
consensus_number_of_views_per_decide_event_bucket{le=\"0.005\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.01\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.025\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.05\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.1\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.25\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"0.5\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"1\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"2.5\"} 0
consensus_number_of_views_per_decide_event_bucket{le=\"5\"} 4
consensus_number_of_views_per_decide_event_bucket{le=\"10\"} 4
consensus_number_of_views_per_decide_event_bucket{le=\"+Inf\"} 4
consensus_number_of_views_per_decide_event_sum 12
consensus_number_of_views_per_decide_event_count 4
# HELP consensus_number_of_views_since_last_decide number_of_views_since_last_decide
# TYPE consensus_number_of_views_since_last_decide gauge
consensus_number_of_views_since_last_decide 4
# HELP consensus_outstanding_transactions outstanding_transactions
# TYPE consensus_outstanding_transactions gauge
consensus_outstanding_transactions 0
# HELP consensus_outstanding_transactions_memory_size outstanding_transactions_memory_size
# TYPE consensus_outstanding_transactions_memory_size gauge
consensus_outstanding_transactions_memory_size 0
# HELP consensus_version version
# TYPE consensus_version gauge
consensus_version{desc=\"20240701-15-gbd0957fd-dirty\",rev=\"bd0957fddad19caab010dc59e5a92bc1c95cbc07\",timestamp=\"1980-01-01T00:00:00.000000000Z\"} 1
# HELP consensus_view_duration_as_leader view_duration_as_leader
# TYPE consensus_view_duration_as_leader histogram
consensus_view_duration_as_leader_bucket{le=\"0.005\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.01\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.025\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.05\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.1\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.25\"} 0
consensus_view_duration_as_leader_bucket{le=\"0.5\"} 0
consensus_view_duration_as_leader_bucket{le=\"1\"} 0
consensus_view_duration_as_leader_bucket{le=\"2.5\"} 1
consensus_view_duration_as_leader_bucket{le=\"5\"} 1
consensus_view_duration_as_leader_bucket{le=\"10\"} 1
consensus_view_duration_as_leader_bucket{le=\"+Inf\"} 1
consensus_view_duration_as_leader_sum 2
consensus_view_duration_as_leader_count 1"
    }

    #[test]
    fn test_prometheus_scraping_example() {
        let example_input = example_prometheus_output();

        let buffered_reader = BufReader::new(example_input.as_bytes());
        let lines = buffered_reader.lines();

        let scrape_result = prometheus_parse::Scrape::parse(lines);

        assert!(scrape_result.is_ok());
        let scrape = scrape_result.unwrap();

        let node_identity_general_key = scrape
            .docs
            .iter()
            .find(|(_, key)| key == &"node_identity_general")
            .map(|(key, _)| key);
        let node_identity_location_key = scrape
            .docs
            .iter()
            .find(|(_, key)| key == &"node_identity_location")
            .map(|(key, _)| key);

        assert!(node_identity_general_key.is_some());
        assert!(node_identity_location_key.is_some());

        let node_identity_general_key = node_identity_general_key.unwrap();
        let node_identity_location_key = node_identity_location_key.unwrap();

        // Let's look for the general_info
        let node_identity_general = scrape
            .samples
            .iter()
            .find(|sample| &sample.metric == node_identity_general_key);

        let node_identity_location = scrape
            .samples
            .iter()
            .find(|sample| &sample.metric == node_identity_location_key);

        assert!(node_identity_general.is_some());
        assert!(node_identity_location.is_some());

        let node_identity_general = node_identity_general.unwrap();
        let node_identity_location = node_identity_location.unwrap();

        assert_eq!(
            node_identity_general.labels.get("company_name"),
            Some("Espresso Systems")
        );
        assert_eq!(node_identity_general.labels.get("name"), Some("sequencer0"));
        assert_eq!(
            node_identity_general.labels.get("network_type"),
            Some("local")
        );
        assert_eq!(
            node_identity_general.labels.get("node_type"),
            Some("espresso-sequencer 0.1")
        );
        assert_eq!(
            node_identity_general.labels.get("node_type"),
            Some("espresso-sequencer 0.1")
        );
        assert_eq!(
            node_identity_general.labels.get("operating_system"),
            Some("Linux 5.15.153.1")
        );
        assert_eq!(
            node_identity_general.labels.get("wallet"),
            Some("0x00000000000000000000000000000000")
        );

        assert_eq!(node_identity_location.labels.get("country"), Some("US"));
        assert_eq!(
            node_identity_location.labels.get("latitude"),
            Some("-40.7128")
        );
        assert_eq!(
            node_identity_location.labels.get("longitude"),
            Some("-74.0060")
        );

        print!("{:?}", scrape);
    }

    #[test]
    fn test_node_identity_from_scrape() {
        let example_input = example_prometheus_output();

        let buffered_reader = BufReader::new(example_input.as_bytes());
        let lines = buffered_reader.lines();

        let scrape_result = prometheus_parse::Scrape::parse(lines);

        assert!(scrape_result.is_ok());
        let scrape = scrape_result.unwrap();

        let node_identity = super::node_identity_from_scrape(scrape);

        assert!(node_identity.is_some());
        let node_identity = node_identity.unwrap();

        assert_eq!(
            node_identity.company(),
            &Some("Espresso Systems".to_string())
        );
        assert_eq!(node_identity.name(), &Some("sequencer0".to_string()));
        assert_eq!(node_identity.network_type(), &Some("local".to_string()));
        assert_eq!(
            node_identity.node_type(),
            &Some("espresso-sequencer 0.1".to_string())
        );
        assert_eq!(
            node_identity.operating_system(),
            &Some("Linux 5.15.153.1".to_string())
        );
        assert_eq!(
            node_identity.wallet_address(),
            &Some(FeeAccount::from_str("0x00000000000000000000000000000000").unwrap())
        );

        assert!(node_identity.location().is_some());
        let node_identity_location = node_identity.location().unwrap();

        assert_eq!(node_identity_location.country(), &Some("US".to_string()));
        assert_eq!(node_identity_location.coords, Some((-40.7128, -74.0060)));
    }
}
