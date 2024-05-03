use crate::{
    block2::{
        ns_table::ns_iter::NsIndex,
        payload_bytes::{
            num_txs_as_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
        },
        Payload,
    },
    NamespaceId, Transaction,
};
use serde::{Deserialize, Serialize};
use tx_iter::{TxIndex, TxIter};

pub mod num_txs;
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
    fn num_txs_byte_len(&self) -> usize {
        NUM_TXS_BYTE_LEN.min(self.0.len())
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
