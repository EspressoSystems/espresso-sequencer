use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use committable::Committable;
use derivative::Derivative;
use derive_more::derive::{From, Into};
use espresso_types::{
    parse_duration,
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
    BackoffParams, Leaf, NetworkConfig, Payload,
};
use futures::stream::StreamExt;
use hotshot_query_service::data_source::storage::sql::{syntax_helpers::MAX_FN, Db};
use hotshot_query_service::data_source::{
    storage::{
        pruning::PrunerCfg,
        sql::{include_migrations, query_as, Config, SqlStorage},
    },
    Transaction as _, VersionedDataSource,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::{Event, EventType, HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{node_implementation::ConsensusTime, BlockPayload},
    utils::View,
    vid::VidSchemeType,
    vote::HasViewNumber,
};
use jf_vid::VidScheme;
use sqlx::Row;
use sqlx::{query, Executor};
use std::{collections::BTreeMap, path::PathBuf, str::FromStr, sync::Arc, time::Duration};
use tempfile::TempDir;

use crate::{catchup::SqlStateCatchup, SeqTypes, ViewNumber};

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Derivative, Default)]
#[derivative(Debug)]
pub struct PostgresOptions {
    /// Hostname for the remote Postgres database server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_HOST")]
    pub(crate) host: Option<String>,

    /// Port for the remote Postgres database server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PORT")]
    pub(crate) port: Option<u16>,

    /// Name of database to connect to.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_DATABASE")]
    pub(crate) database: Option<String>,

    /// Postgres user to connect as.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_USER")]
    pub(crate) user: Option<String>,

    /// Password for Postgres user.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PASSWORD")]
    // Hide from debug output since may contain sensitive data.
    #[derivative(Debug = "ignore")]
    pub(crate) password: Option<String>,

    /// Use TLS for an encrypted connection to the database.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_USE_TLS")]
    pub(crate) use_tls: bool,
}

#[derive(Parser, Clone, Derivative, Default, From, Into)]
#[derivative(Debug)]
pub struct SqliteOptions {
    /// Base directory for the SQLite database.
    /// The SQLite file will be created in the `sqlite` subdirectory with filename as `database`.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_STORAGE_PATH",
        value_parser = build_sqlite_path
    )]
    pub(crate) path: PathBuf,
}

pub fn build_sqlite_path(path: &str) -> anyhow::Result<PathBuf> {
    let mut sub_dir = PathBuf::from_str(path)?.join("sqlite");

    // if `sqlite` sub dir does not exist then create it
    if !sub_dir.exists() {
        sub_dir = if let Ok(()) = std::fs::create_dir_all(&sub_dir) {
            sub_dir
        } else {
            let temp = TempDir::new()
                .with_context(|| "failed to initialize default storage directory".to_string())?;

            let sub = &sub_dir.strip_prefix("/")?;
            dbg!(&temp);
            let temp = temp.path().join(sub);
            dbg!(&temp);
            std::fs::create_dir_all(&temp)
                .with_context(|| format!("failed to create directory: {:?}", temp))?;
            temp
        };
    };
    dbg!(&sub_dir.join("database"));
    Ok(sub_dir.join("database"))
}

/// Options for database-backed persistence, supporting both Postgres and SQLite.
#[derive(Parser, Clone, Derivative, Default, From, Into)]
#[derivative(Debug)]
pub struct Options {
    #[cfg(not(feature = "embedded-db"))]
    #[clap(flatten)]
    pub(crate) postgres_options: PostgresOptions,

    #[cfg(feature = "embedded-db")]
    #[clap(flatten)]
    pub(crate) sqlite_options: SqliteOptions,

    /// Database URI for Postgres or SQLite.
    ///
    /// This is a shorthand for setting a number of other options all at once. The URI has the
    /// following format ([brackets] indicate optional segments):
    ///
    /// - **Postgres:** `postgres[ql]://[username[:password]@][host[:port],]/database[?parameter_list]`
    /// - **SQLite:** `sqlite://path/to/db.sqlite`
    ///
    /// Options set explicitly via other env vars or flags will take precedence, so you can use this
    /// URI to set a baseline and then use other parameters to override or add configuration. In
    /// addition, there are some parameters which cannot be set via the URI, such as TLS.
    // Hide from debug output since may contain sensitive data.
    #[derivative(Debug = "ignore")]
    pub(crate) uri: Option<String>,

