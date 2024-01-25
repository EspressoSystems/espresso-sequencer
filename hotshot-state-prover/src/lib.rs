//! SNARK-assisted `HotShot` light client state update verification

/// State verifier circuit builder
pub mod circuit;
/// State proof generation
pub mod proof;
/// Prover service related functionalities
pub mod service;
/// Light client state related structs
pub mod state;
/// Utilities for test
#[cfg(test)]
mod utils;

/// Base field
pub type BaseField = ark_ed_on_bn254::Fq;
