use async_broadcast::{broadcast, InactiveReceiver, Sender as BroadcastSender};
use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::stream::{self, BoxStream, Stream, StreamExt};
use hotshot_types::{
    data::{DAProposal, QuorumProposal},
    error::HotShotError,
    event::{error_adaptor, Event, EventType, LeafChain},
    message::Proposal,
    traits::node_implementation::{ConsensusTime, NodeType},
    PeerConfig,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide_disco::method::ReadState;
const RETAINED_EVENTS_COUNT: usize = 4096;

/// A builder event
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "Types: NodeType"))]
pub struct BuilderEvent<Types: NodeType> {
    /// The view number that this event originates from
    pub view_number: Types::Time,

    /// The underlying event
    pub event: BuilderEventType<Types>,
}

// impl From event to builder event
impl<Types: NodeType> From<Event<Types>> for BuilderEvent<Types> {
    fn from(event: Event<Types>) -> Self {
        BuilderEvent {
            view_number: event.view_number,
            event: match event.event {
                EventType::Error { error } => BuilderEventType::HotshotError { error },
                EventType::Transactions { transactions } => {
                    BuilderEventType::HotshotTransactions { transactions }
                }
                EventType::Decide {
                    leaf_chain,
                    block_size,
                    ..
                } => BuilderEventType::HotshotDecide {
                    leaf_chain,
                    block_size,
                },
                EventType::DAProposal { proposal, sender } => {
                    BuilderEventType::HotshotDAProposal { proposal, sender }
                }
                EventType::QuorumProposal { proposal, sender } => {
                    BuilderEventType::HotshotQuorumProposal { proposal, sender }
                }
                _ => BuilderEventType::Unknown,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "Types: NodeType"))]
pub enum BuilderEventType<Types: NodeType> {
    // Information required by the builder to create a membership to get view leader
    StartupInfo {
        known_node_with_stake: Vec<PeerConfig<Types::SignatureKey>>,
        non_staked_node_count: usize,
    },
    /// Hotshot error
    HotshotError {
        /// The underlying error
        #[serde(with = "error_adaptor")]
        error: Arc<HotShotError<Types>>,
    },
    /// Hotshot public mempool transactions
    HotshotTransactions {
        /// The list of hotshot transactions
        transactions: Vec<Types::Transaction>,
    },
    // Decide event with the chain of decided leaves
    HotshotDecide {
        /// The chain of decided leaves with its corresponding state and VID info.
        leaf_chain: Arc<LeafChain<Types>>,
        /// Optional information of the number of transactions in the block
        block_size: Option<u64>,
    },
    /// DA proposal was received from the network
    HotshotDAProposal {
        /// Contents of the proposal
        proposal: Proposal<Types, DAProposal<Types>>,
        /// Public key of the leader submitting the proposal
        sender: Types::SignatureKey,
    },
    /// Quorum proposal was received from the network
    HotshotQuorumProposal {
        /// Contents of the proposal
        proposal: Proposal<Types, QuorumProposal<Types>>,
        /// Public key of the leader submitting the proposal
        sender: Types::SignatureKey,
    },
    Unknown,
}

#[async_trait]
pub trait EventsSource<Types>
where
    Types: NodeType,
{
    type EventStream: Stream<Item = Arc<BuilderEvent<Types>>> + Unpin + Send + 'static;
    async fn get_event_stream(&self) -> Self::EventStream;

    async fn subscribe_events(&self) -> BoxStream<'static, Arc<BuilderEvent<Types>>> {
        self.get_event_stream().await.boxed()
    }
}

#[async_trait]
pub trait EventConsumer<Types>
where
    Types: NodeType,
{
    async fn handle_event(&mut self, event: Event<Types>);
}

#[derive(Debug)]
pub struct EventsStreamer<Types: NodeType> {
    // required for api subscription
    inactive_to_subscribe_clone_recv: InactiveReceiver<Arc<BuilderEvent<Types>>>,
    subscriber_send_channel: BroadcastSender<Arc<BuilderEvent<Types>>>,

    // required for sending startup info
    known_nodes_with_stake: Vec<PeerConfig<Types::SignatureKey>>,
    non_staked_node_count: usize,
}

#[async_trait]
impl<Types: NodeType> EventConsumer<Types> for EventsStreamer<Types> {
    async fn handle_event(&mut self, event: Event<Types>) {
        let filter = match event {
            Event {
                event: EventType::DAProposal { .. },
                ..
            } => true,
            Event {
                event: EventType::QuorumProposal { .. },
                ..
            } => true,
            Event {
                event: EventType::Transactions { .. },
                ..
            } => true,
            Event {
                event: EventType::Decide { .. },
                ..
            } => true,
            Event { .. } => false,
        };
        if filter {
            let builder_event = Arc::new(BuilderEvent::from(event));
            let _status = self.subscriber_send_channel.broadcast(builder_event).await;
        }
    }
}

#[async_trait]
impl<Types: NodeType> EventsSource<Types> for EventsStreamer<Types> {
    type EventStream = BoxStream<'static, Arc<BuilderEvent<Types>>>;

    async fn get_event_stream(&self) -> Self::EventStream {
        let recv_channel = self.inactive_to_subscribe_clone_recv.activate_cloned();
        let startup_event_initialized = false;
        let startup_event = self.get_startup_event().clone();
        stream::unfold(
            (recv_channel, startup_event, startup_event_initialized),
            |(mut recv_channel, startup_event, mut startup_event_initialized)| async move {
                let event_res = if startup_event_initialized {
                    recv_channel.recv().await.ok()
                } else {
                    startup_event_initialized = true;
                    Some(Arc::new(startup_event.clone()))
                };
                event_res.map(|event| {
                    (
                        event,
                        (recv_channel, startup_event, startup_event_initialized),
                    )
                })
            },
        )
        .boxed()
    }
}
impl<Types: NodeType> EventsStreamer<Types> {
    pub fn new(
        known_nodes_with_stake: Vec<PeerConfig<Types::SignatureKey>>,
        non_staked_node_count: usize,
    ) -> Self {
        let (mut subscriber_send_channel, to_subscribe_clone_recv) =
            broadcast::<Arc<BuilderEvent<Types>>>(RETAINED_EVENTS_COUNT);
        // set the overflow to true to drop older messages from the channel
        subscriber_send_channel.set_overflow(true);
        // set the await active to false to not block the sender
        subscriber_send_channel.set_await_active(false);
        let inactive_to_subscribe_clone_recv = to_subscribe_clone_recv.deactivate();
        EventsStreamer {
            subscriber_send_channel,
            inactive_to_subscribe_clone_recv,
            known_nodes_with_stake,
            non_staked_node_count,
        }
    }
    pub fn get_startup_event(&self) -> BuilderEvent<Types> {
        BuilderEvent {
            view_number: Types::Time::genesis(),
            event: BuilderEventType::StartupInfo {
                known_node_with_stake: self.known_nodes_with_stake.clone(),
                non_staked_node_count: self.non_staked_node_count,
            },
        }
    }
}

#[async_trait]
impl<Types: NodeType> ReadState for EventsStreamer<Types> {
    type State = Self;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}
