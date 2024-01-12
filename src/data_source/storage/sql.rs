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

use super::AvailabilityStorage;
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, QueryablePayload, ResourceId,
        TransactionHash, TransactionIndex, UpdateAvailabilityData,
    },
    data_source::VersionedDataSource,
    node::{NodeDataSource, UpdateNodeData},
    Header, Leaf, MissingSnafu, NotFoundSnafu, Payload, QueryError, QueryResult, SignatureKey,
};
use async_std::{net::ToSocketAddrs, sync::Arc, task::spawn};
use async_trait::async_trait;
use commit::Committable;
use futures::{
    channel::oneshot,
    future::{select, Either, FutureExt},
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
use itertools::Itertools;
use postgres_native_tls::TlsConnector;
use snafu::OptionExt;
use std::{
    borrow::Cow,
    ops::{Bound, RangeBounds},
    pin::Pin,
    str::FromStr,
};
use tokio_postgres::{
    config::Host,
    tls::TlsConnect,
    types::{BorrowToSql, ToSql},
    Client, NoTls, Row, ToStatement,
};

pub use crate::include_migrations;
pub use anyhow::Error;
pub use refinery::Migration;
pub use tokio_postgres as postgres;

// This needs to be reexported so that we can reference it by absolute path relative to this crate
// in the expansion of `include_migrations`, even when `include_migrations` is invoked from another
// crate which doesn't have `include_dir` as a dependency.
pub use include_dir::include_dir;

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
/// let migrations: Vec<Migration> = include_migrations!("$CARGO_MANIFEST_DIR/migrations").collect();
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
}

/// Storage for the APIs provided in this crate, backed by a remote PostgreSQL database.
#[derive(Debug)]
pub struct SqlStorage {
    client: Arc<Client>,
    tx_in_progress: bool,
    kill: Option<oneshot::Sender<()>>,
}

impl SqlStorage {
    /// Connect to a remote database.
    pub async fn connect(mut config: Config) -> Result<Self, Error> {
        // Establish a TCP connection to the server.
        let tcp = TcpStream::connect((config.host.as_str(), config.port)).await?;

        // Convert the TCP connection into a postgres connection.
        let (mut client, kill) = if config.tls {
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
                return Err(Error::msg(format!("DB is out of date: last applied migration is {last_applied:?}, but expected {last_expected:?}")));
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
            kill: Some(kill),
        })
    }
}

