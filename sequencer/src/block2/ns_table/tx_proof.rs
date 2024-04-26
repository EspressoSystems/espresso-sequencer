use crate::{
    block2::{
        iter::Index,
        payload_bytes::{
            ns_offset_as_bytes, ns_offset_from_bytes, tx_offset_as_bytes, NS_OFFSET_BYTE_LEN,
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
use std::ops::Range;

use super::ns_payload::tx_iter::TxIndex;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxProof {
    // Conventions:
    // - `payload_x`: bytes from the payload
    // - `payload_proof_x`: a proof of those bytes from the payload

    // TODO can we trust ns_range claims? Or do we need to take the ns table as
    // a separate arg, and replace ns_range_x here with ns_index into the ns
    // table. I think we can trust them because payload proofs are tied to a
    // specific location
    ns_range_start: [u8; NS_OFFSET_BYTE_LEN], // serialized usize
    ns_range_end: [u8; NS_OFFSET_BYTE_LEN],   // serialized usize

    // TODO make [u8; XXX] a newtype at a lower level of code so that XXX is not
    // exposed here.
    payload_num_txs: [u8; NUM_TXS_BYTE_LEN], // serialized usize
    payload_proof_num_txs: SmallRangeProofType,

    tx_index: TxIndex,
    payload_tx_table_entry_prev: Option<[u8; TX_OFFSET_BYTE_LEN]>, // serialized usize, `None` for the 0th tx
    payload_tx_table_entry: [u8; TX_OFFSET_BYTE_LEN],              // serialized usize
    payload_proof_tx_table_entries: SmallRangeProofType,
    // payload_proof_tx: Option<SmallRangeProofType>, // `None` if the tx has zero length
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

        // BEGIN WIP
        // range of contiguous bytes in this namespace's tx table
        // TODO refactor as a function of `index`?
        // let tx_table_range = {
        //     let start = if index.tx_index.index == 0 {
        //         // Special case: the desired range includes only one entry from
        //         // the tx table: the first entry. This entry starts immediately
        //         // following the bytes that encode the tx table length.
        //         NUM_NSS_BYTE_LEN
        //     } else {
        //         // the desired range starts at the beginning of the previous
        //         // transaction's tx table entry
        //         (index.tx_index.index - 1)
        //             .saturating_mul(TX_OFFSET_BYTE_LEN)
        //             .saturating_add(NUM_TXS_BYTE_LEN)
        //     };
        //     // The desired range ends at the end of this transaction's tx table
        //     // entry
        //     let end = index
        //         .tx_index
        //         .index
        //         .saturating_add(1)
        //         .saturating_mul(TX_OFFSET_BYTE_LEN)
        //         .saturating_add(NUM_TXS_BYTE_LEN);
        //     Range {
        //         start: start.saturating_add(index.ns_index.ns_range.start),
        //         end: end.saturating_add(index.ns_index.ns_range.start),
        //     }
        // };

        // let payload_tx_range_start: Option<[u8; TX_OFFSET_BYTE_LEN]> = if index.tx_index.index == 0
        // {
        //     None
        // } else {
        //     Some(
        //         self.payload
        //             .get(
        //                 tx_table_range.start
        //                     ..tx_table_range.start.saturating_add(TX_OFFSET_BYTE_LEN),
        //             )?
        //             .try_into()
        //             .unwrap(), // panic is impossible
        //     )
        // };

        // let payload_tx_range_end: [u8; TX_OFFSET_BYTE_LEN] = self
        //     .payload
        //     .get(tx_table_range.end.saturating_sub(TX_OFFSET_BYTE_LEN)..tx_table_range.end)?
        //     .try_into()
        //     .unwrap(); // panic is impossible

        // let tx_range = Range {
        //     start: index
        //         .tx_index
        //         .range
        //         .start
        //         .saturating_add(index.ns_index.ns_range.start),
        //     end: index
        //         .tx_index
        //         .range
        //         .end
        //         .saturating_add(index.ns_index.ns_range.start),
        // };
        // END WIP

        // Read the tx table len from this namespace's tx table and compute a
        // proof of correctness.
        let (payload_num_txs, payload_proof_num_txs) = {
            // TODO make range_num_txs a method (of NsPayload)?
            let range_num_txs = Range {
                start: ns_payload_range.start,
                end: ns_payload_range
                    .start
                    .saturating_add(NUM_TXS_BYTE_LEN)
                    .min(ns_payload_range.end),
            };
            (
                // TODO make read_num_txs a method (of NsPayload)? Careful not to correct the original bytes!
                // TODO should be safe to read NUM_TXS_BYTE_LEN from payload; we would have exited by now otherwise.
                self.payload.get(range_num_txs.clone())?.try_into().unwrap(), // panic is impossible [TODO after we fix ns iterator])
                vid.payload_proof(&self.payload, range_num_txs).ok()?,
            )
        };

        // Read the tx table entries for this tx and compute a proof of
        // correctness.
        let payload_tx_table_entry_prev = ns_payload
            .read_tx_offset_prev(&index.tx_index)
            .map(tx_offset_as_bytes);
        let payload_tx_table_entry = tx_offset_as_bytes(ns_payload.read_tx_offset(&index.tx_index));
        let payload_proof_tx_table_entries = {
            let range = index.tx_index.tx_table_entries_range();

            // TODO add a method Payload::tx_table_entries_range(index: Index)
            // that automatically translates TxIndex::tx_table_entries_range by
            // the namespace offset?
            let range = range.start.saturating_add(ns_payload_range.start)
                ..range.end.saturating_add(ns_payload_range.start);

            tracing::info!(
                "prove: tx_table_entries_range {:?}, content {:?}",
                range,
                &self.payload[range.clone()]
            );

            vid.payload_proof(&self.payload, range).ok()?
        };

        Some((
            self.transaction(index)?,
            TxProof {
                ns_range_start: ns_offset_as_bytes(ns_payload_range.start),
                ns_range_end: ns_offset_as_bytes(ns_payload_range.end),
                payload_num_txs,
                payload_proof_num_txs,
                tx_index: index.tx_index.clone(),
                payload_tx_table_entry_prev,
                payload_tx_table_entry,
                payload_proof_tx_table_entries,
                //     payload_proof_tx: if tx_range.is_empty() {
                //         None
                //     } else {
                //         Some(vid.payload_proof(&self.payload, tx_range).ok()?)
                //     },
            },
        ))
    }
}

impl TxProof {
    // - Returns `None` if an error occurred.
    // - `bool` result, or should we use `Result<(),()>` ?
    pub fn verify(
        &self,
        _tx: &Transaction,
        commit: &VidCommitment,
        common: &VidCommon,
    ) -> Option<bool> {
        VidSchemeType::is_consistent(commit, common).ok()?;

        // TODO need a way to check self.tx_index < self.num_txs. That's a pain
        // because tx_index is an opaque newtype. Solution: make a separate
        // newtype T for num_txs and make a method T::is_valid(index: TxIndex)
        // to check whether index is in bounds.

        // TODO check ns_range: start <= end <= payload byte len
        let ns_payload_range =
            ns_offset_from_bytes(&self.ns_range_start)..ns_offset_from_bytes(&self.ns_range_end);

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // Verify proof for tx table len
        {
            let num_txs_range = Range {
                start: ns_payload_range.start,
                end: ns_payload_range
                    .start
                    .saturating_add(NUM_TXS_BYTE_LEN)
                    .min(ns_payload_range.end),
            };

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
            let range = {
                let range = self.tx_index.tx_table_entries_range();

                // TODO newtype NsPayloadRange wrapping
                // self.ns_range_start..self.ns_range_end with a method
                // tx_table_entries_range() that automatically translates the
                // range by namespace offset? Such a method could be used to
                // impl the corresponding new method
                // `Payload::tx_table_entries_range` proposed for use in
                // transaction_with_proof.
                range.start.saturating_add(ns_payload_range.start)
                    ..range.end.saturating_add(ns_payload_range.start)
            };

            // concatenate the two table entry payloads
            let payload_subslice = &{
                let mut bytes = Vec::with_capacity(TX_OFFSET_BYTE_LEN.saturating_mul(2));
                if let Some(prev) = self.payload_tx_table_entry_prev {
                    bytes.extend(prev);
                }
                bytes.extend(self.payload_tx_table_entry);
                bytes
            };

            tracing::info!(
                "verify: tx_table_entries_range {:?}, content {:?}",
                range,
                payload_subslice
            );

            if vid
                .payload_verify(
                    Statement {
                        payload_subslice,
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

        // Verify proof for tx payload
        {
            // TODO refactor this range arithmetic to a lower level
            // let tx_range = {
            //     let tx_payloads_start = num_txs_from_bytes(&self.payload_num_txs)
            //         .saturating_mul(TX_OFFSET_BYTE_LEN)
            //         .saturating_add(NUM_TXS_BYTE_LEN) // tx_table_byte_len plus...
            //         .saturating_add(ns_range.start); // ...namespace start

            //     let end = tx_offset_from_bytes(&self.payload_tx_table_entry)
            //         .saturating_add(tx_payloads_start)
            //         .min(ns_range.end);
            //     let start = tx_offset_from_bytes(
            //         &self
            //             .payload_tx_table_entry_prev
            //             .unwrap_or([0; TX_OFFSET_BYTE_LEN]),
            //     )
            //     .saturating_add(tx_payloads_start)
            //     .min(end);
            //     start..end
            // };
            // tracing::info!("verify: tx_range {:?}", tx_range);
        }

        Some(true)
    }
}
