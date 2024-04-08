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
use hotshot_types::traits::network::AsyncGenerator;
use hotshot_types::{
    boxed_sync,
    constants::{Version01, VERSION_0_1},
    message::{Message, MessagePurpose},
    traits::{
        network::{
            ConnectedNetwork, ConsensusIntentEvent, NetworkError, NetworkMsg, NetworkReliability,
            TestableNetworkingImplementation, ViewMessage, WebServerNetworkError,
        },
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
    BoxSyncFuture,
};
use hotshot_web_server::{self, config};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::{
    collections::{btree_map::Entry, BTreeSet},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use surf_disco::error::ClientError;
use surf_disco::Url;
use tracing::{debug, error, info, warn};
use vbs::{
    version::{StaticVersionType, Version},
    BinarySerializer, Serializer,
};

/// convenience alias alias for the result of getting transactions from the web server
pub type TxnResult = Result<Option<(u64, Vec<Vec<u8>>)>, ClientError>;

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
pub struct WebServerNetwork<TYPES: NodeType, NetworkVersion: StaticVersionType> {
    /// The inner, core state of the web server network
    inner: Arc<Inner<TYPES, NetworkVersion>>,
    /// An optional shutdown signal. This is only used when this connection is created through the `TestableNetworkingImplementation` API.
    server_shutdown_signal: Option<Arc<OneShotSender<()>>>,
}

impl<TYPES: NodeType, NetworkVersion: StaticVersionType> WebServerNetwork<TYPES, NetworkVersion> {
    /// Post a message to the web server and return the result
    async fn post_message_to_web_server(
        &self,
        message: SendMsg<Message<TYPES>>,
    ) -> Result<(), NetworkError> {
        // Note: it should be possible to get the version of Message and choose client_initial or (if available)
        // client_new_ver based on Message. But we do always know
        let result: Result<(), ClientError> = self
            .inner
            .client
            .post(&message.get_endpoint())
            .body_binary(&message.get_message())
            .unwrap()
            .send()
            .await;
        // error!("POST message error for endpoint {} is {:?}", &message.get_endpoint(), result.clone());
        result.map_err(|_e| {
            error!("{}", &message.get_endpoint());
            NetworkError::WebServer {
                source: WebServerNetworkError::ClientError,
            }
        })
    }
}

/// `TaskChannel` is a type alias for an unbounded sender channel that sends `ConsensusIntentEvent`s.
///
/// This channel is used to send events to a task. The `K` type parameter is the type of the key used in the `ConsensusIntentEvent`.
///
/// # Examples
///
/// ```ignore
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
/// ```ignore
/// # use crate::TaskMap;
/// let mut map: TaskMap<u64> = TaskMap::default();
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
    /// ```ignore
    /// # use crate::TaskMap;
    /// let mut map: TaskMap<u64> = TaskMap::default();
    /// map.prune_tasks(10, ConsensusIntentEvent::CancelPollForProposal(5)).await;
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
struct Inner<TYPES: NodeType, NetworkVersion: StaticVersionType> {
    /// Our own key
    _own_key: TYPES::SignatureKey,
    /// Queue for messages
    poll_queue_0_1: Arc<RwLock<Vec<RecvMsg<Message<TYPES>>>>>,
    /// Client is running
    running: AtomicBool,
    /// The web server connection is ready
    connected: AtomicBool,
    /// The connection to the web server
    client: surf_disco::Client<ClientError, NetworkVersion>,
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
    #[allow(clippy::type_complexity)]
    /// A handle on the task polling for latest quorum propsal
    latest_proposal_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
    /// A handle on the task polling for an upgrade propsal
    upgrade_proposal_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
    /// A handle on the task polling for an upgrade vote
    upgrade_vote_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
    #[allow(clippy::type_complexity)]
    /// A handle on the task polling for the latest view sync certificate
    latest_view_sync_certificate_task: Arc<RwLock<Option<TaskChannel<TYPES::SignatureKey>>>>,
}

impl<TYPES: NodeType, NetworkVersion: StaticVersionType> Inner<TYPES, NetworkVersion> {
    #![allow(clippy::too_many_lines)]

    /// Handle version 0.1 transactions
    ///
    /// * `first_tx_index` - the index of the first transaction received from the server in the latest batch.
    /// * `tx_index` - the last transaction index we saw from the web server.
    async fn handle_tx_0_1(&self, tx: Vec<u8>, first_tx_index: u64, tx_index: &mut u64) {
        let poll_queue = &self.poll_queue_0_1;
        if first_tx_index > *tx_index + 1 {
            debug!(
                "missed txns from {} to {}",
                *tx_index + 1,
                first_tx_index - 1
            );
            *tx_index = first_tx_index - 1;
        }

        *tx_index += 1;

        if let Ok(Some(deserialized_message_inner)) =
            Serializer::<Version01>::deserialize::<Option<Message<TYPES>>>(&tx)
        {
            let deserialized_message = RecvMsg {
                message: Some(deserialized_message_inner),
            };
            poll_queue.write().await.push(deserialized_message.clone());
        } else {
            async_sleep(self.wait_between_polls).await;
        }

        debug!("tx index is {}", tx_index);
    }

    /// Handle version 0.1 messages
    ///
    /// Returns `should_return` as a boolean, which is:
    ///   * `true` if we've seen enough this round and the `poll_web_server` function should return
    ///   * `false` if we want to receive further messages from the server.
    #[allow(clippy::too_many_arguments)]
    async fn handle_message_0_1(
        &self,
        message: Vec<u8>,
        view_number: u64,
        message_purpose: MessagePurpose,
        vote_index: &mut u64,
        upgrade_vote_index: &mut u64,
        seen_proposals: &mut LruCache<u64, ()>,
        seen_view_sync_certificates: &mut LruCache<u64, ()>,
    ) -> bool {
        let poll_queue = &self.poll_queue_0_1;
        match Serializer::<Version01>::deserialize::<Option<Message<TYPES>>>(&message) {
            Ok(Some(deserialized_message_inner)) => {
                let deserialized_message = RecvMsg {
                    message: Some(deserialized_message_inner),
                };
                match message_purpose {
                    MessagePurpose::Data => {
                        error!("We should not receive transactions in this function");
                    }
                    MessagePurpose::Proposal => {
                        let proposal = deserialized_message.clone();
                        poll_queue.write().await.push(proposal);

                        // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                        return true;
                    }
                    MessagePurpose::LatestProposal => {
                        let proposal = deserialized_message.clone();
                        let hash = hash(&proposal);
                        // Only allow unseen proposals to be pushed to the queue
                        if seen_proposals.put(hash, ()).is_none() {
                            poll_queue.write().await.push(proposal);
                        }

                        // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                        return true;
                    }
                    MessagePurpose::LatestViewSyncCertificate => {
                        let cert = deserialized_message.clone();
                        let hash = hash(&cert);
                        if seen_view_sync_certificates.put(hash, ()).is_none() {
                            poll_queue.write().await.push(cert);
                        }
                        return false;
                    }
                    MessagePurpose::Vote
                    | MessagePurpose::ViewSyncVote
                    | MessagePurpose::ViewSyncCertificate => {
                        let vote = deserialized_message.clone();
                        *vote_index += 1;
                        poll_queue.write().await.push(vote);

                        return false;
                    }
                    MessagePurpose::UpgradeVote => {
                        let vote = deserialized_message.clone();
                        *upgrade_vote_index += 1;
                        poll_queue.write().await.push(vote);

                        return false;
                    }
                    MessagePurpose::DAC => {
                        debug!(
                            "Received DAC from web server for view {} {}",
                            view_number, self.is_da
                        );
                        poll_queue.write().await.push(deserialized_message.clone());

                        // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                        // return if we found a DAC, since there will only be 1 per view
                        // In future we should check to make sure DAC is valid
                        return true;
                    }
                    MessagePurpose::VidDisperse => {
                        // TODO copy-pasted from `MessagePurpose::Proposal` https://github.com/EspressoSystems/HotShot/issues/1690

                        self.poll_queue_0_1
                            .write()
                            .await
                            .push(deserialized_message.clone());

                        // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                        return true;
                    }

                    MessagePurpose::Internal => {
                        error!("Received internal message in web server network");

                        return false;
                    }

                    MessagePurpose::UpgradeProposal => {
                        poll_queue.write().await.push(deserialized_message.clone());

                        return true;
                    }
                }
            }
            Ok(None) | Err(_) => {}
        }
        false
    }

    /// Poll a web server.
    async fn poll_web_server(
        &self,
        receiver: UnboundedReceiver<ConsensusIntentEvent<TYPES::SignatureKey>>,
        message_purpose: MessagePurpose,
        view_number: u64,
        additional_wait: Duration,
    ) -> Result<(), NetworkError> {
        let mut vote_index = 0;
        let mut tx_index = 0;
        let mut upgrade_vote_index = 0;
        let mut seen_proposals = LruCache::new(NonZeroUsize::new(100).unwrap());
        let mut seen_view_sync_certificates = LruCache::new(NonZeroUsize::new(100).unwrap());

        if message_purpose == MessagePurpose::Data {
            tx_index = *self.tx_index.read().await;
            debug!("Previous tx index was {}", tx_index);
        };

        while self.running.load(Ordering::Relaxed) {
            async_sleep(additional_wait).await;

            let endpoint = match message_purpose {
                MessagePurpose::Proposal => config::get_proposal_route(view_number),
                MessagePurpose::LatestProposal => config::get_latest_proposal_route(),
                MessagePurpose::LatestViewSyncCertificate => {
                    config::get_latest_view_sync_certificate_route()
                }
                MessagePurpose::Vote => config::get_vote_route(view_number, vote_index),
                MessagePurpose::Data => config::get_transactions_route(tx_index),
                MessagePurpose::Internal => unimplemented!(),
                MessagePurpose::ViewSyncCertificate => {
                    config::get_view_sync_certificate_route(view_number, vote_index)
                }
                MessagePurpose::ViewSyncVote => {
                    config::get_view_sync_vote_route(view_number, vote_index)
                }
                MessagePurpose::DAC => config::get_da_certificate_route(view_number),
                MessagePurpose::VidDisperse => config::get_vid_disperse_route(view_number), // like `Proposal`
                MessagePurpose::UpgradeProposal => config::get_upgrade_proposal_route(0),
                MessagePurpose::UpgradeVote => {
                    config::get_upgrade_vote_route(0, upgrade_vote_index)
                }
            };

            if let MessagePurpose::Data = message_purpose {
                // Note: this should also be polling on client_
                let possible_message: TxnResult = self.client.get(&endpoint).send().await;
                // Deserialize and process transactions from the server.
                // If something goes wrong at any point, we sleep for wait_between_polls
                // then try again next time.
                if let Ok(Some((first_tx_index, txs))) = possible_message {
                    for tx_raw in txs {
                        let tx_version = Version::deserialize(&tx_raw);

                        match tx_version {
                            Ok((VERSION_0_1, _)) => {
                                self.handle_tx_0_1(tx_raw, first_tx_index, &mut tx_index)
                                    .await;
                            }
                            Ok((version, _)) => {
                                warn!(
                                    "Received message with unsupported version: {:?}.\n\nPayload:\n\n{:?}",
                                    version,
                                    tx_raw
                                );
                            }
                            Err(e) => {
                                warn!(
                                    "Error {:?}, could not read version number.\n\nPayload:\n\n{:?}",
                                    e, tx_raw
                                );
                            }
                        }
                    }
                } else {
                    async_sleep(self.wait_between_polls + additional_wait).await;
                }
            } else {
                let possible_message: Result<Option<Vec<Vec<u8>>>, ClientError> =
                    self.client.get(&endpoint).send().await;

                if let Ok(Some(messages)) = possible_message {
                    for message in messages {
                        let message_version = Version::deserialize(&message);

                        let should_return;

                        match message_version {
                            Ok((VERSION_0_1, _)) => {
                                should_return = self
                                    .handle_message_0_1(
                                        message,
                                        view_number,
                                        message_purpose,
                                        &mut vote_index,
                                        &mut upgrade_vote_index,
                                        &mut seen_proposals,
                                        &mut seen_view_sync_certificates,
                                    )
                                    .await;

                                if should_return {
                                    return Ok(());
                                }
                            }
                            Ok((version, _)) => {
                                warn!(
                                "Received message with unsupported version: {:?}.\n\nPayload:\n\n{:?}", version, message);
                            }
                            Err(e) => {
                                warn!("Error {:?}, could not read version number.\n\nPayload:\n\n{:?}", e, message);
                            }
                        }
                    }
                } else {
                    async_sleep(self.wait_between_polls).await;
                }
            }

            if let Ok(event) = receiver.try_recv() {
                match event {
                    // TODO ED Should add extra error checking here to make sure we are intending to cancel a task
                    ConsensusIntentEvent::CancelPollForVotes(event_view)
                    | ConsensusIntentEvent::CancelPollForProposal(event_view)
                    | ConsensusIntentEvent::CancelPollForDAC(event_view)
                    | ConsensusIntentEvent::CancelPollForViewSyncCertificate(event_view)
                    | ConsensusIntentEvent::CancelPollForVIDDisperse(event_view)
                    | ConsensusIntentEvent::CancelPollForLatestProposal(event_view)
                    | ConsensusIntentEvent::CancelPollForLatestViewSyncCertificate(event_view)
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

                    _ => {
                        unimplemented!()
                    }
                }
            }
        }
        Err(NetworkError::ShutDown)
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

impl<TYPES: NodeType + 'static, NetworkVersion: StaticVersionType + 'static>
    WebServerNetwork<TYPES, NetworkVersion>
{
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
        let client = surf_disco::Client::<ClientError, NetworkVersion>::new(url);

        let inner = Arc::new(Inner {
            poll_queue_0_1: Arc::default(),
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
            latest_proposal_task: Arc::default(),
            upgrade_proposal_task: Arc::default(),
            upgrade_vote_task: Arc::default(),
            latest_view_sync_certificate_task: Arc::default(),
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
            | MessagePurpose::LatestProposal
            | MessagePurpose::LatestViewSyncCertificate => {
                return Err(WebServerNetworkError::EndpointError)
            }
            MessagePurpose::ViewSyncCertificate => {
                // error!("Posting view sync proposal route is: {}", config::post_view_sync_certificate_route(*view_number));
                config::post_view_sync_certificate_route(*view_number)
            }
            MessagePurpose::ViewSyncVote => config::post_view_sync_vote_route(*view_number),
            MessagePurpose::DAC => config::post_da_certificate_route(*view_number),
            MessagePurpose::VidDisperse => config::post_vid_disperse_route(*view_number),
            MessagePurpose::UpgradeProposal => config::post_upgrade_proposal_route(0),
            MessagePurpose::UpgradeVote => config::post_upgrade_vote_route(0),
        };

        let network_msg: SendMsg<Message<TYPES>> = SendMsg {
            message: Some(message),
            endpoint,
        };
        Ok(network_msg)
    }

    /// Generates a single webserver network, for use in tests
    fn single_generator(
        expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        is_da: bool,
        _reliability_config: &Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let (server_shutdown_sender, server_shutdown) = oneshot();
        let sender = Arc::new(server_shutdown_sender);

        // pick random, unused port
        let port = portpicker::pick_unused_port().expect("Could not find an open port");

        let url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();
        info!("Launching web server on port {port}");
        // Start web server
        async_spawn(async {
            match hotshot_web_server::run_web_server::<TYPES::SignatureKey, NetworkVersion>(
                Some(server_shutdown),
                url,
                NetworkVersion::instance(),
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
}

#[async_trait]
impl<TYPES: NodeType + 'static, NetworkVersion: StaticVersionType + 'static>
    ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>
    for WebServerNetwork<TYPES, NetworkVersion>
{
    /// Blocks until the network is successfully initialized
    async fn wait_for_ready(&self) {
        while !self.inner.connected.load(Ordering::Relaxed) {
            async_sleep(Duration::from_secs(1)).await;
        }
    }
    fn pause(&self) {
        error!("Pausing CDN network");
        self.inner.running.store(false, Ordering::Relaxed);
    }

    fn resume(&self) {
        error!("Resuming CDN network");
        self.inner.running.store(true, Ordering::Relaxed);
    }

    /// Blocks until the network is shut down
    /// then returns true
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            // Cancel poll for latest proposal on shutdown
            if let Some(ref sender) = *self.inner.latest_proposal_task.read().await {
                let _ = sender
                    .send(ConsensusIntentEvent::CancelPollForLatestProposal(1))
                    .await;
            };

            // Cancel poll for latest view sync certificate on shutdown
            if let Some(ref sender) = *self.inner.latest_view_sync_certificate_task.read().await {
                let _ = sender
                    .send(ConsensusIntentEvent::CancelPollForLatestViewSyncCertificate(1))
                    .await;
            };
            self.inner.running.store(false, Ordering::Relaxed);
        };
        boxed_sync(closure)
    }

    /// broadcast message to some subset of nodes
    /// blocking
    async fn broadcast_message<VER: 'static + StaticVersionType>(
        &self,
        message: Message<TYPES>,
        _recipients: BTreeSet<TYPES::SignatureKey>,
        _: VER,
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

    /// broadcast a message only to a DA committee
    /// blocking
    async fn da_broadcast_message<VER: 'static + StaticVersionType>(
        &self,
        message: Message<TYPES>,
        recipients: BTreeSet<TYPES::SignatureKey>,
        bind_version: VER,
    ) -> Result<(), NetworkError> {
        self.broadcast_message(message, recipients, bind_version)
            .await
    }

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message<VER: 'static + StaticVersionType>(
        &self,
        message: Message<TYPES>,
        _recipient: TYPES::SignatureKey,
        _: VER,
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

    /// Receive one or many messages from the underlying network.
    ///
    /// # Errors
    /// Does not error
    async fn recv_msgs(&self) -> Result<Vec<Message<TYPES>>, NetworkError> {
        let mut queue = self.inner.poll_queue_0_1.write().await;
        Ok(queue
            .drain(..)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.get_message().expect("failed to clone message"))
            .collect())
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
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::Proposal,
                                    view_number,
                                    Duration::ZERO,
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
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::VidDisperse,
                                    view_number,
                                    Duration::ZERO,
                                )
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
            ConsensusIntentEvent::PollForLatestProposal => {
                // Only start this task if we haven't already started it.
                let mut cancel_handle = self.inner.latest_proposal_task.write().await;
                if cancel_handle.is_none() {
                    let inner = self.inner.clone();

                    // Create sender and receiver for cancelling the task
                    let (sender, receiver) = unbounded();
                    *cancel_handle = Some(sender);

                    // Create the new task
                    async_spawn(async move {
                        if let Err(e) = inner
                            .poll_web_server(
                                receiver,
                                MessagePurpose::LatestProposal,
                                1,
                                Duration::from_millis(500),
                            )
                            .await
                        {
                            warn!(
                                "Background receive latest quorum proposal polling encountered an error: {:?}",
                                e
                            );
                        }
                    });
                }
            }
            ConsensusIntentEvent::PollForUpgradeProposal(view_number) => {
                // Only start this task if we haven't already started it.
                let mut cancel_handle = self.inner.upgrade_proposal_task.write().await;
                if cancel_handle.is_none() {
                    error!("Starting poll for upgrade proposals!");
                    let inner = self.inner.clone();

                    // Create sender and receiver for cancelling the task
                    let (sender, receiver) = unbounded();
                    *cancel_handle = Some(sender);

                    // Create the new task
                    async_spawn(async move {
                        if let Err(e) = inner
                            .poll_web_server(
                                receiver,
                                MessagePurpose::UpgradeProposal,
                                view_number,
                                Duration::from_millis(500),
                            )
                            .await
                        {
                            warn!(
                                "Background receive latest upgrade proposal polling encountered an error: {:?}",
                                e
                            );
                        }
                    });
                }
            }
            ConsensusIntentEvent::PollForUpgradeVotes(view_number) => {
                // Only start this task if we haven't already started it.
                let mut cancel_handle = self.inner.upgrade_vote_task.write().await;
                if cancel_handle.is_none() {
                    error!("Starting poll for upgrade proposals!");
                    let inner = self.inner.clone();

                    // Create sender and receiver for cancelling the task
                    let (sender, receiver) = unbounded();
                    *cancel_handle = Some(sender);

                    // Create the new task
                    async_spawn(async move {
                        if let Err(e) = inner
                            .poll_web_server(
                                receiver,
                                MessagePurpose::UpgradeVote,
                                view_number,
                                Duration::from_millis(500),
                            )
                            .await
                        {
                            warn!(
                                "Background receive latest upgrade proposal polling encountered an error: {:?}",
                                e
                            );
                        }
                    });
                }
            }
            ConsensusIntentEvent::PollForLatestViewSyncCertificate => {
                // Only start this task if we haven't already started it.
                let mut cancel_handle = self.inner.latest_view_sync_certificate_task.write().await;
                if cancel_handle.is_none() {
                    let inner = self.inner.clone();

                    // Create sender and receiver for cancelling the task
                    let (sender, receiver) = unbounded();
                    *cancel_handle = Some(sender);

                    // Create the new task
                    async_spawn(async move {
                        if let Err(e) = inner
                            .poll_web_server(
                                receiver,
                                MessagePurpose::LatestViewSyncCertificate,
                                1,
                                Duration::from_millis(500),
                            )
                            .await
                        {
                            warn!(
                                "Background receive latest view sync certificate polling encountered an error: {:?}",
                                e
                            );
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
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::Vote,
                                    view_number,
                                    Duration::ZERO,
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
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::DAC,
                                    view_number,
                                    Duration::ZERO,
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
                                    MessagePurpose::ViewSyncCertificate,
                                    view_number,
                                    Duration::ZERO,
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
                                    Duration::ZERO,
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
                                .poll_web_server(
                                    receiver,
                                    MessagePurpose::Data,
                                    view_number,
                                    Duration::ZERO,
                                )
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

impl<TYPES: NodeType, NetworkVersion: 'static + StaticVersionType>
    TestableNetworkingImplementation<TYPES> for WebServerNetwork<TYPES, NetworkVersion>
{
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        _is_da: bool,
        reliability_config: Option<Box<dyn NetworkReliability>>,
        _secondary_network_delay: Duration,
    ) -> AsyncGenerator<(Arc<Self>, Arc<Self>)> {
        let da_gen = Self::single_generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            true,
            &reliability_config,
        );
        let quorum_gen = Self::single_generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            false,
            &reliability_config,
        );
        // Start each node's web server client
        Box::pin(move |id| {
            let da_gen = da_gen(id);
            let quorum_gen = quorum_gen(id);
            Box::pin(async move { (quorum_gen.into(), da_gen.into()) })
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}
