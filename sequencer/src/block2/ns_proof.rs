use super::{tx_iter::parse_ns_payload, Payload};
use crate::{NamespaceId, Transaction};
use hotshot_types::vid::{
    vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType,
};
use jf_primitives::vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NsProof {
    ns_id: NamespaceId,

    // `None` if namespace ID `ns_id` is not in the block.
    existence: Option<NsProofExistence>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct NsProofExistence {
    #[serde(with = "base64_bytes")]
    ns_payload_flat: Vec<u8>,
    ns_proof: LargeRangeProofType,
    vid_common: VidCommon,
}

impl Payload {
    /// Returns the payload bytes for namespace `ns_id`, along with a proof of
    /// correctness for those bytes.
    ///
    /// RPC-friendly proof contains everything not already available to the
    /// verifier in the block header:
    /// - the namespace payload bytes
    /// - `vid_common` needed to verify the proof
    pub fn namespace_with_proof(
        &self,
        ns_id: NamespaceId,
        vid_common: VidCommon,
    ) -> Option<NsProof> {
        if self.payload.len() != VidSchemeType::get_payload_byte_len(&vid_common) {
            return None; // error: vid_common inconsistent with self
        }

        let ns_range = if let Some(ns_info) = self.ns_index_iter().find(|info| ns_id == info.ns_id)
        {
            ns_info.ns_range
        } else {
            return Some(NsProof {
                ns_id,
                existence: None,
            });
        };

        Some(NsProof {
            ns_id,
            existence: Some(NsProofExistence {
                ns_payload_flat: self.payload[ns_range.clone()].into(),
                ns_proof: vid_scheme(VidSchemeType::get_num_storage_nodes(&vid_common))
                    .payload_proof(&self.payload, ns_range)
                    .ok()?, // error: failure to make a payload proof
                vid_common,
            }),
        })
    }

    /// Verify a [`NsProof`] against a payload commitment.
    pub fn verify_namespace_proof(
        &self,
        ns_proof: &NsProof,
        commit: &VidCommitment,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        let ns_info = self
            .ns_index_iter()
            .find(|info| ns_proof.ns_id == info.ns_id);

        match (ns_info, &ns_proof.existence) {
            (Some(info), Some(pf)) => {
                vid_scheme(VidSchemeType::get_num_storage_nodes(&pf.vid_common))
                    .payload_verify(
                        Statement {
                            payload_subslice: &pf.ns_payload_flat,
                            range: info.ns_range,
                            commit,
                            common: &pf.vid_common,
                        },
                        &pf.ns_proof,
                    )
                    .ok()?
                    .ok()?;

                // verification succeeded, return some data
                // we know ns_id is correct because the corresponding ns_payload_range passed verification
                Some((
                    parse_ns_payload(&pf.ns_payload_flat, ns_proof.ns_id),
                    ns_proof.ns_id,
                ))
            }
            (None, None) => Some((Vec::new(), ns_proof.ns_id)), // successful verification of nonexistence
            (None, Some(_)) | (Some(_), None) => None, // error: expect [non]existence but found the opposite
        }
    }
}

#[cfg(test)]
mod test {
    use super::Payload;
    use crate::{NamespaceId, Transaction};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use hotshot::traits::BlockPayload;
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
            let block = Payload::from_transactions(test.as_vec_tx()).unwrap().0;
            let disperse_data = vid.disperse(&block.payload).unwrap();

            assert_eq!(block.num_namespaces(), test.nss.len());
            for ns_id in block.ns_iter() {
                // tracing::info!("test ns_id {}", ns.ns_id);

                test.nss
                    .remove(&ns_id)
                    .expect("block ns_id missing from test");

                let ns_proof = block
                    .namespace_with_proof(ns_id, disperse_data.common.clone())
                    .expect("namespace_with_proof should succeed");

                assert!(ns_proof.existence.is_some());

                let (_ns_proof_txs, ns_proof_ns_id) = block
                    .verify_namespace_proof(&ns_proof, &disperse_data.commit)
                    .unwrap_or_else(|| panic!("namespace {} proof verification failure", ns_id));

                assert_eq!(ns_proof_ns_id, ns_id);
            }
            assert!(
                test.nss.is_empty(),
                "not all test namespaces consumed by ns_iter"
            );
        }
    }

    // TODO lots of infra here that could be reused in other tests.
    struct ValidTest {
        nss: HashMap<NamespaceId, Vec<Vec<u8>>>,
    }

    impl ValidTest {
        fn from_tx_lengths<R>(tx_lengths: Vec<Vec<usize>>, rng: &mut R) -> Self
        where
            R: RngCore,
        {
            let mut txs = HashMap::new();
            for (ns_index, tx_lens) in tx_lengths.into_iter().enumerate() {
                let ns_id = NamespaceId::from(ns_index as u64);
                for len in tx_lens {
                    let ns: &mut Vec<Vec<u8>> = txs.entry(ns_id).or_default();
                    ns.push(random_bytes(len, rng));
                }
            }
            Self { nss: txs }
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

        fn as_vec_tx(&self) -> Vec<Transaction> {
            let mut txs = Vec::new();
            for (ns_id, tx_payloads) in self.nss.iter() {
                for tx_payload in tx_payloads {
                    txs.push(Transaction::new(*ns_id, tx_payload.clone()));
                }
            }
            txs
        }
    }

    fn random_bytes<R: RngCore>(len: usize, rng: &mut R) -> Vec<u8> {
        let mut result = vec![0; len];
        rng.fill_bytes(&mut result);
        result
    }
}
