#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;

/// Cancel a task
pub async fn cancel_task<T>(task: JoinHandle<T>) {
    #[cfg(async_executor_impl = "async-std")]
    task.cancel().await;
    #[cfg(async_executor_impl = "tokio")]
    task.abort();
}
