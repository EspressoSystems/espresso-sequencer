use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use clap::Parser;
use espresso_types::{
    traits::{PersistenceOptions, SequencerPersistence},
    Leaf, NetworkConfig, SeqTypes,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::node_implementation::ConsensusTime,
    utils::View,
    vote::HasViewNumber,
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
            path: self.path,
            store_undecided_state: self.store_undecided_state,
        })
    }

    async fn reset(self) -> anyhow::Result<()> {
        todo!()
    }
}

/// File system backed persistence.
#[derive(Clone, Debug)]
pub struct Persistence {
    path: PathBuf,
    store_undecided_state: bool,
}

impl Persistence {
    fn config_path(&self) -> PathBuf {
        self.path.join("hotshot.cfg")
    }

    fn voted_view_path(&self) -> PathBuf {
        self.path.join("highest_voted_view")
    }

    fn anchor_leaf_path(&self) -> PathBuf {
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
}

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        let path = self.config_path();
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

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        let path = self.config_path();
        tracing::info!("saving config to {}", path.display());
        Ok(cfg.to_file(path.display().to_string())?)
    }

    async fn collect_garbage(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        let view_number = view.u64();

        let delete_files = |dir_path: PathBuf| -> anyhow::Result<()> {
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

        delete_files(self.da_dir_path())?;
        delete_files(self.vid_dir_path())?;
        delete_files(self.quorum_proposals_dir_path())
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        let path = self.voted_view_path();
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(self.voted_view_path())?
            .try_into()
            .map_err(|bytes| anyhow!("malformed voted view file: {bytes:?}"))?;
        Ok(Some(ViewNumber::new(u64::from_le_bytes(bytes))))
    }

    async fn save_anchor_leaf(
        &mut self,
        leaf: &Leaf,
        qc: &QuorumCertificate<SeqTypes>,
    ) -> anyhow::Result<()> {
        self.replace(
            &self.anchor_leaf_path(),
            |mut file| {
                // Check if we already have a later leaf before writing the new one. The height of
                // the latest saved leaf is in the first 8 bytes of the file.
                if file.metadata()?.len() < 8 {
                    // This shouldn't happen, but if there is an existing file smaller than 8 bytes,
                    // it is not encoding a valid height, and we want to proceed with the swap.
                    tracing::warn!("anchor leaf file smaller than 8 bytes will be replaced");
                    return Ok(true);
                }
                let mut height_bytes = [0; 8];
                file.read_exact(&mut height_bytes).context("read height")?;
                let height = u64::from_le_bytes(height_bytes);
                if height >= leaf.height() {
                    tracing::warn!(
                        saved_height = height,
                        new_height = leaf.height(),
                        "not writing anchor leaf because saved leaf has newer height",
                    );
                    return Ok(false);
                }

                // The existing leaf is older than the new leaf (this is the common case). Proceed
                // with the swap.
                Ok(true)
            },
            |mut file| {
                // Save the new leaf. First we write the height.
                file.write_all(&leaf.height().to_le_bytes())
                    .context("write height")?;
                // Now serialize and write out the actual leaf and its corresponding QC.
                let bytes = bincode::serialize(&(leaf, qc)).context("serialize leaf")?;
                file.write_all(&bytes).context("write leaf")?;
                Ok(())
            },
        )
    }

    async fn load_anchor_leaf(
        &self,
    ) -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>> {
        let path = self.anchor_leaf_path();
        if !path.is_file() {
            return Ok(None);
        }
        let mut file = File::open(path)?;

        // The first 8 bytes just contain the height of the leaf. We can skip this.
        file.seek(SeekFrom::Start(8)).context("seek")?;
        let bytes = file
            .bytes()
            .collect::<Result<Vec<_>, _>>()
            .context("read")?;
        Ok(Some(bincode::deserialize(&bytes).context("deserialize")?))
    }

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        let path = self.undecided_state_path();
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

    async fn load_vid_share(
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

    async fn append_vid(
        &mut self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.view_number().u64();
        let dir_path = self.vid_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create vid dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        self.replace(
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
        &mut self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.view_number().u64();
        let dir_path = self.da_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create da dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        self.replace(
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
    async fn record_action(
        &mut self,
        view: ViewNumber,
        _action: HotShotAction,
    ) -> anyhow::Result<()> {
        self.replace(
            &self.voted_view_path(),
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
        &mut self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        if !self.store_undecided_state {
            return Ok(());
        }

        self.replace(
            &self.undecided_state_path(),
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
        &mut self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.view_number().u64();
        let dir_path = self.quorum_proposals_dir_path();

        fs::create_dir_all(dir_path.clone()).context("failed to create proposals dir")?;

        let file_path = dir_path.join(view_number.to_string()).with_extension("txt");
        self.replace(
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
    ) -> anyhow::Result<Option<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>>>
    {
        // First, get the proposal directory.
        let dir_path = self.quorum_proposals_dir_path();

        // Then, we want to get the entries in this directory since they'll be the
        // key/value pairs for our map.
        let files: Vec<fs::DirEntry> = fs::read_dir(dir_path.clone())?
            .filter_map(|entry| {
                entry
                    .ok()
                    .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            })
            .collect();

        // Do we have any entries?
        if files.is_empty() {
            // Don't bother continuing if we don't have any data.
            return Ok(None);
        }

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

        Ok(Some(map))
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
