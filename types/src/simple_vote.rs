//! Implementations of the simple vote types.

use std::{fmt::Debug, hash::Hash};

use commit::{Commitment, Committable};
use serde::{Deserialize, Serialize};

use crate::{
    data::{Leaf, VidCommitment},
    traits::{
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey},
    },
    vote::{HasViewNumber, Vote},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a yes vote.
#[serde(bound(deserialize = ""))]
pub struct QuorumData<TYPES: NodeType> {
    /// Commitment to the leaf
    pub leaf_commit: Commitment<Leaf<TYPES>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a DA vote.
pub struct DAData {
    /// Commitment to a block payload
    pub payload_commit: VidCommitment,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a timeout vote.
pub struct TimeoutData<TYPES: NodeType> {
    /// View the timeout is for
    pub view: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a VID vote.
pub struct VIDData {
    /// Commitment to the block payload the VID vote is on.
    pub payload_commit: VidCommitment,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Pre Commit vote.
pub struct ViewSyncPreCommitData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: u64,
    /// The view number we are trying to sync on
    pub round: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Commit vote.
pub struct ViewSyncCommitData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: u64,
    /// The view number we are trying to sync on
    pub round: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Finalize vote.
pub struct ViewSyncFinalizeData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: u64,
    /// The view number we are trying to sync on
    pub round: TYPES::Time,
}

/// Marker trait for data or commitments that can be voted on.
/// Only structs in this file can implement voteable.  This is enforced with the `Sealed` trait
/// Sealing this trait prevents creating new vote types outside this file.
pub trait Voteable:
    sealed::Sealed + Committable + Clone + Serialize + Debug + PartialEq + Hash + Eq
{
}

/// Sealed is used to make sure no other files can implement the Voteable trait.
/// All simple voteable types should be implemented here.  This prevents us from
/// creating/using improper types when using the vote types.
mod sealed {
    use commit::Committable;

    /// Only structs in this file can impl `Sealed`
    pub trait Sealed {}

    // TODO: Does the implement for things outside this file that are commitable?
    impl<C: Committable> Sealed for C {}
}

/// A simple yes vote over some votable type.  
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub struct SimpleVote<TYPES: NodeType, DATA: Voteable> {
    /// The signature share associated with this vote
    pub signature: (EncodedPublicKey, EncodedSignature),
    /// The leaf commitment being voted on.
    pub data: DATA,
    /// The view this vote was cast for
    pub view_number: TYPES::Time,
}

impl<TYPES: NodeType, DATA: Voteable + 'static> HasViewNumber<TYPES> for SimpleVote<TYPES, DATA> {
    fn get_view_number(&self) -> <TYPES as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType, DATA: Voteable + 'static> Vote<TYPES> for SimpleVote<TYPES, DATA> {
    type Commitment = DATA;

    fn get_signing_key(&self) -> <TYPES as NodeType>::SignatureKey {
        <TYPES::SignatureKey as SignatureKey>::from_bytes(&self.signature.0).unwrap()
    }

    fn get_signature(&self) -> EncodedSignature {
        self.signature.1.clone()
    }

    fn get_data(&self) -> &DATA {
        &self.data
    }

    fn get_data_commitment(&self) -> Commitment<DATA> {
        self.data.commit()
    }
}

impl<TYPES: NodeType, DATA: Voteable + 'static> SimpleVote<TYPES, DATA> {
    /// Creates and signs a simple vote
    pub fn create_signed_vote(
        data: DATA,
        view: TYPES::Time,
        pub_key: &TYPES::SignatureKey,
        private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let signature = TYPES::SignatureKey::sign(private_key, data.commit().as_ref());
        Self {
            signature: (pub_key.to_bytes(), signature),
            data,
            view_number: view,
        }
    }
}

impl<TYPES: NodeType> Committable for QuorumData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Yes Vote")
            .var_size_bytes(self.leaf_commit.as_ref())
            .finalize()
    }
}

impl<TYPES: NodeType> Committable for TimeoutData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Timeout Vote")
            .u64(*self.view)
            .finalize()
    }
}

impl Committable for DAData {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("DA Vote")
            .var_size_bytes(self.payload_commit.as_ref())
            .finalize()
    }
}
impl Committable for VIDData {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("VID Vote")
            .var_size_bytes(self.payload_commit.as_ref())
            .finalize()
    }
}

/// This implements commit for all the types which contain a view and relay public key.
fn view_and_relay_commit<TYPES: NodeType, T: Committable>(
    view: TYPES::Time,
    relay: u64,
    tag: &str,
) -> Commitment<T> {
    let builder = commit::RawCommitmentBuilder::new(tag);
    builder.u64(*view).u64(relay).finalize()
}

impl<TYPES: NodeType> Committable for ViewSyncPreCommitData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, self.relay, "View Sync Precommit")
    }
}

impl<TYPES: NodeType> Committable for ViewSyncFinalizeData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, self.relay, "View Sync Finalize")
    }
}
impl<TYPES: NodeType> Committable for ViewSyncCommitData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, self.relay, "View Sync Commit")
    }
}

// impl votable for all the data types in this file sealed marker should ensure nothing is accidently
// implemented for structs that aren't "voteable"
impl<V: sealed::Sealed + Committable + Clone + Serialize + Debug + PartialEq + Hash + Eq> Voteable
    for V
{
}

// Type aliases for simple use of all the main votes.  We should never see `SimpleVote` outside this file
/// Quorum vote Alias
pub type QuorumVote<TYPES> = SimpleVote<TYPES, QuorumData<TYPES>>;
/// DA vote type alias
pub type DAVote<TYPES> = SimpleVote<TYPES, DAData<<TYPES as NodeType>::BlockPayload>>;
/// VID vote type alias
pub type VIDVote<TYPES> = SimpleVote<TYPES, VIDData<<TYPES as NodeType>::BlockPayload>>;
/// Timeout Vote type alias
pub type TimeoutVote<TYPES> = SimpleVote<TYPES, TimeoutData<TYPES>>;
/// View Sync Commit Vote type alias
pub type ViewSyncCommitVote<TYPES> = SimpleVote<TYPES, ViewSyncCommitData<TYPES>>;
/// View Sync Pre Commit Vote type alias
pub type ViewSyncPreCommitVote<TYPES> = SimpleVote<TYPES, ViewSyncPreCommitData<TYPES>>;
/// View Sync Finalize Vote type alias
pub type ViewSyncFinalizeVote<TYPES> = SimpleVote<TYPES, ViewSyncFinalizeData<TYPES>>;
