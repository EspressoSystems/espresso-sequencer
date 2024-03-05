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

//! Async task utilites.

use async_std::{
    sync::Arc,
    task::{spawn, JoinHandle},
};
use futures::future::Future;
use std::fmt::Display;
use tracing::{info_span, Instrument};

#[derive(Debug)]
struct BackgroundTaskInner {
    name: String,
    handle: JoinHandle<()>,
}

/// A background task which is cancelled on [`Drop`]
///
/// This handle can be cloned; cloning it does not clone the underlying task. There may be many
/// handles to the same background task, and the task will be cancelled when all handles are
/// dropped.
#[derive(Clone, Debug)]
pub struct BackgroundTask {
    // The task handle is an `Option` so we can `take()` out of it during `drop`, where we have a
    // mutable reference but need to move out of the underlying task handle to cancel it. This will
    // always be `Some` except during cancellation.
    inner: Option<Arc<BackgroundTaskInner>>,
}

impl BackgroundTask {
    /// Spawn a background task, which will be cancelled when every clone is dropped.
    ///
    /// The caller should ensure that `future` yields back to the executor fairly frequently, to
    /// ensure timely cancellation in case the task is dropped. If an operation in `future` may run
    /// for a long time without blocking or yielding, consider using
    /// [`yield_now`](async_std::task::yield_now) periodically, or using
    /// [`spawn`](async_std::task::spawn) or [`spawn_blocking`](async_std::task::spawn_blocking) to
    /// run long operations in a sub-task.
    pub fn spawn<F>(name: impl Display, future: F) -> Self
    where
        F: Future + Send + 'static,
    {
        let name = name.to_string();
        let handle = {
            let span = info_span!("task", name);
            spawn(
                async move {
                    tracing::info!("spawning background task");
                    future.await;
                    tracing::info!("completed background task");
                }
                .instrument(span),
            )
        };

        Self {
            inner: Some(Arc::new(BackgroundTaskInner { name, handle })),
        }
    }
}

impl Drop for BackgroundTask {
    fn drop(&mut self) {
        // `inner` should never be [`None`] here, we only `take` it because we are given `&mut
        // self` (so that this can be called from [`drop`]) and thus we cannot move out of `self`.
        // Nevertheless, it doesn't hurt to explicitly check for [`Some`].
        if let Some(inner) = self.inner.take() {
            // Check if this is the last instance of the [`Arc`] and, if so, cancel the underlying
            // task.
            if let Some(inner) = Arc::into_inner(inner) {
                tracing::info!(name = inner.name, "cancelling background task");
                async_std::task::block_on(inner.handle.cancel());
                tracing::info!(name = inner.name, "cancelled background task");
            }
        }
    }
}
