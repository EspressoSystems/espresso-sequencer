use crate::api::inscriptions::v0::Version01;

use super::{
    client_id::ClientId,
    client_message::InternalClientMessage,
    data_state::DataState,
    espresso_inscription::{EspressoInscription, HexSignature, InscriptionAndSignatureFromService},
    server_message::ServerMessage,
    storage::InscriptionPersistence,
    ESPRESSO_EIP712_DOMAIN,
};
use alloy::{
    signers::{local::PrivateKeySigner, SignerSync},
    sol_types::SolStruct,
};
use async_std::{
    prelude::FutureExt,
    sync::{RwLock, RwLockWriteGuard},
    task::JoinHandle,
};
use espresso_types::{NamespaceId, Transaction};
use futures::{channel::mpsc::SendError, Sink, SinkExt, Stream, StreamExt};
use governor::{
    clock,
    state::{InMemoryState, NotKeyed},
    RateLimiter,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use surf_disco::Client;
use url::Url;

/// ClientState represents the service state of the connected clients.
/// It maintains and represents the connected clients, and their subscriptions.
// This state is meant to be managed in a separate thread that assists with
// processing and updating of individual client states.
pub struct ClientState<K> {
    client_id: ClientId,
    sender: K,
}

impl<K> ClientState<K> {
    /// Create a new ClientState with the given client_id and receiver.
    pub fn new(client_id: ClientId, sender: K) -> Self {
        Self { client_id, sender }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn sender(&self) -> &K {
        &self.sender
    }
}

/// [ClientThreadState] represents the state of all of the active client
/// connections connected to the service. This state governs which clients
/// are connected, and what subscriptions they have setup.
pub struct ClientThreadState<K> {
    clients: HashMap<ClientId, ClientState<K>>,
    connection_id_counter: ClientId,
}

impl<K> ClientThreadState<K> {
    pub fn new(
        clients: HashMap<ClientId, ClientState<K>>,
        connection_id_counter: ClientId,
    ) -> Self {
        Self {
            clients,
            connection_id_counter,
        }
    }
}

/// [drop_client_client_thread_state_write_guard] is a utility function for
/// cleaning up the [ClientThreadState]
fn drop_client_client_thread_state_write_guard<K>(
    client_id: &ClientId,
    client_thread_state_write_guard: &mut RwLockWriteGuard<ClientThreadState<K>>,
) -> Option<ClientState<K>> {
    client_thread_state_write_guard.clients.remove(client_id)
}

/// [drop_client_no_lock_guard] is a utility function for cleaning up the [ClientThreadState]
/// when a client is detected as disconnected.
async fn drop_client_no_lock_guard<K>(
    client_id: &ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
) -> Option<ClientState<K>> {
    let mut client_thread_state_write_lock_guard = client_thread_state.write().await;

    drop_client_client_thread_state_write_guard(
        client_id,
        &mut client_thread_state_write_lock_guard,
    )
}

/// [HandleConnectedError] represents the scope of errors that can be
/// returned from the [handle_client_message_connected] function.
#[derive(Debug)]
pub enum HandleConnectedError {
    ClientSendError(SendError),
}

impl std::fmt::Display for HandleConnectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleConnectedError::ClientSendError(err) => {
                write!(f, "handle connected error: client send error: {}", err)
            }
        }
    }
}

impl std::error::Error for HandleConnectedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HandleConnectedError::ClientSendError(err) => Some(err),
        }
    }
}

