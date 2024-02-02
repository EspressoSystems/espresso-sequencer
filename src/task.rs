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
    inner: Option<Arc<BackgroundTaskInner>>,
}

impl BackgroundTask {
    /// Spawn a background task, which will be cancelled when every clone is dropped.
    pub fn spawn<F>(name: impl Display, future: F) -> Self
    where
        F: Future + Send + 'static,
    {
        let name = name.to_string();
        let handle = {
            let name = name.clone();
            spawn(async move {
                tracing::info!("spawning background task {name}");
                future.await;
                tracing::info!("background task {name} completed");
            })
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
                tracing::info!("cancelling background task {}", inner.name);
                async_std::task::block_on(inner.handle.cancel());
            }
        }
    }
}
