// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use super::{notifier::Notifier, storage::AvailabilityStorage, VersionedDataSource};
use crate::{
    availability::{
        AvailabilityDataSource, BlockId, BlockQueryData, Fetch, LeafId, LeafQueryData,
        QueryablePayload, ResourceId, TransactionHash, TransactionIndex, UpdateAvailabilityData,
    },
    metrics::PrometheusMetrics,
    node::{NodeDataSource, UpdateNodeData},
    status::StatusDataSource,
    Payload, QueryResult, SignatureKey,
};
use async_std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    task::spawn,
};
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::{Display, From};
use futures::{
    future::{join_all, BoxFuture, FutureExt},
    stream::{self, BoxStream, Stream, StreamExt},
};
use hotshot_types::traits::node_implementation::NodeType;
use std::{
    cmp::{min, Ordering},
    fmt::{Debug, Display},
    future::IntoFuture,
    ops::{Bound, Deref, DerefMut, Range, RangeBounds},
};

/// The number of items to process at a time when loading a range or stream.
///
/// This determines:
/// * The number of objects to load from storage in a single request
/// * The number of objects to buffer in memory per request/stream
/// * The number of concurrent notification subscriptions per request/stream
const RANGE_CHUNK_SIZE: usize = 25;

/// The most basic kind of data source.
///
/// A data source is constructed modularly by combining a [storage](super::storage) implementation
/// with a [fetcher](super::fetcher). The former allows the query service to store the data it has
/// persistently in an easily accessible storage medium, such as the local file system or a
/// database. This allows it to answer queries efficiently and to maintain its state across
/// restarts. The latter allows the query service to fetch data that is missing from its storage
/// from an external data availability provider, such as the Tiramisu DA network or another instance
/// of the query service.
///
/// These two components of a data source are combined in [`FetchingDataSource`], which is the
/// lowest level kind of data source available. It simply uses the storage implementation to fetch
/// data when available, and fills in everything else using the fetcher. Various kinds of data
/// sources can be constructed out of [`FetchingDataSource`] by changing the storage and fetcher
/// implementations used, and more complex data sources can be built on top using data source
/// combinators.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = "S: Debug, P: Debug"))]
pub struct FetchingDataSource<Types, S, P>
where
    Types: NodeType,
{
    // The fetcher manages retrieval of resources from both local storage and a remote provider. It
    // encapsulates the data which may need to be shared with a long-lived task or future that
    // implements the asynchronous fetching of a particular object. This is why it gets its own
    // type, wrapped in an [`Arc`] for easy, efficient cloning.
    fetcher: Arc<Fetcher<Types, S, P>>,
    // The rest of the data we need for implementing data source traits but not for fetching.
    metrics: PrometheusMetrics,
}

impl<Types, S, P> FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    S: NodeDataSource<Types>
        + UpdateNodeData<Types>
        + AvailabilityStorage<Types>
        + VersionedDataSource,
    P: Send + Sync,
{
    /// Create a data source with local storage and a remote data availability provider.
    pub async fn new(storage: S, provider: P) -> anyhow::Result<Self> {
        let mut ds = Self {
            fetcher: Arc::new(Fetcher::new(storage, provider)),
            metrics: Default::default(),
        };

        if NodeDataSource::block_height(&ds).await? == 0 {
            // HotShot doesn't emit an event for the genesis block, so we need to manually ensure it
            // is present.
            ds.insert_genesis().await?;
        }

        Ok(ds)
    }

    async fn insert_genesis(&mut self) -> anyhow::Result<()> {
        UpdateAvailabilityData::insert_leaf(self, LeafQueryData::genesis()).await?;
        UpdateNodeData::insert_leaf(self, LeafQueryData::genesis()).await?;
        self.insert_block(BlockQueryData::genesis()).await?;
        self.commit().await?;
        Ok(())
    }
}

#[derive(From)]
pub struct StorageReadGuard<'a, Types, S>
where
    Types: NodeType,
{
    inner: RwLockReadGuard<'a, NotifyStorage<Types, S>>,
}

