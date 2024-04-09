use std::time::Duration;

use anyhow::bail;
use async_trait::async_trait;
use clap::Parser;
use futures::future::{BoxFuture, FutureExt};
use hotshot_query_service::data_source::{
    storage::{
        pruning::PrunerCfg,
        sql::{include_migrations, postgres::types::ToSql, Config, Query, SqlStorage, Transaction},
    },
    VersionedDataSource,
};
use hotshot_types::{
    data::{DAProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::node_implementation::ConsensusTime,
    vote::HasViewNumber,
};

use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{options::parse_duration, Leaf, SeqTypes, ValidatedState, ViewNumber};

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Debug, Default)]
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
    pub uri: Option<String>,

    /// Hostname for the remote Postgres database server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_HOST")]
    pub host: Option<String>,

    /// Port for the remote Postgres database server.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PORT")]
    pub port: Option<u16>,

    /// Name of database to connect to.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_DATABASE")]
    pub database: Option<String>,

    /// Postgres user to connect as.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_USER")]
    pub user: Option<String>,

    /// Password for Postgres user.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PASSWORD")]
    pub password: Option<String>,

    /// Use TLS for an encrypted connection to the database.
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_USE_TLS")]
    pub use_tls: bool,

    /// This will enable the pruner and set the default pruning parameters unless provided.
    /// Default parameters:
    /// - pruning_threshold: 3 TB
    /// - minimum_retention: 1 day
    /// - target_retention: 7 days
    /// - batch_size: 1000
    /// - max_usage: 80%
    /// - interval: 1 hour
    #[clap(long, env = "ESPRESSO_SEQUENCER_POSTGRES_PRUNE")]
    pub prune: bool,

    /// Pruning parameters.
    #[clap(flatten)]
    pub pruning: PruningOptions,
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
        SqlStorage::connect(self.try_into()?).await
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
pub type Persistence = SqlStorage;

async fn transaction(
    db: &mut Persistence,
    f: impl FnOnce(Transaction) -> BoxFuture<anyhow::Result<()>>,
) -> anyhow::Result<()> {
    let tx = db.transaction().await?;
    match f(tx).await {
        Ok(_) => {
            db.commit().await?;
            Ok(())
        }
        Err(err) => {
            tracing::warn!("transaction failed, reverting: {err:#}");
            db.revert().await;
            Err(err)
        }
    }
}

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        tracing::info!("loading config from Postgres");

        // Select the most recent config (although there should only be one).
        let Some(row) = self
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
                tx.execute(stmt1, [&(view.get_u64() as i64)]).await?;

                let stmt2 = "DELETE FROM da_proposal where view <= $1";
                tx.execute(stmt2, [&(view.get_u64() as i64)]).await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn save_anchor_leaf(&mut self, leaf: &Leaf) -> anyhow::Result<()> {
        let stmt = "
            INSERT INTO anchor_leaf (id, height, leaf) VALUES (0, $1, $2)
            ON CONFLICT (id) DO UPDATE SET (height, leaf) = ROW (
                GREATEST(anchor_leaf.height, excluded.height),
                CASE
                    WHEN excluded.height > anchor_leaf.height THEN excluded.leaf
                    ELSE anchor_leaf.leaf
                END
            )
        ";
        let height = leaf.get_height() as i64;
        let leaf_bytes = bincode::serialize(leaf)?;

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(stmt, [sql_param(&height), sql_param(&leaf_bytes)])
                    .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(self
            .query_opt_static("SELECT view FROM highest_voted_view WHERE id = 0")
            .await?
            .map(|row| {
                let view: i64 = row.get("view");
                ViewNumber::new(view as u64)
            }))
    }

    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>> {
        self.query_opt_static("SELECT leaf FROM anchor_leaf WHERE id = 0")
            .await?
            .map(|row| {
                let bytes: Vec<u8> = row.get("leaf");
                Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn load_high_qc(&self) -> anyhow::Result<Option<QuorumCertificate<SeqTypes>>> {
        self.query_opt_static("SELECT data FROM high_qc WHERE id = 0")
            .await?
            .map(|row| {
                let bytes: Vec<u8> = row.get("data");
                Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DAProposal<SeqTypes>>>> {
        let result = self
            .query_opt(
                "SELECT data FROM da_proposal where view = $1",
                [&(view.get_u64() as i64)],
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
            .query_opt(
                "SELECT data FROM vid_share where view = $1",
                [&(view.get_u64() as i64)],
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
        let view = data.get_view_number().get_u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(
                    "INSERT INTO vid_share (view, data) VALUES ($1, $2)",
                    [sql_param(&(view as i64)), sql_param(&data_bytes)],
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
        proposal: &Proposal<SeqTypes, DAProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.get_view_number().get_u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(
                    "INSERT INTO da_proposal (view, data) VALUES ($1, $2)",
                    [sql_param(&(view as i64)), sql_param(&data_bytes)],
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
                tx.execute_one_with_retries(stmt, [view.get_u64() as i64])
                    .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }
    async fn update_high_qc(&mut self, high_qc: QuorumCertificate<SeqTypes>) -> anyhow::Result<()> {
        let view = high_qc.view_number.get_u64();
        let data_bytes = bincode::serialize(&high_qc).unwrap();

        transaction(self, |mut tx| {
            async move {
                tx.execute_one_with_retries(
                    "INSERT INTO high_qc (id, view, data) VALUES (0, $1, $2)
                ON CONFLICT(id) DO UPDATE SET (view, data) = ROW (
                    GREATEST(high_qc.view, excluded.view),
                CASE
                    WHEN excluded.view > high_qc.view THEN excluded.data
                    ELSE high_qc.data
                END )",
                    [sql_param(&(view as i64)), sql_param(&data_bytes)],
                )
                .await?;
                Ok(())
            }
            .boxed()
        })
        .await
    }

    async fn load_validated_state(&self, _height: u64) -> anyhow::Result<ValidatedState> {
        bail!("state persistence not implemented");
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
