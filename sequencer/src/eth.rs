use derive_more::{Add, Display, From, Into, Mul, Sub};
use ethers::types::{BlockId, BlockNumber, U64};
use sequencer_utils::{impl_serde_from_string_or_integer, ser::FromStringOrInteger};
use std::str::FromStr as _;

/// A newtype for L1 block numbers
#[derive(
    Default,
    Copy,
    Clone,
    Debug,
    Hash,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    Mul,
    From,
    Into,
)]
#[display(fmt = "{_0}")]
pub struct L1BlockNum(u64);

impl L1BlockNum {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<L1BlockNum> for BlockId {
    fn from(val: L1BlockNum) -> Self {
        BlockId::Number(BlockNumber::Number(val.0.into()))
    }
}

impl From<U64> for L1BlockNum {
    fn from(n: U64) -> Self {
        Self(n.as_u64())
    }
}

impl FromStringOrInteger for L1BlockNum {
    type Binary = u64;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Ok(Self(b))
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        Ok(Self(i))
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        if s.starts_with("0x") {
            Ok(Self(U64::from_str(&s)?.as_u64()))
        } else {
            Ok(Self(s.parse()?))
        }
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.0)
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{:#x}", self.0))
    }
}

impl_serde_from_string_or_integer!(L1BlockNum);

#[cfg(test)]
mod test {
    use super::L1BlockNum;
    impl PartialEq<L1BlockNum> for u64 {
        fn eq(&self, other: &L1BlockNum) -> bool {
            *self == other.0
        }
    }

    impl PartialEq<u64> for L1BlockNum {
        fn eq(&self, other: &u64) -> bool {
            self.0 == *other
        }
    }
}
