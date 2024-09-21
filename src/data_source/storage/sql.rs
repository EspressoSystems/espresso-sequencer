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

use crate::{
    data_source::{
        storage::pruning::{PruneStorage, PrunerCfg, PrunerConfig},
        update::{ReadOnly, Transaction as _},
        VersionedDataSource,
    },
    BackgroundTask, QueryError, QueryResult,
};
use async_std::{
    net::ToSocketAddrs,
    sync::Mutex,
    task::{Context, Poll},
};
use async_trait::async_trait;
use chrono::Utc;
use futures::{AsyncRead, AsyncWrite};
use hotshot_example_types::node_types::TestVersions;
use itertools::Itertools;
use postgres_native_tls::TlsConnector;
use std::{cmp::min, fmt::Debug, pin::Pin, str::FromStr};
use tokio_postgres::{config::Host, tls::TlsConnect, Client, NoTls};

mod query;
mod transaction;

use self::transaction::Connection;

pub use anyhow::Error;
// This needs to be reexported so that we can reference it by absolute path relative to this crate
// in the expansion of `include_migrations`, even when `include_migrations` is invoked from another
// crate which doesn't have `include_dir` as a dependency.
pub use crate::include_migrations;
pub use include_dir::include_dir;
pub use refinery::Migration;
pub use tokio_postgres as postgres;
pub use transaction::Transaction;

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
    archive: bool,
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
            archive: false,
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

    /// Enable pruning with a given configuration.
    ///
    /// If [`archive`](Self::archive) was previously specified, this will override it.
    pub fn pruner_cfg(mut self, cfg: PrunerCfg) -> Result<Self, Error> {
        cfg.validate()?;
        self.pruner_cfg = Some(cfg);
        self.archive = false;
        Ok(self)
    }

    /// Disable pruning and reconstruct previously pruned data.
    ///
    /// While running without pruning is the default behavior, the default will not try to
    /// reconstruct data that was pruned in a previous run where pruning was enabled. This option
    /// instructs the service to run without pruning _and_ reconstruct all previously pruned data by
    /// fetching from peers.
    ///
    /// If [`pruner_cfg`](Self::pruner_cfg) was previously specified, this will override it.
    pub fn archive(mut self) -> Self {
        self.pruner_cfg = None;
        self.archive = true;
        self
    }
}

/// Storage for the APIs provided in this crate, backed by a remote PostgreSQL database.
#[derive(Debug)]
pub struct SqlStorage {
    client: Mutex<Connection>,
    _client_task: BackgroundTask,
    // We use a separate client for mutable access (database transactions). This allows runtime
    // serialization of mutable operations while immutable operations can proceed in parallel, which
    // mimics the lose concurrency semantics provided by Postgres. Eventually we will have a
    // transaction pool which allows even multiple transactions to happen in parallel.
    tx_client: Mutex<Connection>,
    _tx_client_task: BackgroundTask,
    pruner_cfg: Option<PrunerCfg>,
}

#[derive(Debug, Default)]
pub struct Pruner {
    pruned_height: Option<u64>,
    target_height: Option<u64>,
    minimum_retention_height: Option<u64>,
}

