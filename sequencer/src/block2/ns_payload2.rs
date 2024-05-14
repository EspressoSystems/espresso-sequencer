//! The only thing [`NsPayload2`] does is naively read from its payload given a
//! byte range. It doesn't know anything about the underlying binary format.
//! That's all done in `NsPayloadRange2`.

use crate::{NamespaceId, Transaction};

use super::{
    newtypes::{
        AsPayloadBytes, NsPayloadByteLen, NumTxsRange2, PayloadBytesRange, TxPayloadRange,
        TxTableEntriesRange2,
    },
    tx_iter::TxIter,
};
use serde::{Deserialize, Serialize};

pub struct NsPayload2([u8]);

impl NsPayload2 {
    pub fn new(bytes: &[u8]) -> &NsPayload2 {
        NsPayload2::new_private(bytes)
    }

    pub fn byte_len(&self) -> NsPayloadByteLen {
        NsPayloadByteLen::from_usize(self.0.len())
    }

    /// Access the bytes of this [`NsPayload`].
    pub fn as_byte_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn read<T>(&self, range: &T) -> T::Output<'_>
    where
        T: PayloadBytesRange,
    {
        <T::Output<'_> as AsPayloadBytes>::from_payload_bytes(&self.0[range.ns_payload_range()])
    }

    pub fn export_all_txs(&self, ns_id: &NamespaceId) -> Vec<Transaction> {
        // TODO I guess I need helpers for all this...
        let num_txs = self.read(&NumTxsRange2::new(&self.byte_len()));
        TxIter::new2(&num_txs, &self.byte_len())
            .map(|i| {
                let payload = self
                    .read(&TxPayloadRange::new(
                        &num_txs,
                        &self.read(&TxTableEntriesRange2::new(&i)),
                        &self.byte_len(),
                    ))
                    .to_payload_bytes()
                    .as_ref()
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
pub struct NsPayloadOwned2(Vec<u8>);

/// Crazy boilerplate code to make it so that [`NsPayloadOwned`] is to
/// [`NsPayload`] as [`Vec<T>`] is to `[T]`. See [How can I create newtypes for
/// an unsized type and its owned counterpart (like `str` and `String`) in safe
/// Rust? - Stack Overflow](https://stackoverflow.com/q/64977525)
mod ns_payload_owned {
    use super::{NsPayload2, NsPayloadOwned2};
    use std::borrow::Borrow;
    use std::ops::Deref;

    impl NsPayload2 {
        // pub(super) because I want it visible everywhere in this file but I
        // also want this boilerplate code quarrantined in `ns_payload_owned`.
        pub(super) fn new_private(p: &[u8]) -> &NsPayload2 {
            unsafe { &*(p as *const [u8] as *const NsPayload2) }
        }
    }

    impl Deref for NsPayloadOwned2 {
        type Target = NsPayload2;
        fn deref(&self) -> &NsPayload2 {
            NsPayload2::new_private(&self.0)
        }
    }

    impl Borrow<NsPayload2> for NsPayloadOwned2 {
        fn borrow(&self) -> &NsPayload2 {
            self.deref()
        }
    }

    impl ToOwned for NsPayload2 {
        type Owned = NsPayloadOwned2;
        fn to_owned(&self) -> NsPayloadOwned2 {
            NsPayloadOwned2(self.0.to_owned())
        }
    }
}
