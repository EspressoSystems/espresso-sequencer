use super::{NsTable, NsTableBuilder, NsTableValidationError, NS_OFFSET_BYTE_LEN};
use crate::{block::uint_bytes::usize_max_from_byte_len, NamespaceId};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use rand::{Rng, RngCore};

#[test]
fn byte_len() {
    setup_logging();
    setup_backtrace();
    let mut rng = jf_utils::test_rng();

    // Extremely small byte lengths should get rejected.
    let ns_table = NsTable { bytes: Vec::new() };
    assert_eq!(
        ns_table.validate().unwrap_err(),
        NsTableValidationError::InvalidByteLen
    );
    expect_num_bytes_invalid(
        ns_table,
        NsTableBuilder::fixed_overhead_byte_len(),
        &mut rng,
    );

    // Add enough bytes for a new entry.
    let ns_table = random_valid_ns_table(20, &mut rng);
    ns_table.validate().unwrap();
    expect_num_bytes_invalid(ns_table, NsTableBuilder::ns_overhead_byte_len(), &mut rng);

    // Add 1 byte to the `ns_table` `num_bytes` times. Expect invalidity in all
    // but the final time.
    fn expect_num_bytes_invalid<R>(mut ns_table: NsTable, num_bytes: usize, rng: &mut R)
    where
        R: RngCore,
    {
        for i in 0..num_bytes {
            ns_table.bytes.push(rng.gen());
            if i == num_bytes - 1 {
                break; // final iteration: no error expected
            }
            assert_eq!(
                ns_table.validate().unwrap_err(),
                NsTableValidationError::InvalidByteLen
            );
        }
        ns_table.validate().unwrap();
    }
}

fn random_valid_ns_table<R>(num_entries: usize, rng: &mut R) -> NsTable
where
    R: RngCore,
{
    let max_offset = if num_entries == 0 {
        0
    } else {
        usize_max_from_byte_len(NS_OFFSET_BYTE_LEN) / num_entries
    };
    let mut ns_table_builder = NsTableBuilder::new();
    for _ in 0..num_entries {
        ns_table_builder.append_entry(NamespaceId::random(rng), rng.gen_range(1..=max_offset));
    }
    ns_table_builder.into_ns_table()
}
