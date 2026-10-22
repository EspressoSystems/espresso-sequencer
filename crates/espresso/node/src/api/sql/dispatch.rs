//! One query data source over both SQL backends, so everything above storage compiles once.
//!
//! Every method of every forwarded trait is listed, including those with default bodies: a default
//! would bypass an override on the backend.

use std::ops::{Bound, Range, RangeBounds};

use async_trait::async_trait;
use committable::Commitment;
use espresso_types::{
    BlockMerkleCommitment, ChainConfig, FeeAccount, FeeMerkleCommitment, FeeMerkleTree, Leaf2,
    NodeState,
    v0_3::{RewardAccountQueryDataV1, RewardAccountV1, RewardMerkleTreeV1},
    v0_4::{PermittedRewardMerkleTreeV2, RewardAccountV2, RewardMerkleTreeV2},
    v0_6::RewardAccountQueryDataV2,
};
use futures::{future::Future, stream::BoxStream};
use hotshot_query_service::{
    Header, QueryResult,
    availability::{
        AvailabilityDataSource, BlockId, BlockInfo, BlockQueryData, BlockWithTransaction,
        Certificate2, Fetch, FetchStream, LeafId, LeafQueryData, NamespaceId, PayloadMetadata,
        PayloadQueryData, TransactionHash, UpdateAvailabilityData, VidCommonMetadata,
        VidCommonQueryData,
    },
    data_source::{
        Transaction, VersionedDataSource,
        storage::{AvailabilityStorage, NodeStorage, pruning::PrunedHeightDataSource},
    },
    explorer::{
        BlockDetail, BlockIdentifier, BlockSummary, ExplorerDataSource, ExplorerSummary,
        GetBlockDetailError, GetBlockSummariesError, GetBlockSummariesRequest,
        GetExplorerSummaryError, GetSearchResultsError, GetTransactionDetailError,
        GetTransactionSummariesError, GetTransactionSummariesRequest, SearchResult,
        TransactionIdentifier, TransactionSummary,
    },
    merklized_state::{
        MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence, Snapshot,
        UpdateStateData,
    },
    metrics::PrometheusMetrics,
    node::{NodeDataSource, SyncStatusQueryData, TimeWindowQueryData, WindowStart},
    status::{HasMetrics, StatusDataSource},
};
use hotshot_query_service_types::explorer::TransactionDetailResponse;
use hotshot_types::data::{VidShare, ViewNumber};
use jf_merkle_tree_compat::prelude::MerkleProof;
use light_client::storage::LightClientDbOptions;
use tagged_base64::TaggedBase64;
use vbs::version::Version;

use super::{postgres, sqlite};
use crate::{
    SeqTypes,
    api::{
        BlocksFrontier, RewardMerkleTreeDataSource,
        data_source::{
            DataSourceOptions, DatabaseMetadataSource, MigrationStatus, Provider,
            PruningDataSource, SequencerDataSource, TableSize,
        },
        options::{self, Query},
    },
    catchup::CatchupStorage,
    persistence::{
        ChainConfigPersistence,
        sql::{
            Options as SqlOptions, postgres as postgres_persistence, sqlite as sqlite_persistence,
        },
    },
    sql_dispatch::{Sql, dispatch},
};

impl<P, L> VersionedDataSource for Sql<P, L>
where
    P: VersionedDataSource,
    L: VersionedDataSource,
{
    type Transaction<'a>
        = Sql<P::Transaction<'a>, L::Transaction<'a>>
    where
        Self: 'a;

    type ReadOnly<'a>
        = Sql<P::ReadOnly<'a>, L::ReadOnly<'a>>
    where
        Self: 'a;

    async fn write(&self) -> anyhow::Result<Self::Transaction<'_>> {
        Ok(match self {
            Sql::Postgres(x) => Sql::Postgres(x.write().await?),
            Sql::Sqlite(x) => Sql::Sqlite(x.write().await?),
        })
    }

    async fn read(&self) -> anyhow::Result<Self::ReadOnly<'_>> {
        Ok(match self {
            Sql::Postgres(x) => Sql::Postgres(x.read().await?),
            Sql::Sqlite(x) => Sql::Sqlite(x.read().await?),
        })
    }
}

impl<P, L> Transaction for Sql<P, L>
where
    P: Transaction,
    L: Transaction,
{
    async fn commit(self) -> anyhow::Result<()> {
        dispatch!(self, x => x.commit().await)
    }

    fn revert(self) -> impl Future + Send {
        async move {
            dispatch!(self, x => {
                x.revert().await;
            })
        }
    }
}

