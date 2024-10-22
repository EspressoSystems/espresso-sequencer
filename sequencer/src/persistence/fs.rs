use anyhow::{anyhow, Context};
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use clap::Parser;
use espresso_types::{
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence},
    Leaf, NetworkConfig, Payload, SeqTypes,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::{Event, EventType, HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{block_contents::BlockPayload, node_implementation::ConsensusTime},
    utils::View,
    vote::HasViewNumber,
};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::ViewNumber;

/// Options for file system backed persistence.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Storage path for persistent data.
    #[clap(long, env = "ESPRESSO_SEQUENCER_STORAGE_PATH")]
    path: PathBuf,

    #[clap(long, env = "ESPRESSO_SEQUENCER_STORE_UNDECIDED_STATE", hide = true)]
    store_undecided_state: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

impl Options {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            store_undecided_state: false,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    async fn create(self) -> anyhow::Result<Persistence> {
        Ok(Persistence {
            store_undecided_state: self.store_undecided_state,
            inner: Arc::new(RwLock::new(Inner { path: self.path })),
        })
    }

    async fn reset(self) -> anyhow::Result<()> {
        todo!()
    }
}

/// File system backed persistence.
#[derive(Clone, Debug)]
pub struct Persistence {
    store_undecided_state: bool,

    // We enforce mutual exclusion on access to the data source, as the current file system
    // implementation does not support transaction isolation for concurrent reads and writes. We can
    // improve this in the future by switching to a SQLite-based file system implementation.
    inner: Arc<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
    path: PathBuf,
}

impl Inner {
    fn config_path(&self) -> PathBuf {
        self.path.join("hotshot.cfg")
    }

    fn voted_view_path(&self) -> PathBuf {
        self.path.join("highest_voted_view")
    }

    /// Path to a directory containing decided leaves.
    fn decided_leaf_path(&self) -> PathBuf {
        self.path.join("decided_leaves")
    }

    /// The path from previous versions where there was only a single file for anchor leaves.
    fn legacy_anchor_leaf_path(&self) -> PathBuf {
        self.path.join("anchor_leaf")
    }

    fn vid_dir_path(&self) -> PathBuf {
        self.path.join("vid")
    }

    fn da_dir_path(&self) -> PathBuf {
        self.path.join("da")
    }

    fn undecided_state_path(&self) -> PathBuf {
        self.path.join("undecided_state")
    }

    fn quorum_proposals_dir_path(&self) -> PathBuf {
        self.path.join("quorum_proposals")
    }

    fn upgrade_certificate_dir_path(&self) -> PathBuf {
        self.path.join("upgrade_certificate")
    }

    /// Overwrite a file if a condition is met.
    ///
    /// The file at `path`, if it exists, is opened in read mode and passed to `pred`. If `pred`
    /// returns `true`, or if there was no existing file, then `write` is called to update the
    /// contents of the file. `write` receives a truncated file open in write mode and sets the
    /// contents of the file.
    ///
    /// The final replacement of the original file is atomic; that is, `path` will be modified only
    /// if the entire update succeeds.
    fn replace(
        &mut self,
        path: &Path,
        pred: impl FnOnce(File) -> anyhow::Result<bool>,
        write: impl FnOnce(File) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if path.is_file() {
            // If there is an existing file, check if it is suitable to replace. Note that this
            // check is not atomic with respect to the subsequent write at the file system level,
            // but this object is the only one which writes to this file, and we have a mutable
            // reference, so this should be safe.
            if !pred(File::open(path)?)? {
                // If we are not overwriting the file, we are done and consider the whole operation
                // successful.
                return Ok(());
            }
        }

        // Either there is no existing file or we have decided to overwrite the file. Write the new
        // contents into a temporary file so we can update `path` atomically using `rename`.
        let mut swap_path = path.to_owned();
        swap_path.set_extension("swp");
        let swap = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&swap_path)?;
        write(swap)?;

        // Now we can replace the original file.
        fs::rename(swap_path, path)?;