impl SqlStorage {
    /// Connect to a remote database.
    pub async fn connect(mut config: Config) -> Result<Self, Error> {
        // Establish a TCP connection to the server.
        let tcp = TcpStream::connect((config.host.as_str(), config.port)).await?;

        // Convert the TCP connection into a postgres connection.
        let (mut client, client_task) = if config.tls {
            let tls = TlsConnector::new(native_tls::TlsConnector::new()?, config.host.as_str());
            connect(config.pgcfg.clone(), tcp, tls).await?
        } else {
            connect(config.pgcfg.clone(), tcp, NoTls).await?
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

        if config.archive {
            // If running in archive mode, ensure the pruned height is set to 0, so the fetcher will
            // reconstruct previously pruned data.
            client
                .batch_execute("DELETE FROM pruned_height WHERE id = 1")
                .await?;
        }

        // Open a second connection for mutable transactions.
        let tcp = TcpStream::connect((config.host.as_str(), config.port)).await?;
        let (tx_client, tx_client_task) = if config.tls {
            let tls = TlsConnector::new(native_tls::TlsConnector::new()?, config.host.as_str());
            connect(config.pgcfg, tcp, tls).await?
        } else {
            connect(config.pgcfg, tcp, NoTls).await?
        };
        tx_client
            .batch_execute(&format!("SET search_path TO {}", config.schema))
            .await?;

        Ok(Self {
            client: Mutex::new(client.into()),
            _client_task: client_task,
            tx_client: Mutex::new(tx_client.into()),
            _tx_client_task: tx_client_task,
            pruner_cfg: config.pruner_cfg,
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
        let tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        let row = tx
            .query_one_static("SELECT MIN(height) as height FROM header")
            .await?;

        let height = row.get::<_, Option<i64>>(0).map(|h| h as u64);

        Ok(height)
    }

    async fn get_height_by_timestamp(&self, timestamp: i64) -> QueryResult<Option<u64>> {
        let tx = self.read().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;

        // We order by timestamp and then height, even though logically this is no different than
        // just ordering by height, since timestamps are monotonic. The reason is that this order
        // allows the query planner to efficiently solve the where clause and presort the results
        // based on the timestamp index. The remaining sort on height, which guarantees a unique
        // block if multiple blocks have the same timestamp, is very efficient, because there are
        // never more than a handful of blocks with the same timestamp.
        let row = tx
            .query_opt(
                "SELECT height FROM header
                  WHERE timestamp <= $1
                  ORDER BY timestamp DESC, height DESC
                  LIMIT 1",
                [&timestamp],
            )
            .await?;

        let height = row.map(|row| row.get::<_, i64>(0) as u64);

        Ok(height)
    }
}

#[async_trait]
impl PruneStorage for SqlStorage {
    type Pruner = Pruner;

    async fn get_disk_usage(&self) -> anyhow::Result<u64> {
        let tx = self.read().await?;
        let row = tx
            .query_one_static("SELECT pg_database_size(current_database())")
            .await?;
        let size: i64 = row.get(0);
        Ok(size as u64)
    }

    /// Note: The prune operation may not immediately free up space even after rows are deleted.
    /// This is because a vacuum operation may be necessary to reclaim more space.
    /// PostgreSQL already performs auto vacuuming, so we are not including it here
    /// as running a vacuum operation can be resource-intensive.
    async fn prune(&self, pruner: &mut Pruner) -> anyhow::Result<Option<u64>> {
        let cfg = self.get_pruning_config().ok_or(QueryError::Error {
            message: "Pruning config not found".to_string(),
        })?;
        let batch_size = cfg.batch_size();
        let max_usage = cfg.max_usage();

        // If a pruner run was already in progress, some variables may already be set,
        // depending on whether a batch was deleted and which batch it was (target or minimum retention).
        // This enables us to resume the pruner run from the exact heights.
        // If any of these values are not set, they can be loaded from the database if necessary.
        let mut minimum_retention_height = pruner.minimum_retention_height;
        let mut target_height = pruner.target_height;
        let mut height = match pruner.pruned_height {
            Some(h) => h,
            None => {
                let Some(height) = self.get_minimum_height().await? else {
                    tracing::info!("database is empty, nothing to prune");
                    return Ok(None);
                };

                height
            }
        };

        // Prune data exceeding target retention in batches
        if pruner.target_height.is_none() {
            let th = self
                .get_height_by_timestamp(
                    Utc::now().timestamp() - (cfg.target_retention().as_secs()) as i64,
                )
                .await?;
            target_height = th;
            pruner.target_height = target_height;
        };

        if let Some(target_height) = target_height {
            if height < target_height {
                height = min(height + batch_size, target_height);
                let mut tx = self.write().await?;
                tx.delete_batch(height).await?;
                tx.commit().await.map_err(|e| QueryError::Error {
                    message: format!("failed to commit {e}"),
                })?;

                pruner.pruned_height = Some(height);
                return Ok(Some(height));
            }
        }

        // If threshold is set, prune data exceeding minimum retention in batches
        // This parameter is needed for SQL storage as there is no direct way to get free space.
        if let Some(threshold) = cfg.pruning_threshold() {
            let usage = self.get_disk_usage().await?;

            // Prune data exceeding minimum retention in batches starting from minimum height
            // until usage is below threshold
            if usage > threshold {
                tracing::warn!(
                    "Disk usage {usage} exceeds pruning threshold {:?}",
                    cfg.pruning_threshold()
                );

                if minimum_retention_height.is_none() {
                    minimum_retention_height = self
                        .get_height_by_timestamp(
                            Utc::now().timestamp() - (cfg.minimum_retention().as_secs()) as i64,
                        )
                        .await?;

                    pruner.minimum_retention_height = minimum_retention_height;
                }

                if let Some(min_retention_height) = minimum_retention_height {
                    if (usage as f64 / threshold as f64) > (f64::from(max_usage) / 10000.0)
                        && height < min_retention_height
                    {
                        height = min(height + batch_size, min_retention_height);
                        let mut tx = self.write().await?;
                        tx.delete_batch(height).await?;
                        tx.commit().await.map_err(|e| QueryError::Error {
                            message: format!("failed to commit {e}"),
                        })?;

                        pruner.pruned_height = Some(height);

                        return Ok(Some(height));
                    }
                }
            }
        }

        Ok(None)
    }
}

impl VersionedDataSource for SqlStorage {
    type Transaction<'a> = Transaction<'a>
    where
        Self: 'a;
    type ReadOnly<'a> = ReadOnly<Transaction<'a>>
    where
        Self: 'a;

    async fn write(&self) -> anyhow::Result<Transaction<'_>> {
        let tx = self.tx_client.lock().await;
        Transaction::write(tx).await
    }

    async fn read(&self) -> anyhow::Result<ReadOnly<Transaction<'_>>> {
        let tx = self.client.lock().await;
        Transaction::read(tx).await
    }
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
    use async_compatibility_layer::art::async_timeout;
    use async_std::net::TcpStream;
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
        persistent: bool,
    }
    impl TmpDb {
        pub async fn init() -> Self {
            Self::init_inner(false).await
        }

