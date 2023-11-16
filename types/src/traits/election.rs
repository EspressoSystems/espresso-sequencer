//! The election trait, used to decide which node is the leader and determine if a vote is valid.

// Needed to avoid the non-biding `let` warning.
#![allow(clippy::let_underscore_untyped)]

use super::{
    node_implementation::{NodeImplementation, NodeType},
    signature_key::EncodedSignature,
};
use crate::data::Leaf;

use crate::traits::{
    network::{CommunicationChannel, NetworkMsg},
    signature_key::SignatureKey,
};
use bincode::Options;
use commit::{Commitment, CommitmentBounds, Committable};
use derivative::Derivative;

use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{collections::BTreeSet, fmt::Debug, hash::Hash, marker::PhantomData, num::NonZeroU64};

/// Error for election problems
#[derive(Snafu, Debug)]
pub enum ElectionError {
    /// stub error to be filled in
    StubError,
    /// Math error doing something
    /// NOTE: it would be better to make Election polymorphic over
    /// the election error and then have specific math errors
    MathError,
}

/// For items that will always have the same validity outcome on a successful check,
/// allows for the case of "not yet possible to check" where the check might be
/// attempted again at a later point in time, but saves on repeated checking when
/// the outcome is already knowable.
///
/// This would be a useful general utility.
#[derive(Clone)]
pub enum Checked<T> {
    /// This item has been checked, and is valid
    Valid(T),
    /// This item has been checked, and is not valid
    Inval(T),
    /// This item has not been checked
    Unchecked(T),
}

/// Data to vote on for different types of votes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(bound(deserialize = ""))]
pub enum VoteData<COMMITMENT>
where
    COMMITMENT: CommitmentBounds,
{
    /// Vote to provide availability for a block.
    DA(COMMITMENT),
    /// Vote to append a leaf to the log.
    Yes(COMMITMENT),
    /// Vote to reject a leaf from the log.
    No(COMMITMENT),
    /// Vote to time out and proceed to the next view.
    Timeout(COMMITMENT),
    /// Vote for VID proposal
    VID(COMMITMENT),
    /// Vote to pre-commit the view sync.
    ViewSyncPreCommit(COMMITMENT),
    /// Vote to commit the view sync.
    ViewSyncCommit(COMMITMENT),
    /// Vote to finalize the view sync.
    ViewSyncFinalize(COMMITMENT),
}

/// Make different types of `VoteData` committable
impl<COMMITMENT> Committable for VoteData<COMMITMENT>
where
    COMMITMENT: CommitmentBounds,
{
    fn commit(&self) -> Commitment<Self> {
        let (tag, commit) = match self {
            VoteData::DA(c) => ("DA BlockPayload Commit", c),
            VoteData::VID(c) => ("VID Proposal Commit", c),
            VoteData::Yes(c) => ("Yes Vote Commit", c),
            VoteData::No(c) => ("No Vote Commit", c),
            VoteData::Timeout(c) => ("Timeout View Number Commit", c),
            VoteData::ViewSyncPreCommit(c) => ("ViewSyncPreCommit", c),
            VoteData::ViewSyncCommit(c) => ("ViewSyncCommit", c),
            VoteData::ViewSyncFinalize(c) => ("ViewSyncFinalize", c),
        };
        commit::RawCommitmentBuilder::new(tag)
            .var_size_bytes(commit.as_ref())
            .finalize()
    }

    fn tag() -> String {
        ("VOTE_DATA_COMMIT").to_string()
    }
}

impl<COMMITMENT> VoteData<COMMITMENT>
where
    COMMITMENT: CommitmentBounds,
{
    #[must_use]
    /// Convert vote data into bytes.
    ///
    /// # Panics
    /// Panics if the serialization fails.
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode_opts().serialize(&self).unwrap()
    }
}

/// Proof of this entity's right to vote, and of the weight of those votes
pub trait VoteToken:
    Clone
    + Debug
    + Send
    + Sync
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + PartialEq
    + Hash
    + Eq
{
    // type StakeTable;
    // type KeyPair: SignatureKey;
    // type ConsensusTime: ConsensusTime;

    /// the count, which validation will confirm
    fn vote_count(&self) -> NonZeroU64;
}

/// election config
pub trait ElectionConfig:
    Default
    + Clone
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + Sync
    + Send
    + core::fmt::Debug
{
}

