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

use super::{buffered_channel::BufferedChannel, VersionedDataSource};
use crate::{
    availability::{
        AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData, QueryablePayload,
        ResourceId, TransactionHash, TransactionIndex, UpdateAvailabilityData,
    },
    metrics::PrometheusMetrics,
    status::StatusDataSource,
    Header, Leaf, MissingSnafu, NotFoundSnafu, Payload, QueryError, QueryResult,
};
use async_std::{net::ToSocketAddrs, task::spawn};
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
        signature_key::EncodedPublicKey,
    },
};
use itertools::Itertools;
use snafu::OptionExt;
use std::{
    ops::{Bound, RangeBounds},
    pin::Pin,
};
use tokio_postgres::{
    types::{BorrowToSql, ToSql},
    Client, NoTls, Row, RowStream, ToStatement,
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
        $crate::data_source::sql::include_dir!($dir)
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
                Migration::unapplied(name, sql).expect("invalid migration")
            })
    };
}

/// The migrations requied to build the default schema for this version of [`SqlDataSource`].
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
    host: String,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    migrations: Vec<Migration>,
    no_migrations: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            user: None,
            password: None,
            database: None,
            migrations: vec![],
            no_migrations: false,
        }
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
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Set a password for connecting to the database.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set the name of the database to connect to.
    pub fn database(mut self, database: impl Into<String>) -> Self {
        self.database = Some(database.into());
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

    /// Connect to the database with this config.
    pub async fn connect<Types>(self) -> Result<SqlDataSource<Types>, Error>
    where
        Types: NodeType,
    {
        SqlDataSource::connect(self).await
    }
}

