mod bindings;
pub use bindings::*;

mod deploy;
pub use deploy::*;

pub use bindings::{matic::Matic, proof_of_efficiency::ProofOfEfficiency};
