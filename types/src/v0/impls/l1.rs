use std::{
    cmp::{min, Ordering},
    fmt::Debug,
    iter::FromFn,
    num::NonZeroUsize,
    sync::Arc,
    time::Instant,
};

use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::{
    fee_contract::FeeContract,
    permissioned_stake_table::{PermissionedStakeTable, StakersUpdatedFilter},
};
use ethers::{
    prelude::{Address, BlockNumber, Middleware, Provider, H256, U256},
    providers::{Http, HttpClientError, JsonRpcClient, JsonRpcError, Ws},
};
use futures::{
    future::{Future, FutureExt},
    stream::{self, StreamExt},
};
use hotshot_types::traits::metrics::{CounterFamily, Metrics};
use itertools::Itertools;
use lru::LruCache;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tokio::{
    spawn,
    sync::{Mutex, MutexGuard},
    task::JoinHandle,
    time::sleep,
};
use tracing::Instrument;
use url::Url;

use super::{
    v0_3::StakeTables, L1BlockInfo, L1ClientMetrics, L1State, L1UpdateTask, MultiRpcClient,
    MultiRpcClientStatus,
};
use crate::{FeeInfo, L1Client, L1ClientOptions, L1Event, L1Provider, L1Snapshot};

impl PartialOrd for L1BlockInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for L1BlockInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl Committable for L1BlockInfo {
    fn commit(&self) -> Commitment<Self> {
        let mut timestamp = [0u8; 32];
        self.timestamp.to_little_endian(&mut timestamp);

        RawCommitmentBuilder::new(&Self::tag())
            .u64_field("number", self.number)
            // `RawCommitmentBuilder` doesn't have a `u256_field` method, so we simulate it:
            .constant_str("timestamp")
            .fixed_size_bytes(&timestamp)
            .constant_str("hash")
            .fixed_size_bytes(&self.hash.0)
            .finalize()
    }

    fn tag() -> String {
        "L1BLOCK".into()
    }
}

impl L1BlockInfo {
    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn timestamp(&self) -> U256 {
        self.timestamp
    }

    pub fn hash(&self) -> H256 {
        self.hash
    }
}

impl Drop for L1UpdateTask {
    fn drop(&mut self) {
        if let Some(tasks) = self.0.get_mut().take() {
            for task in tasks {
                task.abort();
            }
        }
    }
}

impl Default for L1ClientOptions {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

impl L1ClientOptions {
    /// Use the given metrics collector to publish metrics related to the L1 client.
    pub fn with_metrics(mut self, metrics: &(impl Metrics + ?Sized)) -> Self {
        self.metrics = Arc::new(metrics.subgroup("l1".into()));
        self
    }

    /// Instantiate an `L1Client` for a given list of provider `Url`s.
    pub fn connect(self, urls: impl IntoIterator<Item = Url>) -> L1Client {
        L1Client::with_provider(Provider::new(MultiRpcClient::new(self, urls)))
    }

    fn rate_limit_delay(&self) -> Duration {
        self.l1_rate_limit_delay.unwrap_or(self.l1_retry_delay)
    }
}

impl L1ClientMetrics {
    fn new(metrics: &(impl Metrics + ?Sized)) -> Self {
        Self {
            head: metrics.create_gauge("head".into(), None).into(),
            finalized: metrics.create_gauge("finalized".into(), None).into(),
            reconnects: metrics
                .create_counter("stream_reconnects".into(), None)
                .into(),
            failovers: metrics.create_counter("failovers".into(), None).into(),
        }
    }
}

impl L1Provider {
    fn new(index: usize, url: Url, failures: &dyn CounterFamily) -> Self {
        Self {
            failures: failures.create(vec![index.to_string()]),
            inner: Http::new(url),
        }
    }
}

impl MultiRpcClient {
    fn new(opt: L1ClientOptions, clients: impl IntoIterator<Item = Url>) -> Self {
        // The type of messages in this channel is (), i.e. all messages are identical, and we only
        // ever use it to await the next single failure (see `next_failover`). In other words, it
        // functions as a oneshot broadcast channel, so a capacity of 1 is safe.
        let (mut failover_send, failover_recv) = async_broadcast::broadcast(1);
        failover_send.set_await_active(false);
        failover_send.set_overflow(true);

        let metrics = L1ClientMetrics::new(&**opt.metrics);
        let failures = opt
            .metrics
            .counter_family("failed_requests".into(), vec!["provider".into()]);

        Self {
            clients: Arc::new(
                clients
                    .into_iter()
                    .enumerate()
                    .map(|(i, url)| L1Provider::new(i, url, &*failures))
                    .collect(),
            ),
            status: Default::default(),
            failover_send,
            failover_recv: failover_recv.deactivate(),
            opt,
            metrics,
        }
    }

    async fn failover(&self, time: Instant, status: &mut MultiRpcClientStatus) {
        tracing::warn!(
            ?status,
            ?time,
            frequent_failure_tolerance = ?self.opt.l1_frequent_failure_tolerance,
            consecutive_failure_tolerance = ?self.opt.l1_consecutive_failure_tolerance,
            current = status.client,
            "L1 client failing over",
        );
        status.client += 1;
        status.rate_limited_until = None;
        status.last_failure = None;
        status.consecutive_failures = 0;
        self.metrics.failovers.add(1);
        self.failover_send.broadcast_direct(()).await.ok();
    }

    fn next_failover(&self) -> impl Future<Output = ()> {
        let recv = self.failover_recv.activate_cloned();
        recv.into_future().map(|_| ())
    }

    fn options(&self) -> &L1ClientOptions {
        &self.opt
    }