/// A protocol for determining membership in and participating in a committee.
pub trait Membership<TYPES: NodeType>:
    Clone + Debug + Eq + PartialEq + Send + Sync + Hash + 'static
{
    /// generate a default election configuration
    fn default_election_config(num_nodes: u64) -> TYPES::ElectionConfigType;

    /// create an election
    /// TODO may want to move this to a testableelection trait
    fn create_election(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
    ) -> Self;

    /// Clone the public key and corresponding stake table for current elected committee
    fn get_committee_qc_stake_table(
        &self,
    ) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// The leader of the committee for view `view_number`.
    fn get_leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey;

    /// The members of the committee for view `view_number`.
    fn get_committee(&self, view_number: TYPES::Time) -> BTreeSet<TYPES::SignatureKey>;

    /// Check if a key has stake
    fn has_stake(&self, pub_key: &TYPES::SignatureKey) -> bool;

    /// Get the stake table entry for a public key, returns `None` if the
    /// key is not in the table
    fn get_stake(
        &self,
        pub_key: &TYPES::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Returns the number of total nodes in the committee
    fn total_nodes(&self) -> usize;

    /// Returns the threshold for a specific `Membership` implementation
    fn success_threshold(&self) -> NonZeroU64;

    /// Returns the threshold for a specific `Membership` implementation
    fn failure_threshold(&self) -> NonZeroU64;
}

/// Protocol for exchanging proposals and votes to make decisions in a distributed network.
///
/// An instance of [`ConsensusExchange`] represents the state of one participant in the protocol,
/// allowing them to vote and query information about the overall state of the protocol (such as
/// membership and leader status).
pub trait ConsensusExchange<TYPES: NodeType, M: NetworkMsg>: Send + Sync {
    /// The committee eligible to make decisions.
    type Membership: Membership<TYPES>;
    /// Network used by [`Membership`](Self::Membership) to communicate.
    type Networking: CommunicationChannel<TYPES, M, Self::Membership>;

    /// Join a [`ConsensusExchange`] with the given identity (`pk` and `sk`).
    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self;

    /// The network being used by this exchange.
    fn network(&self) -> &Self::Networking;

    /// The leader of the [`Membership`](Self::Membership) at time `view_number`.
    fn get_leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey {
        self.membership().get_leader(view_number)
    }

    /// Whether this participant is leader at time `view_number`.
    fn is_leader(&self, view_number: TYPES::Time) -> bool {
        &self.get_leader(view_number) == self.public_key()
    }

    /// Threshold required to approve a [`Proposal`](Self::Proposal).
    fn success_threshold(&self) -> NonZeroU64 {
        self.membership().success_threshold()
    }

    /// Threshold required to know a success threshold will not be reached
    fn failure_threshold(&self) -> NonZeroU64 {
        self.membership().failure_threshold()
    }

    /// The total number of nodes in the committee.
    fn total_nodes(&self) -> usize {
        self.membership().total_nodes()
    }

    /// The committee which votes on proposals.
    fn membership(&self) -> &TYPES::Membership;

    /// This participant's public key.
    fn public_key(&self) -> &TYPES::SignatureKey;

    /// This participant's private key.
    fn private_key(&self) -> &<TYPES::SignatureKey as SignatureKey>::PrivateKey;
}

/// A [`ConsensusExchange`] where participants vote to provide availability for blobs of data.
pub trait CommitteeExchangeType<TYPES: NodeType, M: NetworkMsg>:
    ConsensusExchange<TYPES, M>
{
    /// Sign a DA proposal.
    fn sign_da_proposal(
        &self,
        payload_commitment: &Commitment<TYPES::BlockPayload>,
    ) -> EncodedSignature;
}

/// Standard implementation of [`CommitteeExchangeType`] utilizing a DA committee.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct CommitteeExchange<
    TYPES: NodeType,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: TYPES::Membership,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(TYPES, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > CommitteeExchangeType<TYPES, M> for CommitteeExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    /// Sign a DA proposal.Self as ConsensusExchange<TYPES, M>
    fn sign_da_proposal(
        &self,
        payload_commitment: &Commitment<TYPES::BlockPayload>,
    ) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, payload_commitment.as_ref());
        signature
    }
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for CommitteeExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership = <TYPES as NodeType>::Membership::create_election(entries, config);
        Self {
            network,
            membership,
            public_key: pk,
            entry,
            private_key: sk,
            _pd: PhantomData,
        }
    }
    fn network(&self) -> &NETWORK {
        &self.network
    }

    fn membership(&self) -> &TYPES::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}

/// A [`ConsensusExchange`] where participants vote to provide availability for blobs of data.
pub trait VIDExchangeType<TYPES: NodeType, M: NetworkMsg>: ConsensusExchange<TYPES, M> {
    /// Sign a VID disperse
    fn sign_vid_disperse(
        &self,
        payload_commitment: &Commitment<TYPES::BlockPayload>,
    ) -> EncodedSignature;
}