#[async_trait]
impl<P, L> AvailabilityDataSource<SeqTypes> for Sql<P, L>
where
    P: AvailabilityDataSource<SeqTypes> + Send + Sync,
    L: AvailabilityDataSource<SeqTypes> + Send + Sync,
{
    async fn get_leaf<ID>(&self, id: ID) -> Fetch<LeafQueryData<SeqTypes>>
    where
        ID: Into<LeafId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => AvailabilityDataSource::get_leaf(x, id).await)
    }

    async fn get_header<ID>(&self, id: ID) -> Fetch<Header<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_header(id).await)
    }

    async fn get_block<ID>(&self, id: ID) -> Fetch<BlockQueryData<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_block(id).await)
    }

    async fn get_payload<ID>(&self, id: ID) -> Fetch<PayloadQueryData<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_payload(id).await)
    }

    async fn get_payload_metadata<ID>(&self, id: ID) -> Fetch<PayloadMetadata<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_payload_metadata(id).await)
    }

    async fn get_vid_common<ID>(&self, id: ID) -> Fetch<VidCommonQueryData<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_vid_common(id).await)
    }

    async fn get_vid_common_metadata<ID>(&self, id: ID) -> Fetch<VidCommonMetadata<SeqTypes>>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.get_vid_common_metadata(id).await)
    }

    async fn get_leaf_range<R>(&self, range: R) -> FetchStream<LeafQueryData<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_leaf_range(range).await)
    }

    async fn get_header_range<R>(&self, range: R) -> FetchStream<Header<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_header_range(range).await)
    }

    async fn get_block_range<R>(&self, range: R) -> FetchStream<BlockQueryData<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_block_range(range).await)
    }

    async fn get_payload_range<R>(&self, range: R) -> FetchStream<PayloadQueryData<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_payload_range(range).await)
    }

    async fn get_payload_metadata_range<R>(
        &self,
        range: R,
    ) -> FetchStream<PayloadMetadata<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_payload_metadata_range(range).await)
    }

    async fn get_vid_common_range<R>(&self, range: R) -> FetchStream<VidCommonQueryData<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_vid_common_range(range).await)
    }

    async fn get_vid_common_metadata_range<R>(
        &self,
        range: R,
    ) -> FetchStream<VidCommonMetadata<SeqTypes>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_vid_common_metadata_range(range).await)
    }

    async fn get_leaf_ranges(
        &self,
        ranges: Vec<Range<u64>>,
    ) -> Fetch<Vec<LeafQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_leaf_ranges(ranges).await)
    }

    async fn get_block_ranges(
        &self,
        ranges: Vec<Range<u64>>,
    ) -> Fetch<Vec<BlockQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_block_ranges(ranges).await)
    }

    async fn get_vid_common_ranges(
        &self,
        ranges: Vec<Range<u64>>,
    ) -> Fetch<Vec<VidCommonQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_vid_common_ranges(ranges).await)
    }

    async fn get_leaf_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<LeafQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_leaf_range_rev(start, end).await)
    }

    async fn get_block_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<BlockQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_block_range_rev(start, end).await)
    }

    async fn get_payload_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<PayloadQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_payload_range_rev(start, end).await)
    }

    async fn get_payload_metadata_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<PayloadMetadata<SeqTypes>> {
        dispatch!(self, x => x.get_payload_metadata_range_rev(start, end).await)
    }

    async fn get_vid_common_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<VidCommonQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_vid_common_range_rev(start, end).await)
    }

    async fn get_vid_common_metadata_range_rev(
        &self,
        start: Bound<usize>,
        end: usize,
    ) -> FetchStream<VidCommonMetadata<SeqTypes>> {
        dispatch!(self, x => x.get_vid_common_metadata_range_rev(start, end).await)
    }

    async fn get_block_containing_transaction(
        &self,
        h: TransactionHash<SeqTypes>,
    ) -> Fetch<BlockWithTransaction<SeqTypes>> {
        dispatch!(self, x => x.get_block_containing_transaction(h).await)
    }

    async fn subscribe_blocks(&self, from: usize) -> BoxStream<'static, BlockQueryData<SeqTypes>> {
        dispatch!(self, x => x.subscribe_blocks(from).await)
    }

    async fn subscribe_payloads(
        &self,
        from: usize,
    ) -> BoxStream<'static, PayloadQueryData<SeqTypes>> {
        dispatch!(self, x => x.subscribe_payloads(from).await)
    }

    async fn subscribe_payload_metadata(
        &self,
        from: usize,
    ) -> BoxStream<'static, PayloadMetadata<SeqTypes>> {
        dispatch!(self, x => x.subscribe_payload_metadata(from).await)
    }

    async fn subscribe_leaves(&self, from: usize) -> BoxStream<'static, LeafQueryData<SeqTypes>> {
        dispatch!(self, x => x.subscribe_leaves(from).await)
    }

    async fn subscribe_headers(&self, from: usize) -> BoxStream<'static, Header<SeqTypes>> {
        dispatch!(self, x => x.subscribe_headers(from).await)
    }

    async fn subscribe_vid_common(
        &self,
        from: usize,
    ) -> BoxStream<'static, VidCommonQueryData<SeqTypes>> {
        dispatch!(self, x => x.subscribe_vid_common(from).await)
    }

    async fn subscribe_vid_common_metadata(
        &self,
        from: usize,
    ) -> BoxStream<'static, VidCommonMetadata<SeqTypes>> {
        dispatch!(self, x => x.subscribe_vid_common_metadata(from).await)
    }

    async fn get_cert2(&self, height: u64) -> Fetch<Certificate2<SeqTypes>> {
        dispatch!(self, x => x.get_cert2(height).await)
    }
}

