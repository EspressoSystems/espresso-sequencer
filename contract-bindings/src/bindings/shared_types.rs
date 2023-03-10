#[doc = "`ForcedBatchData(bytes,bytes32,uint64)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ForcedBatchData {
    pub transactions: ethers::core::types::Bytes,
    pub global_exit_root: [u8; 32],
    pub min_forced_timestamp: u64,
}
#[doc = "`InitializePackedParameters(address,address,uint64,address,uint64)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct InitializePackedParameters {
    pub admin: ethers::core::types::Address,
    pub trusted_sequencer: ethers::core::types::Address,
    pub pending_state_timeout: u64,
    pub trusted_aggregator: ethers::core::types::Address,
    pub trusted_aggregator_timeout: u64,
}
#[doc = "`BatchData(bytes,bytes32,uint64,uint64)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct BatchData {
    pub transactions: ethers::core::types::Bytes,
    pub global_exit_root: [u8; 32],
    pub timestamp: u64,
    pub min_forced_timestamp: u64,
}
