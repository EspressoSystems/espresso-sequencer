#![cfg(any(test, feature = "testing"))]

use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use ark_serialize::{CanonicalSerialize, Compress};
pub use contract_bindings::hot_shot::{G1Point, G2Point};
use ethers::types::U256;

pub(crate) mod hotshot_contract {
    use contract_bindings::hot_shot::HotShot;
    use contract_bindings::TestClients;
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::Wallet;
    use ethers::providers::{Http, Middleware, Provider};
    use sequencer_utils::Anvil;
    use std::time::Duration;
    pub(crate) async fn get_hotshot_contract_and_provider() -> (
        HotShot<
            SignerMiddleware<
                ethers::providers::Provider<Http>,
                Wallet<ethers::core::k256::ecdsa::SigningKey>,
            >,
        >,
        Provider<Http>,
    ) {
        let anvil = Anvil::spawn(None).await;
        let mut provider = Provider::try_from(&anvil.url().to_string()).unwrap();
        provider.set_interval(Duration::from_millis(10));

        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let deployer = clients.deployer.clone();

        let hotshot = HotShot::deploy(deployer.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        (hotshot, provider)
    }
}

// TODO these functions are copied from https://github.com/EspressoSystems/cape/blob/77f343849db3d9e721c6e3d0f7108155c178dbee/contracts/rust/src/types.rs#L41
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
            let x_bytes = serialize_field_element(p.x);
            let y_bytes = serialize_field_element(p.y);
            Self {
                x: U256::from_little_endian(&x_bytes),
                y: U256::from_little_endian(&y_bytes),
                // x: convert_fq_to_u256(p.x),
                // y: convert_fq_to_u256(p.y),
            }
        }
    }
}

// TODO put somewhere else, like jellyfish?
// TODO is it correct?
fn serialize_field_element(f: ark_bn254::Fq) -> Vec<u8> {
    let mut f_bytes = vec![0u8; f.serialized_size(Compress::Yes)];
    let res = f.serialize_compressed(&mut f_bytes);
    assert!(res.is_ok());
    f_bytes[32..].to_vec() // TODO why are the first 32 set to 0?
}
#[allow(dead_code)]
fn convert_fq_to_u256(f: ark_bn254::Fq) -> U256 {
    let b_int = f.into_bigint();
    U256([b_int.0[0], b_int.0[1], b_int.0[2], b_int.0[3]])
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
            let x_bytes = serialize_field_element(p.0);
            let y_bytes = serialize_field_element(p.1);
            Self {
                x: U256::from_little_endian(&x_bytes),
                y: U256::from_little_endian(&y_bytes),
                // x: convert_fq_to_u256(p.0),
                // y: convert_fq_to_u256(p.1),
            }
        }
    }
}

// impl From<MyG1Point> for ark_bn254::G1Affine {
//     fn from(p_sol: MyG1Point) -> Self {
//         if p_sol.x.is_zero() && p_sol.y.is_zero() {
//             Self::zero()
//         } else {
//             Self::new(u256_to_field(p_sol.x), u256_to_field(p_sol.y))
//         }
//     }
// }

impl From<ark_bn254::G2Affine> for MyG2Point {
    fn from(p: ark_bn254::G2Affine) -> Self {
        // NOTE: in contract, x = x0 * z + x1, whereas in arkwork x = c0 + c1 * X.

        let p_x_c1_bytes = serialize_field_element(p.x.c1);
        let p_x_c0_bytes = serialize_field_element(p.x.c0);
        let p_y_c1_bytes = serialize_field_element(p.y.c1);
        let p_y_c0_bytes = serialize_field_element(p.y.c0);

        Self {
            // x_0: convert_fq_to_u256(p.x.c1),
            // x_1: convert_fq_to_u256(p.x.c0),
            // y_0: convert_fq_to_u256(p.y.c1),
            // y_1: convert_fq_to_u256(p.y.c0),
            x_0: U256::from_little_endian(&p_x_c1_bytes),
            x_1: U256::from_little_endian(&p_x_c0_bytes),
            y_0: U256::from_little_endian(&p_y_c1_bytes),
            y_1: U256::from_little_endian(&p_y_c0_bytes),
        }
    }
}

// convert a field element (at most BigInteger256).
// pub fn field_to_u256<F: PrimeField>(f: F) -> U256 {
//
//     // if F::size_in_bits() > 256 {
//     //     panic!("Don't support field size larger than 256 bits.");
//     // }
//     U256::from_little_endian(&to_bytes!(&f).unwrap())
// }
//
// /// convert a U256 to a field element.
// pub fn u256_to_field<F: PrimeField>(v: U256) -> F {
//     let mut bytes = vec![0u8; 32];
//     v.to_little_endian(&mut bytes);
//     F::from_le_bytes_mod_order(&bytes)
// }
