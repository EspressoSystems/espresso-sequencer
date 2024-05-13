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

use anyhow::{bail, ensure, Context};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{
    sync::RwLock,
    task::{sleep, spawn},
};
use clap::Parser;
use committable::Committable;
use derivative::Derivative;
use es_version::{SequencerVersion, SEQUENCER_VERSION};
use futures::{
    future::{FutureExt, TryFuture, TryFutureExt},
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
use jf_merkle_tree::{
    ForgetableMerkleTreeScheme, MerkleCommitment, MerkleTreeScheme, UniversalMerkleTreeScheme,
};
use jf_vid::VidScheme;
use rand::{seq::SliceRandom, RngCore};
use sequencer::{
    api::endpoints::NamespaceProofQueryData,
    options::parse_duration,
    state::{BlockMerkleTree, FeeMerkleTree},
    Header, SeqTypes,
};
use serde::de::DeserializeOwned;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};
use strum::{EnumDiscriminants, VariantArray};
use surf_disco::{error::ClientError, socket, Url};
use tide_disco::{error::ServerError, App};
use time::OffsetDateTime;
use toml::toml;
use tracing::info_span;

/// An adversarial stress test for sequencer APIs.
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Timeout for HTTP requests.
    ///
    /// Requests that take longer than this will fail, causing an error log and an increment of the
    /// `failed_actions` metric.
    #[clap(
        long,
        env = "ESPRESS_NASTY_CLIENT_HTTP_TIMEOUT",
        default_value = "30s",
        value_parser = parse_duration,
    )]
    http_timeout: Duration,

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

    #[clap(flatten)]
    client_config: ClientConfig,

    #[clap(flatten)]
    distribution: ActionDistribution,
}

#[derive(Clone, Copy, Debug, Parser)]
struct ClientConfig {
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

    /// The maximum number of retries before considering a fallible query failed.
    #[clap(long, env = "ESPRESSO_NASTY_CLIENT_MAX_RETRIES", default_value = "3")]
    max_retries: usize,

    /// The amount of time to wait between each retry of a fallible query.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_RETRY_DELAY",
        default_value = "1s",
        value_parser = parse_duration,
    )]
    retry_delay: Duration,

    /// The minimum number of successful tries to consider a failed operation "healed".
    #[clap(long, env = "ESPRESSO_NASTY_CLIENT_MIN_RETRIES", default_value = "5")]
    min_retries: usize,

    /// Time after which WebSockets connection failures are allowed.
    ///
    /// If there is an error polling a WebSockets connection last used more recently than this
    /// duration, it is considered an error. If the connection is staler than this, it is only a
    /// warning, and the connection is automatically refreshed.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEB_SOCKET_TIMEOUT",
        default_value = "60s",
        value_parser = parse_duration,
    )]
    web_socket_timeout: Duration,
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

    /// The weight of "query block state" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_QUERY_BLOCK_STATE",
        default_value = "3"
    )]
    weight_query_block_state: u8,

    /// The weight of "query fee state" actions in the random distribution.
    #[clap(
        long,
        env = "ESPRESSO_NASTY_CLIENT_WEIGHT_QUERY_FEE_STATE",
        default_value = "3"
    )]
    weight_query_fee_state: u8,
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
            ActionDiscriminants::QueryBlockState => self.weight_query_block_state,
            ActionDiscriminants::QueryFeeState => self.weight_query_fee_state,
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
    query_block_state_actions: Box<dyn Counter>,
    query_fee_state_actions: Box<dyn Counter>,
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
            query_block_state_actions: registry
                .create_counter("query_block_state_actions".into(), None),
            query_fee_state_actions: registry
                .create_counter("query_fee_state_actions".into(), None),
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
    refreshed: Instant,
}