        Ok(())
    }

    fn collect_garbage(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        let view_number = view.u64();

        let delete_files = |view_number: u64, dir_path: PathBuf| -> anyhow::Result<()> {
            if !dir_path.is_dir() {
                return Ok(());
            }

            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if let Some(file) = path.file_stem().and_then(|n| n.to_str()) {
                    if let Ok(v) = file.parse::<u64>() {
                        if v <= view_number {
                            fs::remove_file(&path)?;
                        }
                    }
                }
            }

            Ok(())
        };

        delete_files(view_number, self.da_dir_path())?;
        delete_files(view_number, self.vid_dir_path())?;
        delete_files(view_number, self.quorum_proposals_dir_path())?;

        // Save the most recent leaf as it will be our anchor point if the node restarts.
        if view_number > 0 {
            delete_files(view_number - 1, self.decided_leaf_path())?;
        }

        Ok(())
    }

    fn decide_event(&self, view: ViewNumber) -> anyhow::Result<Event<SeqTypes>> {
        // Construct a chain of all decided leaves up to `view` which have not yet been garbage
        // collected.
        let mut leaves = BTreeMap::new();
        let mut high_qc: Option<QuorumCertificate<SeqTypes>> = None;

        for entry in fs::read_dir(self.decided_leaf_path())? {
            let entry = entry?;
            let path = entry.path();

            let Some(file) = path.file_stem().and_then(|n| n.to_str()) else {
                continue;
            };
            let Ok(v) = file.parse::<u64>() else {
                continue;
            };
            if v > view.u64() {
                continue;
            }

            let bytes =
                fs::read(&path).context(format!("reading decided leaf {}", path.display()))?;
            let (mut leaf, qc) =
                bincode::deserialize::<(Leaf, QuorumCertificate<SeqTypes>)>(&bytes)
                    .context(format!("parsing decided leaf {}", path.display()))?;

            // Include the VID share if available.
            let vid_share = self
                .load_vid_share(ViewNumber::new(v))?
                .map(|proposal| proposal.data);
            if vid_share.is_none() {
                tracing::debug!(view = v, "VID share not available at decide");
            }

            // Fill in the full block payload using the DA proposals we had persisted.
            if let Some(proposal) = self.load_da_proposal(ViewNumber::new(v))? {
                let payload = Payload::from_bytes(
                    &proposal.data.encoded_transactions,
                    &proposal.data.metadata,
                );
                leaf.fill_block_payload_unchecked(payload);
            } else {
                tracing::debug!(view = v, "DA proposal not available at decide");
            }

            let info = LeafInfo {
                leaf,
                vid_share,

                // Note: the following fields are not used in Decide event processing, and should be
                // removed. For now, we just default them.
                state: Default::default(),
                delta: Default::default(),
            };

            leaves.insert(v, info);
            if let Some(high_qc) = &mut high_qc {
                if v > high_qc.view_number.u64() {
                    *high_qc = qc;
                }
            } else {
                high_qc = Some(qc);
            }
        }

        // The invariant is that the oldest existing leaf in the `anchor_leaf` table -- if there is
        // one -- was always included in the _previous_ decide event...but not removed from the
        // database, because we always persist the most recent anchor leaf.
        if let Some((oldest_view, _)) = leaves.first_key_value() {
            // The only exception is when the oldest leaf is the genesis leaf; then there was no
            // previous decide event.
            if *oldest_view > 0 {
                leaves.pop_first();
            }
        }

        let high_qc = high_qc.context("no new leaves at decide event")?;
        Ok(Event {
            view_number: view,
            event: EventType::Decide {
                qc: Arc::new(high_qc),
                block_size: None,
                leaf_chain: Arc::new(leaves.into_values().rev().collect()),
            },
        })
    }

    fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>> {
        let dir_path = self.da_dir_path();

        let file_path = dir_path.join(view.u64().to_string()).with_extension("txt");

        if !file_path.exists() {
            return Ok(None);
        }

        let da_bytes = fs::read(file_path)?;

        let da_proposal: Proposal<SeqTypes, DaProposal<SeqTypes>> =
            bincode::deserialize(&da_bytes)?;
        Ok(Some(da_proposal))
    }

    fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        let dir_path = self.vid_dir_path();

        let file_path = dir_path.join(view.u64().to_string()).with_extension("txt");

        if !file_path.exists() {
            return Ok(None);
        }

        let vid_share_bytes = fs::read(file_path)?;
        let vid_share: Proposal<SeqTypes, VidDisperseShare<SeqTypes>> =
            bincode::deserialize(&vid_share_bytes)?;
        Ok(Some(vid_share))
    }

    fn load_anchor_leaf(&self) -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>> {
        if self.decided_leaf_path().is_dir() {
            let mut anchor: Option<(Leaf, QuorumCertificate<SeqTypes>)> = None;

            // Return the latest decided leaf.
            for entry in
                fs::read_dir(self.decided_leaf_path()).context("opening decided leaf directory")?
            {
                let file = entry.context("reading decided leaf directory")?.path();
                let bytes =
                    fs::read(&file).context(format!("reading decided leaf {}", file.display()))?;
                let (leaf, qc) =
                    bincode::deserialize::<(Leaf, QuorumCertificate<SeqTypes>)>(&bytes)
                        .context(format!("parsing decided leaf {}", file.display()))?;
                if let Some((anchor_leaf, _)) = &anchor {
                    if leaf.view_number() > anchor_leaf.view_number() {
                        anchor = Some((leaf, qc));
                    }
                } else {
                    anchor = Some((leaf, qc));
                }
            }

            return Ok(anchor);
        }

        if self.legacy_anchor_leaf_path().is_file() {
            // We may have an old version of storage, where there is just a single file for the
            // anchor leaf. Read it and return the contents.
            let mut file = File::open(self.legacy_anchor_leaf_path())?;

            // The first 8 bytes just contain the height of the leaf. We can skip this.
            file.seek(SeekFrom::Start(8)).context("seek")?;
            let bytes = file
                .bytes()
                .collect::<Result<Vec<_>, _>>()
                .context("read")?;
            return Ok(Some(bincode::deserialize(&bytes).context("deserialize")?));
        }

        Ok(None)
    }
}

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        let inner = self.inner.read().await;
        let path = inner.config_path();
        if !path.is_file() {
            tracing::info!("config not found at {}", path.display());
            return Ok(None);
        }
        tracing::info!("loading config from {}", path.display());

        let bytes =
            fs::read(&path).context(format!("unable to read config from {}", path.display()))?;
        let json = serde_json::from_slice(&bytes).context("config file is not valid JSON")?;
        let json = migrate_network_config(json).context("migration of network config failed")?;
        let config = serde_json::from_value(json).context("malformed config file")?;
        Ok(Some(config))
    }

    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        let inner = self.inner.write().await;
        let path = inner.config_path();
        tracing::info!("saving config to {}", path.display());
        Ok(cfg.to_file(path.display().to_string())?)
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        let inner = self.inner.read().await;
        let path = inner.voted_view_path();
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(inner.voted_view_path())?
            .try_into()
            .map_err(|bytes| anyhow!("malformed voted view file: {bytes:?}"))?;
        Ok(Some(ViewNumber::new(u64::from_le_bytes(bytes))))
    }

    async fn append_decided_leaves(
        &self,
        view: ViewNumber,
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, QuorumCertificate<SeqTypes>)> + Send,
        consumer: &impl EventConsumer,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        let path = inner.decided_leaf_path();

        // Ensure the anchor leaf directory exists.
        fs::create_dir_all(&path).context("creating anchor leaf directory")?;

        // Earlier versions stored only a single decided leaf in a regular file. If our storage is
        // still on this version, migrate to a directory structure storing (possibly) many leaves.
        let legacy_path = inner.legacy_anchor_leaf_path();
        if !path.is_dir() && legacy_path.is_file() {
            tracing::info!("migrating to multi-leaf storage");

            // Move the existing data into the new directory.
            let (leaf, qc) = inner
                .load_anchor_leaf()?
                .context("anchor leaf file exists but unable to load contents")?;
            let view = leaf.view_number().u64();
            let bytes = bincode::serialize(&(leaf, qc))?;
            let new_file = path.join(view.to_string()).with_extension("txt");
            fs::write(new_file, bytes).context(format!("writing anchor leaf file {view}"))?;

            // Now we can remove the old file.
            fs::remove_file(&legacy_path).context("removing legacy anchor leaf file")?;
        }

        for (info, qc) in leaf_chain {
            let view = info.leaf.view_number().u64();
            let file_path = path.join(view.to_string()).with_extension("txt");
            inner.replace(
                &file_path,
                |_| {
                    // Don't overwrite an existing leaf, but warn about it as this is likely not
                    // intended behavior from HotShot.
                    tracing::warn!(view, "duplicate decided leaf");
                    Ok(false)
                },
                |mut file| {
                    let bytes = bincode::serialize(&(&info.leaf, qc))?;
                    file.write_all(&bytes)?;
                    Ok(())
                },
            )?;
        }

        // Event processing failure is not an error, since by this point we have at least managed to
        // persist the decided leaves successfully, and the event processing will just run again at
        // the next decide. If there is an error here, we just log it and return early with success
        // to prevent GC from running before the decided leaves are processed.
        match inner.decide_event(view) {
            Ok(event) => {
                if let Err(err) = consumer.handle_event(&event).await {
                    tracing::warn!(?view, "event processing failed: {err:#}");
                    return Ok(());
                }
            }
            Err(err) => {
                tracing::warn!(?view, "event creation: {err:#}");
                return Ok(());
            }
        }

        if let Err(err) = inner.collect_garbage(view) {
            // Similarly, garbage collection is not an error. We have done everything we strictly
            // needed to do, and GC will run again at the next decide. Log the error but do not
            // return it.
            tracing::warn!(?view, "GC failed: {err:#}");
        }

        Ok(())
    }

    async fn load_anchor_leaf(
        &self,
    ) -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>> {
        self.inner.read().await.load_anchor_leaf()
    }

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        let inner = self.inner.read().await;
        let path = inner.undecided_state_path();
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(&path).context("read")?;
        Ok(Some(bincode::deserialize(&bytes).context("deserialize")?))
    }

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>> {
        self.inner.read().await.load_da_proposal(view)
    }

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        self.inner.read().await.load_vid_share(view)
    }

    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        let view_number = proposal.data.view_number().u64();
        let dir_path = inner.vid_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create vid dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        inner.replace(
            &file_path,
            |_| {
                // Don't overwrite an existing share, but warn about it as this is likely not intended
                // behavior from HotShot.
                tracing::warn!(view_number, "duplicate VID share");
                Ok(false)
            },
            |mut file| {
                let proposal_bytes = bincode::serialize(&proposal).context("serialize proposal")?;
                file.write_all(&proposal_bytes)?;
                Ok(())
            },
        )
    }
    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        let view_number = proposal.data.view_number().u64();
        let dir_path = inner.da_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create da dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        inner.replace(
            &file_path,
            |_| {
                // Don't overwrite an existing proposal, but warn about it as this is likely not
                // intended behavior from HotShot.
                tracing::warn!(view_number, "duplicate DA proposal");
                Ok(false)
            },
            |mut file| {
                let proposal_bytes = bincode::serialize(&proposal).context("serialize proposal")?;
                file.write_all(&proposal_bytes)?;
                Ok(())
            },
        )
    }
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        // Todo Remove this after https://github.com/EspressoSystems/espresso-sequencer/issues/1931
        if !matches!(action, HotShotAction::Propose | HotShotAction::Vote) {
            return Ok(());
        }
        let mut inner = self.inner.write().await;
        let path = &inner.voted_view_path();
        inner.replace(
            path,
            |mut file| {
                let mut bytes = vec![];
                file.read_to_end(&mut bytes)?;
                let bytes = bytes
                    .try_into()
                    .map_err(|bytes| anyhow!("malformed voted view file: {bytes:?}"))?;
                let saved_view = ViewNumber::new(u64::from_le_bytes(bytes));

                // Overwrite the file if the saved view is older than the new view.
                Ok(saved_view < view)
            },
            |mut file| {
                file.write_all(&view.u64().to_le_bytes())?;
                Ok(())
            },
        )
    }
    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        if !self.store_undecided_state {
            return Ok(());
        }

        let mut inner = self.inner.write().await;
        let path = &inner.undecided_state_path();
        inner.replace(
            path,
            |_| {
                // Always overwrite the previous file.
                Ok(true)
            },
            |mut file| {
                let bytes =
                    bincode::serialize(&(leaves, state)).context("serializing undecided state")?;
                file.write_all(&bytes)?;
                Ok(())
            },
        )
    }
    async fn append_quorum_proposal(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        let view_number = proposal.data.view_number().u64();
        let dir_path = inner.quorum_proposals_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create proposals dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        inner.replace(
            &file_path,
            |_| {
                // Always overwrite the previous file
                Ok(true)
            },
            |mut file| {
                let proposal_bytes = bincode::serialize(&proposal).context("serialize proposal")?;

                file.write_all(&proposal_bytes)?;
                Ok(())
            },
        )
    }
    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>> {
        let inner = self.inner.read().await;

        // First, get the proposal directory.
        let dir_path = inner.quorum_proposals_dir_path();
        if !dir_path.is_dir() {
            return Ok(Default::default());
        }

        // Then, we want to get the entries in this directory since they'll be the
        // key/value pairs for our map.
        let files: Vec<fs::DirEntry> = fs::read_dir(dir_path.clone())?
            .filter_map(|entry| {
                entry
                    .ok()
                    .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            })
            .collect();

        // Read all of the files
        let proposal_files = files
            .into_iter()
            .map(|entry| dir_path.join(entry.file_name()).with_extension("txt"));

        let mut map = BTreeMap::new();
        for file in proposal_files.into_iter() {
            // This operation shouldn't fail, but we don't want to panic here if the filesystem
            // somehow gets corrupted. We get the stem to remove the ".txt" from the end.
            if let Some(file_name) = file.file_stem() {
                // We need to convert the filename (which corresponds to the view)
                let view_number = ViewNumber::new(
                    file_name
                        .to_string_lossy()
                        .parse::<u64>()
                        .context("convert file name to u64")?,
                );

                // Now, we'll try and load the proposal associated with this function.
                let proposal_bytes = fs::read(file)?;

                // Then, deserialize.
                let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> =
                    bincode::deserialize(&proposal_bytes)?;

                // Push to the map and we're done.
                map.insert(view_number, proposal);
            }
        }

        Ok(map)
    }

    async fn load_quorum_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Proposal<SeqTypes, QuorumProposal<SeqTypes>>> {
        let inner = self.inner.read().await;
        let dir_path = inner.quorum_proposals_dir_path();
        let file_path = dir_path.join(view.to_string()).with_extension("txt");
        let bytes = fs::read(file_path)?;
        let proposal = bincode::deserialize(&bytes)?;
        Ok(proposal)
    }

    async fn load_upgrade_certificate(
        &self,
    ) -> anyhow::Result<Option<UpgradeCertificate<SeqTypes>>> {
        let inner = self.inner.read().await;
        let path = inner.upgrade_certificate_dir_path();
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(&path).context("read")?;
        Ok(Some(
            bincode::deserialize(&bytes).context("deserialize upgrade certificate")?,
        ))
    }

    async fn store_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let mut inner = self.inner.write().await;
        let path = &inner.upgrade_certificate_dir_path();
        let certificate = match decided_upgrade_certificate {
            Some(cert) => cert,
            None => return Ok(()),
        };
        inner.replace(
            path,
            |_| {
                // Always overwrite the previous file.
                Ok(true)
            },
            |mut file| {
                let bytes =
                    bincode::serialize(&certificate).context("serializing upgrade certificate")?;
                file.write_all(&bytes)?;
                Ok(())
            },
        )
    }
}

