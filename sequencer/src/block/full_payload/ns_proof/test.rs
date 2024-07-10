use crate::{
    block::{full_payload::NsIndex, test::ValidTest, NsProof},
    Payload,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use futures::future;
use hotshot::traits::BlockPayload;
use hotshot_types::{traits::EncodeBytes, vid::vid_scheme};
use jf_vid::VidScheme;

#[async_std::test]
async fn ns_proof() {
    let test_cases = vec![
        vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8], vec![7, 8, 9]],
        vec![vec![1, 2, 3], vec![4, 5, 6]],
    ];

    setup_logging();
    setup_backtrace();

    let mut rng = jf_utils::test_rng();
    let mut tests = ValidTest::many_from_tx_lengths(test_cases, &mut rng);

    // each item is a tuple: (block, vid_info, list_of_ns_proofs)
    let blocks: Vec<(Payload, _, Vec<(NsIndex, NsProof)>)> = {
        // compute blocks separately to avoid async error `captured variable
        // cannot escape `FnMut` closure body` caused by mutable variable `vid`
        // below.
        let blocks_only = future::join_all(tests.iter().map(|t| async {
            Payload::from_transactions(t.all_txs(), &Default::default(), &Default::default())
                .await
                .unwrap()
                .0
        }))
        .await;

        let mut vid = vid_scheme(10);
        blocks_only
            .into_iter()
            .map(|block| {
                let vid = vid.disperse(block.encode()).unwrap();
                let ns_proofs: Vec<(NsIndex, NsProof)> = block
                    .ns_table()
                    .iter()
                    .map(|ns_index| {
                        (
                            ns_index.clone(),
                            NsProof::new(&block, &ns_index, &vid.common).unwrap(),
                        )
                    })
                    .collect();
                (block, vid, ns_proofs)
            })
            .collect()
    };

    // sanity: verify all valid namespace proofs
    for ((block, vid, ns_proofs), test) in blocks.iter().zip(tests.iter_mut()) {
        for (ns_index, ns_proof) in ns_proofs.iter() {
            let ns_id = block.ns_table().read_ns_id(ns_index).unwrap();
            let txs = test
                .nss
                .remove(&ns_id)
                .unwrap_or_else(|| panic!("namespace {} missing from test", ns_id));

            // verify ns_proof
            let (ns_proof_txs, ns_proof_ns_id) = ns_proof
                .verify(block.ns_table(), &vid.commit, &vid.common)
                .unwrap_or_else(|| panic!("namespace {} proof verification failure", ns_id));

            assert_eq!(ns_proof_ns_id, ns_id);
            assert_eq!(ns_proof_txs, txs);
        }
    }

    // mix and match ns_table, vid_commit, vid_common
    {
        assert!(blocks.len() >= 2, "need at least 2 test_cases");

        let ns_proof_0_0 = &blocks[0].2[0].1;
        let ns_table_0 = blocks[0].0.ns_table();
        let ns_table_1 = blocks[1].0.ns_table();
        let vid_commit_0 = &blocks[0].1.commit;
        let vid_commit_1 = &blocks[1].1.commit;
        let vid_common_0 = &blocks[0].1.common;
        let vid_common_1 = &blocks[1].1.common;

        // wrong ns_table
        assert!(ns_proof_0_0
            .verify(ns_table_1, vid_commit_0, vid_common_0)
            .is_none());

        // wrong vid commitment
        assert!(ns_proof_0_0
            .verify(ns_table_0, vid_commit_1, vid_common_0)
            .is_none());

        // wrong vid common
        assert!(ns_proof_0_0
            .verify(ns_table_0, vid_commit_0, vid_common_1)
            .is_none());
    }

    // ensure well-formed test case
    // assert!(blocks.len() >= 2, "need at least 2 test_cases");
    // assert!(
    //     blocks[0].ns_table().len().in_bounds(&ns_index),
    //     "block[0] has too few namespaces"
    // );
    // assert!(
    //     blocks[0].ns_table().len().as_usize() > blocks[1].ns_table().len().as_usize(),
    //     "block[0] should have fewer namespaces than block[1]"
    // );
}
