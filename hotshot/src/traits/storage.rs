//! Abstraction over on-disk storage of node state
// pub mod atomic_storage;
pub mod memory_storage;

pub use hotshot_types::traits::storage::{Result, Storage};
