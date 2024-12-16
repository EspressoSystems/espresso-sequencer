use anyhow::{bail, Context};
use async_trait::async_trait;
use clap::Parser;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::{
    fee_contract::FeeContract, permissioned_stake_table::PermissionedStakeTable,
};
use ethers::{
    prelude::{Address, BlockNumber, Middleware, Provider, H256, U256},
    providers::{Http, JsonRpcClient, ProviderError, PubsubClient, Ws, WsClientError},
};
use futures::{
    future::Future,
    stream::{self, StreamExt},
};
use hotshot_types::{data::EpochNumber, traits::metrics::Metrics};
use lru::LruCache;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    cmp::{min, Ordering},
    collections::BTreeMap,
    fmt::Debug,
    num::NonZeroUsize,
    sync::{self, Arc},
    time::Duration,
};
use tokio::{
    spawn,
    sync::{Mutex, MutexGuard, RwLock},
    time::sleep,
};
use tracing::Instrument;
use url::Url;

use super::{L1BlockInfo, L1ClientMetrics, L1State, L1UpdateTask, RpcClient};
use crate::{
    v0_3::StakeTables, FeeInfo, L1Client, L1ClientOptions, L1Event, L1ReconnectTask, L1Snapshot,
};

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

impl RpcClient {
    fn http(url: Url, metrics: Arc<L1ClientMetrics>) -> Self {
        Self::Http {
            conn: Http::new(url),
            metrics,
        }
    }

    async fn ws(
        url: Url,
        metrics: Arc<L1ClientMetrics>,
        retry_delay: Duration,
    ) -> anyhow::Result<Self> {
        Ok(Self::Ws {
            conn: Arc::new(RwLock::new(Ws::connect(url.clone()).await?)),
            reconnect: Default::default(),
            retry_delay,
            url,
            metrics,
        })
    }

    async fn shut_down(&self) {
        if let Self::Ws { reconnect, .. } = self {
            *reconnect.lock().await = L1ReconnectTask::Cancelled;
        }
    }

    fn metrics(&self) -> &Arc<L1ClientMetrics> {
        match self {
            Self::Http { metrics, .. } => metrics,
            Self::Ws { metrics, .. } => metrics,
        }
    }
}

#[async_trait]
impl JsonRpcClient for RpcClient {
    type Error = ProviderError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let res = match self {
            Self::Http { conn, .. } => conn
                .request(method, params)
                .await
                .inspect_err(|err| tracing::warn!(method, "L1 RPC error: {err:#}"))?,
            Self::Ws {
                conn,
                reconnect,
                url,
                retry_delay,
                metrics,
            } => {
                let conn_guard = conn
                    .try_read()
                    // We only lock the connection exclusively when we are resetting it, so if it is
                    // locked that means it was closed and is still being reset. There is no point
                    // in trying a request with a closed connection.
                    .map_err(|_| {
                        ProviderError::CustomError("connection closed; reset in progress".into())
                    })?;
                match conn_guard.request(method, params).await {
                    Ok(res) => res,
                    Err(err @ WsClientError::UnexpectedClose) => {
                        // If the WebSocket connection is closed, try to reopen it.
                        if let Ok(mut reconnect_guard) = reconnect.try_lock() {
                            if matches!(*reconnect_guard, L1ReconnectTask::Idle) {
                                // No one is currently resetting this connection, so it's up to us.
                                metrics.ws_reconnects.add(1);
                                let conn = conn.clone();
                                let reconnect = reconnect.clone();
                                let url = url.clone();
                                let retry_delay = *retry_delay;
                                let span = tracing::warn_span!("ws resetter");
                                *reconnect_guard = L1ReconnectTask::Reconnecting(spawn(
                                    async move {
                                        tracing::warn!("ws connection closed, trying to reset");
                                        let new_conn = loop {
                                            match Ws::connect(url.clone()).await {
                                                Ok(conn) => break conn,
                                                Err(err) => {
                                                    tracing::warn!("failed to reconnect: {err:#}");
                                                    sleep(retry_delay).await;
                                                }
                                            }
                                        };

                                        // Reset the connection, and set the reconnect task back to
                                        // idle, so that the connection can be reset again if
                                        // needed.
                                        let mut conn = conn.write().await;
                                        let mut reconnect = reconnect.lock().await;
                                        *conn = new_conn;
                                        if !matches!(*reconnect, L1ReconnectTask::Cancelled) {
                                            *reconnect = L1ReconnectTask::Idle;
                                        }

                                        tracing::info!("ws connection successfully reestablished");
                                    }
                                    .instrument(span),
                                ));
                            }
                        } else {
                            // If we fail to get a lock on the reconnect task, it can only mean one
                            // of two things:
                            // * someone else is already preparing to reset the connection
                            // * the entire L1 client is being shut down
                            // In either case, we don't want/need to reset the connection ourselves,
                            // so nothing to do here.
                        }
                        Err(err)?
                    }
                    Err(err) => {
                        tracing::warn!(method, "L1 RPC error: {err:#}");
                        Err(err)?
                    }
                }
            }
        };
        Ok(res)
    }
}

