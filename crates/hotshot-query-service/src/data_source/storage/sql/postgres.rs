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

//! SQL storage backed by a PostgreSQL server.

use std::{collections::HashMap, time::Duration};

use anyhow::Context as _;
use futures::{FutureExt as _, stream::TryStreamExt};
use sqlx::{
    Row as _,
    pool::PoolOptions,
    postgres::{PgConnectOptions, PgRow, PgSslMode},
    query,
};

use super::{
    Backend, Config, Db, Migration, QueryBuilder, StateQueries, Transaction, Write,
    queries::state::Node, release_only_idle,
};
use crate::{QueryError, QueryResult, include_migrations};

impl Backend for Db {
    type ConnectOptions = PgConnectOptions;

    const EMBEDDED: bool = false;
    const MAX_FN: &'static str = "GREATEST";
    const BINARY_TYPE: &'static str = "BYTEA";

    fn default_connect_options() -> PgConnectOptions {
        PgConnectOptions::default()
            .username("postgres")
            .password("password")
            .host("localhost")
            .port(5432)
    }

    fn migrations() -> Vec<Migration> {
        include_migrations!("$CARGO_MANIFEST_DIR/migrations/postgres").collect()
    }

    fn delete_file(_opts: &PgConnectOptions) -> std::io::Result<()> {
        Ok(())
    }

    fn session_hooks(pool: PoolOptions<Db>, schema: String) -> PoolOptions<Db> {
        let pool = pool.after_connect(move |conn, _| {
            let schema = schema.clone();
            async move {
                query(&format!("SET search_path TO {schema}"))
                    .execute(conn)
                    .await?;
                Ok(())
            }
            .boxed()
        });
        release_only_idle(pool)
    }
}

impl StateQueries for Db {
    async fn hashes_by_id(
        conn: &mut sqlx::PgConnection,
        ids: Vec<i64>,
    ) -> QueryResult<HashMap<i64, Vec<u8>>> {
        Ok(
            sqlx::query_as("SELECT id, value FROM hash WHERE id = ANY($1::BIGINT[])")
                .bind(&ids)
                .fetch_all(conn)
                .await
                .map_err(|e| QueryError::Error {
                    message: format!("hash lookup failed: {e}"),
                })?
                .into_iter()
                .collect(),
        )
    }

    /// Batch insert hashes using UNNEST for large batches.
    async fn insert_hashes(
        tx: &mut Transaction<Write>,
        hashes: Vec<Vec<u8>>,
    ) -> QueryResult<HashMap<Vec<u8>, i64>> {
        if hashes.is_empty() {
            return Ok(HashMap::new());
        }

        // Use UNNEST-based batch insert (more efficient and avoids parameter limits).
        let sql = "INSERT INTO hash(value) SELECT * FROM UNNEST($1::bytea[]) ON CONFLICT (value) \
                   DO UPDATE SET value = EXCLUDED.value RETURNING value, id";

        let result: HashMap<Vec<u8>, i64> = sqlx::query_as(sql)
            .bind(&hashes)
            .fetch(tx.as_mut())
            .try_collect()
            .await
            .map_err(|e| QueryError::Error {
                message: format!("batch hash insert failed: {e}"),
            })?;

        Ok(result)
    }

    async fn upsert_nodes(
        name: &str,
        nodes: Vec<Node>,
        tx: &mut Transaction<Write>,
    ) -> anyhow::Result<()> {
        if nodes.is_empty() {
            return Ok(());
        }

        // Deduplicate nodes by (path, created) - keep the last occurrence
        // This is required because UNNEST + ON CONFLICT cannot handle duplicates in the same batch
        let mut deduped = HashMap::new();
        for node in nodes {
            deduped.insert((node.path.to_string(), node.created), node);
        }

        let mut paths = Vec::with_capacity(deduped.len());
        let mut createds = Vec::with_capacity(deduped.len());
        let mut hash_ids = Vec::with_capacity(deduped.len());
        let mut childrens = Vec::with_capacity(deduped.len());
        let mut children_bitvecs = Vec::with_capacity(deduped.len());
        let mut idxs = Vec::with_capacity(deduped.len());
        let mut entries = Vec::with_capacity(deduped.len());

        for node in deduped.into_values() {
            paths.push(node.path);
            createds.push(node.created);
            hash_ids.push(node.hash_id);
            childrens.push(node.children);
            children_bitvecs.push(node.children_bitvec);
            idxs.push(node.idx);
            entries.push(node.entry);
        }

        let sql = format!(
            r#"
            INSERT INTO "{name}" (path, created, hash_id, children, children_bitvec, idx, entry)
            SELECT * FROM UNNEST($1::jsonb[], $2::bigint[], $3::bigint[], $4::jsonb[], $5::bit varying[], $6::jsonb[], $7::jsonb[])
            ON CONFLICT (path, created) DO UPDATE SET
                hash_id = EXCLUDED.hash_id,
                children = EXCLUDED.children,
                children_bitvec = EXCLUDED.children_bitvec,
                idx = EXCLUDED.idx,
                entry = EXCLUDED.entry
            "#
        );

        sqlx::query(&sql)
            .bind(&paths)
            .bind(&createds)
            .bind(&hash_ids)
            .bind(&childrens)
            .bind(&children_bitvecs)
            .bind(&idxs)
            .bind(&entries)
            .execute(tx.as_mut())
            .await
            .context("batch upsert with UNNEST failed")?;

        Ok(())
    }

