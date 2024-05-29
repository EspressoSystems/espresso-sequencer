use crate::{
    block::{
        full_payload::{ns_table::NsTable, payload::Payload, payload::PayloadByteLen},
        namespace_payload::NsPayloadOwned,
    },
    NamespaceId, Transaction,
};
use hotshot_types::{
    traits::EncodeBytes,
    vid::{vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType},
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
    ns_payload: NsPayloadOwned,
    ns_proof: LargeRangeProofType,
}

impl NsProof {
    /// Returns the payload bytes for namespace `ns_id`, along with a proof of
    /// correctness for those bytes. Returns `None` if `ns_id` is not in the
    /// namespace table, or on error.
    ///
    /// The namespace payload [`NsPayloadOwned`] is included as a hidden field
    /// in the returned [`NsProof`]. A conventional API would instead return
    /// `(NsPayload, NsProof)` and [`NsProof`] would not contain the namespace
    /// payload.
    /// ([`TxProof::new`](crate::block::namespace_payload::TxProof::new)
    /// conforms to this convention.) In the future we should change this API to
    /// conform to convention. But that would require a change to our RPC
    /// endpoint API at [`endpoints`](crate::api::endpoints), which is a hassle.
    pub fn new(payload: &Payload, ns_id: NamespaceId, common: &VidCommon) -> Option<NsProof> {
        let payload_byte_len = payload.byte_len();
        payload_byte_len.is_consistent(common).ok()?;
        let ns_index = payload.ns_table().find_ns_id(&ns_id)?;
        let ns_payload_range = payload.ns_table().ns_range(&ns_index, &payload_byte_len);

        // TODO vid_scheme() arg should be u32 to match get_num_storage_nodes
        let vid = vid_scheme(
            VidSchemeType::get_num_storage_nodes(common)
                .try_into()
                .ok()?, // error: failure to convert u32 to usize
        );

        Some(NsProof {
            ns_id,
            ns_payload: payload.read_ns_payload(&ns_payload_range).to_owned(),
            ns_proof: vid
                .payload_proof(payload.encode(), ns_payload_range.as_block_range())
                .ok()?, // error: internal to payload_proof()
        })
    }

    /// Verify a [`NsProof`] against a payload commitment. Returns `None` on
    /// error or if verification fails.
    ///
    /// There is no [`NsPayload`](crate::block::namespace_payload::NsPayload)
    /// arg because this data is already included in the [`NsProof`]. See
    /// [`NsProof::new`] for discussion.
    ///
    /// If verification is successful then return `(Vec<Transaction>,
    /// NamespaceId)` obtained by post-processing the underlying
    /// [`NsPayload`](crate::block::namespace_payload::NsPayload). Why? This
    /// method might be run by a client in a WASM environment who might be
    /// running non-Rust code, in which case the client is unable to perform
    /// this post-processing himself.
    pub fn verify(
        &self,
        ns_table: &NsTable,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<(Vec<Transaction>, NamespaceId)> {
        VidSchemeType::is_consistent(commit, common).ok()?;
        let Some(ns_index) = ns_table.find_ns_id(&self.ns_id) else {
            return None; // error: ns_id does not exist
        };

        // TODO vid_scheme() arg should be u32 to match get_num_storage_nodes
        let vid = vid_scheme(
            VidSchemeType::get_num_storage_nodes(common)
                .try_into()
                .ok()?, // error: failure to convert u32 to usize
        );

        let range = ns_table
            .ns_range(&ns_index, &PayloadByteLen::from_vid_common(common))
            .as_block_range();
        vid.payload_verify(
            Statement {
                payload_subslice: self.ns_payload.as_bytes_slice(),
                range,
                commit,
                common,
            },
            &self.ns_proof,
        )
        .ok()? // error: internal to payload_verify()
        .ok()?; // verification failure

        // verification succeeded, return some data
        Some((self.ns_payload.export_all_txs(&self.ns_id), self.ns_id))
    }

    /// Return all transactions in the namespace whose payload is proven by
    /// `self`.
    ///
    /// # Design warning
    ///
    /// This method relies on a promise that a [`NsProof`] stores the entire
    /// namespace payload. If in the future we wish to remove the payload from a
    /// [`NsProof`] then this method can no longer be supported.
    ///
    /// In that case, use the following a workaround:
    /// - Given a [`NamespaceId`], get a
    ///   [`NsIndex`](crate::block::full_payload::NsIndex) `i` via
    ///   [`NsTable::find_ns_id`].
    /// - Use `i` to get a
    ///   [`NsPayload`](crate::block::namespace_payload::NsPayload) `p` via
    ///   [`Payload::ns_payload`].
    /// - Use `p` to get the desired [`Vec<Transaction>`] via
    ///   [`NsPayload::export_all_txs`](crate::block::namespace_payload::NsPayload::export_all_txs).
    ///
    /// This workaround duplicates the work done in [`NsProof::new`]. If you
    /// don't like that then you could instead hack [`NsProof::new`] to return a
    /// pair `(NsProof, Vec<Transaction>)`.
    pub fn export_all_txs(&self) -> Vec<Transaction> {
        self.ns_payload.export_all_txs(&self.ns_id)
    }
}
