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
pub(crate) trait SequencerDataSource<N: network::Type>:
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

pub(crate) trait SubmitDataSource<N: network::Type> {
    fn handle(&self) -> &SystemContextHandle<SeqTypes, Node<N>>;
}

#[cfg(test)]
pub(crate) mod testing {
    use super::super::Options;
    use super::*;
    use crate::network::Memory;

    #[async_trait]
    pub(crate) trait TestableSequencerDataSource: SequencerDataSource<Memory> {
        type Storage;

        async fn create_storage() -> Self::Storage;
        fn options(storage: &Self::Storage, opt: Options) -> Options;
    }
}
