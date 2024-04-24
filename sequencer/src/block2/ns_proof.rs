use super::{ns_iter::NsTable, tx_iter::parse_ns_payload, Payload};
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
}

impl Payload {
    /// Returns the payload bytes for namespace `ns_id`, along with a proof of
    /// correctness for those bytes.
    pub fn namespace_with_proof(&self, ns_id: NamespaceId, common: &VidCommon) -> Option<NsProof> {
        if self.payload.len() != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: vid_common inconsistent with self
        }
        let Some(ns_index) = self.ns_table.find_ns_id(&ns_id) else {
            // ns_id does not exist
            return Some(NsProof {
                ns_id,
                existence: None,
            });
        };
        let ns_range = self.ns_table.ns_payload_range(ns_index, self.payload.len());

        Some(NsProof {
            ns_id,
            existence: Some(NsProofExistence {
                ns_payload_flat: self.payload[ns_range.clone()].into(),
                ns_proof: vid_scheme(VidSchemeType::get_num_storage_nodes(common))
                    .payload_proof(&self.payload, ns_range)
                    .ok()?, // error: failure to make a payload proof
            }),
        })
    }
}
impl NsProof {
    /// Verify a [`NsProof`] against a payload commitment.
    pub fn verify_namespace_proof(
        &self,
        ns_table: &NsTable,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        VidSchemeType::is_consistent(commit, common).ok()?;

        let ns_index = ns_table.find_ns_id(&self.ns_id);

        match (ns_index, &self.existence) {
            (Some(ns_index), Some(pf)) => {
                vid_scheme(VidSchemeType::get_num_storage_nodes(common))
                    .payload_verify(
                        Statement {
                            payload_subslice: &pf.ns_payload_flat,
                            range: ns_table.ns_payload_range(
                                ns_index,
                                VidSchemeType::get_payload_byte_len(common),
                            ),
                            commit,
                            common,
                        },
                        &pf.ns_proof,
                    )
                    .ok()?
                    .ok()?;

                // verification succeeded, return some data
                // we know ns_id is correct because the corresponding ns_payload_range passed verification
                Some((
                    parse_ns_payload(&pf.ns_payload_flat, self.ns_id),
                    self.ns_id,
                ))
            }
            (None, None) => Some((Vec::new(), self.ns_id)), // successful verification of nonexistence
            (None, Some(_)) | (Some(_), None) => None, // error: expect [non]existence but found the opposite
        }
    }
}
