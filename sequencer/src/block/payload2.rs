// use serde::{Deserialize, Serialize};

use self::tx_table::{num_txs_as_bytes, tx_offset_as_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN};
use crate::Transaction;

// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

mod tx_table {
    use std::mem::size_of;

    pub const NUM_TXS_BYTE_LEN: usize = 4;
    pub const TX_OFFSET_BYTE_LEN: usize = 4;

    pub fn num_txs_as_bytes(num_txs: usize) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes(num_txs)
    }

    pub fn tx_offset_as_bytes(tx_offset: usize) -> [u8; TX_OFFSET_BYTE_LEN] {
        usize_to_bytes(tx_offset)
    }

    /// Return `n` as an array of `BYTE_LEN` bytes in little-endian form,
    /// padding with 0 as needed.
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

    #[cfg(test)]
    mod test {
        use super::{max_from_byte_len, usize_to_bytes};
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
