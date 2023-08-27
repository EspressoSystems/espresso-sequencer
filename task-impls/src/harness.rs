use crate::events::SequencingHotShotEvent;
use async_compatibility_layer::art::async_spawn;

use futures::FutureExt;
use hotshot_task::{
    event_stream::{self, ChannelStream, EventStream},
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
    task_launcher::TaskRunner,
};
use hotshot_types::traits::node_implementation::{NodeImplementation, NodeType};
use snafu::Snafu;
use std::{collections::HashMap, future::Future, sync::Arc};

/// The state for the test harness task. Keeps track of which events and how many we expect to get
pub struct TestHarnessState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The expected events we get from the test.  Maps an event to the number of times we expect to see it
    expected_output: HashMap<SequencingHotShotEvent<TYPES, I>, usize>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TS for TestHarnessState<TYPES, I> {}

/// Error emitted if the test harness task fails
#[derive(Snafu, Debug)]
pub struct TestHarnessTaskError {}

/// Type alias for the Test Harness Task
pub type TestHarnessTaskTypes<TYPES, I> = HSTWithEvent<
    TestHarnessTaskError,
    SequencingHotShotEvent<TYPES, I>,
    ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    TestHarnessState<TYPES, I>,
>;

/// Runs a test by building the task using `build_fn` and then passing it the `input` events
/// and testing the make sure all of the `expected_output` events are seen
///
/// `event_stream` - if given, will be used to register the task builder.
///
/// # Panics
/// Panics if any state the test expects is not set. Panicing causes a test failure
#[allow(clippy::implicit_hasher)]
pub async fn run_harness<TYPES, I, Fut>(
    input: Vec<SequencingHotShotEvent<TYPES, I>>,
    expected_output: HashMap<SequencingHotShotEvent<TYPES, I>, usize>,
    event_stream: Option<ChannelStream<SequencingHotShotEvent<TYPES, I>>>,
    build_fn: impl FnOnce(TaskRunner, ChannelStream<SequencingHotShotEvent<TYPES, I>>) -> Fut,
) where
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    Fut: Future<Output = TaskRunner>,
{
    let task_runner = TaskRunner::new();
    let registry = task_runner.registry.clone();
    let event_stream = event_stream.unwrap_or(event_stream::ChannelStream::new());
    let state = TestHarnessState { expected_output };
    let handler = HandleEvent(Arc::new(move |event, state| {
        async move { handle_event(event, state) }.boxed()
    }));
    let filter = FilterEvent::default();
    let builder = TaskBuilder::<TestHarnessTaskTypes<TYPES, I>>::new("test_harness".to_string())
        .register_event_stream(event_stream.clone(), filter)
        .await
        .register_registry(&mut registry.clone())
        .await
        .register_state(state)
        .register_event_handler(handler);

    let id = builder.get_task_id().unwrap();

    let task = TestHarnessTaskTypes::build(builder).launch();

    let task_runner = task_runner.add_task(id, "test_harness".to_string(), task);
    let task_runner = build_fn(task_runner, event_stream.clone()).await;

    let runner = async_spawn(async move { task_runner.launch().await });

    for event in input {
        let _ = event_stream.publish(event).await;
    }

    let _ = runner.await;
}

/// Handles an event for the Test Harness Task.  If the event is expected, remove it from
/// the `expected_output` in state.  If unexpected fail test.
///
///  # Panics
/// Will panic to fail the test when it receives and unexpected event
#[allow(clippy::needless_pass_by_value)]
pub fn handle_event<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    event: SequencingHotShotEvent<TYPES, I>,
    mut state: TestHarnessState<TYPES, I>,
) -> (
    std::option::Option<HotShotTaskCompleted>,
    TestHarnessState<TYPES, I>,
) {
    assert!(
        state.expected_output.contains_key(&event),
        "Got an unexpected event: {event:?}",
    );
    let num_expected = state.expected_output.get_mut(&event).unwrap();
    if *num_expected == 1 {
        state.expected_output.remove(&event);
    } else {
        *num_expected -= 1;
    }

    if state.expected_output.is_empty() {
        return (Some(HotShotTaskCompleted::ShutDown), state);
    }
    (None, state)
}