impl<P, L> UpdateAvailabilityData<SeqTypes> for Sql<P, L>
where
    P: UpdateAvailabilityData<SeqTypes> + Send + Sync,
    L: UpdateAvailabilityData<SeqTypes> + Send + Sync,
{
    async fn append(&self, info: BlockInfo<SeqTypes>) -> anyhow::Result<()> {
        dispatch!(self, x => x.append(info).await)
    }

    async fn append_payload(&self, block: BlockQueryData<SeqTypes>) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_payload(block).await)
    }
}

#[async_trait]
impl<P, L> NodeDataSource<SeqTypes> for Sql<P, L>
where
    P: NodeDataSource<SeqTypes> + Send + Sync,
    L: NodeDataSource<SeqTypes> + Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        dispatch!(self, x => NodeDataSource::block_height(x).await)
    }

    async fn count_transactions_in_range(
        &self,
        range: impl RangeBounds<usize> + Send + Sync + Clone,
        namespace: Option<NamespaceId<SeqTypes>>,
    ) -> QueryResult<usize> {
        dispatch!(self, x => x.count_transactions_in_range(range, namespace).await)
    }

    async fn payload_size_in_range(
        &self,
        range: impl RangeBounds<usize> + Send + Sync + Clone,
        namespace: Option<NamespaceId<SeqTypes>>,
    ) -> QueryResult<usize> {
        dispatch!(self, x => x.payload_size_in_range(range, namespace).await)
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.vid_share(id).await)
    }

    async fn get_header_window(
        &self,
        start: impl Into<WindowStart<SeqTypes>> + Send + Sync,
        end: u64,
        limit: usize,
    ) -> QueryResult<TimeWindowQueryData<Header<SeqTypes>>> {
        dispatch!(self, x => x.get_header_window(start, end, limit).await)
    }

    async fn sync_status(&self) -> QueryResult<SyncStatusQueryData> {
        dispatch!(self, x => x.sync_status().await)
    }

    async fn count_transactions(&self) -> QueryResult<usize> {
        dispatch!(self, x => NodeDataSource::count_transactions(x).await)
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        dispatch!(self, x => NodeDataSource::payload_size(x).await)
    }
}

impl<P, L> HasMetrics for Sql<P, L>
where
    P: HasMetrics,
    L: HasMetrics,
{
    fn metrics(&self) -> &PrometheusMetrics {
        dispatch!(self, x => x.metrics())
    }
}

#[async_trait]
impl<P, L> StatusDataSource for Sql<P, L>
where
    P: StatusDataSource + Send + Sync,
    L: StatusDataSource + Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        dispatch!(self, x => StatusDataSource::block_height(x).await)
    }

    fn consensus_metrics(&self) -> QueryResult<PrometheusMetrics> {
        dispatch!(self, x => x.consensus_metrics())
    }

    async fn elapsed_time_since_last_decide(&self) -> QueryResult<u64> {
        dispatch!(self, x => x.elapsed_time_since_last_decide().await)
    }

    async fn success_rate(&self) -> QueryResult<f64> {
        dispatch!(self, x => x.success_rate().await)
    }
}

