use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Leaf, ValidatedState, ViewNumber};
use anyhow::bail;
use async_trait::async_trait;
use clap::Parser;
use hotshot_query_service::data_source::{
    storage::sql::{include_migrations, postgres::types::ToSql, Config, Query, SqlStorage},
    VersionedDataSource,
};
use hotshot_types::traits::node_implementation::ConsensusTime;

/// Options for Postgres-backed persistence.
#[derive(Parser, Clone, Debug, Default)]
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
    pub uri: Option<String>,

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
        Ok(cfg)
    }
}

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    async fn create(self) -> anyhow::Result<Persistence> {
        SqlStorage::connect(self.try_into()?).await
    }

    async fn reset(self) -> anyhow::Result<()> {
        SqlStorage::connect(Config::try_from(self)?.reset_schema()).await?;
        Ok(())
    }
}

/// Postgres-backed persistence.
pub type Persistence = SqlStorage;

#[async_trait]
impl SequencerPersistence for Persistence {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        tracing::info!("loading config from Postgres");
        // Select the most recent config (although there should only be one).
        let Some(row) = self
            .query_opt_static("SELECT config FROM network_config ORDER BY id DESC LIMIT 1")
            .await?
        else {
            tracing::info!("config not found");
            return Ok(None);
        };
        let config = row.try_get("config")?;
        Ok(serde_json::from_value(config)?)
    }

    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        tracing::info!("saving config to Postgres");
        let json = serde_json::to_value(cfg)?;
        self.transaction()
            .await?
            .execute_one_with_retries("INSERT INTO network_config (config) VALUES ($1)", [&json])
            .await?;
        self.commit().await?;
        Ok(())
    }

    async fn save_voted_view(&mut self, view: ViewNumber) -> anyhow::Result<()> {
        let stmt = "
            INSERT INTO highest_voted_view (id, view) VALUES (0, $1)
            ON CONFLICT (id) DO UPDATE SET view = GREATEST(highest_voted_view.view, excluded.view)
        ";
        self.transaction()
            .await?
            .execute_one_with_retries(stmt, [view.get_u64() as i64])
            .await?;
        self.commit().await?;
        Ok(())
    }

    async fn save_anchor_leaf(&mut self, leaf: &Leaf) -> anyhow::Result<()> {
        let stmt = "
            INSERT INTO anchor_leaf (id, height, leaf) VALUES (0, $1, $2)
            ON CONFLICT (id) DO UPDATE SET (height, leaf) = ROW (
                GREATEST(anchor_leaf.height, excluded.height),
                CASE
                    WHEN excluded.height > anchor_leaf.height THEN excluded.leaf
                    ELSE anchor_leaf.leaf
                END
            )
        ";
        let leaf_bytes = bincode::serialize(leaf)?;
        self.transaction()
            .await?
            .execute_one_with_retries(
                stmt,
                [
                    sql_param(&(leaf.get_height() as i64)),
                    sql_param(&leaf_bytes),
                ],
            )
            .await?;
        self.commit().await?;
        Ok(())
    }

    async fn load_voted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(self
            .query_opt_static("SELECT view FROM highest_voted_view WHERE id = 0")
            .await?
            .map(|row| {
                let view: i64 = row.get("view");
                ViewNumber::new(view as u64)
            }))
    }

    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>> {
        self.query_opt_static("SELECT leaf FROM anchor_leaf WHERE id = 0")
            .await?
            .map(|row| {
                let bytes: Vec<u8> = row.get("leaf");
                Ok(bincode::deserialize(&bytes)?)
            })
            .transpose()
    }

    async fn load_validated_state(&self, _height: u64) -> anyhow::Result<ValidatedState> {
        bail!("state persistence not implemented");
    }
}

fn sql_param<T: ToSql + Sync>(param: &T) -> &(dyn ToSql + Sync) {
    param
}

#[cfg(test)]
mod testing {
    use super::super::testing::TestablePersistence;
    use super::*;
    use hotshot_query_service::data_source::storage::sql::testing::TmpDb;

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
    use super::super::persistence_tests;
    use super::Persistence;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_persistence_tests!(Persistence);
}