/// [handle_client_message_connected] is a function that processes the client
/// message to connect a client to the service.
pub async fn handle_client_message_connected<K, Persistence>(
    mut sender: K,
    data_state: Arc<RwLock<DataState<Persistence>>>,
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
) -> Result<ClientId, HandleConnectedError>
where
    K: Sink<ServerMessage, Error = SendError> + Clone + Unpin,
{
    let client_id = {
        let mut client_thread_state_write_lock_guard = client_thread_state.write().await;

        client_thread_state_write_lock_guard.connection_id_counter += 1;
        let client_id = client_thread_state_write_lock_guard.connection_id_counter;

        client_thread_state_write_lock_guard.clients.insert(
            client_id,
            ClientState {
                client_id,
                sender: sender.clone(),
            },
        );

        client_id
    };

    // Send the client their new id.
    if let Err(err) = sender.send(ServerMessage::YouAre(client_id)).await {
        // We need to remove drop the client now.
        drop_client_no_lock_guard(&client_id, client_thread_state.clone()).await;
        return Err(HandleConnectedError::ClientSendError(err));
    }

    let current_inscriptions = {
        let data_state_read_lock_guard = data_state.read().await;
        data_state_read_lock_guard.current_inscriptions()
    };

    for inscription_and_chain_details in current_inscriptions {
        if let Err(err) = sender
            .send(ServerMessage::LatestInscription(Arc::new(
                inscription_and_chain_details,
            )))
            .await
        {
            // We need to remove drop the client now.
            drop_client_no_lock_guard(&client_id, client_thread_state.clone()).await;
            return Err(HandleConnectedError::ClientSendError(err));
        }
    }

    Ok(client_id)
}

/// [handle_client_message_disconnected] is a function that processes the client
/// message to disconnect a client from the service.
pub async fn handle_client_message_disconnected<K>(
    client_id: ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
) {
    // We might receive an implicit disconnect when attempting to
    // send a message, as the receiving channel might be closed.
    drop_client_no_lock_guard(&client_id, client_thread_state.clone()).await;
}

pub enum HandlePutInscriptionError {
    ClientSendError(SendError),
}

/// [ProcessClientMessageError] represents the scope of errors that can be
/// returned from the [process_client_message] function.
#[derive(Debug)]
pub enum ProcessClientMessageError {
    Connected(HandleConnectedError),
}

impl From<HandleConnectedError> for ProcessClientMessageError {
    fn from(err: HandleConnectedError) -> Self {
        ProcessClientMessageError::Connected(err)
    }
}

impl std::fmt::Display for ProcessClientMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessClientMessageError::Connected(err) => {
                write!(f, "process client message error: connected: {}", err)
            }
        }
    }
}

impl std::error::Error for ProcessClientMessageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessClientMessageError::Connected(err) => Some(err),
        }
    }
}

/// [drop_failed_client_sends] is a function that will drop all of the failed
/// client sends from the client thread state.
async fn drop_failed_client_sends<K>(
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
    failed_client_sends: Vec<ClientId>,
) {
    // Let's acquire our write lock
    let mut client_thread_state_write_lock_guard = client_thread_state.write().await;

    // We want to drop all of the failed clients.
    // There's an optimization to be had here
    for client_id in failed_client_sends {
        drop_client_client_thread_state_write_guard(
            &client_id,
            &mut client_thread_state_write_lock_guard,
        );
    }
}

/// [process_client_message] is a function that processes the client message
/// and processes the message accordingly.
///
/// The [DataState] is provided and is used only as a Read lock to distribute
/// the current state of the system to the clients upon request.
///
/// The [ClientThreadState] is provided as it needs to be updated with new
/// subscriptions / new connections depending on the incoming
/// [InternalClientMessage]
pub async fn process_client_message<K, Persistence>(
    message: InternalClientMessage<K>,
    data_state: Arc<RwLock<DataState<Persistence>>>,
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
) -> Result<(), ProcessClientMessageError>
where
    K: Sink<ServerMessage, Error = SendError> + Clone + Unpin,
{
    match message {
        InternalClientMessage::Connected(sender) => {
            handle_client_message_connected(sender, data_state, client_thread_state).await?;
            Ok(())
        }

        InternalClientMessage::Disconnected(client_id) => {
            handle_client_message_disconnected(client_id, client_thread_state).await;
            Ok(())
        }
    }
}

