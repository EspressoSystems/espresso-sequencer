use std::{collections::BTreeMap, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use anyhow::{bail, Context};
use async_trait::async_trait;
use clap::Parser;
use committable::Committable;
use derivative::Derivative;
use derive_more::derive::{From, Into};
use espresso_types::{
    parse_duration, parse_size, upgrade_commitment_map,
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
    BackoffParams, BlockMerkleTree, FeeMerkleTree, Leaf, Leaf2, NetworkConfig, Payload,
};
use futures::stream::StreamExt;
use hotshot::InitializerEpochInfo;
use hotshot_query_service::{
    availability::LeafQueryData,
    data_source::{
        storage::{
            pruning::PrunerCfg,
            sql::{
                include_migrations, query_as, syntax_helpers::MAX_FN, Config, Db, SqlStorage,
                Transaction, TransactionMode, Write,
            },
        },
        Transaction as _, VersionedDataSource,
    },
    fetching::{
        request::{LeafRequest, PayloadRequest, VidCommonRequest},
        Provider,
    },
    merklized_state::MerklizedState,
    VidCommon,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{
        vid_disperse::{ADVZDisperseShare, VidDisperseShare2},
        DaProposal, DaProposal2, EpochNumber, QuorumProposal, QuorumProposalWrapper, VidCommitment,
        VidDisperseShare,
    },
    drb::DrbResult,
    event::{Event, EventType, HotShotAction, LeafInfo},
    message::{convert_proposal, Proposal},
    simple_certificate::{
        NextEpochQuorumCertificate2, QuorumCertificate, QuorumCertificate2, UpgradeCertificate,
    },
    traits::{
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::ConsensusTime,
    },
    utils::View,
    vote::HasViewNumber,
};
use itertools::Itertools;
use sqlx::{query, Executor, Row};

use crate::{catchup::SqlStateCatchup, NodeType, SeqTypes, ViewNumber};

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Derivative)]
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

impl Default for PostgresOptions {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
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
    pub(crate) path: Option<PathBuf>,
}

pub fn build_sqlite_path(path: &str) -> anyhow::Result<PathBuf> {
    let sub_dir = PathBuf::from_str(path)?.join("sqlite");

    // if `sqlite` sub dir does not exist then create it
    if !sub_dir.exists() {
        std::fs::create_dir_all(&sub_dir)
            .with_context(|| format!("failed to create directory: {:?}", sub_dir))?;
    }

    Ok(sub_dir.join("database"))
}

/// Options for database-backed persistence, supporting both Postgres and SQLite.
#[derive(Parser, Clone, Derivative, From, Into)]
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
    #[clap(long, env = "ESPRESSO_SEQUENCER_DATABASE_PRUNE")]
    pub(crate) prune: bool,

    /// Pruning parameters.
    #[clap(flatten)]
    pub(crate) pruning: PruningOptions,

    /// Pruning parameters for ephemeral consensus storage.
    #[clap(flatten)]
    pub(crate) consensus_pruning: ConsensusPruningOptions,

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

    /// Turns on leaf only data storage
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_LIGHTWEIGHT",
        default_value_t = false,
        conflicts_with = "archive"
    )]
    pub(crate) lightweight: bool,

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

impl Default for Options {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
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

        if let Some(path) = opt.path {
            cfg = cfg.db_path(path);
        }

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
impl TryFrom<&Options> for Config {
    type Error = anyhow::Error;

    fn try_from(opt: &Options) -> Result<Self, Self::Error> {
        let mut cfg = match &opt.uri {
            Some(uri) => uri.parse()?,
            None => Self::default(),
        };

        if let Some(pool) = &opt.pool {
            cfg = cfg.pool(pool.clone());
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

            let pg_options = &opt.postgres_options;

            if let Some(host) = &pg_options.host {
                cfg = cfg.host(host.clone());
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

            if let Some(path) = &opt.sqlite_options.path {
                cfg = cfg.db_path(path.clone());
            }
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
#[derive(Parser, Clone, Copy, Debug)]
pub struct PruningOptions {
    /// Threshold for pruning, specified in bytes.
    /// If the disk usage surpasses this threshold, pruning is initiated for data older than the specified minimum retention period.
    /// Pruning continues until the disk usage drops below the MAX USAGE.
    #[clap(long, env = "ESPRESSO_SEQUENCER_PRUNER_PRUNING_THRESHOLD", value_parser = parse_size)]
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

        cfg = cfg.with_state_tables(vec![
            BlockMerkleTree::state_type().to_string(),
            FeeMerkleTree::state_type().to_string(),
        ]);

        cfg
    }
}

/// Pruning parameters for ephemeral consensus storage.
#[derive(Parser, Clone, Copy, Debug)]
pub struct ConsensusPruningOptions {
    /// Number of views to try to retain in consensus storage before data that hasn't been archived
    /// is garbage collected.
    ///
    /// The longer this is, the more certain that all data will eventually be archived, even if
    /// there are temporary problems with archive storage or partially missing data. This can be set
    /// very large, as most data is garbage collected as soon as it is finalized by consensus. This
    /// setting only applies to views which never get decided (ie forks in consensus) and views for
    /// which this node is partially offline. These should be exceptionally rare.
    ///
    /// Note that in extreme scenarios, data may be garbage collected even before TARGET_RETENTION
    /// views, if consensus storage exceeds TARGET_USAGE. For a hard lower bound on how long
    /// consensus data will be retained, see MINIMUM_RETENTION.
    ///
    /// The default of 302000 views equates to approximately 1 week (604800 seconds) at an average
    /// view time of 2s.
    #[clap(
        name = "TARGET_RETENTION",
        long = "consensus-storage-target-retention",
        env = "ESPRESSO_SEQUENCER_CONSENSUS_STORAGE_TARGET_RETENTION",
        default_value = "302000"
    )]
    target_retention: u64,

    /// Minimum number of views to try to retain in consensus storage before data that hasn't been
    /// archived is garbage collected.
    ///
    /// This bound allows data to be retained even if consensus storage occupies more than
    /// TARGET_USAGE. This can be used to ensure sufficient time to move consensus data to archival
    /// storage as necessary, even under extreme circumstances where otherwise garbage collection
    /// would kick in based on TARGET_RETENTION.
    ///
    /// The default of 130000 views equates to approximately 3 days (259200 seconds) at an average
    /// view time of 2s.
    #[clap(
        name = "MINIMUM_RETENTION",
        long = "consensus-storage-minimum-retention",
        env = "ESPRESSO_SEQUENCER_CONSENSUS_STORAGE_MINIMUM_RETENTION",
        default_value = "130000"
    )]
    minimum_retention: u64,

