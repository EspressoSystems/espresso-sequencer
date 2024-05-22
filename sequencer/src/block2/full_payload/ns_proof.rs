use crate::{
    block2::{
        full_payload::{ns_table::NsTable, payload::Payload, payload::PayloadByteLen},
        namespace_payload::NsPayloadOwned,
    },
    NamespaceId, Transaction,
};
use hotshot_types::vid::{
    vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType,
};
use jf_vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use serde::{Deserialize, Serialize};

/// Proof of correctness for namespace payload bytes in a block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NsProof {
    ns_id: NamespaceId,
    existence: Option<NsProofExistence>, // `None` if `ns_id` is not in the block.
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct NsProofExistence {
    ns_payload: NsPayloadOwned,
    ns_proof: LargeRangeProofType,
}

impl NsProof {
    /// Returns the payload bytes for namespace `ns_id`, along with a proof of
    /// correctness for those bytes. Returns `None` on error.
    ///
    /// The namespace payload is included as a hidden field in the returned
    /// [`NsProof`]. A conventional API would instead return `(NsPayload,
    /// NsProof)` and [`NsProof`] would not contain the namespace payload.
    /// ([`TxProof::new`](super::tx_proof::TxProof::new) conforms to this
    /// convention.) In the future we should change this API to conform to
    /// convention. But that would require a change to our RPC endpoint API at
    /// [`endpoints`](crate::api::endpoints), which is a hassle.
    pub fn new(payload: &Payload, ns_id: NamespaceId, common: &VidCommon) -> Option<NsProof> {
        let payload_byte_len = payload.byte_len();
        if !payload_byte_len.is_consistent(common) {
            return None; // error: vid_common inconsistent with self
        }
        let Some(ns_index) = payload.ns_table().find_ns_id(&ns_id) else {
            // ns_id does not exist
            return Some(NsProof {
                ns_id,
                existence: None,
            });
        };

        let ns_payload_range = payload.ns_table().ns_range(&ns_index, &payload_byte_len);

        // TODO vid_scheme() arg should be u32
        let vid = vid_scheme(
            VidSchemeType::get_num_storage_nodes(common)
                .try_into()
                .unwrap(),
        );

        Some(NsProof {
            ns_id,
            existence: Some(NsProofExistence {
                ns_payload: payload.read_ns_payload(&ns_payload_range).to_owned(),
                ns_proof: vid
                    .payload_proof(payload.as_byte_slice(), ns_payload_range.as_block_range())
                    .ok()?,
            }),
        })
    }

    /// Verify a [`NsProof`] against a payload commitment. Returns `None` on
    /// error or if verification fails.
    ///
    /// There is no [`NsPayload`](super::ns_payload::NsPayload) arg because this
    /// data is already included in the [`NsProof`]. See [`NsProof::new`] for
    /// discussion.
    ///
    /// If verification is successful then return `(Vec<Transaction>,
    /// NamespaceId)` obtained by post-processing the underlying
    /// [`NsPayload`](super::ns_payload::NsPayload). Why? This method might be
    /// run by a client in a WASM environment who might be running non-Rust
    /// code, in which case the client is unable to perform this post-processing
    /// himself.
    pub fn verify(
        &self,
        ns_table: &NsTable,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        VidSchemeType::is_consistent(commit, common).ok()?;
        let ns_index = ns_table.find_ns_id(&self.ns_id);

        match (ns_index, &self.existence) {
            (Some(ns_index), Some(pf)) => {
                let vid = vid_scheme(
                    VidSchemeType::get_num_storage_nodes(common)
                        .try_into()
                        .unwrap(),
                );
                let range = ns_table
                    .ns_range(&ns_index, &PayloadByteLen::from_vid_common(common))
                    .as_block_range();
                vid.payload_verify(
                    Statement {
                        payload_subslice: pf.ns_payload.as_bytes_slice(),
                        range,
                        commit,
                        common,
                    },
                    &pf.ns_proof,
                )
                .ok()?
                .ok()?;

                // verification succeeded, return some data
                Some((pf.ns_payload.export_all_txs(&self.ns_id), self.ns_id))
            }
            (None, None) => Some((Vec::new(), self.ns_id)), // successful verification of nonexistence
            (None, Some(_)) | (Some(_), None) => {
                tracing::info!("ns verify: expect [non]existence but found the opposite");
                None // error: expect [non]existence but found the opposite
            }
        }
    }

    /// Does this proof indicate existence or non-existence of a namespace id?
    pub fn is_existence(&self) -> bool {
        self.existence.is_some()
    }
}
