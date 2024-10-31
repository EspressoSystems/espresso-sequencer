use std::{future::Future, pin::Pin};

use std::{
    collections::HashSet,
    hash::Hash,
    mem,
    time::{Duration, Instant},
};

use anyhow::Context;
use async_compatibility_layer::art::async_sleep;
use async_std::prelude::FutureExt;
use either::Either::{self, Left, Right};
use futures::stream::unfold;
use futures::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_events_service::events::Error as EventStreamError;
use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::client::HealthStatus;
use surf_disco::Client;
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

        async {
            loop {
                match client.healthcheck::<HealthStatus>().await {
                    Ok(_) => break,
                    Err(err) => {
                        tracing::debug!(?err, "Healthcheck failed, retrying");
                    }
                }
                async_sleep(Self::RETRY_PERIOD).await;
            }
        }
        .timeout(Self::CONNECTION_TIMEOUT)
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
        };

        let stream = unfold(this, |mut this| async move {
            loop {
                match &mut this.connection {
                    Left(connection) => match connection.next().await {
                        Some(Ok(event)) => {
                            return Some((event, this));
                        }
                        Some(Err(err)) => {
                            warn!(?err, "Error in event stream");
                            continue;
                        }
                        None => {
                            warn!("Event stream ended, attempting reconnection");
                            let fut = Self::connect_inner(this.api_url.clone());
                            let _ = std::mem::replace(&mut this.connection, Right(Box::pin(fut)));
                            continue;
                        }
                    },
                    Right(reconnection) => match reconnection.await {
                        Ok(connection) => {
                            let _ = std::mem::replace(&mut this.connection, Left(connection));
                            continue;
                        }
                        Err(err) => {
                            error!(?err, "Error while reconnecting, will retry in a while");
                            async_sleep(Self::RETRY_PERIOD).await;
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

    use async_compatibility_layer::art::async_spawn;
    use async_std::prelude::FutureExt;
    #[cfg(async_executor_impl = "async-std")]
    use async_std::task::JoinHandle;
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
    #[cfg(async_executor_impl = "tokio")]
    use tokio::task::JoinHandle;
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

        async_spawn(async move { app.serve(bind_url, MockVersion {}).await.unwrap() })
    }

    #[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn event_stream_wrapper() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();

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

        stream
            .next()
            .timeout(TIMEOUT)
            .await
            .expect("When mock event server is spawned, stream should work")
            .unwrap();

        #[cfg(async_executor_impl = "tokio")]
        app_handle.abort();
        #[cfg(async_executor_impl = "async-std")]
        app_handle.cancel().await;

        stream
            .next()
            .timeout(TIMEOUT)
            .await
            .expect_err("When mock event server is killed, stream should be in reconnecting state and never return");

        let app_handle = run_app("hotshot-events", url.clone());

        stream
            .next()
            .timeout(TIMEOUT)
            .await
            .expect("When mock event server is restarted, stream should work again")
            .unwrap();

        #[cfg(async_executor_impl = "tokio")]
        app_handle.abort();
        #[cfg(async_executor_impl = "async-std")]
        app_handle.cancel().await;

        run_app("wrong-path", url.clone());

        stream
            .next()
            .timeout(TIMEOUT)
            .await
            .expect_err("API is reachable, but is on wrong path");
    }
}