    fn metrics(&self) -> &L1ClientMetrics {
        &self.metrics
    }
}

#[async_trait]
impl JsonRpcClient for MultiRpcClient {
    type Error = HttpClientError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let current = {
            let status = self.status.read().await;

            // If we've been rate limited, back off until the limit (hopefully) expires.
            if let Some(t) = status.rate_limited_until {
                if t > Instant::now() {
                    // Return an error with a non-standard code to indicate client-side rate limit.
                    return Err(JsonRpcError {
                        code: -20000,
                        message: "rate limit exceeded".into(),
                        data: None,
                    }
                    .into());
                }
            }

            status.client
        };
        let provider = current % self.clients.len();
        let client = &self.clients[provider];
        match client.request(method, &params).await {
            Ok(res) => Ok(res),
            Err(err) => {
                let t = Instant::now();
                tracing::warn!(?t, method, ?params, provider, "L1 client error: {err:#}");
                client.failures.add(1);

                // Keep track of failures, failing over to the next client if necessary.
                let mut status = self.status.write().await;
                if status.client != current {
                    // Someone else has also gotten a failure, and the client has already been
                    // failed over.
                    return Err(err);
                }

                // Treat rate limited errors specially; these should not cause failover, but instead
                // should only cause us to temporarily back off on making requests to the RPC
                // server.
                if let HttpClientError::ReqwestError(e) = &err {
                    if matches!(e.status(), Some(StatusCode::TOO_MANY_REQUESTS)) {
                        status.rate_limited_until = Some(t + self.opt.rate_limit_delay());
                        return Err(err);
                    }
                }

                if let Some(prev) = status.last_failure {
                    if t - prev < self.opt.l1_frequent_failure_tolerance {
                        // We have failed twice inside the allowed window, so we should failover to
                        // the next client.
                        self.failover(t, &mut status).await;
                        return Err(err);
                    }
                }

                status.consecutive_failures += 1;
                if status.consecutive_failures >= self.opt.l1_consecutive_failure_tolerance {
                    // We have failed too many times in a row, albeit not rapidly enough to trigger
                    // the frequent failure tolerance. Still, we now trigger a failover based on the
                    // consecutive failures policy.
                    self.failover(t, &mut status).await;
                    return Err(err);
                }

                // If we're not failing over, update the last failure time.
                status.last_failure = Some(t);
                Err(err)
            }
        }
    }
}

impl L1Client {
    fn with_provider(mut provider: Provider<MultiRpcClient>) -> Self {
        let opt = provider.as_ref().options().clone();

        let (sender, mut receiver) = async_broadcast::broadcast(opt.l1_events_channel_capacity);
        receiver.set_await_active(false);
        receiver.set_overflow(true);

        provider.set_interval(opt.l1_polling_interval);
        Self {
            provider: Arc::new(provider),
            state: Arc::new(Mutex::new(L1State::new(opt.l1_blocks_cache_size))),
            sender,
            receiver: receiver.deactivate(),
            update_task: Default::default(),
        }
    }

    /// Construct a new L1 client with the default options.
    pub fn new(url: Url) -> Self {
        L1ClientOptions::default().connect([url])
    }

    /// Start the background tasks which keep the L1 client up to date.
    pub async fn spawn_tasks(&self) {
        let mut update_task = self.update_task.0.lock().await;
        if update_task.is_none() {
            // TODO either explictly order or use JoinSet
            let tasks: Vec<JoinHandle<()>> = vec![self.update_loop()]
                .into_iter()
                .map(|x| spawn(x))
                .collect();
            *update_task = Some(tasks);
        }
    }

    /// Shut down background tasks associated with this L1 client.
    ///
    /// The L1 client will still be usable, but will stop updating until [`start`](Self::start) is
    /// called again.
    pub async fn shut_down_tasks(&self) {
        if let Some(update_task) = self.update_task.0.lock().await.take() {
            // TODO review order
            for task in update_task {
                task.abort();
            }
        }
    }

