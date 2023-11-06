//! Implementations of the simple vote types.

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use commit::{Commitment, Committable};
use serde::{Deserialize, Serialize};

use crate::{
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey},
    },
    vote2::{HasViewNumber, Vote2},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a yes vote.
pub struct QuorumData<LEAF: Committable> {
    /// Commitment to the leaf
    pub leaf_commit: Commitment<LEAF>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a DA vote.
pub struct DAData<PAYLOAD: Committable> {
    /// Commitment to a block payload
    pub payload_commit: Commitment<PAYLOAD>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a timeout vote.
pub struct TimeoutData<TYPES: NodeType> {
    /// View the timeout is for
    pub view: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a VID vote.
pub struct VIDData<PAYLOAD: Committable> {
    /// Commitment to the block payload the VID vote is on.
    pub payload_commit: Commitment<PAYLOAD>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Pre Commit vote.
pub struct ViewSyncPreCommitData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: EncodedPublicKey,
    /// The view number we are trying to sync on
    pub round: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Commit vote.
pub struct ViewSyncCommitData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: EncodedPublicKey,
    /// The view number we are trying to sync on
    pub round: TYPES::Time,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
/// Data used for a Finalize vote.
pub struct ViewSyncFinalizeData<TYPES: NodeType> {
    /// The relay this vote is intended for
    pub relay: EncodedPublicKey,
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
pub struct SimpleVote<TYPES: NodeType, DATA: Voteable, MEMBERSHIP: Membership<TYPES>> {
    /// The signature share associated with this vote
    pub signature: (EncodedPublicKey, EncodedSignature),
    /// The leaf commitment being voted on.
    pub data: DATA,
    /// The view this vote was cast for
    pub view_number: TYPES::Time,
    /// phantom data for `MEMBERSHIP`
    _pd: PhantomData<MEMBERSHIP>,
}

impl<TYPES: NodeType, DATA: Voteable + 'static, MEMBERSHIP: Membership<TYPES>> HasViewNumber<TYPES>
    for SimpleVote<TYPES, DATA, MEMBERSHIP>
{
    fn get_view_number(&self) -> <TYPES as NodeType>::Time {
        self.view_number
    }
}

impl<TYPES: NodeType, DATA: Voteable + 'static, MEMBERSHIP: Membership<TYPES>> Vote2<TYPES>
    for SimpleVote<TYPES, DATA, MEMBERSHIP>
{
    type Commitment = DATA;
    type Membership = MEMBERSHIP;

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

impl<TYPES: NodeType, DATA: Voteable + 'static, MEMBERSHIP: Membership<TYPES>>
    SimpleVote<TYPES, DATA, MEMBERSHIP>
{
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
            _pd: PhantomData,
        }
    }
}

impl<LEAF: Committable> Committable for QuorumData<LEAF> {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Yes Vote")
            .var_size_bytes(self.leaf_commit.as_ref())
            .finalize()
    }
}

impl<PAYLOAD: Committable> Committable for DAData<PAYLOAD> {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("DA Vote")
            .var_size_bytes(self.payload_commit.as_ref())
            .finalize()
    }
}
impl<PAYLOAD: Committable> Committable for VIDData<PAYLOAD> {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("VID Vote")
            .var_size_bytes(self.payload_commit.as_ref())
            .finalize()
    }
}

/// This implements commit for all the types which contain a view and relay public key.
fn view_and_relay_commit<TYPES: NodeType, T: Committable>(
    view: TYPES::Time,
    relay: &EncodedPublicKey,
    tag: &str,
) -> Commitment<T> {
    let builder = commit::RawCommitmentBuilder::new(tag);
    builder
        .var_size_field("Relay public key", &relay.0)
        .u64(*view)
        .finalize()
}

impl<TYPES: NodeType> Committable for ViewSyncPreCommitData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, &self.relay, "View Sync Precommit")
    }
}

impl<TYPES: NodeType> Committable for ViewSyncFinalizeData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, &self.relay, "View Sync Finalize")
    }
}
impl<TYPES: NodeType> Committable for ViewSyncCommitData<TYPES> {
    fn commit(&self) -> Commitment<Self> {
        view_and_relay_commit::<TYPES, Self>(self.round, &self.relay, "View Sync Commit")
    }
}

// impl votable for all the data types in this file sealed marker should ensure nothing is accidently
// implemented for structs that aren't "voteable"
impl<V: sealed::Sealed + Committable + Clone + Serialize + Debug + PartialEq + Hash + Eq> Voteable
    for V
{
}

// Type aliases for simple use of all the main votes.  We should never see `SimpleVote` outside this file
/// Yes vote Alias
pub type QuorumVote<TYPES, LEAF, M> = SimpleVote<TYPES, QuorumData<LEAF>, M>;
/// DA vote type alias
pub type DAVote2<TYPES, M> = SimpleVote<TYPES, DAData<<TYPES as NodeType>::BlockPayload>, M>;
/// VID vote type alias
pub type VIDVote2<TYPES, M> = SimpleVote<TYPES, VIDData<<TYPES as NodeType>::BlockPayload>, M>;
/// Timeout Vote type alias
pub type TimeoutVote<TYPES, M> = SimpleVote<TYPES, TimeoutData<TYPES>, M>;
/// View Sync Commit Vote type alias
pub type ViewSyncCommitVote<TYPES, M> = SimpleVote<TYPES, ViewSyncCommitData<TYPES>, M>;
/// View Sync Pre Commit Vote type alias
pub type ViewSyncPreCommitVote<TYPES, M> = SimpleVote<TYPES, ViewSyncPreCommitData<TYPES>, M>;
/// View Sync Finalize Vote type alias
pub type ViewSyncFinalizeVote<TYPES, M> = SimpleVote<TYPES, ViewSyncFinalizeData<TYPES>, M>;
