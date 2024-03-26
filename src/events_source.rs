use crate::notifier::Notifier;
use async_std::sync::RwLock;
use async_trait::async_trait;
use futures::stream::{self, BoxStream, Stream, StreamExt};
use hotshot_types::event::{Event, EventType};
use hotshot_types::traits::node_implementation::ConsensusTime;
use hotshot_types::traits::node_implementation::NodeType;
use std::future::IntoFuture;
use std::sync::Arc;

#[async_trait]
pub trait EventsSource<Types>
where
    Types: NodeType,
{
    type EventStream: Stream<Item = Event<Types>> + Unpin + Send + 'static;
    async fn get_events_starting_from_view(&self, view_number: u64) -> Self::EventStream;

    async fn subscribe_events(&self, view_number: u64) -> BoxStream<'static, Event<Types>> {
        self.get_events_starting_from_view(view_number)
            .await
            .boxed()
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
    notifier: Arc<RwLock<Notifier<Event<Types>>>>,
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
            self.notifier.write().await.notify(event);
        }
    }
}

#[async_trait]
impl<Types: NodeType> EventsSource<Types> for EventsStreamer<Types> {
    type EventStream = BoxStream<'static, Event<Types>>;
    async fn get_events_starting_from_view(&self, view_number: u64) -> Self::EventStream {
        let notifier = self.notifier.clone();
        stream::unfold(notifier, move |notifier| async move {
            let future = notifier
                .read()
                .await
                .wait_for(move |event| event.view_number.get_u64() >= view_number)
                .await
                .into_future();
            let event = future.await.unwrap();
            Some((event, notifier))
        })
        .boxed()
    }
}

impl<Types: NodeType> EventsStreamer<Types> {
    pub fn new() -> Self {
        EventsStreamer {
            notifier: Arc::new(RwLock::new(Notifier::new())),
        }
    }
}

impl<Types: NodeType> Default for EventsStreamer<Types> {
    fn default() -> Self {
        Self::new()
    }
}
