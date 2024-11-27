use crate::PubKey;
use ark_ec::{
    bn,
    short_weierstrass::{self, Projective},
    twisted_edwards, AffineRepr,
};
use ark_ed_on_bn254::EdwardsConfig;
use ark_serialize::{CanonicalDeserialize as _, CanonicalSerialize};
use contract_bindings::permissioned_stake_table::{EdOnBN254Point, NodeInfo};
use derive_more::derive::From;
use hotshot_contract_adapter::stake_table::ParsedG1Point;
use hotshot_types::{light_client::StateVerKey, network::PeerConfigKeys};
use jf_utils::to_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct PermissionedPeerConfig(pub(crate) PeerConfigKeys<PubKey>);

/// Stake table holding all staking information (DA and non-DA stakers)
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct CombinedStakeTable(Vec<PeerConfigKeys<PubKey>>);

impl From<NodeInfo> for PermissionedPeerConfig {
    fn from(node_info: NodeInfo) -> Self {
        let NodeInfo {
            bls_vk,
            schnorr_vk,
            is_da,
        } = node_info;
        let stake_table_key = {
            let g2 = diff_test_bn254::ParsedG2Point {
                x0: bls_vk.x_0,
                x1: bls_vk.x_1,
                y0: bls_vk.y_0,
                y1: bls_vk.y_1,
            };
            let g2_affine = short_weierstrass::Affine::<ark_bn254::g2::Config>::from(g2);
            // TODO: remove unwrap
            let b = to_bytes!(&g2_affine.into_group()).unwrap();
            PubKey::deserialize_compressed(&b[..]).unwrap()
        };
        let state_ver_key = {
            let EdOnBN254Point { x, y } = schnorr_vk;
            let g1_point = ParsedG1Point { x, y };
            let state_sk_affine = twisted_edwards::Affine::<EdwardsConfig>::from(g1_point);
            StateVerKey::from(state_sk_affine)
        };
        PeerConfigKeys {
            stake_table_key,
            state_ver_key,
            da: is_da,
            stake: 1,
        }
        .into()
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct StakeTable(Vec<PeerConfig<BLSPubKey>>);
