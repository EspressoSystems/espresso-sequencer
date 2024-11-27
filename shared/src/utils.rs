use std::{future::Future, pin::Pin};

use std::{
    collections::HashSet,
    hash::Hash,
    mem,
    time::{Duration, Instant},
};

use anyhow::Context;
use either::Either::{self, Left, Right};
use futures::stream::unfold;
use futures::{Stream, StreamExt};
use hotshot::traits::BlockPayload;
use hotshot::types::Event;
use hotshot_events_service::events::Error as EventStreamError;
use hotshot_types::data::{DaProposal, QuorumProposal};
use hotshot_types::traits::block_contents::BlockHeader;
use hotshot_types::traits::node_implementation::NodeType;
use hotshot_types::utils::BuilderCommitment;
use surf_disco::client::HealthStatus;
use surf_disco::Client;
use tokio::time::{sleep, timeout};
use tracing::{error, warn};
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
    pub last_rotation: Instant,
    pub period: Duration,
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
    last_event_time: Instant, // a field to track the time of the last received event
}

impl<Types: NodeType, ApiVer: StaticVersionType + 'static> EventServiceStream<Types, ApiVer> {
    const RETRY_PERIOD: Duration = Duration::from_secs(1);
    const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);

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

        timeout(Self::CONNECTION_TIMEOUT, async {
            loop {
                match client.healthcheck::<HealthStatus>().await {
                    Ok(_) => break,
                    Err(err) => {
                        tracing::debug!(?err, "Healthcheck failed, retrying");
                    }
                }
                sleep(Self::RETRY_PERIOD).await;
            }
        })
        .await
        .context("Couldn't connect to hotshot events API")?;

        tracing::info!("Builder client connected to the hotshot events API");

        Ok(client
            .socket("hotshot-events/events")
            .subscribe::<Event<Types>>()
            .await?)
    }

    /// Establish initial connection to the events service at `api_url`
    pub async fn connect(api_url: Url) -> anyhow::Result<impl Stream<Item = Event<Types>> + Unpin> {
        let connection = Self::connect_inner(api_url.clone()).await?;

        let this = Self {
            api_url,
            connection: Left(connection),
            last_event_time: Instant::now(), // Initialize with the current time
        };

        let stream = unfold(this, |mut this| async move {
            loop {
                match &mut this.connection {
                    Left(connection) => {
                        match tokio::time::timeout(Self::RETRY_PERIOD, connection.next()).await {
                            Ok(Some(Ok(event))) => {
                                // Update the last_event_time on receiving an event
                                this.last_event_time = Instant::now();
                                return Some((event, this));
                            }
                            Ok(Some(Err(err))) => {
                                warn!(?err, "Error in event stream");
                                continue;
                            }
                            Ok(None) => {
                                warn!("Event stream ended, attempting reconnection");
                                let fut = Self::connect_inner(this.api_url.clone());
                                let _ =
                                    std::mem::replace(&mut this.connection, Right(Box::pin(fut)));
                                continue;
                            }
                            Err(_) => {
                                // Timeout occurred, reconnect
                                warn!("Timeout waiting for next event; reconnecting");
                                let fut = Self::connect_inner(this.api_url.clone());
                                let _ =
                                    std::mem::replace(&mut this.connection, Right(Box::pin(fut)));
                                continue;
                            }
                        }
                    }
                    Right(reconnection) => match reconnection.await {
                        Ok(connection) => {
                            let _ = std::mem::replace(&mut this.connection, Left(connection));
                            continue;
                        }
                        Err(err) => {
                            error!(?err, "Error while reconnecting, will retry in a while");
                            sleep(Self::RETRY_PERIOD).await;
                            let fut = Self::connect_inner(this.api_url.clone());
                            let _ = std::mem::replace(&mut this.connection, Right(Box::pin(fut)));
                            continue;
                        }
                    },
                }
            }
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc,
        },
        time::Duration,
    };

    use async_trait::async_trait;
    use futures::{future::BoxFuture, stream, StreamExt};
    use hotshot::types::{Event, EventType};
    use hotshot_events_service::{
        events::define_api,
        events_source::{EventFilterSet, EventsSource, StartupInfo},
    };
    use hotshot_example_types::node_types::TestTypes;
    use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
    use tide_disco::{method::ReadState, App};
    use tokio::{spawn, task::JoinHandle, time::timeout};
    use url::Url;
    use vbs::version::StaticVersion;

    use crate::utils::EventServiceStream;

    type MockVersion = StaticVersion<0, 1>;

    struct MockEventsSource {
        counter: AtomicU64,
    }

    #[async_trait]
    impl EventsSource<TestTypes> for MockEventsSource {
        type EventStream = futures::stream::Iter<std::vec::IntoIter<Arc<Event<TestTypes>>>>;

        async fn get_event_stream(
            &self,
            _filter: Option<EventFilterSet<TestTypes>>,
        ) -> Self::EventStream {
            let view = ViewNumber::new(self.counter.load(Ordering::SeqCst));
            let test_event = Arc::new(Event {
                view_number: view,
                event: EventType::ViewFinished { view_number: view },
            });
            self.counter.fetch_add(1, Ordering::SeqCst);
            stream::iter(vec![test_event])
        }

        async fn get_startup_info(&self) -> StartupInfo<TestTypes> {
            StartupInfo {
                known_node_with_stake: Vec::new(),
                non_staked_node_count: 0,
            }
        }
    }

    #[async_trait]
    impl ReadState for MockEventsSource {
        type State = Self;

        async fn read<T>(
            &self,
            op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
        ) -> T {
            op(self).await
        }
    }

    fn run_app(path: &'static str, bind_url: Url) -> JoinHandle<()> {
        let source = MockEventsSource {
            counter: AtomicU64::new(0),
        };
        let api = define_api::<MockEventsSource, _, MockVersion>(&Default::default()).unwrap();

        let mut app: App<MockEventsSource, hotshot_events_service::events::Error> =
            App::with_state(source);

        app.register_module(path, api).unwrap();

        spawn(async move { app.serve(bind_url, MockVersion {}).await.unwrap() })
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn event_stream_wrapper() {
        const TIMEOUT: Duration = Duration::from_secs(3);

        let url: Url = format!(
            "http://localhost:{}",
            portpicker::pick_unused_port().unwrap()
        )
        .parse()
        .unwrap();

        let app_handle = run_app("hotshot-events", url.clone());

        let mut stream = EventServiceStream::<TestTypes, MockVersion>::connect(url.clone())
            .await
            .unwrap();

        timeout(TIMEOUT, stream.next())
            .await
            .expect("When mock event server is spawned, stream should work")
            .unwrap();

        app_handle.abort();

        timeout(TIMEOUT, stream.next())
            .await
            .expect_err("When mock event server is killed, stream should be in reconnecting state and never return");

        let app_handle = run_app("hotshot-events", url.clone());

        timeout(TIMEOUT, stream.next())
            .await
            .expect("When mock event server is restarted, stream should work again")
            .unwrap();

        app_handle.abort();

        run_app("wrong-path", url.clone());

        timeout(TIMEOUT, stream.next())
            .await
            .expect_err("API is reachable, but is on wrong path");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn event_stream_wrapper_with_idle_timeout() {
        const TIMEOUT: Duration = Duration::from_secs(3);

        let url: Url = format!(
            "http://localhost:{}",
            portpicker::pick_unused_port().unwrap()
        )
        .parse()
        .unwrap();

        let app_handle = run_app("hotshot-events", url.clone());

        let mut stream = EventServiceStream::<TestTypes, MockVersion>::connect(url.clone())
            .await
            .unwrap();

        // The stream should work when the server is running
        timeout(TIMEOUT, stream.next())
            .await
            .expect("When mock event server is spawned, stream should work")
            .unwrap();

        // Simulate idle timeout by stopping the server and waiting
        app_handle.abort();
        tokio::time::sleep(
            EventServiceStream::<TestTypes, MockVersion>::RETRY_PERIOD + Duration::from_millis(500),
        )
        .await; // Wait longer than idle timeout

        // Stream should reconnect after idle timeout
        let new_app_handle = run_app("hotshot-events", url.clone());

        timeout(TIMEOUT, stream.next())
            .await
            .expect("After idle timeout, stream should reconnect when the server restarts")
            .unwrap();

        // Cleanup
        new_app_handle.abort();
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProposalId<Types>
where
    Types: NodeType,
{
    view_number: Types::View,
    builder_commitment: BuilderCommitment,
}

impl<Types> ProposalId<Types>
where
    Types: NodeType,
{
    pub fn from_quorum_proposal(proposal: &QuorumProposal<Types>) -> Self {
        Self {
            builder_commitment: proposal.block_header.builder_commitment(),
            view_number: proposal.view_number,
        }
    }

    pub fn from_da_proposal(proposal: &DaProposal<Types>) -> Self {
        let builder_commitment = <Types::BlockPayload as BlockPayload<Types>>::from_bytes(
            &proposal.encoded_transactions,
            &proposal.metadata,
        )
        .builder_commitment(&proposal.metadata);

        Self {
            builder_commitment,
            view_number: proposal.view_number,
        }
    }

    pub fn view_number(&self) -> <Types as NodeType>::View {
        self.view_number
    }

    pub fn builder_commitment(&self) -> &BuilderCommitment {
        &self.builder_commitment
    }
}
