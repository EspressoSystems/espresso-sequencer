#[cfg(test)]

mod test {

    use contract_bindings::hot_shot::NewBlocksCall;
    use ethers::{abi::AbiDecode, providers::Middleware, types::U256};

    use crate::helpers::hotshot_contract::get_hotshot_contract_and_provider;

    #[async_std::test]
    async fn test_hotshot_block_commitment() {
        let (hotshot, provider) = get_hotshot_contract_and_provider().await;

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
        use ark_bn254::{Fq, G1Affine};
        use ark_ff::BigInt;
        use ark_ff::{BigInteger, PrimeField};
        use jf_primitives::signatures::bls_over_bn254::hash_to_curve;
        use sha3::Keccak256;

        fn compare_field_elems(field_elem_rust: Fq, field_elem_contract: U256) {
            let x_rust_big_int = field_elem_rust.into_bigint();
            let x_contract_big_int = BigInt::new(field_elem_contract.0);
            assert_eq!(x_rust_big_int, x_contract_big_int);
        }

        fn compare_group_elems(group_elem_rust: G1Affine, group_elem_contract: (U256, U256)) {
            compare_field_elems(group_elem_rust.x, group_elem_contract.0);
            compare_field_elems(group_elem_rust.y, group_elem_contract.1);
        }

        #[async_std::test]
        async fn test_hash_to_curve() {
            // https://geometry.xyz/notebook/Optimized-BLS-multisignatures-on-EVM
            // https://github.com/thehubbleproject/hubble-contracts/blob/master/contracts/libs/BLS.sol
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            let msg_input = vec![1u8, 2u8, 3u8, 45u8, 88u8];
            let group_elem = hash_to_curve::<Keccak256>(&msg_input);
            let group_elem_contract = hotshot.hash_to_curve(msg_input).call().await.unwrap();

            compare_group_elems(group_elem.into(), group_elem_contract);
        }

        #[async_std::test]
        async fn test_field_op() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            let bi = BigInt::new([
                11200547023649581266,
                3785745850373481645,
                5086809825638434109,
                2308022455161686953,
            ]);
            let x = Fq::from(bi);
            let x_bytes: Vec<u8> = bi.to_bytes_be();
            let x_u256 = U256::from(x_bytes.as_slice());
            let f_contract = hotshot.field_square(x_u256).call().await.unwrap();
            let expected_f_elem = x * x;
            compare_field_elems(expected_f_elem, f_contract);
        }
    }
}
