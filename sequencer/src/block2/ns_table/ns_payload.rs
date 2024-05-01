// use serde::{Deserialize, Serialize};

use super::NsIndex;
use crate::block2::{
    payload_bytes::{
        num_txs_as_bytes, num_txs_from_bytes, tx_offset_as_bytes, tx_offset_from_bytes,
        NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
    },
    Payload,
};
use crate::{NamespaceId, Transaction};
use serde::{Deserialize, Serialize};
use std::ops::Range;

use tx_iter::{TxIndex, TxIter};

// TODO move all the modules from inside ns_table back up to block2?
// TODO move this to ns_table.rs so we can construct a `Payload` there and keep `NsTable` fields private?
#[derive(Default)]
pub struct NamespacePayloadBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NamespacePayloadBuilder {
    /// Add a transaction's payload to this namespace
    pub fn append_tx(&mut self, tx: Transaction) {
        self.tx_bodies.extend(tx.into_payload());
        self.tx_table_entries
            .extend(tx_offset_as_bytes(self.tx_bodies.len()));
    }

    /// Serialize to bytes and consume self.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            NUM_TXS_BYTE_LEN + self.tx_table_entries.len() + self.tx_bodies.len(),
        );
        let num_txs = self.tx_table_entries.len() / TX_OFFSET_BYTE_LEN;
        result.extend(num_txs_as_bytes(num_txs));
        result.extend(self.tx_table_entries);
        result.extend(self.tx_bodies);
        result
    }
}

/// TODO explain: [`NsPayloadOwned`] to [`NsPayload`] as [`Vec<T>`] is to `[T]`.
/// TODO store `ns_id` in [`NsPayload`] and [`NsPayloadOwned`]? TODO we'd like
/// `NsPayload` to be of the form
/// ```
/// pub struct NsPayload {
///     range: NsPayloadRange,
///     data: [u8],
/// }
/// ```
/// But it seems impossible to construct a struct with a DST field unless that
/// struct is a newtype wrapper for a single DST. (See comments below.) This is
/// really only needed if you want to impl `Borrow` for use in things like
/// hashmaps. I guess we don't really benefit much from `Borrow`, so it's
/// probably ok to just impl `Deref` and call it a day.
///
/// IMPORTANT LINKS:
/// - We can do this if we don't have the `range` header: [How can I create newtypes for an unsized type and its owned counterpart (like `str` and `String`) in safe Rust? - Stack Overflow](https://stackoverflow.com/questions/64977525/how-can-i-create-newtypes-for-an-unsized-type-and-its-owned-counterpart-like-s)
/// - We can do this via `slice-dst` crate but we need to put it into a `Box` (or `Rc` or `Arc`): [slice_dst - Rust](https://docs.rs/slice-dst/latest/slice_dst/)
///
// #[repr(transparent)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Debug)]
pub struct NsPayload<'a> {
    range: NsPayloadRange,
    data: &'a [u8],
}

// #[repr(transparent)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
// #[serde(transparent)]
pub struct NsPayloadOwned {
    range: NsPayloadRange,
    data: Vec<u8>,
}

impl NsPayloadOwned {
    // TODO `as_unowned` needed only because Rust does not allow custom DSTs.
    pub fn as_unowned(&self) -> NsPayload<'_> {
        NsPayload {
            range: self.range.clone(),
            data: &self.data,
        }
    }
}

