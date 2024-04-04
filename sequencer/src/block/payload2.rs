// use serde::{Deserialize, Serialize};

use self::tx_table::{num_txs_as_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN};
use crate::Transaction;

// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Default)]
pub struct NamespaceBuilder {
    tx_table_entries: Vec<u8>,
    tx_bodies: Vec<u8>,
}

impl NamespaceBuilder {
    /// Return an empty namespace
    pub fn new() -> Self {
        Self {
            tx_table_entries: Vec::new(),
            tx_bodies: Vec::new(),
        }
    }

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
    use std::{fmt::Display, mem::size_of};

    use num_traits::{Bounded, Num, PrimInt, ToBytes};

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
        usize_to_bytes(num_txs)
    }

    /// Serialize `tx_offset` into `TX_OFFSET_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `tx_offset` cannot fit into `TX_OFFSET_BYTE_LEN` bytes.
    pub fn tx_offset_as_bytes(tx_offset: usize) -> [u8; TX_OFFSET_BYTE_LEN] {
        usize_to_bytes(tx_offset)
    }

    /// Serialize `num_nss` into `NUM_NSS_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `num_nss` cannot fit into `NUM_NSS_BYTE_LEN` bytes.
    pub fn num_nss_as_bytes(num_nss: usize) -> [u8; NUM_NSS_BYTE_LEN] {
        usize_to_bytes(num_nss)
    }

    /// Serialize `ns_offset` into `NS_OFFSET_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `ns_offset` cannot fit into `NS_OFFSET_BYTE_LEN` bytes.
    pub fn ns_offset_as_bytes(ns_offset: usize) -> [u8; NS_OFFSET_BYTE_LEN] {
        usize_to_bytes(ns_offset)
    }

    /// Serialize `ns_id` into `NS_ID_BYTE_LEN` bytes.
    ///
    /// # Panics
    /// If `ns_id` cannot fit into `NS_ID_BYTE_LEN` bytes.
    pub fn ns_id_as_bytes(ns_id: usize) -> [u8; NS_ID_BYTE_LEN] {
        usize_to_bytes(ns_id)
    }

    /// Serialize `n` into `BYTE_LEN` bytes in little-endian form, padding with
    /// 0 as needed.
    ///
    /// # Panics
    /// If `n` cannot fit into `BYTE_LEN` bytes.
    fn usize_to_bytes<const BYTE_LEN: usize>(n: usize) -> [u8; BYTE_LEN] {
        if size_of::<usize>() > BYTE_LEN {
            assert!(
                n <= max_from_byte_len(BYTE_LEN),
                "n {n} cannot fit into {BYTE_LEN} bytes"
            );
            n.to_le_bytes()[..BYTE_LEN].try_into().unwrap() // panic is impossible
        } else {
            // convert `n` to bytes and pad with 0
            let mut result = [0; BYTE_LEN];
            result[..size_of::<usize>()].copy_from_slice(&n.to_le_bytes()[..]);
            result
        }
    }

    trait Foo: PrimInt + ToBytes + Display {}

    fn to_bytes<const BYTE_LEN: usize, T: Foo>(n: T) -> [u8; BYTE_LEN] {
        if size_of::<T>() > BYTE_LEN {
            assert!(
                n <= max_from_byte_len2(BYTE_LEN),
                "n {n} cannot fit into {BYTE_LEN} bytes"
            );
            n.to_le_bytes().as_ref()[..BYTE_LEN].try_into().unwrap() // panic is impossible
        } else {
            // convert `n` to bytes and pad with 0
            let mut result = [0; BYTE_LEN];
            result[..size_of::<usize>()].copy_from_slice(&n.to_le_bytes().as_ref());
            result
        }
    }

    fn max_from_byte_len2<T: Foo>(byte_len: usize) -> T {
        if byte_len >= size_of::<T>() {
            T::max_value()
        } else {
            // panic is impossible because `byte_len < size_of::<T>()`
            T::from((1 << (byte_len * 8)) - 1).unwrap()
        }
    }

    // const fn max_num_txs() -> usize {
    //     max_from_byte_len(NUM_TXS_BYTE_LEN)
    // }

    /// Return the largest `usize` value that can fit into `byte_len` bytes.
    const fn max_from_byte_len(byte_len: usize) -> usize {
        if byte_len >= size_of::<usize>() {
            usize::MAX
        } else {
            // overflow cannot occur because `byte_len < size_of::<usize>()`
            (1 << (byte_len * 8)) - 1
        }
    }

    // pub trait ToBytes<const SIZE: usize> {
    //     fn to_le_bytes(self) -> [u8; SIZE];
    // }

    // impl ToBytes<{ size_of::<usize>() }> for usize {
    //     fn to_le_bytes(self) -> [u8; size_of::<usize>()] {
    //         self.to_le_bytes()
    //     }
    // }

    #[cfg(test)]
    mod test {
        use super::{max_from_byte_len, max_from_byte_len2, usize_to_bytes, Foo};
        use fluent_asserter::prelude::*;
        use std::mem::size_of;

        #[test]
        fn max_from_byte_len_correctness() {
            // test byte lengths 0 to size_of::<usize>()
            let mut bytes = [0; size_of::<usize>()];
            assert_eq!(max_from_byte_len(0), 0);
            for i in 0..bytes.len() {
                bytes[i] = 0xff;
                assert_eq!(max_from_byte_len(i + 1).to_le_bytes(), bytes);
            }

            // test byte lengths size_of::<usize>() to twice that length
            for i in size_of::<usize>()..2 * size_of::<usize>() {
                assert_eq!(max_from_byte_len(i + 1), usize::MAX);
            }
        }

        #[test]
        fn max_from_byte_len2_correctness() {}

        fn max_from_byte_len2_correctness_generic<T: Foo>() {
            // test byte lengths 0 to size_of::<T>()
            let mut bytes = [0; size_of::<T>()];
            assert_eq!(max_from_byte_len2(0), 0);
            for i in 0..bytes.len() {
                bytes[i] = 0xff;
                assert_eq!(max_from_byte_len2(i + 1).to_le_bytes(), bytes);
            }

            // test byte lengths size_of::<usize>() to twice that length
            for i in size_of::<T>()..2 * size_of::<T>() {
                assert_eq!(max_from_byte_len2(i + 1), T::max_value());
            }
        }

        #[test]
        fn usize_to_bytes_correctness() {
            // byte length 0
            assert_eq!(usize_to_bytes(0), [0; 0]);
            assert_that_code!(|| usize_to_bytes::<0>(1)).panics();

            // byte length 1
            assert_eq!(usize_to_bytes(0), [0; 1]);
            assert_eq!(usize_to_bytes(255), [255; 1]);
            assert_that_code!(|| usize_to_bytes::<1>(256)).panics();

            // byte length 2
            assert_eq!(usize_to_bytes(0), [0; 2]);
            assert_eq!(usize_to_bytes(65535), [255; 2]);
            assert_that_code!(|| usize_to_bytes::<2>(65536)).panics();

            // byte length size_of::<usize>()
            assert_eq!(usize_to_bytes(0), [0; size_of::<usize>()]);
            assert_eq!(usize_to_bytes(usize::MAX), [255; size_of::<usize>()]);

            // byte length size_of::<usize>() + 1
            assert_eq!(usize_to_bytes(0), [0; size_of::<usize>() + 1]);
            let usize_max_bytes = {
                let mut bytes = [255; size_of::<usize>() + 1];
                bytes[bytes.len() - 1] = 0;
                bytes
            };
            assert_eq!(usize_to_bytes(usize::MAX), usize_max_bytes);
        }
    }
}
