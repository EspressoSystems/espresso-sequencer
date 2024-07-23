use crate::service::client_message::{ClientMessage, InternalClientMessage};
use crate::service::server_message::ServerMessage;
use futures::SinkExt;
use futures::{future::FutureExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::mpsc::{self, Sender};
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
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}

impl tide_disco::Error for Error {
    fn catch_all(_status: tide_disco::StatusCode, _msg: String) -> Self {
        todo!()
    }

    fn status(&self) -> tide_disco::StatusCode {
        todo!()
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
                let client_message_sender = state.sender();
                let (server_message_sender, server_message_receiver) = mpsc::channel();

                // Let's register ourselves with the Server
                if let Err(_) = client_message_sender
                    .send(InternalClientMessage::Connected(server_message_sender))
                {
                    todo!();
                }

                // We should receive a response from the server that identifies us
                // uniquely.
                let client_id =
                    if let Ok(ServerMessage::YouAre(client_id)) = server_message_receiver.recv() {
                        client_id
                    } else {
                        todo!();
                    };

                let (sink, mut stream) = socket.split();

                // Now we want to just auto-forward any server message to the client
                // in it's own thread, and we also want to aut forward any client
                // message to the server.
                let client_id = client_id.clone();
                let client_message_sender = client_message_sender.clone();
                let handle = async_std::task::spawn(async move {
                    let mut sink = sink;
                    while let Ok(message) = server_message_receiver.recv() {
                        if let Err(_) = sink.send(&message).await {
                            // we're closed at this point
                            break;
                        }
                    }
                });

                // Start forwarding message from the client
                while let Some(Ok(request)) = stream.next().await {
                    let internal_client_message =
                        request.to_internal_with_client_id(client_id.clone());
                    if let Err(_) = client_message_sender.send(internal_client_message) {
                        todo!();
                    }
                }

                // wait for the spawned task to finish
                handle.await;

                Ok(())
            }
            .boxed()
        },
    )?;
    Ok(api)
}

#[cfg(test)]
mod tests {
    use super::StateClientMessageSender;
    use crate::service::client_message::InternalClientMessage;
    use std::sync::mpsc::{self, Receiver, Sender};

    struct TestState(
        pub(crate) Sender<InternalClientMessage>,
        pub(crate) Receiver<InternalClientMessage>,
    );

    impl TestState {
        fn new() -> Self {
            let (sender, receiver) = mpsc::channel();
            TestState(sender, receiver)
        }
    }

    impl StateClientMessageSender for TestState {
        fn sender(&self) -> Sender<InternalClientMessage> {
            self.0.clone()
        }
    }

    unsafe impl Send for TestState {}
    unsafe impl Sync for TestState {}

    // Woo hoo
    #[test]
    fn test_api_creation() {
        let api = super::define_api::<TestState>();
        match api {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}
