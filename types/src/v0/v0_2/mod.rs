use vbs::version::Version;

use super::header::ResolvableChainConfigOrVersion;
// Re-export types which haven't changed since the last minor version.
pub use super::v0_1::{
    AccountQueryData, BlockMerkleCommitment, BlockMerkleTree, BlockSize, BuilderSignature,
    BuilderValidationError, ChainConfig, ChainId, Delta, FeeAccount, FeeAccountProof, FeeAmount,
    FeeError, FeeInfo, FeeMerkleCommitment, FeeMerkleProof, FeeMerkleTree, FromNsPayloadBytes,
    GenesisHeader, Header, Index, Iter, L1BlockInfo, L1Client, L1Snapshot, NamespaceId, NodeState,
    NsIndex, NsIter, NsPayload, NsPayloadBuilder, NsPayloadByteLen, NsPayloadBytesRange,
    NsPayloadOwned, NsPayloadRange, NsProof, NsTable, NsTableBuilder, NsTableValidationError,
    NumNss, NumTxs, NumTxsRange, NumTxsUnchecked, Payload, PayloadByteLen, PersistenceOptions,
    ProposalValidationError, ResolvableChainConfig, SequencerPersistence, StateCatchup,
    StateValidationError, Timestamp, Transaction, TxIndex, TxIter, TxPayload, TxPayloadRange,
    TxProof, TxTableEntries, TxTableEntriesRange, Upgrade, UpgradeType, ValidatedState,
    BACKOFF_FACTOR, BACKOFF_JITTER, BLOCK_MERKLE_TREE_HEIGHT, FEE_MERKLE_TREE_HEIGHT,
    MAX_RETRY_DELAY, MIN_RETRY_DELAY, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN, NUM_NSS_BYTE_LEN,
    NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};

pub const VERSION: Version = Version { major: 0, minor: 2 };

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct VersionedHeader {
    pub version: ResolvableChainConfigOrVersion,
    pub fields: Header,
}
