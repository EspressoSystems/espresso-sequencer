use contract_bindings::permissioned_stake_table::NodeInfo;
use hotshot::types::SignatureKey;
use hotshot_contract_adapter::stake_table::ParsedEdOnBN254Point;
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

impl<K: SignatureKey> From<PermissionedStakeTableConfig<K>> for Vec<NodeInfo> {
    fn from(value: PermissionedStakeTableConfig<K>) -> Self {
        value
            .public_keys
            .into_iter()
            .map(|peer_config| {
                let g1: ParsedEdOnBN254Point = peer_config.state_ver_key.to_affine().into();
                // XXX We don't have a trait on the Bls key to provide .to_affine()
                // let g2: ParsedG2Point = peer_config.stake_table_key.to_affine().into();
                NodeInfo {
                    bls_vk: todo!(),
                    schnorr_vk: g1.into(),
                    is_da: peer_config.da,
                }
            })
            .collect()
    }
}
