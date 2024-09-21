use std::{
    collections::HashSet,
    future::Future,
    hash::Hash,
    mem,
    pin::Pin,
    task::Poll,
    time::{Duration, Instant},
};

use committable::Commitment;
use either::Either::{self, Left, Right};
use futures::{FutureExt, Stream, StreamExt};
use hotshot::types::Event;
use hotshot_events_service::events::Error as EventStreamError;
use hotshot_types::{
    data::Leaf, traits::node_implementation::NodeType, utils::BuilderCommitment, vid::VidCommitment,
};
use surf_disco::Client;
use tracing::error;
use url::Url;
use vbs::version::StaticVersionType;

/// A set that allows for time-based garbage collection,
/// implemented as three sets that are periodically shifted right.
/// Garage collection is triggered by calling [`Self::rotate`].
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BlockId<TYPES: NodeType> {
    pub hash: BuilderCommitment,
    pub view: TYPES::Time,
}

impl<TYPES: NodeType> std::fmt::Display for BlockId<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block({}@{})",
            hex::encode(self.hash.as_ref()),
            *self.view
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BuilderStateId<TYPES: NodeType> {
    pub parent_view: TYPES::Time,
    pub parent_commitment: VidCommitment,
}

impl<TYPES: NodeType> std::fmt::Display for BuilderStateId<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BuilderState({}@{})",
            self.parent_commitment, *self.parent_view
        )
    }
}

/// References to the parent block that is extended to spawn the new builder state.
#[derive(Debug, Clone)]
pub struct ParentBlockReferences<TYPES: NodeType> {
    pub view_number: TYPES::Time,
    pub vid_commitment: VidCommitment,
    pub leaf_commit: Commitment<Leaf<TYPES>>,
    pub builder_commitment: BuilderCommitment,
}

// implement display for the derived info
impl<TYPES: NodeType> std::fmt::Display for ParentBlockReferences<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View Number: {:?}", self.view_number)
    }
}

type EventServiceConnection<TYPES, V> = surf_disco::socket::Connection<
    Event<TYPES>,
    surf_disco::socket::Unsupported,
    EventStreamError,
    V,
>;

type EventServiceReconnect<TYPES, V> =
    Pin<Box<dyn Future<Output = anyhow::Result<EventServiceConnection<TYPES, V>>> + Send + Sync>>;

/// A wrapper around event streaming API that provides auto-reconnection capability
pub struct EventServiceStream<TYPES: NodeType, V: StaticVersionType> {
    api_url: Url,
    connection: Either<EventServiceConnection<TYPES, V>, EventServiceReconnect<TYPES, V>>,
}

impl<TYPES: NodeType, V: StaticVersionType> EventServiceStream<TYPES, V> {
    async fn connect_inner(
        url: Url,
    ) -> anyhow::Result<
        surf_disco::socket::Connection<
            Event<TYPES>,
            surf_disco::socket::Unsupported,
            EventStreamError,
            V,
        >,
    > {
        let client = Client::<hotshot_events_service::events::Error, V>::new(url.clone());

        if !(client.connect(None).await) {
            anyhow::bail!("Couldn't connect to API url");
        }

        tracing::info!("Builder client connected to the hotshot events api");

        Ok(client
            .socket("hotshot-events/events")
            .subscribe::<Event<TYPES>>()
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

impl<TYPES: NodeType, V: StaticVersionType + 'static> Stream for EventServiceStream<TYPES, V> {
    type Item = Event<TYPES>;

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

// TODO use new commitment
pub trait LegacyCommit<T: NodeType> {
    fn legacy_commit(&self) -> committable::Commitment<hotshot_types::data::Leaf<T>>;
}

impl<T: NodeType> LegacyCommit<T> for hotshot_types::data::Leaf<T> {
    fn legacy_commit(&self) -> committable::Commitment<hotshot_types::data::Leaf<T>> {
        <hotshot_types::data::Leaf<T> as committable::Committable>::commit(self)
    }
}
