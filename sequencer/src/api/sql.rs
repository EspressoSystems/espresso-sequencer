use super::{
    data_source::SequencerDataSource, endpoints::TimeWindowQueryData, options::Sql as Options,
};
use crate::{network, Node, SeqTypes};
use async_trait::async_trait;
use futures::StreamExt;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId, ResourceId},
    data_source::sql::{include_migrations, Config, Migration, SqlDataSource},
    NotFoundSnafu, QueryError, QueryResult,
};
use snafu::OptionExt;

pub type DataSource<N> = SqlDataSource<SeqTypes, Node<N>>;

#[async_trait]
impl<N: network::Type> SequencerDataSource<N> for DataSource<N> {
    type Options = Options;

    async fn create(opt: Self::Options) -> anyhow::Result<Self> {
        let mut cfg =
            Config::default().migrations(include_migrations!("$CARGO_MANIFEST_DIR/api/migrations"));

        if let Some(host) = opt.host {
            cfg = cfg.host(host);
        }
        if let Some(port) = opt.port {
            cfg = cfg.port(port);
        }
        if let Some(database) = opt.database {
            cfg = cfg.database(database);
        }
        if let Some(user) = opt.user {
            cfg = cfg.user(user);
        }
        if let Some(password) = opt.password {
            cfg = cfg.password(password);
        }

        Ok(cfg.connect().await?)
    }

    async fn refresh_indices(&mut self, _from_block: usize) -> anyhow::Result<()> {
        // We don't maintain any extra sequencer-specific indices ourselves. They are implicitly
        // managed by the Postgres engine.
        Ok(())
    }

    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData> {
        // Find the first block with timestamp at least `start`. We will then use
        // `self.window_from()` to find the window starting from this block and ending with
        // timestamp `end`.
        //
        // Note that it would be better to simply select all blocks with timestamp between `start`
        // and `end` using a single SQL query. However, due to current limitations in the sequencer,
        // blocks are not necessarily ordered by timestamp, and as a consequence this query could
        // return a non-contiguous list of blocks, which is not allowed.
        //
        // TODO improve this method once block timestamps are monotonic.
        // https://github.com/EspressoSystems/espresso-sequencer/issues/794
        let query = "
            SELECT height
              FROM header
             WHERE (data->'timestamp')::bigint >= $1
             ORDER BY data->'timestamp', height
             LIMIT 1";
        let mut rows = self
            .query(query, [&(start as i64)])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?
            .boxed();
        let row = rows
            .next()
            .await
            .context(NotFoundSnafu)?
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
        let height: i64 = row.get("height");

        // Find the window from `height` to `end`.
        self.window_from(height as usize, end).await
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
        //
        // Note that it would be better to simply select all blocks with height >= `first_block` and
        // timestamp < `end` in a single SQL query. However, due to current limitations in the
        // sequencer, blocks are not necessarily ordered by timestamp, and as a consequence this
        // query could return a non-contiguous list of blocks, which is not allowed.
        //
        // TODO improve this method once block timestamps are monotonic.
        // https://github.com/EspressoSystems/espresso-sequencer/issues/794
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

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::{
        api::{self, data_source::testing::TestableSequencerDataSource},
        network::Memory,
    };
    use hotshot_query_service::data_source::sql::testing::TmpDb;

    #[async_trait]
    impl TestableSequencerDataSource for DataSource<Memory> {
        type Storage = TmpDb;

        async fn create_storage() -> Self::Storage {
            TmpDb::init().await
        }

        fn options(storage: &Self::Storage, opt: api::Options) -> api::Options {
            opt.query_sql(Options {
                port: Some(storage.port()),
                user: Some("postgres".into()),
                password: Some("password".into()),
                ..Default::default()
            })
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::super::generic_tests;
    use super::DataSource;
    use crate::network::Memory;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_generic_tests!(DataSource<Memory>);
}