    /// Amount (in bytes) of data to retain in consensus storage before garbage collecting more
    /// aggressively.
    ///
    /// See also TARGET_RETENTION and MINIMUM_RETENTION.
    #[clap(
        name = "TARGET_USAGE",
        long = "consensus-storage-target-usage",
        env = "ESPRESSO_SEQUENCER_CONSENSUS_STORAGE_TARGET_USAGE",
        default_value = "1000000000"
    )]
    target_usage: u64,
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    fn set_view_retention(&mut self, view_retention: u64) {
        self.consensus_pruning.target_retention = view_retention;
        self.consensus_pruning.minimum_retention = view_retention;
    }

    async fn create(&mut self) -> anyhow::Result<Self::Persistence> {
        let store_undecided_state = self.store_undecided_state;
        let config = (&*self).try_into()?;
        let persistence = Persistence {
            store_undecided_state,
            db: SqlStorage::connect(config).await?,
            gc_opt: self.consensus_pruning,
        };
        persistence.migrate_quorum_proposal_leaf_hashes().await?;
        self.pool = Some(persistence.db.pool());
        Ok(persistence)
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(&self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
#[derive(Clone, Debug)]
pub struct Persistence {
    db: SqlStorage,
    store_undecided_state: bool,
    gc_opt: ConsensusPruningOptions,
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

    async fn generate_decide_events(&self, consumer: &impl EventConsumer) -> anyhow::Result<()> {
        let mut last_processed_view: Option<i64> = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT last_processed_view FROM event_stream WHERE id = 1 LIMIT 1")
            .await?
            .map(|row| row.get("last_processed_view"));
        loop {
            // In SQLite, overlapping read and write transactions can lead to database errors. To
            // avoid this:
            // - start a read transaction to query and collect all the necessary data.
            // - Commit (or implicitly drop) the read transaction once the data is fetched.
            // - use the collected data to generate a "decide" event for the consumer.
            // - begin a write transaction to delete the data and update the event stream.
            let mut tx = self.db.read().await?;

            // Collect a chain of consecutive leaves, starting from the first view after the last
            // decide. This will correspond to a decide event, and defines a range of views which
            // can be garbage collected. This may even include views for which there was no leaf,
            // for which we might still have artifacts like proposals that never finalized.
            let from_view = match last_processed_view {
                Some(v) => v + 1,
                None => 0,
            };

            let mut parent = None;
            let mut rows =
                query("SELECT leaf, qc FROM anchor_leaf2 WHERE view >= $1 ORDER BY view")
                    .bind(from_view)
                    .fetch(tx.as_mut());
            let mut leaves = vec![];
            let mut final_qc = None;
            while let Some(row) = rows.next().await {
                let row = match row {
                    Ok(row) => row,
                    Err(err) => {
                        // If there's an error getting a row, try generating an event with the rows
                        // we do have.
                        tracing::warn!("error loading row: {err:#}");
                        break;
                    },
                };

                let leaf_data: Vec<u8> = row.get("leaf");
                let leaf = bincode::deserialize::<Leaf2>(&leaf_data)?;
                let qc_data: Vec<u8> = row.get("qc");
                let qc = bincode::deserialize::<QuorumCertificate2<SeqTypes>>(&qc_data)?;
                let height = leaf.block_header().block_number();

                // Ensure we are only dealing with a consecutive chain of leaves. We don't want to
                // garbage collect any views for which we missed a leaf or decide event; at least
                // not right away, in case we need to recover that data later.
                if let Some(parent) = parent {
                    if height != parent + 1 {
                        tracing::debug!(
                            height,
                            parent,
                            "ending decide event at non-consecutive leaf"
                        );
                        break;
                    }
                }
                parent = Some(height);
                leaves.push(leaf);
                final_qc = Some(qc);
            }
            drop(rows);

            let Some(final_qc) = final_qc else {
                // End event processing when there are no more decided views.
                tracing::debug!(from_view, "no new leaves at decide");
                return Ok(());
            };

            // Find the range of views encompassed by this leaf chain. All data in this range can be
            // processed by the consumer and then deleted.
            let from_view = leaves[0].view_number();
            let to_view = leaves[leaves.len() - 1].view_number();

            // Collect VID shares for the decide event.
            let mut vid_shares = tx
                .fetch_all(
                    query("SELECT view, data FROM vid_share2 where view >= $1 AND view <= $2")
                        .bind(from_view.u64() as i64)
                        .bind(to_view.u64() as i64),
                )
                .await?
                .into_iter()
                .map(|row| {
                    let view: i64 = row.get("view");
                    let data: Vec<u8> = row.get("data");
                    let vid_proposal = bincode::deserialize::<
                        Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
                    >(&data)?;
                    Ok((view as u64, vid_proposal.data))
                })
                .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

            // Collect DA proposals for the decide event.
            let mut da_proposals = tx
                .fetch_all(
                    query("SELECT view, data FROM da_proposal2 where view >= $1 AND view <= $2")
                        .bind(from_view.u64() as i64)
                        .bind(to_view.u64() as i64),
                )
                .await?
                .into_iter()
                .map(|row| {
                    let view: i64 = row.get("view");
                    let data: Vec<u8> = row.get("data");
                    let da_proposal =
                        bincode::deserialize::<Proposal<SeqTypes, DaProposal2<SeqTypes>>>(&data)?;
                    Ok((view as u64, da_proposal.data))
                })
                .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

            drop(tx);

            // Collate all the information by view number and construct a chain of leaves.
            let leaf_chain = leaves
                .into_iter()
                // Go in reverse chronological order, as expected by Decide events.
                .rev()
                .map(|mut leaf| {
                    let view = leaf.view_number();

                    // Include the VID share if available.
                    let vid_share = vid_shares.remove(&view);
                    if vid_share.is_none() {
                        tracing::debug!(?view, "VID share not available at decide");
                    }

                    // Fill in the full block payload using the DA proposals we had persisted.
                    if let Some(proposal) = da_proposals.remove(&view) {
                        let payload =
                            Payload::from_bytes(&proposal.encoded_transactions, &proposal.metadata);
                        leaf.fill_block_payload_unchecked(payload);
                    } else if view == ViewNumber::genesis() {
                        // We don't get a DA proposal for the genesis view, but we know what the
                        // payload always is.
                        leaf.fill_block_payload_unchecked(Payload::empty().0);
                    } else {
                        tracing::debug!(?view, "DA proposal not available at decide");
                    }

                    LeafInfo {
                        leaf,
                        vid_share,
                        // Note: the following fields are not used in Decide event processing, and
                        // should be removed. For now, we just default them.
                        state: Default::default(),
                        delta: Default::default(),
                    }
                })
                .collect();

            // Generate decide event for the consumer.
            tracing::debug!(?to_view, ?final_qc, ?leaf_chain, "generating decide event");
            consumer
                .handle_event(&Event {
                    view_number: to_view,
                    event: EventType::Decide {
                        leaf_chain: Arc::new(leaf_chain),
                        qc: Arc::new(final_qc),
                        block_size: None,
                    },
                })
                .await?;

            let mut tx = self.db.write().await?;

            // Now that we have definitely processed leaves up to `to_view`, we can update
            // `last_processed_view` so we don't process these leaves again. We may still fail at
            // this point, or shut down, and fail to complete this update. At worst this will lead
            // to us sending a duplicate decide event the next time we are called; this is fine as
            // the event consumer is required to be idempotent.
            tx.upsert(
                "event_stream",
                ["id", "last_processed_view"],
                ["id"],
                [(1i32, to_view.u64() as i64)],
            )
            .await?;

            // Delete the data that has been fully processed.
            tx.execute(
                query("DELETE FROM vid_share2 where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;
            tx.execute(
                query("DELETE FROM da_proposal2 where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;
            tx.execute(
                query("DELETE FROM quorum_proposals2 where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;
            tx.execute(
                query("DELETE FROM quorum_certificate2 where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;

            // Clean up leaves, but do not delete the most recent one (all leaves with a view number
            // less than the given value). This is necessary to ensure that, in case of a restart,
            // we can resume from the last decided leaf.
            tx.execute(
                query("DELETE FROM anchor_leaf2 WHERE view >= $1 AND view < $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;

            tx.commit().await?;
            last_processed_view = Some(to_view.u64() as i64);
        }
    }

    #[tracing::instrument(skip(self))]
    async fn prune(&self, cur_view: ViewNumber) -> anyhow::Result<()> {
        let mut tx = self.db.write().await?;

        // Prune everything older than the target retention period.
        prune_to_view(
            &mut tx,
            cur_view.u64().saturating_sub(self.gc_opt.target_retention),
        )
        .await?;

        // Check our storage usage; if necessary we will prune more aggressively (up to the minimum
        // retention) to get below the target usage.
        #[cfg(feature = "embedded-db")]
        let usage_query = format!(
            "SELECT sum(pgsize) FROM dbstat WHERE name IN ({})",
            PRUNE_TABLES
                .iter()
                .map(|table| format!("'{table}'"))
                .join(",")
        );

        #[cfg(not(feature = "embedded-db"))]
        let usage_query = {
            let table_sizes = PRUNE_TABLES
                .iter()
                .map(|table| format!("pg_table_size('{table}')"))
                .join(" + ");
            format!("SELECT {table_sizes}")
        };

        let (usage,): (i64,) = query_as(&usage_query).fetch_one(tx.as_mut()).await?;
        tracing::debug!(usage, "consensus storage usage after pruning");

        if (usage as u64) > self.gc_opt.target_usage {
            tracing::warn!(
                usage,
                gc_opt = ?self.gc_opt,
                "consensus storage is running out of space, pruning to minimum retention"
            );
            prune_to_view(
                &mut tx,
                cur_view.u64().saturating_sub(self.gc_opt.minimum_retention),
            )
            .await?;
        }

        tx.commit().await
    }
}

const PRUNE_TABLES: &[&str] = &[
    "anchor_leaf2",
    "vid_share2",
    "da_proposal2",
    "quorum_proposals2",
    "quorum_certificate2",
];

async fn prune_to_view(tx: &mut Transaction<Write>, view: u64) -> anyhow::Result<()> {
    if view == 0 {
        // Nothing to prune, the entire chain is younger than the retention period.
        return Ok(());
    }
    tracing::debug!(view, "pruning consensus storage");

    for table in PRUNE_TABLES {
        let res = query(&format!("DELETE FROM {table} WHERE view < $1"))
            .bind(view as i64)
            .execute(tx.as_mut())
            .await
            .context(format!("pruning {table}"))?;
        if res.rows_affected() > 0 {
            tracing::info!(
                "garbage collected {} rows from {table}",
                res.rows_affected()
            );
        }
    }

    Ok(())
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
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, QuorumCertificate2<SeqTypes>)> + Send,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<()> {
        let values = leaf_chain
            .into_iter()
            .map(|(info, qc2)| {
                // The leaf may come with a large payload attached. We don't care about this payload
                // because we already store it separately, as part of the DA proposal. Storing it
                // here contributes to load on the DB for no reason, so we remove it before
                // serializing the leaf.
                let mut leaf = info.leaf.clone();
                leaf.unfill_block_payload();

                let view = qc2.view_number.u64() as i64;
                let leaf_bytes = bincode::serialize(&leaf)?;
                let qc_bytes = bincode::serialize(&qc2)?;
                Ok((view, leaf_bytes, qc_bytes))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        // First, append the new leaves. We do this in its own transaction because even if GC or the
        // event consumer later fails, there is no need to abort the storage of the leaves.
        let mut tx = self.db.write().await?;

        tx.upsert("anchor_leaf2", ["view", "leaf", "qc"], ["view"], values)
            .await?;
        tx.commit().await?;

        // Generate an event for the new leaves and, only if it succeeds, clean up data we no longer
        // need.
        if let Err(err) = self.generate_decide_events(consumer).await {
            // GC/event processing failure is not an error, since by this point we have at least
            // managed to persist the decided leaves successfully, and GC will just run again at the
            // next decide. Log an error but do not return it.
            tracing::warn!(?view, "event processing failed: {err:#}");
            return Ok(());
        }

        // Garbage collect data which was not included in any decide event, but which at this point
        // is old enough to just forget about.
        if let Err(err) = self.prune(view).await {
            tracing::warn!(?view, "pruning failed: {err:#}");
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
    ) -> anyhow::Result<Option<(Leaf2, QuorumCertificate2<SeqTypes>)>> {
        let Some(row) = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT leaf, qc FROM anchor_leaf2 ORDER BY view DESC LIMIT 1")
            .await?
        else {
            return Ok(None);
        };

        let leaf_bytes: Vec<u8> = row.get("leaf");
        let leaf2: Leaf2 = bincode::deserialize(&leaf_bytes)?;

        let qc_bytes: Vec<u8> = row.get("qc");
        let qc2: QuorumCertificate2<SeqTypes> = bincode::deserialize(&qc_bytes)?;

        Ok(Some((leaf2, qc2)))
    }

    async fn load_anchor_view(&self) -> anyhow::Result<ViewNumber> {
        let mut tx = self.db.read().await?;
        let (view,) = query_as::<(i64,)>("SELECT coalesce(max(view), 0) FROM anchor_leaf2")
            .fetch_one(tx.as_mut())
            .await?;
        Ok(ViewNumber::new(view as u64))
    }

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf2>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        let Some(row) = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT leaves, state FROM undecided_state2 WHERE id = 0")
            .await?
        else {
            return Ok(None);
        };

        let leaves_bytes: Vec<u8> = row.get("leaves");
        let leaves2: CommitmentMap<Leaf2> = bincode::deserialize(&leaves_bytes)?;

        let state_bytes: Vec<u8> = row.get("state");
        let state = bincode::deserialize(&state_bytes)?;

        Ok(Some((leaves2, state)))
    }

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal2<SeqTypes>>>> {
        let result = self
            .db
            .read()
            .await?
            .fetch_optional(
                query("SELECT data FROM da_proposal2 where view = $1").bind(view.u64() as i64),
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
                query("SELECT data FROM vid_share2 where view = $1").bind(view.u64() as i64),
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
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>>>
    {
        let rows = self
            .db
            .read()
            .await?
            .fetch_all("SELECT * FROM quorum_proposals2")
            .await?;

        Ok(BTreeMap::from_iter(
            rows.into_iter()
                .map(|row| {
                    let view: i64 = row.get("view");
                    let view_number: ViewNumber = ViewNumber::new(view.try_into()?);
                    let bytes: Vec<u8> = row.get("data");
                    let proposal = bincode::deserialize(&bytes)?;
                    Ok((view_number, proposal))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }

    async fn load_quorum_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>> {
        let mut tx = self.db.read().await?;
        let (data,) =
            query_as::<(Vec<u8>,)>("SELECT data FROM quorum_proposals2 WHERE view = $1 LIMIT 1")
                .bind(view.u64() as i64)
                .fetch_one(tx.as_mut())
                .await?;
        let proposal = bincode::deserialize(&data)?;

        Ok(proposal)
    }

    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, ADVZDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view = proposal.data.view_number.u64();
        let payload_hash = proposal.data.payload_commitment;
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "vid_share",
            ["view", "data", "payload_hash"],
            ["view"],
            [(view as i64, data_bytes, payload_hash.to_string())],
        )
        .await?;
        tx.commit().await
    }

    async fn append_vid2(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare2<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view = proposal.data.view_number.u64();
        let payload_hash = proposal.data.payload_commitment;
        let proposal: Proposal<SeqTypes, VidDisperseShare<SeqTypes>> =
            convert_proposal(proposal.clone());
        let data_bytes = bincode::serialize(&proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "vid_share2",
            ["view", "data", "payload_hash"],
            ["view"],
            [(view as i64, data_bytes, payload_hash.to_string())],
        )
        .await?;
        tx.commit().await
    }

    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
        vid_commit: VidCommitment,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "da_proposal",
            ["view", "data", "payload_hash"],
            ["view"],
            [(view as i64, data_bytes, vid_commit.to_string())],
        )
        .await?;
        tx.commit().await
    }

    async fn record_action(
        &self,
        view: ViewNumber,
        _epoch: Option<EpochNumber>,
        action: HotShotAction,
    ) -> anyhow::Result<()> {
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

    async fn append_quorum_proposal2(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.view_number().u64();

        let proposal_bytes = bincode::serialize(&proposal).context("serializing proposal")?;
        let leaf_hash = Committable::commit(&Leaf2::from_quorum_proposal(&proposal.data));
        let mut tx = self.db.write().await?;
        tx.upsert(
            "quorum_proposals2",
            ["view", "leaf_hash", "data"],
            ["view"],
            [(view_number as i64, leaf_hash.to_string(), proposal_bytes)],
        )
        .await?;

        // We also keep track of any QC we see in case we need it to recover our archival storage.
        let justify_qc = proposal.data.justify_qc();
        let justify_qc_bytes = bincode::serialize(&justify_qc).context("serializing QC")?;
        tx.upsert(
            "quorum_certificate2",
            ["view", "leaf_hash", "data"],
            ["view"],
            [(
                justify_qc.view_number.u64() as i64,
                justify_qc.data.leaf_commit.to_string(),
                &justify_qc_bytes,
            )],
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

    async fn migrate_anchor_leaf(&self) -> anyhow::Result<()> {
        let batch_size: i64 = 10000;
        let mut offset: i64 = 0;
        let mut tx = self.db.read().await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'anchor_leaf'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("decided leaves already migrated");
            return Ok(());
        }

        tracing::warn!("migrating decided leaves..");
        loop {
            let mut tx = self.db.read().await?;
            let rows =
                query("SELECT view, leaf, qc FROM anchor_leaf ORDER BY view LIMIT $1 OFFSET $2")
                    .bind(batch_size)
                    .bind(offset)
                    .fetch_all(tx.as_mut())
                    .await?;

            drop(tx);
            if rows.is_empty() {
                break;
            }
            let mut values = Vec::new();

            for row in rows.iter() {
                let leaf: Vec<u8> = row.try_get("leaf")?;
                let qc: Vec<u8> = row.try_get("qc")?;
                let leaf1: Leaf = bincode::deserialize(&leaf)?;
                let qc1: QuorumCertificate<SeqTypes> = bincode::deserialize(&qc)?;
                let view: i64 = row.try_get("view")?;

                let leaf2: Leaf2 = leaf1.into();
                let qc2: QuorumCertificate2<SeqTypes> = qc1.to_qc2();

                let leaf2_bytes = bincode::serialize(&leaf2)?;
                let qc2_bytes = bincode::serialize(&qc2)?;

                values.push((view, leaf2_bytes, qc2_bytes));
            }

            let mut query_builder: sqlx::QueryBuilder<Db> =
                sqlx::QueryBuilder::new("INSERT INTO anchor_leaf2 (view, leaf, qc) ");

            query_builder.push_values(values.into_iter(), |mut b, (view, leaf, qc)| {
                b.push_bind(view).push_bind(leaf).push_bind(qc);
            });

            let query = query_builder.build();

            let mut tx = self.db.write().await?;
            query.execute(tx.as_mut()).await?;
            tx.commit().await?;
            offset += batch_size;
            tracing::info!("anchor leaf migration progress: {} rows", offset);

            if rows.len() < batch_size as usize {
                break;
            }
        }

        tracing::warn!("migrated decided leaves");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("anchor_leaf".to_string(), true)],
        )
        .await?;
        tx.commit().await?;

        tracing::info!("updated epoch_migration table for anchor_leaf");

        Ok(())
    }

    async fn migrate_da_proposals(&self) -> anyhow::Result<()> {
        let batch_size: i64 = 10000;
        let mut offset: i64 = 0;
        let mut tx = self.db.read().await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'da_proposal'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("da proposals migration already done");
            return Ok(());
        }

        tracing::warn!("migrating da proposals..");

        loop {
            let mut tx = self.db.read().await?;
            let rows = query(
                "SELECT payload_hash, data FROM da_proposal ORDER BY view LIMIT $1 OFFSET $2",
            )
            .bind(batch_size)
            .bind(offset)
            .fetch_all(tx.as_mut())
            .await?;

            drop(tx);
            if rows.is_empty() {
                break;
            }
            let mut values = Vec::new();

            for row in rows.iter() {
                let data: Vec<u8> = row.try_get("data")?;
                let payload_hash: String = row.try_get("payload_hash")?;

                let da_proposal: DaProposal<SeqTypes> = bincode::deserialize(&data)?;
                let da_proposal2: DaProposal2<SeqTypes> = da_proposal.into();

                let view = da_proposal2.view_number.u64() as i64;
                let data = bincode::serialize(&da_proposal2)?;

                values.push((view, payload_hash, data));
            }

            let mut query_builder: sqlx::QueryBuilder<Db> =
                sqlx::QueryBuilder::new("INSERT INTO da_proposal2 (view, payload_hash, data) ");

            query_builder.push_values(values.into_iter(), |mut b, (view, payload_hash, data)| {
                b.push_bind(view).push_bind(payload_hash).push_bind(data);
            });

            let query = query_builder.build();

            let mut tx = self.db.write().await?;
            query.execute(tx.as_mut()).await?;

            tx.commit().await?;

            tracing::info!("DA proposals migration progress: {} rows", offset);
            offset += batch_size;

            if rows.len() < batch_size as usize {
                break;
            }
        }

        tracing::warn!("migrated da proposals");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("da_proposal".to_string(), true)],
        )
        .await?;
        tx.commit().await?;

        tracing::info!("updated epoch_migration table for da_proposal");

        Ok(())
    }

    async fn migrate_vid_shares(&self) -> anyhow::Result<()> {
        let batch_size: i64 = 10000;
        let mut offset: i64 = 0;
        let mut tx = self.db.read().await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'vid_share'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("vid_share migration already done");
            return Ok(());
        }

        tracing::warn!("migrating vid shares..");
        loop {
            let mut tx = self.db.read().await?;
            let rows =
                query("SELECT payload_hash, data FROM vid_share ORDER BY view LIMIT $1 OFFSET $2")
                    .bind(batch_size)
                    .bind(offset)
                    .fetch_all(tx.as_mut())
                    .await?;

            drop(tx);
            if rows.is_empty() {
                break;
            }
            let mut values = Vec::new();

            for row in rows.iter() {
                let data: Vec<u8> = row.try_get("data")?;
                let payload_hash: String = row.try_get("payload_hash")?;

                let vid_share: ADVZDisperseShare<SeqTypes> = bincode::deserialize(&data)?;
                let vid_share2: VidDisperseShare<SeqTypes> = vid_share.into();

                let view = vid_share2.view_number().u64() as i64;
                let data = bincode::serialize(&vid_share2)?;

                values.push((view, payload_hash, data));
            }

            let mut query_builder: sqlx::QueryBuilder<Db> =
                sqlx::QueryBuilder::new("INSERT INTO vid_share2 (view, payload_hash, data) ");

            query_builder.push_values(values.into_iter(), |mut b, (view, payload_hash, data)| {
                b.push_bind(view).push_bind(payload_hash).push_bind(data);
            });

            let query = query_builder.build();

            let mut tx = self.db.write().await?;
            query.execute(tx.as_mut()).await?;
            tx.commit().await?;
            tracing::info!("VID shares migration progress: {} rows", offset);
            offset += batch_size;

            if rows.len() < batch_size as usize {
                break;
            }
        }

        tracing::warn!("migrated vid shares");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("vid_share".to_string(), true)],
        )
        .await?;
        tx.commit().await?;

        tracing::info!("updated epoch_migration table for vid_share");

        Ok(())
    }

    async fn migrate_undecided_state(&self) -> anyhow::Result<()> {
        let mut tx = self.db.read().await?;

        let row = tx
            .fetch_optional("SELECT leaves, state FROM undecided_state WHERE id = 0")
            .await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'undecided_state'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("undecided state migration already done");

            return Ok(());
        }

        tracing::warn!("migrating undecided state..");

        if let Some(row) = row {
            let leaves_bytes: Vec<u8> = row.try_get("leaves")?;
            let leaves: CommitmentMap<Leaf> = bincode::deserialize(&leaves_bytes)?;

            let leaves2 = upgrade_commitment_map(leaves);
            let leaves2_bytes = bincode::serialize(&leaves2)?;
            let state_bytes: Vec<u8> = row.try_get("state")?;

            let mut tx = self.db.write().await?;
            tx.upsert(
                "undecided_state2",
                ["id", "leaves", "state"],
                ["id"],
                [(0_i32, leaves2_bytes, state_bytes)],
            )
            .await?;
            tx.commit().await?;
        };

        tracing::warn!("migrated undecided state");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("undecided_state".to_string(), true)],
        )
        .await?;
        tx.commit().await?;

        tracing::info!("updated epoch_migration table for undecided_state");

        Ok(())
    }

    async fn migrate_quorum_proposals(&self) -> anyhow::Result<()> {
        let batch_size: i64 = 10000;
        let mut offset: i64 = 0;
        let mut tx = self.db.read().await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'quorum_proposals'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("quorum proposals migration already done");
            return Ok(());
        }

        tracing::warn!("migrating quorum proposals..");

        loop {
            let mut tx = self.db.read().await?;
            let rows =
                query("SELECT view, leaf_hash, data FROM quorum_proposals ORDER BY view LIMIT $1 OFFSET $2")
                    .bind(batch_size)
                    .bind(offset)
                    .fetch_all(tx.as_mut())
                    .await?;

            drop(tx);

            if rows.is_empty() {
                break;
            }

            let mut values = Vec::new();

            for row in rows.iter() {
                let leaf_hash: String = row.try_get("leaf_hash")?;
                let data: Vec<u8> = row.try_get("data")?;

                let quorum_proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
                    bincode::deserialize(&data)?;
                let quorum_proposal2: Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>> =
                    convert_proposal(quorum_proposal);

                let view = quorum_proposal2.data.view_number().u64() as i64;
                let data = bincode::serialize(&quorum_proposal2)?;

                values.push((view, leaf_hash, data));
            }

            let mut query_builder: sqlx::QueryBuilder<Db> =
                sqlx::QueryBuilder::new("INSERT INTO quorum_proposals2 (view, leaf_hash, data) ");

            query_builder.push_values(values.into_iter(), |mut b, (view, leaf_hash, data)| {
                b.push_bind(view).push_bind(leaf_hash).push_bind(data);
            });

            let query = query_builder.build();

            let mut tx = self.db.write().await?;
            query.execute(tx.as_mut()).await?;
            tx.commit().await?;

            offset += batch_size;
            tracing::info!("quorum proposals migration progress: {} rows", offset);

            if rows.len() < batch_size as usize {
                break;
            }
        }

        tracing::warn!("migrated quorum proposals");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("quorum_proposals".to_string(), true)],
        )
        .await?;
        tx.commit().await?;

        tracing::info!("updated epoch_migration table for quorum_proposals");

        Ok(())
    }

    async fn migrate_quorum_certificates(&self) -> anyhow::Result<()> {
        let batch_size: i64 = 10000;
        let mut offset: i64 = 0;
        let mut tx = self.db.read().await?;

        let (is_completed,) = query_as::<(bool,)>(
            "SELECT completed from epoch_migration WHERE table_name = 'quorum_certificate'",
        )
        .fetch_one(tx.as_mut())
        .await?;

        if is_completed {
            tracing::info!("quorum certificates migration already done");
            return Ok(());
        }

        tracing::warn!("migrating quorum certificates..");
        loop {
            let mut tx = self.db.read().await?;
            let rows =
                query("SELECT view, leaf_hash, data FROM quorum_certificate ORDER BY view LIMIT $1 OFFSET $2")
                    .bind(batch_size)
                    .bind(offset)
                    .fetch_all(tx.as_mut())
                    .await?;

            drop(tx);
            if rows.is_empty() {
                break;
            }
            let mut values = Vec::new();

            for row in rows.iter() {
                let leaf_hash: String = row.try_get("leaf_hash")?;
                let data: Vec<u8> = row.try_get("data")?;

                let qc: QuorumCertificate<SeqTypes> = bincode::deserialize(&data)?;
                let qc2: QuorumCertificate2<SeqTypes> = qc.to_qc2();

                let view = qc2.view_number().u64() as i64;
                let data = bincode::serialize(&qc2)?;

                values.push((view, leaf_hash, data));
            }

            let mut query_builder: sqlx::QueryBuilder<Db> =
                sqlx::QueryBuilder::new("INSERT INTO quorum_certificate2 (view, leaf_hash, data) ");

            query_builder.push_values(values.into_iter(), |mut b, (view, leaf_hash, data)| {
                b.push_bind(view).push_bind(leaf_hash).push_bind(data);
            });

            let query = query_builder.build();

            let mut tx = self.db.write().await?;
            query.execute(tx.as_mut()).await?;
            tx.commit().await?;
            offset += batch_size;

            tracing::info!("Quorum certificates migration progress: {} rows", offset);

            if rows.len() < batch_size as usize {
                break;
            }
        }

        tracing::warn!("migrated quorum certificates");

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_migration",
            ["table_name", "completed"],
            ["table_name"],
            [("quorum_certificate".to_string(), true)],
        )
        .await?;
        tx.commit().await?;
        tracing::info!("updated epoch_migration table for quorum_certificate");

        Ok(())
    }

    async fn store_next_epoch_quorum_certificate(
        &self,
        high_qc: NextEpochQuorumCertificate2<SeqTypes>,
    ) -> anyhow::Result<()> {
        let qc2_bytes = bincode::serialize(&high_qc).context("serializing next epoch qc")?;
        let mut tx = self.db.write().await?;
        tx.upsert(
            "next_epoch_quorum_certificate",
            ["id", "data"],
            ["id"],
            [(true, qc2_bytes)],
        )
        .await?;
        tx.commit().await
    }

    async fn load_next_epoch_quorum_certificate(
        &self,
    ) -> anyhow::Result<Option<NextEpochQuorumCertificate2<SeqTypes>>> {
        let result = self
            .db
            .read()
            .await?
            .fetch_optional("SELECT * FROM next_epoch_quorum_certificate where id = true")
            .await?;

        result
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                anyhow::Result::<_>::Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn append_da2(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal2<SeqTypes>>,
        vid_commit: VidCommitment,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "da_proposal2",
            ["view", "data", "payload_hash"],
            ["view"],
            [(view as i64, data_bytes, vid_commit.to_string())],
        )
        .await?;
        tx.commit().await
    }

    async fn update_undecided_state2(
        &self,
        leaves: CommitmentMap<Leaf2>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        if !self.store_undecided_state {
            return Ok(());
        }

        let leaves_bytes = bincode::serialize(&leaves).context("serializing leaves")?;
        let state_bytes = bincode::serialize(&state).context("serializing state")?;

        let mut tx = self.db.write().await?;
        tx.upsert(
            "undecided_state2",
            ["id", "leaves", "state"],
            ["id"],
            [(0_i32, leaves_bytes, state_bytes)],
        )
        .await?;
        tx.commit().await
    }

    async fn add_drb_result(
        &self,
        epoch: EpochNumber,
        drb_result: DrbResult,
    ) -> anyhow::Result<()> {
        let drb_result_vec = Vec::from(drb_result);
        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_drb_and_root",
            ["epoch", "drb_result"],
            ["epoch"],
            [(epoch.u64() as i64, drb_result_vec)],
        )
        .await?;
        tx.commit().await
    }

    async fn add_epoch_root(
        &self,
        epoch: EpochNumber,
        block_header: <SeqTypes as NodeType>::BlockHeader,
    ) -> anyhow::Result<()> {
        let block_header_bytes =
            bincode::serialize(&block_header).context("serializing block header")?;

        let mut tx = self.db.write().await?;
        tx.upsert(
            "epoch_drb_and_root",
            ["epoch", "block_header"],
            ["epoch"],
            [(epoch.u64() as i64, block_header_bytes)],
        )
        .await?;
        tx.commit().await
    }

    async fn load_start_epoch_info(&self) -> anyhow::Result<Vec<InitializerEpochInfo<SeqTypes>>> {
        let rows = self
            .db
            .read()
            .await?
            .fetch_all("SELECT * from epoch_drb_and_root ORDER BY epoch ASC")
            .await?;

        rows.into_iter()
            .map(|row| {
                let epoch: i64 = row.get("epoch");
                let drb_result: Option<Vec<u8>> = row.get("drb_result");
                let block_header: Option<Vec<u8>> = row.get("block_header");
                if let Some(drb_result) = drb_result {
                    let drb_result_array = drb_result
                        .try_into()
                        .or_else(|_| bail!("invalid drb result"))?;
                    let block_header: Option<<SeqTypes as NodeType>::BlockHeader> = block_header
                        .map(|data| bincode::deserialize(&data))
                        .transpose()?;
                    Ok(Some(InitializerEpochInfo::<SeqTypes> {
                        epoch: <SeqTypes as NodeType>::Epoch::new(epoch as u64),
                        drb_result: drb_result_array,
                        block_header,
                    }))
                } else {
                    // Right now we skip the epoch_drb_and_root row if there is no drb result.
                    // This seems reasonable based on the expected order of events, but please double check!
                    Ok(None)
                }
            })
            .filter_map(|e| match e {
                Err(v) => Some(Err(v)),
                Ok(Some(v)) => Some(Ok(v)),
                Ok(None) => None,
            })
            .collect()
    }
}

#[async_trait]
impl Provider<SeqTypes, VidCommonRequest> for Persistence {
    #[tracing::instrument(skip(self))]
    async fn fetch(&self, req: VidCommonRequest) -> Option<VidCommon> {
        let mut tx = match self.db.read().await {
            Ok(tx) => tx,
            Err(err) => {
                tracing::warn!("could not open transaction: {err:#}");
                return None;
            },
        };

        let bytes = match query_as::<(Vec<u8>,)>(
            "SELECT data FROM vid_share2 WHERE payload_hash = $1 LIMIT 1",
        )
        .bind(req.0.to_string())
        .fetch_optional(tx.as_mut())
        .await
        {
            Ok(Some((bytes,))) => bytes,
            Ok(None) => return None,
            Err(err) => {
                tracing::error!("error loading VID share: {err:#}");
                return None;
            },
        };

        let share: Proposal<SeqTypes, VidDisperseShare<SeqTypes>> =
            match bincode::deserialize(&bytes) {
                Ok(share) => share,
                Err(err) => {
                    tracing::warn!("error decoding VID share: {err:#}");
                    return None;
                },
            };

        match share.data {
            VidDisperseShare::V0(vid) => Some(VidCommon::V0(vid.common)),
            VidDisperseShare::V1(vid) => Some(VidCommon::V1(vid.common)),
        }
    }
}

#[async_trait]
impl Provider<SeqTypes, PayloadRequest> for Persistence {
    #[tracing::instrument(skip(self))]
    async fn fetch(&self, req: PayloadRequest) -> Option<Payload> {
        let mut tx = match self.db.read().await {
            Ok(tx) => tx,
            Err(err) => {
                tracing::warn!("could not open transaction: {err:#}");
                return None;
            },
        };

        let bytes = match query_as::<(Vec<u8>,)>(
            "SELECT data FROM da_proposal2 WHERE payload_hash = $1 LIMIT 1",
        )
        .bind(req.0.to_string())
        .fetch_optional(tx.as_mut())
        .await
        {
            Ok(Some((bytes,))) => bytes,
            Ok(None) => return None,
            Err(err) => {
                tracing::warn!("error loading DA proposal: {err:#}");
                return None;
            },
        };

        let proposal: Proposal<SeqTypes, DaProposal2<SeqTypes>> = match bincode::deserialize(&bytes)
        {
            Ok(proposal) => proposal,
            Err(err) => {
                tracing::error!("error decoding DA proposal: {err:#}");
                return None;
            },
        };

        Some(Payload::from_bytes(
            &proposal.data.encoded_transactions,
            &proposal.data.metadata,
        ))
    }
}

#[async_trait]
impl Provider<SeqTypes, LeafRequest<SeqTypes>> for Persistence {
    #[tracing::instrument(skip(self))]
    async fn fetch(&self, req: LeafRequest<SeqTypes>) -> Option<LeafQueryData<SeqTypes>> {
        let mut tx = match self.db.read().await {
            Ok(tx) => tx,
            Err(err) => {
                tracing::warn!("could not open transaction: {err:#}");
                return None;
            },
        };

        let (leaf, qc) = match fetch_leaf_from_proposals(&mut tx, req).await {
            Ok(res) => res?,
            Err(err) => {
                tracing::info!("requested leaf not found in undecided proposals: {err:#}");
                return None;
            },
        };

        match LeafQueryData::new(leaf, qc) {
            Ok(leaf) => Some(leaf),
            Err(err) => {
                tracing::warn!("fetched invalid leaf: {err:#}");
                None
            },
        }
    }
}

async fn fetch_leaf_from_proposals<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    req: LeafRequest<SeqTypes>,
) -> anyhow::Result<Option<(Leaf2, QuorumCertificate2<SeqTypes>)>> {
    // Look for a quorum proposal corresponding to this leaf.
    let Some((proposal_bytes,)) =
        query_as::<(Vec<u8>,)>("SELECT data FROM quorum_proposals2 WHERE leaf_hash = $1 LIMIT 1")
            .bind(req.expected_leaf.to_string())
            .fetch_optional(tx.as_mut())
            .await
            .context("fetching proposal")?
    else {
        return Ok(None);
    };

    // Look for a QC corresponding to this leaf.
    let Some((qc_bytes,)) =
        query_as::<(Vec<u8>,)>("SELECT data FROM quorum_certificate2 WHERE leaf_hash = $1 LIMIT 1")
            .bind(req.expected_leaf.to_string())
            .fetch_optional(tx.as_mut())
            .await
            .context("fetching QC")?
    else {
        return Ok(None);
    };

    let proposal: Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>> =
        bincode::deserialize(&proposal_bytes).context("deserializing quorum proposal")?;
    let qc: QuorumCertificate2<SeqTypes> =
        bincode::deserialize(&qc_bytes).context("deserializing quorum certificate")?;

    let leaf = Leaf2::from_quorum_proposal(&proposal.data);
    Ok(Some((leaf, qc)))
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

        #[allow(refining_impl_trait)]
        fn options(db: &Self::Storage) -> Options {
            #[cfg(not(feature = "embedded-db"))]
            {
                PostgresOptions {
                    port: Some(db.port()),
                    host: Some(db.host()),
                    user: Some("postgres".into()),
                    password: Some("password".into()),
                    ..Default::default()
                }
                .into()
            }

            #[cfg(feature = "embedded-db")]
            {
                SqliteOptions {
                    path: Some(db.path()),
                }
                .into()
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

    use committable::{Commitment, CommitmentBoundsArkless};
    use espresso_types::{traits::NullEventConsumer, Header, Leaf, NodeState, ValidatedState};
    use futures::stream::TryStreamExt;
    use hotshot_example_types::node_types::TestVersions;
    use hotshot_types::{
        data::{
            ns_table::parse_ns_table, vid_commitment, vid_disperse::VidDisperseShare2, EpochNumber,
            QuorumProposal2,
        },
        message::convert_proposal,
        simple_certificate::QuorumCertificate,
        simple_vote::QuorumData,
        traits::{
            block_contents::BlockHeader, node_implementation::Versions,
            signature_key::SignatureKey, EncodeBytes,
        },
        vid::{
            advz::advz_scheme,
            avidm::{init_avidm_param, AvidMScheme},
        },
    };
    use jf_vid::VidScheme;
    use sequencer_utils::test_utils::setup_test;
    use vbs::version::StaticVersionType;

    use super::*;
    use crate::{persistence::testing::TestablePersistence, BLSPubKey, PubKey};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_quorum_proposals_leaf_hash_migration() {
        setup_test();

        // Create some quorum proposals to test with.
        let leaf: Leaf2 =
            Leaf::genesis::<TestVersions>(&ValidatedState::default(), &NodeState::mock())
                .await
                .into();
        let privkey = BLSPubKey::generated_from_seed_indexed([0; 32], 1).1;
        let signature = PubKey::sign(&privkey, &[]).unwrap();
        let mut quorum_proposal = Proposal {
            data: QuorumProposal2::<SeqTypes> {
                epoch: None,
                block_header: leaf.block_header().clone(),
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate::genesis::<TestVersions>(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await
                .to_qc2(),
                upgrade_certificate: None,
                view_change_evidence: None,
                next_drb_result: None,
                next_epoch_justify_qc: None,
            },
            signature,
            _pd: Default::default(),
        };

        let qp1: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
            convert_proposal(quorum_proposal.clone());

        quorum_proposal.data.view_number = ViewNumber::new(1);

        let qp2: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
            convert_proposal(quorum_proposal.clone());
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_fetching_providers() {
        setup_test();

        let tmp = Persistence::tmp_storage().await;
        let storage = Persistence::connect(&tmp).await;

        // Mock up some data.
        let leaf =
            Leaf2::genesis::<TestVersions>(&ValidatedState::default(), &NodeState::mock()).await;
        let leaf_payload = leaf.block_payload().unwrap();
        let leaf_payload_bytes_arc = leaf_payload.encode();

        let avidm_param = init_avidm_param(2).unwrap();
        let weights = vec![1u32; 2];

        let ns_table = parse_ns_table(leaf_payload.byte_len().as_usize(), &leaf_payload.encode());
        let (payload_commitment, shares) =
            AvidMScheme::ns_disperse(&avidm_param, &weights, &leaf_payload_bytes_arc, ns_table)
                .unwrap();
        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let vid_share = VidDisperseShare2::<SeqTypes> {
            view_number: ViewNumber::new(0),
            payload_commitment,
            share: shares[0].clone(),
            recipient_key: pubkey,
            epoch: None,
            target_epoch: None,
            common: avidm_param.clone(),
        }
        .to_proposal(&privkey)
        .unwrap()
        .clone();

        let quorum_proposal = QuorumProposalWrapper::<SeqTypes> {
            proposal: QuorumProposal2::<SeqTypes> {
                block_header: leaf.block_header().clone(),
                view_number: leaf.view_number(),
                justify_qc: leaf.justify_qc(),
                upgrade_certificate: None,
                view_change_evidence: None,
                next_drb_result: None,
                next_epoch_justify_qc: None,
                epoch: None,
            },
        };
        let quorum_proposal_signature =
            BLSPubKey::sign(&privkey, &bincode::serialize(&quorum_proposal).unwrap())
                .expect("Failed to sign quorum proposal");
        let quorum_proposal = Proposal {
            data: quorum_proposal,
            signature: quorum_proposal_signature,
            _pd: Default::default(),
        };

        let block_payload_signature = BLSPubKey::sign(&privkey, &leaf_payload_bytes_arc)
            .expect("Failed to sign block payload");
        let da_proposal = Proposal {
            data: DaProposal2::<SeqTypes> {
                encoded_transactions: leaf_payload_bytes_arc,
                metadata: leaf_payload.ns_table().clone(),
                view_number: ViewNumber::new(0),
                epoch: None,
            },
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        let mut next_quorum_proposal = quorum_proposal.clone();
        next_quorum_proposal.data.proposal.view_number += 1;
        next_quorum_proposal.data.proposal.justify_qc.view_number += 1;
        next_quorum_proposal
            .data
            .proposal
            .justify_qc
            .data
            .leaf_commit = Committable::commit(&leaf.clone());
        let qc = next_quorum_proposal.data.justify_qc();

        // Add to database.
        storage
            .append_da2(&da_proposal, VidCommitment::V1(payload_commitment))
            .await
            .unwrap();
        storage
            .append_vid2(&convert_proposal(vid_share.clone()))
            .await
            .unwrap();
        storage
            .append_quorum_proposal2(&quorum_proposal)
            .await
            .unwrap();

        // Add an extra quorum proposal so we have a QC pointing back at `leaf`.
        storage
            .append_quorum_proposal2(&next_quorum_proposal)
            .await
            .unwrap();

        // Fetch it as if we were rebuilding an archive.
        assert_eq!(
            Some(VidCommon::V1(avidm_param)),
            storage
                .fetch(VidCommonRequest(VidCommitment::V1(
                    vid_share.data.payload_commitment
                )))
                .await
        );
        assert_eq!(
            leaf_payload,
            storage
                .fetch(PayloadRequest(VidCommitment::V1(
                    vid_share.data.payload_commitment
                )))
                .await
                .unwrap()
        );
        assert_eq!(
            LeafQueryData::new(leaf.clone(), qc.clone()).unwrap(),
            storage
                .fetch(LeafRequest::new(
                    leaf.block_header().block_number(),
                    Committable::commit(&leaf),
                    qc.clone().commit()
                ))
                .await
                .unwrap()
        );
    }

    /// Test conditions that trigger pruning.
    ///
    /// This is a configurable test that can be used to test different configurations of GC,
    /// `pruning_opt`. The test populates the database with some data for view 1, asserts that it is
    /// retained for view 2, and then asserts that it is pruned by view 3. There are various
    /// different configurations that can achieve this behavior, such that the data is retained and
    /// then pruned due to different logic and code paths.
    async fn test_pruning_helper(pruning_opt: ConsensusPruningOptions) {
        setup_test();

        let tmp = Persistence::tmp_storage().await;
        let mut opt = Persistence::options(&tmp);
        opt.consensus_pruning = pruning_opt;
        let storage = opt.create().await.unwrap();

        let data_view = ViewNumber::new(1);

        // Populate some data.
        let leaf =
            Leaf2::genesis::<TestVersions>(&ValidatedState::default(), &NodeState::mock()).await;
        let leaf_payload = leaf.block_payload().unwrap();
        let leaf_payload_bytes_arc = leaf_payload.encode();

        let avidm_param = init_avidm_param(2).unwrap();
        let weights = vec![1u32; 2];

        let ns_table = parse_ns_table(leaf_payload.byte_len().as_usize(), &leaf_payload.encode());
        let (payload_commitment, shares) =
            AvidMScheme::ns_disperse(&avidm_param, &weights, &leaf_payload_bytes_arc, ns_table)
                .unwrap();

        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let vid = VidDisperseShare2::<SeqTypes> {
            view_number: data_view,
            payload_commitment,
            share: shares[0].clone(),
            recipient_key: pubkey,
            epoch: None,
            target_epoch: None,
            common: avidm_param,
        }
        .to_proposal(&privkey)
        .unwrap()
        .clone();
        let quorum_proposal = QuorumProposalWrapper::<SeqTypes> {
            proposal: QuorumProposal2::<SeqTypes> {
                epoch: None,
                block_header: leaf.block_header().clone(),
                view_number: data_view,
                justify_qc: QuorumCertificate2::genesis::<TestVersions>(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await,
                upgrade_certificate: None,
                view_change_evidence: None,
                next_drb_result: None,
                next_epoch_justify_qc: None,
            },
        };
        let quorum_proposal_signature =
            BLSPubKey::sign(&privkey, &bincode::serialize(&quorum_proposal).unwrap())
                .expect("Failed to sign quorum proposal");
        let quorum_proposal = Proposal {
            data: quorum_proposal,
            signature: quorum_proposal_signature,
            _pd: Default::default(),
        };

        let block_payload_signature = BLSPubKey::sign(&privkey, &leaf_payload_bytes_arc)
            .expect("Failed to sign block payload");
        let da_proposal = Proposal {
            data: DaProposal2::<SeqTypes> {
                encoded_transactions: leaf_payload_bytes_arc.clone(),
                metadata: leaf_payload.ns_table().clone(),
                view_number: data_view,
                epoch: Some(EpochNumber::new(0)),
            },
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        tracing::info!(?vid, ?da_proposal, ?quorum_proposal, "append data");
        storage.append_vid2(&vid).await.unwrap();
        storage
            .append_da2(&da_proposal, VidCommitment::V1(payload_commitment))
            .await
            .unwrap();
        storage
            .append_quorum_proposal2(&quorum_proposal)
            .await
            .unwrap();

        // The first decide doesn't trigger any garbage collection, even though our usage exceeds
        // the target, because of the minimum retention.
        tracing::info!("decide view 1");
        storage
            .append_decided_leaves(data_view + 1, [], &NullEventConsumer)
            .await
            .unwrap();
        assert_eq!(
            storage.load_vid_share(data_view).await.unwrap().unwrap(),
            convert_proposal(vid)
        );
        assert_eq!(
            storage.load_da_proposal(data_view).await.unwrap().unwrap(),
            da_proposal
        );
        assert_eq!(
            storage.load_quorum_proposal(data_view).await.unwrap(),
            quorum_proposal
        );

        // After another view, our data is beyond the minimum retention (though not the target
        // retention) so it gets pruned.
        tracing::info!("decide view 2");
        storage
            .append_decided_leaves(data_view + 2, [], &NullEventConsumer)
            .await
            .unwrap();
        assert!(storage.load_vid_share(data_view).await.unwrap().is_none(),);
        assert!(storage.load_da_proposal(data_view).await.unwrap().is_none());
        storage.load_quorum_proposal(data_view).await.unwrap_err();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_pruning_minimum_retention() {
        test_pruning_helper(ConsensusPruningOptions {
            // Use a very low target usage, to show that we still retain data up to the minimum
            // retention even when usage is above target.
            target_usage: 0,
            minimum_retention: 1,
            // Use a very high target retention, so that pruning is only triggered by the minimum
            // retention.
            target_retention: u64::MAX,
        })
        .await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_pruning_target_retention() {
        test_pruning_helper(ConsensusPruningOptions {
            target_retention: 1,
            // Use a very low minimum retention, so that data is only kept around due to the target
            // retention.
            minimum_retention: 0,
            // Use a very high target usage, so that pruning is only triggered by the target
            // retention.
            target_usage: u64::MAX,
        })
        .await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_consensus_migration() {
        setup_test();

        let tmp = Persistence::tmp_storage().await;
        let mut opt = Persistence::options(&tmp);

        let storage = opt.create().await.unwrap();

        let rows = 300;

        for i in 0..rows {
            let view = ViewNumber::new(i);
            let validated_state = ValidatedState::default();
            let instance_state = NodeState::default();

            let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], i);
            let (payload, metadata) =
                Payload::from_transactions([], &validated_state, &instance_state)
                    .await
                    .unwrap();
            let builder_commitment = payload.builder_commitment(&metadata);
            let payload_bytes = payload.encode();

            let payload_commitment = vid_commitment::<TestVersions>(
                &payload_bytes,
                &metadata.encode(),
                4,
                <TestVersions as Versions>::Base::VERSION,
            );

            let block_header = Header::genesis(
                &instance_state,
                payload_commitment,
                builder_commitment,
                metadata,
            );

            let null_quorum_data = QuorumData {
                leaf_commit: Commitment::<Leaf>::default_commitment_no_preimage(),
            };

            let justify_qc = QuorumCertificate::new(
                null_quorum_data.clone(),
                null_quorum_data.commit(),
                view,
                None,
                std::marker::PhantomData,
            );

            let quorum_proposal = QuorumProposal {
                block_header,
                view_number: view,
                justify_qc: justify_qc.clone(),
                upgrade_certificate: None,
                proposal_certificate: None,
            };

            let quorum_proposal_signature =
                BLSPubKey::sign(&privkey, &bincode::serialize(&quorum_proposal).unwrap())
                    .expect("Failed to sign quorum proposal");

            let proposal = Proposal {
                data: quorum_proposal.clone(),
                signature: quorum_proposal_signature,
                _pd: std::marker::PhantomData,
            };

            let proposal_bytes = bincode::serialize(&proposal)
                .context("serializing proposal")
                .unwrap();

            let mut leaf = Leaf::from_quorum_proposal(&quorum_proposal);
            leaf.fill_block_payload::<TestVersions>(
                payload,
                4,
                <TestVersions as Versions>::Base::VERSION,
            )
            .unwrap();

            let mut tx = storage.db.write().await.unwrap();

            let qc_bytes = bincode::serialize(&justify_qc).unwrap();
            let leaf_bytes = bincode::serialize(&leaf).unwrap();

            tx.upsert(
                "anchor_leaf",
                ["view", "leaf", "qc"],
                ["view"],
                [(i as i64, leaf_bytes, qc_bytes)],
            )
            .await
            .unwrap();
            tx.commit().await.unwrap();

            let disperse = advz_scheme(4).disperse(payload_bytes.clone()).unwrap();

            let vid = ADVZDisperseShare::<SeqTypes> {
                view_number: ViewNumber::new(i),
                payload_commitment: Default::default(),
                share: disperse.shares[0].clone(),
                common: disperse.common,
                recipient_key: pubkey,
            };

            let (payload, metadata) =
                Payload::from_transactions([], &ValidatedState::default(), &NodeState::default())
                    .await
                    .unwrap();

            let da = DaProposal::<SeqTypes> {
                encoded_transactions: payload.encode(),
                metadata,
                view_number: ViewNumber::new(i),
            };

            let block_payload_signature =
                BLSPubKey::sign(&privkey, &payload_bytes).expect("Failed to sign block payload");

            let da_proposal = Proposal {
                data: da,
                signature: block_payload_signature,
                _pd: Default::default(),
            };

            storage
                .append_vid(&vid.to_proposal(&privkey).unwrap())
                .await
                .unwrap();
            storage
                .append_da(&da_proposal, VidCommitment::V0(disperse.commit))
                .await
                .unwrap();

            let leaf_hash = Committable::commit(&leaf);
            let mut tx = storage.db.write().await.expect("failed to start write tx");
            tx.upsert(
                "quorum_proposals",
                ["view", "leaf_hash", "data"],
                ["view"],
                [(i as i64, leaf_hash.to_string(), proposal_bytes)],
            )
            .await
            .expect("failed to upsert quorum proposal");

            let justify_qc = &proposal.data.justify_qc;
            let justify_qc_bytes = bincode::serialize(&justify_qc)
                .context("serializing QC")
                .unwrap();
            tx.upsert(
                "quorum_certificate",
                ["view", "leaf_hash", "data"],
                ["view"],
                [(
                    justify_qc.view_number.u64() as i64,
                    justify_qc.data.leaf_commit.to_string(),
                    &justify_qc_bytes,
                )],
            )
            .await
            .expect("failed to upsert qc");

            tx.commit().await.expect("failed to commit");
        }

        storage.migrate_consensus().await.unwrap();

        let mut tx = storage.db.read().await.unwrap();
        let (anchor_leaf2_count,) = query_as::<(i64,)>("SELECT COUNT(*) from anchor_leaf2")
            .fetch_one(tx.as_mut())
            .await
            .unwrap();
        assert_eq!(
            anchor_leaf2_count, rows as i64,
            "anchor leaf count does not match rows",
        );

        let (da_proposal_count,) = query_as::<(i64,)>("SELECT COUNT(*) from da_proposal2")
            .fetch_one(tx.as_mut())
            .await
            .unwrap();
        assert_eq!(
            da_proposal_count, rows as i64,
            "da proposal count does not match rows",
        );

        let (vid_share_count,) = query_as::<(i64,)>("SELECT COUNT(*) from vid_share2")
            .fetch_one(tx.as_mut())
            .await
            .unwrap();
        assert_eq!(
            vid_share_count, rows as i64,
            "vid share count does not match rows"
        );

        let (quorum_proposals_count,) =
            query_as::<(i64,)>("SELECT COUNT(*) from quorum_proposals2")
                .fetch_one(tx.as_mut())
                .await
                .unwrap();
        assert_eq!(
            quorum_proposals_count, rows as i64,
            "quorum proposals count does not match rows",
        );

        let (quorum_certificates_count,) =
            query_as::<(i64,)>("SELECT COUNT(*) from quorum_certificate2")
                .fetch_one(tx.as_mut())
                .await
                .unwrap();
        assert_eq!(
            quorum_certificates_count, rows as i64,
            "quorum certificates count does not match rows",
        );
    }
}
