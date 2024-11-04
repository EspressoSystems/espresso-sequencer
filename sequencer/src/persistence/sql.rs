use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use committable::Committable;
use derivative::Derivative;
use espresso_types::{
    parse_duration,
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
    BackoffParams, Leaf, NetworkConfig, Payload,
};
use futures::stream::StreamExt;
use hotshot_query_service::{
    availability::LeafQueryData,
    data_source::{
        storage::{
            pruning::PrunerCfg,
            sql::{include_migrations, query_as, Config, SqlStorage, Transaction, TransactionMode},
        },
        Transaction as _, VersionedDataSource,
    },
    fetching::{
        request::{LeafRequest, PayloadRequest, VidCommonRequest},
        Provider,
    },
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::{Event, EventType, HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::ConsensusTime,
    },
    utils::View,
    vid::{VidCommitment, VidCommon},
    vote::HasViewNumber,
};
use sqlx::Row;
use sqlx::{query, Executor};
use std::sync::Arc;
use std::{collections::BTreeMap, time::Duration};

use crate::{catchup::SqlStateCatchup, SeqTypes, ViewNumber};

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Derivative)]
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

    /// Number of views to retain in consensus storage before data that hasn't been archived is
    /// garbage collected.
    ///
    /// The longer this is, the more certain that all data will eventually be archived, even if
    /// there are temporary problems with archive storage or partially missing data. This can be set
    /// very large, as most data is garbage collected as soon as it is finalized by consensus. This
    /// setting only applies to views which never get decided (ie forks in consensus) and views for
    /// which this node is partially offline. These should be exceptionally rare.
    ///
    /// The default of 130000 views equates to approximately 3 days (259200 seconds) at an average
    /// view time of 2s.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_CONSENSUS_VIEW_RETENTION",
        default_value = "130000"
    )]
    pub(crate) consensus_view_retention: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
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
#[derive(Parser, Clone, Debug)]
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

    fn set_view_retention(&mut self, view_retention: u64) {
        self.consensus_view_retention = view_retention;
    }

    async fn create(self) -> anyhow::Result<Persistence> {
        let persistence = Persistence {
            store_undecided_state: self.store_undecided_state,
            view_retention: self.consensus_view_retention,
            db: SqlStorage::connect(self.try_into()?).await?,
        };
        persistence.migrate_quorum_proposal_leaf_hashes().await?;
        Ok(persistence)
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
#[derive(Debug)]
pub struct Persistence {
    db: SqlStorage,
    store_undecided_state: bool,
    view_retention: u64,
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
            let mut tx = self.db.write().await?;

            // Collect a chain of consecutive leaves, starting from the first view after the last
            // decide. This will correspond to a decide event, and defines a range of views which
            // can be garbage collected. This may even include views for which there was no leaf,
            // for which we might still have artifacts like proposals that never finalized.
            let from_view = match last_processed_view {
                Some(v) => v + 1,
                None => 0,
            };

            let mut parent = None;
            let mut rows = query("SELECT leaf, qc FROM anchor_leaf WHERE view >= $1 ORDER BY view")
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
                    }
                };

                let leaf_data: Vec<u8> = row.get("leaf");
                let leaf = bincode::deserialize::<Leaf>(&leaf_data)?;
                let qc_data: Vec<u8> = row.get("qc");
                let qc = bincode::deserialize::<QuorumCertificate<SeqTypes>>(&qc_data)?;
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
            // deleted once we have processed this chain.
            let from_view = leaves[0].view_number();
            let to_view = leaves[leaves.len() - 1].view_number();

            // Clean up decided leaves, that we no longer need after this processing, except do not
            // delete the most recent leaf: we need to remember this so that in case we restart, we
            // can resume consensus from the last decided leaf.
            tx.execute(
                query("DELETE FROM anchor_leaf WHERE view >= $1 AND view < $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;

            // Clean up and collect VID shares.
            let mut vid_shares = tx
            .fetch_all(
                query("DELETE FROM vid_share where view >= $1 AND view <= $2 RETURNING view, data")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
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

            // Clean up and collect DA proposals.
            let mut da_proposals = tx
            .fetch_all(
                query(
                    "DELETE FROM da_proposal where view >= $1 AND view <= $2 RETURNING view, data",
                )
                .bind(from_view.u64() as i64)
                .bind(to_view.u64() as i64),
            )
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

            // Clean up old proposals and certificates. These are not part of the decide event we
            // generate for the consumer, so we don't need to return them.
            tx.execute(
                query("DELETE FROM quorum_proposals where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;
            tx.execute(
                query("DELETE FROM quorum_certificate where view >= $1 AND view <= $2")
                    .bind(from_view.u64() as i64)
                    .bind(to_view.u64() as i64),
            )
            .await?;

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
            tx.commit().await?;
            last_processed_view = Some(to_view.u64() as i64);
        }
    }

    #[tracing::instrument(skip(self))]
    async fn prune(&self, view: ViewNumber) -> anyhow::Result<()> {
        let view = view.u64().saturating_sub(self.view_retention) as i64;
        if view == 0 {
            // Nothing to prune, the entire chain is younger than the retention period.
            return Ok(());
        }

        let mut tx = self.db.write().await?;

        let res = query("DELETE FROM anchor_leaf WHERE view < $1")
            .bind(view)
            .execute(tx.as_mut())
            .await
            .context("deleting old anchor leaves")?;
        tracing::debug!("garbage collecting {} leaves", res.rows_affected());

        let res = query("DELETE FROM vid_share WHERE view < $1")
            .bind(view)
            .execute(tx.as_mut())
            .await
            .context("deleting old VID shares")?;
        tracing::debug!("garbage collecting {} VID shares", res.rows_affected());

        let res = query("DELETE FROM da_proposal WHERE view < $1")
            .bind(view)
            .execute(tx.as_mut())
            .await
            .context("deleting old DA proposals")?;
        tracing::debug!("garbage collecting {} DA proposals", res.rows_affected());

        let res = query("DELETE FROM quorum_proposals WHERE view < $1")
            .bind(view)
            .execute(tx.as_mut())
            .await
            .context("deleting old quorum proposals")?;
        tracing::debug!(
            "garbage collecting {} quorum proposals",
            res.rows_affected()
        );

        let res = query("DELETE FROM quorum_certificate WHERE view < $1")
            .bind(view)
            .execute(tx.as_mut())
            .await
            .context("deleting old quorum certificates")?;
        tracing::debug!(
            "garbage collecting {} quorum certificates",
            res.rows_affected()
        );

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
        tracing::info!("saving config to Postgres");
        let json = serde_json::to_value(cfg)?;

        let mut tx = self.db.write().await?;
        tx.execute_one_with_retries("INSERT INTO network_config (config) VALUES ($1)", (json,))
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

        tx.upsert("anchor_leaf", ["view", "leaf", "qc"], ["view"], values)
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

    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        // Todo Remove this after https://github.com/EspressoSystems/espresso-sequencer/issues/1931
        if !matches!(action, HotShotAction::Propose | HotShotAction::Vote) {
            return Ok(());
        }
        let stmt = "
        INSERT INTO highest_voted_view (id, view) VALUES (0, $1)
        ON CONFLICT (id) DO UPDATE SET view = GREATEST(highest_voted_view.view, excluded.view)";

        let mut tx = self.db.write().await?;
        tx.execute_one_with_retries(stmt, (view.u64() as i64,))
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

        // We also keep track of any QC we see in case we need it to recover our archival storage.
        let justify_qc = &proposal.data.justify_qc;
        let justify_qc_bytes = bincode::serialize(&justify_qc).context("serializing QC")?;
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

#[async_trait]
impl Provider<SeqTypes, VidCommonRequest> for Persistence {
    #[tracing::instrument(skip(self))]
    async fn fetch(&self, req: VidCommonRequest) -> Option<VidCommon> {
        let mut tx = match self.db.read().await {
            Ok(tx) => tx,
            Err(err) => {
                tracing::warn!("could not open transaction: {err:#}");
                return None;
            }
        };

        let bytes = match query_as::<(Vec<u8>,)>(
            "SELECT data FROM vid_share WHERE payload_hash = $1 LIMIT 1",
        )
        .bind(req.0.to_string())
        .fetch_one(tx.as_mut())
        .await
        {
            Ok((bytes,)) => bytes,
            Err(err) => {
                tracing::warn!("error loading VID share: {err:#}");
                return None;
            }
        };

        let share: Proposal<SeqTypes, VidDisperseShare<SeqTypes>> =
            match bincode::deserialize(&bytes) {
                Ok(share) => share,
                Err(err) => {
                    tracing::warn!("error decoding VID share: {err:#}");
                    return None;
                }
            };

        Some(share.data.common)
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
            }
        };

        let bytes = match query_as::<(Vec<u8>,)>(
            "SELECT data FROM da_proposal WHERE payload_hash = $1 LIMIT 1",
        )
        .bind(req.0.to_string())
        .fetch_one(tx.as_mut())
        .await
        {
            Ok((bytes,)) => bytes,
            Err(err) => {
                tracing::warn!("error loading DA proposal: {err:#}");
                return None;
            }
        };

        let proposal: Proposal<SeqTypes, DaProposal<SeqTypes>> = match bincode::deserialize(&bytes)
        {
            Ok(proposal) => proposal,
            Err(err) => {
                tracing::warn!("error decoding DA proposal: {err:#}");
                return None;
            }
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
            }
        };

        let (leaf, qc) = match fetch_leaf_from_proposals(&mut tx, req).await {
            Ok(res) => res,
            Err(err) => {
                tracing::info!("requested leaf not found in undecided proposals: {err:#}");
                return None;
            }
        };

        match LeafQueryData::new(leaf, qc) {
            Ok(leaf) => Some(leaf),
            Err(err) => {
                tracing::warn!("fetched invalid leaf: {err:#}");
                None
            }
        }
    }
}

async fn fetch_leaf_from_proposals<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    req: LeafRequest<SeqTypes>,
) -> anyhow::Result<(Leaf, QuorumCertificate<SeqTypes>)> {
    // Look for a quorum proposal corresponding to this leaf.
    let (proposal_bytes,) =
        query_as::<(Vec<u8>,)>("SELECT data FROM quorum_proposals WHERE leaf_hash = $1 LIMIT 1")
            .bind(req.expected_leaf.to_string())
            .fetch_one(tx.as_mut())
            .await
            .context("fetching proposal")?;

    // Look for a QC corresponding to this leaf.
    let (qc_bytes,) =
        query_as::<(Vec<u8>,)>("SELECT data FROM quorum_certificate WHERE leaf_hash = $1 LIMIT 1")
            .bind(req.expected_leaf.to_string())
            .fetch_one(tx.as_mut())
            .await
            .context("fetching QC")?;

    let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
        bincode::deserialize(&proposal_bytes).context("deserializing quorum proposal")?;
    let qc: QuorumCertificate<SeqTypes> =
        bincode::deserialize(&qc_bytes).context("deserializing quorum certificate")?;

    let leaf = Leaf::from_quorum_proposal(&proposal.data);
    Ok((leaf, qc))
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

        fn options(db: &Self::Storage) -> impl PersistenceOptions<Persistence = Self> {
            Options {
                port: Some(db.port()),
                host: Some(db.host()),
                user: Some("postgres".into()),
                password: Some("password".into()),
                ..Default::default()
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
    use hotshot_types::{
        traits::{signature_key::SignatureKey, EncodeBytes},
        vid::vid_scheme,
    };
    use jf_vid::VidScheme;
    use sequencer_utils::test_utils::setup_test;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_quorum_proposals_leaf_hash_migration() {
        setup_test();

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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_fetching_providers() {
        setup_test();

        let tmp = Persistence::tmp_storage().await;
        let storage = Persistence::connect(&tmp).await;

        // Mock up some data.
        let leaf = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
        let leaf_payload = leaf.block_payload().unwrap();
        let leaf_payload_bytes_arc = leaf_payload.encode();
        let disperse = vid_scheme(2)
            .disperse(leaf_payload_bytes_arc.clone())
            .unwrap();
        let payload_commitment = disperse.commit;
        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let vid_share = VidDisperseShare::<SeqTypes> {
            view_number: ViewNumber::new(0),
            payload_commitment,
            share: disperse.shares[0].clone(),
            common: disperse.common,
            recipient_key: pubkey,
        }
        .to_proposal(&privkey)
        .unwrap()
        .clone();

        let quorum_proposal = QuorumProposal::<SeqTypes> {
            block_header: leaf.block_header().clone(),
            view_number: leaf.view_number(),
            justify_qc: leaf.justify_qc(),
            upgrade_certificate: None,
            proposal_certificate: None,
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
            data: DaProposal::<SeqTypes> {
                encoded_transactions: leaf_payload_bytes_arc,
                metadata: leaf_payload.ns_table().clone(),
                view_number: ViewNumber::new(0),
            },
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        let mut next_quorum_proposal = quorum_proposal.clone();
        next_quorum_proposal.data.view_number += 1;
        next_quorum_proposal.data.justify_qc.view_number += 1;
        next_quorum_proposal.data.justify_qc.data.leaf_commit = Committable::commit(&leaf);
        let qc = &next_quorum_proposal.data.justify_qc;

        // Add to database.
        storage
            .append_da(&da_proposal, payload_commitment)
            .await
            .unwrap();
        storage.append_vid(&vid_share).await.unwrap();
        storage
            .append_quorum_proposal(&quorum_proposal)
            .await
            .unwrap();

        // Add an extra quorum proposal so we have a QC pointing back at `leaf`.
        storage
            .append_quorum_proposal(&next_quorum_proposal)
            .await
            .unwrap();

        // Fetch it as if we were rebuilding an archive.
        assert_eq!(
            vid_share.data.common,
            storage
                .fetch(VidCommonRequest(vid_share.data.payload_commitment))
                .await
                .unwrap()
        );
        assert_eq!(
            leaf_payload,
            storage
                .fetch(PayloadRequest(vid_share.data.payload_commitment))
                .await
                .unwrap()
        );
        assert_eq!(
            LeafQueryData::new(leaf.clone(), qc.clone()).unwrap(),
            storage
                .fetch(LeafRequest::new(
                    leaf.block_header().block_number(),
                    Committable::commit(&leaf),
                    qc.commit()
                ))
                .await
                .unwrap()
        );
    }
}