/// A data source for the APIs provided in this crate, backed by a remote PostgreSQL database.
///
/// # Administration
///
/// This data source will automatically connect to and perform queries on a remote SQL database.
/// However, _administration_ of the database, such as initialization, resetting, and backups, is
/// left out of the scope of this implementation, and is expected to be performed manually using
/// off-the-shelf DBMS adminstration tools. The one exception is migrations, which are handled
/// transparently by the [`SqlDataSource`].
///
/// ## Initialization
///
/// When creating a [`SqlDataSource`], the caller can use [`Config`] to specify the host, user, and
/// database to connect to. As such, [`SqlDataSource`] is not very opinionated about how the
/// Postgres instance is set up. The administrator must simply ensure that there is a database
/// dedicated to the [`SqlDataSource`] and a user with appropriate permissions (all on `SCHEMA` and
/// all on `DATABASE`) over that database.
///
/// Here is an example of how a sufficient database could be initialized. When using the standard
/// `postgres` Docker image, these statements could be placed in
/// `/docker-entrypoint-initdb.d/init.sql` to automatically initialize the database upon startup.
///
/// ```sql
/// CREATE DATABASE hotshot_query_service;
/// \connect hotshot_query_service;
/// CREATE USER hotshot_user WITH PASSWORD 'password';
/// GRANT ALL ON SCHEMA public TO hotshot_user;
/// GRANT ALL ON DATABASE hotshot_query_service TO hotshot_user WITH GRANT OPTION;
/// ```
///
/// One could then connect to this database with the following [`Config`]:
///
/// ```
/// # use hotshot_query_service::data_source::sql::Config;
/// Config::default()
///     .host("postgres.database.hostname")
///     .database("hotshot_query_service")
///     .user("hotshot_user")
///     .password("password")
/// # ;
/// ```
///
/// ## Migrations
///
/// For the [`SqlDataSource`] to work, the database must be initialized with the appropriate schema,
/// and the schema must be kept up to date when deploying a new version of this software which
/// depends on a different schema. Both of these tasks are accomplished via _migrations_.
///
/// Each release of this software is bundled with a sequence of migration files: one migration for
/// each release that changed the schema, including the latest one. Replaying these SQL files
/// against a database with an older version of the schema, including a completely empty database,
/// will bring it up to date with the schema required by this version of the software. Upon creating
/// an instance of [`SqlDataSource`] and connecting to a database, the data source will
/// automatically fetch the current version from the database and, if it is old, replay the
/// necessary migration files.
///
/// ## Custom Migrations
///
/// In keeping with the philosophy of this crate, [`SqlDataSource`] is designed to be
/// [extensible and composable](#extension-and-composition). When extending the provided APIs with
/// new, application-specific queries, it will often be desirable to alter the schema of the
/// database in some way, such as adding additional columns to some of the tables or creating new
/// indices. When composing the provided APIs with additional API modules, it may also be desirable
/// to alter the schema, although the changes are more likely to be completely independent of the
/// schema used by this data source, such as adding entirely new tables.
///
/// In either case, the default schema can be modified by inserting additional migrations between
/// the migrations distributed with this crate. The new migrations will then automatically be
/// replayed as necessary when initializing a [`SqlDataSource`]. New custom migrations can be
/// added with each software update, to keep the custom data up to date as the default schema
/// changes.
///
/// Custom migrations can be inserted using [`Config::migrations`]. Each custom migration will be
/// inserted into the overall sequence of migrations in order of version number. The migrations
/// provided by this crate only use version numbers which are multiples of 10, so the non-multiples
/// can be used to insert custom migrations between the default migrations. You can also replace a
/// default migration completely by providing a custom migration with the same version number. This
/// may be useful when an earlier custom migration has altered the schema in such a way that a later
/// migration no longer works as-is. However, this technique is error prone and should be used only
/// when necessary.
///
/// When using custom migrations, it is the user's responsibility to ensure that the resulting
/// schema is compatible with the schema expected by [`SqlDataSource`]. Adding things (tables,
/// columns, indices) should usually be safe. Removing, altering, or renaming things should be done
/// with extreme caution.
///
/// It is standard to store custom migrations as SQL files in a sub-directory of the crate. For ease
/// of release and deploymenet, such directories can be embedded into a Rust binary and parsed into
/// a list of [`Migration`] objects using the [`include_migrations`] macro.
///
/// It is also possible to take complete control over migrating the schema using
/// [`Config::no_migrations`] to prevent the [`SqlDataSource`] from running its own migrations. The
/// database administrator then becomes responsible for manually migrating the database, ensuring the
/// schema is up to date, and ensuring that the schema is at all times compatible with the schema
/// expected by the current version of this software. Nevertheless, this may be the best option when
/// your application-specific schema has diverged significantly from the default schema.
///
/// # Synchronization
///
/// [`SqlDataSource`] implements [`VersionedDataSource`], which means changes are not applied to the
/// underlying database with every operation. Instead, outstanding changes are batched and applied
/// all at once, atomically, whenever [`commit`](Self::commit) is called. Outstanding, uncommitted
/// changes can also be rolled back completely using [`revert`](Self::revert).
///
/// Internally, the data source maintains an open [`Transaction`] whenever there are outstanding
/// changes, and commits the transaction on [`commit`](Self::commit). The underlying database
/// transaction can be accessed directly via [`transaction`](Self::transaction), which makes it
/// possible to compose application-specific database updates atomically with updates made by the
/// [`SqlDataSource`] itself. This is useful for [extension and
/// composition](#extension-and-composition).
///
/// # Extension and Composition
///
/// [`SqlDataSource`] is designed to be both extensible (so you can add additional state to the API
/// modules defined in this crate) and composable (so you can use [`SqlDataSource`] as one component
/// of a larger state type for an application with additional modules).
///
/// ## Extension
///
/// It is possible to add additional, application-specific state to [`SqlDataSource`]. If the new
/// state should live in memory, simply wrap the [`SqlDataSource`] in an
/// [`ExtensibleDataSource`](super::ExtensibleDataSource):
///
/// ```
/// # use hotshot_query_service::data_source::{
/// #   sql::{Config, Error}, ExtensibleDataSource, SqlDataSource,
/// # };
/// # use hotshot_query_service::testing::mocks::{
/// #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
/// # };
/// # async fn doc(config: Config) -> Result<(), Error> {
/// type AppState = &'static str;
///
/// let data_source: ExtensibleDataSource<SqlDataSource<AppTypes, AppNodeImpl>, AppState> =
///     ExtensibleDataSource::new(SqlDataSource::connect(config).await?, "app state");
/// # Ok(())
/// # }
/// ```
///
/// The [`ExtensibleDataSource`](super::ExtensibleDataSource) wrapper implements all the same data
/// source traits as [`SqlDataSource`], and also provides access to the `AppState` parameter for use
/// in API endpoint handlers. This can be used to implement an app-specific data source trait and
/// add a new API endpoint that uses this app-specific data, as described in the
/// [extension guide](crate#extension).
///
/// If the new application-specific state should live in the SQL database itself, the implementation
/// is more involved, but still possible. Follow the steps for
/// [custom migrations](#custom-migrations) to modify the database schema to account for the new
/// data you want to store. You can then access this data through the [`SqlDataSource`] using
/// [`query`](Self::query) to run a custom read-only SQL query or [`transaction`](Self::transaction)
/// to execute a custom atomic mutation of the database. If you use
/// [`transaction`](Self::transaction), be sure to call [`commit`](Self::commit) when you are ready
/// to persist your changes.
///
/// You will typically use [`query`](Self::query) to read custom data in API endpoint handlers and
/// [`transaction`](Self::transaction) to populate custom data in your web server's update loop.
///
/// ## Composition
///
/// Composing [`SqlDataSource`] with other module states is fairly simple -- just
/// create an aggregate struct containing both [`SqlDataSource`] and your additional module
/// states, as described in the [composition guide](crate#composition). If the additional modules
/// have data that should live in the same database as the [`SqlDataSource`] data, you can follow
/// the steps in [custom migrations](#custom-migrations) to accomodate this. When modifying that
/// data, you can use [`transaction`](Self::transaction) to atomically synchronize updates to the
/// other modules' data with updates to the [`SqlDataSource`]. If the additional data is completely
/// independent of HotShot query service data and does not need to be synchronized, you can also
/// connect to the database directly to make updates.
///
/// In the following example, we compose HotShot query service modules with other application-
/// specific modules, synchronizing updates using [`transaction`](Self::transaction).
///
/// ```
/// # use async_std::{sync::{Arc, RwLock}, task::spawn};
/// # use futures::StreamExt;
/// # use hotshot::types::SystemContextHandle;
/// # use hotshot_query_service::Error;
/// # use hotshot_query_service::data_source::{
/// #   sql::Config, SqlDataSource, UpdateDataSource, VersionedDataSource,
/// # };
/// # use hotshot_query_service::testing::mocks::{
/// #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
/// # };
/// # use tide_disco::App;
/// struct AppState {
///     hotshot_qs: SqlDataSource<AppTypes, AppNodeImpl>,
///     // additional state for other modules
/// }
///
/// async fn init_server(
///     config: Config,
///     mut hotshot: SystemContextHandle<AppTypes, AppNodeImpl>,
/// ) -> Result<App<Arc<RwLock<AppState>>, Error>, Error> {
///     let mut hotshot_qs = SqlDataSource::connect(config).await.map_err(Error::internal)?;
///     // Initialize storage for other modules, using `hotshot_qs` to access the database.
///     let tx = hotshot_qs.transaction().await.map_err(Error::internal)?;
///     // ...
///
///     let state = Arc::new(RwLock::new(AppState {
///         hotshot_qs,
///         // additional state for other modules
///     }));
///     let mut app = App::with_state(state.clone());
///     // Register API modules.
///
///     spawn(async move {
///         let mut events = hotshot.get_event_stream(Default::default()).await.0;
///         while let Some(event) = events.next().await {
///             let mut state = state.write().await;
///             UpdateDataSource::<AppTypes, AppNodeImpl>::update(&mut state.hotshot_qs, &event)
///                 .await
///                 .unwrap();
///             // Update other modules' states based on `event`. Use `hotshot_qs` to include
///             // database updates in the same atomic transaction as `hotshot_qs.update`.
///             let tx = state.hotshot_qs.transaction().await.unwrap();
///
///             // Commit all outstanding changes to the entire state at the same time.
///             state.hotshot_qs.commit().await.unwrap();
///         }
///     });
///
///     Ok(app)
/// }
/// ```
#[derive(Debug)]
pub struct SqlDataSource<Types>
where
    Types: NodeType,
{
    client: Client,
    tx_in_progress: bool,
    leaf_stream: BufferedChannel<LeafQueryData<Types>>,
    block_stream: BufferedChannel<BlockQueryData<Types>>,
    metrics: PrometheusMetrics,
    kill: Option<oneshot::Sender<()>>,
}

