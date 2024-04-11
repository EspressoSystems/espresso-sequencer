//! Adversarial Espresso client.
//!
//! This program runs a randomized client for the Espresso query service. It is designed to hit
//! methods and usage patterns that a normal client, such as a rollup, would not do, so that we can
//! discover edge cases in the performance of the query service.
//!
//! The client works by repeatedly generating random "actions" to execute. Actions include both
//! one-off queries and maintenance of long-lived connections. The client may keep many connections
//! open at one time, which has been a source of performance problems for the server in the past.
//!
//! The program also runs a web server to provide some visibility into its state. The web server
//! provides a healthcheck endpoint as well as a prometheus endpoint which provides metrics like the
//! count of various types of actions performed and the number of open streams.

use anyhow::{ensure, Context};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{
    sync::RwLock,
    task::{sleep, spawn},
};
use clap::Parser;
use commit::Committable;
use derivative::Derivative;
use es_version::{SequencerVersion, SEQUENCER_VERSION};
use futures::{
    future::FutureExt,
    stream::{Peekable, StreamExt},
};
use hotshot_query_service::{
    availability::{BlockQueryData, LeafQueryData, PayloadQueryData, VidCommonQueryData},
    metrics::PrometheusMetrics,
    node::TimeWindowQueryData,
};
use hotshot_types::{
    traits::metrics::{Counter, Gauge, Metrics as _},
    vid::{vid_scheme, VidSchemeType},
};
use jf_primitives::vid::VidScheme;
use rand::{seq::SliceRandom, RngCore};
use sequencer::{api::endpoints::NamespaceProofQueryData, Header, SeqTypes};
use serde::de::DeserializeOwned;
use std::time::Duration;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    pin::Pin,
    sync::Arc,
};
use strum::{EnumDiscriminants, VariantArray};
use surf_disco::{error::ClientError, socket, Url};
use tide_disco::{error::ServerError, App};
use time::OffsetDateTime;
use toml::toml;

/// An adversarial stress test for sequencer APIs.
#[derive(Clone, Debug, Parser)]
struct Options {
    /// The maximum number of open WebSockets connections for each resource type at any time.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_MAX_OPEN_STREAMS",
        default_value = "100"
    )]
    max_open_streams: usize,

    /// The maximum number of consecutive blocking polls to make at one time.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_MAX_BLOCKING_POLLS",
        default_value = "10"
    )]
    max_blocking_polls: u8,

    /// Timeout for HTTP requests.
    ///
    /// Requests that take longer than this will fail, causing an error log and an increment of the
    /// `failed_actions` metric.
    #[clap(
        long,
        env = "ESPRESS_NASTY_CLIENT_HTTP_TIMEOUT",
        default_value = "30s",
        value_parser = sequencer::options::parse_duration
    )]
    http_timeout: Duration,

    #[clap(flatten)]
    distribution: ActionDistribution,

    /// Port on which to serve the nasty-client API.
    #[clap(
        short,
        long,
        env = "ESPRESSO_NASTY_CLIENT_PORT",
        default_value = "8080"
    )]
    port: u16,

    /// The URL of the query service to connect to.
    #[clap(env = "ESPRESSO_SEQUENCER_URL")]
    url: Url,
}

#[derive(Clone, Debug, Parser)]
struct ActionDistribution {
    /// The weight of query actions in the random distribution.
    #[clap(long, env = "ESPRESSO_NASTY_CLIENT_WEIGHT_QUERY", default_value = "5")]
    weight_query: u8,

    /// The weight of "open stream" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_OPEN_STREAM",
        default_value = "2"
    )]
    weight_open_stream: u8,

    /// The weight of "close stream" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_CLOSE_STREAM",
        default_value = "1"
    )]
    weight_close_stream: u8,

    /// The weight of "poll stream" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_POLL_STREAM",
        default_value = "5"
    )]
    weight_poll_stream: u8,

    /// The weight of "query window" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_QUERY_WINDOW",
        default_value = "3"
    )]
    weight_query_window: u8,

    /// The weight of "query namespace" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_QUERY_NAMESPACE",
        default_value = "3"
    )]
    weight_query_namespace: u8,
}

