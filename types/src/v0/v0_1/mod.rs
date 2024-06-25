use vbs::version::Version;

pub const VERSION: Version = Version { major: 0, minor: 1 };

mod chain_config;

mod block;
mod fee_info;
mod header;
mod instance_state;
mod l1;
mod signature;
mod state;
mod transaction;

pub use chain_config::{ChainConfig, ChainId, ResolvableChainConfig, *};

pub use block::*;
pub use fee_info::{FeeAccount, FeeAmount, FeeInfo, *};
pub use header::Header;
pub use instance_state::{NodeState, StateCatchup, *};
pub use l1::{L1BlockInfo, L1Snapshot, *};
pub use signature::BuilderSignature;
pub use state::{
    BlockMerkleCommitment, BlockMerkleTree, FeeMerkleCommitment, FeeMerkleTree, ValidatedState, *,
};
pub use transaction::{NamespaceId, Transaction};