impl<Types> SqlDataSource<Types>
where
    Types: NodeType,
{
    /// Connect to a remote database.
    pub async fn connect(mut config: Config) -> Result<Self, Error> {
        // Establish a TCP connection to the server.
        let tcp = TcpStream::connect((config.host.as_str(), config.port)).await?;

        // Convert the TCP connection into a postgres connection.
        let mut pgcfg = postgres::Config::default();
        if let Some(user) = &config.user {
            pgcfg.user(user);
        }
        if let Some(password) = &config.password {
            pgcfg.password(password);
        }
        if let Some(database) = &config.database {
            pgcfg.dbname(database);
        }
        let (mut client, connection) = pgcfg.connect_raw(tcp, NoTls).await?;

        // Spawn a task to drive the connection, with a channel to kill it when this data source is
        // dropped.
        let (kill, killed) = oneshot::channel();
        spawn(select(killed, connection).inspect(|res| {
            if let Either::Right((res, _)) = res {
                // If we were killed, do nothing. That is the normal shutdown path. But if the
                // `select` returned because the `connection` terminated, we should log something,
                // as that is unusual.
                match res {
                    Ok(()) => tracing::warn!("postgres connection terminated unexpectedly"),
                    Err(err) => tracing::error!("postgres connection closed with error: {err}"),
                }
            }
        }));

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
            client,
            tx_in_progress: false,
            kill: Some(kill),
            leaf_stream: BufferedChannel::init(),
            block_stream: BufferedChannel::init(),
            metrics: Default::default(),
        })
    }

    /// Query the underlying SQL database.
    pub async fn query<T, P>(&self, query: &T, params: P) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        Ok(self.client.query_raw(query, params).await?)
    }

    /// Access the transaction which is accumulating all uncommitted changes to the data source.
    ///
    /// This can be used to manually group database modifications to custom state atomically with
    /// modifications made through the [`SqlDataSource`].
    ///
    /// If there is no currently open transaction, a new transaction will be opened. No changes
    /// made through the transaction objeect returned by this method will be persisted until
    /// [`commit`](Self::commit) is called.
    pub async fn transaction(&mut self) -> Result<Transaction<'_>, Error> {
        if !self.tx_in_progress {
            // If there is no transaction in progress, open one.
            self.client.batch_execute("BEGIN").await?;
            self.tx_in_progress = true;
        }
        Ok(Transaction {
            client: &mut self.client,
        })
    }
}

