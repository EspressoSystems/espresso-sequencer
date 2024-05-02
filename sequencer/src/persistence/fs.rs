use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Leaf, SeqTypes, ViewNumber};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use clap::Parser;

use hotshot_types::{
    data::{DAProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::node_implementation::ConsensusTime,
    vote::HasViewNumber,
};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

/// Options for file system backed persistence.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Storage path for persistent data.
    #[clap(long, env = "ESPRESSO_SEQUENCER_STORAGE_PATH")]
    pub path: PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    async fn create(self) -> anyhow::Result<Persistence> {
        Ok(Persistence(self.path))
    }

    async fn reset(self) -> anyhow::Result<()> {
        todo!()
    }
}

/// File system backed persistence.
#[derive(Clone, Debug)]
pub struct Persistence(PathBuf);

impl Persistence {
    fn config_path(&self) -> PathBuf {
        self.0.join("hotshot.cfg")
    }

    fn voted_view_path(&self) -> PathBuf {
        self.0.join("highest_voted_view")
    }

    fn anchor_leaf_path(&self) -> PathBuf {
        self.0.join("anchor_leaf")
    }

    fn vid_dir_path(&self) -> PathBuf {
        self.0.join("vid")
    }

    fn da_dir_path(&self) -> PathBuf {
        self.0.join("da")
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
        Ok(Some(NetworkConfig::from_file(path.display().to_string())?))
    }

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        let path = self.config_path();
        tracing::info!("saving config to {}", path.display());
        Ok(cfg.to_file(path.display().to_string())?)
    }

    async fn collect_garbage(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        let view_number = view.get_u64();

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
        delete_files(self.vid_dir_path())
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
                if height >= leaf.get_height() {
                    tracing::warn!(
                        saved_height = height,
                        new_height = leaf.get_height(),
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
                file.write_all(&leaf.get_height().to_le_bytes())
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

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DAProposal<SeqTypes>>>> {
        let dir_path = self.da_dir_path();

        let file_path = dir_path
            .join(view.get_u64().to_string())
            .with_extension("txt");

        if !file_path.exists() {
            return Ok(None);
        }

        let da_bytes = fs::read(file_path)?;

        let da_proposal: Proposal<SeqTypes, DAProposal<SeqTypes>> =
            bincode::deserialize(&da_bytes)?;
        Ok(Some(da_proposal))
    }

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        let dir_path = self.vid_dir_path();

        let file_path = dir_path
            .join(view.get_u64().to_string())
            .with_extension("txt");

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
        let view_number = proposal.data.get_view_number().get_u64();
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
        proposal: &Proposal<SeqTypes, DAProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        let view_number = proposal.data.get_view_number().get_u64();
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
                file.write_all(&view.get_u64().to_le_bytes())?;
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod testing {
    use super::super::testing::TestablePersistence;
    use super::*;
    use tempfile::TempDir;

    #[async_trait]
    impl TestablePersistence for Persistence {
        type Storage = TempDir;

        async fn tmp_storage() -> Self::Storage {
            TempDir::new().unwrap()
        }

        async fn connect(storage: &Self::Storage) -> Self {
            Persistence(storage.path().into())
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::super::persistence_tests;
    use super::Persistence;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_persistence_tests!(Persistence);
}
