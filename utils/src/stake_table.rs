use std::{fs, path::Path, sync::Arc, time::Duration};

/// Utilities for loading an initial permissioned stake table from a toml file.
///
/// The initial stake table is passed to the permissioned stake table contract
/// on deployment.
use contract_bindings_ethers::permissioned_stake_table::{
    G2Point, NodeInfo, PermissionedStakeTable, PermissionedStakeTableErrors,
};
use derive_more::derive::From;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware as _, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _},
    types::Address,
};
use hotshot::types::BLSPubKey;
use hotshot_contract_adapter::stake_table::{bls_jf_to_sol, NodeInfoJf};
use hotshot_types::network::PeerConfigKeys;
use url::Url;

use crate::contract_send;

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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, From, PartialEq)]
pub struct StakerIdentity {
    pub stake_table_key: BLSPubKey,
}

impl From<StakerIdentity> for BLSPubKey {
    fn from(value: StakerIdentity) -> Self {
        value.stake_table_key
    }
}

/// Information to add and remove stakers in the permissioned stake table contract.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(bound(deserialize = ""))]
pub struct PermissionedStakeTableUpdate {
    #[serde(default)]
    pub stakers_to_remove: Vec<StakerIdentity>,
    #[serde(default)]
    pub new_stakers: Vec<PeerConfigKeys<BLSPubKey>>,
}

impl PermissionedStakeTableUpdate {
    pub fn new(
        new_stakers: Vec<PeerConfigKeys<BLSPubKey>>,
        stakers_to_remove: Vec<StakerIdentity>,
    ) -> Self {
        Self {
            stakers_to_remove,
            new_stakers,
        }
    }

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

    pub fn to_toml_file(&self, path: &Path) -> anyhow::Result<()> {
        let toml_string = toml::to_string_pretty(self)
            .unwrap_or_else(|err| panic!("Failed to serialize config to TOML: {err}"));

        fs::write(path, toml_string)
            .unwrap_or_else(|_| panic!("Could not write config file to {}", path.display()));

        Ok(())
    }

    fn stakers_to_remove(&self) -> Vec<G2Point> {
        self.stakers_to_remove
            .iter()
            .map(|v| bls_jf_to_sol(v.clone().into()))
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

    if update.stakers_to_remove().is_empty() && update.new_stakers().is_empty() {
        anyhow::bail!("No changes to update in the stake table");
    }

    let (tx_receipt, _) = contract_send::<_, _, PermissionedStakeTableErrors>(
        &contract.update(update.stakers_to_remove(), update.new_stakers()),
    )
    .await?;

    tracing::info!("Transaction receipt: {:?}", tx_receipt);
    Ok(())
}

#[cfg(test)]
mod test {
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_types::{light_client::StateKeyPair, network::PeerConfigKeys};
    use toml::toml;

    use crate::{
        stake_table::{PermissionedStakeTableConfig, PermissionedStakeTableUpdate},
        test_utils::setup_test,
    };

    fn assert_peer_config_eq(p1: &PeerConfigKeys<BLSPubKey>, p2: &PeerConfigKeys<BLSPubKey>) {
        assert_eq!(p1.stake_table_key, p2.stake_table_key);
        assert_eq!(p1.state_ver_key, p2.state_ver_key);
        assert_eq!(p1.stake, p2.stake);
        assert_eq!(p1.da, p2.da);
    }

    fn mk_keys() -> Vec<PeerConfigKeys<BLSPubKey>> {
        let mut keys = Vec::new();
        for i in 0..3 {
            let (pubkey, _) = BLSPubKey::generated_from_seed_indexed([0; 32], i);
            let state_kp = StateKeyPair::generate_from_seed_indexed([0; 32], i).0;
            let ver_key = state_kp.ver_key();
            keys.push(PeerConfigKeys {
                stake_table_key: pubkey,
                state_ver_key: ver_key,
                stake: i + 1,
                da: i == 0,
            });
        }
        keys
    }

    #[test]
    fn test_permissioned_stake_table_from_toml() {
        setup_test();

        let keys = mk_keys();

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
            stake = 2
            da = da_2

            [[public_keys]]
            stake_table_key = st_key_3
            state_ver_key  =  verkey_3
            stake = 3
            da = da_3

        }
        .to_string();

        let tmpdir = tempfile::tempdir().unwrap();
        let toml_path = tmpdir.path().join("stake_table.toml");
        std::fs::write(&toml_path, toml).unwrap();

        let toml_st = PermissionedStakeTableConfig::from_toml_file(&toml_path).unwrap();

        assert_eq!(toml_st.public_keys.len(), 3);

        assert_peer_config_eq(&toml_st.public_keys[0], &keys[0]);
        assert_peer_config_eq(&toml_st.public_keys[1], &keys[1]);
        assert_peer_config_eq(&toml_st.public_keys[2], &keys[2]);
    }

    #[test]
    fn test_permissioned_stake_table_update_from_toml() {
        setup_test();

        let keys = mk_keys();

        let st_key_1 = keys[0].stake_table_key.to_string();

        let st_key_2 = keys[1].stake_table_key.to_string();
        let verkey_2 = keys[1].state_ver_key.to_string();
        let da_2 = keys[1].da;

        let st_key_3 = keys[2].stake_table_key.to_string();
        let verkey_3 = keys[2].state_ver_key.to_string();
        let da_3 = keys[2].da;

        let toml = toml! {
            [[stakers_to_remove]]
            stake_table_key =  st_key_1

            [[new_stakers]]
            stake_table_key =  st_key_2
            state_ver_key  =  verkey_2
            stake = 2
            da = da_2

            [[new_stakers]]
            stake_table_key = st_key_3
            state_ver_key  =  verkey_3
            stake = 3
            da = da_3
        }
        .to_string();

        let tmpdir = tempfile::tempdir().unwrap();
        let toml_path = tmpdir.path().join("stake_table_update.toml");
        std::fs::write(&toml_path, toml).unwrap();
        let update = PermissionedStakeTableUpdate::from_toml_file(&toml_path).unwrap();

        assert_eq!(update.stakers_to_remove.len(), 1);
        assert_eq!(update.stakers_to_remove[0], keys[0].stake_table_key.into());

        assert_eq!(update.new_stakers.len(), 2);
        assert_peer_config_eq(&update.new_stakers[0], &keys[1]);
        assert_peer_config_eq(&update.new_stakers[1], &keys[2]);
    }
}