impl PubsubClient for RpcClient {
    type NotificationStream = <Ws as PubsubClient>::NotificationStream;

    fn subscribe<T>(&self, id: T) -> Result<Self::NotificationStream, Self::Error>
    where
        T: Into<U256>,
    {
        match self {
            Self::Http { .. } => Err(ProviderError::CustomError(
                "subscriptions not supported with HTTP client".into(),
            )),
            Self::Ws { conn, .. } => Ok(conn
                .try_read()
                // We only lock the connection exclusively when we are resetting it, so if it is
                // locked that means it was closed and is still being reset. There is no point
                // in trying to subscribe with a closed connection.
                .map_err(|_| {
                    ProviderError::CustomError("connection closed; reset in progress".into())
                })?
                .subscribe(id)?),
        }
    }

    fn unsubscribe<T>(&self, id: T) -> Result<(), Self::Error>
    where
        T: Into<U256>,
    {
        match self {
            Self::Http { .. } => Err(ProviderError::CustomError(
                "subscriptions not supported with HTTP client".into(),
            )),
            Self::Ws { conn, .. } => Ok(conn
                .try_read()
                // We only lock the connection exclusively when we are resetting it, so if it is
                // locked that means it was closed and is still being reset. There is no point
                // in doing anything with a closed connection.
                .map_err(|_| {
                    ProviderError::CustomError("connection closed; reset in progress".into())
                })?
                .unsubscribe(id)?),
        }
    }
}

impl Drop for L1ReconnectTask {
    fn drop(&mut self) {
        if let Self::Reconnecting(task) = self {
            tracing::info!("cancelling L1 reconnect task");
            task.abort();
        }
    }
}

