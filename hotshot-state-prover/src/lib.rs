//! SNARK-assisted `HotShot` light client state update verification

/// State verifier circuit builder
pub mod circuit;
/// Prover service related functionalities
pub mod service;
/// SNARK proof generation
pub mod snark;
/// Utilities for test
#[cfg(test)]
mod test_utils;

pub use snark::*;
