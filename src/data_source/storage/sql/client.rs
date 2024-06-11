// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use crate::{task::BackgroundTask, QueryError, QueryResult};
use async_std::{
    net::ToSocketAddrs,
    sync::{Arc, RwLock, RwLockUpgradableReadGuard},
    task::sleep,
};
use futures::{
    channel::mpsc,
    stream::{Stream, StreamExt, TryStreamExt},
    task::{Context, Poll},
    AsyncRead, AsyncWrite,
};
use postgres_native_tls::TlsConnector;
use refinery::{Migration, Report, Runner};
use std::{pin::Pin, time::Duration};
use tokio_postgres::{config::Host, types::BorrowToSql, NoTls, Row, ToStatement};

#[derive(Clone, Debug)]
pub(super) struct Config {
    pub(super) pgcfg: tokio_postgres::Config,
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) tls: bool,
    pub(super) schema: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pgcfg: Default::default(),
            host: "localhost".into(),
            port: 5432,
            schema: "hotshot".into(),
            tls: false,
        }
    }
}

impl From<tokio_postgres::Config> for Config {
    fn from(pgcfg: tokio_postgres::Config) -> Self {
        // We connect via TCP manually, without using the host and port from pgcfg. So we need to
        // pull those out of pgcfg if they have been specified, to override the defaults.
        let host = match pgcfg.get_hosts().first() {
            Some(Host::Tcp(host)) => host.to_string(),
            _ => "localhost".into(),
        };
        let port = *pgcfg.get_ports().first().unwrap_or(&5432);

        Self {
            pgcfg,
            host,
            port,
            ..Default::default()
        }
    }
}

/// A Postgres client.
///
/// This is a wrapper around [`tokio_postgres::Client`] which transparently handles transactions and
/// reconnections if the connection is closed.
#[derive(Debug)]
pub struct Client {
    // The underlying connection is protected by an RwLock, so that we can reset it if necessary.
    conn: Arc<RwLock<Connection>>,
    // The connection is always reset from a dedicated task. There are two reasons for this:
    // * Having a dedicated party as the only one to modify `conn` ensures that we never have a
    //   queue of writers waiting for a lock, even if multiple tasks discover that a reset is needed
    //   at the same time.
    // * Resetting the connection may take a long time, especially if it was closed because the
    //   server went down. We don't want to block client requests indefinitely if this happens; it
    //   is actually better if they fail fast, and the client can figure out how to retry or
    //   recover. We dedicate this slow operation to a background task, and client requests will
    //   automatically start working again once the connection does get reset.
    _resetter: BackgroundTask,
    // Tasks send on this channel to request the resetter task to reset the connection. The reset
    // request includes the unique ID of the connection to reset, so that duplicate requests to
    // reset the same connection can easily be ignored.
    request_reset: mpsc::Sender<usize>,
}

impl Client {
    pub(super) async fn new(cfg: Config) -> anyhow::Result<Self> {
        let conn = Arc::new(RwLock::new(Connection::new(&cfg).await?));

        // Create a channel to communicate with the resetter task. It has a bound of 1 message
        // because reset requests for the same connection are idempotent: we only need to wake the
        // resetter task once at a time.
        let (request_reset, recv_reset_request) = mpsc::channel(1);

        let resetter = BackgroundTask::spawn(
            "postgres client resetter",
            resetter(cfg, recv_reset_request, conn.clone()),
        );
        Ok(Self {
            conn,
            _resetter: resetter,
            request_reset,
        })
    }

