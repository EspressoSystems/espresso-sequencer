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

#![cfg(feature = "sql-data-source")]

use super::{pruning::PrunedHeightStorage, AvailabilityStorage, ExplorerStorage};
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionIndex, UpdateAvailabilityData,
        VidCommonQueryData,
    },
    data_source::{
        storage::pruning::{PruneStorage, PrunerCfg, PrunerConfig},
        VersionedDataSource,
    },
    explorer::{self, data_source::BlockSummary},
    merklized_state::{
        MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence, Snapshot,
        UpdateStateData,
    },
    node::{NodeDataSource, SyncStatus, TimeWindowQueryData, WindowStart},
    task::BackgroundTask,
    types::HeightIndexed,
    Header, Leaf, MissingSnafu, NotFoundSnafu, Payload, QueryError, QueryResult, VidShare,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use async_std::{net::ToSocketAddrs, sync::Arc, task::sleep};
use async_trait::async_trait;
use bit_vec::BitVec;
use chrono::Utc;
use committable::Committable;
use futures::{
    stream::{BoxStream, StreamExt, TryStreamExt},
    task::{Context, Poll},
    AsyncRead, AsyncWrite,
};
use hotshot_types::{
    simple_certificate::QuorumCertificate,
    traits::{
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::NodeType,
    },
};
use itertools::{izip, Itertools};
use jf_primitives::merkle_tree::{
    prelude::{MerkleNode, MerkleProof},
    DigestAlgorithm, MerkleCommitment, ToTraversalPath,
};
use postgres_native_tls::TlsConnector;
use snafu::OptionExt;
use std::{
    borrow::Cow,
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    num::NonZeroUsize,
    ops::{Bound, RangeBounds},
    pin::Pin,
    str::FromStr,
    time::Duration,
};
use tokio_postgres::{
    config::Host,
    tls::TlsConnect,
    types::{private::BytesMut, to_sql_checked, BorrowToSql, FromSql, ToSql, Type},
    Client, NoTls, Row, ToStatement,
};

pub use anyhow::Error;
// This needs to be reexported so that we can reference it by absolute path relative to this crate
// in the expansion of `include_migrations`, even when `include_migrations` is invoked from another
// crate which doesn't have `include_dir` as a dependency.
pub use crate::include_migrations;
pub use include_dir::include_dir;
pub use refinery::Migration;
pub use tokio_postgres as postgres;

/// Embed migrations from the given directory into the current binary.
///
/// The macro invocation `include_migrations!(path)` evaluates to an expression of type `impl
/// Iterator<Item = Migration>`. Each migration must be a text file which is an immediate child of
/// `path`, and there must be no non-migration files in `path`. The migration files must have names
/// of the form `V${version}__${name}.sql`, where `version` is a positive integer indicating how the
/// migration is to be ordered relative to other migrations, and `name` is a descriptive name for
/// the migration.
///
/// `path` should be an absolute path. It is possible to give a path relative to the root of the
/// invoking crate by using environment variable expansions and the `CARGO_MANIFEST_DIR` environment
/// variable.
///
/// As an example, this is the invocation used to load the default migrations from the
/// `hotshot-query-service` crate. The migrations are located in a directory called `migrations` at
/// the root of the crate.
///
/// ```
/// # use hotshot_query_service::data_source::sql::{include_migrations, Migration};
/// let migrations: Vec<Migration> =
///     include_migrations!("$CARGO_MANIFEST_DIR/migrations").collect();
/// assert_eq!(migrations[0].version(), 10);
/// assert_eq!(migrations[0].name(), "init_schema");
/// ```
///
/// Note that a similar macro is available from Refinery:
/// [embed_migrations](https://docs.rs/refinery/0.8.11/refinery/macro.embed_migrations.html). This
/// macro differs in that it evaluates to an iterator of [migrations](Migration), making it an
/// expression macro, while `embed_migrations` is a statement macro that defines a module which
/// provides access to the embedded migrations only indirectly via a
/// [`Runner`](https://docs.rs/refinery/0.8.11/refinery/struct.Runner.html). The direct access to
/// migrations provided by [`include_migrations`] makes this macro easier to use with
/// [`Config::migrations`], for combining custom migrations with [`default_migrations`].
#[macro_export]
macro_rules! include_migrations {
    ($dir:tt) => {
        $crate::data_source::storage::sql::include_dir!($dir)
            .files()
            .map(|file| {
                let path = file.path();
                let name = path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or_else(|| {
                        panic!(
                            "migration file {} must have a non-empty UTF-8 name",
                            path.display()
                        )
                    });
                let sql = file
                    .contents_utf8()
                    .unwrap_or_else(|| panic!("migration file {name} must use UTF-8 encoding"));
                $crate::data_source::storage::sql::Migration::unapplied(name, sql)
                    .expect("invalid migration")
            })
    };
}

/// The migrations requied to build the default schema for this version of [`SqlStorage`].
pub fn default_migrations() -> Vec<Migration> {
    let mut migrations = include_migrations!("$CARGO_MANIFEST_DIR/migrations").collect::<Vec<_>>();

    // Check version uniqueness and sort by version.
    validate_migrations(&mut migrations).expect("default migrations are invalid");

    // Check that all migration versions are multiples of 10, so that custom migrations can be
    // inserted in between.
    for m in &migrations {
        if m.version() == 0 || m.version() % 10 != 0 {
            panic!(
                "default migration version {} is not a positive multiple of 10",
                m.version()
            );
        }
    }

    migrations
}

/// Validate and preprocess a sequence of migrations.
///
/// * Ensure all migrations have distinct versions
/// * Ensure migrations are sorted by increasing version
fn validate_migrations(migrations: &mut [Migration]) -> Result<(), Error> {
    migrations.sort_by_key(|m| m.version());

    // Check version uniqueness.
    for (prev, next) in migrations.iter().zip(migrations.iter().skip(1)) {
        if next <= prev {
            return Err(Error::msg(format!(
                "migration versions are not strictly increasing ({prev}->{next})"
            )));
        }
    }

    Ok(())
}

/// Add custom migrations to a default migration sequence.
///
/// Migrations in `custom` replace migrations in `default` with the same version. Otherwise, the two
/// sequences `default` and `custom` are merged so that the resulting sequence is sorted by
/// ascending version number. Each of `default` and `custom` is assumed to be the output of
/// [`validate_migrations`]; that is, each is sorted by version and contains no duplicate versions.
fn add_custom_migrations(
    default: impl IntoIterator<Item = Migration>,
    custom: impl IntoIterator<Item = Migration>,
) -> impl Iterator<Item = Migration> {
    default
        .into_iter()
        // Merge sorted lists, joining pairs of equal version into `EitherOrBoth::Both`.
        .merge_join_by(custom, |l, r| l.version().cmp(&r.version()))
        // Prefer the custom migration for a given version when both default and custom versions
        // are present.
        .map(|pair| pair.reduce(|_, custom| custom))
}

/// Postgres client config.
#[derive(Clone, Debug)]
pub struct Config {
    pgcfg: postgres::Config,
    host: String,
    port: u16,
    schema: String,
    reset: bool,
    migrations: Vec<Migration>,
    no_migrations: bool,
    tls: bool,
    pruner_cfg: Option<PrunerCfg>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pgcfg: Default::default(),
            host: "localhost".into(),
            port: 5432,
            schema: "hotshot".into(),
            reset: false,
            migrations: vec![],
            no_migrations: false,
            tls: false,
            pruner_cfg: None,
        }
    }
}

impl From<postgres::Config> for Config {
    fn from(pgcfg: postgres::Config) -> Self {
        // We connect via TCP manually, without using the host and port from pgcfg. So we need to
        // pull those out of pgcfg if they have been specified, to override the defaults.
        let host = match pgcfg.get_hosts().first() {
            Some(Host::Tcp(host)) => host.to_string(),
            _ => "localhost".into(),
        };
        let port = *pgcfg.get_ports().first().unwrap_or(&5432);
        Self {
            pgcfg,
            host,
            port,
            ..Default::default()
        }
    }
}

impl FromStr for Config {
    type Err = <postgres::Config as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(postgres::Config::from_str(s)?.into())
    }
}

impl Config {
    /// Set the hostname of the database server.
    ///
    /// The default is `localhost`.
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Set the port on which to connect to the database.
    ///
    /// The default is 5432, the default Postgres port.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the DB user to connect as.
    pub fn user(mut self, user: &str) -> Self {
        self.pgcfg.user(user);
        self
    }

    /// Set a password for connecting to the database.
    pub fn password(mut self, password: &str) -> Self {
        self.pgcfg.password(password);
        self
    }

    /// Set the name of the database to connect to.
    pub fn database(mut self, database: &str) -> Self {
        self.pgcfg.dbname(database);
        self
    }

    /// Set the name of the schema to use for queries.
    ///
    /// The default schema is named `hotshot` and is created via the default migrations.
    pub fn schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = schema.into();
        self
    }

    /// Reset the schema on connection.
    ///
    /// When this [`Config`] is used to [`connect`](Self::connect) a
    /// [`SqlDataSource`](crate::data_source::SqlDataSource), if this option is set, the relevant
    /// [`schema`](Self::schema) will first be dropped and then recreated, yielding a completely
    /// fresh instance of the query service.
    ///
    /// This is a particularly useful capability for development and staging environments. Still, it
    /// must be used with extreme caution, as using this will irrevocably delete any data pertaining
    /// to the query service in the database.
    pub fn reset_schema(mut self) -> Self {
        self.reset = true;
        self
    }

    /// Add custom migrations to run when connecting to the database.
    pub fn migrations(mut self, migrations: impl IntoIterator<Item = Migration>) -> Self {
        self.migrations.extend(migrations);
        self
    }

    /// Skip all migrations when connecting to the database.
    pub fn no_migrations(mut self) -> Self {
        self.no_migrations = true;
        self
    }

    /// Use TLS for an encrypted connection to the database.
    pub fn tls(mut self) -> Self {
        self.tls = true;
        self
    }

    pub fn pruner_cfg(mut self, cfg: PrunerCfg) -> Result<Self, Error> {
        cfg.validate()?;
        self.pruner_cfg = Some(cfg);
        Ok(self)
    }
}

/// Storage for the APIs provided in this crate, backed by a remote PostgreSQL database.
#[derive(Debug)]
pub struct SqlStorage {
    client: Arc<Client>,
    tx_in_progress: bool,
    pruner_cfg: Option<PrunerCfg>,
    _connection: BackgroundTask,
}

impl SqlStorage {
    /// Connect to a remote database.
    pub async fn connect(mut config: Config) -> Result<Self, Error> {
        // Establish a TCP connection to the server.
        let tcp = TcpStream::connect((config.host.as_str(), config.port)).await?;

        // Convert the TCP connection into a postgres connection.
        let (mut client, connection) = if config.tls {
            let tls = TlsConnector::new(native_tls::TlsConnector::new()?, config.host.as_str());
            connect(config.pgcfg, tcp, tls).await?
        } else {
            connect(config.pgcfg, tcp, NoTls).await?
        };

        // Create or connect to the schema for this query service.
        if config.reset {
            client
                .batch_execute(&format!("DROP SCHEMA IF EXISTS {} CASCADE", config.schema))
                .await?;
        }
        client
            .batch_execute(&format!("CREATE SCHEMA IF NOT EXISTS {}", config.schema))
            .await?;
        client
            .batch_execute(&format!("SET search_path TO {}", config.schema))
            .await?;

        // Enable ltree extension
        // this is used for state storage
        client
            .batch_execute("CREATE EXTENSION IF NOT EXISTS ltree")
            .await?;

        // Get migrations and interleave with custom migrations, sorting by version number.
        validate_migrations(&mut config.migrations)?;
        let migrations =
            add_custom_migrations(default_migrations(), config.migrations).collect::<Vec<_>>();

        // Get a migration runner. Depending on the config, we can either use this to actually run
        // the migrations or just check if the database is up to date.
        let runner = refinery::Runner::new(&migrations).set_grouped(true);

        if config.no_migrations {
            // We've been asked not to run any migrations. Abort if the DB is not already up to
            // date.
            let last_applied = runner.get_last_applied_migration_async(&mut client).await?;
            let last_expected = migrations.last();
            if last_applied.as_ref() != last_expected {
                return Err(Error::msg(format!(
                    "DB is out of date: last applied migration is {last_applied:?}, but expected {last_expected:?}"
                )));
            }
        } else {
            // Run migrations using `refinery`.
            match runner.run_async(&mut client).await {
                Ok(report) => {
                    tracing::info!("ran DB migrations: {report:?}");
                }
                Err(err) => {
                    tracing::error!("DB migrations failed: {:?}", err.report());
                    Err(err)?;
                }
            }
        }

        Ok(Self {
            client: Arc::new(client),
            tx_in_progress: false,
            pruner_cfg: config.pruner_cfg,
            _connection: connection,
        })
    }

    /// Access the transaction which is accumulating all uncommitted changes to the data source.
    pub async fn transaction(&mut self) -> QueryResult<Transaction<'_>> {
        if !self.tx_in_progress {
            // If there is no transaction in progress, open one.
            self.client
                .batch_execute("BEGIN")
                .await
                .map_err(postgres_err)?;
            self.tx_in_progress = true;
        }
        Ok(Transaction {
            client: Cow::Borrowed(&self.client),
        })
    }
}

impl PrunerConfig for SqlStorage {
    fn set_pruning_config(&mut self, cfg: PrunerCfg) {
        self.pruner_cfg = Some(cfg);
    }

    fn get_pruning_config(&self) -> Option<PrunerCfg> {
        self.pruner_cfg.clone()
    }
}

impl SqlStorage {
    async fn get_minimum_height(&self) -> QueryResult<Option<u64>> {
        let row = self
            .query_one_static("SELECT MIN(height) as height FROM header")
            .await?;

        let height = row.get::<_, Option<i64>>(0).map(|h| h as u64);

        Ok(height)
    }

    async fn get_height_by_timestamp(&self, timestamp: i64) -> QueryResult<Option<u64>> {
        let row = self
            .query_opt(
                "SELECT height FROM header WHERE timestamp <= $1 ORDER BY height DESC LIMIT 1",
                [&timestamp],
            )
            .await?;

        let height = row.map(|row| row.get::<_, i64>(0) as u64);

        Ok(height)
    }

    /// Load a header from storage.
    ///
    /// This function is similar to `AvailabilityStorage::get_header`, but
    /// * does not require the `QueryablePayload` bound that that trait impl does
    /// * makes it easier to specify types since the type parameter is on the function and not on a
    ///   trait impl
    /// * allows type conversions for the `id` parameter
    /// This more ergonomic interface is useful as loading headers is important for many SQL storage
    /// functions, not just the `AvailabilityStorage` interface.
    async fn load_header<Types: NodeType>(
        &self,
        id: impl Into<BlockId<Types>>,
    ) -> QueryResult<Header<Types>> {
        let (where_clause, param) = header_where_clause(id.into());
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_header::<Types>(row)
    }
}

