pub mod create_inscriptions_api;

use crate::service::client_message::InternalClientMessage;
use crate::service::espresso_inscription::{EspressoInscription, InscriptionAndSignature};
use crate::service::server_message::ServerMessage;
use crate::service::storage::InscriptionPersistence;
use crate::service::{validate_inscription_and_signature, InscriptionVerificationError};
use async_std::task::JoinHandle;
use espresso_types::{BackoffParams, SeqTypes};
use futures::channel::mpsc::SendError;
use futures::select;
use futures::{
    channel::mpsc::{self, Sender},
    FutureExt, Sink, SinkExt, Stream, StreamExt,
};
use hotshot_query_service::availability::BlockQueryData;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tide_disco::socket::Connection;
use tide_disco::RequestError;
use tide_disco::{api::ApiError, Api};
use url::Url;
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

    InscriptionAndSignatureUnpack(RequestError),
    InvalidInscription(InscriptionVerificationError),

    TooManyRequests,
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

            Self::InscriptionAndSignatureUnpack(err) => {
                write!(f, "Failed to unpack InscriptionAndSignature: {}", err)
            }

            Self::InvalidInscription(err) => {
                write!(f, "Invalid Inscription: {}", err)
            }

            Self::TooManyRequests => {
                write!(f, "Too Many Requests")
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
        match self {
            Self::UnhandledSurfDisco(status, _) => *status,
            Self::UnhandledTideDisco(status, _) => *status,

            Self::InscriptionAndSignatureUnpack(_) => tide_disco::StatusCode::BAD_REQUEST,
            Self::InvalidInscription(_) => tide_disco::StatusCode::BAD_REQUEST,

            Self::TooManyRequests => tide_disco::StatusCode::TOO_MANY_REQUESTS,
        }
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

pub(crate) fn load_api<State: 'static, ApiVer: StaticVersionType + 'static>(
    default: &str,
) -> Result<Api<State, Error, ApiVer>, LoadApiError> {
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
#[async_trait::async_trait]
pub trait StateClientMessageSender<K> {
    /// [internal_client_message_sender] retrieves a [Sender] for sending
    /// messages to the server.
    fn internal_client_message_sender(&self) -> Sender<InternalClientMessage<K>>;

    /// [put_inscription] attempts to send an [EspressoInscription] to the server
    /// for processing.  If the server is over capacity, then it should return
    /// an error indicating that the service is rate limited.
    async fn put_inscription(&self, inscription: EspressoInscription) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum EndpointError {}

pub fn define_api<State>() -> Result<Api<State, Error, Version01>, DefineApiError>
where
    State: StateClientMessageSender<Sender<ServerMessage>> + Send + Sync + 'static,
{
    let mut api = load_api::<State, Version01>(include_str!("./inscriptions.toml"))?;

    api.with_version("0.0.1".parse().unwrap())
        .socket(
            "stream_inscriptions",
            move |_req, socket: Connection<ServerMessage, (), Error, Version01>, state| {
                async move {
                    // We consume socket messages here so we can more quickly
                    // realize if we are disconnected, and just in case the
                    // client actually sends us something.
                    let mut socket_stream = socket.clone();
                    let mut socket_sink = socket;

                    let mut internal_client_message_sender = state.internal_client_message_sender();
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

                    loop {
                        let mut next_server_message = server_message_receiver.next().fuse();
                        let mut next_client_message = socket_stream.next().fuse();

                        select! {
                            client_message = next_client_message => {
                                if let Some(Ok(_)) = client_message {
                                    tracing::debug!("received message from client, not expecting any input from the client, ignoring it");
                                } else {
                                    tracing::debug!("client disconnected: {:?}", client_id);
                                    // The client has disconnected, we need to exit the stream
                                    break;
                                }
                            },

                            server_message = next_server_message => {
                                let Some(server_message) = server_message else {
                                    break;
                                } ;

                                tracing::debug!(
                                    "received relay message from server, forwarding to client: {:?}, server message: {:?}",
                                    client_id,
                                    server_message,
                                );

                                // We want to forward the message to the client
                                if let Err(err) = socket_sink.send(&server_message).await {
                                    // This means that the socket is closed
                                    tracing::debug!(
                                        "failed to relay message to client, socket is closed: {}",
                                        err
                                    );
                                    break;
                                }


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
        )?
        .at("put_inscription", move |params, state| {
            async move {
                // Decode the InscriptionAndSignature from the request
                let inscription_and_signature = params
                    .body_auto::<InscriptionAndSignature, Version01>(STATIC_VER_0_1)
                    .map_err(Error::InscriptionAndSignatureUnpack)?;

                // Validate that the InscriptionAndSignature is valid
                validate_inscription_and_signature(&inscription_and_signature)
                    .map_err(Error::InvalidInscription)?;

                tracing::debug!(
                    "received valid inscription for put_inscription: {:?}",
                    inscription_and_signature
                );

                // attempts to send the InscriptionAndSignature to the server,
                // if we're over capacity, indicate that we are rate limited.
                // let mut put_inscription_sender = state.put_inscription_sender();

                // Attempt to send the message for processing, if the queue is
                // full, then return back a rate limiting warning.
                if let Err(err) =
                state.put_inscription(inscription_and_signature.inscription).await
                    // put_inscription_sender.try_send(inscription_and_signature.inscription)
                {
                    tracing::debug!(
                        "failed to send put_inscription to server, queue is likely full: {:?}",
                        err
                    );

                    return Err(Error::TooManyRequests);
                }

                // We have successfully sent the message to the server for processing
                tracing::debug!("successfully send put_inscription message to server");

                Ok(())
            }
            .boxed()
        })?;
    Ok(api)
}

/// BlockStreamRetriever is a general trait that allows for the retrieval of a
/// list of Blocks from a source. The specific implementation doesn't care about
/// the source, only that it is able to retrieve a stream of Blocks.
///
/// This allows us to swap the implementation of the [BlockStreamRetriever] for
/// testing purposes, or for newer sources in the future.
#[async_trait::async_trait]
pub trait BlockStreamRetriever {
    type Item;
    type ItemError: std::error::Error + Send;
    type Error: std::error::Error + Send;
    type Stream: Stream<Item = Result<Self::Item, Self::ItemError>> + Send + Unpin;
    type Future: Future<Output = Result<Self::Stream, Self::Error>> + Send;

    /// [retrieve_stream] retrieves a stream of [BlockQueryData]s from the source.  It
    /// expects the current block height to be provided so that it can determine
    /// the starting block height to retrieve the stream of [BlockQueryData]s from.
    ///
    /// It should check the current height of the chain so that it only needs
    /// to retrieve the number of older blocks that are needed, instead of
    /// starting from the beginning of time.
    fn retrieve_stream(&self, current_block_height: Option<u64>) -> Self::Future;
}

/// [HotshotQueryServiceBlockStreamRetriever] is a [BlockStreamRetriever] that
/// retrieves a stream of [Block]s from the Hotshot Query Service.  It expects
/// the base URL of the Hotshot Query Service to be provided so that it can
/// make the request to the Hotshot Query Service.
pub struct HotshotQueryServiceBlockStreamRetriever {
    base_url: Url,
}

impl HotshotQueryServiceBlockStreamRetriever {
    /// [new] creates a new [HotshotQueryServiceBlockStreamRetriever] that
    /// will use the given base [Url] to be able to retrieve the stream of
    /// [Block]s from the Hotshot Query Service.
    ///
    /// The [Url] is expected to point to the the API version root of the
    /// Hotshot Query Service.  Example:
    ///   https://example.com/v0
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }
}

impl BlockStreamRetriever for HotshotQueryServiceBlockStreamRetriever {
    type Item = BlockQueryData<SeqTypes>;
    type ItemError = hotshot_query_service::Error;
    type Error = hotshot_query_service::Error;
    type Stream = surf_disco::socket::Connection<
        BlockQueryData<SeqTypes>,
        surf_disco::socket::Unsupported,
        Self::ItemError,
        Version01,
    >;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Stream, Self::Error>> + Send>>;

    fn retrieve_stream(&self, current_block_height: Option<u64>) -> Self::Future {
        let client = surf_disco::Client::new(self.base_url.clone());
        async move {
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

            let blocks_stream_result = client
                .socket(&format!(
                    "availability/stream/blocks/{}",
                    start_block_height
                ))
                .subscribe::<BlockQueryData<SeqTypes>>()
                .await;

            let blocks_stream = match blocks_stream_result {
                Ok(blocks_stream) => blocks_stream,
                Err(err) => {
                    tracing::info!("retrieve blocks stream failed: {}", err);
                    return Err(err);
                }
            };

            Ok(blocks_stream)
        }
        .boxed()
    }
}

/// [RetrieveBlockStreamError] indicates the various failure conditions that can
/// occur when attempting to retrieve a stream of [Block]s using the
/// [ProcessProduceBlockStreamTask::retrieve_block_stream] function.
enum RetrieveBlockStreamError {
    /// [MaxAttemptsExceeded] indicates that the maximum number of attempts to
    /// attempt to retrieve the [Stream] of [Block]s has been exceeded.
    /// In this case, it doesn't make sense to continue to re-attempt to
    /// reconnect to the service, as it does not seem to be available.
    MaxAttemptsExceeded,
}

/// [ProcessProduceBlockStreamTask] is a task that produce a stream of [BlockQueryData]s
/// from the Hotshot Query Service.  It will attempt to retrieve the [BlockQueryData]s
/// from the Hotshot Query Service and then send them to the [Sink] provided.
pub struct ProcessProduceBlockStreamTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl ProcessProduceBlockStreamTask {
    /// [new] creates a new [ProcessProduceBlockStreamTask] that produces a
    /// stream of [BlockQueryData]s from the Hotshot Query Service.
    ///
    /// Calling this function will create an async task that will start
    /// processing immediately.  The task's handle will be stored in the
    /// returned state.
    pub fn new<R, Persistence, K>(
        block_stream_retriever: R,
        persistence: Arc<Persistence>,
        minimum_start_block_height: u64,
        block_sender: K,
    ) -> Self
    where
        R: BlockStreamRetriever<Item = BlockQueryData<SeqTypes>> + Send + Sync + 'static,
        K: Sink<BlockQueryData<SeqTypes>, Error = SendError>
            + Clone
            + Send
            + Sync
            + Unpin
            + 'static,
        Persistence: InscriptionPersistence + Send + Sync + 'static,
    {
        let task_handle = async_std::task::spawn(Self::connect_and_process_blocks(
            block_stream_retriever,
            persistence,
            minimum_start_block_height,
            block_sender,
        ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    async fn connect_and_process_blocks<R, Persistence, K>(
        block_stream_retriever: R,
        persistence: Arc<Persistence>,
        minimum_start_block_height: u64,
        block_sender: K,
    ) where
        R: BlockStreamRetriever<Item = BlockQueryData<SeqTypes>>,
        K: Sink<BlockQueryData<SeqTypes>, Error = SendError>
            + Clone
            + Send
            + Sync
            + Unpin
            + 'static,
        Persistence: InscriptionPersistence,
    {
        // We want to try and ensure that we are connected to the HotShot Query
        // Service, and are consuming blocks.
        // - If we are able to connect, then we can start relaying Blocks into
        //   the block sender.
        // - If we are not able to connect, then we should sleep, and retry
        //   until we are able to reconnect.  Each failure **should** make some
        //   noise so that if we are never able to reconnect, we are at least
        //   telling someone about it.
        // - If the task for consuming the blocks completes, then we should
        //   also attempt to reestablish the connection to start consuming
        //   the blocks again.

        loop {
            // Retrieve a stream
            let Ok(stream) = Self::retrieve_block_stream(
                &block_stream_retriever,
                persistence.clone(),
                minimum_start_block_height,
            )
            .await
            else {
                panic!("failed to retrieve block stream");
            };

            // Consume the blocks of a stream
            Self::process_consume_block_stream::<R, K>(stream, block_sender.clone()).await;
            tracing::warn!("block stream ended, will attempt to re-acquire block stream");
        }
    }

    /// [retrieve_block_stream] attempts to retrieve the Stream of Blocks from
    /// the given [BlockStreamRetriever].
    ///
    /// This function will loop on failure until it is able to retrieve the
    /// [Stream].  This does mean that it could potentially get in a state
    /// where it can loop indefinitely.
    ///
    /// This function also implements exponential backoff with a maximum
    /// delay of 5 seconds.
    async fn retrieve_block_stream<R, Persistence>(
        block_stream_receiver: &R,
        persistence: Arc<Persistence>,
        minimum_start_block_height: u64,
    ) -> Result<R::Stream, RetrieveBlockStreamError>
    where
        R: BlockStreamRetriever<Item = BlockQueryData<SeqTypes>>,
        Persistence: InscriptionPersistence,
    {
        let backoff_params = BackoffParams::default();
        let mut delay = Duration::from_millis(50);

        for attempt in 1..=100 {
            let block_height = persistence
                .retrieve_last_received_block()
                .await
                .ok()
                // we want to start **after** the last block we received.
                .map(|(height, _)| std::cmp::max(height + 1, minimum_start_block_height));

            let block_stream_result = block_stream_receiver.retrieve_stream(block_height).await;

            let block_stream = match block_stream_result {
                Err(error) => {
                    // We failed to retrieve the stream. We will try again, but we
                    // should sleep for a bit before so as not to overwhelm the
                    // service.

                    delay = backoff_params.backoff(delay);

                    if delay == Duration::ZERO {
                        panic!("backoff is invalid")
                    }
                    tracing::warn!(
                        "attempt {attempt} to connect to block stream failed with error {error} sleeping for {:?}", delay
                    );
                    async_std::task::sleep(delay).await;
                    continue;
                }

                Ok(blocks_stream) => blocks_stream,
            };

            return Ok(block_stream);
        }

        Err(RetrieveBlockStreamError::MaxAttemptsExceeded)
    }

    /// [process_consume_block_stream] produces a stream of [Block]s from the
    /// Hotshot Query Service.  It will attempt to retrieve the [Block]s from the
    /// Hotshot Query Service and then send them to the [Sink] provided.  If the
    /// [Sink] is closed, or if the Stream ends prematurely, then the function
    /// will return.
    async fn process_consume_block_stream<R, K>(blocks_stream: R::Stream, block_sender: K)
    where
        R: BlockStreamRetriever<Item = BlockQueryData<SeqTypes>>,
        K: Sink<BlockQueryData<SeqTypes>, Error = SendError>
            + Clone
            + Send
            + Sync
            + Unpin
            + 'static,
    {
        let mut block_sender = block_sender;
        let mut blocks_stream = blocks_stream;

        loop {
            let block_result = blocks_stream.next().await;
            let block = if let Some(Ok(block)) = block_result {
                block
            } else {
                tracing::info!("block stream closed");
                break;
            };

            let block_send_result = block_sender.send(block).await;
            if let Err(err) = block_send_result {
                tracing::info!("block sender closed: {}", err);
                break;
            }
        }
    }
}

/// [Drop] implementation for [ProcessProduceBlockStreamTask] that will cancel
/// the task if it hasn't already been completed.
impl Drop for ProcessProduceBlockStreamTask {
    fn drop(&mut self) {
        if let Some(task_handle) = self.task_handle.take() {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

#[cfg(test)]
mod tests {}
