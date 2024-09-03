use std::time::Duration;

use clap::Parser;
use espresso_types::parse_duration;
use tide_disco::Url;

use crate::database::PostgresClient;

#[derive(Parser)]
pub struct Options {
    /// Port to run the server on.
    #[clap(short, long, env = "ESPRESSO_MARKETPLACE_SOLVER_API_PORT")]
    pub solver_api_port: u16,

    /// Hotshot events service api URL
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_HOTSHOT_EVENT_API_URL")]
    pub events_api_url: Url,

    #[clap(flatten)]
    pub database_options: DatabaseOptions,
}

/// Arguments for establishing a database connection
#[derive(Clone, Debug, Parser)]
pub struct DatabaseOptions {
    // Postgres URL connection string
    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_URL")]
    pub url: Option<String>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_HOST")]
    pub host: Option<String>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_PORT")]
    pub port: Option<u16>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_DATABASE_NAME")]
    pub db_name: Option<String>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_USER")]
    pub username: Option<String>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_PASSWORD")]
    pub password: Option<String>,

    #[clap(long, env = "MARKETPLACE_SOLVER_POSTGRES_MAX_CONNECTIONS")]
    pub max_connections: Option<u32>,

    #[clap(long,value_parser = parse_duration, env = "MARKETPLACE_SOLVER_DATABASE_ACQUIRE_TIMEOUT")]
    pub acquire_timeout: Option<Duration>,

    #[clap(
        long,
        env = "MARKETPLACE_SOLVER_DATABASE_REQUIRE_SSL",
        default_value_t = false
    )]
    pub require_ssl: bool,

    #[clap(
        long,
        env = "MARKETPLACE_SOLVER_DATABASE_RUN_MIGRATIONS",
        default_value_t = true
    )]
    pub migrations: bool,
}

impl DatabaseOptions {
    pub async fn connect(self) -> anyhow::Result<PostgresClient> {
        PostgresClient::connect(self).await
    }
}
