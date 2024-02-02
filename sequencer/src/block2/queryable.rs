use crate::block2::payload::{test_vid_factory, NameSpaceTable, Payload, RangeProof};
use crate::block2::tables::{Table, TxTable};
use hotshot_query_service::availability::QueryablePayload;
use jf_primitives::vid::payload_prover::{PayloadProver, Statement};
use serde::{Deserialize, Serialize};
use std::ops::Range;

use crate::Transaction;

use super::{
    entry::TxTableEntry,
    tx_iterator::{TxIndex, TxIterator},
};

impl QueryablePayload for Payload<u32> {
    type TransactionIndex = TxIndex;
    type Iter<'a> = TxIterator<'a, u32>;
    type InclusionProof = TxInclusionProof;

    fn len(&self, meta: &Self::Metadata) -> usize {
        let entry_len = TxTableEntry::byte_len();

        // The number of nss in a block is defined as the minimum of:
        // (1) the number of nss indicated in the ns table
        // (2) the number of ns table entries that could fit inside the ns table byte len
        // Why? Because (1) could be anything. A block should not be allowed to contain 4 billion 0-length nss.
        // The quantity (2) must exclude the prefix of the ns table because this prifix indicates only the length of the ns table, not an actual ns.
        let ns_table = NameSpaceTable::<u32>::from_bytes(meta);
        let ns_table_len = ns_table.len();

        // First, collect the offsets of all the nss
        // (Range starts at 1 to conveniently skip the ns table prefix.)
        let mut ns_end_offsets = vec![0usize];
        for i in 1..=ns_table_len {
            let ns_offset_bytes = meta
                .get(((2 * i) * entry_len)..((2 * i + 1) * entry_len))
                .unwrap();

            let ns_offset = TxTableEntry::from_bytes(ns_offset_bytes)
                .map(|tx| usize::try_from(tx).unwrap())
                .unwrap();
            ns_end_offsets.push(ns_offset);
        }

        // for each entry in the ns table:
        // read the tx table len for that ns
        // that tx table len is the number of txs in that namespace
        // sum over these tx table lens
        let mut result = 0;
        for &offset in ns_end_offsets.iter().take(ns_end_offsets.len() - 1) {
            let tx_table = TxTable::<u32>::from_bytes(&self.raw_payload);
            let tx_table_len = tx_table.get_table_len(offset).try_into().unwrap_or(0);
            // TODO handle large tx_table_len! (https://github.com/EspressoSystems/espresso-sequencer/issues/785)
            result += tx_table_len;
        }
        result
    }

    fn iter<'a>(&'a self, meta: &'a Self::Metadata) -> Self::Iter<'a> {
        let ns_table = NameSpaceTable::from_bytes(meta);
        TxIterator::new(ns_table.clone(), self)
    }

