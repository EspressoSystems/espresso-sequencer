//! A network implementation that connects to a web server.
//!
//! To run the web server, see the `./web_server/` folder in this repo.
//!

use async_compatibility_layer::channel::{unbounded, UnboundedReceiver, UnboundedSender};

use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{oneshot, OneShotSender},
};
use async_lock::RwLock;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    message::{Message, MessagePurpose},
    traits::{
        network::{
            CommunicationChannel, ConnectedNetwork, ConsensusIntentEvent, NetworkError, NetworkMsg,
            TestableChannelImplementation, TestableNetworkingImplementation, TransmitType,
            WebServerNetworkError,
        },
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
};
use hotshot_web_server::{self, config};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use surf_disco::Url;

use hotshot_types::traits::network::ViewMessage;
use std::collections::BTreeMap;
use std::{
    collections::{btree_map::Entry, BTreeSet},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use surf_disco::error::ClientError;
use tracing::{debug, error, info, warn};

/// convenience alias alias for the result of getting transactions from the web server
pub type TxnResult<TYPES> = Result<Option<(u64, Vec<RecvMsg<Message<TYPES>>>)>, NetworkError>;

/// Represents the communication channel abstraction for the web server
#[derive(Clone, Debug)]
pub struct WebCommChannel<TYPES: NodeType>(Arc<WebServerNetwork<TYPES>>);

impl<TYPES: NodeType> WebCommChannel<TYPES> {
    /// Create new communication channel
    #[must_use]
    pub fn new(network: Arc<WebServerNetwork<TYPES>>) -> Self {
        Self(network)
    }
}

/// # Note
///
/// This function uses `DefaultHasher` instead of cryptographic hash functions like SHA-256 because of an `AsRef` requirement.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// The web server network state
#[derive(Clone, Debug)]
pub struct WebServerNetwork<TYPES: NodeType> {
    /// The inner, core state of the web server network
    inner: Arc<Inner<TYPES>>,
    /// An optional shutdown signal. This is only used when this connection is created through the `TestableNetworkingImplementation` API.
    server_shutdown_signal: Option<Arc<OneShotSender<()>>>,
}

impl<TYPES: NodeType> WebServerNetwork<TYPES> {
    /// Post a message to the web server and return the result
    async fn post_message_to_web_server(
        &self,
        message: SendMsg<Message<TYPES>>,
    ) -> Result<(), NetworkError> {
        let result: Result<(), ClientError> = self
            .inner
            .client
            .post(&message.get_endpoint())
            .body_binary(&message.get_message())
            .unwrap()
            .send()
            .await;
        // error!("POST message error for endpoint {} is {:?}", &message.get_endpoint(), result.clone());
        result.map_err(|_e| NetworkError::WebServer {
            source: WebServerNetworkError::ClientError,
        })
    }

    /// Pauses the underlying network
    pub fn pause(&self) {
        error!("Pausing CDN network");
        self.inner.running.store(false, Ordering::Relaxed);
    }

    /// Resumes the underlying network
    pub fn resume(&self) {
        error!("Resuming CDN network");
        self.inner.running.store(true, Ordering::Relaxed);
    }
}

/// `TaskChannel` is a type alias for an unbounded sender channel that sends `ConsensusIntentEvent`s.
///
/// This channel is used to send events to a task. The `K` type parameter is the type of the key used in the `ConsensusIntentEvent`.
///
/// # Examples
///
/// ```
/// let (tx, _rx): (TaskChannel<MyKey>, _) = tokio::sync::mpsc::unbounded_channel();
/// ```
///
/// # Note
///
/// This type alias is used in the context of a `TaskMap`, where each task is represented by a `TaskChannel`.
type TaskChannel<K> = UnboundedSender<ConsensusIntentEvent<K>>;

/// `TaskMap` is a wrapper around a `BTreeMap` that maps view numbers to tasks.
///
/// Each task is represented by a `TaskChannel` that can be used to send events to the task.
/// The key `K` is a type that implements the `SignatureKey` trait.
///
/// # Examples
///
/// ```
/// use your_crate::TaskMap;
/// let mut map: TaskMap<MyKey> = TaskMap::default();
/// ```
///
/// # Note
///
/// This struct is `Clone`, `Deref`, and `DerefMut`, so it can be used just like a `BTreeMap`.
#[derive(Debug, Clone, Deref, DerefMut)]
struct TaskMap<K: SignatureKey>(BTreeMap<u64, TaskChannel<K>>);

impl<K: SignatureKey> Default for TaskMap<K> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

impl<K: SignatureKey> TaskMap<K> {
    /// Prunes tasks that are polling for a view less than or equal to `current_view - 2`.
    ///
    /// This method cancels and removes all entries in the task map that are polling for a view less than or equal to `current_view - 2`.
    /// The cancellation is performed by sending a `cancel_event` to the task.
    ///
    /// # Arguments
    ///
    /// * `current_view` - The current view number. Tasks polling for a view less than or equal to `current_view - 2` will be pruned.
    /// * `cancel_event_fn` - A function that takes a view number and returns a `ConsensusIntentEvent` to be sent to the task for cancellation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut map: TaskMap<MyKey> = TaskMap::default();
    /// map.prune_tasks(10, ConsensusIntentEvent::CancelPollForProposal).await;
    /// ```
    async fn prune_tasks(
        &mut self,
        current_view: u64,
        cancel_event_fn: fn(u64) -> ConsensusIntentEvent<K>,
    ) {
        let cutoff_view = current_view.saturating_sub(2);
        let views_to_remove: Vec<_> = self.range(..cutoff_view).map(|(key, _)| *key).collect();

        for view in views_to_remove {
            let task = self.remove(&view);
            if let Some(task) = task {
                let _ = task.send(cancel_event_fn(view)).await;
            }
        }
    }
}

/// Represents the core of web server networking
#[derive(Debug)]
struct Inner<TYPES: NodeType> {
    /// Our own key
    _own_key: TYPES::SignatureKey,
    /// Queue for broadcasted messages
    broadcast_poll_queue: Arc<RwLock<Vec<RecvMsg<Message<TYPES>>>>>,
    /// Queue for direct messages
    direct_poll_queue: Arc<RwLock<Vec<RecvMsg<Message<TYPES>>>>>,
    /// Client is running
    running: AtomicBool,
    /// The web server connection is ready
    connected: AtomicBool,
    /// The connectioni to the web server
    client: surf_disco::Client<ClientError>,
    /// The duration to wait between poll attempts
    wait_between_polls: Duration,
    /// Whether we are connecting to a DA server
    is_da: bool,

    /// The last tx_index we saw from the web server
    tx_index: Arc<RwLock<u64>>,

    /// Task map for quorum proposals.
    proposal_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for quorum votes.
    vote_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for VID disperse data
    vid_disperse_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for DACs.
    dac_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for view sync certificates.
    view_sync_cert_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for view sync votes.
    view_sync_vote_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task map for transactions
    txn_task_map: Arc<RwLock<TaskMap<TYPES::SignatureKey>>>,
    /// Task polling for latest quorum propsal
    latest_quorum_proposal_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
    /// Task polling for latest view sync proposal
    latest_view_sync_proposal_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
}

impl<TYPES: NodeType> Inner<TYPES> {
    #![allow(clippy::too_many_lines)]
    /// Pull a web server.
    async fn poll_web_server(
        &self,
        receiver: UnboundedReceiver<ConsensusIntentEvent<TYPES::SignatureKey>>,
        message_purpose: MessagePurpose,
        view_number: u64,
    ) -> Result<(), NetworkError> {
        let mut vote_index = 0;
        let mut tx_index = 0;
        let mut seen_proposals = LruCache::new(NonZeroUsize::new(100).unwrap());

        if message_purpose == MessagePurpose::Data {
            tx_index = *self.tx_index.read().await;
            debug!("Previous tx index was {}", tx_index);
        };

        while self.running.load(Ordering::Relaxed) {
            let endpoint = match message_purpose {
                MessagePurpose::Proposal => config::get_proposal_route(view_number),
                MessagePurpose::LatestQuorumProposal => config::get_latest_quorum_proposal_route(),
                MessagePurpose::LatestViewSyncProposal => {
                    config::get_latest_view_sync_proposal_route()
                }
                MessagePurpose::Vote => config::get_vote_route(view_number, vote_index),
                MessagePurpose::Data => config::get_transactions_route(tx_index),
                MessagePurpose::Internal => unimplemented!(),
                MessagePurpose::ViewSyncProposal => {
                    config::get_view_sync_proposal_route(view_number, vote_index)
                }
                MessagePurpose::ViewSyncVote => {
                    config::get_view_sync_vote_route(view_number, vote_index)
                }
                MessagePurpose::DAC => config::get_da_certificate_route(view_number),
                MessagePurpose::VidDisperse => config::get_vid_disperse_route(view_number), // like `Proposal`
            };

            if message_purpose == MessagePurpose::Data {
                let possible_message = self.get_txs_from_web_server(endpoint).await;
                match possible_message {
                    Ok(Some((index, deserialized_messages))) => {
                        let mut broadcast_poll_queue = self.broadcast_poll_queue.write().await;
                        if index > tx_index + 1 {
                            debug!("missed txns from {} to {}", tx_index + 1, index - 1);
                            tx_index = index - 1;
                        }
                        for tx in &deserialized_messages {
                            tx_index += 1;
                            broadcast_poll_queue.push(tx.clone());
                        }
                        debug!("tx index is {}", tx_index);
                    }
                    Ok(None) => {
                        async_sleep(self.wait_between_polls).await;
                    }
                    Err(_e) => {
                        async_sleep(self.wait_between_polls).await;
                    }
                }
            } else {
                let possible_message = self.get_message_from_web_server(endpoint).await;

                match possible_message {
                    Ok(Some(deserialized_messages)) => {
                        match message_purpose {
                            MessagePurpose::Data => {
                                error!("We should not receive transactions in this function");
                            }
                            MessagePurpose::Proposal => {
                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                return Ok(());
                                // Wait for the view to change before polling for proposals again
                                // let event = receiver.recv().await;
                                // match event {
                                //     Ok(event) => view_number = event.view_number(),
                                //     Err(_r) => {
                                //         error!("Proposal receiver error!  It was likely shutdown")
                                //     }
                                // }
                            }
                            MessagePurpose::LatestQuorumProposal => {
                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                return Ok(());
                            }
                            MessagePurpose::LatestViewSyncProposal => {
                                let mut broadcast_poll_queue =
                                    self.broadcast_poll_queue.write().await;

                                for cert in &deserialized_messages {
                                    let hash = hash(&cert);
                                    if seen_proposals.put(hash, ()).is_none() {
                                        broadcast_poll_queue.push(cert.clone());
                                    }
                                }

                                // additional sleep to reduce load on web server
                                async_sleep(Duration::from_millis(300)).await;
                            }
                            MessagePurpose::Vote => {
                                // error!(
                                //     "Received {} votes from web server for view {} is da {}",
                                //     deserialized_messages.len(),
                                //     view_number,
                                //     self.is_da
                                // );
                                let mut direct_poll_queue = self.direct_poll_queue.write().await;
                                for vote in &deserialized_messages {
                                    vote_index += 1;
                                    direct_poll_queue.push(vote.clone());
                                }
                            }
                            MessagePurpose::DAC => {
                                debug!(
                                    "Received DAC from web server for view {} {}",
                                    view_number, self.is_da
                                );
                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                // return if we found a DAC, since there will only be 1 per view
                                // In future we should check to make sure DAC is valid
                                return Ok(());
                            }
                            MessagePurpose::VidDisperse => {
                                // TODO copy-pasted from `MessagePurpose::Proposal` https://github.com/EspressoSystems/HotShot/issues/1690

                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                return Ok(());
                                // Wait for the view to change before polling for proposals again
                                // let event = receiver.recv().await;
                                // match event {
                                //     Ok(event) => view_number = event.view_number(),
                                //     Err(_r) => {
                                //         error!("Proposal receiver error!  It was likely shutdown")
                                //     }
                                // }
                            }
                            MessagePurpose::ViewSyncVote => {
                                // error!(
                                //     "Received {} view sync votes from web server for view {} is da {}",
                                //     deserialized_messages.len(),
                                //     view_number,
                                //     self.is_da
                                // );
                                let mut direct_poll_queue = self.direct_poll_queue.write().await;
                                for vote in &deserialized_messages {
                                    vote_index += 1;
                                    direct_poll_queue.push(vote.clone());
                                }
                            }
                            MessagePurpose::ViewSyncProposal => {
                                // error!(
                                //     "Received {} view sync certs from web server for view {} is da {}",
                                //     deserialized_messages.len(),
                                //     view_number,
                                //     self.is_da
                                // );
                                let mut broadcast_poll_queue =
                                    self.broadcast_poll_queue.write().await;
                                // TODO ED Special case this for view sync
                                // TODO ED Need to add vote indexing to web server for view sync certs
                                for cert in &deserialized_messages {
                                    vote_index += 1;
                                    let hash = hash(cert);
                                    if seen_proposals.put(hash, ()).is_none() {
                                        broadcast_poll_queue.push(cert.clone());
                                    }
                                }
                            }

                            MessagePurpose::Internal => {
                                error!("Received internal message in web server network");
                            }
                        }
                    }
                    Ok(None) => {
                        async_sleep(self.wait_between_polls).await;
                    }
                    Err(_e) => {
                        // error!("error is {:?}", _e);
                        async_sleep(self.wait_between_polls).await;
                    }
                }
            }
            let maybe_event = receiver.try_recv();
            match maybe_event {
                Ok(event) => {
                    match event {
                        // TODO ED Should add extra error checking here to make sure we are intending to cancel a task
                        ConsensusIntentEvent::CancelPollForVotes(event_view)
                        | ConsensusIntentEvent::CancelPollForProposal(event_view)
                        | ConsensusIntentEvent::CancelPollForDAC(event_view)
                        | ConsensusIntentEvent::CancelPollForViewSyncCertificate(event_view)
                        | ConsensusIntentEvent::CancelPollForVIDDisperse(event_view)
                        | ConsensusIntentEvent::CancelPollForViewSyncVotes(event_view) => {
                            if view_number == event_view {
                                debug!("Shutting down polling task for view {}", event_view);
                                return Ok(());
                            }
                        }
                        ConsensusIntentEvent::CancelPollForTransactions(event_view) => {
                            // Write the most recent tx index so we can pick up where we left off later

                            let mut lock = self.tx_index.write().await;
                            *lock = tx_index;

                            if view_number == event_view {
                                debug!("Shutting down polling task for view {}", event_view);
                                return Ok(());
                            }
                        }

                        ConsensusIntentEvent::CancelPollForLatestViewSyncProposal => {
                            return Ok(());
                        }

                        _ => {
                            unimplemented!()
                        }
                    }
                }
                // Nothing on receiving channel
                Err(_) => {
                    debug!("Nothing on receiving channel");
                }
            }
        }
        Err(NetworkError::ShutDown)
    }

    /// Fetches transactions from web server
    async fn get_txs_from_web_server(
        &self,
        endpoint: String,
    ) -> TxnResult<TYPES> {
        let result : Result<Option<(_, Vec<Vec<u8>>)>, _>  = self.client.get(&endpoint).send().await;
        match result {
            Err(_error) => Err(NetworkError::WebServer {
                source: WebServerNetworkError::ClientError,
            }),
            Ok(Some((index, messages))) => {
                let mut deserialized_messages = Vec::new();
                for message in &messages {
                    let deserialized_message = bincode::deserialize(message);
                    if let Err(e) = deserialized_message {
                        return Err(NetworkError::FailedToDeserialize { source: e });
                    }
                    deserialized_messages.push(deserialized_message.unwrap());
                }
                Ok(Some((index, deserialized_messages)))
            }
            Ok(None) => Ok(None),
        }
    }

    /// Sends a GET request to the webserver for some specified endpoint
    /// Returns a vec of deserialized, received messages or an error
    async fn get_message_from_web_server(
        &self,
        endpoint: String,
    ) -> Result<Option<Vec<RecvMsg<Message<TYPES>>>>, NetworkError> {
        let result: Result<Option<Vec<Vec<u8>>>, ClientError> =
            self.client.get(&endpoint).send().await;
        match result {
            Err(_error) => Err(NetworkError::WebServer {
                source: WebServerNetworkError::ClientError,
            }),
            Ok(Some(messages)) => {
                let mut deserialized_messages = Vec::new();
                for message in &messages {
                    let deserialized_message = bincode::deserialize(message);
                    if let Err(e) = deserialized_message {
                        return Err(NetworkError::FailedToDeserialize { source: e });
                    }
                    deserialized_messages.push(deserialized_message.unwrap());
                }
                Ok(Some(deserialized_messages))
            }
            Ok(None) => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(bound(deserialize = ""))]
/// A message being sent to the web server
pub struct SendMsg<M: NetworkMsg> {
    /// The optional message, or body, to send
    message: Option<M>,
    /// The endpoint to send the message to
    endpoint: String,
}

/// A message being received from the web server
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub struct RecvMsg<M: NetworkMsg> {
    /// The optional message being received
    message: Option<M>,
}

/// Trait for messages being sent to the web server
pub trait SendMsgTrait<M: NetworkMsg> {
    /// Returns the endpoint to send the message to
    fn get_endpoint(&self) -> String;
    /// Returns the actual message being sent
    fn get_message(&self) -> Option<M>;
}

/// Trait for messages being received from the web server
pub trait RecvMsgTrait<M: NetworkMsg> {
    /// Returns the actual message being received
    fn get_message(&self) -> Option<M>;
}

impl<M: NetworkMsg> SendMsgTrait<M> for SendMsg<M> {
    fn get_endpoint(&self) -> String {
        self.endpoint.clone()
    }

    fn get_message(&self) -> Option<M> {
        self.message.clone()
    }
}

impl<M: NetworkMsg> RecvMsgTrait<M> for RecvMsg<M> {
    fn get_message(&self) -> Option<M> {
        self.message.clone()
    }
}

impl<M: NetworkMsg> NetworkMsg for SendMsg<M> {}
impl<M: NetworkMsg> NetworkMsg for RecvMsg<M> {}

impl<TYPES: NodeType + 'static> WebServerNetwork<TYPES> {
    /// Creates a new instance of the `WebServerNetwork`
    /// # Panics
    /// if the web server url is malformed
    pub fn create(
        url: Url,
        wait_between_polls: Duration,
        key: TYPES::SignatureKey,
        is_da_server: bool,
    ) -> Self {
        info!("Connecting to web server at {url:?} is da: {is_da_server}");

        // TODO ED Wait for healthcheck
        let client = surf_disco::Client::<ClientError>::new(url);

        let inner = Arc::new(Inner {
            broadcast_poll_queue: Arc::default(),
            direct_poll_queue: Arc::default(),
            running: AtomicBool::new(true),
            connected: AtomicBool::new(false),
            client,
            wait_between_polls,
            _own_key: key,
            is_da: is_da_server,
            tx_index: Arc::default(),
            proposal_task_map: Arc::default(),
            vote_task_map: Arc::default(),
            vid_disperse_task_map: Arc::default(),
            dac_task_map: Arc::default(),
            view_sync_cert_task_map: Arc::default(),
            view_sync_vote_task_map: Arc::default(),
            txn_task_map: Arc::default(),
            latest_quorum_proposal_task: Arc::default(),
            latest_view_sync_proposal_task: Arc::default(),
        });

        inner.connected.store(true, Ordering::Relaxed);

        Self {
            inner,
            server_shutdown_signal: None,
        }
    }

    /// Parses a message to find the appropriate endpoint
    /// Returns a `SendMsg` containing the endpoint
    fn parse_post_message(
        message: Message<TYPES>,
    ) -> Result<SendMsg<Message<TYPES>>, WebServerNetworkError> {
        let view_number: TYPES::Time = message.get_view_number();

        let endpoint = match &message.purpose() {
            MessagePurpose::Proposal => config::post_proposal_route(*view_number),
            MessagePurpose::Vote => config::post_vote_route(*view_number),
            MessagePurpose::Data => config::post_transactions_route(),
            MessagePurpose::Internal
            | MessagePurpose::LatestQuorumProposal
            | MessagePurpose::LatestViewSyncProposal => {
                return Err(WebServerNetworkError::EndpointError)
            }
            MessagePurpose::ViewSyncProposal => {
                // error!("Posting view sync proposal route is: {}", config::post_view_sync_proposal_route(*view_number));
                config::post_view_sync_proposal_route(*view_number)
            }
            MessagePurpose::ViewSyncVote => config::post_view_sync_vote_route(*view_number),
            MessagePurpose::DAC => config::post_da_certificate_route(*view_number),
            MessagePurpose::VidDisperse => config::post_vid_disperse_route(*view_number),
        };

        let network_msg: SendMsg<Message<TYPES>> = SendMsg {
            message: Some(message),
            endpoint,
        };
        Ok(network_msg)
    }
}

#[async_trait]
impl<TYPES: NodeType> CommunicationChannel<TYPES> for WebCommChannel<TYPES> {
    type NETWORK = WebServerNetwork<TYPES>;
    /// Blocks until node is successfully initialized
    /// into the network
    async fn wait_for_ready(&self) {
        <WebServerNetwork<_> as ConnectedNetwork<
            Message<TYPES>,
            TYPES::SignatureKey,
        >>::wait_for_ready(&self.0)
        .await;
    }

    fn pause(&self) {
        self.0.pause();
    }

    fn resume(&self) {
        self.0.resume();
    }

    /// checks if the network is ready
    /// nonblocking
    async fn is_ready(&self) -> bool {
        <WebServerNetwork<_> as ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>>::is_ready(
            &self.0,
        )
        .await
    }

    /// Shut down this network. Afterwards this network should no longer be used.
    ///
    /// This should also cause other functions to immediately return with a [`NetworkError`]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            <WebServerNetwork<_> as ConnectedNetwork<
                Message<TYPES>,
                TYPES::SignatureKey,
            >>::shut_down(&self.0)
            .await;
        };
        boxed_sync(closure)
    }

    /// broadcast message to those listening on the communication channel
    /// blocking
    async fn broadcast_message(
        &self,
        message: Message<TYPES>,
        _election: &TYPES::Membership,
    ) -> Result<(), NetworkError> {
        self.0.broadcast_message(message, BTreeSet::new()).await
    }

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message(
        &self,
        message: Message<TYPES>,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        self.0.direct_message(message, recipient).await
    }

    /// Moves out the entire queue of received messages of 'transmit_type`
    ///
    /// Will unwrap the underlying `NetworkMessage`
    /// blocking
    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            <WebServerNetwork<_> as ConnectedNetwork<
                Message<TYPES>,
                TYPES::SignatureKey,
            >>::recv_msgs(&self.0, transmit_type)
            .await
        };
        boxed_sync(closure)
    }

    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<TYPES::SignatureKey>) {
        <WebServerNetwork<_> as ConnectedNetwork<
            Message<TYPES>,
            TYPES::SignatureKey,
        >>::inject_consensus_info(&self.0, event)
        .await;
    }
}