#[async_trait]
impl<P, L> PrunedHeightDataSource for Sql<P, L>
where
    P: PrunedHeightDataSource + Send + Sync,
    L: PrunedHeightDataSource + Send + Sync,
{
    async fn load_pruned_height(&self) -> anyhow::Result<Option<u64>> {
        dispatch!(self, x => x.load_pruned_height().await)
    }

    async fn load_state_pruned_height(&self) -> anyhow::Result<Option<u64>> {
        dispatch!(self, x => x.load_state_pruned_height().await)
    }
}

#[async_trait]
impl<P, L> ExplorerDataSource<SeqTypes> for Sql<P, L>
where
    P: ExplorerDataSource<SeqTypes> + Send + Sync,
    L: ExplorerDataSource<SeqTypes> + Send + Sync,
{
    async fn get_block_detail(
        &self,
        request: BlockIdentifier<SeqTypes>,
    ) -> Result<BlockDetail<SeqTypes>, GetBlockDetailError> {
        dispatch!(self, x => x.get_block_detail(request).await)
    }

    async fn get_block_summaries(
        &self,
        request: GetBlockSummariesRequest<SeqTypes>,
    ) -> Result<Vec<BlockSummary<SeqTypes>>, GetBlockSummariesError> {
        dispatch!(self, x => x.get_block_summaries(request).await)
    }

    async fn get_transaction_detail(
        &self,
        request: TransactionIdentifier<SeqTypes>,
    ) -> Result<TransactionDetailResponse<SeqTypes>, GetTransactionDetailError> {
        dispatch!(self, x => x.get_transaction_detail(request).await)
    }

    async fn get_transaction_summaries(
        &self,
        request: GetTransactionSummariesRequest<SeqTypes>,
    ) -> Result<Vec<TransactionSummary<SeqTypes>>, GetTransactionSummariesError> {
        dispatch!(self, x => x.get_transaction_summaries(request).await)
    }

    async fn get_explorer_summary(
        &self,
    ) -> Result<ExplorerSummary<SeqTypes>, GetExplorerSummaryError> {
        dispatch!(self, x => x.get_explorer_summary().await)
    }

    async fn get_search_results(
        &self,
        query: TaggedBase64,
    ) -> Result<SearchResult<SeqTypes>, GetSearchResultsError> {
        dispatch!(self, x => x.get_search_results(query).await)
    }
}

