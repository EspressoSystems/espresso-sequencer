/// Utilities for loading an initial permissioned stake table from a toml file.
///
/// The initial stake table is passed to the permissioned stake table contract
/// on deployment.
use contract_bindings::permissioned_stake_table::NodeInfo;
use hotshot::types::BLSPubKey;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
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

#[cfg(test)]
mod test {
    use crate::stake_table::PermissionedStakeTableConfig;
    use crate::test_utils::setup_test;
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_types::{light_client::StateKeyPair, network::PeerConfigKeys};
    use toml::toml;
    #[test]
    fn test_permissioned_stake_table_from_toml() {
        setup_test();

        let mut keys = Vec::new();
        for i in 0..3 {
            let (pubkey, _) = BLSPubKey::generated_from_seed_indexed([0; 32], i);
            let state_kp = StateKeyPair::generate_from_seed_indexed([0; 32], i).0;
            let ver_key = state_kp.ver_key();
            keys.push(PeerConfigKeys {
                stake_table_key: pubkey,
                state_ver_key: ver_key,
                stake: 1,
                da: i == 0,
            });
        }

        let st_key_1 = keys[0].stake_table_key.to_string();
        let verkey_1 = keys[0].state_ver_key.to_string();
        let da_1 = keys[0].da;

        let st_key_2 = keys[1].stake_table_key.to_string();
        let verkey_2 = keys[1].state_ver_key.to_string();
        let da_2 = keys[1].da;

        let st_key_3 = keys[2].stake_table_key.to_string();
        let verkey_3 = keys[2].state_ver_key.to_string();
        let da_3 = keys[2].da;

        let toml = toml! {
            [[public_keys]]
            stake_table_key =  st_key_1
            state_ver_key  =  verkey_1
            stake = 1
            da = da_1

            [[public_keys]]
            stake_table_key =  st_key_2
            state_ver_key  =  verkey_2
            stake = 1
            da = da_2

            [[public_keys]]
            stake_table_key = st_key_3
            state_ver_key  =  verkey_3
            stake = 2
            da = da_3

        }
        .to_string();

        let toml_st: PermissionedStakeTableConfig = toml::from_str(&toml).unwrap();

        assert_eq!(toml_st.public_keys.len(), 3);

        // TODO: add `PartialEq` to PeerConfigKeys
        assert_eq!(toml_st.public_keys[0].state_ver_key, keys[0].state_ver_key);
        assert_eq!(
            toml_st.public_keys[0].stake_table_key,
            keys[0].stake_table_key
        );
        assert_eq!(toml_st.public_keys[0].da, da_1);
        assert_eq!(toml_st.public_keys[0].stake, 1);

        assert_eq!(toml_st.public_keys[1].state_ver_key, keys[1].state_ver_key);
        assert_eq!(
            toml_st.public_keys[1].stake_table_key,
            keys[1].stake_table_key
        );
        assert_eq!(toml_st.public_keys[1].da, da_2);
        assert_eq!(toml_st.public_keys[1].stake, 1);

        assert_eq!(toml_st.public_keys[2].state_ver_key, keys[2].state_ver_key);
        assert_eq!(
            toml_st.public_keys[2].stake_table_key,
            keys[2].stake_table_key
        );
        assert_eq!(toml_st.public_keys[2].da, da_3);
        assert_eq!(toml_st.public_keys[2].stake, 2);
    }
}
