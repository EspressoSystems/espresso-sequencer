use anyhow::Context;
use async_std::{stream::StreamExt, sync::Arc};
use async_trait::async_trait;
use clap::Parser;
use derivative::Derivative;
use espresso_types::{
    parse_duration,
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
    BackoffParams, Leaf, NetworkConfig, Payload,
};
use hotshot_query_service::data_source::{
    storage::{
        pruning::PrunerCfg,
        sql::{include_migrations, postgres::types::ToSql, Config, SqlStorage, Transaction},
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
    vote::HasViewNumber,
};
use std::{collections::BTreeMap, time::Duration};

use crate::{catchup::SqlStateCatchup, SeqTypes, ViewNumber};

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

    /// Disable pruning and reconstruct previously pruned data.
    ///
    /// While running without pruning is the default behavior, the default will not try to
    /// reconstruct data that was pruned in a previous run where pruning was enabled. This option
    /// instructs the service to run without pruning _and_ reconstruct all previously pruned data by
    /// fetching from peers.
    #[clap(long, env = "ESPRESSO_SEQUENCER_ARCHIVE", conflicts_with = "prune")]
    pub(crate) archive: bool,
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
            .query_opt_static("SELECT config FROM network_config ORDER BY id DESC LIMIT 1")
            .await?
        else {
            tracing::info!("config not found");
            return Ok(None);
        };
        let config = row.try_get("config")?;
        Ok(serde_json::from_value(config)?)
    }

    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        tracing::info!("saving config to Postgres");
        let json = serde_json::to_value(cfg)?;

        let mut tx = self.db.write().await?;
        tx.execute_one_with_retries("INSERT INTO network_config (config) VALUES ($1)", [&json])
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
                let view = qc.view_number.u64() as i64;
                let leaf_bytes = bincode::serialize(&info.leaf)?;
                let qc_bytes = bincode::serialize(&qc)?;
                Ok((view, leaf_bytes, qc_bytes))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        // First, append the new leaves. We do this in its own transaction because even if GC or the
        // event consumer later fails, there is no need to abort the storage of the leaves.
        let mut tx = self.db.write().await?;
        let rows = values
            .iter()
            .map(|(view, leaf, qc)| [sql_param(view), sql_param(leaf), sql_param(qc)]);

        tx.upsert("anchor_leaf", ["view", "leaf", "qc"], ["view"], rows)
            .await?;
        tx.commit().await?;

        // Generate an event for the new leaves and, only if it succeeds, clean up data we no longer
        // need.
        let consumer = dyn_clone::clone(consumer);
        let tx = self.db.write().await?;
        if let Err(err) = collect_garbage(tx, view, consumer).await {
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
            .read()
            .await?
            .query_opt_static("SELECT leaf, qc FROM anchor_leaf ORDER BY view DESC LIMIT 1")
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
            .read()
            .await?
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
            .read()
            .await?
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
            .read()
            .await?
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

    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>> {
        let rows = self
            .db
            .read()
            .await?
            .query_static("SELECT * FROM quorum_proposals")
            .await?;

        Ok(BTreeMap::from_iter(
            rows.map(|row| {
                let row = row?;
                let view: i64 = row.get("view");
                let view_number: ViewNumber = ViewNumber::new(view.try_into()?);
                let bytes: Vec<u8> = row.get("data");
                let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
                    bincode::deserialize(&bytes)?;
                Ok((view_number, proposal))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .await?,
        ))
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
            [[sql_param(&(view as i64)), sql_param(&data_bytes)]],
        )
        .await?;
        tx.commit().await
    }
    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let data = &proposal.data;
        let view = data.view_number().u64();
        let data_bytes = bincode::serialize(proposal).unwrap();

        let mut tx = self.db.write().await?;
        tx.upsert(
            "da_proposal",
            ["view", "data"],
            ["view"],
            [[sql_param(&(view as i64)), sql_param(&data_bytes)]],
        )
        .await?;
        tx.commit().await
    }
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        // Todo Remove this after https://github.com/EspressoSystems/espresso-sequencer/issues/1931
        if !matches!(action, HotShotAction::Propose | HotShotAction::Vote) {
            return Ok(());
        }
        let stmt = "
        INSERT INTO highest_voted_view (id, view) VALUES (0, $1)
        ON CONFLICT (id) DO UPDATE SET view = GREATEST(highest_voted_view.view, excluded.view)";

        let mut tx = self.db.write().await?;
        tx.execute_one_with_retries(stmt, [view.u64() as i64])
            .await?;
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
            [[
                sql_param(&0i32),
                sql_param(&leaves_bytes),
                sql_param(&state_bytes),
            ]],
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
        let mut tx = self.db.write().await?;
        tx.upsert(
            "quorum_proposals",
            ["view", "data"],
            ["view"],
            [[sql_param(&(view_number as i64)), sql_param(&proposal_bytes)]],
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
            .query_opt_static("SELECT * FROM upgrade_certificate where id = 0")
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
            [[sql_param(&0i32), sql_param(&upgrade_certificate_bytes)]],
        )
        .await?;
        tx.commit().await
    }
}