/// Update a `NetworkConfig` that may have originally been persisted with an old version.
fn migrate_network_config(
    mut network_config: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let config = network_config
        .get_mut("config")
        .context("missing field `config`")?
        .as_object_mut()
        .context("`config` must be an object")?;

    if !config.contains_key("builder_urls") {
        // When multi-builder support was added, the configuration field `builder_url: Url` was
        // replaced by an array `builder_urls: Vec<Url>`. If the saved config has no `builder_urls`
        // field, it is older than this change. Populate `builder_urls` with a singleton array
        // formed from the old value of `builder_url`, and delete the no longer used `builder_url`.
        let url = config
            .remove("builder_url")
            .context("missing field `builder_url`")?;
        config.insert("builder_urls".into(), vec![url].into());
    }

    // HotShotConfig was upgraded to include parameters for proposing and voting on upgrades.
    // Configs which were persisted before this upgrade may be missing these parameters. This
    // migration initializes them with a default. By default, we use JS MAX_SAFE_INTEGER for the
    // start parameters so that nodes will never do an upgrade, unless explicitly configured
    // otherwise.
    if !config.contains_key("start_proposing_view") {
        config.insert("start_proposing_view".into(), 9007199254740991u64.into());
    }
    if !config.contains_key("stop_proposing_view") {
        config.insert("stop_proposing_view".into(), 0.into());
    }
    if !config.contains_key("start_voting_view") {
        config.insert("start_voting_view".into(), 9007199254740991u64.into());
    }
    if !config.contains_key("stop_voting_view") {
        config.insert("stop_voting_view".into(), 0.into());
    }
    if !config.contains_key("start_proposing_time") {
        config.insert("start_proposing_time".into(), 9007199254740991u64.into());
    }
    if !config.contains_key("stop_proposing_time") {
        config.insert("stop_proposing_time".into(), 0.into());
    }
    if !config.contains_key("start_voting_time") {
        config.insert("start_voting_time".into(), 9007199254740991u64.into());
    }
    if !config.contains_key("stop_voting_time") {
        config.insert("stop_voting_time".into(), 0.into());
    }

    Ok(network_config)
}

