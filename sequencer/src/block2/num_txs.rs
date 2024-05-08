use crate::block2::{
    ns_payload,
    payload_bytes::{usize_from_bytes, usize_to_bytes, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The number of txs declared in a tx table.
///
/// Custom serialization and helper methods.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NumTxs(usize);

impl Serialize for NumTxs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NumTxs {
    fn deserialize<D>(deserializer: D) -> Result<NumTxs, D::Error>
    where
        D: Deserializer<'de>,
    {
        <[u8; NUM_TXS_BYTE_LEN] as Deserialize>::deserialize(deserializer)
            .map(|bytes| NumTxs(usize_from_bytes::<NUM_TXS_BYTE_LEN>(&bytes)))
    }
}

impl NumTxs {
    /// Byte length of a tx table with `self` number of entries.
    ///
    /// "Unchecked" because this quantity might exceed the byte length of
    /// the namespace in which it resides.
    ///
    /// TODO move up to ns_payload or ns_payload_range?
    pub fn tx_table_byte_len_unchecked(&self) -> usize {
        self.0
            .saturating_mul(TX_OFFSET_BYTE_LEN)
            .saturating_add(NUM_TXS_BYTE_LEN)
    }

    /// Infallible serialization.
    ///
    /// TODO used only in [`tx_proof`] module. Delete this method and make
    /// everyone use serde if they want bytes?
    ///
    /// TODO what's the idiomatic way to return an abstraction over a reference
    /// vs owned value? eg. Suppose in the future the underlying representation
    /// of a [`NumTxs`] switches from `usize` to `[u8; N]`. In that case I
    /// prefer to return a reference `&[u8; N]` instead of a copy `[u8; N]`. I
    /// guess it's just `impl Borrow<[u8; N]>` or `Cow<[u8; N]>`? I don't like
    /// `Cow` because the return value variant might change (`Borrowed` vs
    /// `Owned`) when I change the underlying implementation, which leaks info
    /// about the underlying implementation. (Though I guess we can explicitly
    /// state that it could be either.) OTOH `Borrowed` forces the user to clone
    /// if they want an owned value, but I guess we can rely on the compiler to
    /// optimize away any `borrow().clone()` right?
    pub fn as_bytes(&self) -> [u8; NUM_TXS_BYTE_LEN] {
        usize_to_bytes(self.0)
    }

    /// TODO explain: [`ns_payload::A`] arg allows access to this method only
    /// from within [`ns_payload`] module.
    pub fn from_bytes(_: ns_payload::A, bytes: &[u8]) -> Self {
        Self(usize_from_bytes::<NUM_TXS_BYTE_LEN>(bytes))
    }
    pub fn from_usize(_: ns_payload::A, n: usize) -> Self {
        Self(n)
    }
    pub fn as_usize(&self, _: ns_payload::A) -> usize {
        self.0
    }
}
