use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use async_trait::async_trait;
use futures::stream::{self, BoxStream, Stream, StreamExt};
use hotshot_types::event::{Event, EventType};
use hotshot_types::traits::node_implementation::NodeType;
use std::sync::Arc;

const RETAINED_EVENTS_COUNT: usize = 4096;
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
    async fn handle_event(&mut self, event: &Event<Types>);
}

#[derive(Debug)]
pub struct EventsStreamer<Types: NodeType> {
    to_subscribe_clone_recv: BroadcastReceiver<Arc<Event<Types>>>,
    subscriber_send_channel: BroadcastSender<Arc<Event<Types>>>,
}

#[async_trait]
impl<Types: NodeType> EventConsumer<Types> for EventsStreamer<Types> {
    async fn handle_event(&mut self, event: &Event<Types>) {
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
            let event = Arc::new(event.clone());
            let _status = self.subscriber_send_channel.broadcast(event).await;
        }
    }
}

#[async_trait]
impl<Types: NodeType> EventsSource<Types> for EventsStreamer<Types> {
    type EventStream = BoxStream<'static, Arc<Event<Types>>>;
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
            broadcast::<Arc<Event<Types>>>(RETAINED_EVENTS_COUNT);
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
