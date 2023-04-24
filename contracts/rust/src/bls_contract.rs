#[cfg(test)]

mod test {

    use crate::helpers::hotshot_contract::get_bls_test_contract;
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
        let csid = b"BLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_";
        message2.extend(csid);
        let message2 = Bytes::from(message2);

        let message3 = Bytes::from(vec![33u8; 1000]);

        vec![message1, message2, message3]
    }

    #[async_std::test]
    async fn test_full_sig_scheme() {
        let rng = &mut test_rng();
        let message = Bytes::from(vec![1u8, 2u8, 3u8, 45u8, 88u8]);

        // TODO why can't we write let parameters = (); ? cargo clippy complains
        let (sk, pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
        let sig = BLSOverBN254CurveSignatureScheme::sign(&(), &sk, &message, rng).unwrap();
        assert!(BLSOverBN254CurveSignatureScheme::verify(&(), &pk, &message, &sig).is_ok());

        let bls = get_bls_test_contract().await;

        let sig_value: MyG1Point = sig.clone().sigma.into_affine().into();

        let pk_value: MyG2Point = pk.to_affine().into();

        let res = bls
            .verify_bls_sig(
                message.clone(),
                sig_value.clone().into(),
                pk_value.clone().into(),
            )
            .call()
            .await;

        assert!(res.is_ok());

        let wrong_message = Bytes::from(vec![10u8; 3]);
        assert!(
            BLSOverBN254CurveSignatureScheme::verify(&(), &pk, wrong_message.clone(), &sig)
                .is_err()
        );
        let res = bls
            .verify_bls_sig(
                wrong_message.clone(),
                sig_value.clone().into(),
                pk_value.clone().into(),
            )
            .call()
            .await;

        assert!(res.is_err());

        let (_, wrong_pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
        assert!(BLSOverBN254CurveSignatureScheme::verify(&(), &wrong_pk, &message, &sig).is_err());

        let wrong_pk_value: MyG2Point = wrong_pk.to_affine().into();
        let res = bls
            .verify_bls_sig(message.clone(), sig_value.into(), wrong_pk_value.into())
            .call()
            .await;
        assert!(res.is_err());
    }

    #[async_std::test]
    async fn test_hash_to_field() {
        let bls = get_bls_test_contract().await;

        // Same as in the hash_to_curve function
        // See https://github.com/EspressoSystems/jellyfish/blob/6c2c08f4e966fd1d454d48bcf30bd41a952f9f76/primitives/src/signatures/bls_over_bn254.rs#L310
        let hasher_init = &[1u8];
        let hasher = <DefaultFieldHasher<Keccak256> as HashToField<Fq>>::new(hasher_init);

        let msgs = test_inputs();

        for msg in msgs.iter() {
            let x_rust: Fq = hasher.hash_to_field(msg, 1)[0];
            let x_contract = bls.hash_to_field(msg.clone()).call().await.unwrap();
            compare_field_elems(x_rust, x_contract);
        }
    }

    #[async_std::test]
    async fn test_hash_to_curve() {
        let bls = get_bls_test_contract().await;

        let msgs = test_inputs();
        for msg in msgs.iter() {
            let group_elem = hash_to_curve::<Keccak256>(msg);
            let group_elem_contract = bls.hash_to_curve(msg.clone()).call().await.unwrap();
            compare_group_elems(group_elem.into(), group_elem_contract);
        }
    }
}
