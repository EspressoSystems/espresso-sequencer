#[cfg_attr(legacy_builder_refactored, path = "refactored/lib.rs")]
#[cfg_attr(not(legacy_builder_refactored), path = "old/lib.rs")]
mod implementation;

pub use implementation::*;
