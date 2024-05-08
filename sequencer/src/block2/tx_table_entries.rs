use crate::block2::{ns_payload, uint_bytes::usize_to_bytes, TX_OFFSET_BYTE_LEN};
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
        tx_table_entries::TxTableEntries,
        uint_bytes::{usize_from_bytes, usize_to_bytes},
        TX_OFFSET_BYTE_LEN,
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
                cur: usize_to_bytes(self.cur),
                prev: self.prev.map(usize_to_bytes),
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
                    cur: usize_from_bytes::<TX_OFFSET_BYTE_LEN>(&x.cur),
                    prev: x
                        .prev
                        .map(|bytes| usize_from_bytes::<TX_OFFSET_BYTE_LEN>(&bytes)),
                }
            })
        }
    }
}

impl TxTableEntries {
    /// Infallible serialization.
    ///
    /// Returned bytes contain either one entry or two consecutive entries of
    /// the tx table according to whether [`self`] was derived from the first
    /// entry in the table. See [`TxIndex::tx_table_entries_range_relative`].
    ///
    /// Returned bytes differ from [`serde`] serialization.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(TX_OFFSET_BYTE_LEN.saturating_mul(2));
        if let Some(prev) = self.prev {
            bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(prev));
        }
        bytes.extend(usize_to_bytes::<TX_OFFSET_BYTE_LEN>(self.cur));
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
