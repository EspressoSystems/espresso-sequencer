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

//! SQL storage, with one module per database backend.
//!
//! [`postgres`] and [`sqlite`] expose the same API over different databases, and a binary can link
//! both and choose one at runtime. Items that do not depend on the backend live here.

#![cfg(feature = "sql-data-source")]
use std::{
    cmp::min,
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use rand::Rng;
use tokio::time::sleep;

pub extern crate sqlx;

pub use anyhow::Error;
pub use include_dir::include_dir;
pub use refinery::Migration;

// This needs to be reexported so that we can reference it by absolute path relative to this crate
// in the expansion of `include_migrations`, even when `include_migrations` is invoked from another
// crate which doesn't have `include_dir` as a dependency.
pub use crate::include_migrations;

pub mod postgres;
pub mod sqlite;

/// Embed migrations from the given directory into the current binary for PostgreSQL or SQLite.
///
/// The macro invocation `include_migrations!(path)` evaluates to an expression of type `impl
/// Iterator<Item = Migration>`. Each migration must be a text file which is an immediate child of
/// `path`, and there must be no non-migration files in `path`. The migration files must have names
/// of the form `V${version}__${name}.sql`, where `version` is a positive integer indicating how the
/// migration is to be ordered relative to other migrations, and `name` is a descriptive name for
/// the migration.
///
/// `path` should be an absolute path. It is possible to give a path relative to the root of the
/// invoking crate by using environment variable expansions and the `CARGO_MANIFEST_DIR` environment
/// variable.
///
/// As an example, this is the invocation used to load the default migrations from the
/// `hotshot-query-service` crate. The migrations are located in a directory called `migrations` at
/// - PostgreSQL migrations are in `/migrations/postgres`.
/// - SQLite migrations are in `/migrations/sqlite`.
///
/// ```
/// # use hotshot_query_service::data_source::sql::{include_migrations, Migration};
/// let mut migrations: Vec<Migration> =
///     include_migrations!("$CARGO_MANIFEST_DIR/migrations/postgres").collect();
///
/// migrations.sort();
/// assert_eq!(migrations[0].version(), 10);
/// assert_eq!(migrations[0].name(), "init_schema");
/// ```
///
/// Note that a similar macro is available from Refinery:
/// [embed_migrations](https://docs.rs/refinery/0.8.11/refinery/macro.embed_migrations.html). This
/// macro differs in that it evaluates to an iterator of [migrations](Migration), making it an
/// expression macro, while `embed_migrations` is a statement macro that defines a module which
/// provides access to the embedded migrations only indirectly via a
/// [`Runner`](https://docs.rs/refinery/0.8.11/refinery/struct.Runner.html). The direct access to
/// migrations provided by [`include_migrations`] makes this macro easier to use with
/// [`postgres::Config::migrations`], for combining custom migrations with
/// [`postgres::default_migrations`].
#[macro_export]
macro_rules! include_migrations {
    ($dir:tt) => {
        $crate::data_source::storage::sql::include_dir!($dir)
            .files()
            .map(|file| {
                let path = file.path();
                let name = path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or_else(|| {
                        panic!(
                            "migration file {} must have a non-empty UTF-8 name",
                            path.display()
                        )
                    });
                let sql = file
                    .contents_utf8()
                    .unwrap_or_else(|| panic!("migration file {name} must use UTF-8 encoding"));
                $crate::data_source::storage::sql::Migration::unapplied(name, sql)
                    .expect("invalid migration")
            })
    };
}

/// When set to `true`, read transactions begin with
/// `SET TRANSACTION ISOLATION LEVEL SERIALIZABLE, READ ONLY` (no `DEFERRABLE`),
/// so they start immediately instead of waiting for a safe serializable snapshot.
static NO_DEFERRABLE_ON_READ: AtomicBool = AtomicBool::new(true);

/// Configure whether read transactions on Postgres should omit `DEFERRABLE`.
///
/// When `true`, `Read::begin` issues `SET TRANSACTION ISOLATION LEVEL SERIALIZABLE, READ ONLY`
/// (no `DEFERRABLE`) so the transaction starts immediately rather than waiting for a safe
/// serializable snapshot. Default: `true`. Call this once at startup based on the operator's
/// chosen configuration. SQLite ignores it.
pub fn set_no_deferrable_on_read(value: bool) {
    NO_DEFERRABLE_ON_READ.store(value, Ordering::Relaxed);
}

#[derive(PartialEq)]
pub enum StorageConnectionType {
    Sequencer,
    Query,
}

/// Retry policy for transactions aborted by PostgreSQL serialization conflicts (SQLSTATE 40001).
#[derive(Clone, Copy, Debug)]
pub struct SerializableRetryConfig {
    /// Initial delay before the first retry.
    base: Duration,
    /// Maximum delay between retries.
    max: Duration,
    /// Multiplier applied to the delay between successive retries.
    factor: u32,
    /// Backoff jitter as a `(numerator, denominator)` ratio of the delay.
    jitter: (u64, u64),
    /// Maximum number of retries before giving up.
    retry_max: u32,
    /// When set, each retried conflict spawns a background `pg_stat_activity` / `pg_locks` snapshot.
    pg_stat_diag: bool,
}

impl Default for SerializableRetryConfig {
    fn default() -> Self {
        Self {
            base: Duration::from_millis(10),
            max: Duration::from_millis(500),
            factor: 2,
            jitter: (5, 10),
            retry_max: 100,
            pg_stat_diag: false,
        }
    }
}

impl SerializableRetryConfig {
    /// Construct a retry policy. `jitter` is a `(numerator, denominator)` ratio of the delay.
    pub const fn new(
        base: Duration,
        max: Duration,
        factor: u32,
        jitter: (u64, u64),
        retry_max: u32,
        pg_stat_diag: bool,
    ) -> Self {
        Self {
            base,
            max,
            factor,
            jitter,
            retry_max,
            pg_stat_diag,
        }
    }

    /// Run `f`, retrying (up to `retry_max` times with exponential backoff + jitter) whenever
    /// `should_retry` returns `true` for the error. `f` is re-run from scratch on each attempt.
    async fn retry_if<F, Fut, T, E>(
        &self,
        op: &'static str,
        mut should_retry: impl FnMut(&E) -> bool,
        f: F,
    ) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let mut delay = self.base;
        for i in 0..=self.retry_max {
            match f().await {
                Ok(res) => return Ok(res),
                Err(err) if i < self.retry_max && should_retry(&err) => {
                    tracing::warn!(
                        op,
                        attempt = i + 1,
                        max_retries = self.retry_max,
                        delay_ms = delay.as_millis(),
                        "serialization conflict, retrying transaction after {delay:?}"
                    );
                    sleep(delay).await;
                    delay = self.backoff(delay);
                },
                Err(err) => return Err(err),
            }
        }
        unreachable!()
    }

    /// Compute the next backoff delay: `delay * factor` plus random jitter, capped at `max`.
    fn backoff(&self, delay: Duration) -> Duration {
        if delay >= self.max {
            return self.max;
        }
        let ms = delay
            .saturating_mul(self.factor)
            .as_millis()
            .min(u64::MAX as u128) as u64;
        let (jitter_num, jitter_den) = self.jitter;
        let jitter = if jitter_num == 0 || jitter_den == 0 {
            0
        } else {
            let mut rng = rand::thread_rng();
            ms * rng.gen_range(0..jitter_num) / jitter_den
        };
        min(Duration::from_millis(ms + jitter), self.max)
    }
}