#[async_trait]
impl PrunedHeightStorage for SqlStorage {
    type Error = QueryError;
    async fn save_pruned_height(&mut self, height: u64) -> Result<(), Self::Error> {
        // id is set to 1 so that there is only one row in the table.
        // height is updated if the row already exists.
        self.transaction()
            .await?
            .upsert(
                "pruned_height",
                ["id", "last_height"],
                ["id"],
                [[sql_param(&(1_i32)), sql_param(&(height as i64))]],
            )
            .await
    }

    async fn load_pruned_height(&self) -> Result<Option<u64>, Self::Error> {
        let row = self
            .query_opt_static("SELECT last_height FROM pruned_height ORDER BY id DESC LIMIT 1")
            .await?;

        let height = row.map(|row| row.get::<_, i64>(0) as u64);

        Ok(height)
    }
}

#[async_trait]
impl PruneStorage for SqlStorage {
    async fn get_disk_usage(&self) -> QueryResult<u64> {
        let row = self
            .query_one_static("SELECT pg_database_size(current_database())")
            .await?;
        let size: i64 = row.get(0);
        Ok(size as u64)
    }

    async fn prune(&mut self) -> Result<Option<u64>, QueryError> {
        let cfg = self.get_pruning_config().ok_or(QueryError::Error {
            message: "Pruning config not found".to_string(),
        })?;

        let Some(mut height) = self.get_minimum_height().await? else {
            tracing::info!("database is empty, nothing to prune");
            return Ok(None);
        };

        let batch_size = cfg.batch_size();
        let max_usage = cfg.max_usage();
        let mut pruned_height = None;
        // Prune data exceeding target retention in batches
        let target_height = self
            .get_height_by_timestamp(
                Utc::now().timestamp() - (cfg.target_retention().as_secs()) as i64,
            )
            .await?;

        if let Some(target_height) = target_height {
            while height < target_height {
                height = min(height + batch_size, target_height);
                self.query_opt("DELETE FROM header WHERE height <= $1", &[&(height as i64)])
                    .await?;
                pruned_height = Some(height);

                tracing::info!("Pruned data up to height {height}");
            }
        }

        // If threshold is set, prune data exceeding minimum retention in batches
        // This parameter is needed for SQL storage as there is no direct way to get free space.
        if let Some(threshold) = cfg.pruning_threshold() {
            let mut usage = self.get_disk_usage().await?;

            // Prune data exceeding minimum retention in batches starting from minimum height
            // until usage is below threshold
            if usage > threshold {
                tracing::warn!(
                    "Disk usage {usage} exceeds pruning threshold {:?}",
                    cfg.pruning_threshold()
                );
                let minimum_retention_height = self
                    .get_height_by_timestamp(
                        Utc::now().timestamp() - (cfg.minimum_retention().as_secs()) as i64,
                    )
                    .await?;

                if let Some(min_retention_height) = minimum_retention_height {
                    while (usage as f64 / threshold as f64) > (f64::from(max_usage) / 10000.0)
                        && height < min_retention_height
                    {
                        height = min(height + batch_size, min_retention_height);
                        self.query_opt(
                            "DELETE FROM header WHERE height <= $1",
                            &[&(height as i64)],
                        )
                        .await?;
                        usage = self.get_disk_usage().await?;
                        pruned_height = Some(height);
                        tracing::info!("Pruned data up to height {height}");
                    }
                }
            }
        }
        // Vacuum the database to reclaim space.
        // Note: VACUUM FULL is not used as it requires an exclusive lock on the tables, which can
        // cause downtime for the query service.
        self.client
            .batch_execute("VACUUM")
            .await
            .map_err(postgres_err)?;

        Ok(pruned_height)
    }
}

#[async_trait]
impl Query for SqlStorage {
    async fn client(&self) -> Cow<Arc<Client>> {
        Cow::Borrowed(&self.client)
    }
}

#[async_trait]
impl VersionedDataSource for SqlStorage {
    type Error = postgres::error::Error;

    async fn commit(&mut self) -> Result<(), Self::Error> {
        if self.tx_in_progress {
            self.client.batch_execute("COMMIT").await?;
            self.tx_in_progress = false;
        }
        Ok(())
    }

    async fn revert(&mut self) {
        if self.tx_in_progress {
            // If we're trying to roll back a transaction, something has already gone wrong and
            // we're trying to recover. If we're unable to revert the changes and recover, all we
            // can do is panic.
            self.client
                .batch_execute("ROLLBACK")
                .await
                .expect("DB rollback succeeds");
            self.tx_in_progress = false;
        }
    }
}

#[async_trait]
impl<Types> AvailabilityStorage<Types> for SqlStorage
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    Header<Types>: QueryableHeader<Types>,
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id {
            LeafId::Number(n) => ("height = $1", Box::new(n as i64)),
            LeafId::Hash(h) => ("hash = $1", Box::new(h.to_string())),
        };
        let query = format!("SELECT leaf, qc FROM leaf WHERE {where_clause}");
        let row = self.query_one(&query, [param]).await?;
        parse_leaf(row)
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_block(row)
    }

    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        self.load_header(id).await
    }

    async fn get_payload(&self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {PAYLOAD_COLUMNS}
               FROM header AS h
               JOIN payload AS p ON h.height = p.height
               WHERE {where_clause}
               ORDER BY h.height ASC
               LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_payload(row)
    }

    async fn get_vid_common(&self, id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {VID_COMMON_COLUMNS}
               FROM header AS h
               JOIN vid AS v ON h.height = v.height
               WHERE {where_clause}
               ORDER BY h.height ASC
               LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_vid_common(row)
    }

    async fn get_leaf_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "height");
        let query = format!("SELECT leaf, qc FROM leaf {where_clause} ORDER BY height ASC");
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_leaf(res?)).collect().await)
    }

    async fn get_block_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_block(res?)).collect().await)
    }

    async fn get_payload_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {PAYLOAD_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_payload(res?)).collect().await)
    }

    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {VID_COMMON_COLUMNS}
              FROM header AS h
              JOIN vid AS v ON h.height = v.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_vid_common(res?)).collect().await)
    }

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        // ORDER BY ASC ensures that if there are duplicate transactions, we return the first
        // one.
        let query = format!(
            "SELECT {BLOCK_COLUMNS}, t.index AS tx_index
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                JOIN transaction AS t ON t.block_height = h.height
                WHERE t.hash = $1
                ORDER BY (t.block_height, t.index) ASC
                LIMIT 1"
        );
        let row = self.query_one(&query, &[&hash.to_string()]).await?;

        // Extract the transaction index.
        let index = row.try_get("tx_index").map_err(|err| QueryError::Error {
            message: format!("error extracting transaction index from query results: {err}"),
        })?;
        let index: TransactionIndex<Types> =
            serde_json::from_value(index).map_err(|err| QueryError::Error {
                message: format!("malformed transaction index: {err}"),
            })?;

        // Extract the block.
        let block = parse_block(row)?;

        Ok((block, index))
    }
}

#[async_trait]
impl<Types> UpdateAvailabilityData<Types> for SqlStorage
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    Header<Types>: QueryableHeader<Types>,
{
    type Error = QueryError;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        let mut tx = self.transaction().await?;

        // While we don't necessarily have the full block for this leaf yet, we can initialize the
        // header table with block metadata taken from the leaf.
        let header_json = serde_json::to_value(leaf.leaf().get_block_header()).map_err(|err| {
            QueryError::Error {
                message: format!("failed to serialize header: {err}"),
            }
        })?;
        tx.upsert(
            "header",
            ["height", "hash", "payload_hash", "data", "timestamp"],
            ["height"],
            [[
                sql_param(&(leaf.height() as i64)),
                sql_param(&leaf.block_hash().to_string()),
                sql_param(
                    &leaf
                        .leaf()
                        .get_block_header()
                        .payload_commitment()
                        .to_string(),
                ),
                sql_param(&header_json),
                sql_param(&(leaf.leaf().get_block_header().timestamp() as i64)),
            ]],
        )
        .await?;

        // Similarly, we can initialize the payload table with a null payload, which can help us
        // distinguish between blocks that haven't been produced yet and blocks we haven't received
        // yet when answering queries.
        tx.upsert("payload", ["height"], ["height"], [[leaf.height() as i64]])
            .await?;

        // Finally, we insert the leaf itself, which references the header row we created.
        // Serialize the full leaf and QC to JSON for easy storage.
        let leaf_json = serde_json::to_value(leaf.leaf()).map_err(|err| QueryError::Error {
            message: format!("failed to serialize leaf: {err}"),
        })?;
        let qc_json = serde_json::to_value(leaf.qc()).map_err(|err| QueryError::Error {
            message: format!("failed to serialize QC: {err}"),
        })?;
        tx.upsert(
            "leaf",
            ["height", "hash", "block_hash", "leaf", "qc"],
            ["height"],
            [[
                sql_param(&(leaf.height() as i64)),
                sql_param(&leaf.hash().to_string()),
                sql_param(&leaf.block_hash().to_string()),
                sql_param(&leaf_json),
                sql_param(&qc_json),
            ]],
        )
        .await?;

        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        let mut tx = self.transaction().await?;

        // The header and payload tables should already have been initialized when we inserted the
        // corresponding leaf. All we have to do is add the payload itself and its size.
        let payload = block.payload.encode().map_err(|err| QueryError::Error {
            message: format!("failed to serialize block: {err}"),
        })?;
        tx.upsert(
            "payload",
            ["height", "data", "size"],
            ["height"],
            [[
                sql_param(&(block.height() as i64)),
                sql_param(&payload.as_ref()),
                sql_param(&(block.size() as i32)),
            ]],
        )
        .await?;

        // Index the transactions in the block. For each transaction, collect, separately, its hash,
        // height, and index. These items all have different types, so we collect them into
        // different vecs.
        let mut tx_hashes = vec![];
        let mut tx_block_heights = vec![];
        let mut tx_indexes = vec![];
        for (txn_ix, txn) in block.enumerate() {
            let txn_ix = serde_json::to_value(&txn_ix).map_err(|err| QueryError::Error {
                message: format!("failed to serialize transaction index: {err}"),
            })?;
            tx_hashes.push(txn.commit().to_string());
            tx_block_heights.push(block.height() as i64);
            tx_indexes.push(txn_ix);
        }
        if !tx_hashes.is_empty() {
            tx.upsert(
                "transaction",
                ["hash", "block_height", "index"],
                ["block_height", "index"],
                // Now that we have the transaction hashes, block heights, and indexes collected in
                // memory, we can combine them all into a single vec using type erasure: all the
                // values get converted to `&dyn ToSql`. The references all borrow from one of
                // `tx_hashes`, `tx_block_heights`, or `tx_indexes`, which all outlive this function
                // call, so the lifetimes work out.
                izip!(
                    tx_hashes.iter().map(sql_param),
                    tx_block_heights.iter().map(sql_param),
                    tx_indexes.iter().map(sql_param),
                )
                .map(|(hash, height, index)| [hash, height, index]),
            )
            .await?;
        }

        Ok(())
    }

    async fn insert_vid(
        &mut self,
        common: VidCommonQueryData<Types>,
        share: Option<VidShare>,
    ) -> Result<(), Self::Error> {
        let mut tx = self.transaction().await?;
        let common_data = bincode::serialize(common.common()).map_err(|err| QueryError::Error {
            message: format!("failed to serialize VID common data: {err}"),
        })?;
        if let Some(share) = share {
            let share_data = bincode::serialize(&share).map_err(|err| QueryError::Error {
                message: format!("failed to serialize VID share: {err}"),
            })?;
            tx.upsert(
                "vid",
                ["height", "common", "share"],
                ["height"],
                [[
                    sql_param(&(common.height() as i64)),
                    sql_param(&common_data),
                    sql_param(&share_data),
                ]],
            )
            .await
        } else {
            // Don't touch the `share` column at all if we don't have a share to insert. It's
            // possible that this column already exists, and we are just upserting the common data,
            // in which case we don't want to overwrite the share with NULL.
            tx.upsert(
                "vid",
                ["height", "common"],
                ["height"],
                [[
                    sql_param(&(common.height() as i64)),
                    sql_param(&common_data),
                ]],
            )
            .await
        }
    }
}

