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

#[cfg(test)]
impl NsProof {
    pub fn is_existence(&self) -> bool {
        self.existence.is_some()
    }
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

        let ns_range = if let Some(ns_index) = self.ns_iter().find(|i| ns_id == i.ns_id) {
            ns_index.ns_range
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
        let ns_index = self.ns_iter().find(|i| ns_proof.ns_id == i.ns_id);

        match (ns_index, &ns_proof.existence) {
            (Some(ns_index), Some(pf)) => {
                vid_scheme(VidSchemeType::get_num_storage_nodes(&pf.vid_common))
                    .payload_verify(
                        Statement {
                            payload_subslice: &pf.ns_payload_flat,
                            range: ns_index.ns_range,
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
