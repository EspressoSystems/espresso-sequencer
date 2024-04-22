use super::Payload;
use crate::{NamespaceId, Transaction};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use hotshot::traits::BlockPayload;
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::vid::vid_scheme;
use jf_primitives::vid::VidScheme;
use rand::RngCore;
use std::collections::HashMap;

#[test]
fn basic_correctness() {
    // play with this
    let test_cases = vec![
        vec![vec![5, 8, 8], vec![7, 9, 11], vec![10, 5, 8]], // 3 non-empty namespaces
    ];

    setup_logging();
    setup_backtrace();
    let mut rng = jf_utils::test_rng();
    let valid_tests = ValidTest::many_from_tx_lengths(test_cases, &mut rng);

    let mut vid = vid_scheme(10);

    for mut test in valid_tests {
        let mut all_txs = test.all_txs();
        tracing::info!("test case {} nss {} txs", test.nss.len(), all_txs.len());

        let block = Payload::from_transactions(test.all_txs()).unwrap().0;

        // test correct number of nss, txs
        assert_eq!(block.num_namespaces(), test.nss.len());
        assert_eq!(block.ns_iter().count(), test.nss.len());
        assert_eq!(block.len(&block.ns_table), all_txs.len());
        assert_eq!(block.iter(&block.ns_table).count(), all_txs.len());

        let (vid_commit, vid_common) = {
            let disperse_data = vid.disperse(&block.payload).unwrap();
            (disperse_data.commit, disperse_data.common)
        };

        // test iterate over all txs
        for tx_index in block.iter(&block.ns_table) {
            let tx = block.transaction(&tx_index).unwrap();

            // warning: linear search for a tx
            let test_tx = all_txs.remove(all_txs.iter().position(|t| t == &tx).unwrap());
            assert_eq!(tx, test_tx);

            let tx_proof = {
                let (tx2, tx_proof) = block
                    .transaction_with_proof(&tx_index, &vid_common)
                    .unwrap();
                assert_eq!(tx, tx2);
                tx_proof
            };
            assert!(tx_proof.verify(&tx, &vid_commit, &vid_common).unwrap());
        }
        assert!(
            all_txs.is_empty(),
            "not all test txs consumed by block.iter"
        );

        // test iterate over all namespaces
        assert_eq!(block.num_namespaces(), test.nss.len());
        for ns_id in block.ns_iter().map(|i| i.ns_id) {
            // tracing::info!("test ns_id {}", ns.ns_id);

            let txs = test
                .nss
                .remove(&ns_id)
                .expect("block ns_id missing from test");

            let ns_proof = block
                .namespace_with_proof(ns_id, &vid_common)
                .expect("namespace_with_proof should succeed");

            assert!(ns_proof.is_existence());

            let (ns_proof_txs, ns_proof_ns_id) = block
                .verify_namespace_proof(&ns_proof, &vid_commit, &vid_common)
                .unwrap_or_else(|| panic!("namespace {} proof verification failure", ns_id));

            assert_eq!(ns_proof_ns_id, ns_id);
            assert_eq!(ns_proof_txs, txs);
        }
        assert!(
            test.nss.is_empty(),
            "not all test namespaces consumed by ns_iter"
        );
    }
}

// TODO lots of infra here that could be reused in other tests.
struct ValidTest {
    nss: HashMap<NamespaceId, Vec<Transaction>>,
}

impl ValidTest {
    fn from_tx_lengths<R>(tx_lengths: Vec<Vec<usize>>, rng: &mut R) -> Self
    where
        R: RngCore,
    {
        let mut nss = HashMap::new();
        for (ns_index, tx_lens) in tx_lengths.into_iter().enumerate() {
            let ns_id = NamespaceId::from(ns_index as u64);
            for len in tx_lens {
                let ns: &mut Vec<_> = nss.entry(ns_id).or_default();
                ns.push(Transaction::new(ns_id, random_bytes(len, rng)));
            }
        }
        Self { nss }
    }

    fn many_from_tx_lengths<R>(test_cases: Vec<Vec<Vec<usize>>>, rng: &mut R) -> Vec<Self>
    where
        R: RngCore,
    {
        test_cases
            .into_iter()
            .map(|t| Self::from_tx_lengths(t, rng))
            .collect()
    }

    fn all_txs(&self) -> Vec<Transaction> {
        self.nss.iter().flat_map(|(_, txs)| txs.clone()).collect()
    }
}

fn random_bytes<R: RngCore>(len: usize, rng: &mut R) -> Vec<u8> {
    let mut result = vec![0; len];
    rng.fill_bytes(&mut result);
    result
}
