// use serde::{Deserialize, Serialize};

use self::tx_table::{num_txs_as_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN};
use crate::{NamespaceId, Transaction};

// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Default)]
pub struct NamespaceBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NamespaceBuilder {
    // /// Return an empty namespace
    // pub fn new() -> Self {
    //     Self {
    //         tx_table_entries: Vec::new(),
    //         tx_bodies: Vec::new(),
    //     }
    // }

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

// TODO better way to do this?
pub use tx_table::{
    ns_id_as_bytes, ns_offset_as_bytes, num_nss_as_bytes, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN,
    NUM_NSS_BYTE_LEN,
};

// TODO rename from tx_table, this mod also has ns_table utils
mod tx_table {
    use paste::paste;
    use std::mem::size_of;

    use crate::NamespaceId;

    pub const NUM_TXS_BYTE_LEN: usize = 4;
    pub const TX_OFFSET_BYTE_LEN: usize = 4;
    pub const NUM_NSS_BYTE_LEN: usize = NUM_TXS_BYTE_LEN;
    pub const NS_OFFSET_BYTE_LEN: usize = TX_OFFSET_BYTE_LEN;
    pub const NS_ID_BYTE_LEN: usize = 4;

    /// Serialize `num_txs` into `NUM_TXS_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `num_txs` cannot fit into `NUM_TXS_BYTE_LEN` bytes.
    pub fn num_txs_as_bytes(num_txs: usize) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes2(num_txs)
    }

    /// Serialize `tx_offset` into `TX_OFFSET_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `tx_offset` cannot fit into `TX_OFFSET_BYTE_LEN` bytes.
    pub fn tx_offset_as_bytes(tx_offset: usize) -> [u8; TX_OFFSET_BYTE_LEN] {
        usize_to_bytes2(tx_offset)
    }

    /// Serialize `num_nss` into `NUM_NSS_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `num_nss` cannot fit into `NUM_NSS_BYTE_LEN` bytes.
    pub fn num_nss_as_bytes(num_nss: usize) -> [u8; NUM_NSS_BYTE_LEN] {
        usize_to_bytes2(num_nss)
    }

    /// Serialize `ns_offset` into `NS_OFFSET_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `ns_offset` cannot fit into `NS_OFFSET_BYTE_LEN` bytes.
    pub fn ns_offset_as_bytes(ns_offset: usize) -> [u8; NS_OFFSET_BYTE_LEN] {
        usize_to_bytes2(ns_offset)
    }

    /// Serialize `ns_id` into `NS_ID_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `ns_id` cannot fit into `NS_ID_BYTE_LEN` bytes.
    ///
    /// TODO I'm cheating by converting `NamespaceId` via `u64::from`, which is
    /// available only because `NamespaceId` derives `From`. (Not sure it should
    /// be doing that. What's the point of the newtype?). Maybe I should instead
    /// use serialization provided by `NamespaceId`? The problem is that
    /// serialization is not ergonomic compared to mine here, which is
    /// infallible and returns a constant-size array.
    pub fn ns_id_as_bytes(ns_id: NamespaceId) -> [u8; NS_ID_BYTE_LEN] {
        u64_to_bytes2(u64::from(ns_id))
    }

    /// Deserialize `bytes` in little-endian form into a `usize`, padding with 0
    /// as needed.
    ///
    /// # Panics
    /// If `bytes.len()` is too large to fit into a `usize`.
    pub fn usize_from_bytes2(bytes: &[u8]) -> usize {
        assert!(
            bytes.len() <= size_of::<usize>(),
            "bytes len {} cannot fit into usize",
            bytes.len()
        );
        let mut usize_bytes = [0; size_of::<usize>()];
        usize_bytes[..bytes.len()].copy_from_slice(bytes);
        usize::from_le_bytes(usize_bytes)
    }

