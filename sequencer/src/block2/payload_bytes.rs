//! Low-level utils for reading from and writing to the binary block payload.

use crate::NamespaceId;
use paste::paste;
use std::mem::size_of;

pub const NUM_TXS_BYTE_LEN: usize = 4;
pub const TX_OFFSET_BYTE_LEN: usize = 4;
pub const NUM_NSS_BYTE_LEN: usize = NUM_TXS_BYTE_LEN;
pub const NS_OFFSET_BYTE_LEN: usize = TX_OFFSET_BYTE_LEN;
pub const NS_ID_BYTE_LEN: usize = 4;

/// Deserialize `bytes` into a count of transactions (`usize`).
///
/// # Panics
/// If `bytes.len()` exceeds [`NUM_TXS_BYTE_LEN`].
pub fn num_txs_from_bytes(bytes: &[u8]) -> usize {
    usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes)
}

/// Serialize `tx_offset` into [`TX_OFFSET_BYTE_LEN`] bytes.
///
/// # Panics
/// If `tx_offset` cannot fit into [`TX_OFFSET_BYTE_LEN`] bytes.
pub fn tx_offset_as_bytes(tx_offset: usize) -> [u8; TX_OFFSET_BYTE_LEN] {
    usize_to_bytes(tx_offset)
}

/// Deserialize `bytes` into a transaction offset (`usize`).
///
/// # Panics
/// If `bytes.len()` exceeds [`TX_OFFSET_BYTE_LEN`].
pub fn tx_offset_from_bytes(bytes: &[u8]) -> usize {
    usize_from_bytes::<TX_OFFSET_BYTE_LEN>(bytes)
}

/// Serialize `num_nss` into [`NUM_NSS_BYTE_LEN`] bytes.
///
/// # Panics
/// If `num_nss` cannot fit into [`NUM_NSS_BYTE_LEN`] bytes.
pub fn num_nss_as_bytes(num_nss: usize) -> [u8; NUM_NSS_BYTE_LEN] {
    usize_to_bytes(num_nss)
}

/// Deserialize `bytes` into a count of namespaces (`usize`).
///
/// # Panics
/// If `bytes.len()` exceeds [`NUM_NSS_BYTE_LEN`].
pub fn num_nss_from_bytes(bytes: &[u8]) -> usize {
    usize_from_bytes::<NUM_NSS_BYTE_LEN>(bytes)
}

/// Serialize `ns_offset` into [`NS_OFFSET_BYTE_LEN`] bytes.
///
/// # Panics
/// If `ns_offset` cannot fit into [`NS_OFFSET_BYTE_LEN`] bytes.
pub fn ns_offset_as_bytes(ns_offset: usize) -> [u8; NS_OFFSET_BYTE_LEN] {
    usize_to_bytes(ns_offset)
}

/// Deserialize `bytes` into a namespace offset (`usize`).
///
/// # Panics
/// If `bytes.len()` exceeds [`NS_OFFSET_BYTE_LEN`].
pub fn ns_offset_from_bytes(bytes: &[u8]) -> usize {
    usize_from_bytes::<NS_OFFSET_BYTE_LEN>(bytes)
}

/// Serialize `ns_id` into [`NS_ID_BYTE_LEN`] bytes.
///
/// # Panics
/// If `ns_id` cannot fit into [`NS_ID_BYTE_LEN`] bytes.
///
/// TODO I'm cheating by converting `NamespaceId` via `u64::from`, which is
/// available only because `NamespaceId` derives `From`. (Not sure it should
/// be doing that. What's the point of the newtype?). Maybe I should instead
/// use serialization provided by `NamespaceId`? The problem is that
/// serialization is not ergonomic compared to mine here, which is
/// infallible and returns a constant-size array.
pub fn ns_id_as_bytes(ns_id: NamespaceId) -> [u8; NS_ID_BYTE_LEN] {
    u64_to_bytes(u64::from(ns_id))
}

