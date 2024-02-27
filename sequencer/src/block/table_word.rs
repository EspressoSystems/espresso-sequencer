use super::{Deserialize, Serialize};
use crate::VmId;
use core::fmt;
use std::mem::size_of;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct TableWord<T, const SIZE: usize>(T);

impl TableWord<u32, 4> {
    pub const MAX: TableWord<u32, 4> = Self(u32::MAX);

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
    pub const fn to_bytes(&self) -> [u8; size_of::<u32>()] {
        self.0.to_le_bytes()
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self(u32::from_le_bytes(bytes.try_into().ok()?)))
    }
    /// Infallible constructor.
    pub fn from_bytes_array(bytes: [u8; Self::byte_len()]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }
    pub const fn byte_len() -> usize {
        size_of::<u32>()
    }

    pub fn from_usize(val: usize) -> Self {
        Self(
            val.try_into()
                .expect("usize -> TxTableEntry should succeed"),
        )
    }
}

impl fmt::Display for TableWord<u32, 4> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<usize> for TableWord<u32, 4> {
    type Error = <u32 as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value).map(Self)
    }
}
impl TryFrom<TableWord<u32, 4>> for usize {
    type Error = <usize as TryFrom<u32>>::Error;

    fn try_from(value: TableWord<u32, 4>) -> Result<Self, Self::Error> {
        usize::try_from(value.0)
    }
}

impl TryFrom<VmId> for TableWord<u32, 4> {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: VmId) -> Result<Self, Self::Error> {
        u32::try_from(value.0).map(Self)
    }
}

impl TryFrom<TableWord<u32, 4>> for VmId {
    type Error = <u64 as TryFrom<u32>>::Error;

    fn try_from(value: TableWord<u32, 4>) -> Result<Self, Self::Error> {
        Ok(Self(From::from(value.0)))
    }
}
