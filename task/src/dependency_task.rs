// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#[cfg(async_executor_impl = "async-std")]
use async_std::task::{spawn, JoinHandle};
use futures::Future;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::{spawn, JoinHandle};

use crate::dependency::Dependency;

/// Defines a type that can handle the result of a dependency
pub trait HandleDepOutput: Send + Sized + Sync + 'static {
    /// Type we expect from completed dependency
    type Output: Send + Sync + 'static;

    /// Called once when the Dependency completes handles the results
    fn handle_dep_result(self, res: Self::Output) -> impl Future<Output = ()> + Send;
}

/// A task that runs until it's dependency completes and it handles the result
pub struct DependencyTask<D: Dependency<H::Output> + Send, H: HandleDepOutput + Send> {
    /// Dependency this taks waits for
    pub(crate) dep: D,
    /// Handles the results returned from `self.dep.completed().await`
    pub(crate) handle: H,
}

impl<D: Dependency<H::Output> + Send, H: HandleDepOutput + Send> DependencyTask<D, H> {
    /// Create a new `DependencyTask`
    #[must_use]
    pub fn new(dep: D, handle: H) -> Self {
        Self { dep, handle }
    }
}

impl<D: Dependency<H::Output> + Send + 'static, H: HandleDepOutput> DependencyTask<D, H> {
    /// Spawn the dependency task
    pub fn run(self) -> JoinHandle<()>
    where
        Self: Sized,
    {
        spawn(async move {
            if let Some(completed) = self.dep.completed().await {
                self.handle.handle_dep_result(completed).await;
            }
        })
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use async_broadcast::{broadcast, Receiver, Sender};
    #[cfg(async_executor_impl = "async-std")]
    use async_std::task::sleep;
    use futures::{stream::FuturesOrdered, StreamExt};
    #[cfg(async_executor_impl = "tokio")]
    use tokio::time::sleep;

    use super::*;
    use crate::dependency::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TaskResult {
        Success(usize),
        // Failure,
    }

    struct DummyHandle {
        sender: Sender<TaskResult>,
    }
    impl HandleDepOutput for DummyHandle {
        type Output = usize;
        async fn handle_dep_result(self, res: usize) {
            self.sender
                .broadcast(TaskResult::Success(res))
                .await
                .unwrap();
        }
    }

    fn eq_dep(rx: Receiver<usize>, val: usize) -> EventDependency<usize> {
        EventDependency::new(rx, Box::new(move |v| *v == val))
    }

    #[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    // allow unused for tokio because it's a test
    #[allow(unused_must_use)]
    async fn it_works() {
        let (tx, rx) = broadcast(10);
        let (res_tx, mut res_rx) = broadcast(10);
        let dep = eq_dep(rx, 2);
        let handle = DummyHandle { sender: res_tx };
        let join_handle = DependencyTask { dep, handle }.run();
        tx.broadcast(2).await.unwrap();
        assert_eq!(res_rx.recv().await.unwrap(), TaskResult::Success(2));

        join_handle.await;
    }

    #[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn many_works() {
        let (tx, rx) = broadcast(20);
        let (res_tx, mut res_rx) = broadcast(20);

        let mut handles = vec![];
        for i in 0..10 {
            let dep = eq_dep(rx.clone(), i);
            let handle = DummyHandle {
                sender: res_tx.clone(),
            };
            handles.push(DependencyTask { dep, handle }.run());
        }
        let tx2 = tx.clone();
        spawn(async move {
            for i in 0..10 {
                tx.broadcast(i).await.unwrap();
                sleep(Duration::from_millis(10)).await;
            }
        });
        for i in 0..10 {
            assert_eq!(res_rx.recv().await.unwrap(), TaskResult::Success(i));
        }
        tx2.broadcast(100).await.unwrap();
        FuturesOrdered::from_iter(handles).collect::<Vec<_>>().await;
    }
}