/// Deserialize `bytes` into a [`NamespaceId`].
///
/// # Panics
/// If `bytes.len()` exceeds [`NS_ID_BYTE_LEN`].
pub fn ns_id_from_bytes(bytes: &[u8]) -> NamespaceId {
    NamespaceId::from(u64_from_bytes::<NS_ID_BYTE_LEN>(bytes))
}

// Use an ugly macro because it's difficult or impossible to be generic over
// primitive types such as `usize`, `u64`.
macro_rules! uint_bytes_impl {
        ($T:ty) => {
            paste! {
                /// Serialize `n` into `BYTE_LEN` bytes in little-endian form, padding with
                /// 0 as needed.
                ///
                /// # Panics
                /// If `n` cannot fit into `BYTE_LEN` bytes.
                pub fn [<$T _to_bytes>]<const BYTE_LEN: usize>(n: $T) -> [u8; BYTE_LEN] {
                    if size_of::<$T>() > BYTE_LEN {
                        assert!(
                            n <= [<$T _max_from_byte_len>](BYTE_LEN),
                            "n {n} cannot fit into {BYTE_LEN} bytes"
                        );
                        n.to_le_bytes()[..BYTE_LEN].try_into().unwrap() // panic is impossible
                    } else {
                        // convert `n` to bytes and pad with 0
                        let mut result = [0; BYTE_LEN];
                        result[..size_of::<$T>()].copy_from_slice(&n.to_le_bytes()[..]);
                        result
                    }
                }

                /// Deserialize `bytes` in little-endian form into a `$T`, padding with 0
                /// as needed.
                ///
                /// # Panics
                /// If `bytes.len()` is too large to fit into a `$T`.
                pub fn [<$T _from_bytes>]<const BYTE_LEN: usize>(bytes: &[u8]) -> $T {
                    assert!(bytes.len() <= BYTE_LEN, "bytes len {} exceeds BYTE_LEN {BYTE_LEN}", bytes.len());
                    assert!(
                        BYTE_LEN <= size_of::<$T>(),
                        "BYTE_LEN {BYTE_LEN} cannot fit into {}",
                        stringify!($T)
                    );
                    let mut [<$T _bytes>] = [0; size_of::<$T>()];
                    [<$T _bytes>][..bytes.len()].copy_from_slice(bytes);
                    $T::from_le_bytes([<$T _bytes>])
                }

                /// Return the largest `$T` value that can fit into `byte_len` bytes.
                const fn [<$T _max_from_byte_len>](byte_len: usize) -> $T {
                    if byte_len >= size_of::<$T>() {
                        $T::MAX
                    } else {
                        // overflow cannot occur because `byte_len < size_of::<$T>()`
                        (1 << (byte_len * 8)) - 1
                    }
                }
            }
        };
    }

uint_bytes_impl!(usize);
uint_bytes_impl!(u64);

#[cfg(test)]
mod test {
    use fluent_asserter::prelude::*;
    use paste::paste;
    use std::mem::size_of;

