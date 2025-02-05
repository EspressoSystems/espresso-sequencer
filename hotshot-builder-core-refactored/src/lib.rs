//! Builder Phase 1
//! It mainly provides three API services to hotshot proposers:
//! 1. Serves a proposer(leader)'s request to provide blocks information
//! 2. Serves a proposer(leader)'s request to provide the full blocks information
//! 3. Serves a proposer(leader)'s request to provide the block header information
//!
//! It also provides one API service to external users:
//! 1. Serves a user's request to submit a private transaction
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod block_size_limits;
pub mod block_store;
pub mod service;

// tracking the testing
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod testing;
