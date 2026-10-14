use anyhow::{Context, Result, bail, ensure};
use espresso_types::{Header, NamespaceId, NsProof, Transaction};
use hotshot_types::data::VidCommon;
use serde::{Deserialize, Serialize};

use super::payload::check_vid_param;

/// Information required to verify a payload.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]

pub struct NamespaceProof {
    proof: Option<NonEmptyNamespaceProof>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NonEmptyNamespaceProof {
    proof: NsProof,
    common: VidCommon,
}

impl NamespaceProof {
    /// Construct a [`NamespaceProof`].
    ///
    /// Takes the underlying [`NsProof`], plus corresponding [`VidCommon`] data to allow a client to
    /// verify the proof.
    pub fn new(proof: NsProof, common: VidCommon) -> Self {
        Self {
            proof: Some(NonEmptyNamespaceProof { proof, common }),
        }
    }

    /// Create a trivial proof for a namespace which is not present in a given block.
    pub fn not_present() -> Self {
        Self { proof: None }
    }

    /// Verify a [`NamespaceProof`].
    ///
    /// `total_weight` is the VID total weight of the stake table the block was dispersed with,
    /// which the light client derives itself rather than taking from the proof. If the data in
    /// this proof matches the expected `header` and belongs to `namespace`, the list of
    /// transactions from the namespace is returned.
    pub fn verify(
        &self,
        header: &Header,
        namespace: NamespaceId,
        total_weight: usize,
    ) -> Result<Vec<Transaction>> {
        let Some(proof) = &self.proof else {
            // A trivial proof is a claim that the requested namespace is not present in the
            // namespace table. We need to verify this claim.
            if let Some(ix) = header.ns_table().find_ns_id(&namespace) {
                bail!(
                    "received trivial proof for missing namespace, but requested namespace \
                     {namespace} is present at position {ix:?}"
                );
            }
            return Ok(vec![]);
        };

        check_vid_param(&proof.common, total_weight)?;
        let (txs, ns) = proof
            .proof
            .verify(
                header.ns_table(),
                &header.payload_commitment(),
                &proof.common,
            )
            .context("invalid namespace proof")?;
        ensure!(
            ns == namespace,
            "proof is for wrong namespace {ns}, expected namespace {namespace}"
        );
        Ok(txs)
    }
}

#[cfg(test)]
mod test {
    use espresso_types::{Leaf2, NodeState};
    use hotshot_query_service_types::availability::TransactionIndex;
    use hotshot_types::vid::avidm::AvidMParam;
    use versions::FEE_VERSION;

    use super::*;
    use crate::testing::TestClient;

    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_non_empty() {
        let client = TestClient::default();
        let leaf = client.leaf(1).await;
        let payload = client.payload(1).await;
        let common = client.vid_common(1).await;

        let tx = payload
            .transaction(&TransactionIndex {
                ns_index: 0.into(),
                position: 0,
            })
            .unwrap();
        let total_weight = client.vid_total_weight(1).await;
        let proof =
            NamespaceProof::new(NsProof::new(&payload, &0.into(), &common).unwrap(), common);
        assert_eq!(
            proof
                .verify(leaf.header(), tx.namespace(), total_weight)
                .unwrap(),
            vec![tx.clone()]
        );

        // Check that a trivial proof is not accepted for a non-trivial namespace.
        let err = NamespaceProof::not_present()
            .verify(leaf.header(), tx.namespace(), total_weight)
            .unwrap_err();
        assert!(
            err.to_string()
                .contains("received trivial proof for missing namespace"),
            "{err:#}"
        );
    }

    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_empty() {
        let leaf = Leaf2::genesis(&Default::default(), &NodeState::mock(), FEE_VERSION).await;
        let proof = NamespaceProof::not_present();

        assert_eq!(
            proof.verify(leaf.block_header(), 0u64.into(), 1).unwrap(),
            vec![]
        );
    }

    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_invalid_wrong_payload() {
        let client = TestClient::default();

        let payload1 = client.payload(1).await;
        let common1 = client.vid_common(1).await;
        let tx = payload1
            .transaction(&TransactionIndex {
                ns_index: 0.into(),
                position: 0,
            })
            .unwrap();
        let proof = NamespaceProof::new(
            NsProof::new(&payload1, &0.into(), &common1).unwrap(),
            common1,
        );

        let leaf2 = client.leaf(2).await;
        let total_weight = client.vid_total_weight(2).await;
        let err = proof
            .verify(leaf2.header(), tx.namespace(), total_weight)
            .unwrap_err();
        assert!(
            err.to_string().contains("invalid namespace proof"),
            "{err:#}"
        );
    }

    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_invalid_wrong_namespace() {
        let client = TestClient::default();
        let leaf = client.leaf(1).await;
        let payload = client.payload(1).await;
        let common = client.vid_common(1).await;

        let tx = payload
            .transaction(&TransactionIndex {
                ns_index: 0.into(),
                position: 0,
            })
            .unwrap();
        let proof =
            NamespaceProof::new(NsProof::new(&payload, &0.into(), &common).unwrap(), common);
        let err = proof
            .verify(
                leaf.header(),
                NamespaceId::from(u64::from(tx.namespace()) + 1),
                client.vid_total_weight(1).await,
            )
            .unwrap_err();
        assert!(
            err.to_string().contains("proof is for wrong namespace"),
            "{err:#}"
        );
    }

    /// The commitment does not bind the VID parameters a provider sends along with a proof, so
    /// they are checked against the stake table before they drive any computation.
    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_rejects_forged_vid_param() {
        let client = TestClient::default();
        let leaf = client.leaf(1).await;
        let payload = client.payload(1).await;
        let common = client.vid_common(1).await;

        let tx = payload
            .transaction(&TransactionIndex {
                ns_index: 0.into(),
                position: 0,
            })
            .unwrap();
        let ns_proof = NsProof::new(&payload, &0.into(), &common).unwrap();
        let VidCommon::V1(param) = common else {
            panic!("the test client serves AvidM commons");
        };
        let forged = VidCommon::V1(AvidMParam {
            recovery_threshold: 0,
            ..param
        });

        let err = NamespaceProof::new(ns_proof, forged)
            .verify(
                leaf.header(),
                tx.namespace(),
                client.vid_total_weight(1).await,
            )
            .unwrap_err();
        assert!(err.to_string().contains("stake table"), "{err:#}");
    }
}
