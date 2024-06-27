use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use committable::Commitment;
use committable::Committable;
use committable::RawCommitmentBuilder;
use serde::Deserialize;
use serde::Serialize;

use super::FeeAccount;

use super::FeeInfo as OneFeeInfo;

#[derive(
    Hash,
    Clone,
    Default,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
pub struct FeeInfo(Vec<OneFeeInfo>);

impl FeeInfo {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn accounts(&self) -> Vec<FeeAccount> {
        self.0.iter().map(|entry| entry.account).collect()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, OneFeeInfo> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, OneFeeInfo> {
        self.0.iter_mut()
    }
    /// Sum of entry amounts
    pub fn amount(&self) -> Option<u64> {
        self.iter()
            .map(|fee_info| fee_info.amount().as_u64())
            .collect::<Option<Vec<u64>>>()
            .and_then(|amounts| amounts.iter().try_fold(0u64, |acc, n| acc.checked_add(*n)))
    }
    pub fn genesis() -> Self {
        Self(vec![OneFeeInfo::genesis()])
    }
}
// TODO this is likely completely wrong
impl Committable for FeeInfo {
    fn commit(&self) -> Commitment<Self> {
        let bytes: Vec<Commitment<OneFeeInfo>> =
            self.clone().map(|fee_info| fee_info.commit()).collect();
        RawCommitmentBuilder::new(&Self::tag())
            // TODO verify we want `var_size_bytes`
            .var_size_bytes(&bincode::serialize(&bytes).unwrap())
            .finalize()
    }
    fn tag() -> String {
        "FEE_INFO".into()
    }
}

impl From<Vec<OneFeeInfo>> for FeeInfo {
    fn from(items: Vec<OneFeeInfo>) -> Self {
        Self(items)
    }
}

impl Iterator for FeeInfo {
    type Item = OneFeeInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().into_iter().next()
    }
}