    fn node_from_row(row: PgRow) -> Node {
        Node {
            path: row.get_unchecked("path"),
            created: row.get_unchecked("created"),
            hash_id: row.get_unchecked("hash_id"),
            children: row.get_unchecked("children"),
            children_bitvec: row.get_unchecked("children_bitvec"),
            idx: row.get_unchecked("idx"),
            entry: row.get_unchecked("entry"),
        }
    }

    fn get_path_query<'q>(
        table: &'static str,
        paths: Vec<serde_json::Value>,
        created: i64,
    ) -> QueryResult<(QueryBuilder<'q>, String)> {
        let mut query = QueryBuilder::default();
        let paths_param = query.bind(paths)?;
        let created_param = query.bind(created)?;
        // One LATERAL point-lookup per path in the array, instead of N UNION'd subqueries.
        // Postgres plans the inner subquery once and reuses it, collapsing N independent
        // planner decisions into one.
        let sql = format!(
            "SELECT n.* FROM unnest({paths_param}::jsonb[]) AS p(path), LATERAL (SELECT * FROM \
             {table} WHERE {table}.path = p.path AND {table}.created <= {created_param} ORDER BY \
             {table}.path DESC, {table}.created DESC LIMIT 1) AS n ORDER BY n.path DESC"
        );
        Ok((query, sql))
    }
}

impl From<PgConnectOptions> for Config {
    fn from(db_opt: PgConnectOptions) -> Self {
        Self::with_connect_options(db_opt)
    }
}

impl Config {
    /// Set the hostname of the database server.
    ///
    /// The default is `localhost`.
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.db_opt = self.db_opt.host(&host.into());
        self
    }

    /// Set the port on which to connect to the database.
    ///
    /// The default is 5432, the default Postgres port.
    pub fn port(mut self, port: u16) -> Self {
        self.db_opt = self.db_opt.port(port);
        self
    }

    /// Set the DB user to connect as.
    pub fn user(mut self, user: &str) -> Self {
        self.db_opt = self.db_opt.username(user);
        self
    }

    /// Set a password for connecting to the database.
    pub fn password(mut self, password: &str) -> Self {
        self.db_opt = self.db_opt.password(password);
        self
    }

    /// Set the name of the database to connect to.
    pub fn database(mut self, database: &str) -> Self {
        self.db_opt = self.db_opt.database(database);
        self
    }

    /// Use TLS for an encrypted connection to the database.
    ///
    /// Note that an encrypted connection may be established even if this option is not set, as long
    /// as both the client and server support it. This option merely causes connection to fail if an
    /// encrypted stream cannot be established.
    pub fn tls(mut self) -> Self {
        self.db_opt = self.db_opt.ssl_mode(PgSslMode::Require);
        self
    }

    pub fn connect_options(&self) -> &PgConnectOptions {
        &self.db_opt
    }

    /// Set the name of the schema to use for queries.
    ///
    /// The default schema is named `hotshot` and is created via the default migrations.
    pub fn schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = schema.into();
        self
    }

    /// Set the maximum time a single SQL statement is allowed to run before being canceled.
    ///
    /// This helps prevent queries from running indefinitely even when the client is dropped
    pub fn statement_timeout(mut self, timeout: Duration) -> Self {
        // Format duration as milliseconds
        // PostgreSQL interprets values without units as milliseconds
        let timeout_ms = timeout.as_millis();
        self.db_opt = self
            .db_opt
            .options([("statement_timeout", timeout_ms.to_string())]);
        self
    }
}

#[cfg(all(any(test, feature = "testing"), not(target_os = "windows")))]
pub(super) mod tmp_db {
    use super::super::{
        Config,
        testing::{TmpPostgres, test_migrations},
    };

    #[derive(Debug)]
    pub struct TmpDb {
        postgres: TmpPostgres,
    }

    impl TmpDb {
        pub async fn init() -> Self {
            Self {
                postgres: TmpPostgres::init().await,
            }
        }

        pub fn host(&self) -> String {
            self.postgres.host()
        }

        pub fn port(&self) -> u16 {
            self.postgres.port()
        }

        pub fn config(&self) -> Config {
            Config::default()
                .user("postgres")
                .password("password")
                .host(self.host())
                .port(self.port())
                .migrations(test_migrations())
        }
    }
}

#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use std::str::FromStr;

    use futures::FutureExt as _;
    use sqlx::{Connection as _, Executor as _, Row as _};

    use super::super::{Config, SqlStorage, StorageConnectionType, testing::TmpDb};

    #[test]
    fn test_config_from_str() {
        let cfg = Config::from_str("postgresql://user:password@host:8080").unwrap();
        assert_eq!(cfg.db_opt.get_username(), "user");
        assert_eq!(cfg.db_opt.get_host(), "host");
        assert_eq!(cfg.db_opt.get_port(), 8080);
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_cancelled_begin_does_not_leak_transaction() {
        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(
            db.config().max_connections(1),
            StorageConnectionType::Sequencer,
        )
        .await
        .unwrap();

        let mut conn = storage.pool().acquire().await.unwrap();
        assert!(
            conn.begin().now_or_never().is_none(),
            "BEGIN finished on the first poll, so nothing was cancelled"
        );
        drop(conn);

        let mut conn = storage.pool().acquire().await.unwrap();
        let row = conn
            .fetch_one(sqlx::raw_sql(
                "SELECT transaction_timestamp() <> statement_timestamp()",
            ))
            .await
            .unwrap();
        assert!(
            !row.get::<bool, _>(0),
            "the connection came back from the pool inside the cancelled transaction"
        );
    }
}
