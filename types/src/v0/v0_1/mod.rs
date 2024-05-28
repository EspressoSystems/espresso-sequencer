use vbs::version::Version;

pub const VERSION: Version = Version { major: 0, minor: 1 };

mod chain_config;
mod fee_info;
mod header;
mod l1;
mod ns_table;
mod signature;
mod state;

pub use chain_config::{ChainConfig, ChainId, ResolvableChainConfig};
pub use fee_info::{FeeAccount, FeeAmount, FeeInfo};
pub use header::Header;
pub use l1::L1BlockInfo;
pub use ns_table::{NameSpaceTable, TxTableEntryWord};
pub use signature::BuilderSignature;
pub use state::{BlockMerkleCommitment, BlockMerkleTree, FeeMerkleCommitment, FeeMerkleTree};
