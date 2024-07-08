use crate::service::client_message::{ClientMessage, InternalClientMessage};
use crate::service::server_message::ServerMessage;
use futures::future::Either;
use futures::{
    channel::mpsc::{self, Sender},
    FutureExt, SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use std::fmt;
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

#[cfg(test)]
mod tests {
    use super::{Error, StateClientMessageSender, Version01, STATIC_VER_0_1};
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
    use sequencer::Leaf;
    use std::sync::Arc;
    use tide_disco::App;

    struct TestState(Sender<InternalClientMessage>);

    impl StateClientMessageSender for TestState {
        fn sender(&self) -> Sender<InternalClientMessage> {
            self.0.clone()
        }
    }

    #[async_std::test]
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

        let data_state = DataState::new(
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

        let data_state = Arc::new(RwLock::new(data_state));
        let client_thread_state = Arc::new(RwLock::new(client_thread_state));
        let (block_detail_sender, block_detail_receiver) = mpsc::channel(32);
        let (leaf_sender, leaf_receiver) = mpsc::channel(32);
        let (_node_identity_sender, node_identity_receiver) = mpsc::channel(32);
        let (_voters_sender, voters_receiver) = mpsc::channel(32);

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
        ));

        let _leaf_retriever_handle = async_std::task::spawn(async move {
            // Alright, let's get some leaves, bro

            let client: surf_disco::Client<Error, Version01> = surf_disco::Client::new(
                "https://query.cappuccino.testnet.espresso.network/v0"
                    .parse()
                    .unwrap(),
            );

            let block_height_result = client.get("status/block-height").send().await;
            let block_height: u64 = if let Ok(block_height) = block_height_result {
                block_height
            } else {
                tracing::info!("block height request failed");
                return;
            };

            let start_block_height = block_height.saturating_sub(50);

            let mut leaf_sender = leaf_sender;
            let mut leaves = client
                .socket(&format!(
                    "availability/stream/leaves/{}",
                    start_block_height
                ))
                .subscribe::<Leaf>()
                .await
                .unwrap();

            loop {
                let leaf_result = leaves.next().await;
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
}