async fn collect_garbage(
    mut tx: Transaction<'_>,
    view: ViewNumber,
    consumer: impl EventConsumer,
) -> anyhow::Result<()> {
    // Clean up and collect VID shares.
    let mut vid_shares = tx
        .query(
            "DELETE FROM vid_share where view <= $1 RETURNING view, data",
            [&(view.u64() as i64)],
        )
        .await?
        .map(|row| {
            let row = row?;
            let view: i64 = row.get("view");
            let data: Vec<u8> = row.get("data");
            let vid_proposal =
                bincode::deserialize::<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>(&data)?;
            Ok((view as u64, vid_proposal.data))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()
        .await?;

    // Clean up and collect DA proposals.
    let mut da_proposals = tx
        .query(
            "DELETE FROM da_proposal where view <= $1 RETURNING view, data",
            [&(view.u64() as i64)],
        )
        .await?
        .map(|row| {
            let row = row?;
            let view: i64 = row.get("view");
            let data: Vec<u8> = row.get("data");
            let da_proposal =
                bincode::deserialize::<Proposal<SeqTypes, DaProposal<SeqTypes>>>(&data)?;
            Ok((view as u64, da_proposal.data))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()
        .await?;

    // Clean up and collect leaves, except do not delete the most recent leaf: we need to remember
    // this so that in case we restart, we can pick up from the last decided leaf. We still do
    // include this leaf in the query results (the `UNION` clause) so we can include it in the
    // decide event we send to the consumer.
    let mut leaves = tx
        .query(
            "SELECT view, leaf, qc FROM anchor_leaf WHERE view <= $1",
            [&(view.u64() as i64)],
        )
        .await?
        .map(|row| {
            let row = row?;
            let view: i64 = row.get("view");
            let leaf_data: Vec<u8> = row.get("leaf");
            let leaf = bincode::deserialize::<Leaf>(&leaf_data)?;
            let qc_data: Vec<u8> = row.get("qc");
            let qc = bincode::deserialize::<QuorumCertificate<SeqTypes>>(&qc_data)?;
            Ok((view as u64, (leaf, qc)))
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()
        .await?;
    tx.execute(
        "DELETE FROM anchor_leaf WHERE view < $1",
        [&(view.u64() as i64)],
    )
    .await?;

    // Clean up old proposals. These are not part of the decide event we generate for the consumer,
    // so we don't need to return them.
    tx.execute(
        "DELETE FROM quorum_proposals where view <= $1",
        [&(view.u64() as i64)],
    )
    .await?;

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
        .query_opt_static("SELECT last_processed_view FROM event_stream WHERE id = 1 LIMIT 1")
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

    // Collate all the information by view number and construct a chain of leaves and a chain of
    // corresponding QCs.
    let (leaf_chain, qcs): (Vec<_>, Vec<_>) = leaves
        .into_iter()
        // Go in reverse chronological order, as expected by Decide events.
        .rev()
        .map(|(view, (mut leaf, qc))| {
            // Include the VID share if available.
            let vid_share = vid_shares.remove(&view);
            if vid_share.is_none() {
                tracing::debug!(view, "VID share not available at decide");
            }

            // Fill in the full block payload using the DA proposals we had persisted.
            if let Some(proposal) = da_proposals.remove(&view) {
                let payload =
                    Payload::from_bytes(&proposal.encoded_transactions, &proposal.metadata);
                leaf.fill_block_payload_unchecked(payload);
            } else {
                tracing::debug!(view, "DA proposal not available at decide");
            }

            (
                LeafInfo {
                    leaf,
                    vid_share,

                    // Note: the following fields are not used in Decide event processing, and
                    // should be removed. For now, we just default them.
                    state: Default::default(),
                    delta: Default::default(),
                },
                qc,
            )
        })
        .unzip();

    // Generate decide event for the consumer.
    let Some(final_qc) = qcs.into_iter().next() else {
        tracing::info!(?view, "no new leaves at decide");
        return Ok(());
    };
    tracing::debug!(?view, ?final_qc, ?leaf_chain, "generating decide event");

    consumer
        .handle_event(&Event {
            view_number: view,
            event: EventType::Decide {
                leaf_chain: Arc::new(leaf_chain),
                qc: Arc::new(final_qc),
                block_size: None,
            },
        })
        .await?;

    // Now that we have definitely processed leaves up to `view`, we can update
    // `last_processed_view` so we don't process these leaves again. We may still fail at this
    // point, or shut down, and fail to complete this update. At worst this will lead to us sending
    // a duplicate decide event the next time we are called; this is fine as the event consumer is
    // required to be idempotent.
    tx.upsert(
        "event_stream",
        ["id", "last_processed_view"],
        ["id"],
        [[sql_param(&1i32), sql_param(&(view.u64() as i64))]],
    )
    .await?;

    tx.commit().await
}

pub(crate) fn sql_param<T: ToSql + Sync>(param: &T) -> &(dyn ToSql + Sync) {
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
