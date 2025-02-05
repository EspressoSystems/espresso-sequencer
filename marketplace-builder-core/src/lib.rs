//! Marketplace builder
//! It mainly provides this API service to hotshot proposers:
//! 1. Serves a proposer(leader)'s request to provide a bundle of transactions
//!
//! It also provides one API service to external users:
//! 1. Serves a user's request to submit a private transaction
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod hooks;
pub mod service;

// tracking the testing
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod testing;
