#![allow(clippy::module_name_repetitions, clippy::type_complexity)]
use bincode::{
    config::{
        LittleEndian, RejectTrailing, VarintEncoding, WithOtherEndian, WithOtherIntEncoding,
        WithOtherLimit, WithOtherTrailing,
    },
    DefaultOptions, Options,
};

/// For the wire format, we use bincode with the following options:
///   - No upper size limit
///   - Litte endian encoding
///   - Varint encoding
///   - Reject trailing bytes
#[must_use]
pub fn bincode_opts() -> WithOtherTrailing<
    WithOtherIntEncoding<
        WithOtherEndian<WithOtherLimit<DefaultOptions, bincode::config::Infinite>, LittleEndian>,
        VarintEncoding,
    >,
    RejectTrailing,
> {
    bincode::DefaultOptions::new()
        .with_no_limit()
        .with_little_endian()
        .with_varint_encoding()
        .reject_trailing_bytes()
}
