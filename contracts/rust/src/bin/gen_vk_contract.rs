//! This executable generates the solidity files with hardcoded verifying keys for
//! LightClient updates by running `cargo run --bin gen-vk-contract`.
//! Adapted from [CAPE project][https://github.com/EspressoSystems/cape/blob/main/contracts/rust/src/bin/gen-vk-libraries.rs]

use jf_primitives::pcs::prelude::UnivariateUniversalParams;

fn main() {
    let srs = {
        // load SRS from Aztec's ceremony
        let srs =
            crs::aztec20::kzg10_setup(2u64.pow(20) as usize + 2).expect("Aztec SRS fail to load");
        // convert to Jellyfish type
        // TODO: (alex) use constructor instead https://github.com/EspressoSystems/jellyfish/issues/440
        UnivariateUniversalParams {
            powers_of_g: srs.powers_of_g,
            h: srs.h,
            beta_h: srs.beta_h,
        }
    };
    let (_, _vk) = hotshot_state_prover::preprocess(&srs).expect("Circuit preprocess failed");
}