#[async_trait]
impl<Types> NodeDataSource<Types> for SqlStorage
where
    Types: NodeType,
{
    async fn block_height(&self) -> QueryResult<usize> {
        let query = "SELECT max(height) FROM header";
        let row = self.query_one_static(query).await?;
        let height: Option<i64> = row.get(0);
        match height {
            Some(height) => {
                // The height of the block is the number of blocks below it, so the total number of
                // blocks is one more than the height of the highest block.
                Ok(height as usize + 1)
            }
            None => {
                // If there are no blocks yet, the height is 0.
                Ok(0)
            }
        }
    }

    async fn count_transactions(&self) -> QueryResult<usize> {
        let row = self
            .query_one_static("SELECT count(*) FROM transaction")
            .await?;
        let count: i64 = row.get(0);
        Ok(count as usize)
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        let row = self
            .query_one_static("SELECT sum(size) FROM payload")
            .await?;
        let sum: Option<i64> = row.get(0);
        Ok(sum.unwrap_or(0) as usize)
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let (where_clause, param) = header_where_clause(id.into());
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT v.share AS share FROM vid AS v
               JOIN header AS h ON v.height = h.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        let share_data: Option<Vec<u8>> =
            row.try_get("share").map_err(|err| QueryError::Error {
                message: format!("error extracting share data from query results: {err}"),
            })?;
        let share_data = share_data.context(MissingSnafu)?;
        bincode::deserialize(&share_data).map_err(|err| QueryError::Error {
            message: format!("malformed VID share: {err}"),
        })
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        // A leaf can only be missing if there is no row for it in the database (all its columns are
        // non-nullable). A block can be missing if its corresponding leaf is missing or if the
        // block's `data` field is `NULL`. We can find the number of missing leaves and blocks by
        // getting the number of fully missing leaf rows and the number of present but null-payload
        // block rows.
        //
        // Note that it should not be possible for a block's row to be missing (as opposed to
        // present but having a `NULL` payload) if the corresponding leaf is present. The schema
        // enforces this, since the payload table `REFERENCES` the corresponding leaf row. Thus,
        // missing block rows should be a subset of missing leaf rows and do not need to be counted
        // separately. This is very important: if we had to count the number of block rows that were
        // missing whose corresponding leaf row was present, this would require an expensive join
        // and table traversal.
        //
        // We can get the number of missing leaf rows very efficiently, by subtracting the total
        // number of leaf rows from the block height (since the block height by definition is the
        // height of the highest leaf we do have). We can also get the number of null payloads
        // directly using an `IS NULL` filter.
        //
        // For VID, common data can only be missing if the entire row is missing. Shares can be
        // missing in that case _or_ if the row is present but share data is NULL. Thus, we also
        // need to select the total number of VID rows and the number of present VID rows with a
        // NULL share.
        let row = self
            .query_one_static(
                "SELECT max_height, total_leaves, null_payloads, total_vid, null_vid, pruned_height FROM
                    (SELECT max(leaf.height) AS max_height, count(*) AS total_leaves FROM leaf),
                    (SELECT count(*) AS null_payloads FROM payload WHERE data IS NULL),
                    (SELECT count(*) AS total_vid FROM vid),
                    (SELECT count(*) AS null_vid FROM vid WHERE share IS NULL),
                    coalesce((SELECT last_height FROM pruned_height ORDER BY id DESC LIMIT 1)) as pruned_height
                ",
            )
            .await?;

        let block_height = match row.get::<_, Option<i64>>("max_height") {
            Some(height) => {
                // The height of the block is the number of blocks below it, so the total number of
                // blocks is one more than the height of the highest block.
                height as usize + 1
            }
            None => {
                // If there are no blocks yet, the height is 0.
                0
            }
        };
        let total_leaves = row.get::<_, i64>("total_leaves") as usize;
        let null_payloads = row.get::<_, i64>("null_payloads") as usize;
        let total_vid = row.get::<_, i64>("total_vid") as usize;
        let null_vid = row.get::<_, i64>("null_vid") as usize;
        let pruned_height = row
            .get::<_, Option<i64>>("pruned_height")
            .map(|h| h as usize);

        let missing_leaves = block_height.saturating_sub(total_leaves);
        let missing_blocks = missing_leaves + null_payloads;
        let missing_vid_common = block_height.saturating_sub(total_vid);
        let missing_vid_shares = missing_vid_common + null_vid;

        Ok(SyncStatus {
            missing_leaves,
            missing_blocks,
            missing_vid_common,
            missing_vid_shares,
            pruned_height,
        })
    }

    async fn get_header_window(
        &self,
        start: impl Into<WindowStart<Types>> + Send + Sync,
        end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        // Find the specific block that starts the requested window.
        let first_block = match start.into() {
            WindowStart::Time(t) => {
                // If the request is not to start from a specific block, but from a timestamp, we
                // use a different method to find the window, as detecting whether we have
                // sufficient data to answer the query is not as simple as just trying `load_header`
                // for a specific block ID.
                return self.time_window::<Types>(t, end).await;
            }
            WindowStart::Height(h) => h,
            WindowStart::Hash(h) => self.load_header::<Types>(h).await?.block_number(),
        };

        // Find all blocks starting from `first_block` with timestamps less than `end`. Block
        // timestamps are monotonically increasing, so this query is guaranteed to return a
        // contiguous range of blocks ordered by increasing height.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.height >= $1 AND h.timestamp < $2
              ORDER BY h.height"
        );
        let rows = self
            .query(&query, [&(first_block as i64), &(end as i64)])
            .await?;
        let window = rows
            .map(|row| {
                parse_header::<Types>(row.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        // Find the block just before the window.
        let prev = if first_block > 0 {
            Some(self.load_header::<Types>(first_block as usize - 1).await?)
        } else {
            None
        };

        // Find the block just after the window.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1
              ORDER BY h.height
              LIMIT 1"
        );
        let next = self
            .query_opt(&query, [&(end as i64)])
            .await?
            .map(parse_header::<Types>)
            .transpose()?;

        Ok(TimeWindowQueryData { window, prev, next })
    }
}

#[async_trait]
impl<Types: NodeType, State: MerklizedState<Types, ARITY>, const ARITY: usize>
    UpdateStateData<Types, State, ARITY> for SqlStorage
{
    async fn insert_merkle_nodes(
        &mut self,
        proof: MerkleProof<State::Entry, State::Key, State::T, ARITY>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> QueryResult<()> {
        let pos = proof.pos;
        let path = proof.proof;

        let name = State::state_type();
        let block_number = block_number as i64;

        let mut traversal_path = traversal_path.iter();
        let txn = self.transaction().await?;

        // All the nodes are collected here, They depend on the hash ids which are returned after
        // hashes are upserted in the db
        let mut nodes = Vec::new();

        for node in path.iter() {
            match node {
                MerkleNode::Empty => {
                    let ltree_path = LTree::from(traversal_path.clone());
                    let index = serde_json::to_value(pos.clone()).map_err(ParseError::Serde)?;

                    nodes.push((
                        Node {
                            pos: ltree_path,
                            index: Some(index),
                            ..Default::default()
                        },
                        [0_u8; 32].to_vec(),
                    ));
                }
                MerkleNode::ForgettenSubtree { .. } => {
                    return Err(QueryError::Error {
                        message: "Node in the Merkle path contains a forgetten subtree".to_owned(),
                    });
                }
                MerkleNode::Leaf { value, pos, elem } => {
                    let mut leaf_commit = Vec::new();
                    // Serialize the leaf node hash value into a vector
                    value
                        .serialize_compressed(&mut leaf_commit)
                        .map_err(ParseError::Serialize)?;

                    let ltree_path = LTree::from(traversal_path.clone());

                    let index = serde_json::to_value(pos).map_err(ParseError::Serde)?;
                    let entry = serde_json::to_value(elem).map_err(ParseError::Serde)?;

                    nodes.push((
                        Node {
                            pos: ltree_path,
                            index: Some(index),
                            entry: Some(entry),
                            ..Default::default()
                        },
                        leaf_commit,
                    ));
                }
                MerkleNode::Branch { value, children } => {
                    // Get hash
                    let mut branch_hash = Vec::new();
                    value
                        .serialize_compressed(&mut branch_hash)
                        .map_err(ParseError::Serialize)?;

                    // We only insert the non-empty children in the children field of the table
                    // BitVec is used to separate out Empty children positions
                    let mut children_bitvec = BitVec::new();
                    let mut children_values = Vec::new();
                    for child in children {
                        let child = child.as_ref();
                        match child {
                            MerkleNode::Empty => {
                                children_bitvec.push(false);
                            }
                            MerkleNode::Branch { value, .. }
                            | MerkleNode::Leaf { value, .. }
                            | MerkleNode::ForgettenSubtree { value } => {
                                let mut hash = Vec::new();
                                value
                                    .serialize_compressed(&mut hash)
                                    .map_err(ParseError::Serialize)?;

                                children_values.push(hash);
                                // Mark the entry as 1 in bitvec to indiciate a non-empty child
                                children_bitvec.push(true);
                            }
                        }
                    }

                    let (params, batch_hash_insert_stmt) =
                        HashTableRow::build_batch_insert(&children_values);

                    // Batch insert all the child hashes
                    let children_hash_ids = txn
                        .client
                        .query(&batch_hash_insert_stmt, &params[..])
                        .await
                        .map_err(|e| QueryError::Error {
                            message: format!("failed to batch insert children hashes {e}"),
                        })?
                        .iter()
                        .map(|r| r.get(0))
                        .collect();

                    // insert internal node
                    let ltree_path = LTree::from(traversal_path.clone());
                    nodes.push((
                        Node {
                            pos: ltree_path,
                            children: Some(children_hash_ids),
                            children_bitvec: Some(children_bitvec),
                            ..Default::default()
                        },
                        branch_hash,
                    ));
                }
            }

            // advance the traversal path for the internal nodes at each iteration
            // The final node would be the Root node where this iterator is exhausted
            traversal_path.next();
        }
        // We build a hashset to avoid duplicate entries
        let hashset: HashSet<Vec<u8>> = nodes.iter().map(|(_, h)| h.clone()).collect();
        let hashes = hashset.into_iter().collect::<Vec<Vec<u8>>>();
        // insert all the hashes into database
        // It returns all the ids inserted in the order they were inserted
        // We use the hash ids to insert all the nodes
        let (params, batch_hash_insert_stmt) = HashTableRow::build_batch_insert(&hashes);

        // Batch insert all the hashes
        let nodes_hash_ids: HashMap<Vec<u8>, i32> = txn
            .client
            .query(&batch_hash_insert_stmt, &params)
            .await
            .map_err(|e| QueryError::Error {
                message: format!("failed to batch insert children hashes {e}"),
            })?
            .iter()
            .map(|r| (r.get(1), r.get(0)))
            .collect();

        // Updates the node fields
        for (node, hash) in &mut nodes {
            node.created = block_number;
            node.hash_id = *nodes_hash_ids.get(&*hash).ok_or(QueryError::NotFound)?;
        }
        let nodes = nodes.into_iter().map(|(n, _)| n).collect::<Vec<_>>();
        let (params, batch_stmt) = Node::build_batch_insert(name, &nodes);

        // Batch insert all the child hashes
        let rows_inserted =
            txn.client
                .query(&batch_stmt, &params)
                .await
                .map_err(|e| QueryError::Error {
                    message: format!("failed to batch insert merkle nodes {e}"),
                })?;

        if rows_inserted.len() != path.len() {
            return Err(QueryError::Error {
                message: "failed to insert all merkle nodes".to_string(),
            });
        }

        Ok(())
    }
}

#[async_trait]
impl<Types, State, const ARITY: usize> MerklizedStateDataSource<Types, State, ARITY> for SqlStorage
where
    Types: NodeType,
    State: MerklizedState<Types, ARITY> + 'static,
{
    /// Retreives a Merkle path from the database
    async fn get_path(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>> {
        let state_type = State::state_type();
        let tree_height = State::tree_height();

        // Get the traversal path of the index
        let traversal_path = State::Key::to_traversal_path(&key, tree_height)
            .into_iter()
            .map(|x| x as i64)
            .collect::<Vec<_>>();
        let (created, merkle_commitment) = self.snapshot_info(snapshot).await?;

        // Get all the nodes in the path to the index.
        // Order by pos DESC is to return nodes from the leaf to the root
        let nodes = self
            .query(
                &format!(
                    "SELECT DISTINCT ON (pos) *
                    FROM {state_type}
                    WHERE pos @> $1 and created <= $2
                    ORDER BY pos DESC , created DESC;"
                ),
                [
                    sql_param(&LTree::from(traversal_path.iter())),
                    sql_param(&created),
                ],
            )
            .await?;

        let nodes: Vec<_> = nodes.map(|res| Node::try_from(res?)).try_collect().await?;

        // insert all the hash ids to a hashset which is used to query later
        // HashSet is used to avoid duplicates
        let mut hash_ids = HashSet::new();
        nodes.iter().for_each(|n| {
            hash_ids.insert(n.hash_id);
            if let Some(children) = &n.children {
                hash_ids.extend(children);
            }
        });

        // Find all the hash values and create a hashmap
        // Hashmap will be used to get the hash value of the nodes children and the node itself.
        let hashes_query = self
            .query(
                "SELECT * FROM hash WHERE id = ANY( $1)",
                [sql_param(&hash_ids.into_iter().collect::<Vec<i32>>())],
            )
            .await?;
        let hashes: HashMap<_, _> = hashes_query
            .map(|row| HashTableRow::try_from(row?).map(|h| (h.id, h.value)))
            .try_collect()
            .await?;

        let mut proof_path = VecDeque::with_capacity(State::tree_height());
        for Node {
            hash_id,
            children,
            children_bitvec,
            index,
            entry,
            ..
        } in nodes.iter()
        {
            {
                let value = hashes.get(hash_id).ok_or(QueryError::Error {
                    message: format!("node's value references non-existent hash {hash_id}"),
                })?;
                match (children, children_bitvec, index, entry) {
                    // If the row has children then its a branch
                    (Some(children), Some(children_bitvec), None, None) => {
                        let mut children = children.iter();

                        // Reconstruct the Children MerkleNodes from storage.
                        // Children bit_vec is used to create forgotten  or empty node
                        let child_nodes = children_bitvec
                            .iter()
                            .map(|bit| {
                                if bit {
                                    let hash_id = children.next().ok_or(QueryError::Error {
                                        message: "node has fewer children than set bits".into(),
                                    })?;
                                    let value = hashes.get(hash_id).ok_or(QueryError::Error {
                                        message: format!(
                                            "node's child references non-existent hash {hash_id}"
                                        ),
                                    })?;
                                    Ok(Arc::new(MerkleNode::ForgettenSubtree {
                                        value: State::T::deserialize_compressed(value.as_slice())
                                            .map_err(ParseError::Deserialize)?,
                                    }))
                                } else {
                                    Ok(Arc::new(MerkleNode::Empty))
                                }
                            })
                            .collect::<QueryResult<Vec<_>>>()?;
                        // Use the Children merkle nodes to reconstruct the branch node
                        proof_path.push_back(MerkleNode::Branch {
                            value: State::T::deserialize_compressed(value.as_slice())
                                .map_err(ParseError::Deserialize)?,
                            children: child_nodes,
                        });
                    }
                    // If it has an entry, it's a leaf
                    (None, None, Some(index), Some(entry)) => {
                        proof_path.push_back(MerkleNode::Leaf {
                            value: State::T::deserialize_compressed(value.as_slice())
                                .map_err(ParseError::Deserialize)?,
                            pos: serde_json::from_value(index.clone())
                                .map_err(ParseError::Serde)?,
                            elem: serde_json::from_value(entry.clone())
                                .map_err(ParseError::Serde)?,
                        });
                    }
                    // Otherwise, it's empty.
                    (None, None, Some(_), None) => {
                        proof_path.push_back(MerkleNode::Empty);
                    }
                    _ => {
                        return Err(QueryError::Error {
                            message: "Invalid type of merkle node found".to_string(),
                        });
                    }
                }
            }
        }

        // Reconstruct the merkle commitment from the path
        let init = if let Some(MerkleNode::Leaf { value, .. }) = proof_path.front() {
            *value
        } else {
            // If the path ends in a branch (or, as a special case, if the path and thus the entire
            // tree is empty), we are looking up an entry that is not present in the tree. We always
            // store all the nodes on all the paths to all the entries in the tree, so the only
            // nodes we could be missing are empty nodes from unseen entries. Thus, we can
            // reconstruct what the path should be by prepending empty nodes.
            while proof_path.len() <= State::tree_height() {
                proof_path.push_front(MerkleNode::Empty);
            }
            State::T::default()
        };
        let commitment_from_path = traversal_path
            .iter()
            .zip(proof_path.iter().skip(1))
            .try_fold(init, |val, (branch, node)| -> QueryResult<State::T> {
                match node {
                    MerkleNode::Branch { value: _, children } => {
                        let data = children
                            .iter()
                            .map(|node| match node.as_ref() {
                                MerkleNode::ForgettenSubtree { value } => Ok(*value),
                                MerkleNode::Empty => Ok(State::T::default()),
                                _ => Err(QueryError::Error {
                                    message: "Invalid child node".to_string(),
                                }),
                            })
                            .collect::<QueryResult<Vec<_>>>()?;

                        if data[*branch as usize] != val {
                            // This can only happen if data is missing: we have an old version of
                            // one of the nodes in the path, which is why it is not matching up with
                            // its parent.
                            tracing::warn!(
                                ?key,
                                parent = ?data[*branch as usize],
                                child = ?val,
                                branch = %*branch,
                                %created,
                                %merkle_commitment,
                                "missing data in merklized state; parent-child mismatch",
                            );
                            return Err(QueryError::Missing);
                        }

                        State::Digest::digest(&data).map_err(|err| QueryError::Error {
                            message: format!("failed to update digest: {err:#}"),
                        })
                    }
                    MerkleNode::Empty => Ok(init),
                    _ => Err(QueryError::Error {
                        message: "Invalid type of Node in the proof".to_string(),
                    }),
                }
            })?;

        if commitment_from_path != merkle_commitment.digest() {
            return Err(QueryError::Error {
                message:
                    format!("Commitment calcuated from merkle path ({commitment_from_path:?}) does not match the commitment in the header ({:?})", merkle_commitment.digest()),
            });
        }

        Ok(MerkleProof {
            pos: key,
            proof: proof_path.into(),
        })
    }

    async fn keys(&self, snapshot: Snapshot<Types, State, ARITY>) -> QueryResult<Vec<State::Key>> {
        let state_type = State::state_type();

        // Identify the snapshot.
        let created = self.snapshot_info(snapshot).await?.0;

        // Get all the nodes which correspond to an entry, ie have a non-NULL index field.
        let rows = self
            .query(
                &format!(
                    "SELECT DISTINCT ON (pos) index
                    FROM {state_type}
                    WHERE created <= $1 AND index IS NOT NULL
                    ORDER BY pos, created DESC;"
                ),
                [sql_param(&created)],
            )
            .await?;
        rows.map(|row| {
            let row = row.map_err(|err| QueryError::Error {
                message: format!("failed to fetch key: {err:#}"),
            })?;
            serde_json::from_value(row.get("index")).map_err(|err| QueryError::Error {
                message: format!("failed to deserialize key: {err:#}"),
            })
        })
        .try_collect()
        .await
    }

    async fn get_snapshot(&self, snapshot: Snapshot<Types, State, ARITY>) -> QueryResult<State> {
        // Identify the snapshot.
        let commit = self.snapshot_info(snapshot).await?.1;

        // Create a completely sparse snapshot from the header, as a starting point.
        let mut state = State::from_commitment(commit);
        // Remember each path into the tree.
        for key in self.keys(snapshot).await? {
            let path = self.get_path(snapshot, key.clone()).await?;
            state
                .insert_path(key.clone(), &path)
                .map_err(|err| QueryError::Error {
                    message: format!("invalid path for key {key:?}: {err:#}"),
                })?;
        }

        Ok(state)
    }
}

#[async_trait]
impl MerklizedStateHeightPersistence for SqlStorage {
    async fn set_last_state_height(&mut self, height: usize) -> QueryResult<()> {
        self.transaction()
            .await?
            .upsert(
                "last_merklized_state_height",
                ["id", "height"],
                ["id"],
                [[sql_param(&(1_i32)), sql_param(&(height as i64))]],
            )
            .await?;

        Ok(())
    }
    async fn get_last_state_height(&self) -> QueryResult<usize> {
        let row = self
            .query_opt_static("SELECT * from last_merklized_state_height")
            .await?;

        let height = row.map(|r| r.get::<_, i64>("height") as usize);

        Ok(height.unwrap_or(0))
    }
}

/// Low-level Merklized state operations.
impl SqlStorage {
    /// Get information identifying a [`Snapshot`].
    ///
    /// If the given snapshot is known to the database, this function returns
    /// * The block height at which the snapshot was created
    /// * A digest of the Merkle commitment to the snapshotted state
    async fn snapshot_info<Types, State, const ARITY: usize>(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
    ) -> QueryResult<(i64, State::Commit)>
    where
        Types: NodeType,
        State: MerklizedState<Types, ARITY>,
    {
        let header_state_commitment_field = State::header_state_commitment_field();

        let (created, commit) = match snapshot {
            Snapshot::Commit(commit) => {
                // Get the block height using the merkle commitment.
                let query = self
                    .query_one(
                        &format!(
                            "SELECT height FROM Header  where
                             data->>'{header_state_commitment_field}' = $1"
                        ),
                        &[&commit.to_string()],
                    )
                    .await?;

                (query.get(0), commit)
            }
            Snapshot::Index(created) => {
                let created = created as i64;
                let row = self
                    .query_one(
                        &format!(
                            "SELECT data->>'{header_state_commitment_field}' AS root_commmitment
                             FROM header
                             WHERE height = $1"
                        ),
                        [sql_param(&created)],
                    )
                    .await?;
                let commit: String = row.get(0);
                let commit = serde_json::from_value(commit.into()).map_err(ParseError::Serde)?;
                (created, commit)
            }
        };

        // Make sure the requested snapshot is up to date.
        let height = self.get_last_state_height().await?;
        if height < (created as usize) {
            return Err(QueryError::NotFound);
        }

        Ok((created, commit))
    }
}

/// Represents a Hash table row
struct HashTableRow {
    /// Hash id to be used by the state table to save space
    id: i32,
    /// hash value
    value: Vec<u8>,
}

impl HashTableRow {
    // TODO: create a generic upsert function with retries that returns the column
    fn build_batch_insert(hashes: &[Vec<u8>]) -> (Vec<&(dyn ToSql + Sync)>, String) {
        let len = hashes.len();
        let params: Vec<_> = hashes
            .iter()
            .flat_map(|c| [c as &(dyn ToSql + Sync)])
            .collect();
        let stmt = format!(
        "INSERT INTO hash(value) values {} ON CONFLICT (value) DO UPDATE SET value = EXCLUDED.value returning *",
        (1..len+1)
            .format_with(", ", |v, f| { f(&format_args!("(${v})")) }),
    );

        (params, stmt)
    }
}

// Parse a row to a HashTableRow
impl TryFrom<Row> for HashTableRow {
    type Error = QueryError;
    fn try_from(row: Row) -> QueryResult<Self> {
        Ok(Self {
            id: row.try_get(0).map_err(|e| QueryError::Error {
                message: format!("failed to get column id {e}"),
            })?,
            value: row.try_get(1).map_err(|e| QueryError::Error {
                message: format!("failed to get column value {e}"),
            })?,
        })
    }
}

// parsing errors
enum ParseError {
    Serde(serde_json::Error),
    Deserialize(SerializationError),
    Serialize(SerializationError),
}

impl From<ParseError> for QueryError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Serde(err) => Self::Error {
                message: format!("failed to parse value {err:?}"),
            },
            ParseError::Deserialize(err) => Self::Error {
                message: format!("failed to deserialize {err:?}"),
            },
            ParseError::Serialize(err) => Self::Error {
                message: format!("failed to serialize {err:?}"),
            },
        }
    }
}

