use super::{ns_payload_builder::parse_ns_payload, Payload};
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
    /// Verify a [`NsProof`] against a payload commitment.
    pub fn verify_namespace_proof(
        &self,
        ns_proof: &NsProof,
        commit: &VidCommitment,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        let ns_info = self
            .ns_iter_internal()
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