#[async_trait]
impl<TYPES: NodeType + 'static> ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>
    for WebServerNetwork<TYPES>
{
    /// Blocks until the network is successfully initialized
    async fn wait_for_ready(&self) {
        while !self.inner.connected.load(Ordering::Relaxed) {
            async_sleep(Duration::from_secs(1)).await;
        }
    }

    /// checks if the network is ready
    /// nonblocking
    async fn is_ready(&self) -> bool {
        self.inner.connected.load(Ordering::Relaxed)
    }

    /// Blocks until the network is shut down
    /// then returns true
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            self.inner.running.store(false, Ordering::Relaxed);
        };
        boxed_sync(closure)
    }

    /// broadcast message to some subset of nodes
    /// blocking
    async fn broadcast_message(
        &self,
        message: Message<TYPES>,
        _recipients: BTreeSet<TYPES::SignatureKey>,
    ) -> Result<(), NetworkError> {
        // short circuit if we are shut down
        #[cfg(feature = "hotshot-testing")]
        if !self.inner.running.load(Ordering::Relaxed) {
            return Err(NetworkError::ShutDown);
        }

        let network_msg = Self::parse_post_message(message);
        match network_msg {
            Ok(network_msg) => self.post_message_to_web_server(network_msg).await,
            Err(network_msg) => Err(NetworkError::WebServer {
                source: network_msg,
            }),
        }
    }

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message(
        &self,
        message: Message<TYPES>,
        _recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        // short circuit if we are shut down
        #[cfg(feature = "hotshot-testing")]
        if !self.inner.running.load(Ordering::Relaxed) {
            return Err(NetworkError::ShutDown);
        }
        let network_msg = Self::parse_post_message(message);
        match network_msg {
            Ok(network_msg) => {
                // error!("network msg is {:?}", network_msg.clone());

                self.post_message_to_web_server(network_msg).await
            }
            Err(network_msg) => Err(NetworkError::WebServer {
                source: network_msg,
            }),
        }
    }

    /// Moves out the entire queue of received messages of 'transmit_type`
    ///
    /// Will unwrap the underlying `NetworkMessage`
    /// blocking
    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            match transmit_type {
                TransmitType::Direct => {
                    let mut queue = self.inner.direct_poll_queue.write().await;
                    Ok(queue
                        .drain(..)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|x| x.get_message().unwrap())
                        .collect())
                }
                TransmitType::Broadcast => {
                    let mut queue = self.inner.broadcast_poll_queue.write().await;
                    Ok(queue
                        .drain(..)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|x| x.get_message().unwrap())
                        .collect())
                }
            }
        };
        boxed_sync(closure)
    }

    #[allow(clippy::too_many_lines)]
    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<TYPES::SignatureKey>) {
        #[cfg(feature = "hotshot-testing")]
        if !self.inner.running.load(Ordering::Relaxed) {
            return;
        }

        debug!(
            "Injecting event: {:?} is da {}",
            event.clone(),
            self.inner.is_da
        );

        // TODO ED Need to handle canceling tasks that don't receive their expected output (such a proposal that never comes)
        match event {
            ConsensusIntentEvent::PollForProposal(view_number) => {
                // Check if we already have a task for this (we shouldn't)

                // Going to do a write lock since mostly likely we will need it - can change to upgradable read in the future
                let mut task_map = self.inner.proposal_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);

                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::Proposal, view_number)
                                .await
                            {
                                warn!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(view_number, ConsensusIntentEvent::CancelPollForProposal)
                    .await;
            }
            ConsensusIntentEvent::PollForVIDDisperse(view_number) => {
                // Check if we already have a task for this (we shouldn't)

                // Going to do a write lock since mostly likely we will need it - can change to upgradable read in the future
                let mut task_map = self.inner.vid_disperse_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);

                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::VidDisperse, view_number)
                                .await
                            {
                                warn!(
                                    "Background receive VID disperse polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(view_number, ConsensusIntentEvent::CancelPollForVIDDisperse)
                    .await;
            }
            ConsensusIntentEvent::PollForLatestQuorumProposal => {
                let mut proposal_task = self.inner.latest_quorum_proposal_task.write().await;
                if proposal_task.is_none() {
                    // create new task
                    let (sender, receiver) = unbounded();
                    *proposal_task = Some(sender);

                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::LatestQuorumProposal, 1)
                                .await
                            {
                                warn!(
                                "Background receive proposal polling encountered an error: {:?}",
                                e
                            );
                            }
                            let mut proposal_task =
                                inner_clone.latest_quorum_proposal_task.write().await;
                            *proposal_task = None;
                        }
                    });
                }
            }
            ConsensusIntentEvent::PollForLatestViewSyncProposal => {
                let mut latest_view_sync_proposal_task =
                    self.inner.latest_view_sync_proposal_task.write().await;
                if latest_view_sync_proposal_task.is_none() {
                    // create new task
                    let (sender, receiver) = unbounded();
                    *latest_view_sync_proposal_task = Some(sender);

                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::LatestViewSyncProposal,
                                    1,
                                )
                                .await
                            {
                                warn!(
                                "Background receive proposal polling encountered an error: {:?}",
                                e
                            );
                            }
                            let mut latest_view_sync_proposal_task =
                                inner_clone.latest_view_sync_proposal_task.write().await;
                            *latest_view_sync_proposal_task = None;
                        }
                    });
                }
            }
            ConsensusIntentEvent::PollForVotes(view_number) => {
                let mut task_map = self.inner.vote_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);
                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::Vote, view_number)
                                .await
                            {
                                warn!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(view_number, ConsensusIntentEvent::CancelPollForVotes)
                    .await;
            }

            ConsensusIntentEvent::PollForDAC(view_number) => {
                let mut task_map = self.inner.dac_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);
                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::DAC, view_number)
                                .await
                            {
                                warn!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(view_number, ConsensusIntentEvent::CancelPollForDAC)
                    .await;
            }

            ConsensusIntentEvent::CancelPollForVotes(view_number) => {
                let mut task_map = self.inner.vote_task_map.write().await;

                if let Some((_, sender)) = task_map.remove_entry(&(view_number)) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForVotes(view_number))
                        .await;
                }
            }

            ConsensusIntentEvent::CancelPollForLatestViewSyncProposal => {
                let mut latest_view_sync_proposal_task =
                    self.inner.latest_view_sync_proposal_task.write().await;

                if let Some(thing) = latest_view_sync_proposal_task.take() {
                    let _res = thing
                        .send(ConsensusIntentEvent::CancelPollForLatestViewSyncProposal)
                        .await;
                }
            }

            ConsensusIntentEvent::PollForViewSyncCertificate(view_number) => {
                let mut task_map = self.inner.view_sync_cert_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);
                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::ViewSyncProposal,
                                    view_number,
                                )
                                .await
                            {
                                warn!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(
                        view_number,
                        ConsensusIntentEvent::CancelPollForViewSyncCertificate,
                    )
                    .await;
            }
            ConsensusIntentEvent::PollForViewSyncVotes(view_number) => {
                let mut task_map = self.inner.view_sync_vote_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);
                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::ViewSyncVote,
                                    view_number,
                                )
                                .await
                            {
                                warn!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(
                        view_number,
                        ConsensusIntentEvent::CancelPollForViewSyncVotes,
                    )
                    .await;
            }

            ConsensusIntentEvent::CancelPollForViewSyncCertificate(view_number) => {
                let mut task_map = self.inner.view_sync_cert_task_map.write().await;

                if let Some((_, sender)) = task_map.remove_entry(&(view_number)) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForViewSyncCertificate(
                            view_number,
                        ))
                        .await;
                }
            }
            ConsensusIntentEvent::CancelPollForViewSyncVotes(view_number) => {
                let mut task_map = self.inner.view_sync_vote_task_map.write().await;

                if let Some((_, sender)) = task_map.remove_entry(&(view_number)) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForViewSyncVotes(
                            view_number,
                        ))
                        .await;
                }
            }
            ConsensusIntentEvent::PollForTransactions(view_number) => {
                let mut task_map = self.inner.txn_task_map.write().await;
                if let Entry::Vacant(e) = task_map.entry(view_number) {
                    // create new task
                    let (sender, receiver) = unbounded();
                    e.insert(sender);
                    async_spawn({
                        let inner_clone = self.inner.clone();
                        async move {
                            if let Err(e) = inner_clone
                                .poll_web_server(receiver, MessagePurpose::Data, view_number)
                                .await
                            {
                                warn!(
                                                               "Background receive transaction polling encountered an error: {:?}",
                                                                 e
                                                               );
                            }
                        }
                    });
                } else {
                    debug!("Somehow task already existed!");
                }

                // Cancel old, stale tasks
                task_map
                    .prune_tasks(view_number, ConsensusIntentEvent::CancelPollForTransactions)
                    .await;
            }
            ConsensusIntentEvent::CancelPollForTransactions(view_number) => {
                let mut task_map = self.inner.txn_task_map.write().await;

                if let Some((_view, sender)) = task_map.remove_entry(&(view_number)) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForTransactions(view_number))
                        .await;
                } else {
                    info!("Task map entry should have existed");
                };
            }

            _ => {}
        }
    }
}

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES> for WebServerNetwork<TYPES> {
    fn generator(
        expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let (server_shutdown_sender, server_shutdown) = oneshot();
        let sender = Arc::new(server_shutdown_sender);

        // pick random, unused port
        let port = portpicker::pick_unused_port().expect("Could not find an open port");

        let url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();
        info!("Launching web server on port {port}");
        // Start web server
        async_spawn(async {
            match hotshot_web_server::run_web_server::<TYPES::SignatureKey>(
                Some(server_shutdown),
                url,
            )
            .await
            {
                Ok(()) => error!("Web server future finished unexpectedly"),
                Err(e) => error!("Web server task failed: {e}"),
            }
        });

        // We assign known_nodes' public key and stake value rather than read from config file since it's a test
        let known_nodes = (0..expected_node_count as u64)
            .map(|id| {
                TYPES::SignatureKey::from_private(
                    &TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], id).1,
                )
            })
            .collect::<Vec<_>>();

        // Start each node's web server client
        Box::new(move |id| {
            let sender = Arc::clone(&sender);
            let url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();
            let mut network = WebServerNetwork::create(
                url,
                Duration::from_millis(100),
                known_nodes[usize::try_from(id).unwrap()].clone(),
                is_da,
            );
            network.server_shutdown_signal = Some(sender);
            network
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES> for WebCommChannel<TYPES> {
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <WebServerNetwork<TYPES> as TestableNetworkingImplementation<_>>::generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            is_da,
        );
        Box::new(move |node_id| Self(generator(node_id).into()))
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<TYPES: NodeType> TestableChannelImplementation<TYPES> for WebCommChannel<TYPES> {
    fn generate_network() -> Box<dyn Fn(Arc<WebServerNetwork<TYPES>>) -> Self + 'static> {
        Box::new(move |network| WebCommChannel::new(network))
    }
}
