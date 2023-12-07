#[cfg(test)]

mod test {
    use crate::helpers::MyG2Point;

    use contract_bindings::hot_shot::{NewBlocksCall, Qc};
    use contract_bindings::shared_types::G2Point;

    use ethers::{abi::AbiDecode, providers::Middleware, types::U256};

    use jf_primitives::signatures::bls_over_bn254::BLSOverBN254CurveSignatureScheme;
    use jf_primitives::signatures::SignatureScheme;
    use jf_utils::test_rng;
    use sequencer_utils::test_utils::TestL1System;
    use sequencer_utils::AnvilOptions;

    #[async_std::test]
    async fn test_hotshot_block_commitment() {
        let anvil = AnvilOptions::default().spawn().await;
        let provider = anvil.provider();
        let TestL1System { hotshot, .. } = TestL1System::deploy(provider.clone()).await.unwrap();

        let block_num = U256::from(0);
        let commitment = U256::from(1234);
        let qcs = vec![Qc {
            height: block_num,
            block_commitment: commitment,
            ..Default::default()
        }];

        hotshot
            .new_blocks(qcs.clone())
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
            // Ethers does not set the contract address on filters created via contract bindings.
            // This seems like a bug and I have reported it:
            // https://github.com/gakonst/ethers-rs/issues/2528. In the mean time we can work around
            // by setting the address manually.
            .address(hotshot.address().into())
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
        assert_eq!(call.qcs, qcs);
    }

    #[async_std::test]
    async fn test_hotshot_stake_table() {
        let anvil = AnvilOptions::default().spawn().await;
        let provider = anvil.provider();
        let TestL1System { hotshot, .. } = TestL1System::deploy(provider.clone()).await.unwrap();

        let rng = &mut test_rng();
        for i in 0..5 {
            let (_, pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
            let pk = pk.to_affine();
            let pk_value: MyG2Point = pk.into();
            let amount = U256::from(10 * i);
            hotshot
                .add_new_staking_key(pk_value.clone().into(), amount)
                .send()
                .await
                .unwrap()
                .await
                .unwrap();

            let expected_stake_and_amount = (G2Point::from(pk_value), amount);
            assert_eq!(
                hotshot.get_staking_key(U256::from(i)).call().await.unwrap(),
                expected_stake_and_amount,
            );
        }
    }
}
