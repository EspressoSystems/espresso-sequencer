//! Testing infrastructure for `HotShot`

#![cfg_attr(
    // hotshot_example option is set manually in justfile when running examples
    not(any(test, debug_assertions, hotshot_example)),
    deprecated = "suspicious usage of testing/demo implementations in non-test/non-debug build"
)]

/// Helpers for initializing system context handle and building tasks.
pub mod helpers;

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

/// Test implementation of block builder
pub mod block_builder;

/// predicates to use in tests
pub mod predicates;

/// scripting harness for tests
pub mod script;

/// view generator for tests
pub mod view_generator;

/// global event at the test level
#[derive(Clone, Debug)]
pub enum GlobalTestEvent {
    /// the test is shutting down
    ShutDown,
}
