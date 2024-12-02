use vbs::version::Version;

// Re-export types which haven't changed since the last minor version.
pub use super::v0_1::{
    AccountQueryData, BlockMerkleCommitment, BlockMerkleTree, BlockSize, BuilderSignature, ChainId,
    Delta, FeeAccount, FeeAccountProof, FeeAmount, FeeInfo, FeeMerkleCommitment, FeeMerkleProof,
    FeeMerkleTree, Index, Iter, L1BlockInfo, L1Client, L1ClientOptions, L1Snapshot, NamespaceId,
    NsIndex, NsIter, NsPayload, NsPayloadBuilder, NsPayloadByteLen, NsPayloadOwned, NsPayloadRange,
    NsProof, NsTable, NsTableBuilder, NsTableValidationError, NumNss, NumTxs, NumTxsRange,
    NumTxsUnchecked, Payload, PayloadByteLen, TimeBasedUpgrade, Transaction, TxIndex, TxIter,
    TxPayload, TxPayloadRange, TxProof, TxTableEntries, TxTableEntriesRange, Upgrade, UpgradeMode,
    UpgradeType, ViewBasedUpgrade, BLOCK_MERKLE_TREE_HEIGHT, FEE_MERKLE_TREE_HEIGHT,
    NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
pub(crate) use super::v0_1::{L1Event, L1ReconnectTask, L1State, L1UpdateTask, RpcClient};

pub const VERSION: Version = Version { major: 0, minor: 3 };

mod auction;
mod chain_config;
mod fee_info;
mod header;
mod solver;

pub use auction::*;
pub use chain_config::*;
pub use fee_info::IterableFeeInfo;
pub use header::Header;
pub use solver::*;
