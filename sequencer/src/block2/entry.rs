use super::{Deserialize, Serialize};
use crate::VmId;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use core::fmt;
use rand_distr::num_traits::FromBytes;
use std::mem::size_of;

pub(crate) trait TableEntry: Sized {
    type Word: CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Sized
        + FromBytes;
    fn max() -> usize;

    fn checked_add_mut(&mut self, rhs: Self) -> Option<()>;
    fn zero() -> Self;
    fn one() -> Self;
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Option<Self>;

    fn byte_len() -> usize;

    fn from_usize(val: usize) -> Self;
}

// Use newtype pattern so that tx table entires cannot be confused with other types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TxTableEntry(u32);

impl TableEntry for TxTableEntry {
    type Word = u32;

    fn max() -> usize {
        Self::Word::MAX as usize
    }

    /// Adds `rhs` to `self` in place. Returns `None` on overflow.
    fn checked_add_mut(&mut self, rhs: Self) -> Option<()> {
        self.0 = self.0.checked_add(rhs.0)?;
        Some(())
    }
    fn zero() -> Self {
        Self(0)
    }
    fn one() -> Self {
        Self(1)
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self(Self::Word::from_le_bytes(bytes.try_into().ok()?)))
    }

    fn byte_len() -> usize {
        size_of::<Self::Word>()
    }

    fn from_usize(val: usize) -> Self {
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
    type Error = <u32 as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value).map(Self)
    }
}
impl TryFrom<TxTableEntry> for usize {
    type Error = <usize as TryFrom<u32>>::Error;

    fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
        usize::try_from(value.0)
    }
}

impl TryFrom<VmId> for TxTableEntry {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: VmId) -> Result<Self, Self::Error> {
        u32::try_from(value.0).map(Self)
    }
}
impl TryFrom<TxTableEntry> for VmId {
    type Error = <u64 as TryFrom<u32>>::Error;

    fn try_from(value: TxTableEntry) -> Result<Self, Self::Error> {
        Ok(Self(From::from(value.0)))
    }
}
