#![cfg(any(test, feature = "testing"))]
pub(crate) mod hotshot_contract {
    use contract_bindings::hot_shot::HotShot;
    use contract_bindings::TestClients;
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::Wallet;
    use ethers::providers::{Http, Middleware, Provider};
    use sequencer_utils::Anvil;
    use std::time::Duration;
    pub(crate) async fn get_hotshot_contract_and_provider() -> (
        HotShot<
            SignerMiddleware<
                ethers::providers::Provider<Http>,
                Wallet<ethers::core::k256::ecdsa::SigningKey>,
            >,
        >,
        Provider<Http>,
    ) {
        let anvil = Anvil::spawn(None).await;
        let mut provider = Provider::try_from(&anvil.url().to_string()).unwrap();
        provider.set_interval(Duration::from_millis(10));

        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let deployer = clients.deployer.clone();

        let hotshot = HotShot::deploy(deployer.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        (hotshot, provider)
    }
}
