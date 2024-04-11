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
        let all_txs = test.all_txs();
        let block = Payload::from_transactions(test.all_txs()).unwrap().0;
        let disperse_data = vid.disperse(&block.payload).unwrap();

        // test `QueryablePayload::transaction_with_proof`
        for (tx_index, test_tx) in block.iter(&block.ns_table).zip(all_txs.iter()) {
            let tx = block.transaction(&tx_index).unwrap();
            assert_eq!(&tx, test_tx);
        }

        assert_eq!(block.num_namespaces(), test.nss.len());
        for ns_id in block.ns_iter().map(|i| i.ns_id) {
            // tracing::info!("test ns_id {}", ns.ns_id);

            let txs = test
                .nss
                .remove(&ns_id)
                .expect("block ns_id missing from test");

            let ns_proof = block
                .namespace_with_proof(ns_id, disperse_data.common.clone())
                .expect("namespace_with_proof should succeed");

            assert!(ns_proof.is_existence());

            let (ns_proof_txs, ns_proof_ns_id) = block
                .verify_namespace_proof(&ns_proof, &disperse_data.commit)
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
