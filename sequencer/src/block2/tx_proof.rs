use crate::{
    block2::{iter::Index, newtypes::TxPayloadRange, payload::Payload, tx_iter::TxIndex},
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
        AsPayloadBytes, NumTxs2, NumTxsRange2, PayloadBytesRange, TxTableEntries2,
        TxTableEntriesRange2,
    },
    ns_table::NsTable,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxProof2 {
    // Naming conventions for this struct's fields:
    // - `payload_x`: bytes from the payload
    // - `payload_proof_x`: a proof of those bytes from the payload
    tx_index: TxIndex,

    // Number of txs declared in the tx table
    payload_num_txs: NumTxs2,
    payload_proof_num_txs: SmallRangeProofType,

    // Tx table entries for this tx
    payload_tx_table_entries: TxTableEntries2,
    payload_proof_tx_table_entries: SmallRangeProofType,

    // This tx's payload bytes.
    // `None` if this tx has zero length.
    payload_proof_tx: Option<SmallRangeProofType>,
}

impl TxProof2 {
    pub fn new2(
        index: &Index,
        payload: &Payload,
        common: &VidCommon,
    ) -> Option<(Transaction, Self)> {
        let payload_byte_len = payload.as_byte_slice().len(); // TODO newtype?

        if payload_byte_len != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: common inconsistent with self
        }
        if !payload.ns_table().in_bounds(index.ns()) {
            return None; // error: ns index out of bounds
        }
        // check tx index below

        let ns_payload_range = payload
            .ns_table()
            .ns_payload_range2(index.ns(), payload_byte_len);
        let ns_payload = payload.read_ns_payload(&ns_payload_range);
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let num_txs_range = NumTxsRange2::new(&ns_payload_range.byte_len());
        let payload_num_txs = ns_payload.read(&num_txs_range);
        if !ns_payload_range.in_bounds(index.tx(), &payload_num_txs) {
            return None; // error: tx index out of bounds
        }
        let payload_proof_num_txs = vid
            .payload_proof(
                payload.as_byte_slice(),
                num_txs_range.block_payload_range(ns_payload_range.offset()),
            )
            .ok()?;

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        let tx_table_entries_range = TxTableEntriesRange2::new(index.tx());
        let payload_tx_table_entries = ns_payload.read(&tx_table_entries_range);
        let payload_proof_tx_table_entries = {
            vid.payload_proof(
                payload.as_byte_slice(),
                tx_table_entries_range.block_payload_range(ns_payload_range.offset()),
            )
            .ok()?
        };

        // Read the tx payload and compute a proof of correctness.
        let tx_payload_range = TxPayloadRange::new(
            &payload_num_txs,
            &payload_tx_table_entries,
            &ns_payload_range.byte_len(),
        );
        let payload_proof_tx = {
            let range = tx_payload_range.block_payload_range(ns_payload_range.offset());

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
        // TODO a helper would help here
        let tx = {
            let ns_id = payload.ns_table().read_ns_id(index.ns());
            let tx_payload = ns_payload
                .read(&tx_payload_range)
                .to_payload_bytes()
                .as_ref()
                .to_vec();
            Transaction::new(ns_id, tx_payload)
        };

        Some((
            tx,
            TxProof2 {
                tx_index: index.tx().clone(),
                payload_num_txs,
                payload_proof_num_txs,
                payload_tx_table_entries,
                payload_proof_tx_table_entries,
                payload_proof_tx,
            },
        ))
    }

    pub fn verify(
        &self,
        ns_table: &NsTable,
        tx: &Transaction,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<bool> {
        VidSchemeType::is_consistent(commit, common).ok()?;
        let Some(ns_index) = ns_table.find_ns_id(&tx.namespace()) else {
            return None; // error: ns id does not exist
        };
        let ns_payload_range =
            ns_table.ns_payload_range2(&ns_index, VidSchemeType::get_payload_byte_len(common));
        if !ns_payload_range.in_bounds(&self.tx_index, &self.payload_num_txs) {
            return None; // error: tx index out of bounds
        }
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Verify proof for tx table len
        {
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: self.payload_num_txs.to_payload_bytes().as_ref(),
                        range: NumTxsRange2::new(&ns_payload_range.byte_len())
                            .block_payload_range(ns_payload_range.offset()),
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
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: self.payload_tx_table_entries.to_payload_bytes().as_ref(),
                        range: TxTableEntriesRange2::new(&self.tx_index)
                            .block_payload_range(ns_payload_range.offset()),
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
            let range = TxPayloadRange::new(
                &self.payload_num_txs,
                &self.payload_tx_table_entries,
                &ns_payload_range.byte_len(),
            )
            .block_payload_range(ns_payload_range.offset());

            tracing::info!(
                "verify: tx_index {:?}, tx_payload_range {:?}, content {:?}",
                self.tx_index,
                range,
                tx.payload()
            );

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
