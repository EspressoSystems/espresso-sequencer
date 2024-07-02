use super::NsPayloadOwned;
use crate::{
    block::namespace_payload::NsPayloadBuilder, block::uint_bytes::usize_to_bytes, NamespaceId,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

#[test]
fn short_ns_payload() {
    setup_logging();
    setup_backtrace();
    let ns_id = NamespaceId::from(69); // dummy

    // ordinary valid ns_payload
    {
        let entries = [10, 20, 30];
        let ns_payload = tx_table_with_body(&entries, *entries.iter().max().unwrap_or(&0));
        let mut tx_count = 0;
        let mut prev_entry = 0;
        for (i, tx_index) in ns_payload.iter().enumerate() {
            let tx = ns_payload.export_tx(&ns_id, &tx_index).unwrap();
            assert_eq!(tx.payload().len(), entries[i] - prev_entry);
            tx_count += 1;
            prev_entry = entries[i];
        }
        assert_eq!(tx_count, entries.len());
    }

    // tx table offsets out of bounds for payload
    {
        // final tx partly truncated by short payload
        let ns_payload = tx_table_with_body(&[10, 20, 30], 25);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload().len(), 10);
        assert_eq!(txs[1].payload().len(), 10);
        assert_eq!(txs[2].payload().len(), 5);

        // final tx totally truncated, next-to-final partly truncated
        let ns_payload = tx_table_with_body(&[10, 20, 30], 15);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload().len(), 10);
        assert_eq!(txs[1].payload().len(), 5);
        assert_eq!(txs[2].payload().len(), 0);

        // all txs totally truncated
        let ns_payload = tx_table(&[10, 20, 30]);
        let txs = ns_payload.export_all_txs(&ns_id);
        assert_eq!(txs.len(), 3);
        assert_eq!(txs[0].payload().len(), 0);
        assert_eq!(txs[1].payload().len(), 0);
        assert_eq!(txs[2].payload().len(), 0);
    }

    // small payload can't fit the whole tx table
    // extremely small payload can't even fit the tx table header
}

fn tx_table(entries: &[usize]) -> NsPayloadOwned {
    let mut bytes = Vec::new();
    bytes.extend(usize_to_bytes::<
        { NsPayloadBuilder::tx_table_header_byte_len() },
    >(entries.len()));
    for entry in entries {
        bytes.extend(usize_to_bytes::<
            { NsPayloadBuilder::tx_table_entry_byte_len() },
        >(*entry));
    }
    NsPayloadOwned(bytes)
}

fn tx_table_with_body(entries: &[usize], body_byte_len: usize) -> NsPayloadOwned {
    let mut bytes = tx_table(entries).0;
    bytes.extend(vec![42; body_byte_len]);
    NsPayloadOwned(bytes)
}