impl<Types> Drop for SqlDataSource<Types>
where
    Types: NodeType,
{
    fn drop(&mut self) {
        if let Some(kill) = self.kill.take() {
            // Ignore errors, they just mean the task has already exited.
            kill.send(()).ok();
        }
    }
}

#[async_trait]
impl<Types> VersionedDataSource for SqlDataSource<Types>
where
    Types: NodeType,
{
    type Error = postgres::error::Error;

    /// Atomically commit to all outstanding modifications to the data.
    ///
    /// If this method fails, outstanding changes are left unmodified. The caller may opt to retry
    /// or to erase outstanding changes with [`revert`](Self::revert).
    async fn commit(&mut self) -> Result<(), Self::Error> {
        if self.tx_in_progress {
            self.client.batch_execute("COMMIT").await?;
            self.tx_in_progress = false;
        }
        self.leaf_stream.flush().await;
        self.block_stream.flush().await;
        Ok(())
    }

    /// Erase all oustanding modifications to the data.
    ///
    /// This function must not return if it has failed to revert changes. Inability to revert
    /// changes to the database is considered a fatal error, and this function may panic.
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
        self.leaf_stream.clear();
        self.block_stream.clear();
    }
}

#[async_trait]
impl<Types> AvailabilityDataSource<Types> for SqlDataSource<Types>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    type LeafStream = BoxStream<'static, QueryResult<LeafQueryData<Types>>>;
    type BlockStream = BoxStream<'static, QueryResult<BlockQueryData<Types>>>;

    type LeafRange<'a, R> = BoxStream<'static, QueryResult<LeafQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;
    type BlockRange<'a, R>= BoxStream<'static, QueryResult<BlockQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync,
    {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id.into() {
            ResourceId::Number(n) => ("height = $1", Box::new(n as i64)),
            ResourceId::Hash(h) => ("hash = $1", Box::new(h.to_string())),
        };
        let query = format!("SELECT leaf, qc FROM leaf WHERE {where_clause}");
        let row = self
            .client
            .query_opt(&query, &[&*param])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?
            .context(NotFoundSnafu)?;
        parse_leaf(row)
    }

    async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id.into() {
            ResourceId::Number(n) => ("h.height = $1", Box::new(n as i64)),
            ResourceId::Hash(h) => ("h.hash = $1", Box::new(h.to_string())),
        };
        // ORDER BY h.height ASC ensures that if there are duplicate blocks, we return the first one.
        // TODO this ordering should be unnecessary once we eliminate duplicate blocks:
        // https://github.com/EspressoSystems/hotshot-query-service/issues/284
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self
            .client
            .query_opt(&query, &[&*param])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?
            .context(NotFoundSnafu)?;
        parse_block(row)
    }

    async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "height");
        let query = format!("SELECT leaf, qc FROM leaf {where_clause} ORDER BY height ASC");
        let rows =
            self.client
                .query_raw(&query, params)
                .await
                .map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?;

        Ok(rows
            .map(|res| {
                parse_leaf(res.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .boxed())
    }

    async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
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
        let rows =
            self.client
                .query_raw(&query, params)
                .await
                .map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?;

        Ok(rows
            .map(|res| {
                parse_block(res.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .boxed())
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
        let row = self
            .client
            .query_opt(&query, &[&hash.to_string()])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?
            .context(NotFoundSnafu)?;

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

    async fn get_proposals(
        &self,
        proposer: &EncodedPublicKey,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
        let mut query = "SELECT leaf, qc FROM leaf WHERE proposer = $1".to_owned();
        if let Some(limit) = limit {
            // If there is a limit on the number of leaves to return, we want to return the most
            // recent leaves, so order by descending height.
            query = format!("{query} ORDER BY height DESC limit {limit}");
        }
        let rows = self
            .client
            .query_raw(&query, &[&proposer.to_string()])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
        let mut leaves: Vec<_> = rows
            .map(|res| {
                parse_leaf(res.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        if limit.is_some() {
            // If there was a limit, we selected the leaves in descending order to get the most
            // recent leaves. Now reverse them to put them back in chronological order.
            leaves.reverse();
        }

        Ok(leaves)
    }

    async fn count_proposals(&self, proposer: &EncodedPublicKey) -> QueryResult<usize> {
        let query = "SELECT count(*) FROM leaf WHERE proposer = $1";
        let row = self
            .client
            .query_one(query, &[&proposer.to_string()])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
        let count: i64 = row.get(0);
        Ok(count as usize)
    }

    async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream> {
        // Fetch leaves above `height` which have already been produced.
        let current_leaves = self.get_leaf_range(height..).await?;
        // Subscribe to future leaves after that.
        let future_leaves = self.leaf_stream.subscribe().await.map(Ok);
        Ok(current_leaves.chain(future_leaves).boxed())
    }

    async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> {
        // Fetch blocks above `height` which have already been produced.
        let current_blocks = self.get_block_range(height..).await?;
        // Subscribe to future blocks after that.
        let future_blocks = self.block_stream.subscribe().await.map(Ok);
        Ok(current_blocks.chain(future_blocks).boxed())
    }
}

#[async_trait]
impl<Types> UpdateAvailabilityData<Types> for SqlDataSource<Types>
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
            "INSERT INTO header (height, hash, data) VALUES ($1, $2, $3, $4)".into(),
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
        let mut tx = self.transaction().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.execute_many(stmts)
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;

        self.leaf_stream.push(leaf);
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
        for (txn_ix, txn) in block.payload().enumerate() {
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

        let mut tx = self.transaction().await.map_err(|err| QueryError::Error {
            message: err.to_string(),
        })?;
        tx.execute_many(stmts)
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;

        self.block_stream.push(block);
        Ok(())
    }
}

#[async_trait]
impl<Types> StatusDataSource for SqlDataSource<Types>
where
    Types: NodeType,
{
    async fn block_height(&self) -> QueryResult<usize> {
        let query = "SELECT max(height) FROM header";
        let row = self
            .client
            .query_one(query, &[])
            .await
            .map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?;
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

    fn metrics(&self) -> &PrometheusMetrics {
        &self.metrics
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
pub struct Transaction<'a> {
    client: &'a mut Client,
}

impl<'a> Transaction<'a> {
    /// Query the underlying SQL database.
    ///
    /// The results will reflect the state after the statements thus far added to this transaction
    /// have been applied, even though those effects have not been committed to the database yet.
    pub async fn query<T, P>(&self, query: &T, params: P) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        Ok(self.client.query_raw(query, params).await?)
    }

    /// Execute a statement against the underlying database.
    ///
    /// The results of the statement will be reflected immediately in future statements made within
    /// this transaction, but will not be reflected in the underlying database until the transaction
    /// is committed with [`SqlDataSource::commit`].
    pub async fn execute<T, P>(&mut self, statement: &T, params: P) -> Result<(), Error>
    where
        T: ?Sized + ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        self.client.execute_raw(statement, params).await?;
        Ok(())
    }

    pub async fn execute_many<S, T, P>(&mut self, statements: S) -> Result<(), Error>
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
    use super::*;
    use crate::testing::{
        mocks::{MockTypes, TestableDataSource},
        sleep,
    };
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

    #[async_trait]
    impl TestableDataSource for SqlDataSource<MockTypes> {
        type Storage = TmpDb;

        async fn create(_node_id: usize) -> Self::Storage {
            TmpDb::init().await
        }

        async fn connect(tmp_db: &Self::Storage) -> Self {
            Config::default()
                .user("postgres")
                .password("password")
                .port(tmp_db.port())
                .connect()
                .await
                .unwrap()
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod generic_test {
    use super::super::data_source_tests;
    use super::SqlDataSource;
    use crate::testing::mocks::{MockNodeImpl, MockTypes};

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_data_source_tests!(SqlDataSource<MockTypes, MockNodeImpl>);
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use super::{testing::TmpDb, *};
    use crate::testing::{
        mocks::{MockNodeImpl, MockTypes},
        setup_test,
    };

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
            let client: SqlDataSource<MockTypes, MockNodeImpl> = cfg.connect().await?;
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
}
