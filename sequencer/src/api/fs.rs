use super::{
    data_source::{Provider, SequencerDataSource},
    endpoints::TimeWindowQueryData,
};
use crate::{persistence::fs::Options, SeqTypes};
use async_trait::async_trait;
use futures::StreamExt;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId, BlockQueryData},
    data_source::{ExtensibleDataSource, FileSystemDataSource},
    node::NodeDataSource,
    NotFoundSnafu, QueryError, QueryResult,
};
use snafu::OptionExt;
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug, Default)]
pub struct Index {
    blocks_by_time: BTreeMap<u64, Vec<u64>>,
}

pub type DataSource = ExtensibleDataSource<FileSystemDataSource<SeqTypes, Provider>, Index>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        let path = Path::new(&opt.path);
        let data_source = {
            if reset {
                FileSystemDataSource::create(path, provider).await?
            } else {
                FileSystemDataSource::open(path, provider).await?
            }
        };
        let mut index = Index::default();

        // Index blocks by timestamp.
        let mut blocks = data_source
            .get_block_range(..data_source.block_height().await?)
            .await;
        while let Some(block) = blocks.next().await {
            index_block_by_time(
                &mut index.blocks_by_time,
                &block.try_resolve().map_err(|_| QueryError::Missing)?,
            );
        }
        drop(blocks);

        Ok(ExtensibleDataSource::new(data_source, index))
    }

    async fn refresh_indices(&mut self, from_block: usize) -> anyhow::Result<()> {
        // We can't update the index in `self.as_mut()` at the same time as the stream
        // `self.get_block_range()` is live, since that would require conflicting borrows against
        // `self`. By collecting the stream into a vector, we drop our borrow of `self`. This
        // function is called every time a new block is added so this usually requires loading only
        // one block into memory, and rarely very many.
        let blocks: Vec<_> = self
            .get_block_range(from_block..self.block_height().await?)
            .await
            .enumerate()
            .collect()
            .await;
        for (i, block) in blocks {
            let Ok(block) = block.try_resolve() else {
                tracing::warn!("missing block {}, index may be out of date", from_block + i);
                continue;
            };
            index_block_by_time(&mut self.as_mut().blocks_by_time, &block);
        }

        Ok(())
    }

    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData> {
        // Find the minimum timestamp which is at least `start`, and all the blocks with that
        // timestamp.
        let blocks = self
            .as_ref()
            .blocks_by_time
            .range(start..)
            .next()
            .context(NotFoundSnafu)?
            .1;
        // Multiple blocks can have the same timestamp (when truncated to seconds); we want the
        // first one. It is an invariant that any timestamp which has an entry in `blocks_by_time`
        // has a non-empty list associated with it, so this indexing is safe.
        let first_block = blocks[0];
        self.window_from(first_block as usize, end).await
    }

    async fn window_from<ID>(&self, from: ID, end: u64) -> QueryResult<TimeWindowQueryData>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        let first_block = match from.into() {
            BlockId::Number(n) => n,
            id => self
                .get_block(id)
                .await
                .try_resolve()
                .map_err(|_| QueryError::Missing)?
                .height() as usize,
        };

        let mut res = TimeWindowQueryData::default();

        // Include the block just before the start of the window, if there is one.
        if first_block > 0 {
            let prev = self
                .get_block(first_block - 1)
                .await
                .try_resolve()
                .map_err(|_| QueryError::Missing)?;
            res.prev = Some(prev.header().clone());
        }

        // Add blocks to the window, starting from `first_block`, until we reach the end of the
        // requested time window.
        let mut blocks = self
            .get_block_range(first_block..self.block_height().await?)
            .await;
        while let Some(block) = blocks.next().await {
            let block = block.try_resolve().map_err(|_| QueryError::Missing)?;
            let header = block.header().clone();
            if header.timestamp >= end {
                res.next = Some(header);
                break;
            }
            res.window.push(header);
        }

        Ok(res)
    }
}

fn index_block_by_time(
    blocks_by_time: &mut BTreeMap<u64, Vec<u64>>,
    block: &BlockQueryData<SeqTypes>,
) {
    blocks_by_time
        .entry(block.header().timestamp)
        .or_default()
        .push(block.height());
}

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::{
        api::{self, data_source::testing::TestableSequencerDataSource},
        persistence::{fs, PersistenceOptions},
    };
    use tempfile::TempDir;

    #[async_trait]
    impl TestableSequencerDataSource for DataSource {
        type Storage = TempDir;
        type Persistence = fs::Persistence;

        async fn create_storage() -> Self::Storage {
            TempDir::new().unwrap()
        }

        async fn connect(storage: &Self::Storage) -> Self::Persistence {
            Options {
                path: storage.path().into(),
            }
            .create()
            .await
            .unwrap()
        }

        fn options(storage: &Self::Storage, opt: api::Options) -> api::Options {
            opt.query_fs(
                Default::default(),
                Options {
                    path: storage.path().into(),
                },
            )
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::super::api_tests;
    use super::DataSource;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_api_tests!(DataSource);
}