#[cfg(test)]
mod testing {
    use tempfile::TempDir;

    use super::{super::testing::TestablePersistence, *};

    #[async_trait]
    impl TestablePersistence for Persistence {
        type Storage = TempDir;

        async fn tmp_storage() -> Self::Storage {
            TempDir::new().unwrap()
        }

        async fn connect(storage: &Self::Storage) -> Self {
            Options::new(storage.path().into()).create().await.unwrap()
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
    use serde_json::json;

    use super::*;

    #[test]
    fn test_config_migrations_add_builder_urls() {
        let before = json!({
            "config": {
                "builder_url": "https://test:8080",
                "start_proposing_view": 1,
                "stop_proposing_view": 2,
                "start_voting_view": 1,
                "stop_voting_view": 2,
                "start_proposing_time": 1,
                "stop_proposing_time": 2,
                "start_voting_time": 1,
                "stop_voting_time": 2
            }
        });
        let after = json!({
            "config": {
                "builder_urls": ["https://test:8080"],
                "start_proposing_view": 1,
                "stop_proposing_view": 2,
                "start_voting_view": 1,
                "stop_voting_view": 2,
                "start_proposing_time": 1,
                "stop_proposing_time": 2,
                "start_voting_time": 1,
                "stop_voting_time": 2
            }
        });

        assert_eq!(migrate_network_config(before).unwrap(), after);
    }

    #[test]
    fn test_config_migrations_existing_builder_urls() {
        let before = json!({
            "config": {
                "builder_urls": ["https://test:8080", "https://test:8081"],
                "start_proposing_view": 1,
                "stop_proposing_view": 2,
                "start_voting_view": 1,
                "stop_voting_view": 2,
                "start_proposing_time": 1,
                "stop_proposing_time": 2,
                "start_voting_time": 1,
                "stop_voting_time": 2
            }
        });

        assert_eq!(migrate_network_config(before.clone()).unwrap(), before);
    }

    #[test]
    fn test_config_migrations_add_upgrade_params() {
        let before = json!({
            "config": {
                "builder_urls": ["https://test:8080", "https://test:8081"]
            }
        });
        let after = json!({
            "config": {
                "builder_urls": ["https://test:8080", "https://test:8081"],
                "start_proposing_view": 9007199254740991u64,
                "stop_proposing_view": 0,
                "start_voting_view": 9007199254740991u64,
                "stop_voting_view": 0,
                "start_proposing_time": 9007199254740991u64,
                "stop_proposing_time": 0,
                "start_voting_time": 9007199254740991u64,
                "stop_voting_time": 0
            }
        });

        assert_eq!(migrate_network_config(before).unwrap(), after);
    }

    #[test]
    fn test_config_migrations_existing_upgrade_params() {
        let before = json!({
            "config": {
                "builder_urls": ["https://test:8080", "https://test:8081"],
                "start_proposing_view": 1,
                "stop_proposing_view": 2,
                "start_voting_view": 1,
                "stop_voting_view": 2,
                "start_proposing_time": 1,
                "stop_proposing_time": 2,
                "start_voting_time": 1,
                "stop_voting_time": 2
            }
        });

        assert_eq!(migrate_network_config(before.clone()).unwrap(), before);
    }
}
