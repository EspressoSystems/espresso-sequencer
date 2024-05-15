use crate::{
    block2::{ns_table::NsTable, payload::Payload},
    NamespaceId, Transaction,
};
use hotshot_types::vid::{
    vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType,
};
use jf_primitives::vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use serde::{Deserialize, Serialize};

use super::ns_payload::NsPayloadOwned;

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
    // TODO `#[serde(with = "base64_bytes")]` screws up serde for `NsPayloadOwned`.
    ns_payload: NsPayloadOwned,
    ns_proof: LargeRangeProofType,
}

impl NsProof {
    /// Returns the payload bytes for namespace `ns_id`, along with a proof of
    /// correctness for those bytes.
    pub fn new(payload: &Payload, ns_id: NamespaceId, common: &VidCommon) -> Option<NsProof> {
        let payload_byte_len = payload.as_byte_slice().len(); // TODO newtype?

        if payload.as_byte_slice().len() != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: vid_common inconsistent with self
        }
        let Some(ns_index) = payload.ns_table().find_ns_id(&ns_id) else {
            // ns_id does not exist
            return Some(NsProof {
                ns_id,
                existence: None,
            });
        };

        let ns_payload_range = payload
            .ns_table()
            .ns_payload_range2(&ns_index, payload_byte_len);
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        Some(NsProof {
            ns_id,
            existence: Some(NsProofExistence {
                ns_payload: payload.read_ns_payload(&ns_payload_range).to_owned(),
                ns_proof: vid
                    .payload_proof(payload.as_byte_slice(), ns_payload_range.as_range())
                    .ok()?,
            }),
        })
    }

    /// Verify a [`NsProof`] against a payload commitment.
    ///
    /// TODO the only way to verify `ns_id` is to look it up in the ns_table,
    /// read the ns_range from the ns_table, then verify the proof against the
    /// range we looked up. Also, the `ns_range` I just painstakingly added to
    /// `NsPayloadOwned` is now useless -> we ignore it in favour of what we
    /// find in the ns_table.
    ///
    /// TODO yep, we need to verify `ns_id` against `ns_table`, so we don't even
    /// need the ns_range in the proof. Same for tx proofs too. So you may as
    /// well remove the ns_range from the NsPayload (which enables the DST!).
    /// But you need to decide where to put the methods that translage ranges by
    /// the ns_range now that it's no longer with NsPayload.
    ///
    /// If we don't care about checking `ns_id` then we can instead rely on the
    /// ns_range in `NsPayloadOwned` and we can delete the `ns_table` arg from
    /// this method.
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
                            payload_subslice: pf.ns_payload.as_byte_slice(),
                            range: ns_table
                                .ns_payload_range2(
                                    &ns_index,
                                    VidSchemeType::get_payload_byte_len(common),
                                )
                                .as_range(),
                            commit,
                            common,
                        },
                        &pf.ns_proof,
                    )
                    .ok()?
                    .ok()?;

                // verification succeeded, return some data
                // we know ns_id is correct because the corresponding ns_payload_range passed verification
                Some((pf.ns_payload.export_all_txs(&self.ns_id), self.ns_id))
            }
            (None, None) => Some((Vec::new(), self.ns_id)), // successful verification of nonexistence
            (None, Some(_)) | (Some(_), None) => None, // error: expect [non]existence but found the opposite
        }
    }
}
