use super::{NsTable, NsTableBuilder, NsTableValidationError, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN};
use crate::{
    block::uint_bytes::{u32_max_from_byte_len, usize_max_from_byte_len},
    NamespaceId,
};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use rand::{Rng, RngCore};

#[test]
fn random_valid() {
    setup_logging();
    setup_backtrace();
    let mut rng = jf_utils::test_rng();

    for num_entries in 0..20 {
        random_valid_ns_table(num_entries, &mut rng)
            .validate()
            .unwrap();
    }
}

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
    expect_num_bytes_invalid(ns_table, NsTableBuilder::header_byte_len(), &mut rng);

    // Add enough bytes for a new entry.
    expect_num_bytes_invalid(
        random_valid_ns_table(20, &mut rng),
        NsTableBuilder::entry_byte_len(),
        &mut rng,
    );

    // Helper fn: add 1 byte to the `ns_table` `num_bytes` times. Expect
    // invalidity in all but the final time.
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

#[test]
fn monotonic_increase() {
    setup_logging();
    setup_backtrace();

    // Duplicate namespace ID
    two_entries_ns_table((1, 1), (1, 2), false);

    // Decreasing namespace ID
    two_entries_ns_table((1, 1), (0, 2), false);

    // Duplicate offset
    two_entries_ns_table((1, 1), (2, 1), false);

    // Decreasing offset
    two_entries_ns_table((1, 1), (2, 0), false);

    // Everything increasing
    two_entries_ns_table((1, 1), (2, 2), true);

    // Helper fn: build a 2-entry NsTable, assert success/failure
    fn two_entries_ns_table(entry1: (u32, usize), entry2: (u32, usize), expect_success: bool) {
        let mut ns_table_builder = NsTableBuilder::new();
        ns_table_builder.append_entry(NamespaceId::from(entry1.0), entry1.1);
        ns_table_builder.append_entry(NamespaceId::from(entry2.0), entry2.1);
        let ns_table = ns_table_builder.into_ns_table();
        if expect_success {
            ns_table.validate().unwrap();
        } else {
            assert_eq!(
                ns_table.validate().unwrap_err(),
                NsTableValidationError::NonIncreasingEntries
            );
        }
    }
}

fn random_valid_ns_table<R>(num_entries: usize, rng: &mut R) -> NsTable
where
    R: RngCore,
{
    let (offset_max_increment, ns_id_max_increment) = if num_entries == 0 {
        (0, 0)
    } else {
        let num_entries_u32: u32 = num_entries.try_into().unwrap();
        (
            usize_max_from_byte_len(NS_OFFSET_BYTE_LEN) / num_entries,
            u32_max_from_byte_len(NS_ID_BYTE_LEN) / num_entries_u32,
        )
    };

    let mut ns_id = 0;
    let mut offset = 0;
    let mut ns_table_builder = NsTableBuilder::new();
    for _ in 0..num_entries {
        // ns_id, offset must increase monotonically
        ns_id += rng.gen_range(1..=ns_id_max_increment);
        offset += rng.gen_range(1..=offset_max_increment);
        ns_table_builder.append_entry(NamespaceId::from(ns_id), offset);
    }
    ns_table_builder.into_ns_table()
}
