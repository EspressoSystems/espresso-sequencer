use hotshot::types::SignatureKey;
use hotshot_types::network::PeerConfigKeys;
use std::{fs, path::Path};

/// A stake table config stored in a file
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(bound(deserialize = ""))]
pub struct PermissionedStakeTableConfig<K: SignatureKey> {
    /// The list of public keys that are initially inserted into the
    /// permissioned stake table contract.
    #[serde(default)]
    pub public_keys: Vec<PeerConfigKeys<K>>,
}

impl<K: SignatureKey> PermissionedStakeTableConfig<K> {
    pub fn from_toml_file(path: &Path) -> anyhow::Result<Self> {
        let config_file_as_string: String = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not read config file located at {}", path.display()));

        Ok(
            toml::from_str::<Self>(&config_file_as_string).expect(&format!(
                "Unable to convert config file {} to TOML",
                path.display()
            )),
        )
    }
}