// Represents a row in a state table
#[derive(Debug, Default, Clone)]
struct Node {
    pos: LTree,
    created: i64,
    hash_id: i32,
    children: Option<Vec<i32>>,
    children_bitvec: Option<BitVec>,
    index: Option<serde_json::Value>,
    entry: Option<serde_json::Value>,
}

impl Node {
    fn build_batch_insert<'a>(
        name: &'a str,
        nodes: &'a [Self],
    ) -> (Vec<&'a (dyn ToSql + Sync)>, String) {
        let params: Vec<&(dyn ToSql + Sync)> = nodes
            .iter()
            .flat_map(|n| {
                [
                    &n.pos as &(dyn ToSql + Sync),
                    &n.created,
                    &n.hash_id,
                    &n.children,
                    &n.children_bitvec,
                    &n.index,
                    &n.entry,
                ]
            })
            .collect();

        let stmt = format!(
                "INSERT INTO {name} (pos, created, hash_id, children, children_bitvec, index, entry) values {} ON CONFLICT (pos, created) 
                DO UPDATE SET hash_id = EXCLUDED.hash_id, children = EXCLUDED.children, children_bitvec = EXCLUDED.children_bitvec, 
                index = EXCLUDED.index, entry = EXCLUDED.entry RETURNING pos",
                (1..params.len()+1)
                .tuples()
                    .format_with(", ", |(pos, created, id, children, bitmap, i, e), f| 
                    { f(&format_args!("(${pos}, ${created}, ${id}, ${children}, ${bitmap}, ${i}, ${e})")) }),
            );

        (params, stmt)
    }
}

// Parse a Row to a Node
impl TryFrom<Row> for Node {
    type Error = QueryError;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            pos: row.try_get(0).map_err(|e| QueryError::Error {
                message: format!("failed to get column pos: {e}"),
            })?,
            created: row.try_get(1).map_err(|e| QueryError::Error {
                message: format!("failed to get column created: {e}"),
            })?,
            hash_id: row.try_get(2).map_err(|e| QueryError::Error {
                message: format!("failed to get column hash_id: {e}"),
            })?,
            children: row.try_get(3).map_err(|e| QueryError::Error {
                message: format!("failed to get column children: {e}"),
            })?,
            children_bitvec: row.try_get(4).map_err(|e| QueryError::Error {
                message: format!("failed to get column children bitmap: {e}"),
            })?,
            index: row.try_get(5).map_err(|e| QueryError::Error {
                message: format!("failed to get column index: {e}"),
            })?,
            entry: row.try_get(6).map_err(|e| QueryError::Error {
                message: format!("failed to get column entry: {e}"),
            })?,
        })
    }
}

impl SqlStorage {
    async fn time_window<Types: NodeType>(
        &self,
        start: u64,
        end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        // Find all blocks whose timestamps fall within the window [start, end). Block timestamps
        // are monotonically increasing, so this query is guaranteed to return a contiguous range of
        // blocks ordered by increasing height. Note that we order by height explicitly, rather than
        // ordering by timestamp (which might be more efficient, since it could reuse the timestamp
        // index that is used in the WHERE clause) because multiple blocks may have the same
        // timestamp, due to the 1-second timestamp resolution.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1 AND h.timestamp < $2
              ORDER BY h.height"
        );
        let rows = self.query(&query, [&(start as i64), &(end as i64)]).await?;
        let window: Vec<_> = rows
            .map(|row| {
                parse_header::<Types>(row.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        // Find the block just after the window.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1
              ORDER BY h.height
              LIMIT 1"
        );
        let next = self
            .query_opt(&query, [&(end as i64)])
            .await?
            .map(parse_header::<Types>)
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
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp < $1
              ORDER BY h.height DESC
              LIMIT 1"
        );
        let prev = self
            .query_opt(&query, [&(start as i64)])
            .await?
            .map(parse_header::<Types>)
            .transpose()?;

        Ok(TimeWindowQueryData { window, prev, next })
    }
}

