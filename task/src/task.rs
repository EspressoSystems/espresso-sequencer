// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::{spawn, JoinHandle};
use async_trait::async_trait;
#[cfg(async_executor_impl = "async-std")]
use futures::future::join_all;
#[cfg(async_executor_impl = "tokio")]
use futures::future::try_join_all;
#[cfg(async_executor_impl = "tokio")]
use futures::FutureExt;
use futures::StreamExt;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::{spawn, JoinHandle};

/// Trait for events that long-running tasks handle
pub trait TaskEvent: PartialEq {
    /// The shutdown signal for this event type
    ///
    /// Note that this is necessarily uniform across all tasks.
    /// Exiting the task loop is handled by the task spawner, rather than the task individually.
    fn shutdown_event() -> Self;

    /// The heartbeat event
    fn heartbeat_event(task_id: String) -> Self;
}

#[async_trait]
/// Type for mutable task state that can be used as the state for a `Task`
pub trait TaskState: Send + Sync {
    /// Type of event sent and received by the task
    type Event: TaskEvent + Clone + Send + Sync;

    /// Joins all subtasks.
    async fn cancel_subtasks(&mut self);

    /// Handles an event, providing direct access to the specific channel we received the event on.
    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        _sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()>;

    /// Runs a specified job in the main task every `Task::PERIODIC_INTERVAL_IN_SECS`
    async fn periodic_task(&self, sender: &Sender<Arc<Self::Event>>, task_id: String) {
        match sender
            .broadcast_direct(Arc::new(Self::Event::heartbeat_event(task_id)))
            .await
        {
            Ok(None) => (),
            Ok(Some(_overflowed)) => {
                tracing::error!(
                    "Event sender queue overflow, Oldest event removed form queue: Heartbeat Event"
                );
            }
            Err(async_broadcast::SendError(_e)) => {
                tracing::warn!("Event: Heartbeat\n Sending failed, event stream probably shutdown");
            }
        }
    }

    /// Gets the name of the current task
    fn get_task_name(&self) -> &'static str;
}

/// A basic task which loops waiting for events to come from `event_receiver`
/// and then handles them using its state
/// It sends events to other `Task`s through `sender`
/// This should be used as the primary building block for long running
/// or medium running tasks (i.e. anything that can't be described as a dependency task)
pub struct Task<S: TaskState> {
    /// The state of the task.  It is fed events from `receiver`
    /// and mutated via `handle_event`.
    state: S,
    /// Sends events all tasks including itself
    sender: Sender<Arc<S::Event>>,
    /// Receives events that are broadcast from any task, including itself
    receiver: Receiver<Arc<S::Event>>,
    /// The generated task id
    task_id: String,
}

impl<S: TaskState + Send + 'static> Task<S> {
    /// Constant for how often we run our periodic tasks, such as broadcasting a hearbeat
    const PERIODIC_INTERVAL_IN_SECS: u64 = 10;

    /// Create a new task
    pub fn new(
        state: S,
        sender: Sender<Arc<S::Event>>,
        receiver: Receiver<Arc<S::Event>>,
        task_id: String,
    ) -> Self {
        Task {
            state,
            sender,
            receiver,
            task_id,
        }
    }

    /// The state of the task, as a boxed dynamic trait object.
    fn boxed_state(self) -> Box<dyn TaskState<Event = S::Event>> {
        Box::new(self.state) as Box<dyn TaskState<Event = S::Event>>
    }

    #[cfg(async_executor_impl = "async-std")]
    /// Periodic delay
    pub fn get_periodic_interval_in_secs() -> futures::stream::Fuse<async_std::stream::Interval> {
        async_std::stream::interval(Duration::from_secs(Self::PERIODIC_INTERVAL_IN_SECS)).fuse()
    }

    #[cfg(async_executor_impl = "async-std")]
    /// Handle periodic delay interval
    pub fn handle_periodic_delay(
        periodic_interval: &mut futures::stream::Fuse<async_std::stream::Interval>,
    ) -> futures::stream::Next<'_, futures::stream::Fuse<async_std::stream::Interval>> {
        periodic_interval.next()
    }

    #[cfg(async_executor_impl = "tokio")]
    #[must_use]
    /// Periodic delay
    pub fn get_periodic_interval_in_secs() -> tokio::time::Interval {
        tokio::time::interval(Duration::from_secs(Self::PERIODIC_INTERVAL_IN_SECS))
    }

    #[cfg(async_executor_impl = "tokio")]
    /// Handle periodic delay interval
    pub fn handle_periodic_delay(
        periodic_interval: &mut tokio::time::Interval,
    ) -> futures::future::Fuse<impl futures::Future<Output = tokio::time::Instant> + '_> {
        periodic_interval.tick().fuse()
    }

    /// Spawn the task loop, consuming self.  Will continue until
    /// the task reaches some shutdown condition
    pub fn run(mut self) -> HotShotTaskHandle<S::Event> {
        let task_id = self.task_id.clone();
        let handle = spawn(async move {
            let recv_stream =
                futures::stream::unfold(self.receiver.clone(), |mut recv| async move {
                    match recv.recv_direct().await {
                        Ok(event) => Some((Ok(event), recv)),
                        Err(e) => Some((Err(e), recv)),
                    }
                })
                .boxed();

            let fused_recv_stream = recv_stream.fuse();
            let periodic_interval = Self::get_periodic_interval_in_secs();
            futures::pin_mut!(periodic_interval, fused_recv_stream);
            loop {
                futures::select! {
                    input = fused_recv_stream.next() => {
                        match input {
                            Some(Ok(input)) => {
                                if *input == S::Event::shutdown_event() {
                                    self.state.cancel_subtasks().await;

                                    break self.boxed_state();
                                }
                                let _ = S::handle_event(
                                    &mut self.state,
                                    input,
                                    &self.sender,
                                    &self.receiver,
                                )
                                .await
                                .inspect_err(|e| tracing::info!("{e}"));
                            }
                            Some(Err(e)) => {
                                tracing::error!("Failed to receive from event stream Error: {}", e);
                            }
                            None => {}
                        }
                    }
                    _ = Self::handle_periodic_delay(&mut periodic_interval) => {
                        self.state.periodic_task(&self.sender, self.task_id.clone()).await;
                    },
                }
            }
        });
        HotShotTaskHandle { handle, task_id }
    }
}

