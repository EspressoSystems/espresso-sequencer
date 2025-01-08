use crate::{parse_duration, v0_3::StakeTables};
use async_broadcast::{InactiveReceiver, Sender};
use clap::Parser;
use derive_more::Deref;
use ethers::{
    prelude::{H256, U256},
    providers::{Http, Provider},
};
use hotshot_types::{
    data::EpochNumber,
    traits::metrics::{Counter, Gauge, Metrics, NoMetrics},
};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap, num::NonZeroUsize, sync::{self, Arc}, time::{Duration, Instant}
};
use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
};
use url::Url;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub number: u64,
    pub timestamp: U256,
    pub hash: H256,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1Snapshot {
    /// The relevant snapshot of the L1 includes a reference to the current head of the L1 chain.
    ///
    /// Note that the L1 head is subject to changing due to a reorg. However, no reorg will change
    /// the _number_ of this block in the chain: L1 block numbers will always be sequentially
    /// increasing. Therefore, the sequencer does not have to worry about reorgs invalidating this
    /// snapshot.
    pub head: u64,

    /// The snapshot also includes information about the latest finalized L1 block.
    ///
    /// Since this block is finalized (ie cannot be reorged) we can include specific information
    /// about the particular block, such as its hash and timestamp.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub finalized: Option<L1BlockInfo>,
}

/// Configuration for an L1 client.
#[derive(Clone, Debug, Parser)]
pub struct L1ClientOptions {
    /// Delay when retrying failed L1 queries.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_RETRY_DELAY",
        default_value = "1s",
        value_parser = parse_duration,
    )]
    pub l1_retry_delay: Duration,

    /// Request rate when polling L1.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_POLLING_INTERVAL",
        default_value = "7s",
        value_parser = parse_duration,
    )]
    pub l1_polling_interval: Duration,

    /// Maximum number of L1 blocks to keep in cache at once.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_BLOCKS_CACHE_SIZE",
        default_value = "100"
    )]
    pub l1_blocks_cache_size: NonZeroUsize,

    /// Number of L1 events to buffer before discarding.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_EVENTS_CHANNEL_CAPACITY",
        default_value = "100"
    )]
    pub l1_events_channel_capacity: usize,

    /// Maximum number of L1 blocks that can be scanned for events in a single query.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_EVENTS_MAX_BLOCK_RANGE",
        default_value = "10000"
    )]
    pub l1_events_max_block_range: u64,

    /// Maximum time to wait for new heads before considering a stream invalid and reconnecting.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_SUBSCRIPTION_TIMEOUT",
        default_value = "1m",
        value_parser = parse_duration,
    )]
    pub subscription_timeout: Duration,

    /// Fail over to another provider if the current provider fails twice within this window.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_FREQUENT_FAILURE_TOLERANCE",
        default_value = "1m",
        value_parser = parse_duration,
    )]
    pub l1_frequent_failure_tolerance: Duration,

    /// Fail over to another provider if the current provider fails many times in a row, within any
    /// time window.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_CONSECUTIVE_FAILURE_TOLERANCE",
        default_value = "10"
    )]
    pub l1_consecutive_failure_tolerance: usize,

    /// Amount of time to wait after receiving a 429 response before making more L1 RPC requests.
    ///
    /// If not set, the general l1-retry-delay will be used.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_L1_RATE_LIMIT_DELAY",
        value_parser = parse_duration,
    )]
    pub l1_rate_limit_delay: Option<Duration>,

    /// Separate provider to use for subscription feeds.
    ///
    /// Typically this would be a WebSockets endpoint while the main provider uses HTTP.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_WS_PROVIDER", value_delimiter = ',')]
    pub l1_ws_provider: Option<Vec<Url>>,

    #[clap(skip = Arc::<Box<dyn Metrics>>::new(Box::new(NoMetrics)))]
    pub metrics: Arc<Box<dyn Metrics>>,
}

#[derive(Clone, Debug)]
/// An Ethereum provider and configuration to interact with the L1.
///
/// This client runs asynchronously, updating an in-memory snapshot of the relevant L1 information
/// each time a new L1 block is published. The main advantage of this is that we can update the L1
/// state at the pace of the L1, instead of the much faster pace of HotShot consensus.This makes it
/// easy to use a subscription instead of polling for new blocks, vastly reducing the number of L1
/// RPC calls we make.
pub struct L1Client {
    /// `Provider` from `ethers-provider`.
    pub(crate) provider: Arc<Provider<MultiRpcClient>>,
    /// Shared state updated by an asynchronous task which polls the L1.
    pub(crate) state: Arc<Mutex<L1State>>,
    /// TODO: We need to be able to take out sync locks on this part of the
    /// state. until the trait definition of Membership is updated in HotShot.
    pub(crate) stake_table_state: sync::Arc<sync::RwLock<BTreeMap<EpochNumber, StakeTables>>>,
    /// Channel used by the async update task to send events to clients.
    pub(crate) sender: Sender<L1Event>,
    /// Receiver for events from the async update task.
    pub(crate) receiver: InactiveReceiver<L1Event>,
    /// Async task which updates the shared state.
    pub(crate) update_task: Arc<L1UpdateTask>,
}

/// In-memory view of the L1 state, updated asynchronously.
#[derive(Debug)]
pub(crate) struct L1State {
    pub(crate) snapshot: L1Snapshot,
    pub(crate) finalized: LruCache<u64, L1BlockInfo>,
}

#[derive(Clone, Debug)]
pub(crate) enum L1Event {
    NewHead { head: u64 },
    NewFinalized { finalized: L1BlockInfo },
}

#[derive(Debug, Default)]
pub(crate) struct L1UpdateTask(pub(crate) Mutex<Option<JoinHandle<()>>>);

#[derive(Clone, Debug)]
pub(crate) struct L1ClientMetrics {
    pub(crate) head: Arc<dyn Gauge>,
    pub(crate) finalized: Arc<dyn Gauge>,
    pub(crate) reconnects: Arc<dyn Counter>,
    pub(crate) failovers: Arc<dyn Counter>,
}

/// An RPC client with multiple remote providers.
///
/// This client utilizes one RPC provider at a time, but if it detects that the provider is in a
/// failing state, it will automatically switch to the next provider in its list.
#[derive(Clone, Debug)]
pub(crate) struct MultiRpcClient {
    pub(crate) clients: Arc<Vec<L1Provider>>,
    pub(crate) status: Arc<RwLock<MultiRpcClientStatus>>,
    pub(crate) failover_send: Sender<()>,
    pub(crate) failover_recv: InactiveReceiver<()>,
    pub(crate) opt: L1ClientOptions,
    pub(crate) metrics: L1ClientMetrics,
}

/// The state of the current provider being used by a [`MultiRpcClient`].
#[derive(Debug, Default)]
pub(crate) struct MultiRpcClientStatus {
    pub(crate) client: usize,
    pub(crate) last_failure: Option<Instant>,
    pub(crate) consecutive_failures: usize,
    pub(crate) rate_limited_until: Option<Instant>,
}

/// A single provider in a [`MultiRpcClient`].
#[derive(Debug, Deref)]
pub(crate) struct L1Provider {
    #[deref]
    pub(crate) inner: Http,
    pub(crate) failures: Box<dyn Counter>,
}