#[async_trait]
impl<Types: NodeType> ExplorerStorage<Types> for SqlStorage
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
    Header<Types>: QueryableHeader<Types> + explorer::traits::ExplorerHeader<Types>,
    explorer::data_source::BalanceAmount<Types>: Into<explorer::monetary_value::MonetaryValue>,
{
    async fn get_block_summaries(
        &self,
        request: &explorer::data_source::GetBlockSummariesRequest<Types>,
    ) -> QueryResult<Vec<explorer::data_source::BlockSummary<Types>>> {
        let request = &request.0;

        let (query, params): (String, Vec<Box<dyn ToSql + Send + Sync>>) = match request.target {
            explorer::data_source::BlockIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        ORDER BY h.height DESC 
                        LIMIT $1"
                ),
                vec![Box::new(request.num_blocks.get() as i64)],
            ),
            explorer::data_source::BlockIdentifier::Height(height) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height <= $1
                        ORDER BY h.height DESC 
                        LIMIT $2"
                ),
                vec![
                    Box::new(height as i64),
                    Box::new(request.num_blocks.get() as i64),
                ],
            ),
            explorer::data_source::BlockIdentifier::Hash(hash) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.hash <= (SELECT h1.height FROM header AS h1 WHERE h1.hash = $1)
                        ORDER BY h.height DESC 
                        LIMIT $2"
                ),
                vec![
                    Box::new(hash.to_string()),
                    Box::new(request.num_blocks.get() as i64),
                ],
            ),
        };

        let result = self.query(&query, params).await.map(|row_stream| {
            row_stream.map(|row| -> Result<BlockSummary<Types>, QueryError> {
                let block = parse_block::<Types>(row?)?;
                Ok(BlockSummary::try_from(block)?)
            })
        });

        let results =
            result.map(|res| async move { res.try_collect::<Vec<BlockSummary<Types>>>().await });

        results?.await
    }

    async fn get_block_detail(
        &self,
        request: &explorer::data_source::BlockIdentifier<Types>,
    ) -> QueryResult<explorer::data_source::BlockDetail<Types>> {
        let (query, params): (String, Vec<Box<dyn ToSql + Sync + Send>>) = match request {
            explorer::data_source::BlockIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![],
            ),
            explorer::data_source::BlockIdentifier::Height(height) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height = $1
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![Box::new(*height as i64)],
            ),
            explorer::data_source::BlockIdentifier::Hash(hash) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.hash = $1
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![Box::new(hash.to_string())],
            ),
        };

        let query_result = self.query_one(&query, params).await?;
        let block = parse_block(query_result)?;

        Ok(
            explorer::data_source::BlockDetail::try_from(block).map_err(|e| QueryError::Error {
                message: e.to_string(),
            })?,
        )
    }

    async fn get_transaction_summaries(
        &self,
        request: &explorer::data_source::GetTransactionSummariesRequest<Types>,
    ) -> QueryResult<Vec<explorer::data_source::TransactionSummary<Types>>> {
        let range = &request.range;
        let target = &range.target;
        let filter = &request.filter;

        let (query, params, offset): (String, Vec<Box<dyn ToSql + Send + Sync>>, Option<usize>) =
            match (target, filter) {
                (_, explorer::data_source::TransactionSummaryFilter::RollUp(_)) => {
                    // TODO: implement roll-up based filtering
                    return Ok(vec![]);
                }
                (
                    explorer::data_source::TransactionIdentifier::Latest,
                    explorer::data_source::TransactionSummaryFilter::None,
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                           FROM header AS h
                           JOIN payload AS p ON h.height = p.height
                           WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $1
                            )
                           ORDER BY h.height DESC"
                    ),
                    vec![Box::new(range.num_transactions.get() as i64)],
                    Some(0),
                ),
                (
                    explorer::data_source::TransactionIdentifier::Latest,
                    explorer::data_source::TransactionSummaryFilter::Block(height),
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                           FROM header AS h
                           JOIN payload AS p ON h.height = p.height
                           WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE t1.block_height = $2
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $1
                            )
                           ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(range.num_transactions.get() as i64),
                        Box::new(*height as i64),
                    ],
                    Some(0),
                ),

                (
                    explorer::data_source::TransactionIdentifier::HeightAndOffset(height, offset),
                    explorer::data_source::TransactionSummaryFilter::None,
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE (t1.block_height, t1.index) <= (
                                        SELECT t2.block_height, t2.index
                                            FROM transaction AS t2
                                            WHERE t2.block_height = $1
                                            ORDER BY (t2.block_height, t2.index) DESC
                                            OFFSET $2
                                            LIMIT 1
                                    )
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $3
                            )
                            ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(*height as i64),
                        Box::new(*offset as i64),
                        Box::new(range.num_transactions.get() as i64),
                    ],
                    Some(*offset),
                ),
                (
                    explorer::data_source::TransactionIdentifier::HeightAndOffset(height, offset),
                    explorer::data_source::TransactionSummaryFilter::Block(block_height),
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE (t1.block_height, t1.index) <= (
                                        SELECT t2.block_height, t2.index
                                            FROM transaction AS t2
                                            WHERE t2.block_height = $1
                                            ORDER BY (t2.block_height, t2.index) DESC
                                            OFFSET $2
                                            LIMIT 1
                                    )
                                    AND t1.block_height = $4
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $3
                            )
                            ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(*height as i64),
                        Box::new(*offset as i64),
                        Box::new(range.num_transactions.get() as i64),
                        Box::new(*block_height as i64),
                    ],
                    Some(*offset),
                ),
                (
                    explorer::data_source::TransactionIdentifier::Hash(hash),
                    explorer::data_source::TransactionSummaryFilter::None,
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE (t1.block_height, t1.index) <= (
                                        SELECT t2.block_height, t2.index
                                            FROM transaction AS t2
                                            WHERE t2.hash = $1
                                            ORDER BY (t2.block_height, t2.index) DESC
                                            OFFSET $2
                                            LIMIT 1
                                    )
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $2
                            )
                            ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(hash.to_string()),
                        Box::new(range.num_transactions.get() as i64),
                    ],
                    // This should be some value... but we have no way of
                    // telling what that should be at this point.
                    None,
                ),
                (
                    explorer::data_source::TransactionIdentifier::Hash(hash),
                    explorer::data_source::TransactionSummaryFilter::Block(height),
                ) => (
                    format!(
                        "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height IN (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE (t1.block_height, t1.index) <= (
                                        SELECT t2.block_height, t2.index
                                            FROM transaction AS t2
                                            WHERE t2.hash = $1
                                            ORDER BY (t2.block_height, t2.index) DESC
                                            OFFSET $2
                                            LIMIT 1
                                    )
                                    AND t1.block_height = $3
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT $2
                            )
                            ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(hash.to_string()),
                        Box::new(range.num_transactions.get() as i64),
                        Box::new(*height as i64),
                    ],
                    // This should be some value... but we have no way of
                    // telling what that should be at this point.
                    None,
                ),
            };

        let result = self.query(&query, params).await.map(|row_stream| {
            row_stream.map(|row| -> Result<Vec<explorer::data_source::TransactionSummary<Types>>, QueryError> {
                let block = parse_block::<Types>(row?)?;
                let block_ref = &block;


                block
                    .enumerate()
                    .enumerate()
                    .map(move |(index, (_, txn))| {
                        Ok(explorer::data_source::TransactionSummary::try_from((block_ref, index, txn))?)
                    })
                    .try_collect::<explorer::data_source::TransactionSummary<Types>, Vec<explorer::data_source::TransactionSummary<Types>>, QueryError>()
            })
        });

        let non_flattened_results = result?
            .try_collect::<Vec<Vec<explorer::data_source::TransactionSummary<Types>>>>()
            .await?;

        let flattened_results = non_flattened_results
            .into_iter()
            .flat_map(|v| v.into_iter().rev())
            .skip(offset.unwrap_or(0))
            .skip_while(|txn| {
                if let explorer::data_source::TransactionIdentifier::Hash(hash) = target {
                    txn.hash != *hash
                } else {
                    false
                }
            })
            .take(range.num_transactions.get())
            .collect::<Vec<explorer::data_source::TransactionSummary<Types>>>();

        Ok(flattened_results)
    }

    async fn get_transaction_detail(
        &self,
        request: &explorer::data_source::TransactionIdentifier<Types>,
    ) -> QueryResult<explorer::data_source::TransactionDetailResponse<Types>> {
        let target = request;

        let (query, params): (String, Vec<Box<dyn ToSql + Send + Sync>>) = match target {
            explorer::data_source::TransactionIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT 1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![],
            ),
            explorer::data_source::TransactionIdentifier::HeightAndOffset(height, offset) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE t1.block_height = $1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    OFFSET $2
                                    LIMIT 1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![Box::new(*height as i64), Box::new(*offset as i64)],
            ),
            explorer::data_source::TransactionIdentifier::Hash(hash) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE t1.hash = $1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT 1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![Box::new(hash.to_string())],
            ),
        };

        let query_row = self.query_one(&query, params).await?;
        let block = parse_block::<Types>(query_row)?;

        let txns = block.enumerate().map(|(_, txn)| txn).collect::<Vec<_>>();

        let (offset, txn) = match target {
            explorer::data_source::TransactionIdentifier::Latest => {
                txns.into_iter().enumerate().last().unwrap()
            }
            explorer::data_source::TransactionIdentifier::HeightAndOffset(_, offset) => {
                txns.into_iter().enumerate().rev().nth(*offset).unwrap()
            }
            explorer::data_source::TransactionIdentifier::Hash(hash) => txns
                .into_iter()
                .enumerate()
                .find(|(_, txn)| txn.commit() == *hash)
                .unwrap(),
        };

        Ok(explorer::data_source::TransactionDetailResponse::try_from(
            (&block, offset, txn),
        )?)
    }

    async fn get_explorer_summary(
        &self,
    ) -> QueryResult<explorer::data_source::ExplorerSummary<Types>> {
        let histograms= async {
            let historgram_query_result = self.query(
                "SELECT
                    h.height AS height,
                    h.timestamp AS timestamp,
                    h.timestamp - lead(timestamp) OVER (ORDER BY h.height DESC) AS time,
                    p.size AS size,
                    (SELECT COUNT(*) AS transactions FROM transaction AS t WHERE t.block_height = h.height) as transactions
                FROM header AS h
                JOIN payload AS p ON
                    p.height = h.height
                ORDER BY h.height DESC
                LIMIT 50",
                Vec::<Box<dyn ToSql + Send + Sync>>::new(),
            ).await?;

            let histograms:Result<explorer::data_source::ExplorerHistograms, QueryError> = historgram_query_result
                .map(|row_stream| {
                    row_stream.map(|row| {
                        let height: i64 = row.try_get("height").map_err(|e| QueryError::Error {
                            message: format!("failed to get column height {e}"),
                        })?;
                        let timestamp: i64 =
                            row.try_get("timestamp").map_err(|e| QueryError::Error {
                                message: format!("failed to get column timestamp {e}"),
                            })?;
                        let time: Option<i64> = row.try_get("time").map_err(|e| QueryError::Error {
                            message: format!("failed to get column time {e}"),
                        })?;
                        let size: i32 = row.try_get("size").map_err(|e| QueryError::Error {
                            message: format!("failed to get column size {e}"),
                        })?;
                        let num_transactions: i64 =
                            row.try_get("transactions").map_err(|e| QueryError::Error {
                                message: format!("failed to get column transactions {e}"),
                            })?;

                        let height: u64 = height.try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert height to u64 {e}"),
                        })?;
                        let timestamp: u64 =
                            timestamp.try_into().map_err(|e| QueryError::Error {
                                message: format!("failed to convert timestamp to u64 {e}"),
                            })?;
                        let time: u64 = time.unwrap_or(0).try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert time to u64 {e}"),
                        })?;
                        let size: u64 = size.try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert size to u64 {e}"),
                        })?;
                        let num_transactions: u64 =
                            num_transactions.try_into().map_err(|e| QueryError::Error {
                                message: format!("failed to convert num_transactions to u64 {e}"),
                            })?;

                        Ok((height, timestamp, time, size, num_transactions))
                    })
                })
                .try_fold(
                    explorer::data_source::ExplorerHistograms {
                        block_time: Vec::with_capacity(50),
                        block_size: Vec::with_capacity(50),
                        block_transactions: Vec::with_capacity(50),
                        block_heights: Vec::with_capacity(50),
                    },
                    |mut histograms: explorer::data_source::ExplorerHistograms,
                     row: Result<(u64, u64, u64, u64, u64), QueryError>| async {
                        let (height, _timestamp, time, size, num_transactions) = row?;
                        histograms
                            .block_time
                            .push(time);
                        histograms.block_size.push(size);
                        histograms.block_transactions.push(num_transactions);
                        histograms.block_heights.push(height);
                        Ok(histograms)
                    },
                ).await;

            histograms
        }.await?;

        let genesis_overview = async {
            let row = self
                .query_one(
                    "SELECT
                    (SELECT COUNT(*) FROM header) AS blocks,
                    (SELECT COUNT(*) FROM transaction) AS transactions",
                    Vec::<Box<dyn ToSql + Send + Sync>>::new(),
                )
                .await?;

            let blocks: i64 = row.try_get("blocks").map_err(|e| QueryError::Error {
                message: format!("failed to get column blocks {e}"),
            })?;
            let transactions: i64 = row.try_get("transactions").map_err(|e| QueryError::Error {
                message: format!("failed to get column transactions {e}"),
            })?;

            let blocks: u64 = blocks.try_into().map_err(|e| QueryError::Error {
                message: format!("failed to convert blocks to u64 {e}"),
            })?;
            let transactions: u64 = transactions.try_into().map_err(|e| QueryError::Error {
                message: format!("failed to convert transactions to u64 {e}"),
            })?;

            Ok::<_, QueryError>(explorer::data_source::GenesisOverview {
                rollups: 0,
                transactions,
                blocks,
            })
        }
        .await?;

        let latest_block: explorer::data_source::BlockDetail<Types> = self
            .get_block_detail(&explorer::data_source::BlockIdentifier::Latest)
            .await?;
        let latest_blocks: Vec<BlockSummary<Types>> = self
            .get_block_summaries(&explorer::data_source::GetBlockSummariesRequest(
                explorer::data_source::BlockRange {
                    target: explorer::data_source::BlockIdentifier::Latest,
                    num_blocks: NonZeroUsize::new(10).unwrap(),
                },
            ))
            .await?;
        let latest_transactions: Vec<explorer::data_source::TransactionSummary<Types>> = self
            .get_transaction_summaries(&explorer::data_source::GetTransactionSummariesRequest {
                range: explorer::data_source::TransactionRange {
                    target: explorer::data_source::TransactionIdentifier::Latest,
                    num_transactions: NonZeroUsize::new(10).unwrap(),
                },
                filter: explorer::data_source::TransactionSummaryFilter::None,
            })
            .await?;

        Ok(explorer::data_source::ExplorerSummary {
            genesis_overview,
            latest_block,
            latest_transactions,
            latest_blocks,
            histograms,
        })
    }
}

/// An atomic SQL transaction.
//
// Note: we use a custom `Transaction` type instead of `tokio_postgres::Transaction` because with
// the latter, the lifecycle of the underlying SQL transaction is coupled to the lifecycle of the
// Rust object: a `BEGIN` statement is executed every time a `Transaction` is created and a `COMMIT`
// is executed whenever the `Transaction` is dropped. This is undesirable here because, logically,
// the underlying SQL transaction may persist between several calls into the `SqlDataSource`, and is
// only closed when `commit` is finally called. However, due to the lifetime of the reference within
// `Transaction`, we cannot actually store the `Transaction` object in the `SqlDataSource`, and so
// we create a new `Transaction` wrapper each time `SqlDataSource::transaction` is called. Thus, the
// lifecycle of the underlying logical transaction is not the same as the lifecycle of the Rust
// wrapper object.
//
// The lifetime parameter here is mostly for type safety for callers. Internally, we can change the
// lifetime parameter to `'static` very easily, using `Cow::into_owned` on the inner `client`. This
// is even necessary, to move the [`Transaction`] out of an [`RwLockWriteGuard`] when exposing it
// via the higher-level [`SqlDataSource`]. However, none of the _public_ APIs exposed by this crate
// allow a caller to detach the lifetime parameter here from the lifetime of the [`SqlStorage`] or
// [`SqlDataSource`] from which the [`Transaction`] is borrowed, providing enhanced type safety.
pub struct Transaction<'a> {
    client: Cow<'a, Arc<Client>>,
}

impl<'a> Transaction<'a> {
    /// Change the lifetime parameter of a [`Transaction`].
    ///
    /// This allows the caller to change the lifetime parameter of this [`Transaction`], by taking
    /// ownership of a clone of the referenced client. This function must be used with care (hence
    /// the restricted visibility). It is used in this crate to return a [`Transaction`] which is
    /// borrowed from an [`RwLockWriteGuard`] up the stack, by replacing the lifetime parameter of
    /// the write guard with the lifetime parameter of the [`SqlDataSource`] that owns the
    /// [`RwLock`].
    pub(crate) fn change_lifetime<'b>(self) -> Transaction<'b> {
        Transaction {
            client: Cow::Owned(self.client.into_owned()),
        }
    }
}