impl<'a, Types, S> Deref for StorageReadGuard<'a, Types, S>
where
    Types: NodeType,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner.storage
    }
}

#[derive(From)]
pub struct StorageWriteGuard<'a, Types, S>
where
    Types: NodeType,
{
    inner: RwLockWriteGuard<'a, NotifyStorage<Types, S>>,
}

impl<'a, Types, S> Deref for StorageWriteGuard<'a, Types, S>
where
    Types: NodeType,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner.storage
    }
}

impl<'a, Types, S> DerefMut for StorageWriteGuard<'a, Types, S>
where
    Types: NodeType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.storage
    }
}

impl<Types, S, P> FetchingDataSource<Types, S, P>
where
    Types: NodeType,
{
    /// Obtain direct, read-only access to the underlying local storage.
    pub async fn storage(&self) -> StorageReadGuard<Types, S> {
        self.fetcher.storage.read().await.into()
    }

    /// Obtain direct, mutable access the underlying local storage.
    pub async fn storage_mut(&self) -> StorageWriteGuard<Types, S> {
        self.fetcher.storage.write().await.into()
    }
}

#[async_trait]
impl<Types, S, P> StatusDataSource for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    S: NodeDataSource<Types> + Send + Sync,
    P: Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        NodeDataSource::block_height(&*self.storage().await).await
    }

    fn metrics(&self) -> &PrometheusMetrics {
        &self.metrics
    }
}

#[async_trait]
impl<Types, S, P> AvailabilityDataSource<Types> for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    S: AvailabilityStorage<Types> + 'static,
    P: Send + Sync + 'static,
{
    type LeafRange<R> = BoxStream<'static, Fetch<LeafQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send;
    type BlockRange<R> = BoxStream<'static, Fetch<BlockQueryData<Types>>>
    where
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> Fetch<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync,
    {
        self.fetcher.get(id.into()).await
    }

    async fn get_block<ID>(&self, id: ID) -> Fetch<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        self.fetcher.get(id.into()).await
    }

    async fn get_leaf_range<R>(&self, range: R) -> Self::LeafRange<R>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.fetcher.clone().get_range(range)
    }

    async fn get_block_range<R>(&self, range: R) -> Self::BlockRange<R>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        self.fetcher.clone().get_range(range)
    }

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> Fetch<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        self.fetcher.get(hash).await
    }
}

#[async_trait]
impl<Types, S, P> UpdateAvailabilityData<Types> for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    S: UpdateAvailabilityData<Types> + Send + Sync,
    P: Send + Sync,
{
    type Error = S::Error;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        leaf.store(&mut *self.fetcher.storage.write().await).await
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        block.store(&mut *self.fetcher.storage.write().await).await
    }
}

#[async_trait]
impl<Types, S, P> NodeDataSource<Types> for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    S: NodeDataSource<Types> + Send + Sync,
    P: Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        self.storage().await.block_height().await
    }

    async fn get_proposals(
        &self,
        proposer: &SignatureKey<Types>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
        self.storage().await.get_proposals(proposer, limit).await
    }

    async fn count_proposals(&self, proposer: &SignatureKey<Types>) -> QueryResult<usize> {
        self.storage().await.count_proposals(proposer).await
    }
}

#[async_trait]
impl<Types, S, P> UpdateNodeData<Types> for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    S: UpdateNodeData<Types> + Send + Sync,
    P: Send + Sync,
{
    type Error = S::Error;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        self.fetcher
            .storage
            .write()
            .await
            .storage
            .insert_leaf(leaf)
            .await
    }
}

#[async_trait]
impl<Types, S, P> VersionedDataSource for FetchingDataSource<Types, S, P>
where
    Types: NodeType,
    S: VersionedDataSource + Send + Sync,
    P: Send + Sync,
{
    type Error = S::Error;

    async fn commit(&mut self) -> Result<(), Self::Error> {
        self.storage_mut().await.commit().await
    }

    async fn revert(&mut self) {
        self.storage_mut().await.revert().await
    }
}

