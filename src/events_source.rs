use async_broadcast::{broadcast, InactiveReceiver, Sender as BroadcastSender};
use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, Stream, StreamExt};
use hotshot_types::event::EventType;
use hotshot_types::{
    data::{DaProposal, QuorumProposal},
    error::HotShotError,
    event::{error_adaptor, Event},
    message::Proposal,
    traits::node_implementation::NodeType,
    PeerConfig,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;
use tide_disco::method::ReadState;
const RETAINED_EVENTS_COUNT: usize = 4096;

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
        latest_decide_view_num: Types::Time,
        /// Optional information of the number of transactions in the block
        block_size: Option<u64>,
    },
    /// DA proposal was received from the network
    HotshotDaProposal {
        /// Contents of the proposal
        proposal: Proposal<Types, DaProposal<Types>>,
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
    type EventStream: Stream<Item = Arc<Event<Types>>> + Unpin + Send + 'static;
    async fn get_event_stream(&self) -> Self::EventStream;

    async fn subscribe_events(&self) -> BoxStream<'static, Arc<Event<Types>>> {
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
    inactive_to_subscribe_clone_recv: InactiveReceiver<Arc<Event<Types>>>,
    subscriber_send_channel: BroadcastSender<Arc<Event<Types>>>,

    // required for sending startup info
    known_nodes_with_stake: Vec<PeerConfig<Types::SignatureKey>>,
    non_staked_node_count: usize,
    filter: Option<EventFilterSet<Types>>,
}

impl<Types: NodeType> EventsStreamer<Types> {
    pub fn known_node_with_stake(&self) -> Vec<PeerConfig<Types::SignatureKey>> {
        self.known_nodes_with_stake.clone()
    }

    pub fn non_staked_node_count(&self) -> usize {
        self.non_staked_node_count
    }

    pub fn filter(&self) -> Option<&EventFilterSet<Types>> {
        self.filter.as_ref()
    }
}

#[async_trait]
impl<Types: NodeType> EventConsumer<Types> for EventsStreamer<Types> {
    async fn handle_event(&mut self, event: Event<Types>) {
        let should_broadcast = self
            .filter()
            .map_or(true, |filter| filter.should_broadcast(&event.event));

        if should_broadcast {
            if let Err(e) = self.subscriber_send_channel.broadcast(event.into()).await {
                tracing::error!("Error broadcasting the event: {:?}", e);
            }
        }
    }
}

/// Wrapper struct representing a set of event filters.
#[derive(Clone, Debug)]
pub struct EventFilterSet<Types: NodeType>(pub(crate) Vec<EventFilter<Types>>);

/// `From` trait impl to create an `EventFilterSet` from a vector of `EventFilter`s.
impl<Types: NodeType> From<Vec<EventFilter<Types>>> for EventFilterSet<Types> {
    fn from(filters: Vec<EventFilter<Types>>) -> Self {
        EventFilterSet(filters)
    }
}

/// `From` trait impl to create an `EventFilterSet` from a single `EventFilter`.
impl<Types: NodeType> From<EventFilter<Types>> for EventFilterSet<Types> {
    fn from(filter: EventFilter<Types>) -> Self {
        EventFilterSet(vec![filter])
    }
}

impl<Types: NodeType> EventFilterSet<Types> {
    /// Determines whether the given hotshot event should be broadcast based on the filters in the set.
    ///
    ///  Returns `true` if the event should be broadcast, `false` otherwise.
    pub(crate) fn should_broadcast(&self, hotshot_event: &EventType<Types>) -> bool {
        let filter = &self.0;

        match hotshot_event {
            EventType::Error { .. } => filter.contains(&EventFilter::Error),
            EventType::Decide { .. } => filter.contains(&EventFilter::Decide),
            EventType::ReplicaViewTimeout { .. } => {
                filter.contains(&EventFilter::ReplicaViewTimeout)
            }
            EventType::ViewFinished { .. } => filter.contains(&EventFilter::ViewFinished),
            EventType::ViewTimeout { .. } => filter.contains(&EventFilter::ViewTimeout),
            EventType::Transactions { .. } => filter.contains(&EventFilter::Transactions),
            EventType::DaProposal { .. } => filter.contains(&EventFilter::DaProposal),
            EventType::QuorumProposal { .. } => filter.contains(&EventFilter::QuorumProposal),
            EventType::UpgradeProposal { .. } => filter.contains(&EventFilter::UpgradeProposal),
            _ => false,
        }
    }
}

/// Possible event filters
/// If the hotshot`EventType` enum is modified, this enum should also be updated
#[derive(Clone, Debug, PartialEq)]
pub enum EventFilter<Types: NodeType> {
    Error,
    Decide,
    ReplicaViewTimeout,
    ViewFinished,
    ViewTimeout,
    Transactions,
    DaProposal,
    QuorumProposal,
    UpgradeProposal,
    Pd(PhantomData<Types>),
}

#[async_trait]
impl<Types: NodeType> EventsSource<Types> for EventsStreamer<Types> {
    type EventStream = BoxStream<'static, Arc<Event<Types>>>;

    async fn get_event_stream(&self) -> Self::EventStream {
        self.inactive_to_subscribe_clone_recv
            .activate_cloned()
            .boxed()
    }
}
impl<Types: NodeType> EventsStreamer<Types> {
    pub fn new(
        known_nodes_with_stake: Vec<PeerConfig<Types::SignatureKey>>,
        non_staked_node_count: usize,
        filter: Option<EventFilterSet<Types>>,
    ) -> Self {
        let (mut subscriber_send_channel, to_subscribe_clone_recv) =
            broadcast::<Arc<Event<Types>>>(RETAINED_EVENTS_COUNT);
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
            filter,
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
