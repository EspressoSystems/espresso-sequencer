// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{env, fs, path::PathBuf};

use crate::{traits::signature_key::SignatureKey, ValidatorConfig};
use toml;
use tracing::error;

/// Holds configuration for a validator node
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(bound(deserialize = ""))]
pub struct ValidatorConfigFile {
    /// The validator's seed
    pub seed: [u8; 32],
    /// The validator's index, which can be treated as another input to the seed
    pub node_id: u64,
    // The validator's stake, commented for now
    // pub stake_value: u64,
    /// Whether or not we are DA
    pub is_da: bool,
}

impl ValidatorConfigFile {
    /// read the validator config from a file
    /// # Panics
    /// Panics if unable to get the current working directory
    pub fn from_file(dir_str: &str) -> Self {
        let current_working_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                error!("get_current_working_dir error: {:?}", e);
                PathBuf::from("")
            }
        };
        let filename =
            current_working_dir.into_os_string().into_string().unwrap() + "/../../" + dir_str;
        let contents = fs::read_to_string(filename.clone()).expect("Could not read file");
        let data: ValidatorConfigFile =
            toml::from_str(&contents).expect("Unable to load data from file");
        data
    }
}

impl<K: SignatureKey> From<ValidatorConfigFile> for ValidatorConfig<K> {
    fn from(val: ValidatorConfigFile) -> Self {
        // here stake_value is set to 1, since we don't input stake_value from ValidatorConfigFile for now
        ValidatorConfig::generated_from_seed_indexed(val.seed, val.node_id, 1, val.is_da)
    }
}
