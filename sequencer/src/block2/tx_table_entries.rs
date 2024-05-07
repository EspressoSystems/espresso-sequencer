use crate::block2::{
    ns_payload,
    payload_bytes::{tx_offset_as_bytes, TX_OFFSET_BYTE_LEN},
};
use std::ops::Range;

/// manual serde as a byte array.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TxTableEntries {
    cur: usize,
    prev: Option<usize>,
}

/// Manual [`serde`] impl for [`TxTableEntries`].
mod tx_table_entries_serde {
    use crate::block2::{
        payload_bytes::{tx_offset_as_bytes, tx_offset_from_bytes, TX_OFFSET_BYTE_LEN},
        tx_table_entries::TxTableEntries,
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Serialize, Deserialize)]
    struct TxTableEntriesSerde {
        cur: [u8; TX_OFFSET_BYTE_LEN],
        prev: Option<[u8; TX_OFFSET_BYTE_LEN]>,
    }

    impl Serialize for TxTableEntries {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            TxTableEntriesSerde {
                cur: tx_offset_as_bytes(self.cur),
                prev: self.prev.map(tx_offset_as_bytes),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for TxTableEntries {
        fn deserialize<D>(deserializer: D) -> Result<TxTableEntries, D::Error>
        where
            D: Deserializer<'de>,
        {
            <TxTableEntriesSerde as Deserialize>::deserialize(deserializer).map(|x| {
                TxTableEntries {
                    cur: tx_offset_from_bytes(&x.cur),
                    prev: x.prev.map(|bytes| tx_offset_from_bytes(&bytes)),
                }
            })
        }
    }
}

impl TxTableEntries {
    /// Infallible serialization.
    ///
    /// TODO same question as [`NumTxs::as_bytes`]
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(TX_OFFSET_BYTE_LEN.saturating_mul(2));
        if let Some(prev) = self.prev {
            bytes.extend(tx_offset_as_bytes(prev));
        }
        bytes.extend(tx_offset_as_bytes(self.cur));
        bytes
    }

    /// Convert a [`TxTableEntries`] to a valid [`Range`], translated and capped.
    ///
    /// Returned range guaranteed to satisfy `translate <= start <= end <= cap`.
    pub fn as_range(&self, translate: usize, cap: usize) -> Range<usize> {
        let end = self.cur.saturating_add(translate).min(cap);
        let start = self.prev.unwrap_or(0).saturating_add(translate).min(end);
        start..end
    }

    /// TODO explain: [`ns_payload::A`] arg allows access to this method only
    /// from within [`ns_payload`] module.
    pub fn new(_: ns_payload::A, cur: usize, prev: Option<usize>) -> Self {
        Self { cur, prev }
    }
}