impl Drop for L1UpdateTask {
    fn drop(&mut self) {
        if let Some(task) = self.0.get_mut().take() {
            task.abort();
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

    /// Instantiate an `L1Client` for a given `Url`.
    ///
    /// The type of the JSON-RPC client is inferred from the scheme of the URL. Supported schemes
    /// are `ws`, `wss`, `http`, and `https`.
    pub async fn connect(self, url: Url) -> anyhow::Result<L1Client> {
        match url.scheme() {
            "http" | "https" => Ok(self.http(url)),
            "ws" | "wss" => self.ws(url).await,
            scheme => bail!("unsupported JSON-RPC protocol {scheme}"),
        }
    }

    /// Synchronous, infallible version of `connect` for HTTP clients.
    ///
    /// `url` must have a scheme `http` or `https`.
    pub fn http(self, url: Url) -> L1Client {
        let metrics = self.create_metrics();
        L1Client::with_provider(self, Provider::new(RpcClient::http(url, metrics)))
    }

    /// Construct a new WebSockets client.
    ///
    /// `url` must have a scheme `ws` or `wss`.
    pub async fn ws(self, url: Url) -> anyhow::Result<L1Client> {
        let metrics = self.create_metrics();
        let retry_delay = self.l1_retry_delay;
        Ok(L1Client::with_provider(
            self,
            Provider::new(RpcClient::ws(url, metrics, retry_delay).await?),
        ))
    }

    fn create_metrics(&self) -> Arc<L1ClientMetrics> {
        Arc::new(L1ClientMetrics::new(&**self.metrics))
    }
}

impl L1ClientMetrics {
    fn new(metrics: &(impl Metrics + ?Sized)) -> Self {
        Self {
            head: metrics.create_gauge("head".into(), None),
            finalized: metrics.create_gauge("finalized".into(), None),
            ws_reconnects: metrics.create_counter("ws_reconnects".into(), None),
            stream_reconnects: metrics.create_counter("stream_reconnects".into(), None),
        }
    }
}

impl L1Client {
    fn with_provider(opt: L1ClientOptions, mut provider: Provider<RpcClient>) -> Self {
        let (sender, mut receiver) = async_broadcast::broadcast(opt.l1_events_channel_capacity);
        receiver.set_await_active(false);
        receiver.set_overflow(true);

        provider.set_interval(opt.l1_polling_interval);
        Self {
            retry_delay: opt.l1_retry_delay,
            provider: Arc::new(provider),
            events_max_block_range: opt.l1_events_max_block_range,
            state: Arc::new(Mutex::new(L1State::new(opt.l1_blocks_cache_size))),
            stake_table_state: Arc::new(sync::RwLock::new(BTreeMap::new())),
            sender,
            receiver: receiver.deactivate(),
            update_task: Default::default(),
        }
    }

    /// Construct a new L1 client with the default options.
    pub async fn new(url: Url) -> anyhow::Result<Self> {
        L1ClientOptions::default().connect(url).await
    }

    /// Construct a new WebSockets client with the default options.
    ///
    /// `url` must have a scheme `ws` or `wss`.
    pub async fn ws(url: Url) -> anyhow::Result<Self> {
        L1ClientOptions::default().ws(url).await
    }

    /// Synchronous, infallible version of `new` for HTTP clients.
    ///
    /// `url` must have a scheme `http` or `https`.
    pub fn http(url: Url) -> Self {
        L1ClientOptions::default().http(url)
    }

    /// Start the background tasks which keep the L1 client up to date.
    pub async fn spawn_tasks(&self) {
        let mut update_task = self.update_task.0.lock().await;
        if update_task.is_none() {
            *update_task = Some(spawn(self.update_loop()));
        }
    }
    pub async fn update_membership(
        &self,
        contract: Address,
        l1_block_number: u64,
        epoch: EpochNumber,
    ) {
        let retry_delay = self.retry_delay;
        let state = self.stake_table_state.clone();

        let span = tracing::warn_span!("L1 client memberships update");

        async move {
            loop {
                match self.get_stake_table(contract, l1_block_number).await {
                    Err(err) => {
                        tracing::warn!(
                            ?epoch,
                            ?l1_block_number,
                            "error fetching stake table from l1. err {err}"
                        );
                    }
                    Ok(stake_tables) => {
                        let mut state = state.write().unwrap();
                        let _ = state.insert(epoch, stake_tables);
                    }
                }

                sleep(retry_delay).await;
            }
        }
        .instrument(span)
        .await
    }

    /// Shut down background tasks associated with this L1 client.
    ///
    /// The L1 client will still be usable, but will stop updating until [`start`](Self::start) is
    /// called again.
    pub async fn shut_down_tasks(&self) {
        if let Some(update_task) = self.update_task.0.lock().await.take() {
            update_task.abort();
        }
        (*self.provider).as_ref().shut_down().await;
    }

    pub fn provider(&self) -> &impl Middleware<Error: 'static> {
        &self.provider
    }

    fn update_loop(&self) -> impl Future<Output = ()> {
        let rpc = self.provider.clone();
        let retry_delay = self.retry_delay;
        let state = self.state.clone();
        let sender = self.sender.clone();
        let metrics = (*rpc).as_ref().metrics().clone();

        let span = tracing::warn_span!("L1 client update");
        async move {
            loop {
                // Subscribe to new blocks. This task cannot fail; retry until we succeed.
                let mut block_stream = loop {
                    let res = match (*rpc).as_ref() {
                        RpcClient::Ws { .. } => rpc.subscribe_blocks().await.map(StreamExt::boxed),
                        RpcClient::Http { .. } => rpc
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
                            .boxed()),
                    };
                    match res {
                        Ok(stream) => break stream,
                        Err(err) => {
                            tracing::error!("error subscribing to L1 blocks: {err:#}");
                            sleep(retry_delay).await;
                        }
                    }
                };

                tracing::info!("established L1 block stream");
                loop {
                    // Wait for a block, timing out if we don't get one within 60 seconds
                    let block_timeout = tokio::time::timeout(Duration::from_secs(60), block_stream.next()).await;

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
                            if head > state.snapshot.head {
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

                metrics.stream_reconnects.add(1);
            }
        }.instrument(span)
    }

    /// Get a snapshot from the l1.
    pub async fn snapshot(&self) -> L1Snapshot {
        self.state.lock().await.snapshot
    }

    // TODO remove after `Memberships` trait update on hotshot
    // https://github.com/EspressoSystems/HotShot/issues/3966
    pub fn stake_table(&self, epoch: &EpochNumber) -> StakeTables {
        if let Some(stake_tables) = self.stake_table_state.read().unwrap().get(epoch) {
            stake_tables.clone()
        } else {
            StakeTables::new(vec![].into(), vec![].into())
        }
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
            sleep(self.retry_delay).await;
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
            sleep(self.retry_delay).await;
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
            sleep(self.retry_delay).await;
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
                    sleep(self.retry_delay).await;
                    continue;
                }
                Err(err) => {
                    tracing::warn!(number, "failed to get finalized L1 block: {err:#}");
                    sleep(self.retry_delay).await;
                    continue;
                }
            };
            let Some(hash) = block.hash else {
                tracing::warn!(number, ?block, "finalized L1 block has no hash");
                sleep(self.retry_delay).await;
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

        // `prev` should have already been processed unless we
        // haven't processed *any* blocks yet.
        let prev = prev_finalized.map(|prev| prev + 1).unwrap_or(0);

        // Divide the range `prev_finalized..=new_finalized` into chunks of size
        // `events_max_block_range`.
        let mut start = prev;
        let end = new_finalized;
        let chunk_size = self.events_max_block_range;
        let chunks = std::iter::from_fn(move || {
            let chunk_end = min(start + chunk_size - 1, end);
            if chunk_end < start {
                return None;
            }

            let chunk = (start, chunk_end);
            start = chunk_end + 1;
            Some(chunk)
        });

        // Fetch events for each chunk.
        let events = stream::iter(chunks).then(|(from, to)| {
            let retry_delay = self.retry_delay;
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

    /// Get `StakeTable` at block height.
    pub async fn get_stake_table(
        &self,
        contract: Address,
        block: u64,
    ) -> anyhow::Result<StakeTables> {
        // TODO stake_table_address needs to be passed in to L1Client
        // before update loop starts.
        let stake_table_contract = PermissionedStakeTable::new(contract, self.provider.clone());

        let events = stake_table_contract
            .stakers_updated_filter()
            .from_block(0)
            .to_block(block)
            .query()
            .await?;

        Ok(StakeTables::from_l1_events(events.clone()))
    }
}

impl L1State {
    fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            snapshot: Default::default(),
            finalized: LruCache::new(cache_size),
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

async fn get_finalized_block(rpc: &Provider<RpcClient>) -> anyhow::Result<Option<L1BlockInfo>> {
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
        utils::{hex, parse_ether, Anvil, AnvilInstance},
    };
    use hotshot_contract_adapter::stake_table::NodeInfoJf;
    use hotshot_types::traits::metrics::NoMetrics;
    use portpicker::pick_unused_port;
    use sequencer_utils::test_utils::setup_test;
    use std::time::Duration;
    use time::OffsetDateTime;

    use super::*;

    async fn new_l1_client(anvil: &AnvilInstance, ws: bool) -> L1Client {
        let url = if ws {
            anvil.ws_endpoint()
        } else {
            anvil.endpoint()
        };

        let client = L1ClientOptions {
            l1_events_max_block_range: 1,
            l1_polling_interval: Duration::from_secs(1),
            ..Default::default()
        }
        .connect(url.parse().unwrap())
        .await
        .unwrap();

        client.spawn_tasks().await;
        client
    }

    async fn test_get_finalized_deposits_helper(ws: bool) -> anyhow::Result<()> {
        setup_test();

        // how many deposits will we make
        let deposits = 5;
        let deploy_txn_count = 2;

        let anvil = Anvil::new().spawn();
        let wallet_address = anvil.addresses().first().cloned().unwrap();
        let l1_client = new_l1_client(&anvil, ws).await;
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
        let l1_client = new_l1_client(&anvil, ws).await;
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_finalized_deposits_ws() -> anyhow::Result<()> {
        test_get_finalized_deposits_helper(true).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_finalized_deposits_http() -> anyhow::Result<()> {
        test_get_finalized_deposits_helper(false).await
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_l1_ws_reconnect_rpc_request() {
        setup_test();

        let port = pick_unused_port().unwrap();
        let mut anvil = Anvil::new().block_time(1u32).port(port).spawn();
        let provider = Provider::new(
            RpcClient::ws(
                anvil.ws_endpoint().parse().unwrap(),
                Arc::new(L1ClientMetrics::new(&NoMetrics)),
                Duration::from_secs(1),
            )
            .await
            .unwrap(),
        );

        // Check the provider is working.
        assert_eq!(provider.get_chainid().await.unwrap(), 31337.into());

        // Test two reconnects in a row, to ensure the reconnecter is reset properly after the first
        // one.
        'outer: for i in 0..2 {
            tracing::info!("reconnect {i}");
            // Disconnect the WebSocket and reconnect it. Technically this spawns a whole new Anvil
            // chain, but for the purposes of this test it should look to the client like an L1
            // server closing a WebSocket connection.
            drop(anvil);
            let err = provider.get_chainid().await.unwrap_err();
            tracing::info!("L1 request failed as expected with closed connection: {err:#}");

            // Let the connection stay down for a little while: Ethers internally tries to
            // reconnect, and starting up to fast again might hit that and cause a false positive.
            // The problem is, Ethers doesn't try very hard, and if we wait a bit, we will test the
            // worst possible case where the internal retry logic gives up and just kills the whole
            // provider.
            tracing::info!("sleep 5");
            sleep(Duration::from_secs(5)).await;

            // Once a connection is reestablished, the provider will eventually work again.
            tracing::info!("restarting L1");
            anvil = Anvil::new().block_time(1u32).port(port).spawn();
            // Give a bit of time for the provider to reconnect.
            for retry in 0..5 {
                if let Ok(chain_id) = provider.get_chainid().await {
                    assert_eq!(chain_id, 31337.into());
                    continue 'outer;
                }
                tracing::warn!(retry, "waiting for provider to reconnect");
                sleep(Duration::from_secs(1)).await;
            }
            panic!("request never succeeded after reconnect");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_l1_ws_reconnect_update_task() {
        setup_test();

        let port = pick_unused_port().unwrap();
        let anvil = Anvil::new().block_time(1u32).port(port).spawn();
        let client = new_l1_client(&anvil, true).await;

        let initial_state = client.snapshot().await;
        tracing::info!(?initial_state, "initial state");

        // Check the state is updating.
        let mut retry = 0;
        let updated_state = loop {
            assert!(retry < 5, "state did not update in time");

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
    async fn test_fetch_stake_table() -> anyhow::Result<()> {
        setup_test();

        let anvil = Anvil::new().spawn();
        let l1_client = L1Client::http(anvil.endpoint().parse().unwrap());
        let wallet: LocalWallet = anvil.keys()[0].clone().into();

        // In order to deposit we need a provider that can sign.
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client =
            SignerMiddleware::new(provider.clone(), wallet.with_chain_id(anvil.chain_id()));
        let client = Arc::new(client);

        let v: Vec<NodeInfo> = Vec::new();
        // deploy the stake_table contract
        let stake_table_contract = PermissionedStakeTable::deploy(client.clone(), v.clone())
            .unwrap()
            .send()
            .await?;

        let address = stake_table_contract.address();

        let mut rng = rand::thread_rng();
        let node = NodeInfoJf::random(&mut rng);

        let new_nodes: Vec<NodeInfo> = vec![node.into()];
        let updater = stake_table_contract.update(v, new_nodes);
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
}
