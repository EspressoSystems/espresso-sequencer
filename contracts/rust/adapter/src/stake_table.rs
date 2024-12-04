use crate::jellyfish::u256_to_field;
use ark_ec::{
    twisted_edwards::{Affine, TECurveConfig},
    AffineRepr,
};
use ark_ff::{BigInteger, Fp2, Fp2Config, PrimeField};
use contract_bindings::permissioned_stake_table::{EdOnBN254Point, NodeInfo};
use derive_more::derive::From;
use ethers::{
    abi::AbiDecode,
    prelude::{AbiError, EthAbiCodec, EthAbiType},
    types::U256,
};
use hotshot_types::{network::PeerConfigKeys, signature_key::BLSPubKey};
use std::str::FromStr;

// TODO: (alex) maybe move these commonly shared util to a crate
/// convert a field element to U256, panic if field size is larger than 256 bit
pub fn field_to_u256<F: PrimeField>(f: F) -> U256 {
    if F::MODULUS_BIT_SIZE > 256 {
        panic!("Shouldn't convert a >256-bit field to U256");
    }
    U256::from_little_endian(&f.into_bigint().to_bytes_le())
}

/// an intermediate representation of `EdOnBN254Point` in solidity.
#[derive(Clone, PartialEq, Eq, Debug, EthAbiType, EthAbiCodec)]
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

impl FromStr for ParsedEdOnBN254Point {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
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
                u256_to_field::<P::BaseField>(p.x),
                u256_to_field::<P::BaseField>(p.y),
            )
        }
    }
}

// /// Intermediate representation of `G2Point` in Solidity
// #[derive(Clone, PartialEq, Eq, Debug, EthAbiType, EthAbiCodec)]
// pub struct ParsedG2Point {
//     /// x0 of x = x0 + u * x1 coordinate
//     pub x0: U256,
//     /// x1 of x = x0 + u * x1 coordinate
//     pub x1: U256,
//     /// y0 of y = y0 + u * y1 coordinate
//     pub y0: U256,
//     /// y1 of y = y0 + u * y1 coordinate
//     pub y1: U256,
// }

// impl FromStr for ParsedG2Point {
//     type Err = AbiError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let parsed: (Self,) = AbiDecode::decode_hex(s)?;
//         Ok(parsed.0)
//     }
// }

// impl<P: TECurveConfig<BaseField = Fp2<C>>, C> From<ParsedG2Point> for Affine<P>
// where
//     C: Fp2Config,
// {
//     fn from(p: ParsedG2Point) -> Self {
//         Self::new_unchecked(
//             Fp2::new(u256_to_field(p.x0), u256_to_field(p.x1)),
//             Fp2::new(u256_to_field(p.y0), u256_to_field(p.y1)),
//         )
//     }
// }

// impl<P: TECurveConfig<BaseField = Fp2<C>>, C> From<Affine<P>> for ParsedG2Point
// where
//     C: Fp2Config,
// {
//     fn from(p: Affine<P>) -> Self {
//         Self {
//             x0: field_to_u256(p.x().unwrap().c0),
//             x1: field_to_u256(p.x().unwrap().c1),
//             y0: field_to_u256(p.y().unwrap().c0),
//             y1: field_to_u256(p.y().unwrap().c1),
//         }
//     }
// }

// pub trait ToAffine {
//     fn to_affine(&self) -> Affine<P>;
// }
// impl<P: TECurveConfig<BaseField = Fp2<C>>, C> ToAffine for BLSPubKey
// where
//     C: Fp2Config,
// {
//     fn to_affine(&self) -> Affine<P> {
//         self.to_affine()
//     }
// }

#[derive(Clone, Debug, From)]
pub struct NodeInfoSol(NodeInfo);

// impl From<> for NodeInfoSol {
//     fn from(p: PeerConfigKeys<BLSPubKey>) -> Self {
//         let bls: ParsedG2Point = p.stake_table_key.to_affine().into();
//         let schnorr: ParsedG1Point = p.state_ver_key.to_affine().into();
//         Self(NodeInfo {
//             bls_vk: bls.into(),
//             schnorr_vk: schnorr.into(),
//             is_da: p.da,
//         })
//     }
// }

// impl From<NodeInfoSol> for PeerConfigKeys<BLSPubKey> {
//     fn from(p: NodeInfoSol) -> Self {
//         PeerConfigKeys {
//             bls_pub_key: p.0.bls_pub_key,
//             ip: p.0.ip,
//             port: p.0.port,
//         }
//     }
// }