impl ActionDistribution {
    fn weight(&self, action: ActionDiscriminants) -> u8 {
        match action {
            ActionDiscriminants::Query => self.weight_query,
            ActionDiscriminants::OpenStream => self.weight_open_stream,
            ActionDiscriminants::CloseStream => self.weight_close_stream,
            ActionDiscriminants::PollStream => self.weight_poll_stream,
            ActionDiscriminants::QueryWindow => self.weight_query_window,
            ActionDiscriminants::QueryNamespace => self.weight_query_namespace,
        }
    }
}

#[derive(Debug)]
struct Metrics {
    open_streams: HashMap<Resource, Box<dyn Gauge>>,
    query_actions: HashMap<Resource, Box<dyn Counter>>,
    open_stream_actions: HashMap<Resource, Box<dyn Counter>>,
    close_stream_actions: HashMap<Resource, Box<dyn Counter>>,
    poll_stream_actions: HashMap<Resource, Box<dyn Counter>>,
    query_window_actions: Box<dyn Counter>,
    query_namespace_actions: Box<dyn Counter>,
}

impl Metrics {
    fn new(registry: &PrometheusMetrics) -> Self {
        Self {
            open_streams: Resource::VARIANTS
                .iter()
                .map(|resource| {
                    (
                        *resource,
                        registry
                            .create_gauge(format!("open_{}_streams", resource.singular()), None),
                    )
                })
                .collect(),

            query_actions: Resource::VARIANTS
                .iter()
                .map(|resource| {
                    (
                        *resource,
                        registry
                            .create_counter(format!("{}_query_actions", resource.singular()), None),
                    )
                })
                .collect(),
            open_stream_actions: Resource::VARIANTS
                .iter()
                .map(|resource| {
                    (
                        *resource,
                        registry.create_counter(
                            format!("{}_open_stream_actions", resource.singular()),
                            None,
                        ),
                    )
                })
                .collect(),
            close_stream_actions: Resource::VARIANTS
                .iter()
                .map(|resource| {
                    (
                        *resource,
                        registry.create_counter(
                            format!("{}_close_stream_actions", resource.singular()),
                            None,
                        ),
                    )
                })
                .collect(),
            poll_stream_actions: Resource::VARIANTS
                .iter()
                .map(|resource| {
                    (
                        *resource,
                        registry.create_counter(
                            format!("{}_poll_stream_actions", resource.singular()),
                            None,
                        ),
                    )
                })
                .collect(),

            query_window_actions: registry.create_counter("query_window_actions".into(), None),
            query_namespace_actions: registry
                .create_counter("query_namespace_actions".into(), None),
        }
    }
}

trait Queryable: DeserializeOwned + Debug + Eq {
    const RESOURCE: Resource;

    /// URL segment used to indicate that we want to fetch this resource by block hash.
    const HASH_URL_SEGMENT: &'static str;

    fn hash(&self) -> String;
}

impl Queryable for BlockQueryData<SeqTypes> {
    const RESOURCE: Resource = Resource::Blocks;
    const HASH_URL_SEGMENT: &'static str = "hash";

    fn hash(&self) -> String {
        self.hash().to_string()
    }
}

impl Queryable for LeafQueryData<SeqTypes> {
    const RESOURCE: Resource = Resource::Leaves;
    const HASH_URL_SEGMENT: &'static str = "hash";

    fn hash(&self) -> String {
        self.hash().to_string()
    }
}

impl Queryable for Header {
    const RESOURCE: Resource = Resource::Headers;
    const HASH_URL_SEGMENT: &'static str = "hash";

    fn hash(&self) -> String {
        self.commit().to_string()
    }
}

impl Queryable for PayloadQueryData<SeqTypes> {
    const RESOURCE: Resource = Resource::Payloads;
    const HASH_URL_SEGMENT: &'static str = "block-hash";