    /// This will enable the pruner and set the default pruning parameters unless provided.
    /// Default parameters:
    /// - pruning_threshold: 3 TB
    /// - minimum_retention: 1 day
    /// - target_retention: 7 days
    /// - batch_size: 1000
    /// - max_usage: 80%
    /// - interval: 1 hour
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PRUNE")]
    pub(crate) prune: bool,

    /// Pruning parameters.
    #[clap(flatten)]
    pub(crate) pruning: PruningOptions,

    #[clap(long, env = "ESPRESSO_SEQUENCER_STORE_UNDECIDED_STATE", hide = true)]
    pub(crate) store_undecided_state: bool,

    /// Specifies the maximum number of concurrent fetch requests allowed from peers.
    #[clap(long, env = "ESPRESSO_SEQUENCER_FETCH_RATE_LIMIT")]
    pub(crate) fetch_rate_limit: Option<usize>,

    /// The minimum delay between active fetches in a stream.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ACTIVE_FETCH_DELAY", value_parser = parse_duration)]
    pub(crate) active_fetch_delay: Option<Duration>,

    /// The minimum delay between loading chunks in a stream.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHUNK_FETCH_DELAY", value_parser = parse_duration)]
    pub(crate) chunk_fetch_delay: Option<Duration>,

    /// Disable pruning and reconstruct previously pruned data.
    ///
    /// While running without pruning is the default behavior, the default will not try to
    /// reconstruct data that was pruned in a previous run where pruning was enabled. This option
    /// instructs the service to run without pruning _and_ reconstruct all previously pruned data by
    /// fetching from peers.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ARCHIVE", conflicts_with = "prune")]
    pub(crate) archive: bool,

    /// The maximum idle time of a database connection.
    ///
    /// Any connection which has been open and unused longer than this duration will be
    /// automatically closed to reduce load on the server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_DATABASE_IDLE_CONNECTION_TIMEOUT", value_parser = parse_duration, default_value = "10m")]
    pub(crate) idle_connection_timeout: Duration,

    /// The maximum lifetime of a database connection.
    ///
    /// Any connection which has been open longer than this duration will be automatically closed
    /// (and, if needed, replaced), even if it is otherwise healthy. It is good practice to refresh
    /// even healthy connections once in a while (e.g. daily) in case of resource leaks in the
    /// server implementation.
    #[clap(long, env = "ESPRESSO_SEQUENCER_DATABASE_CONNECTION_TIMEOUT", value_parser = parse_duration, default_value = "30m")]
    pub(crate) connection_timeout: Duration,

    #[clap(long, env = "ESPRESSO_SEQUENCER_DATABASE_SLOW_STATEMENT_THRESHOLD", value_parser = parse_duration, default_value = "1s")]
    pub(crate) slow_statement_threshold: Duration,

    /// The minimum number of database connections to maintain at any time.
    ///
    /// The database client will, to the best of its ability, maintain at least `min` open
    /// connections at all times. This can be used to reduce the latency hit of opening new
    /// connections when at least this many simultaneous connections are frequently needed.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_DATABASE_MIN_CONNECTIONS",
        default_value = "0"
    )]
    pub(crate) min_connections: u32,

    /// The maximum number of database connections to maintain at any time.
    ///
    /// Once `max` connections are in use simultaneously, further attempts to acquire a connection
    /// (or begin a transaction) will block until one of the existing connections is released.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_DATABASE_MAX_CONNECTIONS",
        default_value = "25"
    )]
    pub(crate) max_connections: u32,

    // Keep the database connection pool when persistence is created,
    // allowing it to be reused across multiple instances instead of creating
    // a new pool each time such as for API, consensus storage etc
    // This also ensures all storage instances adhere to the MAX_CONNECTIONS limit if set
    //
    // Note: Cloning the `Pool` is lightweight and efficient because it simply
    // creates a new reference-counted handle to the underlying pool state.
    #[clap(skip)]
    pub(crate) pool: Option<sqlx::Pool<Db>>,
}

#[cfg(not(feature = "embedded-db"))]
impl From<PostgresOptions> for Config {
    fn from(opt: PostgresOptions) -> Self {
        let mut cfg = Config::default();

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

        cfg = cfg.max_connections(20);
        cfg = cfg.idle_connection_timeout(Duration::from_secs(120));
        cfg = cfg.connection_timeout(Duration::from_secs(10240));
        cfg = cfg.slow_statement_threshold(Duration::from_secs(1));

        cfg
    }
}