/// Asynchronous retrieval and storage of [`Fetchable`] resources.
#[derive(Debug)]
struct Fetcher<Types, S, P>
where
    Types: NodeType,
{
    storage: RwLock<NotifyStorage<Types, S>>,
    _provider: P,
}

#[derive(Debug)]
struct NotifyStorage<Types, S>
where
    Types: NodeType,
{
    storage: S,
    block_notifier: Notifier<BlockQueryData<Types>>,
    leaf_notifier: Notifier<LeafQueryData<Types>>,
}

impl<Types, S, P> Fetcher<Types, S, P>
where
    Types: NodeType,
{
    fn new(storage: S, provider: P) -> Self {
        Self {
            storage: RwLock::new(NotifyStorage {
                storage,
                block_notifier: Notifier::new(),
                leaf_notifier: Notifier::new(),
            }),
            _provider: provider,
        }
    }
}

impl<Types, S, P> Fetcher<Types, S, P>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    S: AvailabilityStorage<Types> + 'static,
    P: Send + Sync + 'static,
{
    async fn get<T, R>(self: &Arc<Self>, req: R) -> Fetch<T>
    where
        T: Fetchable<Types>,
        R: Into<T::Request> + Send,
    {
        let req = req.into();
        // Hold a read lock on `storage` while we run `ok_or_fetch`. This means that no
        // notifications are sent in between checking local storage and triggering a fetch if
        // necessary, since sending notifications requires a write lock. Hence, we will not miss a
        // notification.
        let storage = self.storage.read().await;
        self.ok_or_fetch(&storage, req, T::load(&storage.storage, req).await)
            .await
    }

    /// Get a range of objects from local storage or a provider.
    ///
    /// Convert a finite stream of fallible local storage lookups into a (possibly infinite) stream
    /// of infallible fetches. Objects in `range` are loaded from local storage. Any gaps or missing
    /// objects are filled by fetching from a provider. Items in the resulting stream are futures
    /// that will never fail to produce a resource, although they may block indefinitely if the
    /// resource needs to be fetched.
    ///
    /// Objects are loaded and fetched in chunks, which strikes a good balance of limiting the total
    /// number of storage and network requests, while also keeping the amount of simultaneous
    /// resource consumption bounded.
    fn get_range<R, T>(self: Arc<Self>, range: R) -> BoxStream<'static, Fetch<T>>
    where
        R: RangeBounds<usize> + Send + 'static,
        T: RangedFetchable<Types>,
    {
        stream::iter(range_chunks(range))
            .then(move |chunk| self.clone().get_chunk(chunk))
            .flatten()
            .boxed()
    }

    /// Get a range of objects from local storage or a provider.
    ///
    /// This method is similar to `get_range`, except that:
    /// * It fetches all desired objects together, as a single chunk
    /// * It loads the object or triggers fetches right now rather than providing a lazy stream
    ///   which only fetches objects when polled.
    async fn get_chunk<T>(self: Arc<Self>, chunk: Range<usize>) -> impl Stream<Item = Fetch<T>>
    where
        T: RangedFetchable<Types>,
    {
        let storage = self.storage.read().await;
        let ts = T::load_range(&storage.storage, chunk.clone())
            .await
            .unwrap_or_default();
        // Log and discard error information; we want a list of Option where None indicates an
        // object that needs to be fetched.
        let ts = ts.into_iter().map(ResultExt::ok_or_trace);
        // Storage may return fewer objects than asked for if we hit the end of the current chain.
        // Pad out to the end of the chunk with None, indicating that objects we don't have yet must
        // be fetched.
        let padding = std::iter::repeat(None).take(chunk.len() - ts.len());
        let ts = ts.chain(padding);
        // Kick off a fetch for each missing object.
        let fetcher = &self;
        let ts = ts
            .enumerate()
            .map(|(i, opt)| fetcher.some_or_fetch(&storage, chunk.start + i, opt));

        // We `join_all` here because we want this iterator to be evaluated eagerly for two reasons:
        // 1. It borrows from `self`, which is local to this future. This avoids having to clone
        //    `self` for every entry, instead we clone it for every chunk.
        // 2. We evaluate all the `some_or_fetch` calls eagerly, so the fetches are triggered as
        //    soon as we evaluate the chunk. This ensures we don't miss any notifications, since we
        //    load from storage and subscribe to notifications for missing objects all while we have
        //    a read lock on `self.storage`. No notifications can be sent during this time since
        //    sending a notification requires a write lock.
        stream::iter(join_all(ts).await)
    }

    async fn ok_or_fetch<R, T>(
        self: &Arc<Self>,
        storage: &RwLockReadGuard<'_, NotifyStorage<Types, S>>,
        req: R,
        res: QueryResult<T>,
    ) -> Fetch<T>
    where
        R: Into<T::Request> + Send,
        T: Fetchable<Types> + Send + Sync + 'static,
    {
        self.some_or_fetch(storage, req, res.ok_or_trace()).await
    }

    async fn some_or_fetch<R, T>(
        self: &Arc<Self>,
        storage: &RwLockReadGuard<'_, NotifyStorage<Types, S>>,
        req: R,
        res: Option<T>,
    ) -> Fetch<T>
    where
        R: Into<T::Request> + Send,
        T: Fetchable<Types> + Send + Sync + 'static,
    {
        match res {
            Some(t) => Fetch::Ready(t),
            None => self.fetch(storage, req).await,
        }
    }

    async fn fetch<R, T>(
        self: &Arc<Self>,
        storage: &RwLockReadGuard<'_, NotifyStorage<Types, S>>,
        req: R,
    ) -> Fetch<T>
    where
        R: Into<T::Request>,
        T: Fetchable<Types>,
    {
        let req = req.into();
        tracing::debug!("fetching resource {req:?}");

        // Subscribe to notifications so we are alerted when we get the resource.
        let fut = T::passive_fetch(&**storage, req)
            .await
            .then(move |opt| async move {
                match opt {
                    Some(t) => t,
                    None => {
                        // If `passive_fetch` returns `None`, it means the notifier was dropped
                        // without ever sending a notification. In this case, the correct behavior
                        // is actually to block forever (unless the `Fetch` itself is dropped),
                        // since the semantics of `Fetch` are to never fail. This is analogous to
                        // fetching an object which doesn't actually exist: the `Fetch` will never
                        // return.
                        //
                        // However, for ease of debugging, and since this is never expected to
                        // happen in normal usage, we panic instead. This should only happen in two
                        // cases:
                        // * The server was shut down (dropping the notifier) without cleaning up
                        //   some background tasks. This will not affect runtime behavior, but
                        //   should be fixed if it happens.
                        // * There is a very unexpected runtime bug resulting in the notifier being
                        //   dropped. If this happens, things are very broken in any case, and it is
                        //   better to panic loudly than simply block forever.
                        panic!("notifier dropped without satisfying request {req:?}");
                    }
                }
            });

        // Trigger an active fetch from a remote provider if possible.
        if let Some(active) = T::active_fetch(self, req).await {
            let fetcher = self.clone();
            spawn(async move {
                tracing::info!("spawned active fetch for {req:?}");
                let obj = active.await;
                tracing::info!("fetched object {req:?}");

                // Store the resource in local storage, so we can avoid fetching it in the future.
                let mut storage = fetcher.storage.write().await;
                if let Err(err) = obj.store(&mut *storage).await {
                    // It is unfortunate if this fails, but we can still proceed by returning the
                    // resource that we fetched, keeping it in memory. Simply log the error and move
                    // on.
                    tracing::warn!("failed to store fetched resource {req:?}: {err}");
                }
            });
        }

        // Wait for the object to arrive.
        Fetch::Pending(fut.boxed())
    }
}

