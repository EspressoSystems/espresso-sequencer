//! Persistence on an embedded SQLite file.

use std::time::Duration;

use anyhow::Context as _;
use hotshot_query_service::{data_source::storage::sql::sqlite as qs, include_migrations};

#[expect(
    clippy::duplicate_mod,
    reason = "each backend compiles the shared tree against its own `Db`"
)]
#[path = "../sql.rs"]
mod imp;

pub use imp::*;
use qs::Config;

use super::{Options as SqlOptions, SqliteOptions};

impl BackendOptions for qs::Db {
    fn apply(mut cfg: Config, opt: &SqlOptions) -> anyhow::Result<Config> {
        let path = opt
            .sqlite_options
            .path
            .clone()
            .context("ESPRESSO_NODE_STORAGE_PATH must be set for an embedded database")?;
        if let Some(dir) = path.parent().filter(|dir| !dir.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir)
                .with_context(|| format!("failed to create directory: {dir:?}"))?;
        }

        cfg = cfg.migrations(include_migrations!(
            "$CARGO_MANIFEST_DIR/api/migrations/sqlite"
        ));

        Ok(cfg.db_path(path))
    }
}

impl From<SqliteOptions> for Config {
    fn from(opt: SqliteOptions) -> Self {
        let mut cfg = Config::default();

        if let Some(path) = opt.path {
            cfg = cfg.db_path(path);
        }

        cfg = cfg.max_connections(20);
        cfg = cfg.idle_connection_timeout(Duration::from_secs(120));
        cfg = cfg.connection_timeout(Duration::from_secs(10240));
        cfg = cfg.slow_statement_threshold(Duration::from_secs(2));
        cfg = cfg.statement_timeout(Duration::from_secs(600));
        cfg
    }
}

#[cfg(any(test, feature = "testing"))]
pub(crate) fn tmp_options(db: &qs::testing::TmpDb) -> SqlOptions {
    SqliteOptions {
        path: Some(db.path()),
    }
    .into()
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use espresso_types::traits::{PersistenceOptions as _, SequencerPersistence as _};
    use hotshot_query_service::metrics::PrometheusMetrics;
    use hotshot_types::traits::metrics::Metrics;

    use super::{
        Config, SqlOptions, SqliteOptions,
        imp::{Persistence, read_pragmas, sqlite_probe_dir},
    };
    use crate::persistence::tests::TestablePersistence as _;

    #[test]
    fn embedded_db_without_a_storage_path_is_an_error() {
        let opt = SqlOptions {
            embedded_db: true,
            sqlite_options: SqliteOptions { path: None },
            ..SqlOptions::default()
        };

        let Err(err) = Config::try_from(&opt) else {
            panic!("an embedded database with no storage path was accepted");
        };
        assert!(
            format!("{err:#}").contains("ESPRESSO_NODE_STORAGE_PATH"),
            "{err:#}"
        );
    }

    #[tokio::test]
    async fn read_pragmas_falls_back_to_none_on_query_error() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(":memory:")
            .await
            .unwrap();
        pool.close().await;

        assert!(read_pragmas(&pool).await.is_none());
    }

    #[tokio::test]
    async fn read_pragmas_reads_live_values() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(":memory:")
            .await
            .unwrap();

        let pragmas = read_pragmas(&pool).await.expect("pragmas readable");

        assert_eq!(pragmas.journal_mode, "memory");
        assert_ne!(pragmas.synchronous, "unknown");
        assert!(pragmas.page_size > 0);
    }

    #[test]
    fn sqlite_probe_dir_falls_back_to_cwd_without_a_parent() {
        // `SqliteOptions::default()`'s `path` is empty, whose `parent()` is `Some("")`, not `None`.
        assert_eq!(sqlite_probe_dir(&PathBuf::new()), Path::new("."));
        assert_eq!(sqlite_probe_dir(Path::new("database")), Path::new("."));
        assert_eq!(
            sqlite_probe_dir(Path::new("/var/lib/espresso/sqlite/database")),
            Path::new("/var/lib/espresso/sqlite")
        );
    }

    /// The probe is taken in `create()` and only reaches the exported registry through
    /// `enable_metrics`, which `init_node` calls.
    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_storage_probe_reaches_exported_registry() {
        let tmp = Persistence::tmp_storage().await;
        let mut persistence = Persistence::options(&tmp).create().await.unwrap();

        // Dropping WAL from `sqlite_options()` should fail this assertion.
        assert_eq!(
            persistence
                .probe
                .as_ref()
                .and_then(|probe| probe.pragmas.as_ref())
                .map(|pragmas| pragmas.journal_mode.as_str()),
            Some("wal")
        );

        let metrics = PrometheusMetrics::default();
        persistence.enable_metrics(&*Metrics::subgroup(&metrics, "consensus".to_string()));

        let exported = metrics.export().unwrap();
        assert!(exported.contains("consensus_disk_info"), "{exported}");
        assert!(exported.contains("backend=\"sqlite\""), "{exported}");
        assert!(
            exported.contains("consensus_disk_fsync_micros"),
            "{exported}"
        );
    }
}
