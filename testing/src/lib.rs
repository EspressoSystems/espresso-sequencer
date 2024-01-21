//! Testing infrastructure for `HotShot`

#![cfg_attr(
    // hotshot_example option is set manually in justfile when running examples
    not(any(test, debug_assertions, hotshot_example)),
    deprecated = "suspicious usage of testing/demo implementations in non-test/non-debug build"
)]
#![allow(clippy::panicking_unwrap)]

use hotshot_task::{event_stream::ChannelStream, task_impls::HSTWithEvent};

/// Helpers for initializing system context handle and building tasks.
pub mod task_helpers;

///  builder
pub mod test_builder;

/// launcher
pub mod test_launcher;

/// runner
pub mod test_runner;

/// task that's consuming events and asserting safety
pub mod overall_safety_task;

/// task that's submitting transactions to the stream
pub mod txn_task;

/// task that decides when things are complete
pub mod completion_task;

/// task to spin nodes up and down
pub mod spinning_task;

/// task for checking if view sync got activated
pub mod view_sync_task;

/// block types
pub mod block_types;

/// Implementations for testing/examples
pub mod state_types;

/// node types
pub mod node_types;

/// global event at the test level
#[derive(Clone, Debug)]
pub enum GlobalTestEvent {
    /// the test is shutting down
    ShutDown,
}

/// the reason for shutting down the test
pub enum ShutDownReason {
    /// the test is shutting down because of a safety violation
    SafetyViolation,
    /// the test is shutting down because the test has completed successfully
    SuccessfullyCompleted,
}

/// type alias for the type of tasks created in testing
pub type TestTask<ERR, STATE> =
    HSTWithEvent<ERR, GlobalTestEvent, ChannelStream<GlobalTestEvent>, STATE>;