    // Use an ugly macro because it's difficult or impossible to be generic over
    // primitive types such as `usize`, `u64`.
    macro_rules! to_bytes_impl {
        ($T:ty) => {
            paste! {
                /// Serialize `n` into `BYTE_LEN` bytes in little-endian form, padding with
                /// 0 as needed.
                ///
                /// # Panics
                /// If `n` cannot fit into `BYTE_LEN` bytes.
                fn [<$T _to_bytes2>]<const BYTE_LEN: usize>(n: $T) -> [u8; BYTE_LEN] {
                    if size_of::<$T>() > BYTE_LEN {
                        assert!(
                            n <= [<$T _max_from_byte_len2>](BYTE_LEN),
                            "n {n} cannot fit into {BYTE_LEN} bytes"
                        );
                        n.to_le_bytes()[..BYTE_LEN].try_into().unwrap() // panic is impossible
                    } else {
                        // convert `n` to bytes and pad with 0
                        let mut result = [0; BYTE_LEN];
                        result[..size_of::<$T>()].copy_from_slice(&n.to_le_bytes()[..]);
                        result
                    }
                }

                /// Return the largest `$T` value that can fit into `byte_len` bytes.
                const fn [<$T _max_from_byte_len2>](byte_len: usize) -> $T {
                    if byte_len >= size_of::<$T>() {
                        $T::MAX
                    } else {
                        // overflow cannot occur because `byte_len < size_of::<$T>()`
                        (1 << (byte_len * 8)) - 1
                    }
                }
            }
        };
    }

    to_bytes_impl!(usize);
    to_bytes_impl!(u64);

    #[cfg(test)]
    mod test {
        use fluent_asserter::prelude::*;
        use paste::paste;
        use std::mem::size_of;

        use super::usize_from_bytes2;

        #[test]
        fn usize_from_bytes2_correctness() {
            let bytes = [255; size_of::<usize>() + 1];
            for len in 0..=size_of::<usize>() {
                assert_eq!(
                    usize_from_bytes2(&bytes[..len]),
                    usize_max_from_byte_len2(len)
                );
            }
            assert_that_code!(|| usize_from_bytes2(&bytes[..])).panics();
        }

        macro_rules! to_bytes_test_impl {
            ($T:ty) => {
                paste! {
                    use super::{[<$T _max_from_byte_len2>], [<$T _to_bytes2>]};

                    #[test]
                    fn [<$T _max_from_byte_len2_correctness>]() {
                        // test byte lengths 0 to size_of::<$T>()
                        let mut bytes = [0; size_of::<$T>()];
                        assert_eq!([<$T _max_from_byte_len2>](0), 0);
                        for i in 0..bytes.len() {
                            bytes[i] = 0xff;
                            assert_eq!([<$T _max_from_byte_len2>](i + 1).to_le_bytes(), bytes);
                        }

                        // test byte lengths size_of::<$T>() to twice that length
                        for i in size_of::<$T>()..2 * size_of::<$T>() {
                            assert_eq!([<$T _max_from_byte_len2>](i + 1), $T::MAX);
                        }
                    }

                    #[test]
                    fn [<$T _to_bytes2_correctness>]() {
                        // byte length 0
                        assert_eq!([<$T _to_bytes2>](0), [0; 0]);
                        assert_that_code!(|| [<$T _to_bytes2>]::<0>(1)).panics();

                        // byte length 1
                        assert_eq!([<$T _to_bytes2>](0), [0; 1]);
                        assert_eq!([<$T _to_bytes2>](255), [255; 1]);
                        assert_that_code!(|| [<$T _to_bytes2>]::<1>(256)).panics();

                        // byte length 2
                        assert_eq!([<$T _to_bytes2>](0), [0; 2]);
                        assert_eq!([<$T _to_bytes2>](65535), [255; 2]);
                        assert_that_code!(|| [<$T _to_bytes2>]::<2>(65536)).panics();

                        // byte length size_of::<$T>()
                        assert_eq!([<$T _to_bytes2>](0), [0; size_of::<$T>()]);
                        assert_eq!([<$T _to_bytes2>]($T::MAX), [255; size_of::<$T>()]);

                        // byte length size_of::<$T>() + 1
                        assert_eq!([<$T _to_bytes2>](0), [0; size_of::<$T>() + 1]);
                        let [<$T _max_bytes>] = {
                            let mut bytes = [255; size_of::<$T>() + 1];
                            bytes[bytes.len() - 1] = 0;
                            bytes
                        };
                        assert_eq!([<$T _to_bytes2>]($T::MAX), [<$T _max_bytes>]);
                    }
                }
            };
        }

        to_bytes_test_impl!(usize);
        to_bytes_test_impl!(u64);
    }
}
