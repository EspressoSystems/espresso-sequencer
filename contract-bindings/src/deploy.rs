use crate::HotShot;
use anyhow::Result;
use ethers::{
    abi::Tokenize,
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
};
use std::sync::Arc;

pub type EthMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;

#[async_trait::async_trait]
pub trait Deploy<M: Middleware> {
    async fn deploy_contract<T: Tokenize + Send>(client: &Arc<M>, args: T) -> Self;
}

#[derive(Debug, Clone)]
pub struct TestClient {
    pub index: u32,
    pub provider: Arc<EthMiddleware>,
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
            deployer: get_test_client(0, provider, chain_id),
            funded: vec![
                get_test_client(1, provider, chain_id),
                get_test_client(2, provider, chain_id),
                get_test_client(3, provider, chain_id),
            ],
            block_driver: get_test_client(4, provider, chain_id),
        }
    }
}

pub fn create_signer(
    index: u32,
    provider: &Provider<Http>,
    chain_id: u64,
    mnemonic_str: &str,
) -> Arc<EthMiddleware> {
    let mnemonic = MnemonicBuilder::<English>::default().phrase(mnemonic_str);
    Arc::new(SignerMiddleware::new(
        provider.clone(),
        mnemonic
            .index(index)
            .unwrap()
            .build()
            .unwrap()
            .with_chain_id(chain_id),
    ))
}

pub fn get_test_client(index: u32, provider: &Provider<Http>, chain_id: u64) -> TestClient {
    let test_mnemonic = "test test test test test test test test test test test junk";
    let provider = create_signer(index, provider, chain_id, test_mnemonic);
    TestClient { provider, index }
}

#[derive(Debug, Clone)]
pub struct TestL1System {
    pub clients: TestClients,
    pub hotshot: HotShot<EthMiddleware>,
    pub provider: Provider<Http>,
}

impl TestL1System {
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
