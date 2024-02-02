#[cfg(test)]
use hotshot_constants::Version;
use hotshot_utils::version::read_version;

#[test]
/// Check that the version number is read correctly.
fn read_version_1() {
    let bytes: [u8; 6] = [0, 0, 1, 0, 4, 9];
    let version = Version { major: 0, minor: 1 };
    assert_eq!(read_version(&bytes), Some(version));
}

#[test]
/// Check that the version number is read correctly.
fn read_version_2() {
    let bytes: [u8; 4] = [9, 0, 3, 0];
    let version = Version { major: 9, minor: 3 };
    assert_eq!(read_version(&bytes), Some(version));
}

#[test]
/// Check that `None` is returned if there are not enough bytes.
fn read_version_insufficient_bytes_1() {
    let bytes: [u8; 3] = [0, 0, 0];
    assert_eq!(read_version(&bytes), None);
}

#[test]
/// Check that `None` is returned if there are not enough bytes.
fn read_version_insufficient_bytes_2() {
    let bytes: [u8; 0] = [];
    assert_eq!(read_version(&bytes), None);
}