#[cfg(feature = "embedded-db")]
impl From<SqliteOptions> for Config {
    fn from(opt: SqliteOptions) -> Self {
        let mut cfg = Config::default();

        cfg = cfg.db_path(opt.path);
        cfg = cfg.max_connections(20);
        cfg = cfg.idle_connection_timeout(Duration::from_secs(120));
        cfg = cfg.connection_timeout(Duration::from_secs(10240));
        cfg = cfg.slow_statement_threshold(Duration::from_secs(2));
        cfg
    }
}

#[cfg(not(feature = "embedded-db"))]
impl From<PostgresOptions> for Options {
    fn from(opt: PostgresOptions) -> Self {
        Options {
            postgres_options: opt,
            max_connections: 20,
            idle_connection_timeout: Duration::from_secs(120),
            connection_timeout: Duration::from_secs(10240),
            slow_statement_threshold: Duration::from_secs(1),
            ..Default::default()
        }
    }
}

#[cfg(feature = "embedded-db")]
impl From<SqliteOptions> for Options {
    fn from(opt: SqliteOptions) -> Self {
        Options {
            sqlite_options: opt,
            max_connections: 10,
            idle_connection_timeout: Duration::from_secs(120),
            connection_timeout: Duration::from_secs(10240),
            slow_statement_threshold: Duration::from_secs(1),
            ..Default::default()
        }
    }
}
impl TryFrom<Options> for Config {
    type Error = anyhow::Error;

    fn try_from(opt: Options) -> Result<Self, Self::Error> {
        let mut cfg = match opt.uri {
            Some(uri) => uri.parse()?,
            None => Self::default(),
        };

        if let Some(pool) = opt.pool {
            cfg = cfg.pool(pool);
        }

        cfg = cfg.max_connections(opt.max_connections);
        cfg = cfg.idle_connection_timeout(opt.idle_connection_timeout);
        cfg = cfg.min_connections(opt.min_connections);
        cfg = cfg.connection_timeout(opt.connection_timeout);
        cfg = cfg.slow_statement_threshold(opt.slow_statement_threshold);

        #[cfg(not(feature = "embedded-db"))]
        {
            cfg = cfg.migrations(include_migrations!(
                "$CARGO_MANIFEST_DIR/api/migrations/postgres"
            ));

            let pg_options = opt.postgres_options;

            if let Some(host) = pg_options.host {
                cfg = cfg.host(host);
            }

            if let Some(port) = pg_options.port {
                cfg = cfg.port(port);
            }

            if let Some(database) = &pg_options.database {
                cfg = cfg.database(database);
            }

            if let Some(user) = &pg_options.user {
                cfg = cfg.user(user);
            }

            if let Some(password) = &pg_options.password {
                cfg = cfg.password(password);
            }

            if pg_options.use_tls {
                cfg = cfg.tls();
            }
        }

        #[cfg(feature = "embedded-db")]
        {
            cfg = cfg.migrations(include_migrations!(
                "$CARGO_MANIFEST_DIR/api/migrations/sqlite"
            ));

            cfg = cfg.db_path(opt.sqlite_options.path);
        }

        if opt.prune {
            cfg = cfg.pruner_cfg(PrunerCfg::from(opt.pruning))?;
        }
        if opt.archive {
            cfg = cfg.archive();
        }

        Ok(cfg)
    }
}

/// Pruning parameters.
#[derive(Parser, Clone, Debug, Default)]
pub struct PruningOptions {
    /// Threshold for pruning, specified in bytes.
    /// If the disk usage surpasses this threshold, pruning is initiated for data older than the specified minimum retention period.
    /// Pruning continues until the disk usage drops below the MAX USAGE.
    #[clap(long, env = "ESPRESSO_SEQUENCER_PRUNER_PRUNING_THRESHOLD")]
    pruning_threshold: Option<u64>,

