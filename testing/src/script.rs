use crate::predicates::Predicate;
use async_broadcast::broadcast;
use hotshot_task_impls::events::HotShotEvent;

use hotshot_task::task::{Task, TaskRegistry, TaskState};
use hotshot_types::traits::node_implementation::NodeType;
use std::sync::Arc;

/// A `TestScript` is a sequence of triples (input sequence, output sequence, assertions).
type TestScript<TYPES, S> = Vec<TestScriptStage<TYPES, S>>;

pub struct TestScriptStage<TYPES: NodeType, S: TaskState<Event = HotShotEvent<TYPES>>> {
    pub inputs: Vec<HotShotEvent<TYPES>>,
    pub outputs: Vec<Predicate<HotShotEvent<TYPES>>>,
    pub asserts: Vec<Predicate<S>>,
}

impl<INPUT> std::fmt::Debug for Predicate<INPUT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.info)
    }
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
pub async fn run_test_script<TYPES, S: TaskState<Event = HotShotEvent<TYPES>>>(
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
        for input in &mut *stage.inputs {
            if !task.state_mut().filter(input) {
                tracing::debug!("Test sent: {:?}", input);

                if let Some(res) = S::handle_event(input.clone(), &mut task).await {
                    task.state_mut().handle_result(&res).await;
                }
            }
        }

        for expected in &stage.outputs {
            let output_missing_error = format!(
                "Stage {} | Failed to receive output for predicate: {:?}",
                stage_number, expected
            );

            if let Ok(received_output) = test_receiver.try_recv() {
                tracing::debug!("Test received: {:?}", received_output);
                assert!(
                    (expected.function)(&received_output),
                    "Stage {} | Output failed to satisfy {:?}",
                    stage_number,
                    expected
                );
            } else {
                panic!("{}", output_missing_error);
            }
        }

        for assert in &stage.asserts {
            assert!(
                (assert.function)(task.state()),
                "Stage {} | Assertion on task state failed: {:?}",
                stage_number,
                assert
            );
        }

        if let Ok(received_output) = test_receiver.try_recv() {
            let extra_output_error = format!(
                "Stage {} | Received unexpected additional output: {:?}",
                stage_number, received_output
            );

            panic!("{}", extra_output_error);
        }
    }
}
