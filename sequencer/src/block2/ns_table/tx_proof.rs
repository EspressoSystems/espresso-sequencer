use super::ns_payload::{tx_iter::TxIndex, NsPayloadRange};
use crate::{
    block2::{
        iter::Index,
        payload_bytes::{
            ns_offset_from_bytes, num_txs_from_bytes, tx_offset_as_bytes, tx_offset_from_bytes,
            NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
        },
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
    // Conventions:
    // - `payload_x`: bytes from the payload
    // - `payload_proof_x`: a proof of those bytes from the payload

    // TODO can we trust ns_range claims? Or do we need to take the ns table as
    // a separate arg, and replace ns_range_x here with ns_index into the ns
    // table. I think we can trust them because payload proofs are tied to a
    // specific location
    ns_payload_range: NsPayloadRange,

    // TODO make [u8; XXX] a newtype at a lower level of code so that XXX is not
    // exposed here.
    payload_num_txs: [u8; NUM_TXS_BYTE_LEN], // serialized usize
    payload_proof_num_txs: SmallRangeProofType,

    tx_index: TxIndex,
    payload_tx_table_entry_prev: Option<[u8; TX_OFFSET_BYTE_LEN]>, // serialized usize, `None` for the 0th tx
    payload_tx_table_entry: [u8; TX_OFFSET_BYTE_LEN],              // serialized usize
    payload_proof_tx_table_entries: SmallRangeProofType,
    payload_proof_tx: Option<SmallRangeProofType>, // `None` if the tx has zero length
}

impl Payload {
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        // TODO check index.ns_index in bounds
        // TODO don't copy the tx bytes into the return value
        // https://github.com/EspressoSystems/hotshot-query-service/issues/267
        Some(
            self.ns_payload(&index.ns_index)
                .export_tx(&self.ns_table.read_ns_id(&index.ns_index), &index.tx_index),
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

        // TODO check index.ns_index in bounds
        let ns_payload = self.ns_payload(&index.ns_index);
        let ns_payload_range = self
            .ns_table
            .ns_payload_range(&index.ns_index, self.payload.len());

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let (payload_num_txs, payload_proof_num_txs) = {
            let range_num_txs = ns_payload_range.num_txs_range();

            (
                // TODO make read_num_txs a method (of NsPayload)? Careful not to correct the original bytes!
                // TODO should be safe to read NUM_TXS_BYTE_LEN from payload; we would have exited by now otherwise.
                self.payload.get(range_num_txs.clone())?.try_into().unwrap(), // panic is impossible [TODO after we fix ns iterator])
                vid.payload_proof(&self.payload, range_num_txs).ok()?,
            )
        };

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        // TODO read_tx_offset converts bytes->usize, then we convert back to bytes
        let payload_tx_table_entry_prev = ns_payload
            .read_tx_offset_prev(&index.tx_index)
            .map(tx_offset_as_bytes);
        let payload_tx_table_entry = tx_offset_as_bytes(ns_payload.read_tx_offset(&index.tx_index));
        let payload_proof_tx_table_entries = {
            let range = ns_payload_range.tx_table_entries_range(&index.tx_index);

            // tracing::info!(
            //     "prove: (ns,tx) ({:?},{:?}), tx_table_entries_range {:?}, content {:?}",
            //     index.ns_index,
            //     index.tx_index,
            //     range,
            //     &self.payload[range.clone()]
            // );

            vid.payload_proof(&self.payload, range).ok()?
        };

        // Read the tx payload and compute a proof of correctness.
        let payload_proof_tx = {
            // TODO sucks that I need ns_payload AND ns_payload_range here.
            // should be able to get this with less...
            let range = ns_payload.tx_payload_range(&index.tx_index, &ns_payload_range);

            // TODO IDEA: NsPayload[Owned] should also come with a NsPayloadRange

            tracing::info!(
                "prove: (ns,tx) ({:?},{:?}), tx_payload_range {:?}, content {:?}",
                index.ns_index,
                index.tx_index,
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
                tx_index: index.tx_index.clone(),
                payload_tx_table_entry_prev,
                payload_tx_table_entry,
                payload_proof_tx_table_entries,
                payload_proof_tx,
            },
        ))
    }
}

impl TxProof {
    // - Returns `None` if an error occurred.
    // - `bool` result, or should we use `Result<(),()>` ?
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
            let num_txs_range = self.ns_payload_range.num_txs_range();

            if vid
                .payload_verify(
                    Statement {
                        payload_subslice: &self.payload_num_txs,
                        range: num_txs_range,
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
            let range = self.ns_payload_range.tx_table_entries_range(&self.tx_index);

            // concatenate the two table entry payloads
            let payload_subslice = &{
                let mut bytes = Vec::with_capacity(TX_OFFSET_BYTE_LEN.saturating_mul(2));
                if let Some(prev) = self.payload_tx_table_entry_prev {
                    bytes.extend(prev);
                }
                bytes.extend(self.payload_tx_table_entry);
                bytes
            };

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
            // TODO refactor this range arithmetic to a lower level
            let range = {
                let tx_payloads_start = num_txs_from_bytes(&self.payload_num_txs)
                    .saturating_mul(TX_OFFSET_BYTE_LEN)
                    .saturating_add(NUM_TXS_BYTE_LEN) // tx_table_byte_len plus...
                    .saturating_add(ns_offset_from_bytes(&self.ns_payload_range.0.start)); // ...namespace start

                let end = tx_offset_from_bytes(&self.payload_tx_table_entry)
                    .saturating_add(tx_payloads_start)
                    .min(ns_offset_from_bytes(&self.ns_payload_range.0.end));
                let start = tx_offset_from_bytes(
                    &self
                        .payload_tx_table_entry_prev
                        .unwrap_or([0; TX_OFFSET_BYTE_LEN]),
                )
                .saturating_add(tx_payloads_start)
                .min(end);
                start..end
            };

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
