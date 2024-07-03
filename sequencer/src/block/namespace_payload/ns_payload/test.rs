use super::NsPayloadOwned;
use crate::{
    block::{namespace_payload::NsPayloadBuilder, uint_bytes::usize_to_bytes},
    NamespaceId,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

#[test]
fn ns_payload_len() {
    setup_logging();
    setup_backtrace();
    let ns_id = NamespaceId::from(69); // dummy

    // ordinary valid ns_payload
    {
        let ns_payload = ns_payload_with_body(&[10, 20, 30], 30);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(txs[1].payload(), [10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(txs[2].payload(), [20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
    }

    // large payload has wasted space
    {
        let ns_payload = ns_payload_with_body(&[10, 20, 30], 40);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(txs[1].payload(), [10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(txs[2].payload(), [20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);

        // inaccessible payload bytes
        assert_eq!(ns_payload.0.len(), 56);
        assert_eq!(ns_payload.0[46..], [30, 31, 32, 33, 34, 35, 36, 37, 38, 39]);
    }

    // small payload truncates txs
    {
        // final tx partly truncated by short payload
        let ns_payload = ns_payload_with_body(&[10, 20, 30], 25);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(txs[1].payload(), [10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(txs[2].payload(), [20, 21, 22, 23, 24]);

        // final tx totally truncated, next-to-final partly truncated
        let ns_payload = ns_payload_with_body(&[10, 20, 30], 15);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(txs[1].payload(), [10, 11, 12, 13, 14]);
        assert_eq!(txs[2].payload().len(), 0);

        // all txs totally truncated
        let ns_payload = NsPayloadOwned(tx_table(&[10, 20, 30]));
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload().len(), 0);
        assert_eq!(txs[1].payload().len(), 0);
        assert_eq!(txs[2].payload().len(), 0);
    }

    // small payload can't fit the whole tx table
    {
        // final tx table entry partly truncated by short payload
        let ns_payload = tx_table_truncated(&[10, 20, 30], tx_table_byte_len(3) - 1);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 2);
        assert_eq!(txs[0].payload().len(), 0);
        assert_eq!(txs[1].payload().len(), 0);

        // final tx table entry totally truncated, next-to-final partly truncated
        let ns_payload = tx_table_truncated(&[10, 20, 30], tx_table_byte_len(2) - 1);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].payload().len(), 0);

        // all tx table entries totally truncated (tx table header only)
        let ns_payload =
            tx_table_truncated(&[10, 20, 30], NsPayloadBuilder::tx_table_header_byte_len());
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 0);
    }

    // extremely small payload can't even fit the tx table header
    {
        for i in 0..NsPayloadBuilder::tx_table_header_byte_len() {
            let ns_payload = tx_table_truncated(&[10, 20, 30], i);
            assert_eq!(ns_payload.iter().count(), 0);
        }
    }
}

fn tx_table(entries: &[usize]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(usize_to_bytes::<
        { NsPayloadBuilder::tx_table_header_byte_len() },
    >(entries.len()));
    for entry in entries {
        bytes.extend(usize_to_bytes::<
            { NsPayloadBuilder::tx_table_entry_byte_len() },
        >(*entry));
    }
    bytes
}

fn ns_payload_with_body(entries: &[usize], body_byte_len: usize) -> NsPayloadOwned {
    let mut bytes = tx_table(entries);
    bytes.append(
        &mut (0..body_byte_len)
            .map(|i| (i % u8::MAX as usize) as u8)
            .collect(),
    );
    NsPayloadOwned(bytes)
}

fn tx_table_truncated(entries: &[usize], tx_table_byte_len: usize) -> NsPayloadOwned {
    let mut bytes = tx_table(entries);
    bytes.truncate(tx_table_byte_len);
    NsPayloadOwned(bytes)
}

fn tx_table_byte_len(num_entries: usize) -> usize {
    NsPayloadBuilder::tx_table_header_byte_len()
        + num_entries * NsPayloadBuilder::tx_table_entry_byte_len()
}