impl<'a> Transaction<'a> {
    /// Execute a statement against the underlying database.
    ///
    /// The results of the statement will be reflected immediately in future statements made within
    /// this transaction, but will not be reflected in the underlying database until the transaction
    /// is committed with [`commit`](VersionedDataSource::commit).
    pub async fn execute<T, P>(&mut self, statement: &T, params: P) -> QueryResult<u64>
    where
        T: ?Sized + ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        self.client
            .execute_raw(statement, params)
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })
    }

    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified.
    pub async fn execute_one<T, P>(&mut self, statement: &T, params: P) -> QueryResult<()>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let nrows = self.execute_many(statement, params).await?;
        if nrows > 1 {
            // If more than one row is affected, we don't return an error, because clearly
            // _something_ happened and modified the database. So we don't necessarily want the
            // caller to retry. But we do log an error, because it seems the query did something
            // different than the caller intended.
            tracing::error!(
                %statement,
                "statement modified more rows ({nrows}) than expected (1)"
            );
        }
        Ok(())
    }

    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_one_with_retries<T, P>(
        &mut self,
        statement: &T,
        params: P,
    ) -> QueryResult<()>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator + Clone,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        while let Err(err) = self.execute_one(statement, params.clone()).await {
            tracing::error!(
                %statement,
                "error in statement execution ({retries} tries remaining): {err}"
            );
            if retries == 0 {
                return Err(err);
            }
            retries -= 1;
            sleep(interval).await;
        }

        Ok(())
    }

    /// Execute a statement that is expected to modify at least one row.
    ///
    /// Returns an error if the database is not modified.
    pub async fn execute_many<T, P>(&mut self, statement: &T, params: P) -> QueryResult<u64>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let nrows = self.execute(statement, params).await?;
        if nrows == 0 {
            return Err(QueryError::Error {
                message: format!("statement failed: 0 rows affected. Statement: {statement}"),
            });
        }

        Ok(nrows)
    }

    /// Execute a statement that is expected to modify at least one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_many_with_retries<T, P>(
        &mut self,
        statement: &T,
        params: P,
    ) -> QueryResult<u64>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator + Clone,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        loop {
            match self.execute_many(statement, params.clone()).await {
                Ok(nrows) => return Ok(nrows),
                Err(err) => {
                    tracing::error!(
                        %statement,
                        "error in statement execution ({retries} tries remaining): {err}"
                    );
                    if retries == 0 {
                        return Err(err);
                    }
                    retries -= 1;
                    sleep(interval).await;
                }
            }
        }
    }

    pub async fn upsert<const N: usize, P>(
        &mut self,
        table: &str,
        columns: [&str; N],
        pk: impl IntoIterator<Item = &str>,
        rows: impl IntoIterator<Item = [P; N]>,
    ) -> QueryResult<()>
    where
        P: BorrowToSql + Clone,
    {
        let set_columns = columns
            .iter()
            .map(|col| format!("{col} = excluded.{col}"))
            .join(",");
        let columns = columns.into_iter().join(",");
        let pk = pk.into_iter().join(",");

        let mut values = vec![];
        let mut params = vec![];
        let mut num_rows = 0;
        for (row, entries) in rows.into_iter().enumerate() {
            let start = row * N;
            let end = (row + 1) * N;
            let row_params = (start..end).map(|i| format!("${}", i + 1)).join(",");

            values.push(format!("({row_params})"));
            params.extend(entries);
            num_rows += 1;
        }

        if num_rows == 0 {
            tracing::warn!("trying to upsert 0 rows, this has no effect");
            return Ok(());
        }
        tracing::debug!("upserting {num_rows} rows");

        let values = values.into_iter().join(",");
        let stmt = format!(
            "INSERT INTO {table} ({columns})
                  VALUES {values}
             ON CONFLICT ({pk}) DO UPDATE SET {set_columns}"
        );
        let rows_modified = self.execute_many_with_retries(&stmt, params).await?;
        if rows_modified != num_rows {
            tracing::error!(
                stmt,
                "unexpected number of rows modified: expected {num_rows} but got {rows_modified}"
            );
        }
        Ok(())
    }
}

/// Query the underlying SQL database.
///
/// The results will reflect the state after the statements thus far added to this transaction have
/// been applied, even though those effects have not been committed to the database yet.
#[async_trait]
impl<'a> Query for Transaction<'a> {
    async fn client(&self) -> Cow<Arc<Client>> {
        self.client.clone()
    }
}

#[async_trait]
pub trait Query {
    async fn client(&self) -> Cow<Arc<Client>>;

    // Query the underlying SQL database.
    async fn query<T, P>(
        &self,
        query: &T,
        params: P,
    ) -> QueryResult<BoxStream<'static, QueryResult<Row>>>
    where
        T: ?Sized + ToStatement + Sync,
        P: IntoIterator + Send,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        Ok(self
            .client()
            .await
            .query_raw(query, params)
            .await
            .map_err(postgres_err)?
            .map_err(postgres_err)
            .boxed())
    }

    /// Query the underlying SQL database with no parameters.
    async fn query_static<T>(&self, query: &T) -> QueryResult<BoxStream<'static, QueryResult<Row>>>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.query::<T, [i64; 0]>(query, []).await
    }

    /// Query the underlying SQL database, returning exactly one result or failing.
    async fn query_one<T, P>(&self, query: &T, params: P) -> QueryResult<Row>
    where
        T: ?Sized + ToStatement + Sync,
        P: IntoIterator + Send,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        self.query_opt(query, params).await?.context(NotFoundSnafu)
    }

    /// Query the underlying SQL database with no parameters, returning exactly one result or
    /// failing.
    async fn query_one_static<T>(&self, query: &T) -> QueryResult<Row>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.query_one::<T, [i64; 0]>(query, []).await
    }

    /// Query the underlying SQL database, returning zero or one results.
    async fn query_opt<T, P>(&self, query: &T, params: P) -> QueryResult<Option<Row>>
    where
        T: ?Sized + ToStatement + Sync,
        P: IntoIterator + Send,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        self.query(query, params).await?.try_next().await
    }

    /// Query the underlying SQL database with no parameters, returning zero or one results.
    async fn query_opt_static<T>(&self, query: &T) -> QueryResult<Option<Row>>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.query_opt::<T, [i64; 0]>(query, []).await
    }
}

fn postgres_err(err: tokio_postgres::Error) -> QueryError {
    QueryError::Error {
        message: err.to_string(),
    }
}

fn parse_leaf<Types>(row: Row) -> QueryResult<LeafQueryData<Types>>
where
    Types: NodeType,
{
    let leaf = row.try_get("leaf").map_err(|err| QueryError::Error {
        message: format!("error extracting leaf from query results: {err}"),
    })?;
    let leaf: Leaf<Types> = serde_json::from_value(leaf).map_err(|err| QueryError::Error {
        message: format!("malformed leaf: {err}"),
    })?;

    let qc = row.try_get("qc").map_err(|err| QueryError::Error {
        message: format!("error extracting QC from query results: {err}"),
    })?;
    let qc: QuorumCertificate<Types> =
        serde_json::from_value(qc).map_err(|err| QueryError::Error {
            message: format!("malformed QC: {err}"),
        })?;

    Ok(LeafQueryData { leaf, qc })
}

fn header_where_clause<Types: NodeType>(
    id: BlockId<Types>,
) -> (&'static str, Box<dyn ToSql + Send + Sync>) {
    match id {
        BlockId::Number(n) => ("h.height = $1", Box::new(n as i64)),
        BlockId::Hash(h) => ("h.hash = $1", Box::new(h.to_string())),
        BlockId::PayloadHash(h) => ("h.payload_hash = $1", Box::new(h.to_string())),
    }
}

const BLOCK_COLUMNS: &str =
    "h.hash AS hash, h.data AS header_data, p.size AS payload_size, p.data AS payload_data";

fn parse_block<Types>(row: Row) -> QueryResult<BlockQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    // First, check if we have the payload for this block yet.
    let size: Option<i32> = row
        .try_get("payload_size")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting payload size from query results: {err}"),
        })?;
    let payload_data: Option<Vec<u8>> =
        row.try_get("payload_data")
            .map_err(|err| QueryError::Error {
                message: format!("error extracting payload data from query results: {err}"),
            })?;
    let (size, payload_data) = size.zip(payload_data).context(MissingSnafu)?;
    let size = size as u64;

    // Reconstruct the full header.
    let header_data = row
        .try_get("header_data")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting header data from query results: {err}"),
        })?;
    let header: Header<Types> =
        serde_json::from_value(header_data).map_err(|err| QueryError::Error {
            message: format!("malformed header: {err}"),
        })?;

    // Reconstruct the full block payload.
    let payload = Payload::<Types>::from_bytes(&payload_data, header.metadata());

    // Reconstruct the query data by adding metadata.
    let hash: String = row.try_get("hash").map_err(|err| QueryError::Error {
        message: format!("error extracting block hash from query results: {err}"),
    })?;
    let hash = hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed block hash: {err}"),
    })?;

    Ok(BlockQueryData {
        num_transactions: payload.len(header.metadata()) as u64,
        header,
        payload,
        size,
        hash,
    })
}

const PAYLOAD_COLUMNS: &str = BLOCK_COLUMNS;

fn parse_payload<Types>(row: Row) -> QueryResult<PayloadQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    parse_block(row).map(PayloadQueryData::from)
}

const VID_COMMON_COLUMNS: &str = "h.height AS height, h.hash AS block_hash, h.payload_hash AS payload_hash, v.common AS common_data";

fn parse_vid_common<Types>(row: Row) -> QueryResult<VidCommonQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    let height = row
        .try_get::<_, i64>("height")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting height from query results: {err}"),
        })? as u64;
    let block_hash: String = row.try_get("block_hash").map_err(|err| QueryError::Error {
        message: format!("error extracting block_hash from query results: {err}"),
    })?;
    let block_hash = block_hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed block hash: {err}"),
    })?;
    let payload_hash: String = row
        .try_get("payload_hash")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting payload_hash from query results: {err}"),
        })?;
    let payload_hash = payload_hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed payload hash: {err}"),
    })?;
    let common_data: Vec<u8> = row
        .try_get("common_data")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting common_data from query results: {err}"),
        })?;
    let common = bincode::deserialize(&common_data).map_err(|err| QueryError::Error {
        message: format!("malformed VID common data: {err}"),
    })?;
    Ok(VidCommonQueryData {
        height,
        block_hash,
        payload_hash,
        common,
    })
}

const HEADER_COLUMNS: &str = "h.data AS data";

fn parse_header<Types>(row: Row) -> QueryResult<Header<Types>>
where
    Types: NodeType,
{
    // Reconstruct the full header.
    let data = row.try_get("data").map_err(|err| QueryError::Error {
        message: format!("error extracting header data from query results: {err}"),
    })?;
    serde_json::from_value(data).map_err(|err| QueryError::Error {
        message: format!("malformed header: {err}"),
    })
}

/// Convert range bounds to a SQL where clause constraining a given column.
///
/// Returns the where clause as a string and a list of query parameters. We assume that there are no
/// other parameters in the query; that is, parameters in the where clause will start from $1.
fn bounds_to_where_clause<R>(range: R, column: &str) -> (String, Vec<i64>)
where
    R: RangeBounds<usize>,
{
    let mut bounds = vec![];
    let mut params = vec![];

    match range.start_bound() {
        Bound::Included(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} >= ${}", params.len()));
        }
        Bound::Excluded(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} > ${}", params.len()));
        }
        Bound::Unbounded => {}
    }
    match range.end_bound() {
        Bound::Included(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} <= ${}", params.len()));
        }
        Bound::Excluded(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} < ${}", params.len()));
        }
        Bound::Unbounded => {}
    }

    let mut where_clause = bounds.join(" AND ");
    if !where_clause.is_empty() {
        where_clause = format!(" WHERE {where_clause}");
    }

    (where_clause, params)
}

/// Connect to a Postgres database with a TLS implementation.
///
/// Spawns a background task to run the connection. Returns a client and a handle to the spawned
/// task.
async fn connect<T>(
    pgcfg: postgres::Config,
    tcp: TcpStream,
    tls: T,
) -> anyhow::Result<(Client, BackgroundTask)>
where
    T: TlsConnect<TcpStream>,
    T::Stream: Send + 'static,
{
    let (client, connection) = pgcfg.connect_raw(tcp, tls).await?;
    Ok((
        client,
        BackgroundTask::spawn("postgres connection", connection),
    ))
}

fn sql_param<T: ToSql + Sync>(param: &T) -> &(dyn ToSql + Sync) {
    param
}

/// LTree SQL data type
///
/// The traversal path in a merkle tree is from the leaf to the root.
/// The LTREE path is created by reversing the traversal path.
/// Root node is represented as an empty LTree
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LTree(String);

impl<I, Iter: Iterator<Item = I> + DoubleEndedIterator> From<Iter> for LTree
where
    I: Display + Clone,
{
    fn from(iter: Iter) -> Self {
        Self(itertools::intersperse(iter.map(|x| x.to_string()).rev(), ".".to_string()).collect())
    }
}

impl ToSql for LTree {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        <String as ToSql>::to_sql(&self.0, ty, out)
    }

    fn accepts(ty: &postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for LTree {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <String as FromSql>::from_sql(ty, raw).map(LTree)
    }

    fn accepts(ty: &Type) -> bool {
        <String as FromSql>::accepts(ty)
    }
}

// tokio-postgres is written in terms of the tokio AsyncRead/AsyncWrite traits. However, these
// traits do not require any specifics of the tokio runtime. Thus we can implement them using the
// async_std TcpStream type, and have a stream which is compatible with tokio-postgres but will run
// on the async_std executor.
//
// To avoid orphan impls, we wrap this tream in a new type.
struct TcpStream(async_std::net::TcpStream);

impl TcpStream {
    async fn connect<A: ToSocketAddrs>(addrs: A) -> Result<Self, Error> {
        Ok(Self(async_std::net::TcpStream::connect(addrs).await?))
    }
}

impl tokio::io::AsyncRead for TcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // tokio uses this hyper-optimized `ReadBuf` construct, where there is a filled portion, an
        // unfilled portion where we append new data, and the unfilled portion of the buffer need
        // not even be initialized. However the async_std implementation we're delegating to just
        // expects a normal `&mut [u8]` buffer which is entirely unfilled. To simplify the
        // conversion, we will abandon the uninitialized buffer optimization and force
        // initialization of the entire buffer, resulting in a plain old `&mut [u8]` representing
        // the unfilled portion. But first, we need to grab the length of the filled region so we
        // can increment it after we read new data from async_std.
        let filled = buf.filled().len();

        // Initialize the buffer and get a slice of the unfilled region. This operation is free
        // after the first time it is called, so we don't need to worry about maintaining state
        // between subsequent calls to `poll_read`.
        let unfilled = buf.initialize_unfilled();

        // Read data into the unfilled portion of the buffer.
        match Pin::new(&mut self.0).poll_read(cx, unfilled) {
            Poll::Ready(Ok(bytes_read)) => {
                // After the read completes, the first `bytes_read` of `unfilled` have now been
                // filled. Increment the `filled` cursor within the `ReadBuf` to account for this.
                buf.set_filled(filled + bytes_read);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl tokio::io::AsyncWrite for TcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(any(test, feature = "testing"), not(target_os = "windows")))]
pub mod testing {
    use std::{
        env,
        process::{Command, Stdio},
        str,
        time::Duration,
    };

    use portpicker::pick_unused_port;
    use refinery::Migration;

    use super::Config;
    use crate::testing::sleep;

    #[derive(Debug)]
    pub struct TmpDb {
        host: String,
        port: u16,
        container_id: String,
    }
    impl TmpDb {
        pub async fn init() -> Self {
            let docker_hostname = env::var("DOCKER_HOSTNAME");
            // This picks an unused port on the current system.  If docker is
            // configured to run on a different host then this may not find a
            // "free" port on that system.
            // We *might* be able to get away with this as any remote docker
            // host should hopefully be pretty open with it's port space.
            let port = pick_unused_port().unwrap();
            let host = docker_hostname.unwrap_or("localhost".to_string());

            let output = Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-d")
                .args(["-p", &format!("{port}:5432")])
                .args(["-e", "POSTGRES_PASSWORD=password"])
                .arg("postgres")
                .output()
                .unwrap();
            let stdout = str::from_utf8(&output.stdout).unwrap();
            let stderr = str::from_utf8(&output.stderr).unwrap();
            if !output.status.success() {
                panic!("failed to start postgres docker: {stderr}");
            }

            // Create the TmpDb object immediately after starting the Docker container, so if
            // anything panics after this `drop` will be called and we will clean up.
            let container_id = stdout.trim().to_owned();
            tracing::info!("launched postgres docker {container_id}");
            let db = Self {
                host,
                port,
                container_id,
            };

            // Wait for the database to be ready.
            while !Command::new("psql")
                .args([
                    "-h",
                    &(db.host()),
                    "-p",
                    &(db.port().to_string()),
                    "-U",
                    "postgres",
                ])
                .env("PGPASSWORD", "password")
                // Null input so the command terminates as soon as it manages to connect.
                .stdin(Stdio::null())
                // Output from this command is not useful, it's just a prompt.
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap()
                .success()
            {
                tracing::warn!("database is not ready");
                sleep(Duration::from_secs(1)).await;
            }

            db
        }

