use std::{future::Future, pin::Pin, task::Poll};

use std::{
    collections::HashSet,
    hash::Hash,
    mem,
    time::{Duration, Instant},
};

use either::Either::{self, Left, Right};
use futures::{FutureExt, Stream, StreamExt};
use hotshot::types::Event;
use hotshot_events_service::events::Error as EventStreamError;
use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::Client;
use tracing::error;
use url::Url;
use vbs::version::StaticVersionType;

/// A set that allows for time-based garbage collection,
/// implemented as three sets that are periodically shifted right.
/// Garbage collection is triggered by calling [`Self::rotate`].
#[derive(Clone, Debug)]
pub struct RotatingSet<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    fresh: HashSet<T>,
    stale: HashSet<T>,
    expiring: HashSet<T>,
    last_rotation: Instant,
    period: Duration,
}

impl<T> RotatingSet<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    /// Construct a new `RotatingSet`
    pub fn new(period: Duration) -> Self {
        Self {
            fresh: HashSet::new(),
            stale: HashSet::new(),
            expiring: HashSet::new(),
            last_rotation: Instant::now(),
            period,
        }
    }

    /// Returns `true` if the key is contained in the set
    pub fn contains(&self, key: &T) -> bool {
        self.fresh.contains(key) || self.stale.contains(key) || self.expiring.contains(key)
    }

    /// Insert a `key` into the set. Doesn't trigger garbage collection
    pub fn insert(&mut self, value: T) {
        self.fresh.insert(value);
    }

    /// Force garbage collection, even if the time elapsed since
    ///  the last garbage collection is less than `self.period`
    pub fn force_rotate(&mut self) {
        let now_stale = mem::take(&mut self.fresh);
        let now_expiring = mem::replace(&mut self.stale, now_stale);
        self.expiring = now_expiring;
        self.last_rotation = Instant::now();
    }

    /// Trigger garbage collection.
    pub fn rotate(&mut self) -> bool {
        if self.last_rotation.elapsed() > self.period {
            self.force_rotate();
            true
        } else {
            false
        }
    }
}

impl<T> Extend<T> for RotatingSet<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.fresh.extend(iter)
    }
}

type EventServiceConnection<Types, ApiVer> = surf_disco::socket::Connection<
    Event<Types>,
    surf_disco::socket::Unsupported,
    EventStreamError,
    ApiVer,
>;

type EventServiceReconnect<Types, ApiVer> = Pin<
    Box<dyn Future<Output = anyhow::Result<EventServiceConnection<Types, ApiVer>>> + Send + Sync>,
>;

/// A wrapper around event streaming API that provides auto-reconnection capability
pub struct EventServiceStream<Types: NodeType, V: StaticVersionType> {
    api_url: Url,
    connection: Either<EventServiceConnection<Types, V>, EventServiceReconnect<Types, V>>,
}

impl<Types: NodeType, ApiVer: StaticVersionType> EventServiceStream<Types, ApiVer> {
    async fn connect_inner(
        url: Url,
    ) -> anyhow::Result<
        surf_disco::socket::Connection<
            Event<Types>,
            surf_disco::socket::Unsupported,
            EventStreamError,
            ApiVer,
        >,
    > {
        let client = Client::<hotshot_events_service::events::Error, ApiVer>::new(url.clone());

        if !(client.connect(None).await) {
            anyhow::bail!("Couldn't connect to API url");
        }

        tracing::info!("Builder client connected to the hotshot events api");

        Ok(client
            .socket("hotshot-events/events")
            .subscribe::<Event<Types>>()
            .await?)
    }

    /// Establish initial connection to the events service at `api_url`
    pub async fn connect(api_url: Url) -> anyhow::Result<Self> {
        let connection = Self::connect_inner(api_url.clone()).await?;

        Ok(Self {
            api_url,
            connection: Left(connection),
        })
    }
}

impl<Types: NodeType, V: StaticVersionType + 'static> Stream for EventServiceStream<Types, V> {
    type Item = Event<Types>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match &mut self.connection {
            Left(connection) => {
                let Poll::Ready(next) = connection.poll_next_unpin(cx) else {
                    return Poll::Pending;
                };
                match next {
                    Some(Ok(event)) => Poll::Ready(Some(event)),
                    Some(Err(err)) => {
                        error!("Error in the event stream: {err:?}");
                        Poll::Pending
                    }
                    None => {
                        let fut = Self::connect_inner(self.api_url.clone());
                        let _ = std::mem::replace(&mut self.connection, Right(Box::pin(fut)));
                        Poll::Pending
                    }
                }
            }
            Right(reconnect_future) => {
                let Poll::Ready(ready) = reconnect_future.poll_unpin(cx) else {
                    return Poll::Pending;
                };
                match ready {
                    Ok(connection) => {
                        let _ = std::mem::replace(&mut self.connection, Left(connection));
                        Poll::Pending
                    }
                    Err(err) => {
                        error!("Failed to reconnect to the event service: {err:?}");
                        Poll::Ready(None)
                    }
                }
            }
        }
    }
}
