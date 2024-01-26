//! SNARK-assisted `HotShot` light client state update verification

/// State verifier circuit builder
pub mod circuit;
/// Prover service related functionalities
pub mod service;
/// SNARK proof generation
pub mod snark;
/// Light client state related structs
pub mod state;
/// Utilities for test
#[cfg(test)]
mod utils;

/// Base field
pub type CircuitField = ark_ed_on_bn254::Fq;