    pub fn provider(&self) -> &impl Middleware<Error: 'static> {
        &self.provider
    }
    /// Update stake-table cache on `L1Event::NewHead`.
    fn stake_update_loop(&self) -> impl Future<Output = ()> {
        let opt = self.options();
        let max_block_range = opt.l1_events_max_block_range;
        let retry_delay = opt.l1_retry_delay;
        let span = tracing::warn_span!("L1 client update");
        let state = self.state.clone();
        let stake_table_contract =
            // TODO attach address to L1Client
            PermissionedStakeTable::new(Address::default(), self.provider.clone());

        let mut events = self.receiver.activate_cloned();

        async move {
            loop {
                let last_head = {
                    let state = state.lock().await;
                    state.snapshot.head
                };
                while let Some(event) = events.next().await {
                    let L1Event::NewHead { head } = event else {
                        continue;
                    };

                    let chunks = self.chunky2(last_head, block);
                    let mut events: Vec<StakersUpdatedFilter> = Vec::new();
                    for (from, to) in chunks {
                        tracing::debug!(from, to, "fetch stake table events in range");
                        match stake_table_contract
                            .stakers_updated_filter()
                            .from_block(*from)
                            .to_block(*to)
                            .query()
                            .await
                        {
                            Ok(e) => {
                                for event in e {
                                    events.push(event)
                                }
                                break;
                            }
                            Err(err) => {
                                tracing::warn!(from, to, %err, "Stake Table L1Event Error");
                                sleep(retry_delay).await;
                            }
                        }
                    }
                    sleep(retry_delay).await;
                }
                sleep(retry_delay).await;
            }
        }
        .instrument(span)
    }

    fn update_loop(&self) -> impl Future<Output = ()> {
        let opt = self.options();
        let rpc = self.provider.clone();
        let ws_urls = opt.l1_ws_provider.clone();
        let retry_delay = opt.l1_retry_delay;
        let subscription_timeout = opt.subscription_timeout;
        let state = self.state.clone();
        let sender = self.sender.clone();
        let metrics = self.metrics().clone();

        let span = tracing::warn_span!("L1 client update");
        async move {
            for i in 0.. {
                let ws;

                // Subscribe to new blocks.
                let mut block_stream = {
                    let res = match &ws_urls {
                        Some(urls) => {
                            // Use a new WebSockets host each time we retry in case there is a
                            // problem with one of the hosts specifically.
                            let provider = i % urls.len();
                            let url = &urls[provider];
                            ws = match Provider::<Ws>::connect(url.clone()).await {
                                Ok(ws) => ws,
                                Err(err) => {
                                    tracing::warn!(provider, "failed to connect WebSockets provider: {err:#}");
                                    sleep(retry_delay).await;
                                    continue;
                                }
                            };
                            ws.subscribe_blocks().await.map(StreamExt::boxed)
                        }
                        None => {
                            let failover = (*rpc).as_ref().next_failover().map(|()| {
                                tracing::warn!("aborting subscription stream due to provider failover");
                            });
                            rpc
                            .watch_blocks()
                            .await
                            .map(|stream| {
                                let rpc = rpc.clone();

                                // For HTTP, we simulate a subscription by polling. The polling
                                // stream provided by ethers only yields block hashes, so for each
                                // one, we have to go fetch the block itself.
                                stream.filter_map(move |hash| {
                                    let rpc = rpc.clone();
                                    async move {
                                        match rpc.get_block(hash).await {
                                            Ok(Some(block)) => Some(block),
                                            // If we can't fetch the block for some reason, we can
                                            // just skip it.
                                            Ok(None) => {
                                                tracing::warn!(%hash, "HTTP stream yielded a block hash that was not available");
                                                None
                                            }
                                            Err(err) => {
                                                tracing::warn!(%hash, "error fetching block from HTTP stream: {err:#}");
                                                None
                                            }
                                        }
                                    }
                                })
                            }
                            .take_until(failover)
                            .boxed())
                        }
                    };
                    match res {
                        Ok(stream) => stream,
                        Err(err) => {
                            tracing::error!("error subscribing to L1 blocks: {err:#}");
                            sleep(retry_delay).await;
                            continue;
                        }
                    }
                };

                tracing::info!("established L1 block stream");
                loop {
                    // Wait for a block, timing out if we don't get one soon enough
                    let block_timeout = tokio::time::timeout(subscription_timeout, block_stream.next()).await;
                    match block_timeout {
                        // We got a block
                        Ok(Some(head)) => {
                            let Some(head_number) = head.number else {
                                // This shouldn't happen, but if it does, it means the block stream has
                                // erroneously given us a pending block. We are only interested in committed
                                // blocks, so just skip this one.
                                tracing::warn!("got block from L1 block stream with no number");
                                continue;
                            };
                            let head = head_number.as_u64();
                            tracing::debug!(head, "received L1 block");

                            // A new block has been produced. This happens fairly rarely, so it is now ok to
                            // poll to see if a new block has been finalized.
                            let finalized = loop {
                                match get_finalized_block(&rpc).await {
                                    Ok(finalized) => break finalized,
                                    Err(err) => {
                                        tracing::warn!("error getting finalized block: {err:#}");
                                        sleep(retry_delay).await;
                                    }
                                }
                            };

                            // Update the state snapshot;
                            let mut state = state.lock().await;
                            let snapshot_head = state.snapshot.head;
                            if head > snapshot_head {
                                tracing::debug!(head, old_head = state.snapshot.head, "L1 head updated");
                                metrics.head.set(head as usize);
                                state.snapshot.head = head;
                                // Emit an event about the new L1 head. Ignore send errors; it just means no
                                // one is listening to events right now.
                                sender
                                    .broadcast_direct(L1Event::NewHead { head })
                                    .await
                                    .ok();
                            }
                            if finalized > state.snapshot.finalized {
                                tracing::info!(
                                    ?finalized,
                                    old_finalized = ?state.snapshot.finalized,
                                    "L1 finalized updated",
                                );
                                if let Some(finalized) = finalized {
                                    metrics.finalized.set(finalized.number as usize);
                                }
                                state.snapshot.finalized = finalized;
                                if let Some(finalized) = finalized {
                                    sender
                                        .broadcast_direct(L1Event::NewFinalized { finalized })
                                        .await
                                        .ok();
                                }
                            }
                            tracing::debug!("updated L1 snapshot to {:?}", state.snapshot);
                        }
                        // The stream ended
                        Ok(None) => {
                            tracing::error!("L1 block stream ended unexpectedly, trying to re-establish block stream");
                            break;
                        }
                        // We timed out waiting for a block
                        Err(_) => {
                            tracing::error!("No block received for 60 seconds, trying to re-establish block stream");
                            break;
                        }
                    }
                }

                metrics.reconnects.add(1);
            }
        }.instrument(span)
    }

    /// Get a snapshot from the l1.
    pub async fn snapshot(&self) -> L1Snapshot {
        self.state.lock().await.snapshot
    }

    /// Wait until the highest L1 block number reaches at least `number`.
    ///
    /// This function does not return any information about the block, since the block is not
    /// necessarily finalized when it returns. It is only used to guarantee that some block at
    /// height `number` exists, possibly in the unsafe part of the L1 chain.
    pub async fn wait_for_block(&self, number: u64) {
        loop {
            // Subscribe to events before checking the current state, to ensure we don't miss a
            // relevant event.
            let mut events = self.receiver.activate_cloned();

            // Check if the block we are waiting for already exists.
            {
                let state = self.state.lock().await;
                if state.snapshot.head >= number {
                    return;
                }
                tracing::info!(number, head = state.snapshot.head, "waiting for l1 block");
            }

            // Wait for the block.
            while let Some(event) = events.next().await {
                let L1Event::NewHead { head } = event else {
                    continue;
                };
                if head >= number {
                    tracing::info!(number, head, "got L1 block");
                    return;
                }
                tracing::debug!(number, head, "waiting for L1 block");
            }

            // This should not happen: the event stream ended. All we can do is try again.
            tracing::warn!(number, "L1 event stream ended unexpectedly; retry");
            self.retry_delay().await;
        }
    }

    /// Get information about the given block.
    ///
    /// If the desired block number is not finalized yet, this function will block until it becomes
    /// finalized.
    pub async fn wait_for_finalized_block(&self, number: u64) -> L1BlockInfo {
        loop {
            // Subscribe to events before checking the current state, to ensure we don't miss a relevant
            // event.
            let mut events = self.receiver.activate_cloned();

            // Check if the block we are waiting for already exists.
            {
                let state = self.state.lock().await;
                if let Some(finalized) = state.snapshot.finalized {
                    if finalized.number >= number {
                        return self.get_finalized_block(state, number).await.1;
                    }
                    tracing::info!(
                        number,
                        finalized = ?state.snapshot.finalized,
                        "waiting for l1 finalized block",
                    );
                };
            }

            // Wait for the block.
            while let Some(event) = events.next().await {
                let L1Event::NewFinalized { finalized } = event else {
                    continue;
                };
                if finalized.number >= number {
                    tracing::info!(number, ?finalized, "got finalized L1 block");
                    return self
                        .get_finalized_block(self.state.lock().await, number)
                        .await
                        .1;
                }
                tracing::debug!(number, ?finalized, "waiting for finalized L1 block");
            }

            // This should not happen: the event stream ended. All we can do is try again.
            tracing::warn!(number, "L1 event stream ended unexpectedly; retry",);
            self.retry_delay().await;
        }
    }

    /// Get information about the first finalized block with timestamp greater than or equal
    /// `timestamp`.
    pub async fn wait_for_finalized_block_with_timestamp(&self, timestamp: U256) -> L1BlockInfo {
        // Wait until the finalized block has timestamp >= `timestamp`.
        let (mut state, mut block) = 'outer: loop {
            // Subscribe to events before checking the current state, to ensure we don't miss a
            // relevant event.
            let mut events = self.receiver.activate_cloned();

            // Check if the block we are waiting for already exists.
            {
                let state = self.state.lock().await;
                if let Some(finalized) = state.snapshot.finalized {
                    if finalized.timestamp >= timestamp {
                        break 'outer (state, finalized);
                    }
                }
                tracing::info!(
                    %timestamp,
                    finalized = ?state.snapshot.finalized,
                    "waiting for L1 finalized block",
                );
            }

            // Wait for the block.
            while let Some(event) = events.next().await {
                let L1Event::NewFinalized { finalized } = event else {
                    continue;
                };
                if finalized.timestamp >= timestamp {
                    tracing::info!(%timestamp, ?finalized, "got finalized block");
                    break 'outer (self.state.lock().await, finalized);
                }
                tracing::debug!(%timestamp, ?finalized, "waiting for L1 finalized block");
            }

            // This should not happen: the event stream ended. All we can do is try again.
            tracing::warn!(%timestamp, "L1 event stream ended unexpectedly; retry",);
            self.retry_delay().await;
        };

        // It is possible there is some earlier block that also has the proper timestamp. Work
        // backwards until we find the true earliest block.
        loop {
            let (state_lock, parent) = self.get_finalized_block(state, block.number - 1).await;
            if parent.timestamp < timestamp {
                return block;
            }
            state = state_lock;
            block = parent;
        }
    }

    async fn get_finalized_block<'a>(
        &'a self,
        mut state: MutexGuard<'a, L1State>,
        number: u64,
    ) -> (MutexGuard<'a, L1State>, L1BlockInfo) {
        // Try to get the block from the finalized block cache.
        assert!(
            state.snapshot.finalized.is_some()
                && number <= state.snapshot.finalized.unwrap().number,
            "requesting a finalized block {number} that isn't finalized; snapshot: {:?}",
            state.snapshot,
        );
        if let Some(block) = state.finalized.get(&number) {
            let block = *block;
            return (state, block);
        }
        drop(state);

        // If not in cache, fetch the block from the L1 provider.
        let block = loop {
            let block = match self.provider.get_block(number).await {
                Ok(Some(block)) => block,
                Ok(None) => {
                    tracing::warn!(
                        number,
                        "provider error: finalized L1 block should always be available"
                    );
                    self.retry_delay().await;
                    continue;
                }
                Err(err) => {
                    tracing::warn!(number, "failed to get finalized L1 block: {err:#}");
                    self.retry_delay().await;
                    continue;
                }
            };
            let Some(hash) = block.hash else {
                tracing::warn!(number, ?block, "finalized L1 block has no hash");
                self.retry_delay().await;
                continue;
            };
            break L1BlockInfo {
                number,
                hash,
                timestamp: block.timestamp,
            };
        };

        // After fetching, add the block to the cache.
        let mut state = self.state.lock().await;
        state.put_finalized(block);
        (state, block)
    }

    /// Divide the range `start..=end` into chunks of size
    /// `events_max_block_range`.
    fn chunky(&self, start: u64, end: u64) -> FromFn<impl FnMut() -> Option<(u64, u64)>> {
        let mut start = start;
        let chunk_size = self.options().l1_events_max_block_range;
        std::iter::from_fn(move || {
            let chunk_end = min(start + chunk_size - 1, end);
            if chunk_end < start {
                return None;
            }

            let chunk = (start, chunk_end);
            start = chunk_end + 1;
            Some(chunk)
        })
    }

    /// Divide the range `start..=end` into chunks of size
    /// `events_max_block_range`.
    fn chunky2(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        let chunks: Vec<u64> = (start..=end).collect();
        let tups: Vec<(u64, u64)> = chunks
            .chunks(3)
            .map(|s| {
                // should never be empty
                s.first().cloned().zip(s.last().cloned()).unwrap()
            })
            .collect();

        tups
    }

    /// Get fee info for each `Deposit` occurring between `prev`
    /// and `new`. Returns `Vec<FeeInfo>`
    pub async fn get_finalized_deposits(
        &self,
        fee_contract_address: Address,
        prev_finalized: Option<u64>,
        new_finalized: u64,
    ) -> Vec<FeeInfo> {
        // No new blocks have been finalized, therefore there are no
        // new deposits.
        if prev_finalized >= Some(new_finalized) {
            return vec![];
        }

        let opt = self.options();

        // `prev` should have already been processed unless we
        // haven't processed *any* blocks yet.
        let prev = prev_finalized.map(|prev| prev + 1).unwrap_or(0);

        // Divide the range `prev_finalized..=new_finalized` into chunks of size
        // `events_max_block_range`.
        let chunks = self.chunky(prev, new_finalized);

        // Fetch events for each chunk.
        let events = stream::iter(chunks).then(|(from, to)| {
            let retry_delay = opt.l1_retry_delay;
            let fee_contract = FeeContract::new(fee_contract_address, self.provider.clone());
            async move {
                tracing::debug!(from, to, "fetch events in range");

                // query for deposit events, loop until successful.
                loop {
                    match fee_contract
                        .deposit_filter()
                        .address(fee_contract.address().into())
                        .from_block(from)
                        .to_block(to)
                        .query()
                        .await
                    {
                        Ok(events) => break stream::iter(events),
                        Err(err) => {
                            tracing::warn!(from, to, %err, "Fee L1Event Error");
                            sleep(retry_delay).await;
                        }
                    }
                }
            }
        });
        events.flatten().map(FeeInfo::from).collect().await
    }

    // /// Get `StakeTable` at block height. If unavailable in local cache, poll the l1.
    pub async fn get_stake_table(
        &self,
        contract_address: Address,
        block: u64,
    ) -> Option<StakeTables> {
        let opt = self.options();
        let retry_delay = opt.l1_retry_delay;
        let max_block_range = opt.l1_events_max_block_range;
        let mut lock = self.state.lock().await;

        if let Some(st) = lock.stake.get(&block) {
            Some(st.clone())
        } else {
            let last_head = lock.snapshot.head;

            let chunks = self.chunky2(last_head, block);
            let contract = PermissionedStakeTable::new(contract_address, self.provider.clone());

            let mut events: Vec<StakersUpdatedFilter> = Vec::new();
            for (from, to) in chunks {
                tracing::debug!(from, to, "fetch stake table events in range");
                loop {
                    match contract
                        .stakers_updated_filter()
                        .from_block(from)
                        .to_block(to)
                        .query()
                        .await
                    {
                        Ok(e) => {
                            for event in e {
                                events.push(event)
                            }
                            break;
                        }
                        Err(err) => {
                            tracing::warn!(from, to, %err, "Stake Table L1Event Error");
                            sleep(retry_delay).await;
                        }
                    }
                }
            }
            Some(StakeTables::from_l1_events(events))
        }
    }

    fn options(&self) -> &L1ClientOptions {
        (*self.provider).as_ref().options()
    }

    fn metrics(&self) -> &L1ClientMetrics {
        (*self.provider).as_ref().metrics()
    }

    async fn retry_delay(&self) {
        sleep(self.options().l1_retry_delay).await;
    }
}

