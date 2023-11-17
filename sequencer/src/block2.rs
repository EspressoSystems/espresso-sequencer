use crate::Transaction;
use hotshot_query_service::QueryableBlock;
use serde::{Deserialize, Serialize};
use std::{mem::size_of, ops::Range};

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BlockPayload {
    payload: Vec<u8>,
}

impl BlockPayload {
    #[allow(dead_code)] // TODO temporary
    fn build(txs: impl IntoIterator<Item = Transaction>) -> Self {
        // tx_table[i] is the end index (exclusive) of the ith tx
        // so that the payload bytes of the ith tx is
        // tx_bodies[tx_table[i-1]..tx_table[i]].
        // edge case: tx_table[-1] is defined as 0.
        //
        // TODO final entry should be implicit:
        // https://github.com/EspressoSystems/espresso-sequencer/issues/757
        let mut tx_table = Vec::new();

        // concatenation of all tx payloads
        let mut tx_bodies = Vec::<u8>::new();

        // build tx_table, tx_bodies
        let mut end: u32 = 0;
        for tx in txs.into_iter() {
            // TODO do not panic, return Result instead:
            // https://github.com/EspressoSystems/espresso-sequencer/pull/756#discussion_r1394550640
            let len: u32 = tx
                .payload()
                .len()
                .try_into()
                .expect("tx byte length should fit into u32");

            end = end
                .checked_add(len)
                .expect("total byte length of all tx bodies should fit into u32");

            tx_table.extend(end.to_le_bytes());
            tx_bodies.extend(tx.payload());
        }

        // tx_table_len is the number of 4-byte entries.
        // tx_table.len() is the number of bytes in the tx table,
        // so we divide by size_of::<u32>()
        let tx_table_len: u32 = (tx_table.len() / size_of::<u32>())
            .try_into()
            .expect("tx_table len should fit into u32");

        // Naively copy all pieces into a flat payload.
        // We could avoid this by allocating memory in advance,
        // but that would require knowing the number of txs and their total
        // byte length in advance, which we can't do without a complete scan
        // of the `txs` iterator.
        let mut payload = Vec::new();
        payload.extend(tx_table_len.to_le_bytes());
        payload.extend(tx_table);
        payload.extend(tx_bodies);
        Self { payload }
    }

    fn tx_table_len(&self) -> u32 {
        // TODO don't panic
        // Alternative: store this value as a separate field of `Self`.
        u32::from_le_bytes(self.payload[..size_of::<u32>()].try_into().unwrap())
    }
}

impl QueryableBlock for BlockPayload {
    type TransactionIndex = u32;
    type Iter<'a> = Range<Self::TransactionIndex>;
    type InclusionProof = ();

    fn len(&self) -> usize {
        // TODO don't panic
        // Alternative: do `as usize` and pray?
        // We should probably fail the build if usize is smaller than u32.
        self.tx_table_len()
            .try_into()
            .expect("number of txs should fit into usize")
    }

    fn iter(&self) -> Self::Iter<'_> {
        0..self.tx_table_len()
    }

    fn transaction_with_proof(
        &self,
        _index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        todo!()
    }
}

mod boilerplate {
    use super::{BlockPayload, Transaction};
    use commit::Committable;
    use std::fmt::Display;

    // TODO temporary.
    // `Block` trait subject to change/delection.
    // Skeleton impl for now so as to enable `QueryableBlock`.
    impl hotshot::traits::Block for BlockPayload {
        type Error = crate::Error;
        type Transaction = Transaction;

        fn new() -> Self {
            todo!()
        }

        fn add_transaction_raw(
            &self,
            _tx: &Self::Transaction,
        ) -> std::result::Result<Self, Self::Error> {
            todo!()
        }

        fn contained_transactions(
            &self,
        ) -> std::collections::HashSet<commit::Commitment<Self::Transaction>> {
            todo!()
        }
    }

    impl Display for BlockPayload {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:#?}")
        }
    }

    impl Committable for BlockPayload {
        fn commit(&self) -> commit::Commitment<Self> {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BlockPayload;
    use crate::Transaction;
    use std::mem::size_of;

    #[test]
    fn build_basic_correctness() {
        // play with this
        let test_cases = vec![
            // 3 non-empty txs
            vec![
                vec![0, 1, 2, 3, 4],
                vec![5, 6, 7, 8, 9, 10, 11, 12],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx at the beginning
            vec![
                vec![],
                vec![5, 6, 7, 8, 9, 10, 11, 12],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx in the middle
            vec![
                vec![0, 1, 2, 3, 4],
                vec![],
                vec![13, 14, 15, 16, 17, 18, 19, 20],
            ],
            // 1 empty tx at the end
            vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9, 10, 11, 12], vec![]],
            // 1 nonempty tx
            vec![vec![0, 1, 2, 3, 4]],
            // 1 empty tx
            vec![vec![]],
            // zero txs
            vec![],
        ];

        for tx_payloads in test_cases {
            // prepare things as a function of the test case
            let txs = tx_payloads
                .iter()
                .cloned()
                .map(|payload| Transaction::new(crate::VmId(0), payload));
            let tx_offsets: Vec<u32> = tx_payloads
                .iter()
                .scan(0, |end, tx| {
                    *end += u32::try_from(tx.len()).unwrap();
                    Some(*end)
                })
                .collect();

            let block = BlockPayload::build(txs);

            // test tx table length
            let (tx_table_len_bytes, payload) = block.payload.split_at(size_of::<u32>());
            let tx_table_len = u32::from_le_bytes(tx_table_len_bytes.try_into().unwrap());
            assert_eq!(tx_table_len, u32::try_from(tx_payloads.len()).unwrap());

            // test tx table contents
            let (tx_table_bytes, payload) = payload.split_at(tx_payloads.len() * size_of::<u32>());
            let tx_table: Vec<u32> = tx_table_bytes
                .chunks(size_of::<u32>())
                .map(|len_bytes| u32::from_le_bytes(len_bytes.try_into().unwrap()))
                .collect();
            assert_eq!(tx_table, tx_offsets);

            // test block payload body
            let tx_payloads_flat: Vec<u8> = tx_payloads.into_iter().flatten().collect();
            assert_eq!(payload, tx_payloads_flat);
        }
    }
}
