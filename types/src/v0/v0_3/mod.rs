use vbs::version::Version;

// Re-export types which haven't changed since the last minor version.
pub use super::v0_1::{
    AccountQueryData, BlockMerkleCommitment, BlockMerkleTree, BlockSize, BuilderSignature, ChainId,
    Delta, FeeAccount, FeeAccountProof, FeeAmount, FeeInfo, FeeMerkleCommitment, FeeMerkleProof,
    FeeMerkleTree, Index, Iter, L1BlockInfo, L1Client, L1Snapshot, NamespaceId, NodeState, NsIndex,
    NsIter, NsPayload, NsPayloadBuilder, NsPayloadByteLen, NsPayloadOwned, NsPayloadRange, NsProof,
    NsTable, NsTableBuilder, NsTableValidationError, NumNss, NumTxs, NumTxsRange, NumTxsUnchecked,
    Payload, PayloadByteLen, Transaction, TxIndex, TxIter, TxPayload, TxPayloadRange, TxProof,
    TxTableEntries, TxTableEntriesRange, Upgrade, UpgradeType, ValidatedState,
    BLOCK_MERKLE_TREE_HEIGHT, FEE_MERKLE_TREE_HEIGHT, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN,
    NUM_NSS_BYTE_LEN, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};

pub const VERSION: Version = Version { major: 0, minor: 3 };

mod auction;
mod chain_config;
mod fee_info;
mod header;

pub use auction::{BidTx, BidTxBody, FullNetworkTx};
pub use chain_config::*;
pub use fee_info::IterableFeeInfo;
pub use header::Header;
