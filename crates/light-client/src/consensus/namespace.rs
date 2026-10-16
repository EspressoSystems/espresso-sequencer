use std::borrow::Cow;

use anyhow::{Context, Result, bail, ensure};
use espresso_types::{Header, NamespaceId, NsProof, Transaction};
use hotshot_types::{data::VidCommon, vid::avidm::init_avidm_param};
use serde::{Deserialize, Serialize};

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
    /// If the data in this proof matches the expected `header` and belongs to `namespace`, the list
    /// of transactions from the namespace is returned.
    ///
    /// `vid_total_weight` is the total VID weight consensus dispersed the block with, which
    /// [`vid_total_weight_of_stakes`](hotshot_types::data::vid_disperse::vid_total_weight_of_stakes)
    /// computes from the block's stake table. A bad-encoding proof is verified under the VID
    /// parameter this weight fixes, not under the one the proof came with.
    pub fn verify(
        &self,
        header: &Header,
        namespace: NamespaceId,
        vid_total_weight: usize,
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

        // Nothing binds the V1 VID parameter to the commitment, and under a smaller recovery
        // threshold than consensus used any honest share passes as a bad-encoding proof.
        let common = match &proof.proof {
            NsProof::V1IncorrectEncoding(_) => {
                Cow::Owned(VidCommon::V1(init_avidm_param(vid_total_weight)?))
            },
            NsProof::V0(_) | NsProof::V1(_) | NsProof::V2(_) => Cow::Borrowed(&proof.common),
        };
        let (txs, ns) = proof
            .proof
            .verify(header.ns_table(), &header.payload_commitment(), &common)
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
    use espresso_types::{Leaf2, NodeState, NsIndex, v0_3::AvidMIncorrectEncodingNsProof};
    use hotshot_query_service_types::availability::TransactionIndex;
    use hotshot_types::{
        data::{VidCommitment, ns_table::parse_ns_table},
        traits::block_contents::{EncodeBytes, GENESIS_VID_NUM_STORAGE_NODES},
        vid::avidm::{AvidMScheme, init_avidm_param},
    };
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
        let vid_total_weight = client.vid_total_weight(1).await;
        let proof =
            NamespaceProof::new(NsProof::new(&payload, &0.into(), &common).unwrap(), common);
        assert_eq!(
            proof
                .verify(leaf.header(), tx.namespace(), vid_total_weight)
                .unwrap(),
            vec![tx.clone()]
        );

        // Check that a trivial proof is not accepted for a non-trivial namespace.
        let err = NamespaceProof::not_present()
            .verify(leaf.header(), tx.namespace(), vid_total_weight)
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
            proof
                .verify(
                    leaf.block_header(),
                    0u64.into(),
                    GENESIS_VID_NUM_STORAGE_NODES
                )
                .unwrap(),
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
        let err = proof
            .verify(
                leaf2.header(),
                tx.namespace(),
                client.vid_total_weight(2).await,
            )
            .unwrap_err();
        assert!(
            err.to_string().contains("invalid namespace proof"),
            "{err:#}"
        );
    }

    /// Nothing binds the V1 VID parameter to the commitment, so a server can pick one with
    /// `total_weights = 1` and turn any honest share into a bad-encoding proof for a namespace
    /// that fits one erasure-coding chunk.
    #[tokio::test]
    #[test_log::test]
    async fn test_namespace_proof_rejects_bad_encoding_proof_under_forged_param() {
        let client = TestClient::default();
        let height = 1;
        let namespace = NamespaceId::from(1u32);
        client
            .add_block(height, vec![Transaction::new(namespace, vec![7; 20])])
            .await;
        let header = client.leaf(height).await.header().clone();
        let bytes = client.payload(height).await.encode();
        let VidCommon::V1(param) = client.vid_common(height as u64).await else {
            panic!("test harness builds V1 blocks");
        };
        assert!(param.recovery_threshold > 1);

        let weights = vec![1; param.total_weights];
        let ns_table = parse_ns_table(bytes.len(), &header.ns_table().encode());
        let (commit, shares) =
            AvidMScheme::ns_disperse(&param, &weights, &bytes, ns_table).unwrap();
        assert_eq!(VidCommitment::V1(commit), header.payload_commitment());

        let forged = init_avidm_param(1).unwrap();
        let proof = AvidMIncorrectEncodingNsProof::new(
            &shares,
            header.ns_table(),
            &NsIndex::from(0),
            &header.payload_commitment(),
            &forged,
        )
        .unwrap();
        let proof = NamespaceProof::new(NsProof::V1IncorrectEncoding(proof), VidCommon::V1(forged));

        proof
            .verify(&header, namespace, param.total_weights)
            .unwrap_err();
        // the weight is the only thing standing in the way
        assert_eq!(proof.verify(&header, namespace, 1).unwrap(), vec![]);
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
}