/// Objects which can be fetched from a remote DA provider and cached in local storage.
///
/// This trait lets us abstract over leaves, blocks, and other types that can be fetched. Thus, the
/// logistics of fetching are shared between all objects, and only the low-level particulars are
/// type-specific.
#[async_trait]
trait Fetchable<Types>: Clone + Send + Sync + 'static
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    /// A succinct specification of the object to be fetched.
    type Request: Copy + Debug + Send + Sync + 'static;

    /// Does this object satisfy the given request?
    fn satisfies(&self, req: Self::Request) -> bool;

    /// Create a future for fetching the object from a remote provider, if possible.
    ///
    /// An active fetch will only be triggered if:
    /// * There is not already an active fetch in progress for the same object
    /// * The requested object is known to exist. For example, we will trigger a fetch of a block
    ///   with a height less than the current block height, but not greater, since the latter might
    ///   not exist yet, and we should receive it passively once it is produced. Or, we will fetch a
    ///   leaf by height but not by hash, since we can't guarantee that a leaf with an arbitrary
    ///   hash exists.
    ///
    /// If we do not trigger an active fetch for an object, but the object does in fact exist, we
    /// will still eventually receive it passively, since we will eventually receive all blocks and
    /// leaves that are ever produced.
    async fn active_fetch<S, P>(
        _fetcher: &Fetcher<Types, S, P>,
        _req: Self::Request,
    ) -> Option<BoxFuture<'static, Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        // TODO implement active fetching
        None
    }

    /// Wait for someone else to fetch the object.
    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>;

    /// Cache the object in local storage.
    async fn store<S>(self, storage: &mut NotifyStorage<Types, S>) -> Result<(), S::Error>
    where
        S: UpdateAvailabilityData<Types> + Send + Sync;

    /// Load an object from local storage.
    async fn load<S>(storage: &S, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>;
}