    /// Minimum retention period.
    /// Data is retained for at least this duration, even if there's no free disk space.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PRUNER_MINIMUM_RETENTION",
        value_parser = parse_duration,
    )]
    minimum_retention: Option<Duration>,

    /// Target retention period.
    /// Data older than this is pruned to free up space.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PRUNER_TARGET_RETENTION",
        value_parser = parse_duration,
    )]
    target_retention: Option<Duration>,

    /// Batch size for pruning.
    /// This is the number of blocks data to delete in a single transaction.
    #[clap(long, env = "ESPRESSO_SEQUENCER_PRUNER_BATCH_SIZE")]
    batch_size: Option<u64>,

    /// Maximum disk usage (in basis points).
    ///
    /// Pruning stops once the disk usage falls below this value, even if
    /// some data older than the `MINIMUM_RETENTION` remains. Values range
    /// from 0 (0%) to 10000 (100%).
    #[clap(long, env = "ESPRESSO_SEQUENCER_PRUNER_MAX_USAGE")]
    max_usage: Option<u16>,

    /// Interval for running the pruner.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PRUNER_INTERVAL",
        value_parser = parse_duration,
    )]
    interval: Option<Duration>,
}

impl From<PruningOptions> for PrunerCfg {
    fn from(opt: PruningOptions) -> Self {
        let mut cfg = PrunerCfg::new();
        if let Some(threshold) = opt.pruning_threshold {
            cfg = cfg.with_pruning_threshold(threshold);
        }
        if let Some(min) = opt.minimum_retention {
            cfg = cfg.with_minimum_retention(min);
        }
        if let Some(target) = opt.target_retention {
            cfg = cfg.with_target_retention(target);
        }
        if let Some(batch) = opt.batch_size {
            cfg = cfg.with_batch_size(batch);
        }
        if let Some(max) = opt.max_usage {
            cfg = cfg.with_max_usage(max);
        }
        if let Some(interval) = opt.interval {
            cfg = cfg.with_interval(interval);
        }

        cfg
    }
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    async fn create(&mut self) -> anyhow::Result<Self::Persistence> {
        let store_undecided_state = self.store_undecided_state;
        let config = self.clone().try_into()?;
        let persistence = Persistence {
            store_undecided_state,
            db: SqlStorage::connect(config).await?,
        };
        persistence.migrate_quorum_proposal_leaf_hashes().await?;
        self.pool = Some(persistence.db.pool());
        Ok(persistence)
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
#[derive(Clone)]
pub struct Persistence {
    db: SqlStorage,
    store_undecided_state: bool,
}

impl Persistence {
    /// Ensure the `leaf_hash` column is populated for all existing quorum proposals.
    ///
    /// This column was added in a migration, but because it requires computing a commitment of the
    /// existing data, it is not easy to populate in the SQL migration itself. Thus, on startup, we
    /// check if there are any just-migrated quorum proposals with a `NULL` value for this column,
    /// and if so we populate the column manually.
    async fn migrate_quorum_proposal_leaf_hashes(&self) -> anyhow::Result<()> {
        let mut tx = self.db.write().await?;

        let mut proposals = tx.fetch("SELECT * FROM quorum_proposals");

        let mut updates = vec![];
        while let Some(row) = proposals.next().await {
            let row = row?;

            let hash: Option<String> = row.try_get("leaf_hash")?;
            if hash.is_none() {
                let view: i64 = row.try_get("view")?;
                let data: Vec<u8> = row.try_get("data")?;
                let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
                    bincode::deserialize(&data)?;
                let leaf = Leaf::from_quorum_proposal(&proposal.data);
                let leaf_hash = Committable::commit(&leaf);
                tracing::info!(view, %leaf_hash, "populating quorum proposal leaf hash");
                updates.push((view, leaf_hash.to_string()));
            }
        }
        drop(proposals);

        tx.upsert("quorum_proposals", ["view", "leaf_hash"], ["view"], updates)
            .await?;

        tx.commit().await
    }
}

#[async_trait]
impl SequencerPersistence for Persistence {
    fn into_catchup_provider(
        self,
        backoff: BackoffParams,
    ) -> anyhow::Result<Arc<dyn StateCatchup>> {
        Ok(Arc::new(SqlStateCatchup::new(Arc::new(self.db), backoff)))
    }

    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        tracing::info!("loading config from Postgres");

        // Select the most recent config (although there should only be one).
        let Some(row) = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT config FROM network_config ORDER BY id DESC LIMIT 1")
            .await?
        else {
            tracing::info!("config not found");
            return Ok(None);
        };
        let config = row.try_get("config")?;
        Ok(serde_json::from_value(config)?)
    }

    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        tracing::info!("saving config to database");
        let json = serde_json::to_value(cfg)?;

        let mut tx = self.db.write().await?;
        tx.execute(query("INSERT INTO network_config (config) VALUES ($1)").bind(json))
            .await?;
        tx.commit().await
    }