#[async_trait]
impl<P, L, State, const ARITY: usize> MerklizedStateDataSource<SeqTypes, State, ARITY> for Sql<P, L>
where
    P: MerklizedStateDataSource<SeqTypes, State, ARITY> + Sync,
    L: MerklizedStateDataSource<SeqTypes, State, ARITY> + Sync,
    State: MerklizedState<SeqTypes, ARITY>,
{
    async fn get_path(
        &self,
        snapshot: Snapshot<SeqTypes, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>> {
        dispatch!(self, x => x.get_path(snapshot, key).await)
    }
}

#[async_trait]
impl<P, L> MerklizedStateHeightPersistence for Sql<P, L>
where
    P: MerklizedStateHeightPersistence + Sync,
    L: MerklizedStateHeightPersistence + Sync,
{
    async fn get_last_state_height(&self) -> QueryResult<usize> {
        dispatch!(self, x => x.get_last_state_height().await)
    }
}

impl<P, L> CatchupStorage for Sql<P, L>
where
    P: CatchupStorage,
    L: CatchupStorage,
{
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<(FeeMerkleTree, Leaf2)> {
        dispatch!(self, x => x.get_accounts(instance, height, view, accounts).await)
    }

    async fn get_accounts_at(
        &self,
        height: u64,
        root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        dispatch!(self, x => x.get_accounts_at(height, root, accounts).await)
    }

    async fn get_reward_accounts_v1(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[RewardAccountV1],
    ) -> anyhow::Result<(RewardMerkleTreeV1, Leaf2)> {
        dispatch!(self, x => x.get_reward_accounts_v1(instance, height, view, accounts).await)
    }

    async fn get_reward_accounts_v2(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[RewardAccountV2],
    ) -> anyhow::Result<(RewardMerkleTreeV2, Leaf2)> {
        dispatch!(self, x => x.get_reward_accounts_v2(instance, height, view, accounts).await)
    }

    async fn get_frontier(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> anyhow::Result<BlocksFrontier> {
        dispatch!(self, x => x.get_frontier(instance, height, view).await)
    }

    async fn get_frontier_at(
        &self,
        height: u64,
        root: BlockMerkleCommitment,
    ) -> anyhow::Result<BlocksFrontier> {
        dispatch!(self, x => x.get_frontier_at(height, root).await)
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        dispatch!(self, x => x.get_chain_config(commitment).await)
    }

    async fn get_leaf_chain(&self, height: u64) -> anyhow::Result<Vec<Leaf2>> {
        dispatch!(self, x => x.get_leaf_chain(height).await)
    }

    async fn load_cert2(
        &self,
        height: u64,
    ) -> anyhow::Result<Option<espresso_types::Certificate2<SeqTypes>>> {
        dispatch!(self, x => CatchupStorage::load_cert2(x, height).await)
    }

    async fn get_leaf(&self, height: u64) -> anyhow::Result<Leaf2> {
        dispatch!(self, x => CatchupStorage::get_leaf(x, height).await)
    }

    async fn load_serialized_reward_merkle_tree_v2(&self, height: u64) -> anyhow::Result<Vec<u8>> {
        dispatch!(self, x => x.load_serialized_reward_merkle_tree_v2(height).await)
    }
}

impl<P, L> RewardMerkleTreeDataSource for Sql<P, L>
where
    P: RewardMerkleTreeDataSource,
    L: RewardMerkleTreeDataSource,
{
    async fn load_v1_reward_account_proof(
        &self,
        height: u64,
        account: RewardAccountV1,
    ) -> anyhow::Result<RewardAccountQueryDataV1> {
        dispatch!(self, x => x.load_v1_reward_account_proof(height, account).await)
    }

    async fn save_and_gc_reward_tree_v2(
        &self,
        node_state: &NodeState,
        height: u64,
        version: Version,
        merkle_tree: &RewardMerkleTreeV2,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => {
            x.save_and_gc_reward_tree_v2(node_state, height, version, merkle_tree).await
        })
    }

    async fn persist_reward_proofs(
        &self,
        node_state: &NodeState,
        height: u64,
        version: Version,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.persist_reward_proofs(node_state, height, version).await)
    }

    async fn load_reward_merkle_tree_v2(
        &self,
        height: u64,
    ) -> anyhow::Result<PermittedRewardMerkleTreeV2> {
        dispatch!(self, x => x.load_reward_merkle_tree_v2(height).await)
    }

    async fn load_latest_reward_merkle_tree_v2(
        &self,
        height: u64,
    ) -> anyhow::Result<PermittedRewardMerkleTreeV2> {
        dispatch!(self, x => x.load_latest_reward_merkle_tree_v2(height).await)
    }

    async fn load_reward_account_proof_v2(
        &self,
        height: u64,
        account: RewardAccountV2,
    ) -> anyhow::Result<RewardAccountQueryDataV2> {
        dispatch!(self, x => x.load_reward_account_proof_v2(height, account).await)
    }

    async fn load_latest_reward_account_proof_v2(
        &self,
        account: RewardAccountV2,
    ) -> anyhow::Result<RewardAccountQueryDataV2> {
        dispatch!(self, x => x.load_latest_reward_account_proof_v2(account).await)
    }

    async fn persist_tree(&self, height: u64, merkle_tree: Vec<u8>) -> anyhow::Result<()> {
        dispatch!(self, x => x.persist_tree(height, merkle_tree).await)
    }

    async fn load_tree(&self, height: u64) -> anyhow::Result<Vec<u8>> {
        dispatch!(self, x => x.load_tree(height).await)
    }

    async fn load_latest_tree(&self, height: u64) -> anyhow::Result<Vec<u8>> {
        dispatch!(self, x => x.load_latest_tree(height).await)
    }

    async fn persist_proofs(
        &self,
        height: u64,
        proofs: impl Iterator<Item = (Vec<u8>, Vec<u8>)> + Send,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.persist_proofs(height, proofs).await)
    }

    async fn load_proof(
        &self,
        height: u64,
        account: Vec<u8>,
        epoch_height: u64,
    ) -> anyhow::Result<Vec<u8>> {
        dispatch!(self, x => x.load_proof(height, account, epoch_height).await)
    }

    async fn load_latest_proof(&self, account: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        dispatch!(self, x => x.load_latest_proof(account).await)
    }

    async fn proof_exists(&self, height: u64) -> bool {
        dispatch!(self, x => x.proof_exists(height).await)
    }

    async fn garbage_collect(&self, height: u64) -> anyhow::Result<()> {
        dispatch!(self, x => x.garbage_collect(height).await)
    }
}

