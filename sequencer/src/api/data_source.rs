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

/// A data source with sequencer-specific functionality.
///
/// This trait extends the generic [`AvailabilityDataSource`] with some additional data needed to
/// provided sequencer-specific endpoints.
#[async_trait]
pub(crate) trait SequencerDataSource<N: network::Type>:
    AvailabilityDataSource<SeqTypes, Node<N>>
    + StatusDataSource
    + UpdateDataSource<SeqTypes, Node<N>>
    + VersionedDataSource
    + Sized
{
    type Options;

    /// Instantiate a data source from command line options.
    async fn create(opt: Self::Options) -> anyhow::Result<Self>;

    /// Update sequencer-specific indices when a new block is added.
    ///
    /// `from_block` should be the height of the chain the last time `refresh_indices` was called.
    /// Any blocks in the data sources with number `from_block` or greater will be incorporated into
    /// sequencer-specific data structures.
    async fn refresh_indices(&mut self, from_block: usize) -> anyhow::Result<()>;

    /// Retrieve a list of blocks whose timestamps fall within the window [start, end).
    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData>;

    /// Retrieve a list of blocks starting from `from` with timestamps less than `end`.
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