/// Standard implementation of [`VIDExchangeType`]
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct VIDExchange<
    TYPES: NodeType,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: TYPES::Membership,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(TYPES, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > VIDExchangeType<TYPES, M> for VIDExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    /// Sign a VID proposal.
    fn sign_vid_disperse(
        &self,
        payload_commitment: &Commitment<TYPES::BlockPayload>,
    ) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, payload_commitment.as_ref());
        signature
    }
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for VIDExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership = <TYPES as NodeType>::Membership::create_election(entries, config);
        Self {
            network,
            membership,
            public_key: pk,
            entry,
            private_key: sk,
            _pd: PhantomData,
        }
    }
    fn network(&self) -> &NETWORK {
        &self.network
    }

    fn membership(&self) -> &TYPES::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}

/// A [`ConsensusExchange`] where participants vote to append items to a log.
pub trait QuorumExchangeType<TYPES: NodeType, M: NetworkMsg>: ConsensusExchange<TYPES, M> {
    /// Sign a validating or commitment proposal.
    fn sign_validating_or_commitment_proposal<I: NodeImplementation<TYPES>>(
        &self,
        leaf_commitment: &Commitment<Leaf<TYPES>>,
    ) -> EncodedSignature;

    /// Sign a block payload commitment.
    fn sign_payload_commitment(
        &self,
        payload_commitment: Commitment<TYPES::BlockPayload>,
    ) -> EncodedSignature;
}

/// Standard implementation of [`QuroumExchangeType`] based on Hot Stuff consensus.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct QuorumExchange<
    TYPES: NodeType,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: TYPES::Membership,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(Leaf<TYPES>, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > QuorumExchangeType<TYPES, M> for QuorumExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    /// Sign a validating or commitment proposal.
    fn sign_validating_or_commitment_proposal<I: NodeImplementation<TYPES>>(
        &self,
        leaf_commitment: &Commitment<Leaf<TYPES>>,
    ) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, leaf_commitment.as_ref());
        signature
    }

    fn sign_payload_commitment(
        &self,
        payload_commitment: Commitment<<TYPES as NodeType>::BlockPayload>,
    ) -> EncodedSignature {
        TYPES::SignatureKey::sign(&self.private_key, payload_commitment.as_ref())
    }
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for QuorumExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership = <TYPES as NodeType>::Membership::create_election(entries, config);
        Self {
            network,
            membership,
            public_key: pk,
            entry,
            private_key: sk,
            _pd: PhantomData,
        }
    }

    fn network(&self) -> &NETWORK {
        &self.network
    }

    fn membership(&self) -> &TYPES::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}

/// A [`ConsensusExchange`] where participants synchronize which view the network should be in.
pub trait ViewSyncExchangeType<TYPES: NodeType, M: NetworkMsg>:
    ConsensusExchange<TYPES, M>
{
}

/// Standard implementation of [`ViewSyncExchangeType`] based on Hot Stuff consensus.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct ViewSyncExchange<
    TYPES: NodeType,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: TYPES::Membership,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation in the stake table.
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ViewSyncExchangeType<TYPES, M> for ViewSyncExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for ViewSyncExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership = <TYPES as NodeType>::Membership::create_election(entries, config);
        Self {
            network,
            membership,
            public_key: pk,
            entry,
            private_key: sk,
            _pd: PhantomData,
        }
    }

    fn network(&self) -> &NETWORK {
        &self.network
    }

    fn membership(&self) -> &TYPES::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}

// TODO ED All the exchange structs are the same.  We could just considate them into one struct
/// Standard implementation of a Timeout Exchange based on Hot Stuff consensus.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct TimeoutExchange<
    TYPES: NodeType,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: TYPES::Membership,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation in the stake table.
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > TimeoutExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
}

/// Trait defining functiosn for a `TimeoutExchange`
pub trait TimeoutExchangeType<TYPES: NodeType, M: NetworkMsg>: ConsensusExchange<TYPES, M> {}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > TimeoutExchangeType<TYPES, M> for TimeoutExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for TimeoutExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership = <TYPES as NodeType>::Membership::create_election(entries, config);
        Self {
            network,
            membership,
            public_key: pk,
            entry,
            private_key: sk,
            _pd: PhantomData,
        }
    }

    fn network(&self) -> &NETWORK {
        &self.network
    }

    fn membership(&self) -> &TYPES::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}
