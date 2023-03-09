mod bindings;
pub use bindings::*;

mod deploy;
pub use deploy::*;

pub use bindings::{matic::Matic, polygon_zk_evm::PolygonZkEVM};