impl<P, L> PruningDataSource for Sql<P, L>
where
    P: PruningDataSource + Sync,
    L: PruningDataSource + Sync,
{
    async fn get_oldest_block(&self) -> anyhow::Result<Option<BlockQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_oldest_block().await)
    }

    async fn get_oldest_leaf(&self) -> anyhow::Result<Option<LeafQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_oldest_leaf().await)
    }
}

impl<P, L> DatabaseMetadataSource for Sql<P, L>
where
    P: DatabaseMetadataSource + Sync,
    L: DatabaseMetadataSource + Sync,
{
    async fn get_table_sizes(&self) -> anyhow::Result<Vec<TableSize>> {
        dispatch!(self, x => x.get_table_sizes().await)
    }

    async fn get_migration_status(&self) -> anyhow::Result<Vec<MigrationStatus>> {
        dispatch!(self, x => x.get_migration_status().await)
    }
}

#[async_trait]
impl<P, L> AvailabilityStorage<SeqTypes> for Sql<P, L>
where
    P: AvailabilityStorage<SeqTypes>,
    L: AvailabilityStorage<SeqTypes>,
{
    async fn get_leaf(&mut self, id: LeafId<SeqTypes>) -> QueryResult<LeafQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_leaf(id).await)
    }

    async fn get_block(&mut self, id: BlockId<SeqTypes>) -> QueryResult<BlockQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_block(id).await)
    }

    async fn get_header(&mut self, id: BlockId<SeqTypes>) -> QueryResult<Header<SeqTypes>> {
        dispatch!(self, x => x.get_header(id).await)
    }

    async fn get_payload(
        &mut self,
        id: BlockId<SeqTypes>,
    ) -> QueryResult<PayloadQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_payload(id).await)
    }

    async fn get_payload_metadata(
        &mut self,
        id: BlockId<SeqTypes>,
    ) -> QueryResult<PayloadMetadata<SeqTypes>> {
        dispatch!(self, x => x.get_payload_metadata(id).await)
    }

    async fn get_vid_common(
        &mut self,
        id: BlockId<SeqTypes>,
    ) -> QueryResult<VidCommonQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_vid_common(id).await)
    }

    async fn get_vid_common_metadata(
        &mut self,
        id: BlockId<SeqTypes>,
    ) -> QueryResult<VidCommonMetadata<SeqTypes>> {
        dispatch!(self, x => x.get_vid_common_metadata(id).await)
    }

    async fn get_leaf_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_leaf_range(range).await)
    }

    async fn get_block_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_block_range(range).await)
    }

    async fn get_header_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<Header<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_header_range(range).await)
    }

    async fn get_payload_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_payload_range(range).await)
    }

    async fn get_payload_metadata_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadMetadata<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_payload_metadata_range(range).await)
    }

    async fn get_vid_common_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_vid_common_range(range).await)
    }

    async fn get_vid_common_metadata_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonMetadata<SeqTypes>>>>
    where
        R: RangeBounds<usize> + Send + 'static,
    {
        dispatch!(self, x => x.get_vid_common_metadata_range(range).await)
    }

    async fn get_leaf_ranges(
        &mut self,
        ranges: &[Range<u64>],
    ) -> QueryResult<Vec<LeafQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_leaf_ranges(ranges).await)
    }

    async fn get_block_ranges(
        &mut self,
        ranges: &[Range<u64>],
    ) -> QueryResult<Vec<BlockQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_block_ranges(ranges).await)
    }

    async fn get_vid_common_ranges(
        &mut self,
        ranges: &[Range<u64>],
    ) -> QueryResult<Vec<VidCommonQueryData<SeqTypes>>> {
        dispatch!(self, x => x.get_vid_common_ranges(ranges).await)
    }

    async fn get_block_with_transaction(
        &mut self,
        hash: TransactionHash<SeqTypes>,
    ) -> QueryResult<BlockQueryData<SeqTypes>> {
        dispatch!(self, x => x.get_block_with_transaction(hash).await)
    }

    async fn load_cert2(&mut self, height: u64) -> QueryResult<Option<Certificate2<SeqTypes>>> {
        dispatch!(self, x => AvailabilityStorage::load_cert2(x, height).await)
    }
}

