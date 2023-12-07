#![cfg(any(test, feature = "testing"))]

use ark_ec::AffineRepr;
use ark_ff::PrimeField;
pub use contract_bindings::shared_types::{G1Point, G2Point};
use ethers::types::U256;

pub(crate) mod hotshot_contract {}

// TODO put somewhere else, like jellyfish?
fn convert_fq_to_u256(f: ark_bn254::Fq) -> U256 {
    let b_int = f.into_bigint();
    U256([b_int.0[0], b_int.0[1], b_int.0[2], b_int.0[3]])
}

// TODO Some of the code below are copied/adapted from https://github.com/EspressoSystems/cape/blob/77f343849db3d9e721c6e3d0f7108155c178dbee/contracts/rust/src/types.rs#L41
// TODO create a library solidity/rust library that can be used across projects
// TODO note though we should update this library to arkworks 4

// TODO how to avoid this wrapping as a work around for this error? https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
#[derive(Clone)]
pub struct MyG1Point {
    pub x: ethers::core::types::U256,
    pub y: ethers::core::types::U256,
}

#[derive(Clone)]
pub struct MyG2Point {
    pub x_0: ethers::core::types::U256,
    pub x_1: ethers::core::types::U256,
    pub y_0: ethers::core::types::U256,
    pub y_1: ethers::core::types::U256,
}

impl From<MyG2Point> for G2Point {
    fn from(val: MyG2Point) -> G2Point {
        G2Point {
            x_0: val.x_0,
            x_1: val.x_1,
            y_0: val.y_0,
            y_1: val.y_1,
        }
    }
}

impl From<MyG1Point> for G1Point {
    fn from(val: MyG1Point) -> G1Point {
        G1Point { x: val.x, y: val.y }
    }
}

impl From<ark_bn254::G1Affine> for MyG1Point {
    fn from(p: ark_bn254::G1Affine) -> Self {
        if p.is_zero() {
            // Solidity precompile have a different affine repr for Point of Infinity
            Self {
                x: U256::from(0),
                y: U256::from(0),
            }
        } else {
            Self {
                x: convert_fq_to_u256(p.x),
                y: convert_fq_to_u256(p.y),
            }
        }
    }
}

impl From<(ark_bn254::Fq, ark_bn254::Fq)> for MyG1Point {
    fn from(p: (ark_bn254::Fq, ark_bn254::Fq)) -> Self {
        let zero = ark_bn254::G1Affine::zero();
        if p.0 == zero.x && p.1 == zero.y {
            // Solidity repr of infinity/zero
            Self {
                x: U256::from(0),
                y: U256::from(0),
            }
        } else {
            Self {
                x: convert_fq_to_u256(p.0),
                y: convert_fq_to_u256(p.1),
            }
        }
    }
}

impl From<ark_bn254::G2Affine> for MyG2Point {
    fn from(p: ark_bn254::G2Affine) -> Self {
        // NOTE: in contract, x = x0 * z + x1, whereas in arkwork x = c0 + c1 * X.

        Self {
            x_0: convert_fq_to_u256(p.x.c1),
            x_1: convert_fq_to_u256(p.x.c0),
            y_0: convert_fq_to_u256(p.y.c1),
            y_1: convert_fq_to_u256(p.y.c0),
        }
    }
}