#[derive(Debug)]
struct ResourceManager<T: Queryable> {
    client: surf_disco::Client<ClientError, SequencerVersion>,
    open_streams: BTreeMap<u64, Subscription<T>>,
    next_stream_id: u64,
    metrics: Arc<Metrics>,
    cfg: ClientConfig,
}

impl<T: Queryable> ResourceManager<T> {
    fn new(opt: &Options, metrics: Arc<Metrics>) -> Self {
        Self {
            client: surf_disco::Client::builder(opt.url.clone())
                .set_timeout(Some(opt.http_timeout))
                .build(),
            open_streams: BTreeMap::new(),
            next_stream_id: 0,
            metrics,
            cfg: opt.client_config,
        }
    }

    fn singular() -> &'static str {
        T::RESOURCE.singular()
    }

    fn plural() -> &'static str {
        T::RESOURCE.plural()
    }

    /// Retry a fallible operation several times before giving up.
    ///
    /// Some queries are allowed to fail occasionally, but should heal themselves quickly. For
    /// example, we may query the block height from one node and then get routed to another node to
    /// query an object at that height. The second node may be lagging and return 404, but it should
    /// catch up quickly and the query should start to succeed. Or, we may query for an object which
    /// is missing, but the server should quickly heal by fetching the missing object from a peer.
    ///
    /// This function will retry a fallible operation a configurable number of times. If the
    /// operation ever fails, a warning will be logged, but it is not considered a failed action
    /// unless the operation fails several times in a row.
    ///
    /// If the operation fails even once but eventually succeeds, we will retry the operation
    /// several times to check that all servers are "healed": at this point, the operation should no
    /// longer fail even once.
    async fn retry<F: TryFuture<Error = anyhow::Error>>(
        &self,
        span: tracing::Span,
        f: impl Fn() -> F,
    ) -> anyhow::Result<F::Ok> {
        let _enter = span.enter();
        tracing::debug!("starting fallible operation");

        let mut i = 0;
        loop {
            match f().into_future().await {
                Ok(res) if i == 0 => {
                    // Succeeded on the first try, get on with it.
                    return Ok(res);
                }
                Ok(res) => {
                    // Succeeded after at least one failure; retry a number of additional times to
                    // be sure the endpoint is healed.
                    tracing::info!("succeeded after {i} retries; checking health");
                    for _ in 0..self.cfg.min_retries {
                        f().into_future().await.context(
                            "operation is flaky; succeeded on retry but not fully healed",
                        )?;
                    }
                    return Ok(res);
                }
                Err(err) if i < self.cfg.max_retries => {
                    tracing::warn!("failed, will retry: {err:#}");
                    i += 1;
                }
                Err(err) => {
                    return Err(err).context("failed too many times");
                }
            }
        }
    }

    async fn query(&self, at: u64) -> anyhow::Result<()> {
        let at = self.adjust_index(at).await?;
        let obj = self
            .retry(
                info_span!("query", resource = Self::singular(), at),
                || async {
                    self.client
                        .get::<T>(&format!("availability/{}/{at}", Self::singular()))
                        .send()
                        .await
                        .context(format!("fetching {} {at}", Self::singular()))
                },
            )
            .await?;

        // Query by hash and check consistency.
        let hash = obj.hash();
        let by_hash = self
            .retry(
                info_span!("query by hash", resource = Self::singular(), at, hash),
                || async {
                    self.client
                        .get(&format!(
                            "availability/{}/{}/{hash}",
                            Self::singular(),
                            T::HASH_URL_SEGMENT,
                        ))
                        .send()
                        .await
                        .context(format!("fetching {} {hash}", Self::singular()))
                },
            )
            .await?;
        ensure!(
            obj == by_hash,
            format!(
                "query for {} {at} by hash {hash} is not consistent",
                Self::singular()
            )
        );

        self.metrics.query_actions[&T::RESOURCE].add(1);
        Ok(())
    }

    async fn open_stream(&mut self, from: u64) -> anyhow::Result<()> {
        if self.open_streams.len() >= self.cfg.max_open_streams {
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
                refreshed: Instant::now(),
            },
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
        tracing::info!("closing {} stream {id}", Self::singular());
        self.open_streams.remove(&id);
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
        let mut blocking = 0;
        for _ in 0..amount {
            let (id, stream) = self.open_streams.iter_mut().nth(index).unwrap();

            // Check if the next item is immediately available or if we're going to block.
            if stream.stream.as_mut().peek().now_or_never().is_none() {
                blocking += 1;
                if blocking > self.cfg.max_blocking_polls {
                    tracing::info!("aborting poll_stream action; exceeded maximum blocking polls");
                    return Ok(());
                }
            }

            let pos = stream.position;
            let refreshed = stream.refreshed;
            stream.position += 1;
            let span = info_span!(
                "polling stream",
                resource = Self::singular(),
                id,
                pos,
                ?refreshed,
            );
            let _enter = span.enter();
            let obj = loop {
                let Some(res) = stream.stream.next().await else {
                    let id = *id;
                    self.open_streams.remove(&id);
                    self.metrics.open_streams[&T::RESOURCE].update(-1);

                    // All of our streams are supposed to be indefinite.
                    bail!("{} stream {id} ended", Self::singular());
                };
                match res {
                    Ok(obj) => {
                        // Successfully polling a WebSockets connection should reset the connection
                        // timeout, so we don't expect errors from this connection in the near
                        // future.
                        stream.refreshed = Instant::now();
                        break obj;
                    }
                    Err(err) if refreshed.elapsed() >= self.cfg.web_socket_timeout => {
                        // Streams are allowed to fail if the connection is too old. Warn about it,
                        // but refresh the connection and try again.
                        tracing::warn!("error in old connection, refreshing connection: {err:#}");
                        let conn = self
                            .client
                            .socket(&format!("availability/stream/{}/{pos}", Self::plural()))
                            .subscribe()
                            .await
                            .context(format!("subscribing to {} from {pos}", Self::plural()))?;
                        stream.stream = Box::pin(conn.peekable());
                        stream.refreshed = Instant::now();
                    }
                    Err(err) => {
                        // Errors on a relatively fresh connection are not allowed. Close the stream
                        // since it is apparently in a bad state, and return an error.
                        let id = *id;
                        self.open_streams.remove(&id);
                        self.metrics.open_streams[&T::RESOURCE].update(-1);
                        return Err(err).context(format!(
                            "polling {} stream {id} at {pos}, last refreshed {:?} ago",
                            Self::singular(),
                            refreshed.elapsed()
                        ));
                    }
                }
            };

            // Check consistency against a regular query.
            let id = *id;
            let expected = self
                .retry(info_span!("fetching expected object"), || async {
                    self.client
                        .get(&format!("availability/{}/{pos}", Self::singular()))
                        .send()
                        .await
                        .context(format!("fetching {} {pos}", Self::singular()))
                })
                .await?;
            ensure!(
                obj == expected,
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

        let window = self
            .retry(
                info_span!("timestamp window", resource = Self::singular(), start, end),
                || async {
                    self.client
                        .get::<TimeWindowQueryData<Header>>(&format!(
                            "node/header/window/{start}/{end}"
                        ))
                        .send()
                        .await
                        .context(format!("fetching timestamp window from {start} to {end}"))
                },
            )
            .await?;

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

    async fn query_block_state(&self, block: u64, index: u64) -> anyhow::Result<()> {
        let (block, index) = match self.adjust_index(block).await? {
            0 | 1 => {
                // The block state at height 0 is empty, so to have a valid query just adjust to
                // querying at height 1. At height 1, the only valid index to query is 0.
                (1, 0)
            }
            block => {
                // At any other height, all indices between 0 and `block - 1` are valid to query.
                (block, index % (block - 1))
            }
        };

        // Get the header of the state snapshot we're going to query and the block commitment we're
        // going to look up from the Merkle tree, so we can later verify our results.
        let block_header = self
            .retry(info_span!("get block header", block), || async {
                self.client
                    .get::<Header>(&format!("availability/header/{block}"))
                    .send()
                    .await
                    .context(format!("getting header {block}"))
            })
            .await?;
        let index_header = self
            .retry(info_span!("get index header", index), || async {
                self.client
                    .get::<Header>(&format!("availability/header/{index}"))
                    .send()
                    .await
                    .context(format!("getting header {index}"))
            })
            .await?;

        // Get a Merkle proof for the block commitment at position `index` from state `block`.
        let proof = self
            .retry(info_span!("get block proof", block, index), || async {
                self.client
                    .get::<<BlockMerkleTree as MerkleTreeScheme>::MembershipProof>(&format!(
                        "block-state/{block}/{index}"
                    ))
                    .send()
                    .await
                    .context(format!("getting merkle proof {block},{index}"))
            })
            .await?;

        // Check that the proof proves inclusion of `index_header` at position `index` relative to
        // `block_header`.
        BlockMerkleTree::verify(block_header.block_merkle_tree_root.digest(), index, &proof)
            .context("malformed merkle proof")?
            .or_else(|_| bail!("invalid merkle proof"))?;
        ensure!(
            proof.elem() == Some(&index_header.commit()),
            "merkle proof is for wrong element: {:?} != {:?}",
            proof.elem(),
            index_header.commit()
        );

        // Look up the proof a different way, by state commitment, and check that we get the same
        // proof.
        let proof2 = self
            .retry(
                info_span!(
                    "get block proof by state commitment",
                    block,
                    index,
                    commitment = %block_header.block_merkle_tree_root,
                ),
                || async {
                    self.client
                        .get::<<BlockMerkleTree as MerkleTreeScheme>::MembershipProof>(&format!(
                            "block-state/commit/{}/{index}",
                            block_header.block_merkle_tree_root,
                        ))
                        .send()
                        .await
                        .context(format!(
                            "getting merkle proof {},{index}",
                            block_header.block_merkle_tree_root
                        ))
                },
            )
            .await?;
        ensure!(
            proof2 == proof,
            "got a different proof when querying by commitment, {proof2:?} != {proof:?}"
        );

        self.metrics.query_block_state_actions.add(1);
        Ok(())
    }

    async fn query_fee_state(&self, block: u64, builder: u64) -> anyhow::Result<()> {
        let block = self.adjust_index(block).await?;
        let builder = if block == 0 { 0 } else { builder % block };

        // Get the header of block `builder` so we can get an address (the builder account) to
        // query.
        let builder_header = self
            .retry(info_span!("get builder header", builder), || async {
                self.client
                    .get::<Header>(&format!("availability/header/{builder}"))
                    .send()
                    .await
                    .context(format!("getting header {builder}"))
            })
            .await?;
        let builder_address = builder_header.fee_info.account();

        // Get the header of the state snapshot we're going to query so we can later verify our
        // results.
        let block_header = self
            .retry(info_span!("get block header", block), || async {
                self.client
                    .get::<Header>(&format!("availability/header/{block}"))
                    .send()
                    .await
                    .context(format!("getting header {block}"))
            })
            .await?;

        // Get a Merkle proof for the fee state of `builder_address` from state `block`.
        let proof = self
            .retry(
                info_span!("get account proof", block, %builder_address),
                || async {
                    self.client
                        .get::<<FeeMerkleTree as MerkleTreeScheme>::MembershipProof>(&format!(
                            "fee-state/{block}/{builder_address}"
                        ))
                        .send()
                        .await
                        .context(format!("getting merkle proof {block},{builder_address}"))
                },
            )
            .await?;

        // Check that the proof is valid relative to `builder_header`.
        if proof.elem().is_some() {
            FeeMerkleTree::verify(
                block_header.fee_merkle_tree_root.digest(),
                builder_address,
                &proof,
            )
            .context("malformed membership proof")?
            .or_else(|_| bail!("invalid membership proof"))?;
        } else {
            ensure!(
                FeeMerkleTree::from_commitment(block_header.fee_merkle_tree_root)
                    .non_membership_verify(builder_address, &proof)
                    .context("malformed non-membership proof")?,
                "invalid non-membership proof"
            );
        }

        // Look up the proof a different way, by state commitment, and check that we get the same
        // proof.
        let proof2 = self
            .retry(
                info_span!(
                    "get account proof by state commitment",
                    block,
                    %builder_address,
                    commitment = %block_header.fee_merkle_tree_root,
                ),
                || async {
                    self.client
                        .get::<<FeeMerkleTree as MerkleTreeScheme>::MembershipProof>(&format!(
                            "fee-state/commit/{}/{builder_address}",
                            block_header.fee_merkle_tree_root,
                        ))
                        .send()
                        .await
                        .context(format!(
                            "getting merkle proof {},{builder_address}",
                            block_header.fee_merkle_tree_root
                        ))
                },
            )
            .await?;
        ensure!(
            proof2 == proof,
            "got a different proof when querying by commitment, {proof2:?} != {proof:?}"
        );

        self.metrics.query_fee_state_actions.add(1);
        Ok(())
    }
}

impl ResourceManager<BlockQueryData<SeqTypes>> {
    async fn query_namespace(&self, block: u64, index: usize) -> anyhow::Result<()> {
        let block = self.adjust_index(block).await?;
        let span = info_span!("query namespace", resource = Self::singular(), block, index);
        let _enter = span.enter();

        // Download the header so we can translate the `namespace` index to a namespace ID using
        // the namespace table.
        let header: Header = self
            .retry(info_span!("fetch header"), || async {
                self.client
                    .get(&format!("availability/header/{block}"))
                    .send()
                    .await
                    .context(format!("fetching header {block}"))
            })
            .await?;
        if header.ns_table.is_empty() {
            tracing::info!("not fetching namespace because block {block} is empty");
            return Ok(());
        }
        let ns = header.ns_table.get_table_entry(index).0;

        let ns_proof: NamespaceProofQueryData = self
            .retry(info_span!("fetch namespace", %ns), || async {
                self.client
                    .get(&format!("availability/block/{block}/namespace/{ns}"))
                    .send()
                    .await
                    .context(format!("fetching namespace {block}:{ns}"))
            })
            .await?;

        // Verify proof.
        let vid_common: VidCommonQueryData<SeqTypes> = self
            .retry(info_span!("fetch VID common"), || async {
                self.client
                    .get(&format!("availability/vid/common/{block}"))
                    .send()
                    .await
                    .context(format!("fetching VID common {block}"))
            })
            .await?;
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(vid_common.common()) as usize);
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
    QueryBlockState {
        block: u64,
        index: u64,
    },
    QueryFeeState {
        block: u64,
        // The index of the block whose builder address should be looked up. This leads to more
        // realistic queries than just randomly generating addresses.
        builder: u64,
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
            ActionDiscriminants::QueryBlockState => Self::QueryBlockState {
                block: rng.next_u64(),
                index: rng.next_u64(),
            },
            ActionDiscriminants::QueryFeeState => Self::QueryFeeState {
                block: rng.next_u64(),
                builder: rng.next_u64(),
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
            Action::QueryBlockState { block, index } => {
                self.headers.query_block_state(block, index).await
            }
            Action::QueryFeeState { block, builder } => {
                self.headers.query_fee_state(block, builder).await
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
    let mut app = App::<_, ServerError>::with_state(RwLock::new(metrics));
    app.module::<ServerError, SequencerVersion>("status", api)
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
    let total_actions = metrics.create_counter("total_actions".into(), None);
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
        total_actions.add(1);
    }
}
