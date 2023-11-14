pub struct BlockPayload {
    payload: Vec<u8>,
    // tx_ranges: Vec<Range<u32>>, // not convinced we need this
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tx {
    // namespace: u32, // TODO how many bytes for namespace id?
    payload: Vec<u8>,
}

impl BlockPayload {
    fn build(txs: impl IntoIterator<Item = Tx>) -> Self {
        // tx_table[i] is the end index (exclusive) of the ith tx
        // so that the payload bytes of the ith tx is
        // tx_bodies[tx_table[i-1]..tx_table[i]].
        // edge case: tx_table[-1] is defined as 0.
        let mut tx_table = Vec::new();

        // concatenation of all tx payloads
        let mut tx_bodies = Vec::new();

        // build tx_table, tx_bodies
        let mut end: u32 = 0;
        for tx in txs.into_iter() {
            // TODO idiomatic usize -> u32 conversion?
            let len: u32 = tx
                .payload
                .len()
                .try_into()
                .expect("tx byte length should fit into u32");

            end = end
                .checked_add(len)
                .expect("total byte length of all tx bodies should fit into u32");

            // TODO to_be_bytes or to_le_bytes?
            tx_table.extend(end.to_be_bytes());
            tx_bodies.extend(tx.payload);
        }

        let tx_table_len: u32 = tx_table
            .len()
            .try_into()
            .expect("tx_table len should fit into u32");
        let mut payload = Vec::new();
        payload.extend(tx_table_len.to_be_bytes());
        payload.extend(tx_table);
        payload.extend(tx_bodies);
        Self { payload }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::{BlockPayload, Tx};

    #[test]
    fn build_basic_correctness() {
        // play with this
        let tx_payloads = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18, 19, 20],
        ];

        // other things as a function of the above
        let txs = tx_payloads.iter().cloned().map(|payload| Tx { payload });
        let tx_payload_lengths: Vec<u32> = tx_payloads
            .iter()
            .map(|payload| payload.len().try_into().unwrap())
            .collect();

        let block = BlockPayload::build(txs);

        // test tx table length
        let (len, payload) = block.payload.split_at(size_of::<u32>());
        let len = u32::from_be_bytes(len.try_into().unwrap());
        assert_eq!(len, tx_payloads.len() as u32);

        // test tx table contents
        let (tx_table, payload) = payload.split_at(tx_payloads.len() * size_of::<u32>());
        let tx_table: Vec<u32> = tx_table
            .chunks(size_of::<u32>())
            .map(|len_bytes| u32::from_be_bytes(len_bytes.try_into().unwrap()))
            .collect();
        assert_eq!(tx_table, tx_payload_lengths);

        // test block payload body
        let tx_payloads: Vec<u8> = tx_payloads.into_iter().flatten().collect();
        assert_eq!(payload, tx_payloads);
    }
}
