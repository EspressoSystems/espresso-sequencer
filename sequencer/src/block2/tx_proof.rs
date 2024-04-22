use super::{
    iter::Index,
    payload_bytes::{ns_offset_as_bytes, NS_OFFSET_BYTE_LEN, NUM_TXS_BYTE_LEN},
    Payload,
};
use crate::{block2::payload_bytes::ns_offset_from_bytes, Transaction};
use hotshot_query_service::{VidCommitment, VidCommon};
use hotshot_types::vid::{vid_scheme, SmallRangeProofType, VidSchemeType};
use jf_primitives::vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

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

    payload_num_txs: [u8; NUM_TXS_BYTE_LEN], // serialized usize
    payload_proof_num_txs: SmallRangeProofType,
    // payload_tx_range_start: Option<[u8; TX_OFFSET_BYTE_LEN]>, // serialized usize, `None` for the 0th tx
    // payload_tx_range_end: [u8; TX_OFFSET_BYTE_LEN],           // serialized usize
    // payload_proof_tx_range: SmallRangeProofType,

    // payload_proof_tx: Option<SmallRangeProofType>, // `None` if the tx has zero length
}

impl Payload {
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        // TODO don't copy the tx bytes into the return value
        // https://github.com/EspressoSystems/hotshot-query-service/issues/267
        Some(Transaction::new(
            index.ns_index.ns_id,
            self.payload
                .get(index.ns_index.ns_range.clone())?
                .get(index.tx_index.range.clone())?
                .to_vec(),
        ))
    }

    pub fn transaction_with_proof(
        &self,
        index: &Index,
        common: &VidCommon,
    ) -> Option<(Transaction, TxProof)> {
        if self.payload.len() != VidSchemeType::get_payload_byte_len(common) {
            return None; // error: common inconsistent with self
        }

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

        // payload byte index range for the `num_txs` header in this namespaces
        // tx table
        //
        // TODO we trust index.ns_index.ns_range to be in bounds for
        // self.payload. instead we should recompute it ourselves from ns_table
        // and payload. Thus, we should remove ns_range from the ns iterator
        let num_txs_range = Range {
            start: index.ns_index.ns_range.start,
            end: index
                .ns_index
                .ns_range
                .start
                .saturating_add(NUM_TXS_BYTE_LEN)
                .min(index.ns_index.ns_range.end),
        };

        Some((
            self.transaction(index)?,
            TxProof {
                ns_range_start: ns_offset_as_bytes(index.ns_index.ns_range.start),
                ns_range_end: ns_offset_as_bytes(index.ns_index.ns_range.end),
                payload_num_txs: self.payload.get(num_txs_range.clone())?.try_into().unwrap(), // panic is impossible [TODO after we fix ns iterator]
                payload_proof_num_txs: vid.payload_proof(&self.payload, num_txs_range).ok()?,
                //     payload_tx_range_start,
                //     payload_tx_range_end,
                //     payload_proof_tx_range: vid.payload_proof(&self.payload, tx_table_range).ok()?,
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

        let vid = vid_scheme(VidSchemeType::get_num_storage_nodes(common));

        // BEGIN WIP
        let ns_range =
            ns_offset_from_bytes(&self.ns_range_start)..ns_offset_from_bytes(&self.ns_range_end);
        // let tx_table_byte_len = (); // from num_txs bytes, capped by namespace size, offset by namespace.start

        // // Computing tx payload range from what's in the tx table
        // // TODO refactor and use this everywhere, don't use tx iter range
        // let tx_range = {
        //     let start = tx_offset_from_bytes(
        //         &self
        //             .payload_tx_range_start
        //             .unwrap_or([0; TX_OFFSET_BYTE_LEN]),
        //     )
        //     .saturating_add(ns_range.start);
        // };
        // END WIP

        let num_txs_range = Range {
            start: ns_range.start,
            end: ns_range
                .start
                .saturating_add(NUM_TXS_BYTE_LEN)
                .min(ns_range.end),
        };

        tracing::info!("verify {:?}, {:?}", num_txs_range, self.payload_num_txs);

        // Verify proof for tx table len
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

        Some(true)
    }
}
