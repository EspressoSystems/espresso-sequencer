use std::{sync::Arc, time::Duration};

use async_broadcast::{Receiver, SendError, Sender};
use async_compatibility_layer::art::async_timeout;
#[cfg(async_executor_impl = "async-std")]
use async_std::{
    sync::RwLock,
    task::{spawn, JoinHandle},
};
#[cfg(async_executor_impl = "async-std")]
use futures::future::join_all;
#[cfg(async_executor_impl = "tokio")]
use futures::future::try_join_all;
use futures::{future::select_all, Future};
#[cfg(async_executor_impl = "tokio")]
use tokio::{
    sync::RwLock,
    task::{spawn, JoinHandle},
};
use tracing::{error, warn};

use crate::{
    dependency::Dependency,
    dependency_task::{DependencyTask, HandleDepOutput},
};

/// Type for mutable task state that can be used as the state for a `Task`
pub trait TaskState: Send {
    /// Type of event sent and received by the task
    type Event: Clone + Send + Sync + 'static;
    /// The result returned when this task completes
    type Output: Send;
    /// Handle event and update state.  Return true if the task is finished
    /// false otherwise.  The handler can access the state through `Task::state_mut`
    fn handle_event(
        event: Self::Event,
        task: &mut Task<Self>,
    ) -> impl Future<Output = Option<Self::Output>> + Send
    where
        Self: Sized;