    fn hash(&self) -> String {
        self.block_hash().to_string()
    }
}

type Connection<T> = socket::Connection<T, socket::Unsupported, ClientError, SequencerVersion>;

#[derive(Derivative)]
#[derivative(Debug)]
struct Subscription<T: Queryable> {
    #[derivative(Debug = "ignore")]
    stream: Pin<Box<Peekable<Connection<T>>>>,
    position: u64,
}

#[derive(Debug)]
struct ResourceManager<T: Queryable> {
    client: surf_disco::Client<ClientError, SequencerVersion>,
    open_streams: BTreeMap<u64, Subscription<T>>,
    next_stream_id: u64,
    max_open_streams: usize,
    max_blocking_polls: u8,
    metrics: Arc<Metrics>,
}

impl<T: Queryable> ResourceManager<T> {
    fn new(opt: &Options, metrics: Arc<Metrics>) -> Self {
        Self {
            client: surf_disco::Client::builder(opt.url.clone())
                .set_timeout(Some(opt.http_timeout))
                .build(),
            open_streams: BTreeMap::new(),
            next_stream_id: 0,
            max_open_streams: opt.max_open_streams,
            max_blocking_polls: opt.max_blocking_polls,
            metrics,
        }
    }

    fn singular() -> &'static str {
        T::RESOURCE.singular()
    }

    fn plural() -> &'static str {
        T::RESOURCE.plural()
    }

    async fn query(&self, at: u64) -> anyhow::Result<()> {
        let at = self.adjust_index(at).await?;
        tracing::debug!("fetching {} {at}", Self::singular());
        let obj: T = self
            .client
            .get(&format!("availability/{}/{at}", Self::singular()))
            .send()
            .await
            .context(format!("fetching {} {at}", Self::singular()))?;

        // Query by hash and check consistency.
        let hash = obj.hash();
        ensure!(
            obj == self
                .client
                .get(&format!(
                    "availability/{}/{}/{hash}",
                    Self::singular(),
                    T::HASH_URL_SEGMENT,
                ))
                .send()
                .await
                .context(format!("fetching {} {hash}", Self::singular()))?,
            format!(
                "query for {} {at} by hash {hash} is not consistent",
                Self::singular()
            )
        );

        self.metrics.query_actions[&T::RESOURCE].add(1);
        Ok(())
    }

    async fn open_stream(&mut self, from: u64) -> anyhow::Result<()> {
        if self.open_streams.len() >= self.max_open_streams {
            tracing::info!(
                num = self.open_streams.len(),
                "not opening stream, number of open streams exceeds maximum"
            );
            return Ok(());
        }

        let from = self.adjust_index(from).await?;
        let stream = self
            .client
            .socket(&format!("availability/stream/{}/{from}", Self::plural()))
            .subscribe()
            .await
            .context(format!("subscribing to {} from {from}", Self::plural()))?;
        let id = self.next_stream_id;
        self.next_stream_id += 1;
        tracing::info!("opened {} stream {id} at {from}", Self::singular());
        self.open_streams.insert(
            id,
            Subscription {
                stream: Box::pin(stream.peekable()),
                position: from,
            },
        );
        tracing::info!(
            "{} open {} streams",
            self.open_streams.len(),
            Self::singular()
        );

        self.metrics.open_streams[&T::RESOURCE].update(1);
        self.metrics.open_stream_actions[&T::RESOURCE].add(1);
        Ok(())
    }

    async fn close_stream(&mut self, index: usize) {
        if self.open_streams.is_empty() {
            tracing::debug!("not closing {} stream; no open streams", Self::singular());
            return;
        }
        let id = *self
            .open_streams
            .keys()
            .nth(index % self.open_streams.len())
            .unwrap();
        tracing::info!("closing stream {id}");
        self.open_streams.remove(&id);
        tracing::info!(
            "{} open {} streams",
            self.open_streams.len(),
            Self::singular()
        );
        self.metrics.open_streams[&T::RESOURCE].update(-1);
        self.metrics.close_stream_actions[&T::RESOURCE].add(1);
    }

    async fn poll_stream(&mut self, index: usize, amount: u8) -> anyhow::Result<()> {
        if self.open_streams.is_empty() {
            tracing::debug!("not polling {} stream; no open streams", Self::singular());
            return Ok(());
        }
        self.metrics.poll_stream_actions[&T::RESOURCE].add(1);

        let index = index % self.open_streams.len();
        let (id, stream) = self.open_streams.iter_mut().nth(index).unwrap();

        let mut blocking = 0;
        for _ in 0..amount {
            // Check if the next item is immediately available or if we're going to block.
            if stream.stream.as_mut().peek().now_or_never().is_none() {
                blocking += 1;
                if blocking > self.max_blocking_polls {
                    tracing::info!("aborting poll_stream action; exceeded maximum blocking polls");
                    return Ok(());
                }
            }

            let pos = stream.position;
            stream.position += 1;
            tracing::debug!("polling {} stream {id} at position {pos}", Self::singular());
            let Some(res) = stream.stream.next().await else {
                tracing::info!("{} stream {id} ended", Self::singular());
                let id = *id;
                self.open_streams.remove(&id);
                tracing::info!(
                    "{} open {} streams",
                    self.open_streams.len(),
                    Self::singular()
                );
                self.metrics.open_streams[&T::RESOURCE].update(-1);
                return Ok(());
            };
            let obj = res.context(format!("polling {} stream {id} at {pos}", Self::singular()))?;

            // Check consistency against a regular query.
            ensure!(
                obj == self
                    .client
                    .get(&format!("availability/{}/{pos}", Self::singular()))
                    .send()
                    .await
                    .context(format!("fetching {} {pos}", Self::singular()))?,
                format!(
                    "{} stream {id} is not consistent with query at {pos}",
                    Self::singular()
                ),
            );
        }

        Ok(())
    }

    async fn adjust_index(&self, at: u64) -> anyhow::Result<u64> {
        let block_height = loop {
            let block_height: u64 = self
                .client
                .get("status/block-height")
                .send()
                .await
                .context("getting block height")?;
            if block_height == 0 {
                // None of our tests work with an empty history, but if we just wait briefly there
                // should be some blocks produced soon.
                tracing::info!("waiting for block height");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            break block_height;
        };
        Ok(at % block_height)
    }
}

