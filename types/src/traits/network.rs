//! Network access compatibility
//!
//! Contains types and traits used by `HotShot` to abstract over network access

#[cfg(async_executor_impl = "async-std")]
use async_std::future::TimeoutError;
use hotshot_task::BoxSyncFuture;
use libp2p_networking::network::NetworkNodeHandleError;
#[cfg(async_executor_impl = "tokio")]
use tokio::time::error::Elapsed as TimeoutError;
#[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}
use super::{election::Membership, node_implementation::NodeType, signature_key::SignatureKey};
use crate::{data::ProposalType, message::MessagePurpose, vote::VoteType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{collections::BTreeSet, fmt::Debug, sync::Arc, time::Duration};

impl From<NetworkNodeHandleError> for NetworkError {
    fn from(error: NetworkNodeHandleError) -> Self {
        match error {
            NetworkNodeHandleError::SerializationError { source } => {
                NetworkError::FailedToSerialize { source }
            }
            NetworkNodeHandleError::DeserializationError { source } => {
                NetworkError::FailedToDeserialize { source }
            }
            NetworkNodeHandleError::TimeoutError { source } => NetworkError::Timeout { source },
            NetworkNodeHandleError::Killed => NetworkError::ShutDown,
            source => NetworkError::Libp2p { source },
        }
    }
}

/// for any errors we decide to add to memory network
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MemoryNetworkError {
    /// stub
    Stub,
}

/// Centralized server specific errors
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CentralizedServerNetworkError {
    /// The centralized server could not find a specific message.
    NoMessagesInQueue,
}

/// Web server specific errors
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum WebServerNetworkError {
    /// The injected consensus data is incorrect
    IncorrectConsensusData,
    /// The client returned an error
    ClientError,
    /// Endpoint parsed incorrectly
    EndpointError,
    /// Client disconnected
    ClientDisconnected,
}

/// the type of transmission
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransmitType {
    /// directly transmit
    Direct,
    /// broadcast the message to all
    Broadcast,
}

/// Error type for networking
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum NetworkError {
    /// Libp2p specific errors
    Libp2p {
        /// source of error
        source: NetworkNodeHandleError,
    },
    /// memory network specific errors
    MemoryNetwork {
        /// source of error
        source: MemoryNetworkError,
    },
    /// Centralized server specific errors
    CentralizedServer {
        /// source of error
        source: CentralizedServerNetworkError,
    },

    /// Web server specific errors
    WebServer {
        /// source of error
        source: WebServerNetworkError,
    },
    /// unimplemented functionality
    UnimplementedFeature,
    /// Could not deliver a message to a specified recipient
    CouldNotDeliver,
    /// Attempted to deliver a message to an unknown node
    NoSuchNode,
    /// Failed to serialize a network message
    FailedToSerialize {
        /// Originating bincode error
        source: bincode::Error,
    },
    /// Failed to deserealize a network message
    FailedToDeserialize {
        /// originating bincode error
        source: bincode::Error,
    },
    /// A timeout occurred
    Timeout {
        /// Source of error
        source: TimeoutError,
    },
    /// Error sending output to consumer of NetworkingImplementation
    /// TODO this should have more information
    ChannelSend,
    /// The underlying connection has been shut down
    ShutDown,
    /// unable to cancel a request, the request has already been cancelled
    UnableToCancel,
}

#[derive(Clone, Debug)]
// Storing view number as a u64 to avoid the need TYPES generic
/// Events to poll or cancel consensus processes.
pub enum ConsensusIntentEvent {
    /// Poll for votes for a particular view
    PollForVotes(u64),
    /// Poll for a proposal for a particular view
    PollForProposal(u64),
    /// Poll for a DAC for a particular view
    PollForDAC(u64),
    /// Poll for view sync votes starting at a particular view
    PollForViewSyncVotes(u64),
    /// Poll for view sync proposals (certificates) for a particular view
    PollForViewSyncCertificate(u64),
    /// Poll for new transactions
    PollForTransactions(u64),
    /// Cancel polling for votes
    CancelPollForVotes(u64),
    /// Cancel polling for view sync votes.
    CancelPollForViewSyncVotes(u64),
    /// Cancel polling for proposals.
    CancelPollForProposal(u64),
    /// Cancal polling for DAC.
    CancelPollForDAC(u64),
    /// Cancel polling for view sync certificate.
    CancelPollForViewSyncCertificate(u64),
    /// Cancel polling for transactions
    CancelPollForTransactions(u64),
}

impl ConsensusIntentEvent {
    /// Get the view number of the event.
    #[must_use]
    pub fn view_number(&self) -> u64 {
        match &self {
            ConsensusIntentEvent::PollForVotes(view_number)
            | ConsensusIntentEvent::PollForProposal(view_number)
            | ConsensusIntentEvent::PollForDAC(view_number)
            | ConsensusIntentEvent::PollForViewSyncVotes(view_number)
            | ConsensusIntentEvent::CancelPollForViewSyncVotes(view_number)
            | ConsensusIntentEvent::CancelPollForVotes(view_number)
            | ConsensusIntentEvent::CancelPollForProposal(view_number)
            | ConsensusIntentEvent::CancelPollForDAC(view_number)
            | ConsensusIntentEvent::CancelPollForViewSyncCertificate(view_number)
            | ConsensusIntentEvent::PollForViewSyncCertificate(view_number)
            | ConsensusIntentEvent::PollForTransactions(view_number)
            | ConsensusIntentEvent::CancelPollForTransactions(view_number) => *view_number,
        }
    }
}

/// common traits we would like our network messages to implement
pub trait NetworkMsg:
    Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + Debug + 'static
{
}

/// a message
pub trait ViewMessage<TYPES: NodeType> {
    /// get the view out of the message
    fn get_view_number(&self) -> TYPES::Time;
    // TODO move out of this trait.
    /// get the purpose of the message
    fn purpose(&self) -> MessagePurpose;
}

/// API for interacting directly with a consensus committee
/// intended to be implemented for both DA and for validating consensus committees
#[async_trait]
pub trait CommunicationChannel<
    TYPES: NodeType,
    M: NetworkMsg,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    VOTE: VoteType<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
>: Clone + Debug + Send + Sync + 'static
{
    /// Underlying Network implementation's type
    type NETWORK;
    /// Blocks until node is successfully initialized
    /// into the network
    async fn wait_for_ready(&self);

    /// checks if the network is ready
    /// nonblocking
    async fn is_ready(&self) -> bool;

    /// Shut down this network. Afterwards this network should no longer be used.
    ///
    /// This should also cause other functions to immediately return with a [`NetworkError`]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b;

    /// broadcast message to those listening on the communication channel
    /// blocking
    async fn broadcast_message(
        &self,
        message: M,
        election: &MEMBERSHIP,
    ) -> Result<(), NetworkError>;

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message(
        &self,
        message: M,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError>;

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
        Self: 'b;

    /// look up a node
    /// blocking
    async fn lookup_node(&self, pk: TYPES::SignatureKey) -> Result<(), NetworkError>;

    /// Injects consensus data such as view number into the networking implementation
    /// blocking
    async fn inject_consensus_info(&self, event: ConsensusIntentEvent);
}

/// represents a networking implmentration
/// exposes low level API for interacting with a network
/// intended to be implemented for libp2p, the centralized server,
/// and memory network
#[async_trait]
pub trait ConnectedNetwork<M: NetworkMsg, K: SignatureKey + 'static>:
    Clone + Send + Sync + 'static
{
    /// Blocks until the network is successfully initialized
    async fn wait_for_ready(&self);

    /// checks if the network is ready
    /// nonblocking
    async fn is_ready(&self) -> bool;

    /// Blocks until the network is shut down
    /// then returns true
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b;

    /// broadcast message to some subset of nodes
    /// blocking
    async fn broadcast_message(
        &self,
        message: M,
        recipients: BTreeSet<K>,
    ) -> Result<(), NetworkError>;

    /// Sends a direct message to a specific node
    /// blocking
    async fn direct_message(&self, message: M, recipient: K) -> Result<(), NetworkError>;

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
        Self: 'b;

    /// look up a node
    /// blocking
    async fn lookup_node(&self, pk: K) -> Result<(), NetworkError>;

    /// Injects consensus data such as view number into the networking implementation
    /// blocking
    /// Ideally we would pass in the `Time` type, but that requires making the entire trait generic over NodeType
    async fn inject_consensus_info(&self, event: ConsensusIntentEvent);
}

/// Describes additional functionality needed by the test network implementation
pub trait TestableNetworkingImplementation<TYPES: NodeType, M: NetworkMsg> {
    /// generates a network given an expected node count
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static>;

    /// Get the number of messages in-flight.
    ///
    /// Some implementations will not be able to tell how many messages there are in-flight. These implementations should return `None`.
    fn in_flight_message_count(&self) -> Option<usize>;
}
/// Describes additional functionality needed by the test communication channel
pub trait TestableChannelImplementation<
    TYPES: NodeType,
    M: NetworkMsg,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    VOTE: VoteType<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK,
>: CommunicationChannel<TYPES, M, PROPOSAL, VOTE, MEMBERSHIP>
{
    /// generates the `CommunicationChannel` given it's associated network type
    fn generate_network() -> Box<dyn Fn(Arc<NETWORK>) -> Self + 'static>;
}

/// Changes that can occur in the network
#[derive(Debug)]
pub enum NetworkChange<P: SignatureKey> {
    /// A node is connected
    NodeConnected(P),

    /// A node is disconnected
    NodeDisconnected(P),
}

/// interface describing how reliable the network is
pub trait NetworkReliability: Debug + Sync + std::marker::Send {
    /// Sample from bernoulli distribution to decide whether
    /// or not to keep a packet
    /// # Panics
    ///
    /// Panics if `self.keep_numerator > self.keep_denominator`
    ///
    fn sample_keep(&self) -> bool;
    /// sample from uniform distribution to decide whether
    /// or not to keep a packet
    fn sample_delay(&self) -> Duration;
}
