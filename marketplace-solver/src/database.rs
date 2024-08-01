use anyhow::Context;
use sqlx::{
    pool::PoolConnection,
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, Error, PgPool, Postgres,
};
use tide_disco::Url;

use crate::{DatabaseOptions, SolverError};

// PgPool is wrapped in an Arc internally so cloning here increments the reference count
#[derive(Clone)]
pub struct PostgresClient(PgPool);

impl PostgresClient {
    pub async fn connect(opts: DatabaseOptions) -> anyhow::Result<Self> {
        let DatabaseOptions {
            url,
            host,
            port,
            db_name,
            username,
            password,
            max_connections,
            acquire_timeout,
            require_ssl,
            migrations,
        } = opts;

        let mut options = PgPoolOptions::new();

        let postgres_url: Url = match url {
            Some(url) => url.parse()?,
            None => {
                let host = host.context("host not provided")?;

                let mut connect_opts = PgConnectOptions::new()
                    .host(&host)
                    .ssl_mode(PgSslMode::Allow);

                if let Some(port) = port {
                    connect_opts = connect_opts.port(port);
                }

                if let Some(username) = username {
                    connect_opts = connect_opts.username(&username);
                }

                if let Some(password) = password {
                    connect_opts = connect_opts.password(&password);
                }

                if let Some(db_name) = db_name {
                    connect_opts = connect_opts.database(&db_name);
                }

                if require_ssl {
                    connect_opts = connect_opts.ssl_mode(PgSslMode::Require)
                }

                connect_opts.to_url_lossy()
            }
        };

        if let Some(max_connections) = max_connections {
            options = options.max_connections(max_connections);
        }

        if let Some(acquire_timeout) = acquire_timeout {
            options = options.acquire_timeout(acquire_timeout);
        }

        let connection = options.connect(postgres_url.as_str()).await?;

        if migrations {
            sqlx::migrate!("./migrations").run(&connection).await?;
        }

        Ok(Self(connection))
    }

    pub fn pool(&self) -> &PgPool {
        &self.0
    }

    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, Error> {
        self.0.acquire().await
    }
}

impl From<sqlx::Error> for SolverError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}

#[cfg(all(any(test, feature = "testing"), not(target_os = "windows")))]
pub mod mock {
    use hotshot_query_service::data_source::sql::testing::TmpDb;

    use super::PostgresClient;
    use crate::DatabaseOptions;

    pub async fn setup_mock_database() -> (TmpDb, PostgresClient) {
        let db: TmpDb = TmpDb::init().await;
        let host = db.host();
        let port = db.port();

        let opts = DatabaseOptions {
            url: None,
            host: Some(host),
            port: Some(port),
            db_name: None,
            username: Some("postgres".to_string()),
            password: Some("password".to_string()),
            max_connections: Some(100),
            acquire_timeout: None,
            require_ssl: false,
            migrations: true,
        };

        // TmpDb will be dropped, which will cause the Docker container to be killed.
        // Therefore, it is returned and kept in scope until needed.
        (
            db,
            PostgresClient::connect(opts)
                .await
                .expect("failed to connect to database"),
        )
    }
}

#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use async_compatibility_layer::logging::setup_logging;

    use crate::database::mock::setup_mock_database;

    #[async_std::test]
    async fn test_database_connection() {
        setup_logging();

        let (tmpdb, client) = setup_mock_database().await;
        let pool = client.pool();

        sqlx::query("SELECT 1;").execute(pool).await.unwrap();

        drop(tmpdb);
    }
}
