use vbs::version::Version;

// Re-export types which haven't changed since the last minor vresion.
pub use super::v0_1::{
    BlockMerkleCommitment, BlockMerkleTree, BuilderSignature, ChainConfig, ChainId, FeeAccount,
    FeeAmount, FeeInfo, FeeMerkleCommitment, FeeMerkleTree, L1BlockInfo, NameSpaceTable,
    ResolvableChainConfig, TxTableEntryWord,
};

pub const VERSION: Version = Version { major: 0, minor: 1 };

mod header;

pub use header::Header;
