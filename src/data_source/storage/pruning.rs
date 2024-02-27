use anyhow::bail;
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct PrunerCfg {
    /// Disk space threshold (in bytes).
    ///
    /// If the disk usage exceeds this threshold, pruning of data starts from
    /// the oldest data and continues until the disk usage falls below `MAX_USAGE`.
    pruning_threshold: Option<u64>,

    /// Minimum data retention period (in seconds).
    ///
    /// Data younger than this is never pruned, regardless of disk usage.
    minimum_retention: Duration,

    /// Target data retention period (in seconds).
    ///
    /// This is the ideal period for which data should be retained. Data younger
    /// than this and older than minimum_retention may be pruned if disk usage exceeds the `pruning_threshold`.
    target_retention: Duration,

    /// Number of blocks to remove in a single pruning operation.
    batch_size: u64,

    /// Maximum disk usage (in basis points).
    ///
    /// Pruning stops once the disk usage falls below this value, even if
    /// some data older than the `minimum_retention` remains. Values range
    /// from 0 (0%) to 10000 (100%).
    max_usage: u16,
    /// Pruning interval
    interval: Duration,
}

#[async_trait]
pub trait PruneStorage: PrunerConfig {
    type Error: Error + Debug + Send + Sync + 'static;

    async fn get_disk_usage(&self) -> Result<u64, Self::Error> {
        Ok(0)
    }
    async fn get_height_by_timestamp(&self, _timestamp: i64) -> Result<Option<u64>, Self::Error> {
        Ok(None)
    }
    async fn prune(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn pruned_height(&self) -> Option<u64> {
        None
    }
}

pub trait PrunerConfig {
    fn pruning_enabled(&self) -> bool {
        false
    }
    fn set_pruning_config(&mut self, _cfg: PrunerCfg) {}
    fn get_pruning_config(&self) -> Option<PrunerCfg> {
        None
    }
}

impl PrunerCfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if let Some(pruning_threshold) = self.pruning_threshold {
            if pruning_threshold == 0 {
                bail!("pruning_threshold must be greater than 0 or set to None")
            }
        }

        if self.max_usage > 10000 {
            bail!("max_usage must be less than or equal to 10000")
        }

        Ok(())
    }

    pub fn with_pruning_threshold(mut self, pruning_threshold: u64) -> Self {
        self.pruning_threshold = Some(pruning_threshold);
        self
    }

    pub fn with_minimum_retention(mut self, minimum_retention: Duration) -> Self {
        self.minimum_retention = minimum_retention;
        self
    }

    pub fn with_target_retention(mut self, target_retention: Duration) -> Self {
        self.target_retention = target_retention;
        self
    }

    pub fn with_batch_size(mut self, batch_size: u64) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn with_max_usage(mut self, max_usage: u16) -> Self {
        self.max_usage = max_usage;
        self
    }

    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn pruning_threshold(&self) -> Option<u64> {
        self.pruning_threshold
    }

    pub fn minimum_retention(&self) -> Duration {
        self.minimum_retention
    }

    pub fn target_retention(&self) -> Duration {
        self.target_retention
    }

    pub fn batch_size(&self) -> u64 {
        self.batch_size
    }

    pub fn max_usage(&self) -> u16 {
        self.max_usage
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }
}

impl Default for PrunerCfg {
    fn default() -> Self {
        Self {
            pruning_threshold: Some(100),
            minimum_retention: Duration::from_secs(0),
            target_retention: Duration::from_secs(30 * 24 * 60 * 60),
            batch_size: 1000,
            max_usage: 8000,
            interval: Duration::from_secs(15 * 60),
        }
    }
}