        pub async fn persistent() -> Self {
            Self::init_inner(true).await
        }

        async fn init_inner(persistent: bool) -> Self {
            let docker_hostname = env::var("DOCKER_HOSTNAME");
            // This picks an unused port on the current system.  If docker is
            // configured to run on a different host then this may not find a
            // "free" port on that system.
            // We *might* be able to get away with this as any remote docker
            // host should hopefully be pretty open with it's port space.
            let port = pick_unused_port().unwrap();
            let host = docker_hostname.unwrap_or("localhost".to_string());

            let mut cmd = Command::new("docker");
            cmd.arg("run")
                .arg("-d")
                .args(["-p", &format!("{port}:5432")])
                .args(["-e", "POSTGRES_PASSWORD=password"]);
            if !persistent {
                cmd.arg("--rm");
            }
            let output = cmd.arg("postgres").output().unwrap();
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
                container_id: container_id.clone(),
                persistent,
            };

            db.wait_for_ready().await;
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

        pub fn stop(&mut self) {
            tracing::info!(container = self.container_id, "stopping postgres");
            let output = Command::new("docker")
                .args(["stop", self.container_id.as_str()])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "error killing postgres docker {}: {}",
                self.container_id,
                str::from_utf8(&output.stderr).unwrap()
            );
        }

        pub async fn start(&mut self) {
            tracing::info!(container = self.container_id, "resuming postgres");
            let output = Command::new("docker")
                .args(["start", self.container_id.as_str()])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "error starting postgres docker {}: {}",
                self.container_id,
                str::from_utf8(&output.stderr).unwrap()
            );