impl NsPayload<'_> {
    // TODO `to_owned` needed only because Rust does not allow custom DSTs.
    pub fn to_owned(&self) -> NsPayloadOwned {
        NsPayloadOwned {
            range: self.range.clone(),
            data: self.data.to_owned(),
        }
    }

    /// Number of bytes used to encode the number of txs in the tx table.
    ///
    /// Returns the minimum of [`NUM_TXS_BYTE_LEN`] and the byte length of the
    /// entire namespace payload.
    ///
    /// In all nontrivial cases this quantity is [`NUM_TXS_BYTE_LEN`]. Anything
    /// else is a degenerate case.
    pub fn num_txs_byte_len(&self) -> usize {
        NUM_TXS_BYTE_LEN.min(self.data.len())
    }

    /// Number of entries in this namespace's tx table.
    ///
    /// Returns the minimum of:
    /// - The declared number of txs from the tx table.
    /// - The maximum number of tx table entries that could fit into the
    ///   namespace payload.
    pub fn num_txs(&self) -> usize {
        let num_txs_byte_len = self.num_txs_byte_len();
        std::cmp::min(
            // Read the declared number of txs from the tx table
            num_txs_from_bytes(&self.data[..num_txs_byte_len]),
            // Max number of entries that could fit in the namespace payload
            self.data.len().saturating_sub(num_txs_byte_len) / TX_OFFSET_BYTE_LEN,
        )
    }

    /// Byte length of this namespace's tx table.
    ///
    /// Guaranteed to be no larger than this namespace's payload byte length.
    pub fn tx_table_byte_len(&self) -> usize {
        self.num_txs()
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN)
            .min(self.data.len())
        // TODO FIX: NsPayload needs a NsPayloadRange field
        // or make it like tx_payload_range()
        // NsPayloadRange::tx_table_byte_len(&self, self.num_txs())
    }

    /// Read subslice range for the `index`th tx from the tx
    /// table, relative to the beginning of this namespace's payload.
    ///
    /// Returned range guaranteed to satisfy `start <= end <= namespace_byte_len`.
    ///
    /// Panics if `index >= self.num_txs()`.
    fn tx_payload_range_relative(&self, index: &TxIndex) -> Range<usize> {
        let tx_table_byte_len = self.tx_table_byte_len();
        let end = self
            .read_tx_offset(index)
            .saturating_add(tx_table_byte_len)
            .min(self.data.len());
        let start = self
            .read_tx_offset_prev(index)
            .unwrap_or(0)
            .saturating_add(tx_table_byte_len)
            .min(end);
        // tracing::info!("tx_payload_range {:?}", start..end);
        start..end
    }

    /// Access the bytes of this [`NsPayload`].
    pub fn as_byte_slice(&self) -> &[u8] {
        self.data
    }

    /// TODO
    pub fn as_range(&self) -> Range<usize> {
        self.range.as_range()
    }

    /// TODO store `ns_id` in `NsPayload` struct?
    pub fn export_all_txs(&self, ns_id: &NamespaceId) -> Vec<Transaction> {
        TxIter::new(self)
            .map(|i| {
                Transaction::new(
                    *ns_id,
                    self.data[self.tx_payload_range_relative(&i)].to_vec(),
                )
            })
            .collect()
    }
    pub fn export_tx(&self, ns_id: &NamespaceId, index: &TxIndex) -> Transaction {
        Transaction::new(
            *ns_id,
            self.data[self.tx_payload_range_relative(index)].to_vec(),
        )
    }
}

// Crazy boilerplate code to make it so that [`NsPayloadOwned`] is to
// [`NsPayload`] as [`Vec<T>`] is to `[T]`. See [How can I create newtypes for
// an unsized type and its owned counterpart (like `str` and `String`) in safe
// Rust? - Stack Overflow](https://stackoverflow.com/q/64977525)
//
// TODO can't impl Borrow, which means I can't get Deref, ToOwned

// TODO move this module to a separate file?
// TODO pub(crate) only for iter module
pub(crate) mod tx_iter {
    use crate::block2::payload_bytes::NUM_NSS_BYTE_LEN;

    use super::*; // TODO temp
    /// TODO explain: index has same byte length as num_txs, store in serialized form
    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct TxIndex([u8; NUM_TXS_BYTE_LEN]);

    impl TxIndex {
        /// Return a byte range into a tx table for use in a transaction proof.
        ///
        /// TODO move this method to NsPayloadRange, where it can be properly translated into the payload.
        ///
        /// The returned range `R` is relative to the beginning of a payload for
        /// a namespace `N`. If `R` is to be used to retrieve bytes in a
        /// multi-namespace payload then `R` must be translated to the beginning
        /// of `N`.
        ///
        /// `R` covers one entry in the tx table if `self` is zero, otherwise it
        /// covers two entries.
        ///
        /// It is the responsibility of the caller to ensure that `R` is used
        /// only when `self` is less than the number of entries in `N`'s tx
        /// table.
        ///
        /// This method should be `const` but that's forbidden by Rust.
        ///
        /// # Tx table format (MOVE THIS DOC ELSEWHERE)
        ///
        /// The bytes in this range encode tx table entries that contain the
        /// (start,end) byte indices for the `tx_index`th transaction payload.
        ///
        /// The `tx_index`th entry in the tx table encodes the byte index of the
        /// *end* of this transaction's payload range. By deinition, this byte
        /// index is also the *start* of the *previous* transaction's payload
        /// range. Thus, the returned range includes `(tx_index - 1)`th and
        /// `tx_index`th entries of the tx table.
        ///
        /// Special case: If `tx_index` is 0 then the start index is implicitly
        /// 0, so the returned range contains only one entry from the tx table:
        /// the first entry of the tx table.
        pub fn tx_table_entries_range(&self) -> Range<usize> {
            let index = tx_offset_from_bytes(&self.0);
            let start = if index == 0 {
                // Special case: the desired range includes only one entry from
                // the tx table: the first entry. This entry starts immediately
                // following the bytes that encode the tx table length.
                NUM_NSS_BYTE_LEN
            } else {
                // The desired range starts at the beginning of the previous tx
                // table entry.
                (index - 1)
                    .saturating_mul(TX_OFFSET_BYTE_LEN)
                    .saturating_add(NUM_TXS_BYTE_LEN)
            };
            // The desired range ends at the end of this transaction's tx table entry
            let end = index
                .saturating_add(1)
                .saturating_mul(TX_OFFSET_BYTE_LEN)
                .saturating_add(NUM_TXS_BYTE_LEN);
            start..end
        }
    }