impl ResourceManager<Header> {
    async fn query_window(&self, from: u64, duration: u16) -> anyhow::Result<()> {
        let now = OffsetDateTime::now_utc().unix_timestamp() as u64;
        let start = from % now;
        let end = start + duration as u64;
        tracing::debug!("querying timestamp window from {start} to {end}");
        let window: TimeWindowQueryData<Header> = self
            .client
            .get(&format!("node/header/window/{start}/{end}"))
            .send()
            .await
            .context(format!("fetching timestamp window from {start} to {end}"))?;

        // Sanity check the window: prev and next should be correct bookends.
        if let Some(prev) = &window.prev {
            ensure!(
                prev.timestamp < start,
                format!("prev header {} is later than {start}", prev.height)
            );
        }
        if let Some(next) = &window.next {
            ensure!(
                next.timestamp >= end,
                format!("next header {} is earlier than {end}", next.height)
            );
        }
        // Each header in the window proper should have an appropriate timestamp.
        let mut prev = window.prev;
        for header in window.window {
            ensure!(
                header.timestamp >= start && header.timestamp < end,
                format!(
                    "header {} with timestamp {} is not in window [{start}, {end})",
                    header.height, header.timestamp
                )
            );

            if let Some(prev) = prev {
                ensure!(
                    prev.height + 1 == header.height,
                    format!(
                        "headers in window from {start} to {end} are not consecutive (prev = {}, curr = {})",
                        prev.height,
                        header.height,
                    ),
                );
                ensure!(
                    prev.timestamp <= header.timestamp,
                    format!(
                        "headers in window from {start} to {end} have decreasing timestamps (prev = {}, curr = {})",
                        prev.timestamp,
                        header.timestamp,
                    ),
                );
            }
            prev = Some(header);
        }

        self.metrics.query_window_actions.add(1);
        Ok(())
    }
}