            self.wait_for_ready().await;
        }

        async fn wait_for_ready(&self) {
            let timeout = Duration::from_secs(
                env::var("SQL_TMP_DB_CONNECT_TIMEOUT")
                    .unwrap_or("60".to_string())
                    .parse()
                    .expect("SQL_TMP_DB_CONNECT_TIMEOUT must be an integer number of seconds"),
            );

            if let Err(err) = async_timeout(timeout, async {
                while Command::new("docker")
                    .args([
                        "exec",
                        &self.container_id,
                        "pg_isready",
                        "-h",
                        "localhost",
                        "-U",
                        "postgres",
                    ])
                    .env("PGPASSWORD", "password")
                    // Null input so the command terminates as soon as it manages to connect.
                    .stdin(Stdio::null())
                    // Discard command output.
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    // We should ensure the exit status. A simple `unwrap`
                    // would panic on unrelated errors (such as network
                    // connection failures)
                    .and_then(|status| {
                        status
                            .success()
                            .then_some(true)
                            // Any ol' Error will do
                            .ok_or(std::io::Error::from_raw_os_error(666))
                    })
                    .is_err()
                {
                    tracing::warn!("database is not ready");
                    sleep(Duration::from_secs(1)).await;
                }

                // The above command ensures the database is ready inside the Docker container.
                // However, on some systems, there is a slight delay before the port is exposed via
                // host networking. We don't need to check again that the database is ready on the
                // host (and maybe can't, because the host might not have pg_isready installed), but
                // we can ensure the port is open by just establishing a TCP connection.
                while let Err(err) =
                    TcpStream::connect(format!("{}:{}", self.host, self.port)).await
                {
                    tracing::warn!("database is ready, but port is not available to host: {err:#}");
                    sleep(Duration::from_millis(100)).await;
                }
            })
            .await
            {
                panic!(
                    "failed to connect to TmpDb within configured timeout {timeout:?}: {err:#}\n{}",
                    "Consider increasing the timeout by setting SQL_TMP_DB_CONNECT_TIMEOUT"
                );
            }
        }
    }

    impl Drop for TmpDb {
        fn drop(&mut self) {
            self.stop();
            if self.persistent {
                let output = Command::new("docker")
                    .args(["container", "rm", self.container_id.as_str()])
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "error removing postgres docker {}: {}",
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
        

            ALTER TABLE header
            ADD column test_merkle_tree_root text
            GENERATED ALWAYS as (data->>'test_merkle_tree_root') STORED;

            CREATE TABLE {name}
            (
                path integer[] NOT NULL, 
                created BIGINT NOT NULL,
                hash_id INT NOT NULL REFERENCES hash (id),
                children INT[],
                children_bitvec BIT(8),
                index JSONB,
                entry JSONB 
            );
            ALTER TABLE {name} ADD CONSTRAINT {name}_pk PRIMARY KEY (path, created);
            CREATE INDEX {name}_created ON {name} (created);"
            )
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use async_std::task::sleep;
    use hotshot_example_types::state_types::{TestInstanceState, TestValidatedState};
    use std::time::Duration;

    use super::{testing::TmpDb, *};
    use crate::{
        availability::{LeafQueryData, UpdateAvailabilityData},
        data_source::storage::pruning::PrunedHeightStorage,
        testing::{mocks::MockTypes, setup_test},
    };

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
                "V33__create_test_table.sql",
                "ALTER TABLE test ADD COLUMN data INTEGER;",
            )
            .unwrap(),
            Migration::unapplied("V32__create_test_table.sql", "CREATE TABLE test ();").unwrap(),
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
        let mut leaf = LeafQueryData::<MockTypes>::genesis::<TestVersions>(
            &TestValidatedState::default(),
            &TestInstanceState::default(),
        )
        .await;
        // insert some mock data
        for i in 0..20 {
            leaf.leaf.block_header_mut().block_number = i;
            leaf.leaf.block_header_mut().timestamp = Utc::now().timestamp() as u64;
            let mut tx = storage.write().await.unwrap();
            tx.insert_leaf(leaf.clone()).await.unwrap();
            tx.commit().await.unwrap();
        }

        let height_before_pruning = storage.get_minimum_height().await.unwrap().unwrap();

        // Set pruner config to default which has minimum retention set to 1 day
        storage.set_pruning_config(PrunerCfg::new());
        // No data will be pruned
        let pruned_height = storage.prune(&mut Default::default()).await.unwrap();

        // Vacuum the database to reclaim space.
        // This is necessary to ensure the test passes.
        // Note: We don't perform a vacuum after each pruner run in production because the auto vacuum job handles it automatically.
        storage.client.lock().await.vacuum().await.unwrap();
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
        let pruned_height = storage.prune(&mut Default::default()).await.unwrap();
        // Vacuum the database to reclaim space.
        // This is necessary to ensure the test passes.
        // Note: We don't perform a vacuum after each pruner run in production because the auto vacuum job handles it automatically.
        storage.client.lock().await.vacuum().await.unwrap();

        // Pruned height should be some
        assert!(pruned_height.is_some());
        let usage_after_pruning = storage.get_disk_usage().await.unwrap();
        // All the tables should be empty
        // counting rows in header table
        let header_rows = storage
            .read()
            .await
            .unwrap()
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
            .read()
            .await
            .unwrap()
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
        let mut leaf = LeafQueryData::<MockTypes>::genesis::<TestVersions>(
            &TestValidatedState::default(),
            &TestInstanceState::default(),
        )
        .await;
        // insert some mock data
        for i in 0..20 {
            leaf.leaf.block_header_mut().block_number = i;
            leaf.leaf.block_header_mut().timestamp = Utc::now().timestamp() as u64;
            let mut tx = storage.write().await.unwrap();
            tx.insert_leaf(leaf.clone()).await.unwrap();
            tx.commit().await.unwrap();
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
        let pruned_height = storage.prune(&mut Default::default()).await.unwrap();
        // Vacuum the database to reclaim space.
        // This is necessary to ensure the test passes.
        // Note: We don't perform a vacuum after each pruner run in production because the auto vacuum job handles it automatically.
        storage.client.lock().await.vacuum().await.unwrap();

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
        let pruned_height = storage.prune(&mut Default::default()).await.unwrap();
        // Vacuum the database to reclaim space.
        // This is necessary to ensure the test passes.
        // Note: We don't perform a vacuum after each pruner run in production because the auto vacuum job handles it automatically.
        storage.client.lock().await.vacuum().await.unwrap();

        // Pruned height should be some
        assert!(pruned_height.is_some());
        // All the tables should be empty
        // counting rows in header table
        let header_rows = storage
            .read()
            .await
            .unwrap()
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

        let storage = SqlStorage::connect(cfg).await.unwrap();
        assert!(storage
            .read()
            .await
            .unwrap()
            .load_pruned_height()
            .await
            .unwrap()
            .is_none());
        for height in [10, 20, 30] {
            let mut tx = storage.write().await.unwrap();
            tx.save_pruned_height(height).await.unwrap();
            tx.commit().await.unwrap();
            assert_eq!(
                storage
                    .read()
                    .await
                    .unwrap()
                    .load_pruned_height()
                    .await
                    .unwrap(),
                Some(height)
            );
        }
    }
}
