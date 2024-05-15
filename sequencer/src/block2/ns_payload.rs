//! The only thing [`NsPayload2`] does is naively read from its payload given a
//! byte range. It doesn't know anything about the underlying binary format.
//! That's all done in `xxx`.

use crate::{NamespaceId, Transaction};

use super::newtypes::{
    FromNsPayloadBytes, NsPayloadByteLen, NsPayloadBytesRange, NumTxs, NumTxsRange, TxIter,
    TxPayloadRange, TxTableEntriesRange,
};
use serde::{Deserialize, Serialize};

pub struct NsPayload([u8]);

impl NsPayload {
    pub fn from_bytes_slice(bytes: &[u8]) -> &NsPayload {
        NsPayload::new_private(bytes)
    }

    pub fn as_bytes_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn byte_len(&self) -> NsPayloadByteLen {
        NsPayloadByteLen::from_usize(self.0.len())
    }

    pub fn read<'a, R>(&'a self, range: &R) -> R::Output
    where
        R: NsPayloadBytesRange<'a>,
    {
        <R::Output as FromNsPayloadBytes<'a>>::from_payload_bytes(&self.0[range.ns_payload_range()])
    }

    /// Return a `Vec` of all transactions in this namespace...
    pub fn export_all_txs(&self, ns_id: &NamespaceId) -> Vec<Transaction> {
        // TODO I guess I need helpers for all this...
        let num_txs = self.read(&NumTxsRange::new(&self.byte_len()));
        let num_txs_checked = NumTxs::new(&num_txs, &self.byte_len());
        TxIter::new(&num_txs_checked)
            .map(|i| {
                let payload = self
                    .read(&TxPayloadRange::new(
                        &num_txs,
                        &self.read(&TxTableEntriesRange::new(&i)),
                        &self.byte_len(),
                    ))
                    .to_payload_bytes()
                    .to_vec();
                Transaction::new(*ns_id, payload)
            })
            .collect()
    }
}

#[repr(transparent)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct NsPayloadOwned(Vec<u8>);

/// Crazy boilerplate code to make it so that [`NsPayloadOwned`] is to
/// [`NsPayload`] as [`Vec<T>`] is to `[T]`. See [How can I create newtypes for
/// an unsized type and its owned counterpart (like `str` and `String`) in safe
/// Rust? - Stack Overflow](https://stackoverflow.com/q/64977525)
mod ns_payload_owned {
    use super::{NsPayload, NsPayloadOwned};
    use std::borrow::Borrow;
    use std::ops::Deref;

    impl NsPayload {
        // pub(super) because I want it visible everywhere in this file but I
        // also want this boilerplate code quarrantined in `ns_payload_owned`.
        pub(super) fn new_private(p: &[u8]) -> &NsPayload {
            unsafe { &*(p as *const [u8] as *const NsPayload) }
        }
    }

    impl Deref for NsPayloadOwned {
        type Target = NsPayload;
        fn deref(&self) -> &NsPayload {
            NsPayload::new_private(&self.0)
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