    async fn append_decided_leaves(
        &self,
        view: ViewNumber,
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, QuorumCertificate<SeqTypes>)> + Send,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<()> {
        let values = leaf_chain
            .into_iter()
            .map(|(info, qc)| {
                // The leaf may come with a large payload attached. We don't care about this payload
                // because we already store it separately, as part of the DA proposal. Storing it
                // here contributes to load on the DB for no reason, so we remove it before
                // serializing the leaf.
                let mut leaf = info.leaf.clone();
                leaf.unfill_block_payload();

                let view = qc.view_number.u64() as i64;
                let leaf_bytes = bincode::serialize(&leaf)?;
                let qc_bytes = bincode::serialize(&qc)?;
                Ok((view, leaf_bytes, qc_bytes))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        // First, append the new leaves. We do this in its own transaction because even if GC or the
        // event consumer later fails, there is no need to abort the storage of the leaves.
        let mut tx = self.db.write().await?;

        tx.upsert("anchor_leaf", ["view", "leaf", "qc"], ["view"], values)
            .await?;
        tx.commit().await?;

        // Generate an event for the new leaves and, only if it succeeds, clean up data we no longer
        // need.
        let consumer = dyn_clone::clone(consumer);

        if let Err(err) = collect_garbage(self, view, consumer).await {
            // GC/event processing failure is not an error, since by this point we have at least
            // managed to persist the decided leaves successfully, and GC will just run again at the
            // next decide. Log an error but do not return it.
            tracing::warn!(?view, "GC/event processing failed: {err:#}");
        }

        Ok(())
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(self
            .db
            .read()
            .await?
            .fetch_optional(query("SELECT view FROM highest_voted_view WHERE id = 0"))
            .await?
            .map(|row| {
                let view: i64 = row.get("view");
                ViewNumber::new(view as u64)
            }))
    }

    async fn load_anchor_leaf(
        &self,
    ) -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>> {
        let Some(row) = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT leaf, qc FROM anchor_leaf ORDER BY view DESC LIMIT 1")
            .await?
        else {
            return Ok(None);
        };

        let leaf_bytes: Vec<u8> = row.get("leaf");
        let leaf = bincode::deserialize(&leaf_bytes)?;

        let qc_bytes: Vec<u8> = row.get("qc");
        let qc = bincode::deserialize(&qc_bytes)?;

        Ok(Some((leaf, qc)))
    }

    async fn load_anchor_view(&self) -> anyhow::Result<ViewNumber> {
        let mut tx = self.db.read().await?;
        let (view,) = query_as::<(i64,)>("SELECT coalesce(max(view), 0) FROM anchor_leaf")
            .fetch_one(tx.as_mut())
            .await?;
        Ok(ViewNumber::new(view as u64))
    }

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        let Some(row) = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT leaves, state FROM undecided_state WHERE id = 0")
            .await?
        else {
            return Ok(None);
        };

        let leaves_bytes: Vec<u8> = row.get("leaves");
        let leaves = bincode::deserialize(&leaves_bytes)?;

        let state_bytes: Vec<u8> = row.get("state");
        let state = bincode::deserialize(&state_bytes)?;

        Ok(Some((leaves, state)))
    }

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>> {
        let result = self
            .db
            .read()
            .await?
            .fetch_optional(
                query("SELECT data FROM da_proposal where view = $1").bind(view.u64() as i64),
            )
            .await?;

        result
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                anyhow::Result::<_>::Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        let result = self
            .db
            .read()
            .await?
            .fetch_optional(
                query("SELECT data FROM vid_share where view = $1").bind(view.u64() as i64),
            )
            .await?;

        result
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                anyhow::Result::<_>::Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>> {
        let rows = self
            .db
            .read()
            .await?
            .fetch_all("SELECT * FROM quorum_proposals")
            .await?;

        Ok(BTreeMap::from_iter(
            rows.into_iter()
                .map(|row| {
                    let view: i64 = row.get("view");
                    let view_number: ViewNumber = ViewNumber::new(view.try_into()?);
                    let bytes: Vec<u8> = row.get("data");
                    let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
                        bincode::deserialize(&bytes)?;
                    Ok((view_number, proposal))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }

    async fn load_quorum_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Proposal<SeqTypes, QuorumProposal<SeqTypes>>> {
        let mut tx = self.db.read().await?;
        let (data,) =
            query_as::<(Vec<u8>,)>("SELECT data FROM quorum_proposals WHERE view = $1 LIMIT 1")
                .bind(view.u64() as i64)
                .fetch_one(tx.as_mut())
                .await?;
        let proposal = bincode::deserialize(&data)?;
        Ok(proposal)
    }

    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "vid_share",
            ["view", "data"],
            ["view"],
            [(view as i64, data_bytes)],
        )
        .await?;
        tx.commit().await
    }
    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
        _vid_commit: <VidSchemeType as VidScheme>::Commit,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "da_proposal",
            ["view", "data"],
            ["view"],
            [(view as i64, data_bytes)],
        )
        .await?;
        tx.commit().await
    }
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        // Todo Remove this after https://github.com/EspressoSystems/espresso-sequencer/issues/1931
        if !matches!(action, HotShotAction::Propose | HotShotAction::Vote) {
            return Ok(());
        }

        let stmt = format!(
            "INSERT INTO highest_voted_view (id, view) VALUES (0, $1) 
            ON CONFLICT (id) DO UPDATE SET view = {MAX_FN}(highest_voted_view.view, excluded.view)"
        );

        let mut tx = self.db.write().await?;
        tx.execute(query(&stmt).bind(view.u64() as i64)).await?;
        tx.commit().await
    }
    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        if !self.store_undecided_state {
            return Ok(());
        }

        let leaves_bytes = bincode::serialize(&leaves).context("serializing leaves")?;
        let state_bytes = bincode::serialize(&state).context("serializing state")?;

        let mut tx = self.db.write().await?;
        tx.upsert(
            "undecided_state",
            ["id", "leaves", "state"],
            ["id"],
            [(0_i32, leaves_bytes, state_bytes)],
        )
        .await?;
        tx.commit().await
    }
    async fn append_quorum_proposal(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.view_number().u64();
        let proposal_bytes = bincode::serialize(&proposal).context("serializing proposal")?;
        let leaf_hash = Committable::commit(&Leaf::from_quorum_proposal(&proposal.data));
        let mut tx = self.db.write().await?;
        tx.upsert(
            "quorum_proposals",
            ["view", "leaf_hash", "data"],
            ["view"],
            [(view_number as i64, leaf_hash.to_string(), proposal_bytes)],
        )
        .await?;
        tx.commit().await
    }

