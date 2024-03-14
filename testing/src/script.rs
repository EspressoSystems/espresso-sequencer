use crate::predicates::Predicate;
use async_broadcast::broadcast;
use hotshot_task_impls::events::HotShotEvent;

use hotshot_task::task::{Task, TaskRegistry, TaskState};
use hotshot_types::traits::node_implementation::NodeType;
use std::sync::Arc;

pub struct TestScriptStage<TYPES: NodeType, S: TaskState<Event = Arc<HotShotEvent<TYPES>>>> {
    pub inputs: Vec<HotShotEvent<TYPES>>,
    pub outputs: Vec<Predicate<Arc<HotShotEvent<TYPES>>>>,
    pub asserts: Vec<Predicate<S>>,
}

/// A `TestScript` is a sequence of triples (input sequence, output sequence, assertions).
type TestScript<TYPES, S> = Vec<TestScriptStage<TYPES, S>>;

pub fn panic_extra_output<S>(stage_number: usize, output: &S)
where
    S: std::fmt::Debug,
{
    let extra_output_error = format!(
        "Stage {} | Received unexpected additional output:\n\n{:?}",
        stage_number, output
    );

    panic!("{}", extra_output_error);
}

pub fn panic_missing_output<S>(stage_number: usize, output: &S)
where
    S: std::fmt::Debug,
{
    let output_missing_error = format!(
        "Stage {} | Failed to receive output for predicate: {:?}",
        stage_number, output
    );

    panic!("{}", output_missing_error);
}

pub fn validate_task_state_or_panic<S>(stage_number: usize, state: &S, assert: &Predicate<S>) {
    assert!(
        (assert.function)(state),
        "Stage {} | Task state failed to satisfy: {:?}",
        stage_number,
        assert
    );
}

pub fn validate_output_or_panic<S>(stage_number: usize, output: &S, assert: &Predicate<S>)
where
    S: std::fmt::Debug,
{
    assert!(
        (assert.function)(output),
        "Stage {} | Output failed to satisfy: {:?}.\n\nReceived:\n\n{:?}",
        stage_number,
        assert,
        output
    );
}

/// `run_test_script` reads a triple (inputs, outputs, asserts) in a `TestScript`,
/// It broadcasts all given inputs (in order) and waits to receive all outputs (in order).
/// Once the expected outputs have been received, it validates the task state at that stage
/// against the given assertions.
///
/// If all assertions pass, it moves onto the next stage. If it receives an unexpected output
/// or fails to receive an output, the test fails immediately with a panic.
///
/// Note: the task is not spawned with an async thread; instead, the harness just calls `handle_event`.
/// This has a few implications, e.g. shutting down tasks doesn't really make sense,
/// and event ordering is deterministic.
pub async fn run_test_script<TYPES, S: TaskState<Event = Arc<HotShotEvent<TYPES>>>>(
    mut script: TestScript<TYPES, S>,
    state: S,
) where
    TYPES: NodeType,
    S: Send + 'static,
{
    let registry = Arc::new(TaskRegistry::default());

    let (test_input, task_receiver) = broadcast(1024);
    // let (task_input, mut test_receiver) = broadcast(1024);

    let task_input = test_input.clone();
    let mut test_receiver = task_receiver.clone();

    let mut task = Task::new(
        task_input.clone(),
        task_receiver.clone(),
        registry.clone(),
        state,
    );

    for (stage_number, stage) in script.iter_mut().enumerate() {
        tracing::debug!("Beginning test stage {}", stage_number);
        for input in &stage.inputs {
            if !task.state_mut().filter(&Arc::new(input.clone())) {
                tracing::debug!("Test sent: {:?}", input.clone());

                if let Some(res) = S::handle_event(input.clone().into(), &mut task).await {
                    task.state_mut().handle_result(&res).await;
                }
            }
        }

        for assert in &stage.outputs {
            if let Ok(received_output) = test_receiver.try_recv() {
                tracing::debug!("Test received: {:?}", received_output);
                validate_output_or_panic(stage_number, &received_output, assert);
            } else {
                panic_missing_output(stage_number, assert);
            }
        }

        for assert in &stage.asserts {
            validate_task_state_or_panic(stage_number, task.state(), assert);
        }

        if let Ok(received_output) = test_receiver.try_recv() {
            panic_extra_output(stage_number, &received_output);
        }
    }
}

pub struct TaskScript<TYPES: NodeType, S> {
    pub state: S,
    pub expectations: Vec<Expectations<TYPES, S>>,
}

pub struct Expectations<TYPES: NodeType, S> {
    pub output_asserts: Vec<Predicate<Arc<HotShotEvent<TYPES>>>>,
    pub task_state_asserts: Vec<Predicate<S>>,
}

pub fn panic_extra_output_in_script<S>(stage_number: usize, script_name: String, output: &S)
where
    S: std::fmt::Debug,
{
    let extra_output_error = format!(
        "Stage {} | Received unexpected additional output in {}:\n\n{:?}",
        stage_number, script_name, output
    );

    panic!("{}", extra_output_error);
}

pub fn panic_missing_output_in_script<S>(stage_number: usize, script_name: String, predicate: &S)
where
    S: std::fmt::Debug,
{
    let output_missing_error = format!(
        "Stage {} | Failed to receive output for predicate in {}: {:?}",
        stage_number, script_name, predicate
    );

    panic!("{}", output_missing_error);
}

pub fn validate_task_state_or_panic_in_script<S>(
    stage_number: usize,
    script_name: String,
    state: &S,
    assert: &Predicate<S>,
) {
    assert!(
        (assert.function)(state),
        "Stage {} | Task state in {} failed to satisfy: {:?}",
        stage_number,
        script_name,
        assert
    );
}

pub fn validate_output_or_panic_in_script<S: std::fmt::Debug>(
    stage_number: usize,
    script_name: String,
    output: &S,
    assert: &Predicate<S>,
) {
    assert!(
        (assert.function)(output),
        "Stage {} | Output in {} failed to satisfy: {:?}.\n\nReceived:\n\n{:?}",
        stage_number,
        script_name,
        assert,
        output
    );
}
