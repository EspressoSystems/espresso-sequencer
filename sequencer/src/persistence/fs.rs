use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use async_trait::async_trait;
use clap::Parser;
use std::path::PathBuf;

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
}

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        if !self.config_path().is_file() {
            return Ok(None);
        }
        Ok(Some(NetworkConfig::from_file(
            self.config_path().display().to_string(),
        )?))
    }

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        Ok(cfg.to_file(self.config_path().display().to_string())?)
    }
}
