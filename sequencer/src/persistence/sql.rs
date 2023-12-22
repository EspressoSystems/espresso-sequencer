use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::api::{data_source::SequencerDataSource, sql::DataSource};
use async_trait::async_trait;
use clap::Parser;
use hotshot_query_service::data_source::sql::Query;

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Debug, Default)]
pub struct Options {
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
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    async fn create(self) -> anyhow::Result<Persistence> {
        DataSource::create(self, false).await
    }

    async fn reset(self) -> anyhow::Result<()> {
        DataSource::create(self, true).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
pub type Persistence = DataSource;

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        // Select the most recent config (although there should only be one).
        let Some(row) = self
            .query_opt_static("SELECT config FROM network_config ORDER BY id DESC LIMIT 1")
            .await?
        else {
            return Ok(None);
        };
        let config = row.try_get("config")?;
        Ok(serde_json::from_value(config)?)
    }

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        let json = serde_json::to_value(cfg)?;
        self.transaction()
            .await?
            .execute("INSERT INTO network_config (config) VALUES ($1)", [&json])
            .await?;
        Ok(())
    }
}