/// Returns `true` if `err`'s [`Display`](std::fmt::Display) output identifies it as a PostgreSQL
/// serialization conflict (SQLSTATE 40001, message "could not serialize access").
fn is_serialization_conflict_err<E: std::fmt::Display>(err: &E) -> bool {
    format!("{err:#}").contains("could not serialize access")
}

#[cfg(test)]
mod serializable_retry_tests {
    use std::sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    };

    use super::{Duration, SerializableRetryConfig, is_serialization_conflict_err};

    /// A retry policy with small delays and a low retry cap, so the exhaustion test runs quickly.
    const TEST_RETRY: SerializableRetryConfig = SerializableRetryConfig::new(
        Duration::from_millis(1),
        Duration::from_millis(5),
        2,
        (5, 10),
        5,
        false,
    );

    /// An error whose message matches a PostgreSQL serialization conflict.
    fn mock_serialization_error() -> anyhow::Error {
        anyhow::anyhow!(
            "could not serialize access due to read/write dependencies among transactions"
        )
    }

    #[test]
    fn test_is_serialization_conflict_err() {
        // The PostgreSQL serialization-conflict message must be recognised.
        assert!(is_serialization_conflict_err(&mock_serialization_error()));
        // Other database errors must NOT match.
        assert!(!is_serialization_conflict_err(&anyhow::anyhow!(
            "duplicate key value violates unique constraint"
        )));
        // Non-database errors must not match.
        assert!(!is_serialization_conflict_err(&anyhow::anyhow!(
            "plain error"
        )));
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_retry_if_succeeds_immediately() {
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = calls.clone();

        let result: anyhow::Result<()> = TEST_RETRY
            .retry_if("test", is_serialization_conflict_err, || {
                let calls = calls_clone.clone();
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_retry_if_retries_on_serialization_error() {
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = calls.clone();

        // The closure fails twice with a serialization error, then succeeds on the third attempt.
        let result: anyhow::Result<()> = TEST_RETRY
            .retry_if("test", is_serialization_conflict_err, || {
                let calls = calls_clone.clone();
                async move {
                    let n = calls.fetch_add(1, Ordering::SeqCst);
                    if n < 2 {
                        Err(mock_serialization_error())
                    } else {
                        Ok(())
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_retry_if_exhausts_retries() {
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = calls.clone();

        // The closure always fails; retry must give up after TEST_RETRY.retry_max (5) retries.
        let result: anyhow::Result<()> = TEST_RETRY
            .retry_if("test", is_serialization_conflict_err, || {
                let calls = calls_clone.clone();
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Err(mock_serialization_error())
                }
            })
            .await;

        assert!(result.is_err());
        // 1 initial attempt + 5 retries = 6 total calls.
        assert_eq!(calls.load(Ordering::SeqCst), 6);
    }

    #[test_log::test(tokio::test(flavor = "multi_thread"))]
    async fn test_retry_if_no_retry_on_other_errors() {
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = calls.clone();

        // Non-serialization errors must not be retried.
        let result: anyhow::Result<()> = TEST_RETRY
            .retry_if("test", is_serialization_conflict_err, || {
                let calls = calls_clone.clone();
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Err(anyhow::anyhow!("unrelated error"))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    /// Verify that [`function_name!`](crate::function_name) resolves to the unqualified name of the
    /// enclosing function, both from a nested helper and from the test fn itself.
    #[test]
    fn test_function_name_macro() {
        fn outer_test_fn() -> &'static str {
            crate::function_name!()
        }
        assert_eq!(outer_test_fn(), "outer_test_fn");
        assert_eq!(crate::function_name!(), "test_function_name_macro");

        // Non-ASCII identifiers must not panic: the macro slices the `type_name` output, and
        // a multi-byte codepoint near a slice boundary would panic if the macro used fixed
        // byte offsets instead of char-boundary-safe operations.
        fn télécharger() -> &'static str {
            crate::function_name!()
        }
        assert_eq!(télécharger(), "télécharger");
    }

    /// Verify that [`function_name!`](crate::function_name) resolves to the function name even from
    /// `async` contexts, where the body is lowered into a generator/closure. This is the case that
    /// every production call site hits, and a naive macro reports `{{closure}}` here.
    #[test_log::test(tokio::test)]
    async fn test_function_name_macro_async() {
        // Plain `async fn`: the body becomes a generator, adding a `{{closure}}` path segment.
        async fn plain_async_fn() -> &'static str {
            crate::function_name!()
        }
        assert_eq!(plain_async_fn().await, "plain_async_fn");

        // Closure returning an async block inside an `async fn`, mirroring the real
        // `serializable_retry!(self, || async { .. })` call sites: adds multiple `{{closure}}`
        // segments to the path.
        async fn nested_async_blocks() -> &'static str {
            let f = || async { crate::function_name!() };
            f().await
        }
        assert_eq!(nested_async_blocks().await, "nested_async_blocks");

        // `#[async_trait]` method: the body is rewritten to `Box::pin(async move { .. })`, exactly
        // as the real `serializable_retry!` call sites are.
        struct S;
        #[async_trait::async_trait]
        trait T {
            async fn async_trait_method(&self) -> &'static str;
        }
        #[async_trait::async_trait]
        impl T for S {
            async fn async_trait_method(&self) -> &'static str {
                crate::function_name!()
            }
        }
        assert_eq!(S.async_trait_method().await, "async_trait_method");
    }
}
