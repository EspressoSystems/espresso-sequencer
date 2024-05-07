use crate::{
    block2::{
        iter::Index,
        ns_table::{ns_payload::tx_iter::TxIndex, ns_payload_range::NsPayloadRange},
        num_txs::NumTxs,
        tx_table_entries::TxTableEntries,
        Payload,
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

impl Payload {
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        // TODO check index.ns() in bounds
        // TODO don't copy the tx bytes into the return value
        // https://github.com/EspressoSystems/hotshot-query-service/issues/267
        Some(
            self.ns_payload(index.ns())
                .export_tx(&self.ns_table.read_ns_id(index.ns()), index.tx()),
        )
    }

    pub fn transaction_with_proof(
        &self,
        index: &Index,
        common: &VidCommon,
    ) -> Option<(Transaction, TxProof)> {
        if self.payload.len() != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: common inconsistent with self
        }

        // TODO check index.ns() in bounds
        let ns_payload = self.ns_payload(index.ns());
        let ns_payload_range = self
            .ns_table
            .ns_payload_range(index.ns(), self.payload.len());

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let payload_num_txs = ns_payload.read_num_txs();
        let payload_proof_num_txs = vid
            .payload_proof(&self.payload, ns_payload_range.num_txs_range())
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

            vid.payload_proof(&self.payload, range).ok()?
        };

        // Read the tx payload and compute a proof of correctness.
        let payload_proof_tx = {
            // TODO sucks that I need ns_payload AND ns_payload_range here.
            // should be able to get this with less...
            //
            // TODO I'm re-reading the tx_payload_range here... because I want automatic translaction by ns_payload_range?
            // In `verify` I don't have this luxury; Perhaps I should instead compute the tx_payload_range the same way I do in `verify`?
            // let range = ns_payload.tx_payload_range(index.tx(), &ns_payload_range);

            // TODO (i) payload_xxx should be a newtype(usize) that serializes to bytes
            let range =
                ns_payload_range.tx_payload_range(&payload_num_txs, &payload_tx_table_entries);

            tracing::info!(
                "prove: (ns,tx) ({:?},{:?}), tx_payload_range {:?}, content {:?}",
                index.ns(),
                index.tx(),
                range,
                &self.payload[range.clone()]
            );

            if range.is_empty() {
                None
            } else {
                Some(vid.payload_proof(&self.payload, range).ok()?)
            }
        };

        Some((
            self.transaction(index)?,
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
}

impl TxProof {
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
