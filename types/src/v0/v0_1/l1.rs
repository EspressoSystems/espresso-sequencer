use committable::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::{H256, U256};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    number: u64,
    timestamp: U256,
    hash: H256,
}