impl L1State {
    fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            snapshot: Default::default(),
            finalized: LruCache::new(cache_size),
            stake: LruCache::new(cache_size),
        }
    }

    fn put_finalized(&mut self, info: L1BlockInfo) {
        assert!(
            self.snapshot.finalized.is_some()
                && info.number <= self.snapshot.finalized.unwrap().number,
            "inserting a finalized block {info:?} that isn't finalized; snapshot: {:?}",
            self.snapshot,
        );

        if let Some((old_number, old_info)) = self.finalized.push(info.number, info) {
            if old_number == info.number {
                tracing::error!(
                    ?old_info,
                    ?info,
                    "got different info for the same finalized height; something has gone very wrong with the L1",
                );
            }
        }
    }
}

async fn get_finalized_block(
    rpc: &Provider<MultiRpcClient>,
) -> anyhow::Result<Option<L1BlockInfo>> {
    let Some(block) = rpc.get_block(BlockNumber::Finalized).await? else {
        // This can happen in rare cases where the L1 chain is very young and has not finalized a
        // block yet. This is more common in testing and demo environments. In any case, we proceed
        // with a null L1 block rather than wait for the L1 to finalize a block, which can take a
        // long time.
        tracing::warn!("no finalized block yet");
        return Ok(None);
    };

    // The number and hash _should_ both exists: they exist unless the block is pending, and the
    // finalized block cannot be pending, unless there has been a catastrophic reorg of the
    // finalized prefix of the L1 chain.
    let number = block.number.context("finalized block has no number")?;
    let hash = block.hash.context("finalized block has no hash")?;

    Ok(Some(L1BlockInfo {
        number: number.as_u64(),
        timestamp: block.timestamp,
        hash,
    }))
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use contract_bindings::{
        fee_contract::FeeContract,
        permissioned_stake_table::{NodeInfo, PermissionedStakeTable},
    };
    use ethers::{
        prelude::{LocalWallet, Signer, SignerMiddleware, H160, U64},
        providers::Http,
        utils::{hex, parse_ether, Anvil, AnvilInstance},
    };
    use hotshot_contract_adapter::stake_table::NodeInfoJf;
    use portpicker::pick_unused_port;
    use sequencer_utils::test_utils::setup_test;
    use std::time::Duration;
    use time::OffsetDateTime;

    use super::*;

    async fn new_l1_client(anvil: &AnvilInstance, ws: bool) -> L1Client {
        let client = L1ClientOptions {
            l1_events_max_block_range: 1,
            l1_polling_interval: Duration::from_secs(1),
            subscription_timeout: Duration::from_secs(5),
            l1_ws_provider: if ws {
                Some(vec![anvil.ws_endpoint().parse().unwrap()])
            } else {
                None
            },
            ..Default::default()
        }
        .connect([anvil.endpoint().parse().unwrap()]);

        client.spawn_tasks().await;
        client
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_finalized_deposits() -> anyhow::Result<()> {
        setup_test();

        // how many deposits will we make
        let deposits = 5;
        let deploy_txn_count = 2;

        let anvil = Anvil::new().spawn();
        let wallet_address = anvil.addresses().first().cloned().unwrap();
        let l1_client = new_l1_client(&anvil, false).await;
        let wallet: LocalWallet = anvil.keys()[0].clone().into();

        // In order to deposit we need a provider that can sign.
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client =
            SignerMiddleware::new(provider.clone(), wallet.with_chain_id(anvil.chain_id()));
        let client = Arc::new(client);

        // Initialize a contract with some deposits

        // deploy the fee contract
        let fee_contract =
            contract_bindings::fee_contract::FeeContract::deploy(Arc::new(client.clone()), ())
                .unwrap()
                .send()
                .await?;

        // prepare the initialization data to be sent with the proxy when the proxy is deployed
        let initialize_data = fee_contract
            .initialize(wallet_address) // Here, you simulate the call to get the transaction data without actually sending it.
            .calldata()
            .expect("Failed to encode initialization data");

        // deploy the proxy contract and set the implementation address as the address of the fee contract and send the initialization data
        let proxy_contract = contract_bindings::erc1967_proxy::ERC1967Proxy::deploy(
            client.clone(),
            (fee_contract.address(), initialize_data),
        )
        .unwrap()
        .send()
        .await?;

        // cast the proxy to be of type fee contract so that we can interact with the implementation methods via the proxy
        let fee_contract_proxy = FeeContract::new(proxy_contract.address(), client.clone());

        // confirm that the owner of the contract is the address that was sent as part of the initialization data
        let owner = fee_contract_proxy.owner().await;
        assert_eq!(owner.unwrap(), wallet_address.clone());

        // confirm that the implementation address is the address of the fee contract deployed above
        // using the implementation slot, 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc, which is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1
        let hex_bytes =
            hex::decode("360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")
                .expect("Failed to decode hex string");
        let implementation_slot = ethers::types::H256::from_slice(&hex_bytes);
        let storage = provider
            .clone()
            .get_storage_at(
                fee_contract_proxy.clone().address(),
                implementation_slot,
                None,
            )
            .await?;
        let implementation_address = H160::from_slice(&storage[12..]);
        assert_eq!(fee_contract.clone().address(), implementation_address);

        // Anvil will produce a bock for every transaction.
        let head = l1_client.provider.get_block_number().await.unwrap();
        // there are two transactions, deploying the implementation contract, FeeContract, and deploying the proxy contract
        assert_eq!(deploy_txn_count, head.as_u64());

        // make some deposits.
        for n in 1..=deposits {
            // Varied amounts are less boring.
            let amount = n as f32 / 10.0;
            let receipt = fee_contract_proxy
                .deposit(wallet_address)
                .value(parse_ether(amount).unwrap())
                .send()
                .await?
                .await?;

            // Successful transactions have `status` of `1`.
            assert_eq!(Some(U64::from(1)), receipt.clone().unwrap().status);
        }

        let head = l1_client.provider.get_block_number().await.unwrap();
        // Anvil will produce a block for every transaction.
        assert_eq!(deposits + deploy_txn_count, head.as_u64());

        // Use non-signing `L1Client` to retrieve data.
        let l1_client = new_l1_client(&anvil, false).await;
        // Set prev deposits to `None` so `Filter` will start at block
        // 0. The test would also succeed if we pass `0` (b/c first
        // block did not deposit).
        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                None,
                deposits + deploy_txn_count,
            )
            .await;

        assert_eq!(deposits as usize, pending.len(), "{pending:?}");
        assert_eq!(&wallet_address, &pending[0].account().into());
        assert_eq!(
            U256::from(1500000000000000000u64),
            pending.iter().fold(U256::from(0), |total, info| total
                .add(U256::from(info.amount())))
        );

        // check a few more cases
        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(0),
                deposits + deploy_txn_count,
            )
            .await;
        assert_eq!(deposits as usize, pending.len());

        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(0), 0)
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(0), 1)
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(deploy_txn_count),
                deploy_txn_count,
            )
            .await;
        assert_eq!(0, pending.len());

        let pending = l1_client
            .get_finalized_deposits(
                fee_contract_proxy.address(),
                Some(deploy_txn_count),
                deploy_txn_count + 1,
            )
            .await;
        assert_eq!(1, pending.len());

        // what happens if `new_finalized` is `0`?
        let pending = l1_client
            .get_finalized_deposits(fee_contract_proxy.address(), Some(deploy_txn_count), 0)
            .await;
        assert_eq!(0, pending.len());

        Ok(())
    }

    async fn test_wait_for_finalized_block_helper(ws: bool) {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = new_l1_client(&anvil, ws).await;
        let provider = &l1_client.provider;

        // Wait for a block 10 blocks in the future.
        let block_height = provider.get_block_number().await.unwrap().as_u64();
        let block = l1_client.wait_for_finalized_block(block_height + 10).await;
        assert_eq!(block.number, block_height + 10);

        // Compare against underlying provider.
        let true_block = provider
            .get_block(block_height + 10)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(block.timestamp, true_block.timestamp);
        assert_eq!(block.hash, true_block.hash.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_finalized_block_ws() {
        test_wait_for_finalized_block_helper(true).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_finalized_block_http() {
        test_wait_for_finalized_block_helper(false).await
    }

    async fn test_wait_for_finalized_block_by_timestamp_helper(ws: bool) {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = new_l1_client(&anvil, ws).await;
        let provider = &l1_client.provider;

        // Wait for a block 10 blocks in the future.
        let timestamp = U256::from(OffsetDateTime::now_utc().unix_timestamp() as u64) + 10;
        let block = l1_client
            .wait_for_finalized_block_with_timestamp(timestamp)
            .await;
        assert!(
            block.timestamp >= timestamp,
            "wait_for_finalized_block_with_timestamp({timestamp}) returned too early a block: {block:?}",
        );
        let parent = provider.get_block(block.number - 1).await.unwrap().unwrap();
        assert!(
            parent.timestamp < timestamp,
            "wait_for_finalized_block_with_timestamp({timestamp}) did not return the earliest possible block: returned {block:?}, but earlier block {parent:?} has an acceptable timestamp too",
        );

        // Compare against underlying provider.
        let true_block = provider.get_block(block.number).await.unwrap().unwrap();
        assert_eq!(block.timestamp, true_block.timestamp);
        assert_eq!(block.hash, true_block.hash.unwrap());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_finalized_block_by_timestamp_ws() {
        test_wait_for_finalized_block_by_timestamp_helper(true).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_finalized_block_by_timestamp_http() {
        test_wait_for_finalized_block_by_timestamp_helper(false).await
    }

    async fn test_wait_for_block_helper(ws: bool) {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let l1_client = new_l1_client(&anvil, ws).await;
        let provider = &l1_client.provider;

        // Wait for a block 10 blocks in the future.
        let block_height = provider.get_block_number().await.unwrap().as_u64();
        l1_client.wait_for_block(block_height + 10).await;

        let new_block_height = provider.get_block_number().await.unwrap().as_u64();
        assert!(
            new_block_height >= block_height + 10,
            "wait_for_block returned too early; initial height = {block_height}, new height = {new_block_height}",
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_block_ws() {
        test_wait_for_block_helper(true).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wait_for_block_http() {
        test_wait_for_block_helper(false).await
    }

    async fn test_reconnect_update_task_helper(ws: bool) {
        setup_test();

        let port = pick_unused_port().unwrap();
        let anvil = Anvil::new().block_time(1u32).port(port).spawn();
        let client = new_l1_client(&anvil, ws).await;

        let initial_state = client.snapshot().await;
        tracing::info!(?initial_state, "initial state");

        // Check the state is updating.
        let mut retry = 0;
        let updated_state = loop {
            assert!(retry < 10, "state did not update in time");

            let updated_state = client.snapshot().await;
            if updated_state.head > initial_state.head {
                break updated_state;
            }
            tracing::info!(retry, "waiting for state update");
            sleep(Duration::from_secs(1)).await;
            retry += 1;
        };
        tracing::info!(?updated_state, "state updated");

        // Disconnect the WebSocket and reconnect it. Technically this spawns a whole new Anvil
        // chain, but for the purposes of this test it should look to the client like an L1 server
        // closing a WebSocket connection.
        drop(anvil);

        // Let the connection stay down for a little while: Ethers internally tries to reconnect,
        // and starting up to fast again might hit that and cause a false positive. The problem is,
        // Ethers doesn't try very hard, and if we wait a bit, we will test the worst possible case
        // where the internal retry logic gives up and just kills the whole provider.
        tracing::info!("sleep 5");
        sleep(Duration::from_secs(5)).await;

        // Once a connection is reestablished, the state will eventually start to update again.
        tracing::info!("restarting L1");
        let _anvil = Anvil::new().block_time(1u32).port(port).spawn();

        let mut retry = 0;
        let final_state = loop {
            assert!(retry < 5, "state did not update in time");

            let final_state = client.snapshot().await;
            if final_state.head > updated_state.head {
                break final_state;
            }
            tracing::info!(retry, "waiting for state update");
            sleep(Duration::from_secs(1)).await;
            retry += 1;
        };
        tracing::info!(?final_state, "state updated");
    }

    #[tokio::test]
    async fn test_chunky() {
        let anvil = Anvil::new().spawn();
        let opt = L1ClientOptions {
            l1_events_max_block_range: 3,
            ..Default::default()
        };
        let l1_client = opt.connect(vec![anvil.endpoint().parse().unwrap()]);

        let chunks = l1_client.chunky(3, 10);
        let tups = stream::iter(chunks).collect::<Vec<_>>().await;

        assert_eq![vec![(3, 5), (6, 8), (9, 10)], tups];

        let tups = l1_client.chunky2(3, 10);
        assert_eq![vec![(3, 5), (6, 8), (9, 10)], tups];
    }

    #[tokio::test]
    async fn test_fetch_stake_table() -> anyhow::Result<()> {
        setup_test();

        let anvil = Anvil::new().spawn();
        let l1_client = new_l1_client(&anvil, false).await;
        let wallet: LocalWallet = anvil.keys()[0].clone().into();

        // In order to deposit we need a provider that can sign.
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client =
            SignerMiddleware::new(provider.clone(), wallet.with_chain_id(anvil.chain_id()));
        let client = Arc::new(client);

        // deploy the stake_table contract
        let stake_table_contract =
            PermissionedStakeTable::deploy(client.clone(), Vec::<NodeInfo>::new())
                .unwrap()
                .send()
                .await?;

        let address = stake_table_contract.address();

        let mut rng = rand::thread_rng();
        let node = NodeInfoJf::random(&mut rng);

        let new_nodes: Vec<NodeInfo> = vec![node.into()];
        let updater = stake_table_contract.update(vec![], new_nodes);
        updater.send().await?.await?;

        let block = client.get_block(BlockNumber::Latest).await?.unwrap();
        let nodes = l1_client
            .get_stake_table(address, block.number.unwrap().as_u64())
            .await
            .unwrap();

        let result = nodes.stake_table.0[0].clone();
        assert_eq!(result.stake_amount.as_u64(), 1);
        Ok(())
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_reconnect_update_task_ws() {
        test_reconnect_update_task_helper(true).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_reconnect_update_task_http() {
        test_reconnect_update_task_helper(false).await
    }

    async fn test_failover_update_task_helper(ws: bool) {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();

        // Create an L1 client with fake providers, and check that the state is still updated after
        // it correctly fails over to the real providers.
        let client = L1ClientOptions {
            l1_polling_interval: Duration::from_secs(1),
            // Use a very long subscription timeout, so that we only succeed by triggering a
            // failover.
            subscription_timeout: Duration::from_secs(1000),
            l1_ws_provider: if ws {
                Some(vec![
                    "ws://notarealurl:1234".parse().unwrap(),
                    anvil.ws_endpoint().parse().unwrap(),
                ])
            } else {
                None
            },
            ..Default::default()
        }
        .connect([
            "http://notarealurl:1234".parse().unwrap(),
            anvil.endpoint().parse().unwrap(),
        ]);

        client.spawn_tasks().await;

        let initial_state = client.snapshot().await;
        tracing::info!(?initial_state, "initial state");

        // Check the state is updating.
        let mut retry = 0;
        let updated_state = loop {
            assert!(retry < 10, "state did not update in time");

            let updated_state = client.snapshot().await;
            if updated_state.head > initial_state.head {
                break updated_state;
            }
            tracing::info!(retry, "waiting for state update");
            sleep(Duration::from_secs(1)).await;
            retry += 1;
        };
        tracing::info!(?updated_state, "state updated");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_failover_update_task_ws() {
        test_failover_update_task_helper(true).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_failover_update_task_http() {
        test_failover_update_task_helper(false).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_failover_consecutive_failures() {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let provider = Provider::new(MultiRpcClient::new(
            L1ClientOptions {
                l1_polling_interval: Duration::from_secs(1),
                // Set a very short tolerance for frequent failovers, so that we will only
                // successfully trigger a failover via the consecutive failover rule.
                l1_frequent_failure_tolerance: Duration::from_millis(0),
                l1_consecutive_failure_tolerance: 3,
                ..Default::default()
            },
            [
                "http://notarealurl:1234".parse().unwrap(),
                anvil.endpoint().parse().unwrap(),
            ],
        ));

        // Make just enough failed requests not to trigger a failover.
        for _ in 0..2 {
            let failover = provider.as_ref().next_failover();
            provider.get_block_number().await.unwrap_err();
            assert!(failover.now_or_never().is_none());
            assert_eq!(provider.as_ref().status.read().await.client, 0);
        }

        // The final request triggers failover.
        let failover = provider.as_ref().next_failover();
        provider.get_block_number().await.unwrap_err();
        assert!(failover.now_or_never().is_some());
        assert_eq!(provider.as_ref().status.read().await.client, 1);

        // Now requests succeed.
        provider.get_block_number().await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_failover_frequent_failures() {
        setup_test();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let provider = Provider::new(MultiRpcClient::new(
            L1ClientOptions {
                l1_polling_interval: Duration::from_secs(1),
                l1_frequent_failure_tolerance: Duration::from_millis(100),
                ..Default::default()
            },
            [
                "http://notarealurl:1234".parse().unwrap(),
                anvil.endpoint().parse().unwrap(),
            ],
        ));

        // Two failed requests that are not within the tolerance window do not trigger a failover.
        let failover = provider.as_ref().next_failover();
        provider.get_block_number().await.unwrap_err();
        sleep(Duration::from_secs(1)).await;
        provider.get_block_number().await.unwrap_err();

        // Check that we didn't fail over.
        assert!(failover.now_or_never().is_none());
        assert_eq!(provider.as_ref().status.read().await.client, 0);

        // Reset the window.
        sleep(Duration::from_secs(1)).await;

        // Two failed requests in a row trigger failover.
        let failover = provider.as_ref().next_failover();
        provider.get_block_number().await.unwrap_err();
        provider.get_block_number().await.unwrap_err();
        provider.get_block_number().await.unwrap();
        assert!(failover.now_or_never().is_some());
        assert_eq!(provider.as_ref().status.read().await.client, 1);
    }
}
