use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn, async_timeout};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::{spawn, JoinHandle};
use async_trait::async_trait;
use futures::future::select_all;
use hotshot_task_impls::{events::HotShotEvent, network::NetworkMessageTaskState};
use hotshot_types::{
    message::{Message, Messages},
    traits::{network::ConnectedNetwork, node_implementation::NodeType},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::{spawn, JoinHandle};
use tracing::error;

/// enum describing how the tasks completed
pub enum TestResult {
    /// the test task passed
    Pass,
    /// the test task failed with an error
    Fail(Box<dyn snafu::Error + Send + Sync>),
}

#[async_trait]
/// Type for mutable task state that can be used as the state for a `Task`
pub trait TestTaskState: Send {
    /// Type of event sent and received by the task
    type Event: Clone + Send + Sync;

    /// Handles an event from one of multiple receivers.
    async fn handle_event(&mut self, (event, id): (Self::Event, usize)) -> Result<()>;

    /// Check the result of the test.
    fn check(&self) -> TestResult;
}

/// A basic task which loops waiting for events to come from `event_receiver`
/// and then handles them using it's state
/// It sends events to other `Task`s through `event_sender`
/// This should be used as the primary building block for long running
/// or medium running tasks (i.e. anything that can't be described as a dependency task)
pub struct TestTask<S: TestTaskState> {
    /// The state of the task.  It is fed events from `event_sender`
    /// and mutates it state ocordingly.  Also it signals the task
    /// if it is complete/should shutdown
    state: S,
    /// Receives events that are broadcast from any task, including itself
    receivers: Vec<Receiver<S::Event>>,
    /// Receiver for test events, used for communication between test tasks.
    test_receiver: Receiver<TestEvent>,
}

#[derive(Clone, Debug)]
pub enum TestEvent {
    Shutdown,
}

impl<S: TestTaskState + Send + 'static> TestTask<S> {
    /// Create a new task
    pub fn new(
        state: S,
        receivers: Vec<Receiver<S::Event>>,
        test_receiver: Receiver<TestEvent>,
    ) -> Self {
        TestTask {
            state,
            receivers,
            test_receiver,
        }
    }

    /// Spawn the task loop, consuming self.  Will continue until
    /// the task reaches some shutdown condition
    pub fn run(mut self) -> JoinHandle<TestResult> {
        spawn(async move {
            loop {
                if let Ok(TestEvent::Shutdown) = self.test_receiver.try_recv() {
                    break self.state.check();
                }

                let mut messages = Vec::new();

                for receiver in &mut self.receivers {
                    messages.push(receiver.recv());
                }

                if let Ok((Ok(input), id, _)) =
                    async_timeout(Duration::from_millis(50), select_all(messages)).await
                {
                    let _ = S::handle_event(&mut self.state, (input, id))
                        .await
                        .inspect_err(|e| tracing::error!("{e}"));
                }
            }
        })
    }
}

/// Add the network task to handle messages and publish events.
pub async fn add_network_message_test_task<
    TYPES: NodeType,
    NET: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
>(
    event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    channel: Arc<NET>,
) -> JoinHandle<()> {
    let net = Arc::clone(&channel);
    let network_state: NetworkMessageTaskState<_> = NetworkMessageTaskState {
        event_stream: event_stream.clone(),
    };

    let network = Arc::clone(&net);
    let mut state = network_state.clone();

    async_spawn(async move {
        loop {
            let msgs = match network.recv_msgs().await {
                Ok(msgs) => Messages(msgs),
                Err(err) => {
                    error!("failed to receive messages: {err}");

                    // return zero messages so we sleep and try again
                    Messages(vec![])
                }
            };
            if msgs.0.is_empty() {
                // TODO: Stop sleeping here: https://github.com/EspressoSystems/HotShot/issues/2558
                async_sleep(Duration::from_millis(100)).await;
            } else {
                state.handle_messages(msgs.0).await;
            }
        }
    })
}
