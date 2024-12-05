/// Utilities for loading an initial permissioned stake table from a toml file.
///
/// The initial stake table is passed to the permissioned stake table contract
/// on deployment.
use contract_bindings::permissioned_stake_table::NodeInfo;
use hotshot::types::{BLSPubKey, SignatureKey};
use hotshot_contract_adapter::stake_table::{NodeInfoJf, ParsedEdOnBN254Point};
use hotshot_types::network::PeerConfigKeys;
use std::{fs, path::Path};

/// A stake table config stored in a file
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(bound(deserialize = ""))]
pub struct PermissionedStakeTableConfig {
    /// The list of public keys that are initially inserted into the
    /// permissioned stake table contract.
    #[serde(default)]
    pub public_keys: Vec<PeerConfigKeys<BLSPubKey>>,
}

impl PermissionedStakeTableConfig {
    pub fn from_toml_file(path: &Path) -> anyhow::Result<Self> {
        let config_file_as_string: String = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not read config file located at {}", path.display()));

        Ok(
            toml::from_str::<Self>(&config_file_as_string).unwrap_or_else(|err| {
                panic!(
                    "Unable to convert config file {} to TOML: {err}",
                    path.display()
                )
            }),
        )
    }
}

impl From<PermissionedStakeTableConfig> for Vec<NodeInfo> {
    fn from(value: PermissionedStakeTableConfig) -> Self {
        value
            .public_keys
            .into_iter()
            .map(|peer_config| {
                let node_info: NodeInfoJf = peer_config.clone().into();
                node_info.into()
            })
            .collect()
    }
}
