use ark_ec::{short_weierstrass, twisted_edwards};
use ark_ed_on_bn254::EdwardsConfig;
use contract_bindings::permissioned_stake_table::{EdOnBN254Point, NodeInfo};
use derive_more::derive::From;
use ethers::types::U256;
use hotshot::types::{BLSPubKey, SignatureKey};
use hotshot_contract_adapter::stake_table::ParsedG1Point;
use hotshot_types::{light_client::StateVerKey, stake_table::StakeTableEntry, PeerConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
struct PermissionedPeerConfig(pub(crate) PeerConfig<BLSPubKey>);

impl From<NodeInfo> for PermissionedPeerConfig {
    fn from(node_info: NodeInfo) -> Self {
        let NodeInfo {
            bls_vk,
            schnorr_vk,
            // TODO: handle DA flag
            ..
        } = node_info;
        let stake_table_entry = {
            let g2 = diff_test_bn254::ParsedG2Point {
                x0: bls_vk.x_0,
                x1: bls_vk.x_1,
                y0: bls_vk.y_0,
                y1: bls_vk.y_1,
            };
            let g2_affine = short_weierstrass::Affine::<ark_bn254::g2::Config>::from(g2);
            // let g2_proj = bn::G2Projective::from(g2_affine);
            // TODO
            let stake_key = BLSPubKey::generated_from_seed_indexed(Default::default(), 0).0;
            StakeTableEntry {
                stake_key,
                stake_amount: U256::from(0),
            }
        };
        let state_ver_key = {
            let EdOnBN254Point { x, y } = schnorr_vk;
            let g1_point = ParsedG1Point { x, y };
            let state_sk_affine = twisted_edwards::Affine::<EdwardsConfig>::from(g1_point);
            StateVerKey::from(state_sk_affine)
        };
        PeerConfig {
            stake_table_entry,
            state_ver_key,
        }
        .into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StakeTable(Vec<PeerConfig<BLSPubKey>>);