impl SqlStorage {
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

#[async_trait]
impl Query for SqlStorage {
    async fn client(&self) -> Cow<Arc<Client>> {
        Cow::Borrowed(&self.client)
    }
}

impl Drop for SqlStorage {
    fn drop(&mut self) {
        if let Some(kill) = self.kill.take() {
            // Ignore errors, they just mean the task has already exited.
            kill.send(()).ok();
        }
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
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id {
            ResourceId::Number(n) => ("height = $1", Box::new(n as i64)),
            ResourceId::Hash(h) => ("hash = $1", Box::new(h.to_string())),
        };
        let query = format!("SELECT leaf, qc FROM leaf WHERE {where_clause}");
        let row = self.query_one(&query, [param]).await?;
        parse_leaf(row)
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id {
            ResourceId::Number(n) => ("h.height = $1", Box::new(n as i64)),
            ResourceId::Hash(h) => ("h.hash = $1", Box::new(h.to_string())),
        };
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_block(row)
    }

    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id {
            ResourceId::Number(n) => ("h.height = $1", Box::new(n as i64)),
            ResourceId::Hash(h) => ("h.hash = $1", Box::new(h.to_string())),
        };
        let query = format!("SELECT {HEADER_COLUMNS} FROM header WHERE {where_clause}");
        let row = self.query_one(&query, [param]).await?;
        parse_header::<Types>(row)
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

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        // ORDER BY t.id ASC ensures that if there are duplicate transactions, we return the first
        // one.
        let query = format!(
            "SELECT {BLOCK_COLUMNS}, t.index AS tx_index
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                JOIN transaction AS t ON t.block_height = h.height
                WHERE t.hash = $1
                ORDER BY t.id ASC
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
{
    type Error = QueryError;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        let mut stmts: Vec<(String, Vec<Box<dyn ToSql + Send + Sync>>)> = vec![];

        // While we don't necessarily have the full block for this leaf yet, we can initialize the
        // header table with block metadata taken from the leaf.
        let header_json =
            serde_json::to_value(&leaf.leaf().block_header).map_err(|err| QueryError::Error {
                message: format!("failed to serialize header: {err}"),
            })?;
        stmts.push((
            "INSERT INTO header (height, hash, data) VALUES ($1, $2, $3)".into(),
            vec![
                Box::new(leaf.height() as i64),
                Box::new(leaf.block_hash().to_string()),
                Box::new(header_json),
            ],
        ));

        // Similarly, we can initialize the payload table with a null payload, which can help us
        // distinguish between blocks that haven't been produced yet and blocks we haven't received
        // yet when answering queries.
        stmts.push((
            "INSERT INTO payload (height) VALUES ($1)".into(),
            vec![Box::new(leaf.height() as i64)],
        ));

        // Finally, we insert the leaf itself, which references the header row we created.
        // Serialize the full leaf and QC to JSON for easy storage.
        let leaf_json = serde_json::to_value(leaf.leaf()).map_err(|err| QueryError::Error {
            message: format!("failed to serialize leaf: {err}"),
        })?;
        let qc_json = serde_json::to_value(leaf.qc()).map_err(|err| QueryError::Error {
            message: format!("failed to serialize QC: {err}"),
        })?;
        stmts.push((
            "INSERT INTO leaf (height, hash, proposer, block_hash, leaf, qc)
             VALUES ($1, $2, $3, $4, $5, $6)"
                .into(),
            vec![
                Box::new(leaf.height() as i64),
                Box::new(leaf.hash().to_string()),
                Box::new(leaf.proposer().to_string()),
                Box::new(leaf.block_hash().to_string()),
                Box::new(leaf_json),
                Box::new(qc_json),
            ],
        ));

        // Grab a transaction and execute all the statements.
        let mut tx = self.transaction().await?;
        tx.execute_many(stmts).await?;

        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        let mut stmts: Vec<(String, Vec<Box<dyn ToSql + Send + Sync>>)> = vec![];

        // The header and payload tables should already have been initialized when we inserted the
        // corresponding leaf. All we have to do is add the payload itself and its size.
        let payload = block
            .payload
            .encode()
            .map_err(|err| QueryError::Error {
                message: format!("failed to serialize block: {err}"),
            })?
            .collect::<Vec<_>>();
        stmts.push((
            "UPDATE payload SET (data, size) = ($1, $2) WHERE height = $3".into(),
            vec![
                Box::new(payload),
                Box::new(block.size() as i32),
                Box::new(block.height() as i64),
            ],
        ));

        // Index the transactions in the block.
        let mut values = vec![];
        let mut params: Vec<Box<dyn ToSql + Send + Sync>> = vec![];
        for (txn_ix, txn) in block.enumerate() {
            let txn_ix = serde_json::to_value(&txn_ix).map_err(|err| QueryError::Error {
                message: format!("failed to serialize transaction index: {err}"),
            })?;
            values.push(format!(
                "(${},${},${})",
                params.len() + 1,
                params.len() + 2,
                params.len() + 3
            ));
            params.push(Box::new(txn.commit().to_string()));
            params.push(Box::new(block.height() as i64));
            params.push(Box::new(txn_ix));
        }
        if !values.is_empty() {
            stmts.push((
                format!(
                    "INSERT INTO transaction (hash, block_height, index) VALUES {}",
                    values.join(",")
                ),
                params,
            ));
        }

        let mut tx = self.transaction().await?;
        tx.execute_many(stmts).await?;

        Ok(())
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

    async fn get_proposals(
        &self,
        proposer: &SignatureKey<Types>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
        let mut query = "SELECT leaf, qc FROM leaf WHERE proposer = $1".to_owned();
        if let Some(limit) = limit {
            // If there is a limit on the number of leaves to return, we want to return the most
            // recent leaves, so order by descending height.
            query = format!("{query} ORDER BY height DESC limit {limit}");
        }
        let rows = self.query(&query, &[&proposer.to_string()]).await?;
        let mut leaves: Vec<_> = rows.map(|res| parse_leaf(res?)).try_collect().await?;

        if limit.is_some() {
            // If there was a limit, we selected the leaves in descending order to get the most
            // recent leaves. Now reverse them to put them back in chronological order.
            leaves.reverse();
        }

        Ok(leaves)
    }

    async fn count_proposals(&self, proposer: &SignatureKey<Types>) -> QueryResult<usize> {
        let query = "SELECT count(*) FROM leaf WHERE proposer = $1";
        let row = self.query_one(query, &[&proposer.to_string()]).await?;
        let count: i64 = row.get(0);
        Ok(count as usize)
    }
}

#[async_trait]
impl<Types> UpdateNodeData<Types> for SqlStorage
where
    Types: NodeType,
{
    type Error = QueryError;

    async fn insert_leaf(&mut self, _leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        // The node data source borrows data that is populated by the availability source, so
        // there's nothing more to do here.
        Ok(())
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
    pub async fn execute<T, P>(&mut self, statement: &T, params: P) -> QueryResult<()>
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
            })?;
        Ok(())
    }