    async fn load_upgrade_certificate(
        &self,
    ) -> anyhow::Result<Option<UpgradeCertificate<SeqTypes>>> {
        let result = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT * FROM upgrade_certificate where id = true")
            .await?;

        result
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                anyhow::Result::<_>::Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn store_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let certificate = match decided_upgrade_certificate {
            Some(cert) => cert,
            None => return Ok(()),
        };
        let upgrade_certificate_bytes =
            bincode::serialize(&certificate).context("serializing upgrade certificate")?;
        let mut tx = self.db.write().await?;
        tx.upsert(
            "upgrade_certificate",
            ["id", "data"],
            ["id"],
            [(true, upgrade_certificate_bytes)],
        )
        .await?;
        tx.commit().await
    }
}

async fn collect_garbage(
    storage: &Persistence,
    view: ViewNumber,
    consumer: impl EventConsumer,
) -> anyhow::Result<()> {
    // In SQLite, overlapping read and write transactions can lead to database errors.
    // To avoid this:
    // - start a read transaction to query and collect all the necessary data.
    // - Commit (or implicitly drop) the read transaction once the data is fetched.
    // - use the collected data to generate a "decide" event for the consumer.
    // - begin a write transaction to delete the data and update the event stream.
    let mut tx = storage.db.read().await?;

    // collect VID shares.
    let mut vid_shares = tx
        .fetch_all(query("SELECT * FROM vid_share where view <= $1").bind(view.u64() as i64))
        .await?
        .into_iter()
        .map(|row| {
            let view: i64 = row.get("view");
            let data: Vec<u8> = row.get("data");
            let vid_proposal =
                bincode::deserialize::<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>(&data)?;
            Ok((view as u64, vid_proposal.data))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

    // collect DA proposals.
    let mut da_proposals = tx
        .fetch_all(query("SELECT * FROM da_proposal where view <= $1").bind(view.u64() as i64))
        .await?
        .into_iter()
        .map(|row| {
            let view: i64 = row.get("view");
            let data: Vec<u8> = row.get("data");
            let da_proposal =
                bincode::deserialize::<Proposal<SeqTypes, DaProposal<SeqTypes>>>(&data)?;
            Ok((view as u64, da_proposal.data))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

    // collect leaves
    let mut leaves = tx
        .fetch_all(
            query("SELECT view, leaf, qc FROM anchor_leaf WHERE view <= $1")
                .bind(view.u64() as i64),
        )
        .await?
        .into_iter()
        .map(|row| {
            let view: i64 = row.get("view");
            let leaf_data: Vec<u8> = row.get("leaf");
            let leaf = bincode::deserialize::<Leaf>(&leaf_data)?;
            let qc_data: Vec<u8> = row.get("qc");
            let qc = bincode::deserialize::<QuorumCertificate<SeqTypes>>(&qc_data)?;
            Ok((view as u64, (leaf, qc)))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

    // Exclude from the decide event any leaves which have definitely already been processed. We may
    // have selected an already-processed leaf because the oldest leaf -- the last leaf processed in
    // the previous decide event -- remained in the database to serve as the anchor leaf, so our
    // query would have returned it. In fact, this will almost always be the case, but there are two
    // cases where it might not be, and we must process this leaf after all:
    //
    // 1. The oldest leaf is the genesis leaf, and there _is_ no previous decide event
    // 2. We previously stored some leaves in the database and then failed while processing the
    //    decide event, or shut down before generating the decide event, and so we are only now
    //    generating the decide event for those previous leaves.
    //
    // Since these cases (particularly case 2) are hard to account for explicitly, we just use a
    // persistent value in the database to remember how far we have successfully processed the event
    // stream.
    let last_processed_view: Option<i64> = tx
        .fetch_optional(query(
            "SELECT last_processed_view FROM event_stream WHERE id = 1 LIMIT 1",
        ))
        .await?
        .map(|row| row.get("last_processed_view"));
    let leaves = if let Some(v) = last_processed_view {
        let new_leaves = leaves.split_off(&((v as u64) + 1));
        if !leaves.is_empty() {
            tracing::debug!(
                v,
                remaining_leaves = new_leaves.len(),
                ?leaves,
                "excluding already-processed leaves from decide event"
            );
        }
        new_leaves
    } else {
        leaves
    };

    drop(tx);

    // Generate a decide event for each leaf, to be processed by the event consumer. We make a
    // separate event for each leaf because it is possible we have non-consecutive leaves in our
    // storage, which would not be valid as a single decide with a single leaf chain.
    for (view, (mut leaf, qc)) in leaves {
        // Include the VID share if available.
        let vid_share = vid_shares.remove(&view);
        if vid_share.is_none() {
            tracing::debug!(view, "VID share not available at decide");
        }

        // Fill in the full block payload using the DA proposals we had persisted.
        if let Some(proposal) = da_proposals.remove(&view) {
            let payload = Payload::from_bytes(&proposal.encoded_transactions, &proposal.metadata);
            leaf.fill_block_payload_unchecked(payload);
        } else if view == ViewNumber::genesis().u64() {
            // We don't get a DA proposal for the genesis view, but we know what the payload always
            // is.
            leaf.fill_block_payload_unchecked(Payload::empty().0);
        } else {
            tracing::debug!(view, "DA proposal not available at decide");
        }

        let leaf_info = LeafInfo {
            leaf,
            vid_share,

            // Note: the following fields are not used in Decide event processing, and
            // should be removed. For now, we just default them.
            state: Default::default(),
            delta: Default::default(),
        };
        tracing::debug!(?view, ?qc, ?leaf_info, "generating decide event");
        consumer
            .handle_event(&Event {
                view_number: ViewNumber::new(view),
                event: EventType::Decide {
                    leaf_chain: Arc::new(vec![leaf_info]),
                    qc: Arc::new(qc),
                    block_size: None,
                },
            })
            .await?;
    }

    let mut tx = storage.db.write().await?;
    // Now that we have definitely processed leaves up to `view`, we can update
    // `last_processed_view` so we don't process these leaves again. We may still fail at this
    // point, or shut down, and fail to complete this update. At worst this will lead to us sending
    // a duplicate decide event the next time we are called; this is fine as the event consumer is
    // required to be idempotent.
    tx.upsert(
        "event_stream",
        ["id", "last_processed_view"],
        ["id"],
        [(1i32, view.u64() as i64)],
    )
    .await?;

    tx.execute(query("DELETE FROM vid_share where view <= $1").bind(view.u64() as i64))
        .await?;

    tx.execute(query("DELETE FROM da_proposal where view <= $1").bind(view.u64() as i64))
        .await?;

    // Clean up leaves, but do not delete the most recent one (all leaves with a view number less than the given value).
    // This is necessary to ensure that, in case of a restart, we can resume from the last decided leaf.
    tx.execute(query("DELETE FROM anchor_leaf WHERE view < $1").bind(view.u64() as i64))
        .await?;

    // Clean up old proposals. These are not part of the decide event we generate for the consumer,
    // so we don't need to return them.
    tx.execute(query("DELETE FROM quorum_proposals where view <= $1").bind(view.u64() as i64))
        .await?;

    tx.commit().await
}

#[cfg(test)]
mod testing {
    use hotshot_query_service::data_source::storage::sql::testing::TmpDb;

    use super::{super::testing::TestablePersistence, *};

    #[async_trait]
    impl TestablePersistence for Persistence {
        type Storage = Arc<TmpDb>;

        async fn tmp_storage() -> Self::Storage {
            Arc::new(TmpDb::init().await)
        }

        async fn connect(db: &Self::Storage) -> Self {
            #[cfg(not(feature = "embedded-db"))]
            {
                let mut opt: Options = PostgresOptions {
                    port: Some(db.port()),
                    host: Some(db.host()),
                    user: Some("postgres".into()),
                    password: Some("password".into()),
                    ..Default::default()
                }
                .into();
                opt.create().await.unwrap()
            }

            #[cfg(feature = "embedded-db")]
            {
                let mut opt: Options = SqliteOptions { path: db.path() }.into();
                opt.create().await.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::{super::persistence_tests, Persistence};
    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_persistence_tests!(Persistence);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{persistence::testing::TestablePersistence, BLSPubKey, PubKey};
    use espresso_types::{NodeState, ValidatedState};
    use futures::stream::TryStreamExt;
    use hotshot_example_types::node_types::TestVersions;
    use hotshot_types::traits::signature_key::SignatureKey;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_quorum_proposals_leaf_hash_migration() {
        // Create some quorum proposals to test with.
        let leaf = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
        let privkey = BLSPubKey::generated_from_seed_indexed([0; 32], 1).1;
        let signature = PubKey::sign(&privkey, &[]).unwrap();
        let mut quorum_proposal = Proposal {
            data: QuorumProposal::<SeqTypes> {
                block_header: leaf.block_header().clone(),
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate::genesis::<TestVersions>(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            },
            signature,
            _pd: Default::default(),
        };

        let qp1 = quorum_proposal.clone();

        quorum_proposal.data.view_number = ViewNumber::new(1);
        let qp2 = quorum_proposal.clone();

        let qps = [qp1, qp2];

        // Create persistence and add the quorum proposals with NULL leaf hash.
        let db = Persistence::tmp_storage().await;
        let persistence = Persistence::connect(&db).await;
        let mut tx = persistence.db.write().await.unwrap();
        let params = qps
            .iter()
            .map(|qp| {
                (
                    qp.data.view_number.u64() as i64,
                    bincode::serialize(&qp).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        tx.upsert("quorum_proposals", ["view", "data"], ["view"], params)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        // Create a new persistence and ensure the commitments get populated.
        let persistence = Persistence::connect(&db).await;
        let mut tx = persistence.db.read().await.unwrap();
        let rows = tx
            .fetch("SELECT * FROM quorum_proposals ORDER BY view ASC")
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        assert_eq!(rows.len(), qps.len());
        for (row, qp) in rows.into_iter().zip(qps) {
            assert_eq!(row.get::<i64, _>("view"), qp.data.view_number.u64() as i64);
            assert_eq!(
                row.get::<Vec<u8>, _>("data"),
                bincode::serialize(&qp).unwrap()
            );
            assert_eq!(
                row.get::<String, _>("leaf_hash"),
                Committable::commit(&Leaf::from_quorum_proposal(&qp.data)).to_string()
            );
        }
    }
}
