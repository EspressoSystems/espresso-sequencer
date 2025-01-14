use crate::jellyfish::u256_to_field;
use alloy::primitives::U256;
use ark_ec::{
    short_weierstrass,
    twisted_edwards::{self, Affine, TECurveConfig},
    AffineRepr,
};
use ark_ed_on_bn254::EdwardsConfig;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use contract_bindings::permissionedstaketable::{
    EdOnBN254::EdOnBN254Point, PermissionedStakeTable::NodeInfo, BN254::G2Point,
};
use diff_test_bn254::ParsedG2Point;
use ethers_conv::{ToAlloy, ToEthers};
use hotshot_types::{light_client::StateVerKey, network::PeerConfigKeys, signature_key::BLSPubKey};
use serde::{Deserialize, Serialize};

// TODO: (alex) maybe move these commonly shared util to a crate
/// convert a field element to U256, panic if field size is larger than 256 bit
pub fn field_to_u256<F: PrimeField>(f: F) -> U256 {
    if F::MODULUS_BIT_SIZE > 256 {
        panic!("Shouldn't convert a >256-bit field to U256");
    }
    U256::from_le_slice(&f.into_bigint().to_bytes_le())
}

/// an intermediate representation of `EdOnBN254Point` in solidity.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParsedEdOnBN254Point {
    /// x coordinate of affine repr
    pub x: U256,
    /// y coordinate of affine repr
    pub y: U256,
}

// this is convention from BN256 precompile
impl Default for ParsedEdOnBN254Point {
    fn default() -> Self {
        Self {
            x: U256::from(0),
            y: U256::from(0),
        }
    }
}

impl From<ParsedEdOnBN254Point> for EdOnBN254Point {
    fn from(value: ParsedEdOnBN254Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<EdOnBN254Point> for ParsedEdOnBN254Point {
    fn from(value: EdOnBN254Point) -> Self {
        let EdOnBN254Point { x, y } = value;
        Self { x, y }
    }
}

impl<P: TECurveConfig> From<Affine<P>> for ParsedEdOnBN254Point
where
    P::BaseField: PrimeField,
{
    fn from(p: Affine<P>) -> Self {
        if p.is_zero() {
            // this convention is from the BN precompile
            Self {
                x: U256::from(0),
                y: U256::from(0),
            }
        } else {
            Self {
                x: field_to_u256::<P::BaseField>(*p.x().unwrap()),
                y: field_to_u256::<P::BaseField>(*p.y().unwrap()),
            }
        }
    }
}

impl<P: TECurveConfig> From<ParsedEdOnBN254Point> for Affine<P>
where
    P::BaseField: PrimeField,
{
    fn from(p: ParsedEdOnBN254Point) -> Self {
        if p == ParsedEdOnBN254Point::default() {
            Self::default()
        } else {
            Self::new_unchecked(
                u256_to_field::<P::BaseField>(p.x.to_ethers()),
                u256_to_field::<P::BaseField>(p.y.to_ethers()),
            )
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeInfoJf {
    pub stake_table_key: BLSPubKey,
    pub state_ver_key: StateVerKey,
    pub da: bool,
}

impl From<NodeInfoJf> for NodeInfo {
    fn from(value: NodeInfoJf) -> Self {
        let NodeInfoJf {
            stake_table_key,
            state_ver_key,
            da,
        } = value;
        let ParsedG2Point { x0, x1, y0, y1 } = stake_table_key.to_affine().into();
        let schnorr: ParsedEdOnBN254Point = state_ver_key.to_affine().into();
        Self {
            blsVK: G2Point {
                x0: x0.to_alloy(),
                x1: x1.to_alloy(),
                y0: y0.to_alloy(),
                y1: y1.to_alloy(),
            },
            schnorrVK: schnorr.into(),
            isDA: da,
        }
    }
}

impl From<NodeInfo> for NodeInfoJf {
    fn from(value: NodeInfo) -> Self {
        let NodeInfo {
            blsVK,
            schnorrVK,
            isDA,
        } = value;
        let stake_table_key = {
            let g2 = diff_test_bn254::ParsedG2Point {
                x0: blsVK.x0.to_ethers(),
                x1: blsVK.x1.to_ethers(),
                y0: blsVK.y0.to_ethers(),
                y1: blsVK.y1.to_ethers(),
            };
            let g2_affine = short_weierstrass::Affine::<ark_bn254::g2::Config>::from(g2);
            let mut bytes = vec![];
            // TODO: remove serde round-trip once jellyfin provides a way to
            // convert from Affine representation to VerKey.
            //
            // Serialization and de-serialization shouldn't fail.
            g2_affine
                .into_group()
                .serialize_compressed(&mut bytes)
                .unwrap();
            BLSPubKey::deserialize_compressed(&bytes[..]).unwrap()
        };
        let state_ver_key = {
            let g1_point: ParsedEdOnBN254Point = schnorrVK.into();
            let state_sk_affine = twisted_edwards::Affine::<EdwardsConfig>::from(g1_point);
            StateVerKey::from(state_sk_affine)
        };
        Self {
            stake_table_key,
            state_ver_key,
            da: isDA,
        }
    }
}

impl From<PeerConfigKeys<BLSPubKey>> for NodeInfoJf {
    fn from(value: PeerConfigKeys<BLSPubKey>) -> Self {
        let PeerConfigKeys {
            stake_table_key,
            state_ver_key,
            da,
            ..
        } = value;
        Self {
            stake_table_key,
            state_ver_key,
            da,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_std::rand::{Rng, RngCore};
    use hotshot_types::{light_client::StateKeyPair, traits::signature_key::BuilderSignatureKey};

    impl NodeInfoJf {
        fn random(rng: &mut impl RngCore) -> Self {
            let mut seed = [0u8; 32];
            rng.fill_bytes(&mut seed);

            let (stake_table_key, _) = BLSPubKey::generated_from_seed_indexed(seed, 0);
            let state_key_pair = StateKeyPair::generate_from_seed_indexed(seed, 0);
            Self {
                stake_table_key,
                state_ver_key: state_key_pair.ver_key(),
                da: rng.gen(),
            }
        }
    }

    #[test]
    fn test_node_info_round_trip() {
        let mut rng = ark_std::rand::thread_rng();
        for _ in 0..20 {
            let jf = NodeInfoJf::random(&mut rng);
            let sol: NodeInfo = jf.clone().into();
            let jf2: NodeInfoJf = sol.into();
            assert_eq!(jf2, jf);
        }
    }
}
