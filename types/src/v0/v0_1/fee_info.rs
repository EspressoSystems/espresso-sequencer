use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use derive_more::{Add, Display, From, Into, Mul, Sub};
use ethers::prelude::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(
    Hash,
    Copy,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
/// `FeeInfo` holds data related to builder fees.
pub struct FeeInfo {
    pub(crate) account: FeeAccount,
    pub(crate) amount: FeeAmount,
}

// New Type for `U256` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    Deserialize,
    Serialize,
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
pub struct FeeAmount(pub(crate) U256);

// New Type for `Address` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Into,
)]
#[display(fmt = "{_0:x}")]
pub struct FeeAccount(pub(crate) Address);
