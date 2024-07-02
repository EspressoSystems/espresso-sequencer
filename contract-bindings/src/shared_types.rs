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
///`LightClientState(uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256)`
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
pub struct LightClientState {
    pub view_num: u64,
    pub block_height: u64,
    pub block_comm_root: ::ethers::core::types::U256,
    pub fee_ledger_comm: ::ethers::core::types::U256,
    pub stake_table_bls_key_comm: ::ethers::core::types::U256,
    pub stake_table_schnorr_key_comm: ::ethers::core::types::U256,
    pub stake_table_amount_comm: ::ethers::core::types::U256,
    pub threshold: ::ethers::core::types::U256,
}