#[async_trait]
trait RangedFetchable<Types>: Fetchable<Types, Request = Self::RangedRequest>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type RangedRequest: From<usize> + Send;

    /// Load a range of these objects from local storage.
    async fn load_range<S, R>(storage: &S, range: R) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static;
}

#[async_trait]
impl<Types> Fetchable<Types> for LeafQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type Request = LeafId<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req {
            ResourceId::Number(n) => self.height() == n as u64,
            ResourceId::Hash(h) => self.hash() == h,
        }
    }

    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        storage
            .leaf_notifier
            .wait_for(move |leaf| leaf.satisfies(req))
            .await
            .into_future()
            .boxed()
    }

    async fn store<S>(self, storage: &mut NotifyStorage<Types, S>) -> Result<(), S::Error>
    where
        S: UpdateAvailabilityData<Types> + Send + Sync,
    {
        storage.leaf_notifier.notify(&self);
        storage.storage.insert_leaf(self).await
    }

    async fn load<S>(storage: &S, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.get_leaf(req).await
    }
}

#[async_trait]
impl<Types> RangedFetchable<Types> for LeafQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type RangedRequest = LeafId<Types>;

    async fn load_range<S, R>(storage: &S, range: R) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.get_leaf_range(range).await
    }
}

/// A request to fetch a block.
///
/// Blocks can be requested either directly by their [`BlockId`], or indirectly, by requesting a
/// block containing a particular transaction.
#[derive(Derivative, From, Display)]
#[derivative(Ord = "feature_allow_slow_enum")]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    Hash(bound = "")
)]
pub enum BlockRequest<Types>
where
    Types: NodeType,
{
    Id(BlockId<Types>),
    WithTransaction(TransactionHash<Types>),
}

impl<Types> From<usize> for BlockRequest<Types>
where
    Types: NodeType,
{
    fn from(i: usize) -> Self {
        Self::Id(i.into())
    }
}

impl<Types> Clone for BlockRequest<Types>
where
    Types: NodeType,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Types> PartialOrd for BlockRequest<Types>
