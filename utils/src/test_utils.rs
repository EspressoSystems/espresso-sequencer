use std::sync::Arc;

use anyhow::Result;
use contract_bindings::hot_shot::HotShot;
use ethers::{
    prelude::{Address, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _},
};

use crate::Signer;

#[derive(Debug, Clone)]
pub struct TestClient {
    pub index: u32,
    pub provider: Arc<Signer>,
}

impl TestClient {
    pub fn new(provider: &Provider<Http>, index: u32, chain_id: u64) -> Self {
        let test_mnemonic = "test test test test test test test test test test test junk";
        let provider = Arc::new(SignerMiddleware::new(
            provider.clone(),
            MnemonicBuilder::<English>::default()
                .phrase(test_mnemonic)
                .index(index)
                .unwrap()
                .build()
                .unwrap()
                .with_chain_id(chain_id),
        ));
        Self { index, provider }
    }
}

// We may want to use different names once we deploy a customized system without
// trusted parties.
#[derive(Debug, Clone)]
pub struct TestClients {
    // Account to use when deploying contracts.
    pub deployer: TestClient,
    // The block_driver client shouldn't be used for anything else to avoid nonce issues.
    pub block_driver: TestClient,
    // Various funded accounts that tests can use.
    pub funded: Vec<TestClient>,
}
impl TestClients {
    pub fn new(provider: &Provider<Http>, chain_id: u64) -> Self {
        Self {
            deployer: TestClient::new(provider, 11, chain_id),
            funded: vec![
                TestClient::new(provider, 12, chain_id),
                TestClient::new(provider, 13, chain_id),
                TestClient::new(provider, 14, chain_id),
            ],
            block_driver: TestClient::new(provider, 15, chain_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestL1System {
    pub clients: TestClients,
    pub hotshot: HotShot<Signer>,
    pub provider: Provider<Http>,
}

impl TestL1System {
    pub async fn new(provider: Provider<Http>, hotshot_address: Address) -> Result<Self> {
        let chain_id = provider.get_chainid().await?.as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let hotshot = HotShot::new(hotshot_address, clients.deployer.provider.clone());
        Ok(Self {
            clients,
            hotshot,
            provider,
        })
    }

    pub async fn deploy(provider: Provider<Http>) -> Result<Self> {
        let chain_id = provider.get_chainid().await?.as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let hotshot = HotShot::deploy(clients.deployer.provider.clone(), ())?
            .send()
            .await?;
        Ok(Self {
            clients,
            hotshot,
            provider,
        })
    }
}

pub fn setup_test() {
    super::logging::Config::from_env().init();
}
