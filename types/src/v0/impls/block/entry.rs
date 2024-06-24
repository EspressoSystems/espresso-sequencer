use crate::{NamespaceId, TxTableEntry, TxTableEntryWord};
use core::fmt;
use std::mem::size_of;

impl TxTableEntry {
    pub const MAX: TxTableEntry = Self(TxTableEntryWord::MAX);

    /// Adds `rhs` to `self` in place. Returns `None` on overflow.
    pub fn checked_add_mut(&mut self, rhs: Self) -> Option<()> {
        self.0 = self.0.checked_add(rhs.0)?;
        Some(())
    }
    pub const fn zero() -> Self {
        Self(0)
    }
    pub const fn one() -> Self {
        Self(1)
    }
    pub const fn to_bytes(&self) -> [u8; size_of::<TxTableEntryWord>()] {
        self.0.to_le_bytes()
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self(TxTableEntryWord::from_le_bytes(
            bytes.try_into().ok()?,
        )))
    }
    /// Infallible constructor.
    pub fn from_bytes_array(bytes: [u8; TxTableEntry::byte_len()]) -> Self {
        Self(TxTableEntryWord::from_le_bytes(bytes))
    }
    pub const fn byte_len() -> usize {
        size_of::<TxTableEntryWord>()
    }

    pub fn from_usize(val: usize) -> Self {
        Self(
            val.try_into()
                .expect("usize -> TxTableEntry should succeed"),
        )
    }
}

impl fmt::Display for TxTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<usize> for TxTableEntry {
    type Error = <TxTableEntryWord as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TxTableEntryWord::try_from(value).map(Self)
    }
}
impl TryFrom<TxTableEntry> for usize {
    type Error = <usize as TryFrom<TxTableEntryWord>>::Error;

    fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
        usize::try_from(value.0)
    }
}

impl TryFrom<NamespaceId> for TxTableEntry {
    type Error = <TxTableEntryWord as TryFrom<u64>>::Error;

    fn try_from(value: NamespaceId) -> Result<Self, Self::Error> {
        TxTableEntryWord::try_from(u64::from(value)).map(Self)
    }
}
impl TryFrom<TxTableEntry> for NamespaceId {
    type Error = <u64 as TryFrom<TxTableEntryWord>>::Error;

    fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
        Ok((value.0 as u64).into())
    }
}