    impl NsPayload<'_> {
        /// Read the tx offset from the `index`th entry from the tx table.
        ///
        /// Panics if `index >= self.num_txs()`.
        pub fn read_tx_offset(&self, index: &TxIndex) -> usize {
            let start = tx_offset_from_bytes(&index.0) * TX_OFFSET_BYTE_LEN + NUM_TXS_BYTE_LEN;
            tx_offset_from_bytes(&self.data[start..start + TX_OFFSET_BYTE_LEN])
        }

        /// Read the tx offset from the `(index-1)`th entry from the tx table.
        /// Returns `None` if `index` is zero.
        ///
        /// Panics if `index >= self.num_txs()`.
        pub fn read_tx_offset_prev(&self, index: &TxIndex) -> Option<usize> {
            if index.0 == [0; NUM_TXS_BYTE_LEN] {
                None
            } else {
                let prev_index = TxIndex(num_txs_as_bytes(num_txs_from_bytes(&index.0) - 1));
                Some(self.read_tx_offset(&prev_index))
            }
        }
    }

    pub struct TxIter(Range<usize>);

    impl TxIter {
        pub fn new(ns_payload: &NsPayload) -> Self {
            Self(0..ns_payload.num_txs())
        }
    }

    // TODO explain: boilerplate `impl Iterator` delegates to `Range`
    impl Iterator for TxIter {
        type Item = TxIndex;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|i| TxIndex(num_txs_as_bytes(i)))
        }
    }
}

// TODO move this module to a separate file?
pub use ns_payload_range::NsPayloadRange;
mod ns_payload_range {
    use super::{tx_iter::TxIndex, NsIndex, NsPayload, Payload};
    use crate::block2::{
        ns_table::NsTable,
        payload_bytes::{NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN},
    };
    use serde::{Deserialize, Serialize};
    use std::ops::Range;

    // TODO need to manually impl serde to [u8; NS_OFFSET_BYTE_LEN]
    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct NsPayloadRange(pub(crate) Range<usize>); // TODO temporary pub(crate) for tx_proof.rs

    impl NsPayloadRange {
        // TODO newtype wrapper over return type?
        pub fn num_txs_range(&self) -> Range<usize> {
            let start = self.0.start;
            Range {
                start,
                end: start.saturating_add(NUM_TXS_BYTE_LEN).min(self.0.end),
            }
        }

        /// TODO fix: Byte length of this namespace's tx table.
        ///
        /// Guaranteed to be no larger than this namespace's payload byte length.
        ///
        /// TODO num_txs arg should be a newtype for [u8; NUM_TXS_BYTE_LEN]
        pub fn tx_table_byte_len(&self, num_txs: usize) -> usize {
            num_txs
                .saturating_mul(TX_OFFSET_BYTE_LEN)
                .saturating_add(NUM_TXS_BYTE_LEN)
                .min(self.0.len())
        }

        /// TODO explain: compute tx table entries range, translated by this namespace's start index
        pub fn tx_table_entries_range(&self, index: &TxIndex) -> Range<usize> {
            let result = index.tx_table_entries_range();
            Range {
                start: result.start.saturating_add(self.0.start),
                end: result.end.saturating_add(self.0.start),
            }
        }

        pub fn as_range(&self) -> Range<usize> {
            self.0.clone()
        }
    }

    impl NsPayload<'_> {
        /// TODO make ns_payload_range a field of NsPayload
        ///
        /// Read subslice range for the `index`th tx from the tx
        /// table, translated by [`ns_payload_range`].
        ///
        /// Returned range guaranteed to satisfy `start <= end <= namespace_byte_len`.
        ///
        /// Panics if `index >= self.num_txs()`.
        pub fn tx_payload_range(
            &self,
            index: &TxIndex,
            ns_payload_range: &NsPayloadRange,
        ) -> Range<usize> {
            let result = self.tx_payload_range_relative(index);
            result.start.saturating_add(ns_payload_range.0.start)
                ..result.end.saturating_add(ns_payload_range.0.start)
        }
    }

    impl NsTable {
        /// Read subslice range for the `index`th namespace from the namespace
        /// table.
        ///
        /// It is the responsibility of the caller to ensure that the `index`th
        /// entry is not a duplicate of a previous entry. Otherwise the returned
        /// range will be invalid. (Can the caller even create his own `NsIndex`??)
        ///
        /// Returned range guaranteed to satisfy `start <= end <=
        /// payload_byte_len`.
        ///
        /// TODO remove `payload_byte_len` arg and do not check `end`?
        ///
        /// Panics if `index >= self.num_nss()`.
        pub fn ns_payload_range(&self, index: &NsIndex, payload_byte_len: usize) -> NsPayloadRange {
            let end = self.read_ns_offset(index).min(payload_byte_len);
            let start = self.read_ns_offset_prev(index).unwrap_or(0).min(end);
            NsPayloadRange(start..end)
        }
    }

    impl Payload {
        /// TODO panics if index out of bounds
        pub fn ns_payload(&self, index: &NsIndex) -> NsPayload {
            let range = self.ns_table.ns_payload_range(index, self.payload.len());
            let data = &self.payload[range.as_range()];
            NsPayload { range, data }
        }
    }
}