/// InternalClientMessageProcessingTask represents an async task for
/// InternalClientMessages, and making the appropriate updates to the
/// [ClientThreadState] and [DataState].
pub struct InternalClientMessageProcessingTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl InternalClientMessageProcessingTask {
    /// new creates a new [InternalClientMessageProcessingTask] with the
    /// given internal_client_message_receiver, data_state, and
    /// client_thread_state.
    ///
    /// Calling this function will start an async task that will start
    /// processing.  The handle for the async task is stored within the
    /// returned state.
    pub fn new<S, Persistence, K>(
        internal_client_message_receiver: S,
        data_state: Arc<RwLock<DataState<Persistence>>>,
        client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
    ) -> Self
    where
        S: Stream<Item = InternalClientMessage<K>> + Send + Sync + Unpin + 'static,
        K: Sink<ServerMessage, Error = SendError> + Clone + Send + Sync + Unpin + 'static,
        Persistence: Send + Sync + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_internal_client_message_stream(
            internal_client_message_receiver,
            data_state.clone(),
            client_thread_state.clone(),
        ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [process_internal_client_message_stream] is a function that processes the
    /// client handling stream. This stream is responsible for managing the state
    /// of the connected clients, and their subscriptions.
    async fn process_internal_client_message_stream<S, Persistence, K>(
        mut stream: S,
        data_state: Arc<RwLock<DataState<Persistence>>>,
        client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
    ) where
        S: Stream<Item = InternalClientMessage<K>> + Unpin,
        K: Sink<ServerMessage, Error = SendError> + Clone + Unpin,
    {
        loop {
            let message_result = stream.next().await;
            let message = if let Some(message) = message_result {
                message
            } else {
                tracing::error!("internal client message handler closed.");
                panic!("InternalClientMessageProcessingTask stream closed, unable to process new requests from clients.");
            };

            if let Err(err) =
                process_client_message(message, data_state.clone(), client_thread_state.clone())
                    .await
            {
                tracing::info!(
                    "internal client message processing encountered an error: {}",
                    err,
                );
            }
        }
    }
}

/// [drop] implementation for [InternalClientMessageProcessingTask] that will
/// cancel the task if it is still running.
impl Drop for InternalClientMessageProcessingTask {
    fn drop(&mut self) {
        let task_handle = self.task_handle.take();
        if let Some(task_handle) = task_handle {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

/// [handle_received_inscription] is a function that processes received
/// inscriptions and will attempt to distribute the message to all of the
/// clients that are connected.
async fn handle_received_inscription<K>(
    client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
    server_message: ServerMessage,
) where
    K: Sink<ServerMessage, Error = SendError> + Clone + Unpin,
{
    tracing::debug!("received inscription, beginning to distribute to connected clients");
    let client_senders: Vec<(ClientId, K)> = {
        let client_thread_state_read_lock_guard = client_thread_state.read().await;
        client_thread_state_read_lock_guard
            .clients
            .iter()
            .map(|(client_id, client_state)| (*client_id, client_state.sender.clone()))
            .collect()
    };

    let client_send_result_future = client_senders.into_iter().map(|(client_id, sender)| {
        let server_message = server_message.clone();
        async move {
            let mut sender = sender;
            let send_result = sender.send(server_message).await;

            (client_id, send_result)
        }
    });

    let client_send_results = futures::future::join_all(client_send_result_future).await;

    // These are the clients we failed to send the message to.  We copy these
    // here so we can drop our read lock.
    let failed_client_sends = client_send_results
        .into_iter()
        .filter(|(_, send_result)| send_result.is_err())
        .map(|(client_id, _)| client_id)
        .collect::<Vec<_>>();

    if failed_client_sends.is_empty() {
        return;
    }

    drop_failed_client_sends(client_thread_state, failed_client_sends).await;
}

/// [ProcessDistributeInscriptionHandlingTask] represents an async task for
/// processing the incoming [Inscription] and distributing them to all clients.
pub struct ProcessDistributeServerMessageHandlingTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl ProcessDistributeServerMessageHandlingTask {
    /// [new] creates a new [ProcessDistributeInscriptionHandlingTask] with the
    /// given client_thread_state and block_detail_receiver.
    ///
    /// Calling this function will start an async task that will start
    /// processing.  The handle for the async task is stored within the
    /// returned state.
    pub fn new<S, K>(
        client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
        server_message_receiver: S,
    ) -> Self
    where
        S: Stream<Item = ServerMessage> + Send + Sync + Unpin + 'static,
        K: Sink<ServerMessage, Error = SendError> + Clone + Send + Sync + Unpin + 'static,
    {
        let task_handle =
            async_std::task::spawn(Self::process_distribute_server_message_handling_stream(
                client_thread_state.clone(),
                server_message_receiver,
            ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [process_distribut_server_message_handling_stream] is a function that
    /// processes the the [Stream] of incoming [ServerMessage] and distributes
    /// them to all connected clients.
    async fn process_distribute_server_message_handling_stream<S, K>(
        client_thread_state: Arc<RwLock<ClientThreadState<K>>>,
        mut stream: S,
    ) where
        S: Stream<Item = ServerMessage> + Unpin,
        K: Sink<ServerMessage, Error = SendError> + Clone + Unpin,
    {
        loop {
            let server_message = match stream.next().await {
                Some(server_message) => server_message,
                None => {
                    tracing::error!(
                        "block detail stream closed.  shutting down client handling stream.",
                    );
                    return;
                }
            };

            handle_received_inscription(client_thread_state.clone(), server_message).await
        }
    }
}

/// [drop] implementation for [ProcessDistributeInscriptionHandlingTask] that will
/// cancel the task if it is still running.
impl Drop for ProcessDistributeServerMessageHandlingTask {
    fn drop(&mut self) {
        let task_handle = self.task_handle.take();
        if let Some(task_handle) = task_handle {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

/// [ProcessRecordPutInscriptionRequestTask] represents an async task for
/// processing the incoming put_inscription requests and recording them to the
/// persistence layer.
pub struct ProcessRecordPutInscriptionRequestTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl ProcessRecordPutInscriptionRequestTask {
    pub fn new<S, Persistence>(
        data_state: Arc<RwLock<DataState<Persistence>>>,
        inscription_receiver: S,
    ) -> Self
    where
        S: Stream<Item = EspressoInscription> + Send + Sync + Unpin + 'static,
        Persistence: InscriptionPersistence + Send + Sync + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_record_put_inscription_request(
            data_state,
            inscription_receiver,
        ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    async fn process_record_put_inscription_request<S, Persistence>(
        data_state: Arc<RwLock<DataState<Persistence>>>,
        mut stream: S,
    ) where
        S: Stream<Item = EspressoInscription> + Unpin,
        Persistence: InscriptionPersistence,
    {
        loop {
            let inscription = match stream.next().await {
                Some(inscription) => inscription,
                None => {
                    tracing::error!(
                        "inscription detail stream closed.  shutting down client handling stream.",
                    );
                    return;
                }
            };

            {
                let read_lock = data_state.read_arc().await;
                if let Err(err) = read_lock
                    .persistence()
                    .record_pending_put_inscription(&inscription)
                    .await
                {
                    tracing::error!("error recording inscription: {:?}", err);
                }
            }
        }
    }
}

/// [ProcessPutInscriptionToChainTask] represents an async task for processing
/// the incoming [EspressoInscription] and submitting them to the chain.
/// This process is rate limited.
pub struct ProcessPutInscriptionToChainTask {
    pub task_handle: Option<JoinHandle<()>>,
}

const INSCRIPTION_BATCH_SUBMISSION_SIZE: usize = 10;

impl ProcessPutInscriptionToChainTask {
    pub fn new<S, Persistence>(
        rate_limiter: RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock>,
        data_state: Arc<RwLock<DataState<Persistence>>>,
        inscription_namespace_id: NamespaceId,
        signer: PrivateKeySigner,
        inscription_receiver: S,
        base_url: Url,
    ) -> Self
    where
        S: Stream<Item = EspressoInscription> + Send + Sync + Unpin + 'static,
        Persistence: InscriptionPersistence + Send + Sync + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_record_inscription(
            rate_limiter,
            data_state,
            inscription_namespace_id,
            signer,
            inscription_receiver,
            base_url,
        ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    fn created_signed_inscription(
        inscription: &EspressoInscription,
        signer: &PrivateKeySigner,
    ) -> InscriptionAndSignatureFromService {
        let signature_result =
            signer.sign_hash_sync(&inscription.eip712_signing_hash(&ESPRESSO_EIP712_DOMAIN));
        let signature = match signature_result {
            Ok(signature) => signature,
            Err(err) => {
                tracing::error!("error signing inscription: {}", err);
                panic!("Error signing inscription: {}", err);
            }
        };

        // We create the new inscription and signature
        InscriptionAndSignatureFromService {
            inscription: inscription.clone(),
            signature: HexSignature(signature),
        }
    }

    async fn process_record_inscription<S, Persistence>(
        rate_limiter: RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock>,
        // We're turning off this logging for now.
        _data_state: Arc<RwLock<DataState<Persistence>>>,
        inscription_namespace_id: NamespaceId,
        signer: PrivateKeySigner,
        mut stream: S,
        base_url: Url,
    ) where
        S: Stream<Item = EspressoInscription> + Unpin,
        Persistence: InscriptionPersistence,
    {
        let client = Client::<hotshot_query_service::Error, Version01>::new(base_url);
        let mut should_exit = false;

        loop {
            if should_exit {
                tracing::error!(
                    "inscription detail stream closed.  shutting down client handling stream.",
                );
                return;
            }

            // Wait for the Rate Limiter
            tracing::debug!("waiting for rate limiter");
            rate_limiter.until_ready().await;
            tracing::debug!("ready to submit next inscription");

            // We're going to try and submit this in batches of 10 transactions
            // at a time.

            let mut pending_inscriptions = Vec::new();

            // Let's wait for the first inscription to come in, then once it
            // does, we'll attempt to eagerly grab as many as we can up to the
            // INSCRIPTION_BATCH_SUBMISSION_SIZE

            let inscription = match stream.next().await {
                Some(inscription) => inscription,
                None => {
                    should_exit = true;
                    continue;
                }
            };

            pending_inscriptions.push(Self::created_signed_inscription(&inscription, &signer));

            for _ in 1..INSCRIPTION_BATCH_SUBMISSION_SIZE {
                let inscription = match stream.next().timeout(Duration::from_millis(20)).await {
                    Err(_) => {
                        // This is a timeout, we're going to break out of the
                        //loop
                        break;
                    }
                    Ok(None) => {
                        // This is the end of the stream, we're going to break
                        // out of the loop.
                        should_exit = true;
                        break;
                    }
                    Ok(Some(inscription)) => inscription,
                };

                tracing::debug!("have inscription to submit to mempool");
                pending_inscriptions.push(Self::created_signed_inscription(&inscription, &signer));
            }

            let pending_inscription_count = pending_inscriptions.len();
            futures::future::join_all(pending_inscriptions.iter().map(
                |inscription_and_signature| {
                    let client = client.clone();
                    // let inscription_and_signature = inscription_and_signature.clone();
                    async move {
                        let serialized = match bincode::serialize(inscription_and_signature) {
                            Ok(serialized) => serialized,
                            Err(err) => {
                                tracing::error!(
                                    "error serializing inscription for the server: {}",
                                    err
                                );
                                return;
                            }
                        };

                        let transaction = Transaction::new(inscription_namespace_id, serialized);

                        let send_result = client
                            .post::<()>("submit")
                            .body_binary(&transaction)
                            .unwrap()
                            .send()
                            .await;

                        if let Err(err) = send_result {
                            tracing::error!("error sending inscription to service: {}", err);
                        }
                    }
                },
            ))
            .await;

            tracing::debug!(
                "successfully submitted {} inscriptions to mempool",
                pending_inscription_count
            );
        }
    }
}

impl Drop for ProcessPutInscriptionToChainTask {
    fn drop(&mut self) {
        let task_handle = self.task_handle.take();
        if let Some(task_handle) = task_handle {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{ClientThreadState, InternalClientMessageProcessingTask};
    use crate::service::{
        client_id::ClientId,
        data_state::{DataState, Stats},
        espresso_inscription::{EspressoInscription, InscriptionAndChainDetails},
        server_message::ServerMessage,
        storage::{
            InscriptionPersistence, RecordConfirmedInscriptionAndChainDetailsError,
            RecordLastReceivedBlockError, RecordPendingPutInscriptionError,
            ResolvePendingPutInscriptionError, RetrieveLastReceivedBlockError,
            RetrieveLatestInscriptionAndChainDetailsError, RetrievePendingPutInscriptionsError,
        },
    };
    use alloy::signers::local::PrivateKeySigner;
    use async_std::sync::RwLock;
    use espresso_types::SeqTypes;
    use futures::channel::mpsc::{self, Sender};
    use hotshot_query_service::availability::BlockQueryData;
    use std::{num::NonZero, sync::Arc};

    pub fn create_test_client_thread_state() -> ClientThreadState<Sender<ServerMessage>> {
        ClientThreadState {
            clients: Default::default(),
            connection_id_counter: ClientId::from_count(1),
        }
    }

    #[derive(Default)]
    pub struct TestPersistence {}

    #[async_trait::async_trait]
    impl InscriptionPersistence for TestPersistence {
        async fn record_pending_put_inscription(
            &self,
            _inscription: &EspressoInscription,
        ) -> Result<(), RecordPendingPutInscriptionError> {
            todo!();
        }

        async fn record_submit_put_inscription(
            &self,
            _inscription: &EspressoInscription,
        ) -> Result<(), ResolvePendingPutInscriptionError> {
            todo!();
        }

        async fn retrieve_pending_put_inscriptions(
            &self,
        ) -> Result<Vec<EspressoInscription>, RetrievePendingPutInscriptionsError> {
            todo!();
        }

        async fn record_confirmed_inscription_and_chain_details(
            &self,
            _inscription_and_block_details: &InscriptionAndChainDetails,
        ) -> Result<(), RecordConfirmedInscriptionAndChainDetailsError> {
            todo!();
        }

        async fn retrieve_latest_inscription_and_chain_details(
            &self,
            _number_of_inscriptions: NonZero<usize>,
        ) -> Result<Vec<InscriptionAndChainDetails>, RetrieveLatestInscriptionAndChainDetailsError>
        {
            todo!();
        }

        async fn record_last_received_block(
            &self,
            _block: &BlockQueryData<SeqTypes>,
        ) -> Result<(), RecordLastReceivedBlockError> {
            todo!();
        }

        async fn retrieve_last_received_block(
            &self,
        ) -> Result<Stats, RetrieveLastReceivedBlockError> {
            todo!();
        }
    }

    pub fn create_test_data_state() -> DataState<TestPersistence> {
        let signer = PrivateKeySigner::random();
        let persistence = TestPersistence::default();
        DataState::new(Default::default(), Arc::new(persistence), signer.address())
    }

    #[async_std::test]
    async fn test_client_handling_stream_task_shutdown() {
        let data_state = create_test_data_state();
        let client_thread_state = Arc::new(RwLock::new(create_test_client_thread_state()));
        let data_state = Arc::new(RwLock::new(data_state));

        let (_internal_client_message_sender, internal_client_message_receiver) = mpsc::channel(1);
        let _process_internal_client_message_handle = InternalClientMessageProcessingTask::new(
            internal_client_message_receiver,
            data_state,
            client_thread_state,
        );
    }
}
