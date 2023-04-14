#[cfg(test)]

mod test {

    use contract_bindings::hot_shot::NewBlocksCall;
    use ethers::{abi::AbiDecode, providers::Middleware, types::U256};
    use sha3::Digest;

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
        use ark_bn254::Fq;
        use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
        use ark_ff::{BigInt, Field, PrimeField};
        use jf_primitives::signatures::bls_over_bn254::hash_to_curve;
        use sha3::Keccak256;

        fn compare_field_elems(field_elem_rust: Fq, field_elem_contract: U256) {
            let x_rust_big_int = field_elem_rust.0;
            let x_contract_big_int = BigInt::new(field_elem_contract.0);

            assert_eq!(x_rust_big_int, x_contract_big_int);
        }

        #[async_std::test]
        async fn test_hash_to_curve() {
            // https://geometry.xyz/notebook/Optimized-BLS-multisignatures-on-EVM
            // https://github.com/thehubbleproject/hubble-contracts/blob/master/contracts/libs/BLS.sol
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
                hasher: Keccak256::new(),
                dst: dst.to_vec(),
                block_size: len_per_base_elem,
            };

            // Simplification in our case:
            // see https://github.com/arkworks-rs/algebra/blob/bc991d44c5e579025b7ed56df3d30267a7b9acac/ff/src/fields/field_hashers/mod.rs#L70
            let len_in_bytes = len_per_base_elem;
            let message: Vec<u8> = vec![1u8, 2u8, 5u8, 45u8];
            let rust_uniform_bytes = expander.expand(&message, len_in_bytes);
            let contract_uniform_bytes = hotshot.expand(message).call().await.unwrap();

            assert_eq!(rust_uniform_bytes, contract_uniform_bytes);
        }

        #[async_std::test]
        async fn test_field_elem_from_random_bytes() {
            let bytes = [1u8, 2u8, 56, 124, 3, 3, 4, 87];
            let f_elem = Fq::from_random_bytes(&bytes).unwrap();
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            let f_elem_contract = hotshot
                .field_from_random_bytes(bytes.to_vec())
                .call()
                .await
                .unwrap();

            compare_field_elems(f_elem, f_elem_contract);
        }

        #[async_std::test]
        async fn test_hash_to_field() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            let hasher_init = &[1u8]; // TODO make it clear that this is the dst vector
            let hasher = <DefaultFieldHasher<Keccak256> as HashToField<Fq>>::new(hasher_init);

            let message: Vec<u8> = vec![1u8, 2u8, 3u8, 45u8];

            let x_rust: Fq = hasher.hash_to_field(&message, 1)[0];
            let x_contract = hotshot.hash_to_field(message).call().await.unwrap();

            compare_field_elems(x_rust, x_contract);
        }

        #[async_std::test]
        async fn test_from_le_bytes_mod_order() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            // Small input, less than 31 bytes
            let input = vec![1u8; 31];
            let x_rust: Fq = Fq::from_le_bytes_mod_order(&input);
            let x_contract = hotshot.from_le_bytes_mod_order(input).call().await.unwrap();

            compare_field_elems(x_rust, x_contract);

            // Large input, more than 31 bytes
            let input = vec![1u8; 32];

            let x_rust: Fq = Fq::from_le_bytes_mod_order(&input);
            let x_contract = hotshot.from_le_bytes_mod_order(input).call().await.unwrap();

            compare_field_elems(x_rust, x_contract);
        }

        #[async_std::test]
        async fn test_field_from_byte() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            let val = 58u8;
            let f = Fq::from(val);
            let f_contract = hotshot.field_from_byte(val).call().await.unwrap();
            compare_field_elems(f, f_contract);
        }

        #[async_std::test]
        async fn test_mul() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;
            let a = [23u8; 31];
            let b = [41u8; 31];

            let a_field = Fq::from_random_bytes(&a).unwrap();
            let b_field = Fq::from_random_bytes(&b).unwrap();

            let res = a_field * b_field;

            let f_contract = hotshot.mul(a.into(), b.into()).call().await.unwrap();

            compare_field_elems(res, f_contract);
        }
    }
}
