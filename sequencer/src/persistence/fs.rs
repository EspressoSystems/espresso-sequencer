use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Leaf, ValidatedState, ViewNumber};
use anyhow::{anyhow, bail, Context};
use async_trait::async_trait;
use clap::Parser;
use hotshot_types::traits::node_implementation::ConsensusTime;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
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

    async fn save_voted_view(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        // Check if we already have a higher view before writing the new view. Note that this check
        // is not atomic with respect to the subsequent write at the file system level, but this
        // object is the only one which writes to this file, and we have a mutable reference, so
        // this should be safe.
        if let Some(prev) = self.load_voted_view().await? {
            if prev >= view {
                return Ok(());
            }
        }
        fs::write(self.voted_view_path(), view.get_u64().to_le_bytes())?;
        Ok(())
    }

    async fn load_voted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        let path = self.voted_view_path();
        if !path.is_file() {
            return Ok(None);
        }
        let bytes = fs::read(self.voted_view_path())?
            .try_into()
            .map_err(|bytes| anyhow!("malformed voted view file: {bytes:?}"))?;
        Ok(Some(ViewNumber::new(u64::from_le_bytes(bytes))))
    }

    async fn save_anchor_leaf(&mut self, leaf: &Leaf) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(self.anchor_leaf_path())?;

        if file.metadata()?.len() > 0 {
            // Check if we already have a later leaf before writing the new one. Note that this
            // check is not atomic with respect to the subsequent write at the file system level,
            // but this object is the only one which writes to this file, and we have a mutable
            // reference, so this should be safe.
            //
            // The height of the latest saved leaf is in the first 8 bytes of the file.
            let mut height_bytes = [0; 8];
            file.read_exact(&mut height_bytes).context("read height")?;
            let height = u64::from_le_bytes(height_bytes);
            if height >= leaf.get_height() {
                return Ok(());
            }
        }

        // Save the new leaf. First we write its height.
        file.set_len(0).context("truncate")?;
        file.write_all(&leaf.get_height().to_le_bytes())
            .context("write height")?;
        // Now serialize and write out the actual leaf.
        let bytes = bincode::serialize(leaf).context("serialize leaf")?;
        file.write_all(&bytes).context("write leaf")?;

        Ok(())
    }

    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>> {
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

    async fn load_validated_state(&self, _height: u64) -> anyhow::Result<ValidatedState> {
        bail!("state persistence not implemented");
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
