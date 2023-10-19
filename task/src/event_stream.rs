use async_compatibility_layer::channel::{unbounded, UnboundedSender, UnboundedStream};
use async_lock::RwLock;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use async_trait::async_trait;
use futures::Stream;

use crate::task::{FilterEvent, PassType};

/// a stream that does nothing.
/// it's immediately closed
#[derive(Clone)]
pub struct DummyStream;

impl Stream for DummyStream {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

#[async_trait]
impl EventStream for DummyStream {
    type EventType = ();

    type StreamType = DummyStream;

    async fn publish(&self, _event: Self::EventType) {}

    async fn subscribe(
        &self,
        _filter: FilterEvent<Self::EventType>,
    ) -> (Self::StreamType, StreamId) {
        (DummyStream, 0)
    }

    async fn unsubscribe(&self, _id: StreamId) {}

    async fn direct_message(&self, _id: StreamId, _event: Self::EventType) {}
}

impl SendableStream for DummyStream {}

/// this is only used for indexing
pub type StreamId = usize;

/// a stream that plays nicely with async
pub trait SendableStream: Stream + Sync + Send + 'static {}

/// Async pub sub event stream
/// NOTE: static bound indicates that if the type points to data, that data lives for the lifetime
/// of the program
#[async_trait]
pub trait EventStream: Clone + 'static + Sync + Send {
    /// the type of event to process
    type EventType: PassType;
    /// the type of stream to use
    type StreamType: SendableStream<Item = Self::EventType>;

    /// publish an event to the event stream
    async fn publish(&self, event: Self::EventType);

    /// subscribe to a particular set of events
    /// specified by `filter`. Filter returns true if the event should be propagated
    /// TODO (justin) rethink API, we might be able just to use `StreamExt::filter` and `Filter`
    /// That would certainly be cleaner
    async fn subscribe(&self, filter: FilterEvent<Self::EventType>)
        -> (Self::StreamType, StreamId);

    /// unsubscribe from the stream
    async fn unsubscribe(&self, id: StreamId);

    /// send direct message to node
    async fn direct_message(&self, id: StreamId, event: Self::EventType);
}

/// Event stream implementation using channels as the underlying primitive.
/// We want it to be cloneable
#[derive(Clone)]
pub struct ChannelStream<EVENT: PassType> {
    /// inner field. Useful for having the stream itself
    /// be clone
    inner: Arc<RwLock<ChannelStreamInner<EVENT>>>,
}

/// trick to make the event stream clonable
struct ChannelStreamInner<EVENT: PassType> {
    /// the subscribers to the channel
    subscribers: HashMap<StreamId, (FilterEvent<EVENT>, UnboundedSender<EVENT>)>,
    /// the next unused assignable id
    next_stream_id: StreamId,
}

impl<EVENT: PassType> ChannelStream<EVENT> {
    /// construct a new event stream
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ChannelStreamInner {
                subscribers: HashMap::new(),
                next_stream_id: 0,
            })),
        }
    }
}

impl<EVENT: PassType> Default for ChannelStream<EVENT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<EVENT: PassType> SendableStream for UnboundedStream<EVENT> {}

#[async_trait]
impl<EVENT: PassType + 'static> EventStream for ChannelStream<EVENT> {
    type EventType = EVENT;
    type StreamType = UnboundedStream<Self::EventType>;

    async fn direct_message(&self, id: StreamId, event: Self::EventType) {
        let inner = self.inner.write().await;
        match inner.subscribers.get(&id) {
            Some((filter, sender)) => {
                if filter(&event) {
                    match sender.send(event.clone()).await {
                        Ok(()) => (),
                        // error sending => stream is closed so remove it
                        Err(_) => self.unsubscribe(id).await,
                    }
                }
            }
            None => {
                tracing::debug!("Requested stream id not found");
            }
        }
    }

    /// publish an event to the event stream
    async fn publish(&self, event: Self::EventType) {
        let inner = self.inner.read().await;
        for (uid, (filter, sender)) in &inner.subscribers {
            if filter(&event) {
                match sender.send(event.clone()).await {
                    Ok(()) => (),
                    // error sending => stream is closed so remove it
                    Err(_) => {
                        self.unsubscribe(*uid).await;
                    }
                }
            }
        }
    }

    async fn subscribe(
        &self,
        filter: FilterEvent<Self::EventType>,
    ) -> (Self::StreamType, StreamId) {
        let mut inner = self.inner.write().await;
        let new_stream_id = inner.next_stream_id;
        let (s, r) = unbounded();
        inner.next_stream_id += 1;
        // NOTE: can never be already existing.
        // so, this should always return `None`
        inner.subscribers.insert(new_stream_id, (filter, s));
        (r.into_stream(), new_stream_id)
    }

    async fn unsubscribe(&self, uid: StreamId) {
        let mut inner = self.inner.write().await;
        inner.subscribers.remove(&uid);
    }
}

#[cfg(test)]
pub mod test {
    use crate::{event_stream::EventStream, StreamExt};
    use async_compatibility_layer::art::{async_sleep, async_spawn};
    use std::time::Duration;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum TestMessage {
        One,
        Two,
        Three,
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn test_channel_stream_basic() {
        use crate::task::FilterEvent;

        use super::ChannelStream;

        let channel_stream = ChannelStream::<TestMessage>::new();
        let (mut stream, _) = channel_stream.subscribe(FilterEvent::default()).await;
        let dup_channel_stream = channel_stream.clone();

        let dup_dup_channel_stream = channel_stream.clone();

        async_spawn(async move {
            let (mut stream, _) = dup_channel_stream.subscribe(FilterEvent::default()).await;
            assert!(stream.next().await.unwrap() == TestMessage::Three);
            assert!(stream.next().await.unwrap() == TestMessage::One);
            assert!(stream.next().await.unwrap() == TestMessage::Two);
        });

        async_spawn(async move {
            dup_dup_channel_stream.publish(TestMessage::Three).await;
            dup_dup_channel_stream.publish(TestMessage::One).await;
            dup_dup_channel_stream.publish(TestMessage::Two).await;
        });
        async_sleep(Duration::new(3, 0)).await;

        assert!(stream.next().await.unwrap() == TestMessage::Three);
        assert!(stream.next().await.unwrap() == TestMessage::One);
        assert!(stream.next().await.unwrap() == TestMessage::Two);
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn test_channel_stream_xtreme() {
        use crate::task::FilterEvent;

        use super::ChannelStream;

        let channel_stream = ChannelStream::<TestMessage>::new();
        let mut streams = Vec::new();

        for _i in 0..1000 {
            let dup_channel_stream = channel_stream.clone();
            let (stream, _) = dup_channel_stream.subscribe(FilterEvent::default()).await;
            streams.push(stream);
        }

        let dup_dup_channel_stream = channel_stream.clone();

        for _i in 0..1000 {
            let mut stream = streams.pop().unwrap();
            async_spawn(async move {
                for event in [TestMessage::One, TestMessage::Two, TestMessage::Three] {
                    for _ in 0..100 {
                        assert!(stream.next().await.unwrap() == event);
                    }
                }
            });
        }

        async_spawn(async move {
            for event in [TestMessage::One, TestMessage::Two, TestMessage::Three] {
                for _ in 0..100 {
                    dup_dup_channel_stream.publish(event.clone()).await;
                }
            }
        });
    }
}