where
    Types: NodeType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[async_trait]
impl<Types> Fetchable<Types> for BlockQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type Request = BlockRequest<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        match req {
            BlockRequest::Id(ResourceId::Number(n)) => self.height() == n as u64,
            BlockRequest::Id(ResourceId::Hash(h)) => self.hash() == h,
            BlockRequest::WithTransaction(h) => self.transaction_by_hash(h).is_some(),
        }
    }

    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        storage
            .block_notifier
            .wait_for(move |block| block.satisfies(req))
            .await
            .into_future()
            .boxed()
    }

    async fn store<S>(self, storage: &mut NotifyStorage<Types, S>) -> Result<(), S::Error>
    where
        S: UpdateAvailabilityData<Types> + Send + Sync,
    {
        storage.block_notifier.notify(&self);
        storage.storage.insert_block(self).await
    }

    async fn load<S>(storage: &S, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        match req {
            BlockRequest::Id(id) => storage.get_block(id).await,
            BlockRequest::WithTransaction(h) => Ok(storage.get_block_with_transaction(h).await?.0),
        }
    }
}

#[async_trait]
impl<Types> RangedFetchable<Types> for BlockQueryData<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type RangedRequest = BlockRequest<Types>;

    async fn load_range<S, R>(storage: &S, range: R) -> QueryResult<Vec<QueryResult<Self>>>
    where
        S: AvailabilityStorage<Types>,
        R: RangeBounds<usize> + Send + 'static,
    {
        storage.get_block_range(range).await
    }
}

#[async_trait]
impl<Types> Fetchable<Types> for (BlockQueryData<Types>, TransactionIndex<Types>)
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type Request = TransactionHash<Types>;

    fn satisfies(&self, req: Self::Request) -> bool {
        self.0.transaction_by_hash(req).is_some()
    }

    async fn passive_fetch<S>(
        storage: &NotifyStorage<Types, S>,
        req: Self::Request,
    ) -> BoxFuture<'static, Option<Self>>
    where
        S: AvailabilityStorage<Types>,
    {
        let wait_block = storage
            .block_notifier
            .wait_for(move |block| block.satisfies(req.into()))
            .await;

        async move {
            let block = wait_block.await?;

            // This `unwrap` is safe, `wait_for` only returns blocks which satisfy the request, and
            // in this case that means the block must contain the requested transaction.
            let ix = block.transaction_by_hash(req).unwrap();

            Some((block, ix))
        }
        .boxed()
    }

    async fn store<S>(self, storage: &mut NotifyStorage<Types, S>) -> Result<(), S::Error>
    where
        S: UpdateAvailabilityData<Types> + Send + Sync,
    {
        storage.block_notifier.notify(&self.0);
        storage.storage.insert_block(self.0).await
    }

    async fn load<S>(storage: &S, req: Self::Request) -> QueryResult<Self>
    where
        S: AvailabilityStorage<Types>,
    {
        storage.get_block_with_transaction(req).await
    }
}

/// Break a range into fixed-size chunks.
fn range_chunks<R>(range: R) -> impl Iterator<Item = Range<usize>>
where
    R: RangeBounds<usize>,
{
    // Transform range to explicit start (inclusive) and end (exclusive) bounds.
    let mut start = match range.start_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(i) => *i + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(i) => *i + 1,
        Bound::Excluded(i) => *i,
        Bound::Unbounded => usize::MAX,
    };
    std::iter::from_fn(move || {
        let chunk_end = min(start + RANGE_CHUNK_SIZE, end);
        if chunk_end == start {
            return None;
        }

        let chunk = start..chunk_end;
        start = chunk_end;
        Some(chunk)
    })
}

trait ResultExt<T, E> {
    fn ok_or_trace(self) -> Option<T>
    where
        E: Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ok_or_trace(self) -> Option<T>
    where
        E: Display,
    {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                tracing::warn!("error load resource from local storage, will try to fetch: {err}");
                None
            }
        }
    }
}
