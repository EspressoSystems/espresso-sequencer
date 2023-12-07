use super::{
    data_source::SequencerDataSource, endpoints::TimeWindowQueryData, options::Fs as Options,
};
use crate::{network, Node, SeqTypes};
use async_trait::async_trait;
use futures::StreamExt;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId, BlockQueryData, ResourceId},
    data_source::{ExtensibleDataSource, FileSystemDataSource},
    NotFoundSnafu, QueryResult,
};
use snafu::OptionExt;
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug, Default)]
pub struct Index {
    blocks_by_time: BTreeMap<u64, Vec<u64>>,
}

pub type DataSource = ExtensibleDataSource<FileSystemDataSource<SeqTypes>, Index>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options) -> anyhow::Result<Self> {
        let storage_path = Path::new(&opt.storage_path);
        let data_source = {
            if opt.reset_store {
                FileSystemDataSource::create(storage_path)?
            } else {
                FileSystemDataSource::open(storage_path)?
            }
        };
        let mut index = Index::default();

        // Index blocks by timestamp.
        let mut blocks = data_source.get_block_range(..).await?;
        while let Some(block) = blocks.next().await {
            index_block_by_time(&mut index.blocks_by_time, &block?);
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
            .get_block_range(from_block..)
            .await?
            .enumerate()
            .collect()
            .await;
        for (i, block) in blocks {
            let Ok(block) = block else {
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
            ResourceId::Number(n) => n,
            ResourceId::Hash(h) => self.get_block(h).await?.height() as usize,
        };

        let mut res = TimeWindowQueryData::new(first_block as u64);

        // Include the block just before the start of the window, if there is one.
        if first_block > 0 {
            let prev = self.get_block(first_block - 1).await?;
            res.prev = Some(prev.block().header());
        }

        // Add blocks to the window, starting from `first_block`, until we reach the end of the
        // requested time window.
        let mut blocks = self.get_block_range(first_block..).await?;
        while let Some(block) = blocks.next().await {
            let block = block?;
            let header = block.block().header();
            if header.timestamp() >= end {
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
        .entry(block.block().timestamp())
        .or_default()
        .push(block.height());
}
