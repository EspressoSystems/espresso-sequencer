use super::{data_source::SequencerDataSource, endpoints::TimeWindowQueryData};
use crate::{persistence::sql::Options, Header, SeqTypes};
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use hotshot_query_service::{
    availability::{AvailabilityDataSource, BlockId, ResourceId},
    data_source::sql::{include_migrations, Config, Migration, Query, SqlDataSource},
    QueryError, QueryResult,
};
use tokio_postgres::Row;

pub type DataSource = SqlDataSource<SeqTypes>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, reset: bool) -> anyhow::Result<Self> {
        let mut cfg =
            Config::default().migrations(include_migrations!("$CARGO_MANIFEST_DIR/api/migrations"));

        if let Some(host) = opt.host {
            cfg = cfg.host(host);
        }
        if let Some(port) = opt.port {
            cfg = cfg.port(port);
        }
        if let Some(database) = &opt.database {
            cfg = cfg.database(database);
        }
        if let Some(user) = &opt.user {
            cfg = cfg.user(user);
        }
        if let Some(password) = &opt.password {
            cfg = cfg.password(password);
        }
        if opt.use_tls {
            cfg = cfg.tls();
        }
        if reset {
            cfg = cfg.reset_schema();
        }

        Ok(cfg.connect().await?)
    }

    async fn refresh_indices(&mut self, _from_block: usize) -> anyhow::Result<()> {
        // We don't maintain any extra sequencer-specific indices ourselves. They are implicitly
        // managed by the Postgres engine.
        Ok(())
    }

    async fn window(&self, start: u64, end: u64) -> QueryResult<TimeWindowQueryData> {
        // Find all blocks whose timestamps fall within the window [start, end). Block timestamps
        // are monotonically increasing, so this query is guaranteed to return a contiguous range of
        // blocks ordered by increasing height. Note that we order by height explicitly, rather than
        // ordering by timestamp (which might be more efficient, since it could reuse the timestamp
        // index that is used in the WHERE clause) because multiple blocks may have the same
        // timestamp, due to the 1-second timestamp resolution.
        let query = "
            SELECT data
              FROM header
             WHERE (data->'timestamp')::bigint >= $1
               AND (data->'timestamp')::bigint < $2
             ORDER BY height";
        let rows = self
            .query(query, [&(start as i64), &(end as i64)])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
        let window: Vec<Header> = rows
            .map(|row| {
                parse_header(row.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        // Find the block just after the window.
        let query = "
            SELECT data
              FROM header
             WHERE (data->'timestamp')::bigint >= $1
             ORDER BY height
             LIMIT 1";
        let next = self
            .query_opt(query, [&(end as i64)])
            .await?
            .map(parse_header)
            .transpose()?;

        // If the `next` block exists, _or_ if any block in the window exists, we know we have
        // enough information to definitively say at least where the window starts (we may or may
        // not have where it ends, depending on how many blocks have thus far been produced).
        // However, if we have neither a block in the window nor a block after it, we cannot say
        // whether the next block produced will have a timestamp before or after the window start.
        // In this case, we don't know what the `prev` field of the response should be, so we return
        // an error: the caller must try again after more blocks have been produced.
        if window.is_empty() && next.is_none() {
            return Err(QueryError::NotFound);
        }

        // Find the block just before the window.
        let query = "
            SELECT data
              FROM header
             WHERE (data->'timestamp')::bigint < $1
             ORDER BY height DESC
             LIMIT 1";
        let prev = self
            .query_opt(query, [&(start as i64)])
            .await?
            .map(parse_header)
            .transpose()?;

        Ok(TimeWindowQueryData { window, prev, next })
    }

    async fn window_from<ID>(&self, from: ID, end: u64) -> QueryResult<TimeWindowQueryData>
    where
        ID: Into<BlockId<SeqTypes>> + Send + Sync,
    {
        // Find the specific block that starts the requested window.
        let first_block = match from.into() {
            ResourceId::Number(n) => n,
            ResourceId::Hash(h) => self.get_block(h).await?.height() as usize,
        };

        // Find all blocks starting from `first_block` with timestamps less than `end`. Block
        // timestamps are monotonically increasing, so this query is guaranteed to return a
        // contiguous range of blocks ordered by increasing height.
        let query = "
            SELECT data
              FROM header
             WHERE height >= $1
               AND (data->'timestamp')::bigint < $2
             ORDER BY height";
        let rows = self
            .query(query, [&(first_block as i64), &(end as i64)])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
        let window = rows
            .map(|row| {
                parse_header(row.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        // Find the block just before the window.
        let prev = if first_block > 0 {
            let prev = self.get_block(first_block - 1).await?;
            Some(prev.header().clone())
        } else {
            None
        };

        // Find the block just after the window.
        let query = "
            SELECT data
              FROM header
             WHERE (data->'timestamp')::bigint >= $1
             ORDER BY height
             LIMIT 1";
        let next = self
            .query_opt(query, [&(end as i64)])
            .await?
            .map(parse_header)
            .transpose()?;

        Ok(TimeWindowQueryData { window, prev, next })
    }
}

fn parse_header(row: Row) -> QueryResult<Header> {
    let data = row.try_get("data").map_err(|err| QueryError::Error {
        message: format!("error extracting header data from query results: {err}"),
    })?;
    serde_json::from_value(data).map_err(|err| QueryError::Error {
        message: format!("malformed header: {err}"),
    })
}

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::api::{self, data_source::testing::TestableSequencerDataSource};
    use hotshot_query_service::data_source::sql::testing::TmpDb;

    #[async_trait]
    impl TestableSequencerDataSource for DataSource {
        type Storage = TmpDb;

        async fn create_storage() -> Self::Storage {
            TmpDb::init().await
        }

        fn options(storage: &Self::Storage, opt: api::Options) -> api::Options {
            opt.query_sql(
                Default::default(),
                Options {
                    port: Some(storage.port()),
                    user: Some("postgres".into()),
                    password: Some("password".into()),
                    ..Default::default()
                },
            )
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::super::generic_tests;
    use super::DataSource;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_generic_tests!(DataSource);
}