    // TODO currently broken, fix in https://github.com/EspressoSystems/espresso-sequencer/issues/1010
    fn transaction_with_proof(
        &self,
        meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        let index_usize = index.tx_idx; // TODO fix in https://github.com/EspressoSystems/espresso-sequencer/issues/1010
        if index_usize >= self.len(meta) {
            return None; // error: index out of bounds
        }

        let vid = test_vid_factory(); // TODO temporary VID construction

        // Read the tx payload range from the tx table into `tx_table_range_[start|end]` and compute a proof that this range is correct.
        //
        // This correctness proof requires a range of its own, which we read into `tx_table_range_proof_[start|end]`.
        //
        // Edge case--the first transaction: tx payload range `start` is implicitly 0 and we do not include this item in the correctness proof.
        //
        // TODO why isn't cargo fmt wrapping these comments?

        // start
        let (tx_table_range_proof_start, tx_table_range_start) = if index_usize == 0 {
            (TxTableEntry::byte_len(), None)
        } else {
            let range_proof_start = index_usize.checked_mul(TxTableEntry::byte_len())?;
            (
                range_proof_start,
                Some(TxTableEntry::from_bytes(self.raw_payload.get(
                    range_proof_start..range_proof_start.checked_add(TxTableEntry::byte_len())?,
                )?)?),
            )
        };

        // end
        let tx_table_range_proof_end = index_usize
            .checked_add(2)?
            .checked_mul(TxTableEntry::byte_len())?;
        let tx_table_range_end = TxTableEntry::from_bytes(self.raw_payload.get(
            tx_table_range_proof_end.checked_sub(TxTableEntry::byte_len())?
                ..tx_table_range_proof_end,
        )?)?;

        // correctness proof for the tx payload range
        let tx_table_range_proof = vid
            .payload_proof(
                &self.raw_payload,
                tx_table_range_proof_start..tx_table_range_proof_end,
            )
            .ok()?;

        let tx_payload_range = tx_payload_range(
            &tx_table_range_start,
            &tx_table_range_end,
            &self.get_tx_table_len(),
            self.raw_payload.len(),
        )?;
        Some((
            // TODO don't copy the tx bytes into the return value
            // https://github.com/EspressoSystems/hotshot-query-service/issues/267
            Transaction::new(
                crate::VmId(0),
                self.raw_payload.get(tx_payload_range.clone())?.to_vec(),
            ),
            TxInclusionProof {
                tx_table_len: self.get_tx_table_len(),
                tx_table_len_proof: self.get_tx_table_len_proof(&vid)?.clone(),
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

/// Returns the byte range for a tx in the block payload bytes.
///
/// Ensures that the returned range is valid (start <= end) and within bounds for `block_payload_byte_len`.
/// Lots of ugly type conversion and checked arithmetic.
fn tx_payload_range(
    tx_table_range_start: &Option<TxTableEntry>,
    tx_table_range_end: &TxTableEntry,
    tx_table_len: &TxTableEntry,
    block_payload_byte_len: usize,
) -> Option<Range<usize>> {
    // TODO(817) allow arbitrary tx_table_len
    // eg: if overflow then just return a 0-length tx
    let tx_bodies_offset = usize::try_from(tx_table_len.clone())
        .ok()?
        .checked_add(1)?
        .checked_mul(TxTableEntry::byte_len())?;
    let start = usize::try_from(tx_table_range_start.clone().unwrap_or(TxTableEntry::zero()))
        .ok()?
        .checked_add(tx_bodies_offset)?;
    let end = usize::try_from(tx_table_range_end.clone())
        .ok()?
        .checked_add(tx_bodies_offset)?;
    let end = std::cmp::max(start, end);
    let start = std::cmp::min(start, block_payload_byte_len);
    let end = std::cmp::min(end, block_payload_byte_len);
    Some(start..end)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxInclusionProof {
    tx_table_len: TxTableEntry,
    tx_table_len_proof: RangeProof,

    tx_table_range_start: Option<TxTableEntry>, // `None` for the 0th tx
    tx_table_range_end: TxTableEntry,
    tx_table_range_proof: RangeProof,

    tx_payload_proof: Option<RangeProof>, // `None` if the tx has zero length
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
        V: PayloadProver<RangeProof>,
    {
        V::is_consistent(vid_commit, vid_common).ok()?;

        // Verify proof for tx payload.
        // Proof is `None` if and only if tx has zero length.
        let tx_payload_range = tx_payload_range(
            &self.tx_table_range_start,
            &self.tx_table_range_end,
            &self.tx_table_len,
            V::get_payload_byte_len(vid_common),
        )?;
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
                    range: 0..TxTableEntry::byte_len(),
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
        let index: usize = tx_index.tx_idx; // TODO fix in https://github.com/EspressoSystems/espresso-sequencer/issues/1010
        let mut tx_table_range_bytes =
            Vec::with_capacity(2usize.checked_mul(TxTableEntry::byte_len())?);
        let start = if let Some(tx_table_range_start) = &self.tx_table_range_start {
            if index == 0 {
                return None; // error: first tx should have empty start index
            }
            tx_table_range_bytes.extend(tx_table_range_start.to_bytes());
            index * TxTableEntry::byte_len()
        } else {
            if index != 0 {
                return None; // error: only the first tx should have empty start index
            }
            TxTableEntry::byte_len()
        };
        tx_table_range_bytes.extend(self.tx_table_range_end.to_bytes());
        let range = start
            ..index
                .checked_add(2)?
                .checked_mul(TxTableEntry::byte_len())?;

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
    tx_table_len: TxTableEntry,
    tx_table_len_proof: RangeProof,
    payload_proof: RangeProof,
) -> TxInclusionProof {
    TxInclusionProof {
        tx_table_len,
        tx_table_len_proof,
        tx_table_range_start: None,
        tx_table_range_end: TxTableEntry::from_usize(1),
        tx_table_range_proof: payload_proof,
        tx_payload_proof: None,
    }
}
