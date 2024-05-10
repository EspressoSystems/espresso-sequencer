use crate::{
    block2::{
        iter::Index, ns_payload_range::NsPayloadRange, num_txs::NumTxs, payload::Payload,
        tx_iter::TxIndex, tx_table_entries::TxTableEntries,
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

use super::NsPayloadRange2;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxProof {
    // Naming conventions for this struct's fields:
    // - `payload_x`: bytes from the payload
    // - `payload_proof_x`: a proof of those bytes from the payload
    ns_payload_range: NsPayloadRange,
    tx_index: TxIndex,

    // Number of txs declared in the tx table
    payload_num_txs: NumTxs,
    payload_proof_num_txs: SmallRangeProofType,

    // Tx table entries for this tx
    payload_tx_table_entries: TxTableEntries,
    payload_proof_tx_table_entries: SmallRangeProofType,

    // This tx's payload bytes.
    // `None` if this tx has zero length.
    payload_proof_tx: Option<SmallRangeProofType>,
}

pub struct TxProof2 {
    ns_payload_range: NsPayloadRange2,
    tx_index: TxIndex,

    // Number of txs declared in the tx table
    payload_num_txs: NumTxs,
    payload_proof_num_txs: SmallRangeProofType,

    // Tx table entries for this tx
    payload_tx_table_entries: TxTableEntries,
    payload_proof_tx_table_entries: SmallRangeProofType,
}

impl TxProof2 {
    pub fn new2(
        index: &Index,
        payload: &Payload,
        common: &VidCommon,
    ) -> Option<(Transaction, Self)> {
        let ns_payload = payload.ns_payload2(index.ns());
        let ns_payload_range = payload.ns_payload_range2(index.ns());
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let payload_num_txs = ns_payload.read(ns_payload_range.num_txs_range_relative());
        let payload_proof_num_txs = vid
            .payload_proof(payload.as_byte_slice(), ns_payload_range.num_txs_range())
            .ok()?;

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        let payload_tx_table_entries =
            ns_payload.read(ns_payload_range.tx_table_entries_range_relative(index.tx()));
        let payload_proof_tx_table_entries = {
            let range = ns_payload_range.tx_table_entries_range(index.tx());
            vid.payload_proof(payload.as_byte_slice(), range).ok()?
        };

        Some((
            payload.transaction(index)?,
            TxProof2 {
                ns_payload_range,
                tx_index: index.tx().clone(),
                payload_num_txs,
                payload_proof_num_txs,
                payload_tx_table_entries,
                payload_proof_tx_table_entries,
            },
        ))
    }

    pub fn verify(
        &self,
        _tx: &Transaction,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<bool> {
        VidSchemeType::is_consistent(commit, common).ok()?;
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Verify proof for tx table len
        {
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: &self.payload_num_txs.as_bytes(),
                        range: self.ns_payload_range.num_txs_range(),
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
            // TODO this is the only place we use `self.tx_index`. But if we
            // want to eliminate it then we need another way to get the
            // tx_table_entries_range -> so we'd have to replace `tx_index` with
            // a new range for tx_table entries, which is no improvement.
            // Basically `tx_index` is a way to compress this range.
            let range = self.ns_payload_range.tx_table_entries_range(&self.tx_index);

            // concatenate the two table entry payloads
            let payload_subslice = &self.payload_tx_table_entries.as_bytes();

            // tracing::info!(
            //     "verify: tx_index {:?}, tx_table_entries_range {:?}, content {:?}",
            //     self.tx_index,
            //     range,
            //     payload_subslice
            // );

            if vid
                .payload_verify(
                    Statement {
                        payload_subslice,
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

        Some(true)
    }
}

impl TxProof {
    pub fn new(
        index: &Index,
        payload: &Payload,
        common: &VidCommon,
    ) -> Option<(Transaction, Self)> {
        if payload.as_byte_slice().len() != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: common inconsistent with self
        }

        if !payload.ns_table().in_bounds(index.ns()) {
            return None; // error: ns index out of bounds
        }

        let ns_payload = payload.ns_payload(index.ns());

        if !ns_payload.in_bounds(index.tx()) {
            return None; // error: tx index out of bounds
        }

        let ns_payload_range = payload.ns_payload_range(index.ns());
        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let payload_num_txs = ns_payload.read_num_txs();
        let payload_proof_num_txs = vid
            .payload_proof(payload.as_byte_slice(), ns_payload_range.num_txs_range())
            .ok()?;

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        let payload_tx_table_entries = ns_payload.read_tx_table_entries(index.tx());
        let payload_proof_tx_table_entries = {
            let range = ns_payload_range.tx_table_entries_range(index.tx());

            // tracing::info!(
            //     "prove: (ns,tx) ({:?},{:?}), tx_table_entries_range {:?}, content {:?}",
            //     index.ns(),
            //     index.tx(),
            //     range,
            //     &self.payload[range.clone()]
            // );

            vid.payload_proof(payload.as_byte_slice(), range).ok()?
        };

        // Read the tx payload and compute a proof of correctness.
        let payload_proof_tx = {
            let range =
                ns_payload_range.tx_payload_range(&payload_num_txs, &payload_tx_table_entries);

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

        Some((
            payload.transaction(index)?,
            TxProof {
                ns_payload_range,
                payload_num_txs,
                payload_proof_num_txs,
                tx_index: index.tx().clone(),
                payload_tx_table_entries,
                payload_proof_tx_table_entries,
                payload_proof_tx,
            },
        ))
    }

    // - Returns `None` if an error occurred.
    // - `bool` result, or should we use `Result<(),()>` ?
    //
    // TODO we're not even checking the namespace id for `tx`. That would
    // require the ns_table so we can look up the corresponding ns_range, which
    // makes TxProof::ns_range redundant
    pub fn verify(
        &self,
        tx: &Transaction,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<bool> {
        VidSchemeType::is_consistent(commit, common).ok()?;

        // TODO need a way to check self.tx_index < self.num_txs. That's a pain
        // because tx_index is an opaque newtype. Solution: make a separate
        // newtype T for num_txs and make a method T::is_valid(index: TxIndex)
        // to check whether index is in bounds.

        // TODO check ns_payload_range: start <= end <= payload byte len

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Verify proof for tx table len
        {
            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: &self.payload_num_txs.as_bytes(),
                        range: self.ns_payload_range.num_txs_range(),
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
            // TODO this is the only place we use `self.tx_index`. But if we
            // want to eliminate it then we need another way to get the
            // tx_table_entries_range -> so we'd have to replace `tx_index` with
            // a new range for tx_table entries, which is no improvement.
            // Basically `tx_index` is a way to compress this range.
            let range = self.ns_payload_range.tx_table_entries_range(&self.tx_index);

            // concatenate the two table entry payloads
            let payload_subslice = &self.payload_tx_table_entries.as_bytes();

            // tracing::info!(
            //     "verify: tx_index {:?}, tx_table_entries_range {:?}, content {:?}",
            //     self.tx_index,
            //     range,
            //     payload_subslice
            // );

            if vid
                .payload_verify(
                    Statement {
                        payload_subslice,
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
            let range = self
                .ns_payload_range
                .tx_payload_range(&self.payload_num_txs, &self.payload_tx_table_entries);

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
