#[cfg(test)]

mod test {
    use contract_bindings::hot_shot::NewBlocksCall;
    use ethers::types::Bytes;
    use ethers::utils::keccak256;
    use ethers::{
        abi::{AbiDecode, AbiEncode},
        providers::Middleware,
        types::U256,
    };

    use crate::helpers::get_hotshot_contract_and_provider;

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
        use crate::hash_to_curve_helpers::Expander;
        // use ark_bn254::Fq as BaseField;
        // use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
        use jf_primitives::signatures::bls_over_bn254::hash_to_curve;
        use sha3::Keccak256;

        #[async_std::test]
        async fn test_hash_to_curve() {
            let msg_input = [44u8, 65u8];
            let _group_elem = hash_to_curve::<Keccak256>(&msg_input);
            // TODO
        }

        #[async_std::test]
        async fn test_expander() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            // We can fix the constants in our case
            let len_per_base_elem = 48;
            let dst = [1u8];

            let expander = crate::hash_to_curve_helpers::ExpanderXmd {
                hasher: Keccak256::default(),
                dst: dst.to_vec(),
                block_size: len_per_base_elem,
            };

            // Simplification in our case: see https://github.com/arkworks-rs/algebra/blob/bc991d44c5e579025b7ed56df3d30267a7b9acac/ff/src/fields/field_hashers/mod.rs#L70
            let len_in_bytes = len_per_base_elem;
            let message: Vec<u8> = vec![1u8, 2u8];
            let m_encoded = message.clone().encode();
            let message_bytes: Bytes = Bytes::from(m_encoded.clone());
            let _rust_uniform_bytes = expander.expand(&m_encoded, len_in_bytes);
            let _contract_uniform_bytes: Bytes =
                hotshot.expand(message_bytes).call().await.unwrap();
            // assert_eq!(rust_uniform_bytes, contract_uniform_bytes.to_vec());
        }

        #[async_std::test]
        async fn test_hash_to_field() {
            // https://geometry.xyz/notebook/Optimized-BLS-multisignatures-on-EVM
            // https://github.com/thehubbleproject/hubble-contracts/blob/master/contracts/libs/BLS.sol
        }

        #[async_std::test]
        async fn test_hash_to_curve_internals() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            // Compute Keccak(m) in both rust and solidity code and compare the result

            let m = vec![3u8, 156u8]; // TODO try with more values (in a loop)?
            let m_encoded = m.clone().encode();
            let res_rust = U256::from(keccak256(m_encoded.as_slice()));

            let m_bytes = Bytes::from(m_encoded);
            let res_contract: U256 = hotshot.keccak(m_bytes).call().await.unwrap().into();

            assert_eq!(res_rust, res_contract);
        }
    }
}