        pub fn host(&self) -> String {
            self.host.clone()
        }

        pub fn port(&self) -> u16 {
            self.port
        }

        pub fn config(&self) -> Config {
            Config::default()
                .user("postgres")
                .password("password")
                .host(self.host())
                .port(self.port())
                .tls()
                .migrations(vec![Migration::unapplied(
                    "V11__create_test_merkle_tree_table.sql",
                    &TestMerkleTreeMigration::create("test_tree"),
                )
                .unwrap()])
        }
    }

    impl Drop for TmpDb {
        fn drop(&mut self) {
            let output = Command::new("docker")
                .args(["stop", self.container_id.as_str()])
                .output()
                .unwrap();
            if !output.status.success() {
                tracing::error!(
                    "error killing postgres docker {}: {}",
                    self.container_id,
                    str::from_utf8(&output.stderr).unwrap()
                );
            }
        }
    }

    pub struct TestMerkleTreeMigration;

    impl TestMerkleTreeMigration {
        fn create(name: &str) -> String {
            format!(
                "CREATE TABLE IF NOT EXISTS hash
            (
                id SERIAL PRIMARY KEY,
                value BYTEA  NOT NULL UNIQUE
            );
        
            CREATE TABLE {name}
            (
                pos LTREE NOT NULL, 
                created BIGINT NOT NULL,
                hash_id INT NOT NULL REFERENCES hash (id),
                children INT[],
                children_bitvec BIT(8),
                index JSONB,
                entry JSONB 
            );
            ALTER TABLE {name} ADD CONSTRAINT {name}_pk PRIMARY KEY (pos, created);
            CREATE INDEX {name}_path ON {name} USING GIST (pos);"
            )
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {

    use hotshot_example_types::state_types::TestInstanceState;
    use jf_primitives::merkle_tree::{
        universal_merkle_tree::UniversalMerkleTree, LookupResult, UniversalMerkleTreeScheme,
    };
    use rand::{seq::IteratorRandom, RngCore};

    use super::{testing::TmpDb, *};

    use crate::testing::mocks::MockMerkleTree;
    use crate::testing::{mocks::MockTypes, setup_test};

    #[async_std::test]
    async fn test_migrations() {
        setup_test();

        let db = TmpDb::init().await;
        let port = db.port();
        let host = &db.host();

        let connect = |migrations: bool, custom_migrations| async move {
            let mut cfg = Config::default()
                .user("postgres")
                .password("password")
                .host(host)
                .port(port)
                .migrations(custom_migrations);
            if !migrations {
                cfg = cfg.no_migrations();
            }
            let client = SqlStorage::connect(cfg).await?;
            Ok::<_, Error>(client)
        };

        // Connecting with migrations disabled should fail if the database is not already up to date
        // (since we've just created a fresh database, it isn't).
        let err = connect(false, vec![]).await.unwrap_err();
        tracing::info!("connecting without running migrations failed as expected: {err}");

        // Now connect and run migrations to bring the database up to date.
        connect(true, vec![]).await.unwrap();
        // Now connecting without migrations should work.
        connect(false, vec![]).await.unwrap();

        // Connect with some custom migrations, to advance the schema even further. Pass in the
        // custom migrations out of order; they should still execute in order of version number.
        // The SQL commands used here will fail if not run in order.
        let migrations = vec![
            Migration::unapplied(
                "V13__create_test_table.sql",
                "ALTER TABLE test ADD COLUMN data INTEGER;",
            )
            .unwrap(),
            Migration::unapplied("V12__create_test_table.sql", "CREATE TABLE test ();").unwrap(),
        ];
        connect(true, migrations.clone()).await.unwrap();

        // Connect using the default schema (no custom migrations) and not running migrations. This
        // should fail because the database is _ahead_ of the client in terms of schema.
        let err = connect(false, vec![]).await.unwrap_err();
        tracing::info!("connecting without running migrations failed as expected: {err}");

        // Connecting with the customized schema should work even without running migrations.
        connect(true, migrations).await.unwrap();
    }

    #[test]
    fn test_config_from_str() {
        let cfg = Config::from_str("postgresql://user:password@host:8080").unwrap();
        assert_eq!(cfg.pgcfg.get_user(), Some("user"));
        assert_eq!(cfg.pgcfg.get_password(), Some("password".as_bytes()));
        assert_eq!(cfg.host, "host");
        assert_eq!(cfg.port, 8080);
    }

    #[test]
    fn test_config_from_pgcfg() {
        let mut pgcfg = postgres::Config::default();
        pgcfg.dbname("db");
        let cfg = Config::from(pgcfg.clone());
        assert_eq!(cfg.pgcfg, pgcfg);
        // Default values.
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 5432);
    }

    #[async_std::test]
    async fn test_target_period_pruning() {
        setup_test();

        let db = TmpDb::init().await;
        let port = db.port();
        let host = &db.host();

        let cfg = Config::default()
            .user("postgres")
            .password("password")
            .host(host)
            .port(port);

        let mut storage = SqlStorage::connect(cfg).await.unwrap();
        let mut leaf = LeafQueryData::<MockTypes>::genesis(&TestInstanceState {});
        // insert some mock data
        for i in 0..20 {
            leaf.leaf.get_block_header_mut().block_number = i;
            leaf.leaf.get_block_header_mut().timestamp = Utc::now().timestamp() as u64;
            storage.insert_leaf(leaf.clone()).await.unwrap();
            storage.commit().await.unwrap();
        }

        let height_before_pruning = storage.get_minimum_height().await.unwrap().unwrap();

        // Set pruner config to default which has minimum retention set to 1 day
        storage.set_pruning_config(PrunerCfg::new());
        // No data will be pruned
        let pruned_height = storage.prune().await.unwrap();
        // Pruned height should be none
        assert!(pruned_height.is_none());

        let height_after_pruning = storage.get_minimum_height().await.unwrap().unwrap();

        assert_eq!(
            height_after_pruning, height_before_pruning,
            "some data has been pruned"
        );

        // Set pruner config to target retention set to 1s
        storage.set_pruning_config(PrunerCfg::new().with_target_retention(Duration::from_secs(1)));
        sleep(Duration::from_secs(2)).await;
        let usage_before_pruning = storage.get_disk_usage().await.unwrap();
        // All of the data is now older than 1s.
        // This would prune all the data as the target retention is set to 1s
        let pruned_height = storage.prune().await.unwrap();

        // Pruned height should be some
        assert!(pruned_height.is_some());
        let usage_after_pruning = storage.get_disk_usage().await.unwrap();
        // All the tables should be empty
        // counting rows in header table
        let header_rows = storage
            .query_one_static("select count(*) as count from header")
            .await
            .unwrap()
            .get::<_, i64>("count");
        // the table should be empty
        assert_eq!(header_rows, 0);

        // counting rows in leaf table.
        // Deleting rows from header table would delete rows in all the tables
        // as each of table implement "ON DELETE CASCADE" fk constraint with the header table.
        let leaf_rows = storage
            .query_one_static("select count(*) as count from leaf")
            .await
            .unwrap()
            .get::<_, i64>("count");
        // the table should be empty
        assert_eq!(leaf_rows, 0);

        assert!(
            usage_before_pruning > usage_after_pruning,
            " disk usage should decrease after pruning"
        )
    }

    #[async_std::test]
    async fn test_minimum_retention_pruning() {
        setup_test();

        let db = TmpDb::init().await;

        let mut storage = SqlStorage::connect(db.config()).await.unwrap();
        let mut leaf = LeafQueryData::<MockTypes>::genesis(&TestInstanceState {});
        // insert some mock data
        for i in 0..20 {
            leaf.leaf.get_block_header_mut().block_number = i;
            leaf.leaf.get_block_header_mut().timestamp = Utc::now().timestamp() as u64;
            storage.insert_leaf(leaf.clone()).await.unwrap();
            storage.commit().await.unwrap();
        }

        let height_before_pruning = storage.get_minimum_height().await.unwrap().unwrap();
        let cfg = PrunerCfg::new();
        // Set pruning_threshold to 1
        // SQL storage size is more than 1000 bytes even without any data indexed
        // This would mean that the threshold would always be greater than the disk usage
        // However, minimum retention is set to 24 hours by default so the data would not be pruned
        storage.set_pruning_config(cfg.clone().with_pruning_threshold(1));
        println!("{:?}", storage.get_pruning_config().unwrap());
        // Pruning would not delete any data
        // All the data is younger than minimum retention period even though the usage > threshold
        let pruned_height = storage.prune().await.unwrap();

        // Pruned height should be none
        assert!(pruned_height.is_none());

        let height_after_pruning = storage.get_minimum_height().await.unwrap().unwrap();

        assert_eq!(
            height_after_pruning, height_before_pruning,
            "some data has been pruned"
        );

        // Change minimum retention to 1s
        storage.set_pruning_config(
            cfg.with_minimum_retention(Duration::from_secs(1))
                .with_pruning_threshold(1),
        );
        // sleep for 2s to make sure the data is older than minimum retention
        sleep(Duration::from_secs(2)).await;
        // This would prune all the data
        let pruned_height = storage.prune().await.unwrap();

        // Pruned height should be some
        assert!(pruned_height.is_some());
        // All the tables should be empty
        // counting rows in header table
        let header_rows = storage
            .query_one_static("select count(*) as count from header")
            .await
            .unwrap()
            .get::<_, i64>("count");
        // the table should be empty
        assert_eq!(header_rows, 0);
    }

    #[async_std::test]
    async fn test_pruned_height_storage() {
        setup_test();

        let db = TmpDb::init().await;
        let port = db.port();
        let host = &db.host();

        let cfg = Config::default()
            .user("postgres")
            .password("password")
            .host(host)
            .port(port);

        let mut storage = SqlStorage::connect(cfg).await.unwrap();
        assert!(storage.load_pruned_height().await.unwrap().is_none());
        storage.save_pruned_height(10).await.unwrap();
        storage.save_pruned_height(20).await.unwrap();
        storage.save_pruned_height(30).await.unwrap();
        assert_eq!(storage.load_pruned_height().await.unwrap(), Some(30));
    }

    use crate::data_source::VersionedDataSource;

    use jf_primitives::merkle_tree::MerkleTreeScheme;

    #[async_std::test]
    async fn test_merklized_state_storage() {
        // In this test we insert some entries into the tree and update the database
        // Each entry's merkle path is compared with the path from the tree
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree: UniversalMerkleTree<_, _, _, 8, _> =
            MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;

        // insert some entries into the tree and the header table
        // Header table is used the get_path query to check if the header exists for the block height.
        for i in 0..27 {
            test_tree.update(i, i).unwrap();

            // data field of the header
            let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
            storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&format!("randomHash{i}")),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
            // proof for the index from the tree
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            // traversal path for the index.
            let traversal_path =
                <usize as ToTraversalPath<8>>::to_traversal_path(&i, test_tree.height());

            <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
                &mut storage,
                proof.clone(),
                traversal_path.clone(),
                block_height as u64,
            )
            .await
            .expect("failed to insert nodes");
        }
        // update saved state height
        storage.set_last_state_height(block_height).await.unwrap();

        //Get the path and check if it matches the lookup
        for i in 0..27 {
            // Query the path for the index
            let merkle_path =
                <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                    &storage,
                    Snapshot::Index(block_height as u64),
                    i,
                )
                .await
                .unwrap();

            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();

            tracing::info!("merkle path {:?}", merkle_path);

            // merkle path from the storage should match the path from test tree
            assert_eq!(merkle_path, proof.clone(), "merkle paths mismatch");
        }

        // Get the proof of index 0 with bh = 1
        let (_, proof_bh_1) = test_tree.lookup(0).expect_ok().unwrap();
        // Inserting Index 0 again with created (bh) = 2
        // Our database should then have 2 versions of this leaf node
        // Update the node so that proof is also updated
        test_tree.update(0, 99).unwrap();
        // Also update the merkle commitment in the header

