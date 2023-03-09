#[cfg(test)]

mod test {
    use contract_bindings::hot_shot::HotShot;
    use contract_bindings::TestClients;
    use ethers::{
        providers::{Middleware, Provider},
        types::U256,
    };
    use sequencer_utils::Anvil;
    use std::time::Duration;

    #[async_std::test]
    async fn test_hotshot_contract() {
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

        let block_num = U256::from(0);
        let commitment = U256::from(1234);

        hotshot
            .new_block(block_num, commitment, vec![1, 2, 3].into())
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(
            hotshot.commitments(block_num).call().await.unwrap(),
            commitment,
        );

        let event = &hotshot
            .new_block_filter()
            .from_block(0)
            .query()
            .await
            .unwrap()[0];

        assert_eq!(event.block_number, block_num);
        assert_eq!(event.commitment, commitment);
    }
}
