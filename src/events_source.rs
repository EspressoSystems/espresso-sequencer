use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use async_trait::async_trait;
use futures::stream::{self, BoxStream, Stream, StreamExt};
use hotshot_types::{
    data::{DAProposal, QuorumProposal},
    error::HotShotError,
    event::{error_adaptor, Event, EventType, LeafChain},
    message::Proposal,
    traits::node_implementation::NodeType,
};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::Arc;
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
pub fn get_builder_event_from_hotshot_event<Types: NodeType>(
    hotshot_event: Event<Types>,
    leader: Option<Types::SignatureKey>,
    vid_nodes: Option<NonZeroUsize>,
) -> Option<BuilderEvent<Types>> {
    // match the event and generate the builder event
    let builder_event = match hotshot_event.event {
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
        EventType::DAProposal { proposal, sender } => BuilderEventType::HotshotDAProposal {
            proposal,
            sender,
            leader: leader.unwrap(),
            vid_nodes: vid_nodes.unwrap(),
        },
        EventType::QuorumProposal { proposal, sender } => BuilderEventType::HotshotQuorumProposal {
            proposal,
            sender,
            leader: leader.unwrap(),
        },
        _ => return None,
    };
    Some(BuilderEvent {
        view_number: hotshot_event.view_number,
        event: builder_event,
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "Types: NodeType"))]
pub enum BuilderEventType<Types: NodeType> {
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
        /// leader for the view
        leader: Types::SignatureKey,
        /// nodes in vid calculation
        vid_nodes: NonZeroUsize,
    },
    /// Quorum proposal was received from the network
    HotshotQuorumProposal {
        /// Contents of the proposal
        proposal: Proposal<Types, QuorumProposal<Types>>,
        /// Public key of the leader submitting the proposal
        sender: Types::SignatureKey,
        /// leader for the view
        leader: Types::SignatureKey,
    },
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
    async fn handle_event(
        &mut self,
        event: Event<Types>,
        leader: Option<Types::SignatureKey>,
        vid_nodes: Option<NonZeroUsize>,
    );
}

#[derive(Debug)]
pub struct EventsStreamer<Types: NodeType> {
    to_subscribe_clone_recv: BroadcastReceiver<Arc<BuilderEvent<Types>>>,
    subscriber_send_channel: BroadcastSender<Arc<BuilderEvent<Types>>>,
}

#[async_trait]
impl<Types: NodeType> EventConsumer<Types> for EventsStreamer<Types> {
    async fn handle_event(
        &mut self,
        event: Event<Types>,
        leader: Option<Types::SignatureKey>,
        vid_nodes: Option<NonZeroUsize>,
    ) {
        let builder_event = get_builder_event_from_hotshot_event(event, leader, vid_nodes);
        if builder_event.is_some() {
            let event = Arc::new(builder_event.unwrap().clone());
            let _status = self.subscriber_send_channel.broadcast(event).await;
        }
    }
}

#[async_trait]
impl<Types: NodeType> EventsSource<Types> for EventsStreamer<Types> {
    type EventStream = BoxStream<'static, Arc<BuilderEvent<Types>>>;
    async fn get_event_stream(&self) -> Self::EventStream {
        let recv_channel = self.to_subscribe_clone_recv.clone();
        stream::unfold(recv_channel, move |mut recv_channel| async move {
            let event_res = recv_channel.recv().await;
            if event_res.is_err() {
                return None;
            }
            Some((event_res.unwrap(), recv_channel))
        })
        .boxed()
    }
}

impl<Types: NodeType> EventsStreamer<Types> {
    pub fn new() -> Self {
        let (mut subscriber_send_channel, to_subscribe_clone_recv) =
            broadcast::<Arc<BuilderEvent<Types>>>(RETAINED_EVENTS_COUNT);
        subscriber_send_channel.set_overflow(true);
        EventsStreamer {
            subscriber_send_channel,
            to_subscribe_clone_recv,
        }
    }
}

impl<Types: NodeType> Default for EventsStreamer<Types> {
    fn default() -> Self {
        Self::new()
    }
}