#[async_trait]
impl<P, L> NodeStorage<SeqTypes> for Sql<P, L>
where
    P: NodeStorage<SeqTypes> + Send,
    L: NodeStorage<SeqTypes> + Send,
{
    async fn block_height(&mut self) -> QueryResult<usize> {
        dispatch!(self, x => x.block_height().await)
    }

    async fn count_transactions_in_range(
        &mut self,
        range: impl RangeBounds<usize> + Send,
        namespace: Option<NamespaceId<SeqTypes>>,
    ) -> QueryResult<usize> {
        dispatch!(self, x => x.count_transactions_in_range(range, namespace).await)
    }

    async fn payload_size_in_range(
        &mut self,
        range: impl RangeBounds<usize> + Send,
        namespace: Option<NamespaceId<SeqTypes>>,
    ) -> QueryResult<usize> {
        dispatch!(self, x => x.payload_size_in_range(range, namespace).await)
    }

    async fn vid_share<ID>(&mut self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        dispatch!(self, x => x.vid_share(id).await)
    }

    async fn get_header_window(
        &mut self,
        start: impl Into<WindowStart<SeqTypes>> + Send + Sync,
        end: u64,
        limit: usize,
    ) -> QueryResult<TimeWindowQueryData<Header<SeqTypes>>> {
        dispatch!(self, x => x.get_header_window(start, end, limit).await)
    }

    async fn load_earliest_cert2(
        &mut self,
        height: u64,
    ) -> QueryResult<Option<Certificate2<SeqTypes>>> {
        dispatch!(self, x => x.load_earliest_cert2(height).await)
    }

    async fn sync_status_for_range(
        &mut self,
        from: usize,
        to: usize,
    ) -> QueryResult<SyncStatusQueryData> {
        dispatch!(self, x => x.sync_status_for_range(from, to).await)
    }
}

#[async_trait]
impl<P, L, State, const ARITY: usize> UpdateStateData<SeqTypes, State, ARITY> for Sql<P, L>
where
    P: UpdateStateData<SeqTypes, State, ARITY>,
    L: UpdateStateData<SeqTypes, State, ARITY>,
    State: MerklizedState<SeqTypes, ARITY>,
{
    async fn set_last_state_height(&mut self, height: usize) -> anyhow::Result<()> {
        dispatch!(self, x => x.set_last_state_height(height).await)
    }

    async fn insert_merkle_nodes(
        &mut self,
        path: MerkleProof<State::Entry, State::Key, State::T, ARITY>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.insert_merkle_nodes(path, traversal_path, block_number).await)
    }

    async fn insert_merkle_nodes_batch(
        &mut self,
        proofs: Vec<(
            MerkleProof<State::Entry, State::Key, State::T, ARITY>,
            Vec<usize>,
        )>,
        block_number: u64,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.insert_merkle_nodes_batch(proofs, block_number).await)
    }
}

#[async_trait]
impl<P, L> ChainConfigPersistence for Sql<P, L>
where
    P: ChainConfigPersistence,
    L: ChainConfigPersistence,
{
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        dispatch!(self, x => x.insert_chain_config(chain_config).await)
    }
}

impl DataSourceOptions for SqlOptions {
    type DataSource = Sql<postgres::DataSource, sqlite::DataSource>;

    fn enable_query_module(&self, opt: options::Options, query: Query) -> options::Options {
        opt.query_sql(query, self.clone())
    }

    fn light_client_db(&self, query: &Query) -> anyhow::Result<LightClientDbOptions> {
        if self.embedded_db {
            sqlite_persistence::Options(self.clone()).light_client_db(query)
        } else {
            postgres_persistence::Options(self.clone()).light_client_db(query)
        }
    }
}

#[async_trait]
impl SequencerDataSource for Sql<postgres::DataSource, sqlite::DataSource> {
    type Options = SqlOptions;

    async fn create(opt: SqlOptions, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        Ok(if opt.embedded_db {
            Sql::Sqlite(
                sqlite::DataSource::create(sqlite_persistence::Options(opt), provider, reset)
                    .await?,
            )
        } else {
            Sql::Postgres(
                postgres::DataSource::create(postgres_persistence::Options(opt), provider, reset)
                    .await?,
            )
        })
    }
}
