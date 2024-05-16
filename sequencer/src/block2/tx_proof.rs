use crate::{
    block2::{
        iter::Index,
        newtypes::TxPayloadRange,
        payload::{Payload, PayloadByteLen},
    },
    Transaction,
};
use hotshot_query_service::{VidCommitment, VidCommon};
use hotshot_types::vid::{vid_scheme, SmallRangeProofType, VidSchemeType};
use jf_primitives::vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use serde::{Deserialize, Serialize};

use super::{
    newtypes::{
        NumTxs, NumTxsRange, NumTxsUnchecked, TxIndex, TxTableEntries, TxTableEntriesRange,
    },
    ns_table::NsTable,
};

/// Proof of correctness for transaction bytes in a block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxProof {
    // Naming conventions for this struct's fields:
    // - `payload_x`: bytes from the payload
    // - `payload_proof_x`: a proof of those bytes from the payload
    tx_index: TxIndex,

    // Number of txs declared in the tx table
    payload_num_txs: NumTxsUnchecked,
    payload_proof_num_txs: SmallRangeProofType,

    // Tx table entries for this tx
    payload_tx_table_entries: TxTableEntries,
    payload_proof_tx_table_entries: SmallRangeProofType,

    // This tx's payload bytes.
    // `None` if this tx has zero length.
    payload_proof_tx: Option<SmallRangeProofType>,
}

impl TxProof {
    /// Returns the [`Transaction`] indicated by `index`, along with a proof of
    /// correctness for that transaction. Returns `None` on error.
    pub fn new(
        index: &Index,
        payload: &Payload,
        common: &VidCommon,
    ) -> Option<(Transaction, Self)> {
        let payload_byte_len = payload.byte_len();
        if !payload_byte_len.is_consistent(common) {
            tracing::info!("payload byte len inconsistent with vid_common");
            return None; // error: common inconsistent with self
        }
        if !payload.ns_table().in_bounds(index.ns()) {
            tracing::info!("ns_index {:?} out of bounds", index.ns());
            return None; // error: ns index out of bounds
        }
        // check tx index below

        let ns_range = payload.ns_table().ns_range(index.ns(), &payload_byte_len);
        let ns_byte_len = ns_range.byte_len();
        let ns_payload = payload.read_ns_payload(&ns_range);
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let num_txs_range = NumTxsRange::new(&ns_byte_len);
        let payload_num_txs = ns_payload.read(&num_txs_range);

        // Check tx index.
        //
        // TODO the next line of code (and other code) could be easier to read
        // if we make a helpers that repeat computation we've already done.
        if !NumTxs::new(&payload_num_txs, &ns_byte_len).in_bounds(index.tx()) {
            return None; // error: tx index out of bounds
        }

        let payload_proof_num_txs = vid
            .payload_proof(
                payload.as_byte_slice(),
                ns_range.block_range(&num_txs_range),
            )
            .ok()?;

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        let tx_table_entries_range = TxTableEntriesRange::new(index.tx());
        let payload_tx_table_entries = ns_payload.read(&tx_table_entries_range);
        let payload_proof_tx_table_entries = {
            vid.payload_proof(
                payload.as_byte_slice(),
                ns_range.block_range(&tx_table_entries_range),
            )
            .ok()?
        };

        // Read the tx payload and compute a proof of correctness.
        let tx_payload_range =
            TxPayloadRange::new(&payload_num_txs, &payload_tx_table_entries, &ns_byte_len);
        let payload_proof_tx = {
            let range = ns_range.block_range(&tx_payload_range);

            tracing::info!(
                "prove: (ns,tx) ({:?},{:?}), tx_payload_range {:?}, content {:?}",
                index.ns(),
                index.tx(),
                range,
                &payload.as_byte_slice()[range.clone()]
            );

            if range.is_empty() {
                None
            } else {
                Some(vid.payload_proof(payload.as_byte_slice(), range).ok()?)
            }
        };

        let tx = {
            let ns_id = payload.ns_table().read_ns_id(index.ns());
            let tx_payload = ns_payload
                .read(&tx_payload_range)
                .to_payload_bytes()
                .to_vec();
            Transaction::new(ns_id, tx_payload)
        };

        Some((
            tx,
            TxProof {
                tx_index: index.tx().clone(),
                payload_num_txs,
                payload_proof_num_txs,
                payload_tx_table_entries,
                payload_proof_tx_table_entries,
                payload_proof_tx,
            },
        ))
    }

    /// Verify a [`TxProof`] for `tx` against a payload commitment. Returns
    /// `None` on error.
    pub fn verify(
        &self,
        ns_table: &NsTable,
        tx: &Transaction,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<bool> {
        VidSchemeType::is_consistent(commit, common).ok()?;
        let Some(ns_index) = ns_table.find_ns_id(&tx.namespace()) else {
            tracing::info!("ns id {} does not exist", tx.namespace());
            return None; // error: ns id does not exist
        };
        let ns_range = ns_table.ns_range(&ns_index, &PayloadByteLen::from_vid_common(common));
        let ns_byte_len = ns_range.byte_len();

        if !NumTxs::new(&self.payload_num_txs, &ns_byte_len).in_bounds(&self.tx_index) {
            tracing::info!("tx index {:?} out of bounds", self.tx_index);
            return None; // error: tx index out of bounds
        }

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Verify proof for tx table len
        {
            let range = ns_range.block_range(&NumTxsRange::new(&ns_byte_len));
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: &self.payload_num_txs.to_payload_bytes(),
                        range,
                        commit,
                        common,
                    },
                    &self.payload_proof_num_txs,
                )
                .ok()?
                .is_err()
            {
                return Some(false);
            }
        }

        // Verify proof for tx table entries
        {
            let range = ns_range.block_range(&TxTableEntriesRange::new(&self.tx_index));
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: &self.payload_tx_table_entries.to_payload_bytes(),
                        range,
                        commit,
                        common,
                    },
                    &self.payload_proof_tx_table_entries,
                )
                .ok()?
                .is_err()
            {
                return Some(false);
            }
        }

        // Verify proof for tx payload
        {
            let range = ns_range.block_range(&TxPayloadRange::new(
                &self.payload_num_txs,
                &self.payload_tx_table_entries,
                &ns_byte_len,
            ));

            match (&self.payload_proof_tx, range.is_empty()) {
                (Some(proof), false) => {
                    if vid
                        .payload_verify(
                            Statement {
                                payload_subslice: tx.payload(),
                                range,
                                commit,
                                common,
                            },
                            proof,
                        )
                        .ok()?
                        .is_err()
                    {
                        return Some(false);
                    }
                }
                (None, true) => {} // 0-length tx, nothing to verify
                (None, false) => {
                    tracing::info!(
                        "tx verify: missing proof for nonempty tx payload range {:?}",
                        range
                    );
                    return None;
                }
                (Some(_), true) => {
                    tracing::info!("tx verify: unexpected proof for empty tx payload range");
                    return None;
                }
            }
        }

        Some(true)
    }
}
