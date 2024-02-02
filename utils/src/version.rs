//! Utilities for reading version number

use hotshot_constants::Version;

/// Read the version number from a message (passed a byte vector),
/// returning `None` is there are not enough bytes.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn read_version(message: &[u8]) -> Option<Version> {
    let bytes_major = message.get(0..2)?.try_into().ok()?;
    let bytes_minor = message.get(2..4)?.try_into().ok()?;
    let major = u16::from_le_bytes(bytes_major);
    let minor = u16::from_le_bytes(bytes_minor);

    Some(Version { major, minor })
}
