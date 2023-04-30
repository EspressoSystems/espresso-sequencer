///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct G1Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`G2Point(uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct G2Point {
    pub x_0: ::ethers::core::types::U256,
    pub x_1: ::ethers::core::types::U256,
    pub y_0: ::ethers::core::types::U256,
    pub y_1: ::ethers::core::types::U256,
}
///`InitializePackedParameters(address,address,uint64,address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InitializePackedParameters {
    pub admin: ::ethers::core::types::Address,
    pub trusted_sequencer: ::ethers::core::types::Address,
    pub pending_state_timeout: u64,
    pub trusted_aggregator: ::ethers::core::types::Address,
    pub trusted_aggregator_timeout: u64,
}
///`PackedHotShotParams(bytes32,bytes32,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PackedHotShotParams {
    pub old_acc_input_hash: [u8; 32],
    pub new_acc_input_hash: [u8; 32],
    pub comm_proof: ::ethers::core::types::Bytes,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
