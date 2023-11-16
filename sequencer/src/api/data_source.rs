use super::endpoints::TimeWindowQueryData;
use crate::{network, Node, SeqTypes};
use async_trait::async_trait;
use hotshot::types::SystemContextHandle;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId},
    data_source::{UpdateDataSource, VersionedDataSource},
    status::StatusDataSource,
    QueryResult,
};

#[async_trait]
pub(super) trait SequencerDataSource<N: network::Type>:
    AvailabilityDataSource<SeqTypes, Node<N>>
    + StatusDataSource
    + UpdateDataSource<SeqTypes, Node<N>>
    + VersionedDataSource
    + Sized
{
    type Options;

    async fn create(opt: Self::Options) -> anyhow::Result<Self>;
    async fn refresh_indices(&mut self, from_block: usize) -> anyhow::Result<()>;

    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData>;
    async fn window_from<ID>(&self, from: ID, end: u64) -> QueryResult<TimeWindowQueryData>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync;
}

pub(super) trait SubmitDataSource<N: network::Type> {
    fn handle(&self) -> &SystemContextHandle<SeqTypes, Node<N>>;
}