impl ResourceManager<BlockQueryData<SeqTypes>> {
    async fn query_namespace(&self, block: u64, index: usize) -> anyhow::Result<()> {
        let block = self.adjust_index(block).await?;

        // Download the header so we can translate the `namespace` index to a namespace ID using
        // the namespace table.
        let header: Header = self
            .client
            .get(&format!("availability/header/{block}"))
            .send()
            .await
            .context(format!("fetching header {block}"))?;
        if header.ns_table.is_empty() {
            tracing::info!("not fetching namespace because block {block} is empty");
            return Ok(());
        }
        let ns = header.ns_table.get_table_entry(index).0;

        tracing::debug!("querying namespace {ns} in block {block}");
        let ns_proof: NamespaceProofQueryData = self
            .client
            .get(&format!("availability/block/{block}/namespace/{ns}"))
            .send()
            .await
            .context(format!("fetching namespace {block}:{ns}"))?;

        // Verify proof.
        let vid_common: VidCommonQueryData<SeqTypes> = self
            .client
            .get(&format!("availability/vid/common/{block}"))
            .send()
            .await
            .context(format!("fetching VID common {block}"))?;
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(vid_common.common()));
        ensure!(
            ns_proof
                .proof
                .verify(&vid, &header.payload_commitment, &header.ns_table)
                .is_some(),
            format!("namespace proof for {block}:{ns} is invalid")
        );

        self.metrics.query_namespace_actions.add(1);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, VariantArray, Hash, PartialEq, Eq)]
enum Resource {
    Blocks,
    Leaves,
    Headers,
    Payloads,
}

impl Resource {
    fn random(rng: &mut impl RngCore) -> Self {
        *Self::VARIANTS.choose(rng).unwrap()
    }

    fn singular(&self) -> &'static str {
        match self {
            Self::Blocks => "block",
            Self::Leaves => "leaf",
            Self::Headers => "header",
            Self::Payloads => "payload",
        }
    }

    fn plural(&self) -> &'static str {
        match self {
            Self::Blocks => "blocks",
            Self::Leaves => "leaves",
            Self::Headers => "headers",
            Self::Payloads => "payloads",
        }
    }
}

#[derive(Clone, Copy, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray))]
enum Action {
    Query {
        resource: Resource,
        at: u64,
    },
    OpenStream {
        resource: Resource,
        from: u64,
    },
    CloseStream {
        resource: Resource,
        id: usize,
    },
    PollStream {
        resource: Resource,
        id: usize,
        amount: u8,
    },
    QueryWindow {
        from: u64,
        duration: u16,
    },
    QueryNamespace {
        block: u64,
        namespace: usize,
    },
}

impl Action {
    fn random(rng: &mut impl RngCore, distribution: &ActionDistribution) -> Self {
        match ActionDiscriminants::VARIANTS
            .choose_weighted(rng, |action| distribution.weight(*action))
            .unwrap()
        {
            ActionDiscriminants::Query => Self::Query {
                resource: Resource::random(rng),
                at: rng.next_u64(),
            },
            ActionDiscriminants::OpenStream => Self::OpenStream {
                resource: Resource::random(rng),
                from: rng.next_u64(),
            },
            ActionDiscriminants::CloseStream => Self::CloseStream {
                resource: Resource::random(rng),
                id: rng.next_u32() as usize,
            },
            ActionDiscriminants::PollStream => Self::PollStream {
                resource: Resource::random(rng),
                id: rng.next_u32() as usize,
                amount: (rng.next_u32() % u8::MAX as u32) as u8,
            },
            ActionDiscriminants::QueryWindow => Self::QueryWindow {
                from: rng.next_u64(),
                duration: (rng.next_u32() % u16::MAX as u32) as u16,
            },
            ActionDiscriminants::QueryNamespace => Self::QueryNamespace {
                block: rng.next_u64(),
                namespace: rng.next_u32() as usize,
            },
        }
    }
}

