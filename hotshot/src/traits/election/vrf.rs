use hotshot_types::traits::signature_key::EncodedPublicKey;

#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData, num::NonZeroU64};

// TODO wrong palce for this
/// the sortition committee size parameter
pub const SORTITION_PARAMETER: u64 = 100;

// TODO compatibility this function's impl into a trait
// TODO do we necessariy want the units of stake to be a u64? or generics
/// The stake table for VRFs
#[derive(Serialize, Deserialize, Debug)]
pub struct VRFStakeTable<VRF, VRFHASHER, VRFPARAMS> {
    /// the mapping of id -> stake
    mapping: BTreeMap<EncodedPublicKey, NonZeroU64>,
    /// total stake present
    total_stake: NonZeroU64,
    /// PhantomData
    _pd: PhantomData<(VRF, VRFHASHER, VRFPARAMS)>,
}

impl<VRF, VRFHASHER, VRFPARAMS> Clone for VRFStakeTable<VRF, VRFHASHER, VRFPARAMS> {
    fn clone(&self) -> Self {
        Self {
            mapping: self.mapping.clone(),
            total_stake: self.total_stake,
            _pd: PhantomData,
        }
    }
}