    pub(super) async fn query_raw<T, I>(
        &self,
        stmt: &T,
        params: I,
    ) -> QueryResult<impl 'static + Stream<Item = QueryResult<Row>>>
    where
        T: ?Sized + ToStatement,
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        let conn = self.conn.read().await;
        Ok(self
            .reset_if_closed(conn.id, conn.query.query_raw(stmt, params).await)?
            .map_err(postgres_err))
    }

    pub(super) async fn query_opt<T, I>(&self, stmt: &T, params: I) -> QueryResult<Option<Row>>
    where
        T: ?Sized + ToStatement,
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        std::pin::pin!(self.query_raw(stmt, params).await?)
            .try_next()
            .await
    }

    pub(super) async fn batch_execute(&self, stmt: &str) -> QueryResult<()> {
        let conn = self.conn.read().await;
        self.reset_if_closed(conn.id, conn.query.batch_execute(stmt).await)
    }

    pub(super) async fn execute_raw<T, I>(&self, stmt: &T, params: I) -> QueryResult<u64>
    where
        T: ?Sized + ToStatement,
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        let conn = self.conn.read().await;
        self.reset_if_closed(conn.id, conn.query.execute_raw(stmt, params).await)
    }

    pub(super) async fn begin(&self) -> QueryResult<()> {
        // Take an upgradable read lock so we can check if there is already a transaction without
        // blocking unnecessarily.
        let conn = self.conn.upgradable_read().await;
        if conn.tx_in_progress {
            // If we are already in a transaction there is nothing to do.
            return Ok(());
        }

        // Upgrade to a write lock and begin a transaction.
        let mut conn = RwLockUpgradableReadGuard::upgrade(conn).await;
        self.reset_if_closed(conn.id, conn.query.batch_execute("BEGIN").await)?;
        conn.tx_in_progress = true;
        Ok(())
    }

    pub(super) async fn commit(&self) -> QueryResult<()> {
        let mut conn = self.conn.write().await;
        if !conn.tx_in_progress {
            return Ok(());
        }
        self.reset_if_closed(conn.id, conn.query.batch_execute("COMMIT").await)?;
        conn.tx_in_progress = false;
        Ok(())
    }

    pub(super) async fn revert(&self) {
        let mut conn = self.conn.write().await;
        if !conn.tx_in_progress {
            return;
        }
        match conn.query.batch_execute("ROLLBACK").await {
            Ok(()) => {
                conn.tx_in_progress = false;
            }
            Err(err) => {
                if err.is_closed() {
                    conn.tx_in_progress = false;
                    if let Some((id, client, task)) = conn.next.take() {
                        // It is possible that the resetter task was already triggered by one of the
                        // statements within the failed transaction, and has prepared an new
                        // connection for us, but was waiting for the transaction to finalize before
                        // enabling the new connection. Now that the transaction is done we can
                        // start using the new connection.
                        conn.reset(id, client, task);
                    } else {
                        // If there is no new connection available, trigger the reset task to make
                        // one.
                        self.request_reset(conn.id);
                    }

                    // We can return successfully here, because having the connection close without
                    // committing the transaction is just as good as reverting it.
                } else {
                    // If we're trying to roll back a transaction, something has already gone wrong
                    // and we're trying to recover. If we're unable to revert the changes and
                    // recover, all we can do is panic.
                    panic!("failed to revert bad transaction: {err:#}");
                }
            }
        }
    }

    pub(super) async fn last_applied_migration(
        &self,
        runner: &Runner,
    ) -> anyhow::Result<Option<Migration>> {
        let mut conn = self.conn.write().await;
        Ok(runner
            .get_last_applied_migration_async(&mut conn.query)
            .await?)
    }

    pub(super) async fn run_migrations(&self, runner: &Runner) -> Result<Report, refinery::Error> {
        let mut conn = self.conn.write().await;
        runner.run_async(&mut conn.query).await
    }

    fn request_reset(&self, id: usize) {
        tracing::warn!(id, "postgres connection closed; requesting reset");

        // Wake up the resetter task. Ignore errors on this send; they mean one of two things:
        // * the channel is saturated, i.e. somebody else has already sent a reset request and the
        //   task will get to it soon
        // * the channel is closed, i.e. the reset task exited, and should have already logged why
        self.request_reset.clone().try_send(id).ok();
    }

    fn reset_if_closed<T>(
        &self,
        id: usize,
        res: Result<T, tokio_postgres::Error>,
    ) -> QueryResult<T> {
        match res {
            Ok(t) => Ok(t),
            Err(err) => {
                if err.is_closed() {
                    self.request_reset(id);
                }
                Err(postgres_err(err))
            }
        }
    }
}

#[derive(Debug)]
struct Connection {
    id: usize,
    query: tokio_postgres::Client,
    conn: BackgroundTask,
    next: Option<(usize, tokio_postgres::Client, BackgroundTask)>,
    tx_in_progress: bool,
}

impl Connection {
    async fn new(cfg: &Config) -> anyhow::Result<Self> {
        let id = 0;
        let (query, conn) = connect(id, cfg).await?;
        Ok(Self {
            id,
            query,
            conn,
            next: None,
            tx_in_progress: false,
        })
    }

    fn reset(&mut self, id: usize, client: tokio_postgres::Client, conn: BackgroundTask) {
        tracing::info!(
            old_id = self.id,
            new_id = id,
            "switching to new postgres connection"
        );
        self.query = client;
        self.conn = conn;
        self.id = id;
    }
}

const CONNECTION_RESET_INTERVAL: Duration = Duration::from_secs(5);

