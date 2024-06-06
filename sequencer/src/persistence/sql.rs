use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{
    catchup::{SqlStateCatchup, StateCatchup},
    options::parse_duration,
    Leaf, SeqTypes, ViewNumber,
};
use anyhow::Context;
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use clap::Parser;
use derivative::Derivative;
use futures::future::{BoxFuture, FutureExt};
use hotshot_query_service::data_source::{
    storage::{
        pruning::PrunerCfg,
        sql::{include_migrations, postgres::types::ToSql, Config, Query, SqlStorage, Transaction},
    },
    VersionedDataSource,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::node_implementation::ConsensusTime,
    utils::View,
    vote::HasViewNumber,
};
use std::{collections::BTreeMap, time::Duration};

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Derivative, Default)]
#[derivative(Debug)]
pub struct Options {
    /// Postgres URI.
    ///
    /// This is a shorthand for setting a number of other options all at once. The URI has the
    /// following format ([brackets] indicate optional segments):
    ///
    ///   postgres[ql]://[username[:password]@][host[:port],]/database[?parameter_list]
    ///
    /// Options set explicitly via other env vars or flags will take precedence, so you can use this
    /// URI to set a baseline and then use other parameters to override or add configuration. In
    /// addition, there are some parameters which cannot be set via the URI, such as TLS.
    // Hide from debug output since may contain sensitive data.
    #[derivative(Debug = "ignore")]
    pub(crate) uri: Option<String>,

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
}

impl TryFrom<Options> for Config {
    type Error = anyhow::Error;