        // data field of the header
        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
        storage
            .query_opt(
                "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                [
                    sql_param(&(2_i64)),
                    sql_param(&"randomstring"),
                    sql_param(&test_data),
                ],
            )
            .await
            .unwrap();
        let (_, proof_bh_2) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // Update storage to insert a new version of this code

        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof_bh_2.clone(),
            traversal_path.clone(),
            2,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        storage.set_last_state_height(2).await.unwrap();

        // Find all the nodes of Index 0 in table
        let ltree_path = LTree::from(traversal_path.iter());
        let rows = storage
            .query(
                "SELECT * from test_tree where pos = $1 ORDER BY created",
                [sql_param(&ltree_path)],
            )
            .await
            .unwrap();

        let nodes: Vec<_> = rows
            .map(|res| Node::try_from(res.unwrap()))
            .try_collect()
            .await
            .unwrap();
        // There should be only 2 versions of this node
        assert!(nodes.len() == 2, "incorrect number of nodes");
        assert_eq!(nodes[0].created, 1, "wrong block height");
        assert_eq!(nodes[1].created, 2, "wrong block height");

        // Now we can have two snapshots for Index 0
        // One with created = 1 and other with 2
        // Query snapshot with created = 2

        let path_with_bh_2 =
            <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                &storage,
                Snapshot::Index(2),
                0,
            )
            .await
            .unwrap();

        assert_eq!(path_with_bh_2, proof_bh_2);
        let path_with_bh_1 =
            <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                &storage,
                Snapshot::Index(1),
                0,
            )
            .await
            .unwrap();
        assert_eq!(path_with_bh_1, proof_bh_1);
    }

    #[async_std::test]
    async fn test_merklized_state_non_membership_proof() {
        // This test updates the Merkle tree with a new entry and inserts the corresponding Merkle nodes into the database with created = 1.
        // A Merkle node is then deleted from the tree.
        // The database is then updated to reflect the deletion of the entry with a created (block height) of 2
        // As the leaf node becomes a non-member, we do a universal lookup to obtain its non-membership proof path.
        // It is expected that the path retrieved from the tree matches the path obtained from the database.
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree
        test_tree.update(0, 0).unwrap();
        let commitment = test_tree.commitment();

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(commitment).unwrap()});
        // insert the header with merkle commitment
        storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        // proof for the index from the tree
        let (_, proof_before_remove) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // insert merkle nodes
        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof_before_remove.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        storage.set_last_state_height(block_height).await.unwrap();
        // the path from the db and and tree should match
        let merkle_path = <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
            &storage,
            Snapshot::Index(block_height as u64),
            0,
        )
        .await
        .unwrap();

        // merkle path from the storage should match the path from test tree
        assert_eq!(
            merkle_path,
            proof_before_remove.clone(),
            "merkle paths mismatch"
        );

        //Deleting the index 0
        test_tree.remove(0).expect("failed to delete index 0 ");

        // Update the database with the proof
        // Created = 2 in this case
        let proof_after_remove = test_tree.universal_lookup(0).expect_not_found().unwrap();

        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof_after_remove.clone(),
            traversal_path.clone(),
            2_u64,
        )
        .await
        .expect("failed to insert nodes");
        // Insert the new header
        storage
        .query_opt(
            "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
            [
                sql_param(&2_i64),
                sql_param(&"randomString2"),
                sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
            ],
        )
        .await
        .unwrap();
        // update saved state height
        storage.set_last_state_height(2).await.unwrap();
        // Get non membership proof
        let non_membership_path =
            <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                &storage,
                Snapshot::Index(2_u64),
                0,
            )
            .await
            .unwrap();
        // Assert that the paths from the db and the tree are equal
        assert_eq!(
            non_membership_path, proof_after_remove,
            "merkle paths dont match"
        );

        // Query the membership proof i.e proof with created = 1
        // This proof should be equal to the proof before deletion
        // Assert that the paths from the db and the tree are equal

        let proof_bh_1 = <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
            &storage,
            Snapshot::Index(1_u64),
            0,
        )
        .await
        .unwrap();
        assert_eq!(proof_bh_1, proof_before_remove, "merkle paths dont match");
    }

    #[async_std::test]
    async fn test_merklized_state_non_membership_proof_unseen_entry() {
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());

        // For each case (where the root is empty, a leaf, and a branch) test getting a
        // non-membership proof for an entry node the database has never seen.
        for i in 0..=2 {
            tracing::info!(i, ?test_tree, "testing non-membership proof");

            // Insert a dummy header
            storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3)",
                    [
                        sql_param(&(i as i64)),
                        sql_param(&format!("hash{i}")),
                        sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
                    ],
                )
                .await
                .unwrap();
            // update saved state height
            storage.set_last_state_height(i).await.unwrap();

            // get a non-membership proof for a never-before-seen node.
            let proof = MerklizedStateDataSource::get_path(
                &storage,
                Snapshot::<MockTypes, MockMerkleTree, 8>::Index(i as u64),
                100,
            )
            .await
            .unwrap();
            assert_eq!(proof.elem(), None);
            assert!(test_tree.non_membership_verify(100, proof).unwrap());

            // insert an additional node into the tree.
            test_tree.update(i, i).unwrap();
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            let traversal_path = ToTraversalPath::<8>::to_traversal_path(&i, test_tree.height());
            UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                &mut storage,
                proof,
                traversal_path,
                (i + 1) as u64,
            )
            .await
            .expect("failed to insert nodes");
        }
    }

    #[async_std::test]
    async fn test_merklized_storage_with_commit() {
        // This test insert a merkle path into the database and queries the path using the merkle commitment
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree
        test_tree.update(0, 0).unwrap();
        let commitment = test_tree.commitment();

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(commitment).unwrap()});
        // insert the header with merkle commitment
        storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        // proof for the index from the tree
        let (_, proof) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // insert merkle nodes
        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        storage.set_last_state_height(block_height).await.unwrap();

        let merkle_proof =
            <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                &storage,
                Snapshot::Commit(commitment),
                0,
            )
            .await
            .unwrap();

        let (_, proof) = test_tree.lookup(0).expect_ok().unwrap();

        assert_eq!(merkle_proof, proof.clone(), "merkle paths mismatch");
    }
    #[async_std::test]
    async fn test_merklized_state_missing_state() {
        // This test checks that header commitment matches the root hash.
        // For this, the header merkle root commitment field is not updated, which should result in an error
        // The full merkle path verification is also done by recomputing the root hash
        // An index and its corresponding merkle nodes with created (bh) = 1 are inserted.
        // The entry of the index is updated, and the updated nodes are inserted with created (bh) = 2.
        // A node which is in the traversal path with bh = 2 is deleted, so the get_path should return an error as an older version of one of the nodes is used.
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree

        for i in 0..27 {
            test_tree.update(i, i).unwrap();
            let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
            // insert the header with merkle commitment
            storage
                    .query_opt(
                        "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                        [
                            sql_param(&(block_height as i64)),
                            sql_param(&format!("randomString{i}")),
                            sql_param(&test_data),
                        ],
                    )
                    .await
                    .unwrap();
            // proof for the index from the tree
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            // traversal path for the index.
            let traversal_path =
                <usize as ToTraversalPath<8>>::to_traversal_path(&i, test_tree.height());
            // insert merkle nodes
            <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
                &mut storage,
                proof.clone(),
                traversal_path.clone(),
                block_height as u64,
            )
            .await
            .expect("failed to insert nodes");
            // update saved state height
            storage.set_last_state_height(block_height).await.unwrap();
        }

        test_tree.update(1, 100).unwrap();
        //insert updated merkle path without updating the header
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&1, test_tree.height());
        let (_, proof) = test_tree.lookup(1).expect_ok().unwrap();

        // insert merkle nodes
        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");

        let merkle_path = <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
            &storage,
            Snapshot::Index(block_height as u64),
            1,
        )
        .await;
        assert!(merkle_path.is_err());

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
        // insert the header with merkle commitment
        storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomStringgg"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        // Querying the path again
        let merkle_proof =
            <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
                &storage,
                Snapshot::Index(block_height as u64),
                1,
            )
            .await
            .unwrap();
        assert_eq!(merkle_proof, proof, "path dont match");

        // Update the tree again for index 0 with created (bh) = 2
        // Delete one of the node in the traversal path
        test_tree.update(1, 200).unwrap();

        let (_, proof) = test_tree.lookup(1).expect_ok().unwrap();
        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});

        // insert the header with merkle commitment
        storage
                .query_opt(
                    "INSERT INTO HEADER VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(2_i64)),
                        sql_param(&"randomHashString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        <SqlStorage as UpdateStateData<_, MockMerkleTree, 8>>::insert_merkle_nodes(
            &mut storage,
            proof.clone(),
            traversal_path.clone(),
            2_u64,
        )
        .await
        .expect("failed to insert nodes");

        // Deleting one internal node
        let rows = storage
            .client
            .execute_raw(
                &format!(
                    "DELETE FROM {} WHERE created = 2 and pos = $1",
                    MockMerkleTree::state_type()
                ),
                [sql_param(&(LTree::from(traversal_path[1..].iter())))],
            )
            .await
            .expect("failed to delete internal node");
        assert_eq!(rows, 1);

        let merkle_path = <SqlStorage as MerklizedStateDataSource<_, MockMerkleTree, 8>>::get_path(
            &storage,
            Snapshot::Index(2_u64),
            1,
        )
        .await;

        assert!(merkle_path.is_err());
    }

    #[async_std::test]
    async fn test_merklized_state_snapshot() {
        setup_test();

        let db = TmpDb::init().await;
        let mut storage = SqlStorage::connect(db.config()).await.unwrap();

        // Define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());

        // We will sample random keys as u32. This is a value that is not a valid u32 and thus is a
        // key we will never insert into the tree.
        const RESERVED_KEY: usize = (u32::MAX as usize) + 1;

        // Randomly insert and delete some entries. For each entry we insert, we also keep track of
        // whether the entry should be in the tree using a HashMap.
        #[tracing::instrument(skip(tree, expected))]
        fn randomize(tree: &mut MockMerkleTree, expected: &mut HashMap<usize, Option<usize>>) {
            let mut rng = rand::thread_rng();
            tracing::info!("randomizing tree");

            for _ in 0..50 {
                // We flip a coin to decide whether to insert or delete, unless the tree is empty,
                // in which case we can only insert.
                if !expected.values().any(|v| v.is_some()) || rng.next_u32() % 2 == 0 {
                    // Insert.
                    let key = rng.next_u32() as usize;
                    let val = rng.next_u32() as usize;
                    tracing::info!(key, val, "inserting");

                    tree.update(key, val).unwrap();
                    expected.insert(key, Some(val));
                } else {
                    // Delete.
                    let key = expected
                        .iter()
                        .filter_map(|(k, v)| if v.is_some() { Some(k) } else { None })
                        .choose(&mut rng)
                        .unwrap();
                    tracing::info!(key, "deleting");

                    tree.remove(key).unwrap();
                    expected.insert(*key, None);
                }
            }
        }

        // Commit the tree to storage.
        #[tracing::instrument(skip(storage, tree, expected))]
        async fn store(
            storage: &mut SqlStorage,
            tree: &MockMerkleTree,
            expected: &HashMap<usize, Option<usize>>,
            block_height: u64,
        ) {
            tracing::info!("persisting tree");

            for key in expected.keys() {
                let proof = match tree.universal_lookup(key) {
                    LookupResult::Ok(_, proof) => proof,
                    LookupResult::NotFound(proof) => proof,
                    LookupResult::NotInMemory => panic!("failed to find key {key}"),
                };
                let traversal_path = ToTraversalPath::<8>::to_traversal_path(key, tree.height());
                UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                    storage,
                    proof,
                    traversal_path,
                    block_height,
                )
                .await
                .unwrap();
            }
            // insert the header with merkle commitment
            storage
            .query_opt(
                "INSERT INTO HEADER VALUES ($1, $2, 'hash', 0, $3)",
                [
                    sql_param(&(block_height as i64)),
                    sql_param(&format!("hash{block_height}")),
                    sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(tree.commitment()).unwrap()})),
                ],
            )
            .await
            .unwrap();
            storage
                .set_last_state_height(block_height as usize)
                .await
                .unwrap();
            storage.commit().await.unwrap();
        }

        #[tracing::instrument(skip(storage, tree, expected))]
        async fn validate(
            storage: &SqlStorage,
            tree: &MockMerkleTree,
            expected: &HashMap<usize, Option<usize>>,
            block_height: u64,
        ) {
            tracing::info!("validating snapshot");

            // Check that we can get a correct path for each key that we touched.
            let snapshot = Snapshot::<_, MockMerkleTree, 8>::Index(block_height);
            let loaded = storage.get_snapshot(snapshot).await.unwrap();
            for (key, val) in expected {
                let proof = match loaded.universal_lookup(key) {
                    LookupResult::Ok(_, proof) => proof,
                    LookupResult::NotFound(proof) => proof,
                    LookupResult::NotInMemory => panic!("failed to find key {key}"),
                };
                assert_eq!(proof, storage.get_path(snapshot, *key).await.unwrap());
                assert_eq!(val.as_ref(), proof.elem());
                // Check path is valid for test_tree
                if val.is_some() {
                    MockMerkleTree::verify(tree.commitment().digest(), key, proof)
                        .unwrap()
                        .unwrap();
                } else {
                    assert!(tree.non_membership_verify(key, proof).unwrap());
                }
            }

            // Check that we can even get a non-membership proof for a key that we never touched.
            let proof = match loaded.universal_lookup(RESERVED_KEY) {
                LookupResult::Ok(_, proof) => proof,
                LookupResult::NotFound(proof) => proof,
                LookupResult::NotInMemory => panic!("failed to find reserved key {RESERVED_KEY}"),
            };
            assert_eq!(
                proof,
                storage.get_path(snapshot, RESERVED_KEY).await.unwrap()
            );
            assert_eq!(proof.elem(), None);
            // Check path is valid for test_tree
            assert!(tree.non_membership_verify(RESERVED_KEY, proof).unwrap());
        }

        // Create a randomized Merkle tree.
        let mut expected = HashMap::<usize, Option<usize>>::new();
        randomize(&mut test_tree, &mut expected);

        // Commit the randomized tree to storage.
        store(&mut storage, &test_tree, &expected, 1).await;
        validate(&storage, &test_tree, &expected, 1).await;

        // Make random edits and commit another snapshot.
        let mut expected2 = expected.clone();
        let mut test_tree2 = test_tree.clone();
        randomize(&mut test_tree2, &mut expected2);
        store(&mut storage, &test_tree2, &expected2, 2).await;
        validate(&storage, &test_tree2, &expected2, 2).await;

        // Ensure the original snapshot is still valid.
        validate(&storage, &test_tree, &expected, 1).await;
    }

    #[async_std::test]
    async fn test_merklized_state_missing_leaf() {
        // Check that if a leaf is missing but its ancestors are present/key is in the tree, we
        // catch it rather than interpreting the entry as an empty node by default. Note that this
        // scenario should be impossible in normal usage, since we never store or delete partial
        // paths. But we should never return an invalid proof even in extreme cases like database
        // corruption.
        setup_test();

        for tree_size in 1..=3 {
            let db = TmpDb::init().await;
            let mut storage = SqlStorage::connect(db.config()).await.unwrap();

            // Define a test tree
            let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
            for i in 0..tree_size {
                test_tree.update(i, i).unwrap();
            }

            // Insert a header with the tree commitment.
            storage
                .query_opt(
                    "INSERT INTO HEADER VALUES (0, 'hash', 'hash', 0, $1)",
                    [
                        sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
                    ],
                )
                .await
                .unwrap();

            // Insert Merkle nodes.
            for i in 0..tree_size {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                let traversal_path =
                    ToTraversalPath::<8>::to_traversal_path(&i, test_tree.height());
                UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                    &mut storage,
                    proof,
                    traversal_path,
                    0,
                )
                .await
                .unwrap();
            }
            storage.set_last_state_height(0).await.unwrap();
            storage.commit().await.unwrap();

            // Test that we can get all the entries.
            let snapshot = Snapshot::<MockTypes, MockMerkleTree, 8>::Index(0);
            for i in 0..tree_size {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                assert_eq!(proof, storage.get_path(snapshot, i).await.unwrap());
                assert_eq!(*proof.elem().unwrap(), i);
            }

            // Now delete the leaf node for the last entry we inserted, corrupting the database.
            let index = serde_json::to_value(tree_size - 1).unwrap();
            storage
                .transaction()
                .await
                .unwrap()
                .execute_one_with_retries(
                    &format!(
                        "DELETE FROM {} WHERE index = $1",
                        MockMerkleTree::state_type()
                    ),
                    [index],
                )
                .await
                .unwrap();
            storage.commit().await.unwrap();

            // Test that we can still get the entries we didn't delete.
            for i in 0..tree_size - 1 {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                assert_eq!(proof, storage.get_path(snapshot, i).await.unwrap());
                assert_eq!(*proof.elem().unwrap(), i);
            }

            // Looking up the entry we deleted fails, rather than return an invalid path.
            let err = storage.get_path(snapshot, tree_size - 1).await.unwrap_err();
            assert!(matches!(err, QueryError::Missing));
        }
    }
}