async fn resetter(
    cfg: Config,
    mut recv_reset_request: mpsc::Receiver<usize>,
    conn: Arc<RwLock<Connection>>,
) {
    let mut current_id = { conn.read().await.id };
    loop {
        let Some(id) = recv_reset_request.next().await else {
            tracing::warn!("postgres reset request channel ended, resetter task exiting");
            return;
        };
        if id < current_id {
            // This is a request to reset an old connection which we have already done.
            tracing::debug!(id, current_id, "ignoring request to reset old connection");
            continue;
        }

        tracing::warn!(current_id, "resetting postgres connection");
        current_id += 1;
        let (new_client, new_task) = loop {
            match connect(current_id, &cfg).await {
                Ok(conn) => break conn,
                Err(err) => {
                    tracing::warn!("unable to reestablish connection: {err:#}");
                    sleep(CONNECTION_RESET_INTERVAL).await;
                }
            }
        };

        let mut conn = conn.write().await;
        if conn.tx_in_progress {
            // Save the new connection but don't enable it yet. We don't want the remainder of this
            // broken transaction to start making statements in the new connection; outside the
            // context of a transaction. When the failed transaction tries to commit or revert, it
            // will automatically switch over to the new connection.
            conn.next = Some((current_id, new_client, new_task));
        } else {
            conn.reset(current_id, new_client, new_task);
        }
    }
}

async fn connect(
    id: usize,
    cfg: &Config,
) -> anyhow::Result<(tokio_postgres::Client, BackgroundTask)> {
    let tcp = TcpStream::connect((cfg.host.as_str(), cfg.port)).await?;
    let task_id = format!("postgres connection {id}");
    let (client, conn) = if cfg.tls {
        let tls = TlsConnector::new(native_tls::TlsConnector::new()?, cfg.host.as_str());
        let (client, conn) = cfg.pgcfg.connect_raw(tcp, tls).await?;
        (client, BackgroundTask::spawn(task_id, conn))
    } else {
        let (client, conn) = cfg.pgcfg.connect_raw(tcp, NoTls).await?;
        (client, BackgroundTask::spawn(task_id, conn))
    };
    client
        .batch_execute(&format!("CREATE SCHEMA IF NOT EXISTS {}", cfg.schema))
        .await?;
    client
        .batch_execute(&format!("SET search_path TO {}", cfg.schema))
        .await?;
    Ok((client, conn))
}

// tokio-postgres is written in terms of the tokio AsyncRead/AsyncWrite traits. However, these
// traits do not require any specifics of the tokio runtime. Thus we can implement them using the
// async_std TcpStream type, and have a stream which is compatible with tokio-postgres but will run
// on the async_std executor.
//
// To avoid orphan impls, we wrap this tream in a new type.
struct TcpStream(async_std::net::TcpStream);

impl TcpStream {
    async fn connect<A: ToSocketAddrs>(addrs: A) -> anyhow::Result<Self> {
        Ok(Self(async_std::net::TcpStream::connect(addrs).await?))
    }
}