#[derive(Debug)]
struct Client {
    blocks: ResourceManager<BlockQueryData<SeqTypes>>,
    leaves: ResourceManager<LeafQueryData<SeqTypes>>,
    headers: ResourceManager<Header>,
    payloads: ResourceManager<PayloadQueryData<SeqTypes>>,
}

impl Client {
    fn new(opt: &Options, registry: &PrometheusMetrics) -> Self {
        let metrics = Arc::new(Metrics::new(registry));
        Self {
            blocks: ResourceManager::new(opt, metrics.clone()),
            leaves: ResourceManager::new(opt, metrics.clone()),
            headers: ResourceManager::new(opt, metrics.clone()),
            payloads: ResourceManager::new(opt, metrics.clone()),
        }
    }

    async fn run(&mut self, action: Action) -> anyhow::Result<()> {
        tracing::trace!(?action, "execute action");

        match action {
            Action::Query { resource, at } => match resource {
                Resource::Blocks => self.blocks.query(at).await,
                Resource::Leaves => self.leaves.query(at).await,
                Resource::Headers => self.headers.query(at).await,
                Resource::Payloads => self.payloads.query(at).await,
            },
            Action::OpenStream { resource, from } => match resource {
                Resource::Blocks => self.blocks.open_stream(from).await,
                Resource::Leaves => self.leaves.open_stream(from).await,
                Resource::Headers => self.headers.open_stream(from).await,
                Resource::Payloads => self.payloads.open_stream(from).await,
            },
            Action::CloseStream { resource, id } => {
                match resource {
                    Resource::Blocks => self.blocks.close_stream(id).await,
                    Resource::Leaves => self.leaves.close_stream(id).await,
                    Resource::Headers => self.headers.close_stream(id).await,
                    Resource::Payloads => self.payloads.close_stream(id).await,
                };
                Ok(())
            }
            Action::PollStream {
                resource,
                id,
                amount,
            } => match resource {
                Resource::Blocks => self.blocks.poll_stream(id, amount).await,
                Resource::Leaves => self.leaves.poll_stream(id, amount).await,
                Resource::Headers => self.headers.poll_stream(id, amount).await,
                Resource::Payloads => self.payloads.poll_stream(id, amount).await,
            },
            Action::QueryWindow { from, duration } => {
                self.headers.query_window(from, duration).await
            }
            Action::QueryNamespace { block, namespace } => {
                self.blocks.query_namespace(block, namespace).await
            }
        }
    }
}

async fn serve(port: u16, metrics: PrometheusMetrics) {
    let api = toml! {
        [route.metrics]
        PATH = ["/metrics"]
        METHOD = "METRICS"
    };
    let mut app = App::<_, ServerError, _>::with_state(RwLock::new(metrics));
    app.module::<ServerError>("status", api)
        .unwrap()
        .metrics("metrics", |_req, state| {
            async move { Ok(Cow::Borrowed(state)) }.boxed()
        })
        .unwrap();
    if let Err(err) = app
        .serve(format!("0.0.0.0:{port}"), SEQUENCER_VERSION)
        .await
    {
        tracing::error!("web server exited unexpectedly: {err:#}");
    }
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let metrics = PrometheusMetrics::default();
    let failed_actions = metrics.create_counter("failed_actions".into(), None);
    let mut client = Client::new(&opt, &metrics);
    let mut rng = rand::thread_rng();

    spawn(serve(opt.port, metrics));

    loop {
        if let Err(err) = client
            .run(Action::random(&mut rng, &opt.distribution))
            .await
        {
            failed_actions.add(1);
            tracing::error!("action failed: {err:#}");
        }
    }
}
