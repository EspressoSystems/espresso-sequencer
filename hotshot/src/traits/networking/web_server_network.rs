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
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    message::{Message, MessagePurpose},
    traits::{
        election::Membership,
        network::{
            CommunicationChannel, ConnectedNetwork, ConsensusIntentEvent, NetworkError, NetworkMsg,
            TestableChannelImplementation, TestableNetworkingImplementation, TransmitType,
            WebServerNetworkError,
        },
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
};
use hotshot_web_server::{self, config};
use rand::random;
use serde::{Deserialize, Serialize};

use hotshot_types::traits::network::{NetworkReliability, ViewMessage};
use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use surf_disco::error::ClientError;
use tracing::{debug, error, info};
/// Represents the communication channel abstraction for the web server
#[derive(Clone, Debug)]
pub struct WebCommChannel<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
>(
    Arc<WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>>,
    PhantomData<(MEMBERSHIP, I)>,
);

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    WebCommChannel<TYPES, I, MEMBERSHIP>
{
    /// Create new communication channel
    #[must_use]
    pub fn new(
        network: Arc<WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>>,
    ) -> Self {
        Self(network, PhantomData)
    }
}

/// The web server network state
#[derive(Clone, Debug)]
pub struct WebServerNetwork<M: NetworkMsg, KEY: SignatureKey, TYPES: NodeType> {
    /// The inner, core state of the web server network
    inner: Arc<Inner<M, KEY, TYPES>>,
    /// An optional shutdown signal. This is only used when this connection is created through the `TestableNetworkingImplementation` API.
    server_shutdown_signal: Option<Arc<OneShotSender<()>>>,
}

impl<M: NetworkMsg, KEY: SignatureKey, TYPES: NodeType> WebServerNetwork<M, KEY, TYPES> {
    /// Post a message to the web server and return the result
    async fn post_message_to_web_server(&self, message: SendMsg<M>) -> Result<(), NetworkError> {
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
}

/// Represents the core of web server networking
#[derive(Debug)]
struct Inner<M: NetworkMsg, KEY: SignatureKey, TYPES: NodeType> {
    /// Phantom data for generic types
    phantom: PhantomData<(KEY, TYPES::ElectionConfigType)>,
    /// Our own key
    _own_key: TYPES::SignatureKey,
    /// Queue for broadcasted messages
    broadcast_poll_queue: Arc<RwLock<Vec<RecvMsg<M>>>>,
    /// Queue for direct messages
    direct_poll_queue: Arc<RwLock<Vec<RecvMsg<M>>>>,
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
    proposal_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
    /// Task map for quorum votes.
    vote_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
    /// Task map for DACs.
    dac_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
    /// Task map for view sync certificates.
    view_sync_cert_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
    /// Task map for view sync votes.
    view_sync_vote_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
    /// Task map for transactions
    txn_task_map:
        Arc<RwLock<HashMap<u64, UnboundedSender<ConsensusIntentEvent<TYPES::SignatureKey>>>>>,
}

impl<M: NetworkMsg, KEY: SignatureKey, TYPES: NodeType> Inner<M, KEY, TYPES> {
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

        if message_purpose == MessagePurpose::Data {
            tx_index = *self.tx_index.read().await;
            debug!("Previous tx index was {}", tx_index);
        };

