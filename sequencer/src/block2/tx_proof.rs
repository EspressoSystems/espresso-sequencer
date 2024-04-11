use super::{iter::Index, Payload};
use crate::Transaction;
use serde::{Deserialize, Serialize};
// use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxProof {}

impl Payload {
    pub fn transaction(&self, _index: &Index) -> Option<Transaction> {
        todo!()
    }

    pub fn transaction_with_proof(&self, _index: &Index) -> Option<(Transaction, TxProof)> {
        todo!()
    }
}

// #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
// pub struct TxInclusionProof {
//     ns_range: Range<usize>,
//     tx_table_len: TxTableEntry,
//     tx_table_len_proof: SmallRangeProofType,

//     tx_table_range_start: Option<TxTableEntry>, // `None` for the 0th tx
//     tx_table_range_end: TxTableEntry,
//     tx_table_range_proof: SmallRangeProofType,

//     tx_payload_proof: Option<SmallRangeProofType>, // `None` if the tx has zero length
// }

// impl TxInclusionProof {
//     // TODO currently broken, fix in https://github.com/EspressoSystems/espresso-sequencer/issues/1010
//     //
//     // - We need to decide where to store VID params.
//     // - Returns `None` if an error occurred.
//     // - Use of `Result<(),()>` pattern to enable use of `?` for concise abort-on-failure.
//     #[allow(dead_code)] // TODO temporary
//     #[allow(clippy::too_many_arguments)]
//     pub fn verify<V>(
//         &self,
//         tx: &Transaction,
//         tx_index: TxIndex,
//         vid: &V,
//         vid_commit: &V::Commit,
//         vid_common: &V::Common,
//     ) -> Option<Result<(), ()>>
//     where
//         V: PayloadProver<SmallRangeProofType>,
//     {
//         V::is_consistent(vid_commit, vid_common).ok()?;

//         // Verify proof for tx payload.
//         // Proof is `None` if and only if tx has zero length.
//         let tx_payloads_offset = usize::try_from(self.tx_table_len.clone())
//             .ok()?
//             .checked_add(1)?
//             .checked_mul(TxTableEntry::byte_len())?
//             .checked_add(self.ns_range.start)?;
//         let tx_payload_range = {
//             let start = usize::try_from(
//                 self.tx_table_range_start
//                     .clone()
//                     .unwrap_or(TxTableEntry::zero()),
//             )
//             .ok()?
//             .checked_add(tx_payloads_offset)?;
//             let end = usize::try_from(self.tx_table_range_end.clone())
//                 .ok()?
//                 .checked_add(tx_payloads_offset)?;
//             let end = std::cmp::min(end, self.ns_range.end);
//             let start = std::cmp::min(start, end);
//             start..end
//         };
//         match &self.tx_payload_proof {
//             Some(tx_payload_proof) => {
//                 if vid
//                     .payload_verify(
//                         Statement {
//                             payload_subslice: tx.payload(),
//                             range: tx_payload_range,
//                             commit: vid_commit,
//                             common: vid_common,
//                         },
//                         tx_payload_proof,
//                     )
//                     .ok()?
//                     .is_err()
//                 {
//                     return Some(Err(())); // TODO it would be nice to use ? here...
//                 }
//             }
//             None => {
//                 if !tx.payload().is_empty() || !tx_payload_range.is_empty() {
//                     return None; // error: nonempty payload but no proof
//                 }
//             }
//         };

//         // Verify proof for tx table len.
//         if vid
//             .payload_verify(
//                 Statement {
//                     payload_subslice: &self.tx_table_len.to_bytes(),
//                     range: self.ns_range.start
//                         ..self.ns_range.start.checked_add(TxTableEntry::byte_len())?,
//                     commit: vid_commit,
//                     common: vid_common,
//                 },
//                 &self.tx_table_len_proof,
//             )
//             .ok()?
//             .is_err()
//         {
//             return Some(Err(()));
//         }

//         // Verify proof for tx table entries.
//         // Start index missing for the 0th tx
//         let index: usize = tx_index.tx_idx;
//         let mut tx_table_range_bytes =
//             Vec::with_capacity(2usize.checked_mul(TxTableEntry::byte_len())?);
//         let start = if let Some(tx_table_range_start) = &self.tx_table_range_start {
//             if index == 0 {
//                 return None; // error: first tx should have empty start index
//             }
//             tx_table_range_bytes.extend(tx_table_range_start.to_bytes());
//             index
//                 .checked_mul(TxTableEntry::byte_len())?
//                 .checked_add(self.ns_range.start)?
//         } else {
//             if index != 0 {
//                 return None; // error: only the first tx should have empty start index
//             }
//             TxTableEntry::byte_len().checked_add(self.ns_range.start)?
//         };
//         tx_table_range_bytes.extend(self.tx_table_range_end.to_bytes());
//         let range = start
//             ..index
//                 .checked_add(2)?
//                 .checked_mul(TxTableEntry::byte_len())?
//                 .checked_add(self.ns_range.start)?;

//         if vid
//             .payload_verify(
//                 Statement {
//                     payload_subslice: &tx_table_range_bytes,
//                     range,
//                     commit: vid_commit,
//                     common: vid_common,
//                 },
//                 &self.tx_table_range_proof,
//             )
//             .ok()?
//             .is_err()
//         {
//             return Some(Err(()));
//         }

//         Some(Ok(()))
//     }
// }