    macro_rules! uint_bytes_test_impl {
            ($T:ty) => {
                paste! {
                    use super::{[<$T _max_from_byte_len>], [<$T _to_bytes>], [<$T _from_bytes>]};

                    #[test]
                    fn [<$T _max_from_byte_len_correctness>]() {
                        // test byte lengths 0 to size_of::<$T>()
                        let mut bytes = [0; size_of::<$T>()];
                        assert_eq!([<$T _max_from_byte_len>](0), 0);
                        for i in 0..bytes.len() {
                            bytes[i] = 0xff;
                            assert_eq!([<$T _max_from_byte_len>](i + 1).to_le_bytes(), bytes);
                        }

                        // test byte lengths size_of::<$T>() to twice that length
                        for i in size_of::<$T>()..2 * size_of::<$T>() {
                            assert_eq!([<$T _max_from_byte_len>](i + 1), $T::MAX);
                        }
                    }

                    #[test]
                    fn [<$T _to_bytes_correctness>]() {
                        // byte length 0
                        assert_eq!([<$T _to_bytes>](0), [0; 0]);
                        assert_that_code!(|| [<$T _to_bytes>]::<0>(1)).panics();

                        // byte length 1
                        assert_eq!([<$T _to_bytes>](0), [0; 1]);
                        assert_eq!([<$T _to_bytes>](255), [255; 1]);
                        assert_that_code!(|| [<$T _to_bytes>]::<1>(256)).panics();

                        // byte length 2
                        assert_eq!([<$T _to_bytes>](0), [0; 2]);
                        assert_eq!([<$T _to_bytes>](65535), [255; 2]);
                        assert_that_code!(|| [<$T _to_bytes>]::<2>(65536)).panics();

                        // byte length size_of::<$T>()
                        assert_eq!([<$T _to_bytes>](0), [0; size_of::<$T>()]);
                        assert_eq!([<$T _to_bytes>]($T::MAX), [255; size_of::<$T>()]);

                        // byte length size_of::<$T>() + 1
                        assert_eq!([<$T _to_bytes>](0), [0; size_of::<$T>() + 1]);
                        let [<$T _max_bytes>] = {
                            let mut bytes = [255; size_of::<$T>() + 1];
                            bytes[bytes.len() - 1] = 0;
                            bytes
                        };
                        assert_eq!([<$T _to_bytes>]($T::MAX), [<$T _max_bytes>]);
                    }

                    #[test]
                    fn [<$T _from_bytes_correctness>]() {
                        let bytes = [255; size_of::<$T>() + 1];

                        // It would be nice to iterate through
                        // `0..size_of::<$T>()` but this is not possible with
                        // const generics for `[<$T _from_bytes>]`. We could
                        // use `seq-macro` crate but it requires an integer
                        // literal whereas our range includes `size_of::<$T>()`.
                        //
                        // Instead we just hard code four constants:
                        // `0`, `1`, `size_of::<$T>() - 1`, `size_of::<$T>()`.
                        assert_eq!(
                            [<$T _from_bytes>]::<0>(&bytes[..0]),
                            [<$T _max_from_byte_len>](0)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<1>(&bytes[..1]),
                            [<$T _max_from_byte_len>](1)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>() - 1}>(&bytes[..size_of::<$T>() - 1]),
                            [<$T _max_from_byte_len>](size_of::<$T>() - 1)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..size_of::<$T>()]),
                            [<$T _max_from_byte_len>](size_of::<$T>())
                        );

                        assert_that_code!(|| [<$T _from_bytes>]::<{size_of::<$T>() + 1}>(&bytes[..])).panics();
                    }

                    #[test]
                    fn [<$T _from_bytes_allows_smaller_byte_lens>]() {
                        // This test same as `xxx_from_bytes_correctness` except
                        // we set the const param `BYTE_LEN` to
                        // `size_of::<$T>()` in all cases. Why? To ensure that
                        // `xxx_from_bytes` allows its arg to have length
                        // smaller than `BYTE_LEN`.
                        let bytes = [255; size_of::<$T>() + 1];

                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..0]),
                            [<$T _max_from_byte_len>](0)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..1]),
                            [<$T _max_from_byte_len>](1)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..size_of::<$T>() - 1]),
                            [<$T _max_from_byte_len>](size_of::<$T>() - 1)
                        );
                        assert_eq!(
                            [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..size_of::<$T>()]),
                            [<$T _max_from_byte_len>](size_of::<$T>())
                        );

                        assert_that_code!(|| [<$T _from_bytes>]::<{size_of::<$T>()}>(&bytes[..])).panics();
                    }
                }
            };
        }

    uint_bytes_test_impl!(usize);
    uint_bytes_test_impl!(u64);
}