    pub async fn execute_many<S, T, P>(&mut self, statements: S) -> QueryResult<()>
    where
        S: IntoIterator<Item = (T, P)>,
        T: ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        for (stmt, params) in statements {
            self.execute(&stmt, params).await?;
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

const BLOCK_COLUMNS: &str =
    "h.hash AS hash, h.data AS header_data, p.size AS payload_size, p.data AS payload_data";

fn parse_block<Types>(row: Row) -> QueryResult<BlockQueryData<Types>>
where
    Types: NodeType,
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
    let payload = Payload::<Types>::from_bytes(payload_data.into_iter(), header.metadata());

    // Reconstruct the query data by adding metadata.
    let hash: String = row.try_get("hash").map_err(|err| QueryError::Error {
        message: format!("error extracting block hash from query results: {err}"),
    })?;
    let hash = hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed block hash: {err}"),
    })?;

    Ok(BlockQueryData {
        header,
        payload,
        size,
        hash,
    })
}

const HEADER_COLUMNS: &str = "data";

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
/// Spawns a background task to run the connection. Returns a client and a channel to kill the
/// connection task.
async fn connect<T>(
    pgcfg: postgres::Config,
    tcp: TcpStream,
    tls: T,
) -> anyhow::Result<(Client, oneshot::Sender<()>)>
where
    T: TlsConnect<TcpStream>,
    T::Stream: Send + 'static,
{
    let (client, connection) = pgcfg.connect_raw(tcp, tls).await?;

    // Spawn a task to drive the connection, with a channel to kill it when this data source is
    // dropped.
    let (kill, killed) = oneshot::channel();
    spawn(select(killed, connection).inspect(|res| {
        if let Either::Right((res, _)) = res {
            // If we were killed, do nothing. That is the normal shutdown path. But if the `select`
            // returned because the `connection` terminated, we should log something, as that is
            // unusual.
            match res {
                Ok(()) => tracing::warn!("postgres connection terminated unexpectedly"),
                Err(err) => tracing::error!("postgres connection closed with error: {err}"),
            }
        }
    }));

    Ok((client, kill))
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
    use crate::testing::sleep;
    use portpicker::pick_unused_port;
    use std::{
        process::{Command, Stdio},
        str,
        time::Duration,
    };

    #[derive(Debug)]
    pub struct TmpDb {
        port: u16,
        container_id: String,
    }

    impl TmpDb {
        pub async fn init() -> Self {
            let port = pick_unused_port().unwrap();

            let output = Command::new("docker")
                .arg("run")
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
            let db = Self { port, container_id };

            // Wait for the database to be ready.
            while !Command::new("psql")
                .args(["-h", "localhost", "-p", &port.to_string(), "-U", "postgres"])
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

        pub fn port(&self) -> u16 {
            self.port
        }
    }

    impl Drop for TmpDb {
        fn drop(&mut self) {
            let output = Command::new("docker")
                .args(["kill", self.container_id.as_str()])
                .output()
                .unwrap();
            if !output.status.success() {
                tracing::error!(
                    "error killing postgres docker {}: {}",
                    self.container_id,
                    str::from_utf8(&output.stderr).unwrap()
                );
            }

            let output = Command::new("docker")
                .args(["rm", self.container_id.as_str()])
                .output()
                .unwrap();
            if !output.status.success() {
                tracing::error!(
                    "error removing postgres docker {}: {}",
                    self.container_id,
                    str::from_utf8(&output.stderr).unwrap()
                );
            }
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use super::{testing::TmpDb, *};
    use crate::testing::setup_test;

    #[async_std::test]
    async fn test_migrations() {
        setup_test();

        let db = TmpDb::init().await;
        let port = db.port();

        let connect = |migrations: bool, custom_migrations| async move {
            let mut cfg = Config::default()
                .user("postgres")
                .password("password")
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
                "V12__create_test_table.sql",
                "ALTER TABLE test ADD COLUMN data INTEGER;",
            )
            .unwrap(),
            Migration::unapplied("V11__create_test_table.sql", "CREATE TABLE test ();").unwrap(),
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
}
