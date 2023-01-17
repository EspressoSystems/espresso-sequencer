pub mod bindings;
mod contract;

pub use bindings::{
    matic::{self, Matic},
    proof_of_efficiency::{self, ProofOfEfficiency},
};