impl tokio::io::AsyncRead for TcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // tokio uses this hyper-optimized `ReadBuf` construct, where there is a filled portion, an
        // unfilled portion where we append new data, and the unfilled portion of the buffer need
        // not even be initialized. However the async_std implementation we're delegating to just
        // expects a normal `&mut [u8]` buffer which is entirely unfilled. To simplify the
        // conversion, we will abandon the uninitialized buffer optimization and force
        // initialization of the entire buffer, resulting in a plain old `&mut [u8]` representing
        // the unfilled portion. But first, we need to grab the length of the filled region so we
        // can increment it after we read new data from async_std.
        let filled = buf.filled().len();

        // Initialize the buffer and get a slice of the unfilled region. This operation is free
        // after the first time it is called, so we don't need to worry about maintaining state
        // between subsequent calls to `poll_read`.
        let unfilled = buf.initialize_unfilled();

        // Read data into the unfilled portion of the buffer.
        match Pin::new(&mut self.0).poll_read(cx, unfilled) {
            Poll::Ready(Ok(bytes_read)) => {
                // After the read completes, the first `bytes_read` of `unfilled` have now been
                // filled. Increment the `filled` cursor within the `ReadBuf` to account for this.
                buf.set_filled(filled + bytes_read);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl tokio::io::AsyncWrite for TcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

fn postgres_err(err: tokio_postgres::Error) -> QueryError {
    QueryError::Error {
        message: format!("postgres error: {err:#}"),
    }
}

#[cfg(test)]
mod test {
    use super::super::testing::TmpDb;
    use super::*;
    use crate::testing::setup_test;
    use futures::stream::TryStreamExt;

    #[async_std::test]
    async fn test_connection_closed_tx() {
        setup_test();

        let mut db = TmpDb::persistent().await;
        let client = Client::new(db.config().client_config).await.unwrap();

        let no_params: [i32; 0] = [];

        tracing::info!("initialize database");
        client
            .batch_execute("CREATE TABLE test (key SERIAL PRIMARY KEY, value INTEGER)")
            .await
            .unwrap();
        let modified = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [1, 2])
            .await
            .unwrap();
        assert_eq!(modified, 2);

        let get_values = || async {
            let rows: Vec<_> = client
                .query_raw("SELECT value FROM test", no_params)
                .await
                .unwrap()
                .map_ok(|r| r.get::<_, i32>(0))
                .try_collect()
                .await
                .unwrap();
            rows
        };
        assert_eq!(get_values().await, [1, 2]);

        // Start a transaction.
        tracing::info!("begin transaction");
        client.begin().await.unwrap();

        // Initially, stuff works.
        tracing::info!("query in transaction");
        let modified = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [3, 4])
            .await
            .unwrap();
        assert_eq!(modified, 2);
        assert_eq!(get_values().await, [1, 2, 3, 4]);

        // Shut down the database.
        db.stop();

        // Subsequent statements and queries within the transaction fail.
        tracing::info!("query with DB shut down");
        let err = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [5, 6])
            .await
            .unwrap_err();
        tracing::info!(%err, "insert failed as expected");
        client
            .query_raw("SELECT value FROM test", no_params)
            .await
            .map(|_| ())
            .unwrap_err();
        tracing::info!(%err, "select failed as expected");

        // Even if we restart the database, the transaction is still dead.
        db.start().await;
        // Give the resetter task time to realize the database has been restarted.
        sleep(2 * CONNECTION_RESET_INTERVAL).await;

        tracing::info!("query in aborted transaction");
        let err = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [7, 8])
            .await
            .unwrap_err();
        tracing::info!(%err, "insert failed as expected");
        let err = client
            .query_raw("SELECT value FROM test", no_params)
            .await
            .map(|_| ())
            .unwrap_err();
        tracing::info!(%err, "select failed as expected");

        // We can't commit the transaction, but we _can_ still revert.
        tracing::info!("finalize aborted transaction");
        let err = client.commit().await.unwrap_err();
        tracing::info!(%err, "commit failed as expected");
        client.revert().await;

        // After reverting, things should work again!
        tracing::info!("query with new connection");
        assert_eq!(get_values().await, [1, 2]);

        // Do a successful transaction with the new connection.
        tracing::info!("transaction with new connection");
        client.begin().await.unwrap();
        let modified = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [9, 10])
            .await
            .unwrap();
        assert_eq!(modified, 2);
        assert_eq!(get_values().await, [1, 2, 9, 10]);
    }

    #[async_std::test]
    async fn test_connection_closed_no_tx() {
        setup_test();

        let mut db = TmpDb::persistent().await;
        let client = Client::new(db.config().client_config).await.unwrap();

        let no_params: [i32; 0] = [];

        tracing::info!("initialize database");
        client
            .batch_execute("CREATE TABLE test (key SERIAL PRIMARY KEY, value INTEGER)")
            .await
            .unwrap();
        let modified = client
            .execute_raw("INSERT INTO test (value) VALUES ($1), ($2)", [1, 2])
            .await
            .unwrap();
        assert_eq!(modified, 2);

        let rows: Vec<_> = client
            .query_raw("SELECT value FROM test", no_params)
            .await
            .unwrap()
            .map_ok(|r| r.get::<_, i32>(0))
            .try_collect()
            .await
            .unwrap();
        assert_eq!(rows, [1, 2]);

        db.stop();
        let err = client
            .query_raw("SELECT value FROM test", no_params)
            .await
            .map(|_| ())
            .unwrap_err();
        tracing::info!(%err, "select while DB down failed as expected");

        db.start().await;

        // It might take a while but eventually queries will start succeeding again.
        let rows = loop {
            match client.query_raw("SELECT value FROM test", no_params).await {
                Ok(rows) => break rows,
                Err(err) => {
                    tracing::info!(%err, "connection still down");
                    sleep(Duration::from_secs(1)).await;
                }
            }
        };
        let rows: Vec<_> = rows
            .map_ok(|r| r.get::<_, i32>(0))
            .try_collect()
            .await
            .unwrap();
        assert_eq!(rows, [1, 2]);
    }
}
