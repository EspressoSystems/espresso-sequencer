#[cfg(test)]

mod test {
    use contract_bindings::bindings::hot_shot::HotShot;
    use contract_bindings::TestClients;

    use ethers::providers::{Middleware, Provider};
    use hermez_adaptor::{wait_for_rpc, ZkEvmEnv};
    use std::time::Duration;

    #[tokio::test]
    async fn test_hotshot_contract() {
        let env = ZkEvmEnv::default();
        let l1_provider = env.l1_provider();

        wait_for_rpc(&l1_provider, Duration::from_millis(200), 100)
            .await
            .unwrap();

        let mut provider = Provider::try_from(&l1_provider.to_string()).unwrap();
        provider.set_interval(Duration::from_millis(10));

        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let clients = TestClients::new(&provider, chain_id);
        let deployer = clients.deployer.clone();

        let _hotshot = HotShot::deploy(deployer.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        // TODO call HotshotContract.newBlock

        // TODO check the block has been inserted

        // TODO check event has been emitted
    }
}
