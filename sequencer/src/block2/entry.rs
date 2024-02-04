use super::{Deserialize, Serialize};
use crate::VmId;
use core::fmt;
use std::mem::size_of;
use crate::block2::payload::TableLenTraits;

// Use newtype pattern so that tx table entires cannot be confused with other types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct TxTableEntry<TableLen:TableLenTraits> (TableLen);



impl<TableLen:TableLenTraits> TxTableEntry<TableLen> {
    pub const MAX: TxTableEntry<TableLen> = Self(TableLen::MAX);

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
    // TODO Philippe we can't use const functions with generic parameters
    pub const fn to_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self(TableLen::from_le_bytes(
            bytes.try_into().ok()?,
        )))
    }
    /// Infallible constructor.
    pub fn from_bytes_array(bytes: [u8; TxTableEntry::byte_len()]) -> Self {
        Self(TableLen::from_le_bytes(bytes))
    }
    pub const fn byte_len() -> usize {
        size_of::<TableLen>()
    }

    pub fn from_usize(val: usize) -> Self {
        Self(
            val.try_into()
                .expect("usize -> TxTableEntry should succeed"),
        )
    }
}

impl<TableLen:TableLenTraits> fmt::Display for TxTableEntry<TableLen> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<TableLen: TableLenTraits> TryFrom<usize> for TxTableEntry<TableLen> {
    type Error = <TableLen as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TableLen::try_from(value).map(Self)
    }
}
impl<TableLen: TableLenTraits> TryFrom<TableLen> for usize {
    type Error = <usize as TryFrom<TableLen>>::Error;

    fn try_from(value: TxTableEntry<TableLen>) -> Result<Self, Self::Error> {
        usize::try_from(value.0)
    }
}

impl<TableLen: TableLenTraits> TryFrom<VmId> for TxTableEntry<TableLen> {
    type Error = <TableLen as TryFrom<u64>>::Error;

    fn try_from(value: VmId) -> Result<Self, Self::Error> {
        TableLen::try_from(value.0).map(Self)
    }
}
impl<TableLen: TableLenTraits> TryFrom<TxTableEntry<TableLen>> for VmId {
    type Error = <u64 as TryFrom<TableLen>>::Error;

    fn try_from(value: TxTableEntry<TableLen>) -> Result<Self, Self::Error> {
        Ok(Self(From::from(value.0)))
    }
}
