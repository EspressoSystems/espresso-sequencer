use crate::{
    block::{test::ValidTest, NsProof},
    Payload,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use hotshot::traits::BlockPayload;
use hotshot_types::{traits::EncodeBytes, vid::vid_scheme};
use jf_vid::VidScheme;

#[async_std::test]
async fn ns_proof() {
    let test_case = vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8]];

    setup_logging();
    setup_backtrace();
    let mut rng = jf_utils::test_rng();
    let mut test = ValidTest::from_tx_lengths(test_case, &mut rng);
    let block =
        Payload::from_transactions(test.all_txs(), &Default::default(), &Default::default())
            .await
            .unwrap()
            .0;
    let (vid_commit, vid_common) = {
        let mut vid = vid_scheme(10);
        let disperse_data = vid.disperse(block.encode()).unwrap();
        (disperse_data.commit, disperse_data.common)
    };

    for ns_index in block.ns_table().iter() {
        let ns_id = block.ns_table().read_ns_id(&ns_index).unwrap();
        let txs = test
            .nss
            .remove(&ns_id)
            .expect("block ns_id missing from test");

        let ns_proof =
            NsProof::new(&block, &ns_index, &vid_common).expect("NsProof::new should succeed");

        let (ns_proof_txs, ns_proof_ns_id) = ns_proof
            .verify(block.ns_table(), &vid_commit, &vid_common)
            .unwrap_or_else(|| panic!("namespace {} proof verification failure", ns_id));

        assert_eq!(ns_proof_ns_id, ns_id);
        assert_eq!(ns_proof_txs, txs);
    }
}
