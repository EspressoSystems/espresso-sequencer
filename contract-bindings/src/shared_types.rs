///`Node(address,uint8,uint64,uint64,uint64,(uint256,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Node {
    pub account: ::ethers::core::types::Address,
    pub stake_type: u8,
    pub balance: u64,
    pub register_epoch: u64,
    pub exit_epoch: u64,
    pub schnorr_vk: EdOnBN254Point,
}
///`G1Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
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
    serde::Serialize,
    serde::Deserialize,
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
///`EdOnBN254Point(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct EdOnBN254Point {
    pub x: ::ethers::core::types::U256,
    pub y: ::ethers::core::types::U256,
}
///`PlonkProof((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PlonkProof {
    pub wire_0: G1Point,
    pub wire_1: G1Point,
    pub wire_2: G1Point,
    pub wire_3: G1Point,
    pub wire_4: G1Point,
    pub prod_perm: G1Point,
    pub split_0: G1Point,
    pub split_1: G1Point,
    pub split_2: G1Point,
    pub split_3: G1Point,
    pub split_4: G1Point,
    pub zeta: G1Point,
    pub zeta_omega: G1Point,
    pub wire_eval_0: ::ethers::core::types::U256,
    pub wire_eval_1: ::ethers::core::types::U256,
    pub wire_eval_2: ::ethers::core::types::U256,
    pub wire_eval_3: ::ethers::core::types::U256,
    pub wire_eval_4: ::ethers::core::types::U256,
    pub sigma_eval_0: ::ethers::core::types::U256,
    pub sigma_eval_1: ::ethers::core::types::U256,
    pub sigma_eval_2: ::ethers::core::types::U256,
    pub sigma_eval_3: ::ethers::core::types::U256,
    pub prod_perm_zeta_omega_eval: ::ethers::core::types::U256,
}
