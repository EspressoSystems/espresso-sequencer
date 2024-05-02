use crate::{
    block2::{
        ns_table::ns_iter::NsIndex,
        payload_bytes::{
            num_txs_as_bytes, num_txs_from_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN,
            TX_OFFSET_BYTE_LEN,
        },
        Payload,
    },
    NamespaceId, Transaction,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use tx_iter::{TxIndex, TxIter};

pub mod tx_iter;

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
#[repr(transparent)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Debug)]
pub struct NsPayload([u8]);

#[repr(transparent)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct NsPayloadOwned(Vec<u8>);

impl NsPayload {
    /// Number of bytes used to encode the number of txs in the tx table.
    ///
    /// Returns the minimum of [`NUM_TXS_BYTE_LEN`] and the byte length of the
    /// entire namespace payload.
    ///
    /// In all nontrivial cases this quantity is [`NUM_TXS_BYTE_LEN`]. Anything
    /// else is a degenerate case.
    pub fn num_txs_byte_len(&self) -> usize {
        NUM_TXS_BYTE_LEN.min(self.0.len())
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
            num_txs_from_bytes(&self.0[..num_txs_byte_len]),
            // Max number of entries that could fit in the namespace payload
            self.0.len().saturating_sub(num_txs_byte_len) / TX_OFFSET_BYTE_LEN,
        )
    }

    /// Byte length of this namespace's tx table.
    ///
    /// Guaranteed to be no larger than this namespace's payload byte length.
    pub fn tx_table_byte_len(&self) -> usize {
        self.num_txs()
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN)
            .min(self.0.len())
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
            .min(self.0.len());
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
        &self.0
    }

    /// TODO store `ns_id` in `NsPayload` struct?
    pub fn export_all_txs(&self, ns_id: &NamespaceId) -> Vec<Transaction> {
        TxIter::new(self)
            .map(|i| Transaction::new(*ns_id, self.0[self.tx_payload_range_relative(&i)].to_vec()))
            .collect()
    }
    pub fn export_tx(&self, ns_id: &NamespaceId, index: &TxIndex) -> Transaction {
        Transaction::new(
            *ns_id,
            self.0[self.tx_payload_range_relative(index)].to_vec(),
        )
    }
}

/// Crazy boilerplate code to make it so that [`NsPayloadOwned`] is to
/// [`NsPayload`] as [`Vec<T>`] is to `[T]`. See [How can I create newtypes for
/// an unsized type and its owned counterpart (like `str` and `String`) in safe
/// Rust? - Stack Overflow](https://stackoverflow.com/q/64977525)
mod ns_payload_owned {
    use super::{NsPayload, NsPayloadOwned};
    use std::borrow::Borrow;
    use std::ops::Deref;

    impl NsPayload {
        // pub(super) because I want it private to this file, but I also want to
        // contain this boilerplate code in its on module.ß
        pub(super) fn new(p: &[u8]) -> &NsPayload {
            unsafe { &*(p as *const [u8] as *const NsPayload) }
        }
    }

    impl Deref for NsPayloadOwned {
        type Target = NsPayload;
        fn deref(&self) -> &NsPayload {
            NsPayload::new(&self.0)
        }
    }

    impl Borrow<NsPayload> for NsPayloadOwned {
        fn borrow(&self) -> &NsPayload {
            self.deref()
        }
    }

    impl ToOwned for NsPayload {
        type Owned = NsPayloadOwned;
        fn to_owned(&self) -> NsPayloadOwned {
            NsPayloadOwned(self.0.to_owned())
        }
    }
}

impl Payload {
    /// TODO panics if index out of bounds
    pub fn ns_payload(&self, index: &NsIndex) -> &NsPayload {
        let range = self
            .ns_table
            .ns_payload_range(index, self.payload.len())
            .as_range();
        NsPayload::new(&self.payload[range])
    }
}