    /// Return true if the event should be filtered
    fn filter(&self, _event: &Self::Event) -> bool {
        // default doesn't filter
        false
    }
    /// Do something with the result of the task before it shuts down
    fn handle_result(&self, _res: &Self::Output) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    /// Return true if the event should shut the task down
    fn should_shutdown(event: &Self::Event) -> bool;
    /// Handle anything before the task is completely shutdown
    fn shutdown(&mut self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

/// Task state for a test.  Similar to `TaskState` but it handles
/// messages as well as events.  Messages are events that are
/// external to this task.  (i.e. a test message would be an event from non test task)
/// This is used as state for `TestTask` and messages can come from many
/// different input streams.
pub trait TestTaskState: Send {
    /// Message type handled by the task
    type Message: Clone + Send + Sync + 'static;
    /// Result returned by the test task on completion
    type Output: Send;
    /// The state type
    type State: TaskState;
    /// Handle and incoming message and return `Some` if the task is finished
    fn handle_message(
        message: Self::Message,
        id: usize,
        task: &mut TestTask<Self::State, Self>,
    ) -> impl Future<Output = Option<Self::Output>> + Send
    where
        Self: Sized;
}

/// A basic task which loops waiting for events to come from `event_receiver`
/// and then handles them using it's state
/// It sends events to other `Task`s through `event_sender`
/// This should be used as the primary building block for long running
/// or medium running tasks (i.e. anything that can't be described as a dependency task)
pub struct Task<S: TaskState> {
    /// Sends events all tasks including itself
    event_sender: Sender<S::Event>,
    /// Receives events that are broadcast from any task, including itself
    event_receiver: Receiver<S::Event>,
    /// Contains this task, used to register any spawned tasks
    registry: Arc<TaskRegistry>,
    /// The state of the task.  It is fed events from `event_sender`
    /// and mutates it state ocordingly.  Also it signals the task
    /// if it is complete/should shutdown
    state: S,
}

impl<S: TaskState + Send + 'static> Task<S> {
    /// Create a new task
    pub fn new(
        tx: Sender<S::Event>,
        rx: Receiver<S::Event>,
        registry: Arc<TaskRegistry>,
        state: S,
    ) -> Self {
        Task {
            event_sender: tx,
            event_receiver: rx,
            registry,
            state,
        }
    }

    /// The Task analog of `TaskState::handle_event`.
    pub fn handle_event(
        &mut self,
        event: S::Event,
    ) -> impl Future<Output = Option<S::Output>> + Send + '_
    where
        Self: Sized,
    {
        S::handle_event(event, self)
    }

    /// Spawn the task loop, consuming self.  Will continue until
    /// the task reaches some shutdown condition
    pub fn run(mut self) -> JoinHandle<()> {
        spawn(async move {
            loop {
                match self.event_receiver.recv_direct().await {
                    Ok(event) => {
                        if S::should_shutdown(&event) {
                            self.state.shutdown().await;
                            break;
                        }
                        if self.state.filter(&event) {
                            continue;
                        }
                        if let Some(res) = S::handle_event(event, &mut self).await {
                            self.state.handle_result(&res).await;
                            self.state.shutdown().await;
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to receiving from event stream Error: {}", e);
                    }
                }
            }
        })
    }

    /// Create a new event `Receiver` from this Task's receiver.
    /// The returned receiver will get all messages not yet seen by this task
    pub fn subscribe(&self) -> Receiver<S::Event> {
        self.event_receiver.clone()
    }
    /// Get a new sender handle for events
    pub fn sender(&self) -> &Sender<S::Event> {
        &self.event_sender
    }
    /// Clone the sender handle
    pub fn clone_sender(&self) -> Sender<S::Event> {
        self.event_sender.clone()
    }
    /// Broadcast a message to all listening tasks
    /// # Errors
    /// Errors if the broadcast fails
    pub async fn send(&self, event: S::Event) -> Result<Option<S::Event>, SendError<S::Event>> {
        self.event_sender.broadcast(event).await
    }
    /// Get a mutable reference to this tasks state
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }
    /// Get an immutable reference to this tasks state
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Spawn a new task and register it.  It will get all events not seend
    /// by the task creating it.
    pub async fn run_sub_task(&self, state: S) {
        let task = Task {
            event_sender: self.clone_sender(),
            event_receiver: self.subscribe(),
            registry: Arc::clone(&self.registry),
            state,
        };
        // Note: await here is only awaiting the task to be added to the
        // registry, not for the task to run.
        self.registry.run_task(task).await;
    }
}

/// Similar to `Task` but adds functionality for testing.  Notably
/// it adds message receivers to collect events from many non-test tasks
pub struct TestTask<S: TaskState, T: TestTaskState + Send> {
    /// Task which handles test events
    task: Task<S>,
    /// Receivers for outside events
    message_receivers: Vec<Receiver<T::Message>>,
}

impl<
        S: TaskState + Send + 'static,
        T: TestTaskState<State = S, Output = S::Output> + Send + Sync + 'static,
    > TestTask<S, T>
{
    /// Create a test task
    pub fn new(task: Task<S>, rxs: Vec<Receiver<T::Message>>) -> Self {
        Self {
            task,
            message_receivers: rxs,
        }
    }
    /// Runs the task, taking events from the the test events and the message receivers.
    /// Consumes self and runs until some shutdown condition is met.
    /// The join handle will return the result of the task, useful for deciding if the test
    /// passed or not.
    pub fn run(mut self) -> JoinHandle<S::Output> {
        spawn(async move {
            loop {
                let mut futs = vec![];

                if let Ok(event) = self.task.event_receiver.try_recv() {
                    if S::should_shutdown(&event) {
                        self.task.state.shutdown().await;
                        tracing::error!("Shutting down test task TODO!");
                        todo!();
                    }
                    if !self.state().filter(&event) {
                        if let Some(res) = S::handle_event(event, &mut self.task).await {
                            self.task.state.handle_result(&res).await;
                            self.task.state.shutdown().await;
                            return res;
                        }
                    }
                }

                for rx in &mut self.message_receivers {
                    futs.push(rx.recv());
                }
                // if let Ok((Ok(msg), id, _)) =
                match async_timeout(Duration::from_secs(1), select_all(futs)).await {
                    Ok((Ok(msg), id, _)) => {
                        if let Some(res) = T::handle_message(msg, id, &mut self).await {
                            self.task.state.handle_result(&res).await;
                            self.task.state.shutdown().await;
                            return res;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to get event from task. Error: {:?}", e);
                    }
                    Ok((Err(e), _, _)) => {
                        error!("A task channel returned an Error: {:?}", e);
                    }
                }
            }
        })
    }

    /// Get a ref to state
    pub fn state(&self) -> &S {
        &self.task.state
    }
    /// Get a mutable ref to state
    pub fn state_mut(&mut self) -> &mut S {
        self.task.state_mut()
    }
    /// Send an event to other listening test tasks
    ///
    /// # Panics
    /// panics if the event can't be sent (ok to panic in test)
    pub async fn send_event(&self, event: S::Event) {
        self.task.send(event).await.unwrap();
    }
}

#[derive(Default)]
/// A collection of tasks which can handle shutdown
pub struct TaskRegistry {
    /// Tasks this registry controls
    task_handles: RwLock<Vec<JoinHandle<()>>>,
}

impl TaskRegistry {
    /// Add a task to the registry
    pub async fn register(&self, handle: JoinHandle<()>) {
        self.task_handles.write().await.push(handle);
    }
    /// Try to cancel/abort the task this registry has
    pub async fn shutdown(&self) {
        let mut handles = self.task_handles.write().await;
        while let Some(handle) = handles.pop() {
            #[cfg(async_executor_impl = "async-std")]
            handle.cancel().await;
            #[cfg(async_executor_impl = "tokio")]
            handle.abort();
        }
    }
    /// Take a task, run it, and register it
    pub async fn run_task<S>(&self, task: Task<S>)
    where
        S: TaskState + Send + 'static,
    {
        self.register(task.run()).await;
    }
    /// Create a new `DependencyTask` run it, and register it
    pub async fn spawn_dependency_task<T, H>(
        &self,
        dep: impl Dependency<T> + Send + 'static,
        handle: impl HandleDepOutput<Output = T>,
    ) {
        let join_handle = DependencyTask { dep, handle }.run();
        self.register(join_handle).await;
    }
    /// Wait for the results of all the tasks registered
    /// # Panics
    /// Panics if one of the tasks panicked
    pub async fn join_all(self) -> Vec<()> {
        #[cfg(async_executor_impl = "async-std")]
        let ret = join_all(self.task_handles.into_inner()).await;
        #[cfg(async_executor_impl = "tokio")]
        let ret = try_join_all(self.task_handles.into_inner()).await.unwrap();
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, time::Duration};

    use async_broadcast::broadcast;
    #[cfg(async_executor_impl = "async-std")]
    use async_std::task::sleep;
    #[cfg(async_executor_impl = "tokio")]
    use tokio::time::sleep;

    use super::*;

    #[derive(Default)]
    pub struct DummyHandle {
        val: usize,
        seen: HashSet<usize>,
    }

    #[allow(clippy::panic)]
    impl TaskState for DummyHandle {
        type Event = usize;
        type Output = ();
        async fn handle_event(event: usize, task: &mut Task<Self>) -> Option<()> {
            sleep(Duration::from_millis(10)).await;
            let state = task.state_mut();
            state.seen.insert(event);
            if event > state.val {
                state.val = event;
                assert!(
                    state.val < 100,
                    "Test should shutdown before getting an event for 100"
                );
                task.send(event + 1).await.unwrap();
            }
            None
        }
        fn should_shutdown(event: &usize) -> bool {
            *event >= 98
        }
        async fn shutdown(&mut self) {
            for i in 1..98 {
                assert!(self.seen.contains(&i));
            }
        }
    }

    impl TestTaskState for DummyHandle {
        type Message = String;
        type Output = ();
        type State = Self;

        async fn handle_message(
            message: Self::Message,
            _: usize,
            _: &mut TestTask<Self::State, Self>,
        ) -> Option<()> {
            if message == *"done".to_string() {
                return Some(());
            }
            None
        }
    }
    #[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[allow(unused_must_use)]
    async fn it_works() {
        let reg = Arc::new(TaskRegistry::default());
        let (tx, rx) = broadcast(10);
        let task1 = Task::<DummyHandle> {
            event_sender: tx.clone(),
            event_receiver: rx.clone(),
            registry: Arc::clone(&reg),
            state: DummyHandle::default(),
        };
        tx.broadcast(1).await.unwrap();
        let task2 = Task::<DummyHandle> {
            event_sender: tx.clone(),
            event_receiver: rx,
            registry: reg,
            state: DummyHandle::default(),
        };
        let handle = task2.run();
        let _res = task1.run().await;
        handle.await;
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 10)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    async fn test_works() {
        let reg = Arc::new(TaskRegistry::default());
        let (tx, rx) = broadcast(10);
        let (msg_tx, msg_rx) = broadcast(10);
        let task1 = Task::<DummyHandle> {
            event_sender: tx.clone(),
            event_receiver: rx.clone(),
            registry: Arc::clone(&reg),
            state: DummyHandle::default(),
        };
        tx.broadcast(1).await.unwrap();
        let task2 = Task::<DummyHandle> {
            event_sender: tx.clone(),
            event_receiver: rx,
            registry: reg,
            state: DummyHandle::default(),
        };
        let test1 = TestTask::<_, DummyHandle> {
            task: task1,
            message_receivers: vec![msg_rx.clone()],
        };
        let test2 = TestTask::<_, DummyHandle> {
            task: task2,
            message_receivers: vec![msg_rx.clone()],
        };

        let handle = test1.run();
        let handle2 = test2.run();
        sleep(Duration::from_millis(30)).await;
        msg_tx.broadcast("done".into()).await.unwrap();
        #[cfg(async_executor_impl = "tokio")]
        {
            handle.await.unwrap();
            handle2.await.unwrap();
        }
        #[cfg(async_executor_impl = "async-std")]
        {
            handle.await;
            handle2.await;
        }
    }
}
