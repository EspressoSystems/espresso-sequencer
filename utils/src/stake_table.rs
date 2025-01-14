/// Utilities for loading an initial permissioned stake table from a toml file.
///
/// The initial stake table is passed to the permissioned stake table contract
/// on deployment.
use contract_bindings_ethers::permissioned_stake_table::{NodeInfo, PermissionedStakeTable};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware as _, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _},
    types::Address,
};
use hotshot::types::BLSPubKey;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::network::PeerConfigKeys;
use url::Url;

use std::{fs, path::Path, sync::Arc, time::Duration};

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

/// Information to add and remove stakers in the permissioned stake table contract.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(bound(deserialize = ""))]
pub struct PermissionedStakeTableUpdate {
    #[serde(default)]
    stakers_to_remove: Vec<PeerConfigKeys<BLSPubKey>>,
    #[serde(default)]
    new_stakers: Vec<PeerConfigKeys<BLSPubKey>>,
}

impl PermissionedStakeTableUpdate {
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

    fn stakers_to_remove(&self) -> Vec<NodeInfo> {
        self.stakers_to_remove
            .iter()
            .map(|peer_config| {
                let node_info: NodeInfoJf = peer_config.clone().into();
                node_info.into()
            })
            .collect()
    }

    fn new_stakers(&self) -> Vec<NodeInfo> {
        self.new_stakers
            .iter()
            .map(|peer_config| {
                let node_info: NodeInfoJf = peer_config.clone().into();
                node_info.into()
            })
            .collect()
    }
}

pub async fn update_stake_table(
    l1url: Url,
    l1_interval: Duration,
    mnemonic: String,
    account_index: u32,
    contract_address: Address,
    update: PermissionedStakeTableUpdate,
) -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from(l1url.to_string())?.interval(l1_interval);
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .index(account_index)?
        .build()?
        .with_chain_id(chain_id);
    let l1 = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    let contract = PermissionedStakeTable::new(contract_address, l1);

    tracing::info!("sending stake table update transaction");

    let tx_receipt = contract
        .update(update.stakers_to_remove(), update.new_stakers())
        .send()
        .await?
        .await?;
    tracing::info!("Transaction receipt: {:?}", tx_receipt);
    Ok(())
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