    fn try_from(opt: Options) -> Result<Self, Self::Error> {
        let mut cfg = match opt.uri {
            Some(uri) => uri.parse()?,
            None => Self::default(),
        };
        cfg = cfg.migrations(include_migrations!("$CARGO_MANIFEST_DIR/api/migrations"));

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

        if opt.prune {
            cfg = cfg.pruner_cfg(PrunerCfg::from(opt.pruning))?;
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

    async fn create(self) -> anyhow::Result<Persistence> {
        Ok(Persistence {
            store_undecided_state: self.store_undecided_state,
            db: SqlStorage::connect(self.try_into()?).await?,
        })
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
pub struct Persistence {
    db: SqlStorage,
    store_undecided_state: bool,
}

async fn transaction(
    persistence: &mut Persistence,
    f: impl FnOnce(Transaction) -> BoxFuture<anyhow::Result<()>>,
) -> anyhow::Result<()> {
    let tx = persistence.db.transaction().await?;
    match f(tx).await {
        Ok(_) => {
            persistence.db.commit().await?;
            Ok(())
        }
        Err(err) => {
            tracing::warn!("transaction failed, reverting: {err:#}");
            persistence.db.revert().await;
            Err(err)
        }
    }
}

#[async_trait]
impl SequencerPersistence for Persistence {
    fn into_catchup_provider(self) -> anyhow::Result<Arc<dyn StateCatchup>> {
        Ok(Arc::new(SqlStateCatchup::from(Arc::new(RwLock::new(
            self.db,
        )))))
    }

    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        tracing::info!("loading config from Postgres");

        // Select the most recent config (although there should only be one).
        let Some(row) = self
            .db
            .query_opt_static("SELECT config FROM network_config ORDER BY id DESC LIMIT 1")
            .await?
        else {
            tracing::info!("config not found");
            return Ok(None);
        };
        let config = row.try_get("config")?;
        Ok(serde_json::from_value(config)?)
    }

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        tracing::info!("saving config to Postgres");
        let json = serde_json::to_value(cfg)?;

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(
                    "INSERT INTO network_config (config) VALUES ($1)",
                    [&json],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn collect_garbage(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        transaction(self, |mut tx| {
            async move {
                let stmt1 = "DELETE FROM vid_share where view <= $1";
                tx.execute(stmt1, [&(view.u64() as i64)]).await?;

                let stmt2 = "DELETE FROM da_proposal where view <= $1";
                tx.execute(stmt2, [&(view.u64() as i64)]).await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn save_anchor_leaf(
        &mut self,
        leaf: &Leaf,
        qc: &QuorumCertificate<SeqTypes>,
    ) -> anyhow::Result<()> {
        let stmt = "
            INSERT INTO anchor_leaf (id, height, view, leaf, qc) VALUES (0, $1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET (height, view, leaf, qc) = ROW (
                GREATEST(anchor_leaf.height, excluded.height),
                CASE
                    WHEN excluded.height > anchor_leaf.height THEN excluded.view
                    ELSE anchor_leaf.view
                END,
                CASE
                    WHEN excluded.height > anchor_leaf.height THEN excluded.leaf
                    ELSE anchor_leaf.leaf
                END,
                CASE
                    WHEN excluded.height > anchor_leaf.height THEN excluded.qc
                    ELSE anchor_leaf.qc
                END
            )
        ";

        let height = leaf.height() as i64;
        let view = qc.view_number.u64() as i64;
        let leaf_bytes = bincode::serialize(leaf)?;
        let qc_bytes = bincode::serialize(qc)?;

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(
                    stmt,
                    [
                        sql_param(&height),
                        sql_param(&view),
                        sql_param(&leaf_bytes),
                        sql_param(&qc_bytes),
                    ],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(self
            .db
            .query_opt_static("SELECT view FROM highest_voted_view WHERE id = 0")
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
            .query_opt_static("SELECT leaf, qc FROM anchor_leaf WHERE id = 0")
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

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        let Some(row) = self
            .db
            .query_opt_static("SELECT leaves, state FROM undecided_state WHERE id = 0")
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
            .query_opt(
                "SELECT data FROM da_proposal where view = $1",
                [&(view.u64() as i64)],
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
            .query_opt(
                "SELECT data FROM vid_share where view = $1",
                [&(view.u64() as i64)],
            )
            .await?;

        result
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                anyhow::Result::<_>::Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn append_vid(
        &mut self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        transaction(self, |mut tx| {
            async move {
                tx.upsert(
                    "vid_share",
                    ["view", "data"],
                    ["view"],
                    [[sql_param(&(view as i64)), sql_param(&data_bytes)]],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }
    async fn append_da(
        &mut self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        transaction(self, |mut tx| {
            async move {
                tx.upsert(
                    "da_proposal",
                    ["view", "data"],
                    ["view"],
                    [[sql_param(&(view as i64)), sql_param(&data_bytes)]],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }
    async fn record_action(
        &mut self,
        view: ViewNumber,
        _action: HotShotAction,
    ) -> anyhow::Result<()> {
        let stmt = "
        INSERT INTO highest_voted_view (id, view) VALUES (0, $1)
        ON CONFLICT (id) DO UPDATE SET view = GREATEST(highest_voted_view.view, excluded.view)";

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(stmt, [view.u64() as i64])
                    .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }
    async fn update_undecided_state(
        &mut self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        if !self.store_undecided_state {
            return Ok(());
        }

        let leaves_bytes = bincode::serialize(&leaves).context("serializing leaves")?;
        let state_bytes = bincode::serialize(&state).context("serializing state")?;

        transaction(self, |mut tx| {
            async move {
                tx.upsert(
                    "undecided_state",
                    ["id", "leaves", "state"],
                    ["id"],
                    [[
                        sql_param(&0i32),
                        sql_param(&leaves_bytes),
                        sql_param(&state_bytes),
                    ]],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }
}

fn sql_param<T: ToSql + Sync>(param: &T) -> &(dyn ToSql + Sync) {
    param
}

#[cfg(test)]
mod testing {
    use hotshot_query_service::data_source::storage::sql::testing::TmpDb;

    use super::{super::testing::TestablePersistence, *};

    #[async_trait]
    impl TestablePersistence for Persistence {
        type Storage = TmpDb;

        async fn tmp_storage() -> Self::Storage {
            TmpDb::init().await
        }

        async fn connect(db: &Self::Storage) -> Self {
            Options {
                port: Some(db.port()),
                host: Some(db.host()),
                user: Some("postgres".into()),
                password: Some("password".into()),
                ..Default::default()
            }
            .create()
            .await
            .unwrap()
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
