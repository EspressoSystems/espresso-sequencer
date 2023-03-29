mod bindings;
pub use bindings::*;

mod deploy;
pub use deploy::*;

pub use bindings::{hot_shot::HotShot, matic::Matic, polygon_zk_evm::PolygonZkEVM};
