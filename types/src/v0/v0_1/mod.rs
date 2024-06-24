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

pub use chain_config::{ChainConfig, ChainId, ResolvableChainConfig};

pub use block::{
    NameSpaceTable, NamespaceInfo, NamespaceProof, NsTable, Payload, Table, TableWordTraits,
    TxIndex, TxIterator, TxTable, TxTableEntry, TxTableEntryWord,
};
pub use fee_info::{FeeAccount, FeeAmount, FeeInfo};
pub use header::Header;
pub use instance_state::{L1Client, NodeState, StateCatchup};
pub use l1::{L1BlockInfo, L1Snapshot};
pub use signature::BuilderSignature;
pub use state::{
    AccountQueryData, BlockMerkleCommitment, BlockMerkleTree, FeeAccountProof, FeeMerkleCommitment,
    FeeMerkleTree, ValidatedState,
};
pub use transaction::{NamespaceId, Transaction};
