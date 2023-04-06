#[cfg(test)]

mod test {
    use contract_bindings::hot_shot::{HotShot, NewBlocksCall};
    use contract_bindings::TestClients;
    use ethers::types::Bytes;
    use ethers::utils::keccak256;
    use ethers::{
        abi::{AbiDecode, AbiEncode},
        providers::{Middleware, Provider},
        types::U256,
    };

    use sequencer_utils::Anvil;
    use std::time::Duration;

    #[async_std::test]
    async fn test_hotshot_block_commitment() {
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
            .new_blocks(vec![commitment], vec![vec![1, 2, 3].into()])
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(
            hotshot.commitments(block_num).call().await.unwrap(),
            commitment,
        );

        let (event, meta) = &hotshot
            .new_blocks_filter()
            .from_block(0)
            .query_with_meta()
            .await
            .unwrap()[0];

        assert_eq!(event.first_block_number, block_num);
        assert_eq!(event.num_blocks, 1.into());

        // Parse the commitments from calldata.
        let tx = provider
            .get_transaction(meta.transaction_hash)
            .await
            .unwrap()
            .unwrap();
        let call = NewBlocksCall::decode(&tx.input).unwrap();
        assert_eq!(call.new_commitments, vec![commitment]);
    }

    mod bls_signature {
        use super::*;
        use jf_primitives::signatures::bls_over_bn254::hash_to_curve;
        use sha3::Keccak256;

        #[async_std::test]
        async fn test_hash_to_curve() {
            let msg_input = [44u8, 65u8];
            let _group_elem = hash_to_curve::<Keccak256>(&msg_input);
        }

        #[async_std::test]
        async fn test_hash_to_curve_internals() {
            // TODO extract function for deploying the Hotshot contract
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

            // Compute Keccak(m) in both rust and solidity code and compare the result

            let m = vec![54u8, 56u8]; // TODO try with more values (in a loop)?
            let m_encoded = m.clone().encode();
            let res_rust = U256::from(keccak256(m_encoded.as_slice()));

            let m_bytes = Bytes::from(m_encoded);

            let res_contract: U256 = hotshot.keccak(m_bytes).call().await.unwrap().into();

            assert_eq!(res_rust, res_contract);
        }
    }
}