        while self.running.load(Ordering::Relaxed) {
            let endpoint = match message_purpose {
                MessagePurpose::Proposal => config::get_proposal_route(view_number),
                MessagePurpose::CurrentProposal => config::get_recent_proposal_route(),
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
                MessagePurpose::VidVote => config::get_vid_vote_route(view_number, vote_index), // like `Vote`
                MessagePurpose::VidCert => config::get_vid_cert_route(view_number), // like `DAC`
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
                            MessagePurpose::CurrentProposal => {
                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                return Ok(());
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
                            MessagePurpose::VidVote => {
                                // TODO copy-pasted from `MessagePurpose::Vote` https://github.com/EspressoSystems/HotShot/issues/1690
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
                            MessagePurpose::VidCert => {
                                // TODO copy-pasted from `MessagePurpose::DAC` https://github.com/EspressoSystems/HotShot/issues/1690
                                debug!(
                                    "Received VID cert from web server for view {} {}",
                                    view_number, self.is_da
                                );
                                // Only pushing the first proposal since we will soon only be allowing 1 proposal per view
                                self.broadcast_poll_queue
                                    .write()
                                    .await
                                    .push(deserialized_messages[0].clone());

                                // return if we found a VID cert, since there will only be 1 per view
                                // In future we should check to make sure VID cert is valid
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
                                    broadcast_poll_queue.push(cert.clone());
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

                        _ => unimplemented!(),
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
    ) -> Result<Option<(u64, Vec<RecvMsg<M>>)>, NetworkError> {
        let result: Result<Option<(u64, Vec<Vec<u8>>)>, ClientError> =
            self.client.get(&endpoint).send().await;
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
    ) -> Result<Option<Vec<RecvMsg<M>>>, NetworkError> {
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl<
        M: NetworkMsg + 'static + ViewMessage<TYPES>,
        K: SignatureKey + 'static,
        TYPES: NodeType + 'static,
    > WebServerNetwork<M, K, TYPES>
{
    /// Creates a new instance of the `WebServerNetwork`
    /// # Panics
    /// if the web server url is malformed
    pub fn create(
        host: &str,
        port: u16,
        wait_between_polls: Duration,
        key: TYPES::SignatureKey,
        is_da_server: bool,
    ) -> Self {
        let base_url_string = format!("http://{host}:{port}");
        info!("Connecting to web server at {base_url_string:?} is da: {is_da_server}");

        let base_url = base_url_string.parse();
        if base_url.is_err() {
            error!("Web server url {:?} is malformed", base_url_string);
        }

        // TODO ED Wait for healthcheck
        let client = surf_disco::Client::<ClientError>::new(base_url.unwrap());

        let inner = Arc::new(Inner {
            phantom: PhantomData,
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
            dac_task_map: Arc::default(),
            view_sync_cert_task_map: Arc::default(),
            view_sync_vote_task_map: Arc::default(),
            txn_task_map: Arc::default(),
        });

        inner.connected.store(true, Ordering::Relaxed);

        Self {
            inner,
            server_shutdown_signal: None,
        }
    }

    /// Parses a message to find the appropriate endpoint
    /// Returns a `SendMsg` containing the endpoint
    fn parse_post_message(message: M) -> Result<SendMsg<M>, WebServerNetworkError> {
        let view_number: TYPES::Time = message.get_view_number();

        let endpoint = match &message.purpose() {
            MessagePurpose::Proposal => config::post_proposal_route(*view_number),
            MessagePurpose::Vote => config::post_vote_route(*view_number),
            MessagePurpose::Data => config::post_transactions_route(),
            MessagePurpose::Internal | MessagePurpose::CurrentProposal => {
                return Err(WebServerNetworkError::EndpointError)
            }
            MessagePurpose::ViewSyncProposal => {
                // error!("Posting view sync proposal route is: {}", config::post_view_sync_proposal_route(*view_number));
                config::post_view_sync_proposal_route(*view_number)
            }
            MessagePurpose::ViewSyncVote => config::post_view_sync_vote_route(*view_number),
            MessagePurpose::DAC => config::post_da_certificate_route(*view_number),
            MessagePurpose::VidVote => config::post_vid_vote_route(*view_number),
            MessagePurpose::VidDisperse => config::post_vid_disperse_route(*view_number),
            MessagePurpose::VidCert => config::post_vid_cert_route(*view_number),
        };

        let network_msg: SendMsg<M> = SendMsg {
            message: Some(message),
            endpoint,
        };
        Ok(network_msg)
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>
    for WebCommChannel<TYPES, I, MEMBERSHIP>
{
    type NETWORK = WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>;
    /// Blocks until node is successfully initialized
    /// into the network
    async fn wait_for_ready(&self) {
        <WebServerNetwork<_, _, _> as ConnectedNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
        >>::wait_for_ready(&self.0)
        .await;
    }

    /// checks if the network is ready
    /// nonblocking
    async fn is_ready(&self) -> bool {
        <WebServerNetwork<_, _, _,> as ConnectedNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
        >>::is_ready(&self.0)
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
            <WebServerNetwork<_, _, _> as ConnectedNetwork<
                Message<TYPES, I>,
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
        message: Message<TYPES, I>,
        _election: &MEMBERSHIP,
    ) -> Result<(), NetworkError> {
        self.0.broadcast_message(message, BTreeSet::new()).await
    }

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message(
        &self,
        message: Message<TYPES, I>,
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
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES, I>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            <WebServerNetwork<_, _, _> as ConnectedNetwork<
                Message<TYPES, I>,
                TYPES::SignatureKey,
            >>::recv_msgs(&self.0, transmit_type)
            .await
        };
        boxed_sync(closure)
    }

    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<TYPES::SignatureKey>) {
        <WebServerNetwork<_, _, _> as ConnectedNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
        >>::inject_consensus_info(&self.0, event)
        .await;
    }
}

#[async_trait]
impl<
        M: NetworkMsg + 'static + ViewMessage<TYPES>,
        K: SignatureKey + 'static,
        TYPES: NodeType + 'static,
    > ConnectedNetwork<M, K> for WebServerNetwork<M, K, TYPES>
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
        message: M,
        _recipients: BTreeSet<K>,
    ) -> Result<(), NetworkError> {
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
    async fn direct_message(&self, message: M, _recipient: K) -> Result<(), NetworkError> {
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
    ) -> BoxSyncFuture<'b, Result<Vec<M>, NetworkError>>
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
    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<K>) {
        debug!(
            "Injecting event: {:?} is da {}",
            event.clone(),
            self.inner.is_da
        );

        // TODO ED Need to handle canceling tasks that don't receive their expected output (such a proposal that never comes)
        // TODO ED Need to GC all old views, not just singular views, could lead to a network leak

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
                                error!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }

                // GC proposal collection if we are two views in the future
                if let Some((_, sender)) = task_map.remove_entry(&view_number.wrapping_sub(2)) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForProposal(
                            view_number.wrapping_sub(2),
                        ))
                        .await;
                }
            }
            ConsensusIntentEvent::PollForCurrentProposal => {
                // create new task
                let (_, receiver) = unbounded();

                async_spawn({
                    let inner_clone = self.inner.clone();
                    async move {
                        if let Err(e) = inner_clone
                            .poll_web_server(receiver, MessagePurpose::CurrentProposal, 1)
                            .await
                        {
                            error!(
                                "Background receive proposal polling encountered an error: {:?}",
                                e
                            );
                        }
                    }
                });
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
                                error!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }

                // GC proposal collection if we are two views in the future
                // TODO ED This won't work for vote collection, last task is more than 2 view ago depending on size of network, will need to rely on cancel task from consensus
                if let Some((_, sender)) = task_map.remove_entry(&(view_number.wrapping_sub(2))) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForVotes(
                            view_number.wrapping_sub(2),
                        ))
                        .await;
                }
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
                                error!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }

                // GC proposal collection if we are two views in the future
                if let Some((_, sender)) = task_map.remove_entry(&(view_number.wrapping_sub(2))) {
                    // Send task cancel message to old task

                    // If task already exited we expect an error
                    let _res = sender
                        .send(ConsensusIntentEvent::CancelPollForDAC(
                            view_number.wrapping_sub(2),
                        ))
                        .await;
                }
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
                                    MessagePurpose::ViewSyncProposal,
                                    view_number,
                                )
                                .await
                            {
                                error!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }

                // TODO ED Do we need to GC before returning?  Or will view sync task handle that?
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
                                error!(
                                    "Background receive proposal polling encountered an error: {:?}",
                                    e
                                );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }
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
                if let std::collections::hash_map::Entry::Vacant(e) = task_map.entry(view_number) {
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
                                error!(
                                                               "Background receive transaction polling encountered an error: {:?}",
                                                                 e
                                                               );
                            }
                        }
                    });
                } else {
                    error!("Somehow task already existed!");
                }

                // TODO ED Do we need to GC before returning?  Or will view sync task handle that?
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>
{
    fn generator(
        expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        is_da: bool,
        _reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let (server_shutdown_sender, server_shutdown) = oneshot();
        let sender = Arc::new(server_shutdown_sender);
        // TODO ED Restrict this to be an open port using portpicker
        let port = random::<u16>();
        info!("Launching web server on port {port}");
        // Start web server
        async_spawn(hotshot_web_server::run_web_server::<TYPES::SignatureKey>(
            Some(server_shutdown),
            port,
        ));

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
            let mut network = WebServerNetwork::create(
                "0.0.0.0",
                port,
                Duration::from_millis(100),
                known_nodes[id as usize].clone(),
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for WebCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
        _reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <WebServerNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
            TYPES,
        > as TestableNetworkingImplementation<_, _>>::generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            is_da,
            // unsupported
            None,
        );
        Box::new(move |node_id| Self(generator(node_id).into(), PhantomData))
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        MEMBERSHIP,
        WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>,
    > for WebCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generate_network() -> Box<
        dyn Fn(Arc<WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>>) -> Self
            + 'static,
    > {
        Box::new(move |network| WebCommChannel::new(network))
    }
}