/// Wrapper around handle and task id so we can map
pub struct HotShotTaskHandle<EVENT> {
    /// Handle for the task
    pub handle: JoinHandle<Box<dyn TaskState<Event = EVENT>>>,
    /// Generated task id
    pub task_id: String,
}

#[derive(Default)]
/// A collection of tasks which can handle shutdown
pub struct ConsensusTaskRegistry<EVENT> {
    /// Tasks this registry controls
    pub task_handles: Vec<HotShotTaskHandle<EVENT>>,
}

impl<EVENT: Send + Sync + Clone + TaskEvent> ConsensusTaskRegistry<EVENT> {
    #[must_use]
    /// Create a new task registry
    pub fn new() -> Self {
        ConsensusTaskRegistry {
            task_handles: vec![],
        }
    }

    /// Add a task to the registry
    pub fn register(&mut self, handle: HotShotTaskHandle<EVENT>) {
        self.task_handles.push(handle);
    }

    #[must_use]
    /// Get all task ids from registry
    pub fn get_task_ids(&self) -> Vec<String> {
        self.task_handles
            .iter()
            .map(|wrapped_handle| wrapped_handle.task_id.clone())
            .collect()
    }

    /// Try to cancel/abort the task this registry has
    ///
    /// # Panics
    ///
    /// Should not panic, unless awaiting on the JoinHandle in tokio fails.
    pub async fn shutdown(&mut self) {
        let handles = &mut self.task_handles;

        while let Some(wrapped_handle) = handles.pop() {
            #[cfg(async_executor_impl = "async-std")]
            let mut task_state = wrapped_handle.handle.await;
            #[cfg(async_executor_impl = "tokio")]
            let mut task_state = wrapped_handle.handle.await.unwrap();

            task_state.cancel_subtasks().await;
        }
    }
    /// Take a task, run it, and register it
    pub fn run_task<S>(&mut self, task: Task<S>)
    where
        S: TaskState<Event = EVENT> + Send + 'static,
    {
        self.register(task.run());
    }

    /// Wait for the results of all the tasks registered
    /// # Panics
    /// Panics if one of the tasks panicked
    pub async fn join_all(self) -> Vec<Box<dyn TaskState<Event = EVENT>>> {
        let handles: Vec<JoinHandle<Box<dyn TaskState<Event = EVENT>>>> = self
            .task_handles
            .into_iter()
            .map(|wrapped| wrapped.handle)
            .collect();
        #[cfg(async_executor_impl = "async-std")]
        let states = join_all(handles).await;
        #[cfg(async_executor_impl = "tokio")]
        let states = try_join_all(handles).await.unwrap();

        states
    }
}

/// Wrapper around join handle and task id for network tasks
pub struct NetworkHandle {
    /// Task handle
    pub handle: JoinHandle<()>,
    /// Generated task id
    pub task_id: String,
}

#[derive(Default)]
/// A collection of tasks which can handle shutdown
pub struct NetworkTaskRegistry {
    /// Tasks this registry controls
    pub handles: Vec<NetworkHandle>,
}

impl NetworkTaskRegistry {
    #[must_use]
    /// Create a new task registry
    pub fn new() -> Self {
        NetworkTaskRegistry { handles: vec![] }
    }

    #[must_use]
    /// Get all task ids from registry
    pub fn get_task_ids(&self) -> Vec<String> {
        self.handles
            .iter()
            .map(|wrapped_handle| wrapped_handle.task_id.clone())
            .collect()
    }

    #[allow(clippy::unused_async)]
    /// Shuts down all tasks managed by this instance.
    ///
    /// This function waits for all tasks to complete before returning.
    ///
    /// # Panics
    ///
    /// When using the tokio executor, this function will panic if any of the
    /// tasks being joined return an error.
    pub async fn shutdown(&mut self) {
        let handles = std::mem::take(&mut self.handles);
        let task_handles: Vec<JoinHandle<()>> =
            handles.into_iter().map(|wrapped| wrapped.handle).collect();
        #[cfg(async_executor_impl = "async-std")]
        join_all(task_handles).await;
        #[cfg(async_executor_impl = "tokio")]
        try_join_all(task_handles)
            .await
            .expect("Failed to join all tasks during shutdown");
    }

    /// Add a task to the registry
    pub fn register(&mut self, handle: NetworkHandle) {
        self.handles.push(handle);
    }
}
