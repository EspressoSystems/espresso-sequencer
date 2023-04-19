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
        use crate::helpers::{compare_field_elems, compare_group_elems, MyG1Point, MyG2Point};
        use ark_bn254::Fq;
        use ark_ec::CurveGroup;
        use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
        use ark_std::vec;
        use ethers::types::Bytes;
        use jf_primitives::signatures::bls_over_bn254::{
            hash_to_curve, BLSOverBN254CurveSignatureScheme,
        };
        use jf_primitives::signatures::SignatureScheme;
        use jf_utils::test_rng;
        use sha3::Keccak256;

        fn test_inputs() -> Vec<Bytes> {
            let message1 = Bytes::from(vec![1u8, 2u8, 3u8, 45u8, 88u8]);
            let mut message2 = vec![1u8, 2u8, 3u8, 45u8, 88u8];
            let csid = [
                66, 76, 83, 95, 83, 73, 71, 95, 66, 78, 50, 53, 52, 71, 49, 95, 88, 77, 68, 58, 75,
                69, 67, 67, 65, 75, 95, 78, 67, 84, 72, 95, 78, 85, 76, 95,
            ];
            message2.extend(csid);
            let message2 = Bytes::from(message2);

            let message3 = Bytes::from(vec![33u8; 1000]);

            let res = vec![message1, message2, message3];

            res
        }

        #[async_std::test]
        async fn test_full_sig_scheme() {
            let rng = &mut test_rng();
            let message = Bytes::from(vec![1u8, 2u8, 3u8, 45u8, 88u8]);

            // TODO why can't we write let parameters = (); ? cargo clippy complains
            let (sk, pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
            let sig = BLSOverBN254CurveSignatureScheme::sign(&(), &sk, &message, rng).unwrap();
            assert!(BLSOverBN254CurveSignatureScheme::verify(&(), &pk, &message, &sig).is_ok());

            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            let sig_value: MyG1Point = sig.clone().get_sig_value().into_affine().into();

            let pk_affine = pk.to_affine();
            let pk_value: MyG2Point = pk_affine.into();

            let is_sig_valid_contract: bool = hotshot
                .verify_bls_sig(
                    message.clone(),
                    sig_value.clone().into(),
                    pk_value.clone().into(),
                )
                .call()
                .await
                .unwrap();
            assert!(is_sig_valid_contract);

            let wrong_message = Bytes::from(vec![10u8; 3]);
            assert!(BLSOverBN254CurveSignatureScheme::verify(
                &(),
                &pk,
                wrong_message.clone(),
                &sig
            )
            .is_err());
            let is_sig_valid_contract = hotshot
                .verify_bls_sig(
                    wrong_message.clone(),
                    sig_value.clone().into(),
                    pk_value.clone().into(),
                )
                .call()
                .await
                .unwrap();
            assert!(!is_sig_valid_contract);

            let (_, wrong_pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
            assert!(
                BLSOverBN254CurveSignatureScheme::verify(&(), &wrong_pk, &message, &sig).is_err()
            );

            let wrong_pk_value: MyG2Point = wrong_pk.to_affine().into();
            let is_sig_valid_contract = hotshot
                .verify_bls_sig(message.clone(), sig_value.into(), wrong_pk_value.into())
                .call()
                .await
                .unwrap();
            assert!(!is_sig_valid_contract);
        }

        #[async_std::test]
        async fn test_hash_to_field() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            let hasher_init = &[1u8]; // TODO make it clear that this is the dst vector
            let hasher = <DefaultFieldHasher<Keccak256> as HashToField<Fq>>::new(hasher_init);

            let msgs = test_inputs();

            for msg in msgs.iter() {
                let x_rust: Fq = hasher.hash_to_field(msg, 1)[0];
                let x_contract = hotshot.hash_to_field(msg.clone()).call().await.unwrap();
                compare_field_elems(x_rust, x_contract);
            }
        }

        #[async_std::test]
        async fn test_hash_to_curve() {
            let (hotshot, _) = get_hotshot_contract_and_provider().await;

            let msgs = test_inputs();
            for msg in msgs.iter() {
                let group_elem = hash_to_curve::<Keccak256>(msg);
                let group_elem_contract = hotshot.hash_to_curve(msg.clone()).call().await.unwrap();
                compare_group_elems(group_elem.into(), group_elem_contract);
            }
        }
    }
}
