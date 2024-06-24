use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::vid::{vid_scheme, SmallRangeProofType};
use jf_vid::payload_prover::{PayloadProver, Statement};
use serde::{Deserialize, Serialize};
use std::ops::Range;

use crate::{Payload, Transaction, TxIndex, TxIterator, TxTable, TxTableEntry, TxTableEntryWord};

// TODO don't hard-code TxTableEntryWord generic param
impl QueryablePayload for Payload<TxTableEntryWord> {
    type TransactionIndex = TxIndex;
    type Iter<'a> = TxIterator<'a, TxTableEntryWord>;
    type InclusionProof = TxInclusionProof;

    fn len(&self, ns_table: &Self::Metadata) -> usize {
        (0..ns_table.len())
            .map(|ns_idx| ns_table.get_payload_range(ns_idx, self.raw_payload.len()).1)
            .map(|ns_range| TxTable::get_tx_table_len(&self.raw_payload[ns_range]))
            .sum()
    }

    fn iter<'a>(&'a self, ns_table: &'a Self::Metadata) -> Self::Iter<'a> {
        TxIterator::new(ns_table, self)
    }

    fn transaction(
        &self,
        meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<Self::Transaction> {
        let (ns_idx, tx_idx) = (index.ns_idx, index.tx_idx);
        if ns_idx >= meta.len() {
            return None; // error: index out of bounds
        }
        let (ns_id, ns_range) = meta.get_payload_range(ns_idx, self.raw_payload.len());

        let tx_table_len = TxTable::get_tx_table_len(&self.raw_payload[ns_range.clone()]);
        if tx_idx >= tx_table_len {
            return None; // error: index out of bounds
        }
        let ns_payload = &self.raw_payload[ns_range.clone()];

        let tx_within_ns = TxTable::get_payload_range(ns_payload, tx_idx, tx_table_len);
        let (start, end) = (tx_within_ns.start, tx_within_ns.end);
        let ns_start = ns_range.start;
        let tx_payload_range = start.saturating_add(ns_start)..end.saturating_add(ns_start);

        let tx_payload = self.raw_payload.get(tx_payload_range)?.to_vec();

        Some(Transaction::new(ns_id, tx_payload))
    }

    fn transaction_with_proof(
        &self,
        meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        let (ns_idx, tx_idx) = (index.ns_idx, index.tx_idx);
        if ns_idx >= meta.len() {
            return None; // error: index out of bounds
        }
        let (ns_id, ns_range) = meta.get_payload_range(ns_idx, self.raw_payload.len());
        let ns_start_offset = ns_range.start;

        let tx_table_len = TxTable::get_tx_table_len(&self.raw_payload[ns_range.clone()]);
        if tx_idx >= tx_table_len {
            return None; // error: index out of bounds
        }

        let tx_payloads_offset = tx_table_len
            .checked_add(1)?
            .checked_mul(TxTableEntry::byte_len())?
            .checked_add(ns_start_offset)?;

        // TODO temporary VID construction. We need to get the number of storage nodes from the VID
        // common data. May need the query service to pass common into this function along with
        // metadata.
        let vid = vid_scheme(10);

        // Read the tx payload range from the tx table into `tx_table_range_[start|end]` and compute a proof that this range is correct.
        //
        // This correctness proof requires a range of its own, which we read into `tx_table_range_proof_[start|end]`.
        //
        // Edge case--the first transaction: tx payload range `start` is implicitly 0 and we do not include this item in the correctness proof.
        //
        // TODO why isn't cargo fmt wrapping these comments?

        // start
        let (tx_table_range_proof_start, tx_table_range_start) = if tx_idx == 0 {
            (TxTableEntry::byte_len().checked_add(ns_start_offset)?, None)
        } else {
            let range_proof_start = tx_idx
                .checked_mul(TxTableEntry::byte_len())?
                .checked_add(ns_start_offset)?;
            (
                range_proof_start,
                Some(TxTableEntry::from_bytes(self.raw_payload.get(
                    range_proof_start..range_proof_start.checked_add(TxTableEntry::byte_len())?,
                )?)?),
            )
        };

        // end
        let tx_table_range_proof_end = tx_idx
            .checked_add(2)?
            .checked_mul(TxTableEntry::byte_len())?
            .checked_add(ns_start_offset)?;

        let tx_table_range_end = TxTableEntry::from_bytes(self.raw_payload.get(
            tx_table_range_proof_end.checked_sub(TxTableEntry::byte_len())?
                ..tx_table_range_proof_end,
        )?)?;

        let tx_payload_range = {
            let start =
                usize::try_from(tx_table_range_start.clone().unwrap_or(TxTableEntry::zero()))
                    .ok()?
                    .checked_add(tx_payloads_offset)?;
            let end = usize::try_from(tx_table_range_end.clone())
                .ok()?
                .checked_add(tx_payloads_offset)?;
            let end = std::cmp::min(end, ns_range.end);
            let start = std::cmp::min(start, end);
            start..end
        };

        // correctness proof for the tx payload range
        let tx_table_range_proof = vid
            .payload_proof(
                &self.raw_payload,
                tx_table_range_proof_start..tx_table_range_proof_end,
            )
            .ok()?;
        let tx_table_len_range = ns_range.start
            ..std::cmp::min(
                ns_range.end,
                ns_range.start.checked_add(TxTableEntry::byte_len())?,
            );
        Some((
            // TODO don't copy the tx bytes into the return value
            // https://github.com/EspressoSystems/hotshot-query-service/issues/267
            Transaction::new(
                ns_id,
                self.raw_payload.get(tx_payload_range.clone())?.to_vec(),
            ),
            TxInclusionProof {
                ns_range: ns_range.clone(),
                tx_table_len: TxTableEntry::from_usize(tx_table_len),
                tx_table_len_proof: vid
                    .payload_proof(&self.raw_payload, tx_table_len_range)
                    .unwrap(),
                tx_table_range_start,
                tx_table_range_end,
                tx_table_range_proof,
                tx_payload_proof: if tx_payload_range.is_empty() {
                    None
                } else {
                    vid.payload_proof(&self.raw_payload, tx_payload_range).ok()
                },
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxInclusionProof {
    ns_range: Range<usize>,
    tx_table_len: TxTableEntry,
    tx_table_len_proof: SmallRangeProofType,

    tx_table_range_start: Option<TxTableEntry>, // `None` for the 0th tx
    tx_table_range_end: TxTableEntry,
    tx_table_range_proof: SmallRangeProofType,

    tx_payload_proof: Option<SmallRangeProofType>, // `None` if the tx has zero length
}

impl TxInclusionProof {
    // TODO currently broken, fix in https://github.com/EspressoSystems/espresso-sequencer/issues/1010
    //
    // - We need to decide where to store VID params.
    // - Returns `None` if an error occurred.
    // - Use of `Result<(),()>` pattern to enable use of `?` for concise abort-on-failure.
    #[allow(dead_code)] // TODO temporary
    #[allow(clippy::too_many_arguments)]
    pub fn verify<V>(
        &self,
        tx: &Transaction,
        tx_index: TxIndex,
        vid: &V,
        vid_commit: &V::Commit,
        vid_common: &V::Common,
    ) -> Option<Result<(), ()>>
    where
        V: PayloadProver<SmallRangeProofType>,
    {
        V::is_consistent(vid_commit, vid_common).ok()?;

        // Verify proof for tx payload.
        // Proof is `None` if and only if tx has zero length.
        let tx_payloads_offset = usize::try_from(self.tx_table_len.clone())
            .ok()?
            .checked_add(1)?
            .checked_mul(TxTableEntry::byte_len())?
            .checked_add(self.ns_range.start)?;
        let tx_payload_range = {
            let start = usize::try_from(
                self.tx_table_range_start
                    .clone()
                    .unwrap_or(TxTableEntry::zero()),
            )
            .ok()?
            .checked_add(tx_payloads_offset)?;
            let end = usize::try_from(self.tx_table_range_end.clone())
                .ok()?
                .checked_add(tx_payloads_offset)?;
            let end = std::cmp::min(end, self.ns_range.end);
            let start = std::cmp::min(start, end);
            start..end
        };
        match &self.tx_payload_proof {
            Some(tx_payload_proof) => {
                if vid
                    .payload_verify(
                        Statement {
                            payload_subslice: tx.payload(),
                            range: tx_payload_range,
                            commit: vid_commit,
                            common: vid_common,
                        },
                        tx_payload_proof,
                    )
                    .ok()?
                    .is_err()
                {
                    return Some(Err(())); // TODO it would be nice to use ? here...
                }
            }
            None => {
                if !tx.payload().is_empty() || !tx_payload_range.is_empty() {
                    return None; // error: nonempty payload but no proof
                }
            }
        };

        // Verify proof for tx table len.
        if vid
            .payload_verify(
                Statement {
                    payload_subslice: &self.tx_table_len.to_bytes(),
                    range: self.ns_range.start
                        ..self.ns_range.start.checked_add(TxTableEntry::byte_len())?,
                    commit: vid_commit,
                    common: vid_common,
                },
                &self.tx_table_len_proof,
            )
            .ok()?
            .is_err()
        {
            return Some(Err(()));
        }

        // Verify proof for tx table entries.
        // Start index missing for the 0th tx
        let index: usize = tx_index.tx_idx;
        let mut tx_table_range_bytes =
            Vec::with_capacity(2usize.checked_mul(TxTableEntry::byte_len())?);
        let start = if let Some(tx_table_range_start) = &self.tx_table_range_start {
            if index == 0 {
                return None; // error: first tx should have empty start index
            }
            tx_table_range_bytes.extend(tx_table_range_start.to_bytes());
            index
                .checked_mul(TxTableEntry::byte_len())?
                .checked_add(self.ns_range.start)?
        } else {
            if index != 0 {
                return None; // error: only the first tx should have empty start index
            }
            TxTableEntry::byte_len().checked_add(self.ns_range.start)?
        };
        tx_table_range_bytes.extend(self.tx_table_range_end.to_bytes());
        let range = start
            ..index
                .checked_add(2)?
                .checked_mul(TxTableEntry::byte_len())?
                .checked_add(self.ns_range.start)?;

        if vid
            .payload_verify(
                Statement {
                    payload_subslice: &tx_table_range_bytes,
                    range,
                    commit: vid_commit,
                    common: vid_common,
                },
                &self.tx_table_range_proof,
            )
            .ok()?
            .is_err()
        {
            return Some(Err(()));
        }

        Some(Ok(()))
    }
}

#[cfg(test)]
pub(crate) fn gen_tx_proof_for_testing(
    ns_range: Range<usize>,
    tx_table_len: TxTableEntry,
    tx_table_len_proof: SmallRangeProofType,
    payload_proof: SmallRangeProofType,
) -> TxInclusionProof {
    TxInclusionProof {
        ns_range,
        tx_table_len,
        tx_table_len_proof,
        tx_table_range_start: None,
        tx_table_range_end: TxTableEntry::from_usize(1),
        tx_table_range_proof: payload_proof,
        tx_payload_proof: None,
    }
}
