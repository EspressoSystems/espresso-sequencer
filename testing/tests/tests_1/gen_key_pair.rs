// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#![allow(clippy::panic)]

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{env, fs::File, io::prelude::*};

    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_orchestrator::config::ValidatorConfigFile;
    use hotshot_types::ValidatorConfig;
    #[test]
    fn gen_key_pair_gen_from_config_file() {
        let config_file = ValidatorConfigFile::from_file("config/ValidatorConfigFile.toml");
        let my_own_validator_config = ValidatorConfig::<BLSPubKey>::from(config_file.clone());
        if config_file.seed == [0u8; 32] && config_file.node_id == 0 {
            assert_eq!(
                my_own_validator_config.public_key,
                <BLSPubKey as SignatureKey>::from_private(&my_own_validator_config.private_key)
            );
        }

        let current_working_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                panic!("get_current_working_dir error: {:?}", e);
            }
        };
        let filename = current_working_dir.into_os_string().into_string().unwrap()
            + "/../../config/ValidatorConfigOutput";
        match File::create(filename) {
            Err(why) => panic!("couldn't create file for output key pairs: {}", why),
            Ok(mut file) => match write!(file, "{my_own_validator_config:?}",) {
                Err(why) => panic!("couldn't generate key pairs and write to the file: {}", why),
                Ok(()) => println!("successfully wrote to file for output key pairs"),
            },
        }
    }
}
