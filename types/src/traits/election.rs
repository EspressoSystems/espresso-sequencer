//! The election trait, used to decide which node is the leader and determine if a vote is valid.

// Needed to avoid the non-biding `let` warning.
#![allow(clippy::let_underscore_untyped)]

use super::{
    node_implementation::{NodeImplementation, NodeType},
    signature_key::{EncodedPublicKey, EncodedSignature},
};
use crate::{
    certificate::{
        AssembledSignature, DACertificate, QuorumCertificate, TimeoutCertificate,
        ViewSyncCertificate,
    },
    data::{DAProposal, ProposalType},
    vote::TimeoutVote,
};

use crate::{
    message::{GeneralConsensusMessage, Message},
    vote::ViewSyncVoteInternal,
};

use crate::{
    data::LeafType,
    traits::{
        network::{CommunicationChannel, NetworkMsg},
        node_implementation::ExchangesType,
        signature_key::SignatureKey,
        state::ConsensusTime,
    },
    vote::{Accumulator, DAVote, QuorumVote, ViewSyncData, ViewSyncVote, VoteType, YesOrNoVote},
};
use bincode::Options;
use commit::{Commitment, CommitmentBounds, Committable};
use derivative::Derivative;
use either::Either;
use ethereum_types::U256;
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{collections::BTreeSet, fmt::Debug, hash::Hash, marker::PhantomData, num::NonZeroU64};
use tracing::error;

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
    /// Return the underlying commitment.
    #[must_use]
    pub fn get_commit(&self) -> COMMITMENT {
        #[allow(clippy::enum_glob_use)]
        use VoteData::*;
        match self {
            DA(c) | Yes(c) | No(c) | Timeout(c) | ViewSyncPreCommit(c) | ViewSyncCommit(c)
            | ViewSyncFinalize(c) => c.clone(),
        }
    }

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

/// A certificate of some property which has been signed by a quroum of nodes.
pub trait SignedCertificate<TYPES: NodeType, TIME, TOKEN, COMMITMENT>
where
    Self: Send + Sync + Clone + Serialize + for<'a> Deserialize<'a>,
    COMMITMENT: CommitmentBounds,
    TOKEN: VoteToken,
{
    /// `VoteType` that is used in this certificate
    type Vote: VoteType<TYPES, COMMITMENT>;

    /// `Accumulator` type to accumulate votes.
    type VoteAccumulator: Accumulator<TYPES, COMMITMENT, Self::Vote>;

    /// Build a QC from the threshold signature and commitment
    // TODO ED Rename this function and rework this function parameters
    // Assumes last vote was valid since it caused a QC to form.
    // Removes need for relay on other cert specific fields
    fn create_certificate(signatures: AssembledSignature<TYPES>, vote: Self::Vote) -> Self;

    /// Get the view number.
    fn view_number(&self) -> TIME;

    /// Get signatures.
    fn signatures(&self) -> AssembledSignature<TYPES>;

    // TODO (da) the following functions should be refactored into a QC-specific trait.
    // TODO ED Make an issue for this

    /// Get the leaf commitment.
    fn leaf_commitment(&self) -> COMMITMENT;

    /// Get whether the certificate is for the genesis block.
    fn is_genesis(&self) -> bool;

    /// To be used only for generating the genesis quorum certificate; will fail if used anywhere else
    fn genesis() -> Self;
}

/// A protocol for determining membership in and participating in a ccommittee.
pub trait Membership<TYPES: NodeType>:
    Clone + Debug + Eq + PartialEq + Send + Sync + 'static
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

    /// Attempts to generate a vote token for self
    ///
    /// Returns `None` if the number of seats would be zero
    /// # Errors
    /// TODO tbd
    fn make_vote_token(
        &self,
        view_number: TYPES::Time,
        priv_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Result<Option<TYPES::VoteTokenType>, ElectionError>;

    /// Checks the claims of a received vote token
    ///
    /// # Errors
    /// TODO tbd
    fn validate_vote_token(
        &self,
        pub_key: TYPES::SignatureKey,
        token: Checked<TYPES::VoteTokenType>,
    ) -> Result<Checked<TYPES::VoteTokenType>, ElectionError>;

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
    /// A proposal for participants to vote on.
    type Proposal: ProposalType<NodeType = TYPES>;
    /// A vote on a [`Proposal`](Self::Proposal).
    // TODO ED Make this equal Certificate vote (if possible?)
    type Vote: VoteType<TYPES, Self::Commitment>;
    /// A [`SignedCertificate`] attesting to a decision taken by the committee.
    type Certificate: SignedCertificate<TYPES, TYPES::Time, TYPES::VoteTokenType, Self::Commitment>
        + Hash
        + Eq;
    /// The committee eligible to make decisions.
    type Membership: Membership<TYPES>;
    /// Network used by [`Membership`](Self::Membership) to communicate.
    type Networking: CommunicationChannel<TYPES, M, Self::Membership>;
    /// Commitments to items which are the subject of proposals and decisions.
    type Commitment: CommitmentBounds;

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

    /// Attempts to generate a vote token for participation at time `view_number`.
    ///
    /// # Errors
    /// When unable to make a vote token because not part of the committee
    fn make_vote_token(
        &self,
        view_number: TYPES::Time,
    ) -> std::result::Result<std::option::Option<TYPES::VoteTokenType>, ElectionError> {
        self.membership()
            .make_vote_token(view_number, self.private_key())
    }

    /// Validate a certificate.
    fn is_valid_cert(&self, qc: &Self::Certificate) -> bool {
        if qc.is_genesis() && qc.view_number() == TYPES::Time::genesis() {
            return true;
        }
        let leaf_commitment = qc.leaf_commitment();

        match qc.signatures() {
            AssembledSignature::DA(qc) => {
                let real_commit = VoteData::DA(leaf_commitment).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(
                    &real_qc_pp,
                    real_commit.as_ref(),
                    &qc,
                )
            }
            AssembledSignature::Yes(qc) => {
                let real_commit = VoteData::Yes(leaf_commitment).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(&real_qc_pp, real_commit.as_ref(), &qc)
            }
            AssembledSignature::No(qc) => {
                let real_commit = VoteData::No(leaf_commitment).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(&real_qc_pp, real_commit.as_ref(), &qc)
            }
            AssembledSignature::Timeout(qc) => {
                let real_commit = VoteData::Timeout(leaf_commitment).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(&real_qc_pp, real_commit.as_ref(), &qc)
            }
            AssembledSignature::Genesis() => true,
            AssembledSignature::ViewSyncPreCommit(_)
            | AssembledSignature::ViewSyncCommit(_)
            | AssembledSignature::ViewSyncFinalize(_) => {
                error!("QC should not be ViewSync type here");
                false
            }
        }
    }

    /// Validate a vote by checking its signature and token.
    fn is_valid_vote(
        &self,
        key: &TYPES::SignatureKey,
        encoded_signature: &EncodedSignature,
        data: &VoteData<Self::Commitment>,
        vote_token: &Checked<TYPES::VoteTokenType>,
    ) -> bool {
        let is_valid_signature = key.validate(encoded_signature, data.commit().as_ref());
        let valid_vote_token = self
            .membership()
            .validate_vote_token(key.clone(), vote_token.clone());
        let is_valid_vote_token = match valid_vote_token {
            Err(_) => {
                error!("Vote token was invalid");
                false
            }
            Ok(Checked::Valid(_)) => true,
            Ok(Checked::Inval(_) | Checked::Unchecked(_)) => false,
        };

        is_valid_signature && is_valid_vote_token
    }

    // TODO ED Depending on what we do in the future with the exchanges trait, we can move the accumulator out of the `SignedCertificate`
    // trait.  Logically, I feel it makes sense to accumulate on the certificate rather than the exchange, however.
    /// Accumulate vote
    /// Returns either the accumulate if no threshold was reached, or a `SignedCertificate` if the threshold was reached
    #[allow(clippy::type_complexity)]
    fn accumulate_vote(
        &self,
        accumulator: <<Self as ConsensusExchange<TYPES, M>>::Certificate as SignedCertificate<
            TYPES,
            TYPES::Time,
            TYPES::VoteTokenType,
            Self::Commitment,
        >>::VoteAccumulator,
        vote: &<<Self as ConsensusExchange<TYPES, M>>::Certificate as SignedCertificate<
            TYPES,
            TYPES::Time,
            TYPES::VoteTokenType,
            Self::Commitment,
        >>::Vote,
        _commit: &Self::Commitment,
    ) -> Either<
        <<Self as ConsensusExchange<TYPES, M>>::Certificate as SignedCertificate<
            TYPES,
            TYPES::Time,
            TYPES::VoteTokenType,
            Self::Commitment,
        >>::VoteAccumulator,
        Self::Certificate,
    > {
        if !self.is_valid_vote(
            &vote.get_key(),
            &vote.get_signature(),
            &vote.get_data(),
            &Checked::Unchecked(vote.get_vote_token()),
        ) {
            error!("Vote data is {:?}", vote.get_data());

            error!("Invalid vote!");
            return Either::Left(accumulator);
        }

        let stake_table_entry = vote.get_key().get_stake_table_entry(1u64);
        // TODO ED Could we make this part of the vote in the future?  It's only a usize.
        let append_node_id = self
            .membership()
            .get_committee_qc_stake_table()
            .iter()
            .position(|x| *x == stake_table_entry.clone())
            .unwrap();

        // TODO ED Should make append function take a reference to vote
        match accumulator.append(
            vote.clone(),
            append_node_id,
            self.membership().get_committee_qc_stake_table(),
        ) {
            Either::Left(accumulator) => Either::Left(accumulator),
            Either::Right(signatures) => Either::Right(Self::Certificate::create_certificate(
                signatures,
                vote.clone(),
            )),
        }
    }

    /// The committee which votes on proposals.
    fn membership(&self) -> &Self::Membership;

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
    fn sign_da_proposal(&self, block_commitment: &Commitment<TYPES::BlockType>)
        -> EncodedSignature;

    /// Sign a vote on DA proposal.
    ///
    /// The block commitment and the type of the vote (DA) are signed, which is the minimum amount
    /// of information necessary for checking that this node voted on that block.
    fn sign_da_vote(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Create a message with a vote on DA proposal.
    fn create_da_message(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> DAVote<TYPES>;

    // TODO temporary vid methods, move to quorum https://github.com/EspressoSystems/HotShot/issues/1696

    /// Create a message with a vote on VID disperse data.
    fn create_vid_message(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> DAVote<TYPES>;

    /// Sign a vote on VID proposal.
    fn sign_vid_vote(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
    ) -> (EncodedPublicKey, EncodedSignature);
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
    membership: MEMBERSHIP,
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
    /// Sign a DA proposal.
    fn sign_da_proposal(
        &self,
        block_commitment: &Commitment<TYPES::BlockType>,
    ) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, block_commitment.as_ref());
        signature
    }
    /// Sign a vote on DA proposal.
    ///
    /// The block commitment and the type of the vote (DA) are signed, which is the minimum amount
    /// of information necessary for checking that this node voted on that block.
    fn sign_da_vote(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::DA(block_commitment).commit().as_ref(),
        );
        (self.public_key.to_bytes(), signature)
    }
    /// Create a message with a vote on DA proposal.
    fn create_da_message(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> DAVote<TYPES> {
        let signature = self.sign_da_vote(block_commitment);
        DAVote {
            signature,
            block_commitment,
            current_view,
            vote_token,
            vote_data: VoteData::DA(block_commitment),
        }
    }

    fn create_vid_message(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
        current_view: <TYPES as NodeType>::Time,
        vote_token: <TYPES as NodeType>::VoteTokenType,
    ) -> DAVote<TYPES> {
        let signature = self.sign_vid_vote(block_commitment);
        DAVote {
            signature,
            block_commitment,
            current_view,
            vote_token,
            vote_data: VoteData::DA(block_commitment),
        }
    }

    fn sign_vid_vote(
        &self,
        block_commitment: Commitment<<TYPES as NodeType>::BlockType>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::DA(block_commitment).commit().as_ref(),
        );
        (self.public_key.to_bytes(), signature)
    }
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for CommitteeExchange<TYPES, MEMBERSHIP, NETWORK, M>
{
    type Proposal = DAProposal<TYPES>;
    type Vote = DAVote<TYPES>;
    type Certificate = DACertificate<TYPES>;
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;
    type Commitment = Commitment<TYPES::BlockType>;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership =
            <Self as ConsensusExchange<TYPES, M>>::Membership::create_election(entries, config);
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
    fn make_vote_token(
        &self,
        view_number: TYPES::Time,
    ) -> std::result::Result<std::option::Option<TYPES::VoteTokenType>, ElectionError> {
        self.membership
            .make_vote_token(view_number, &self.private_key)
    }

    fn membership(&self) -> &Self::Membership {
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
pub trait QuorumExchangeType<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>, M: NetworkMsg>:
    ConsensusExchange<TYPES, M>
{
    /// Create a message with a positive vote on validating or commitment proposal.
    // TODO ED This returns just a general message type, it's not even bound to a proposal, and this is just a function on the QC.  Make proprosal doesn't really apply to all cert types.
    fn create_yes_message<I: NodeImplementation<TYPES, Leaf = LEAF>>(
        &self,
        justify_qc_commitment: Commitment<Self::Certificate>,
        leaf_commitment: Commitment<LEAF>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>
    where
        <Self as ConsensusExchange<TYPES, M>>::Certificate: commit::Committable,
        I::Exchanges: ExchangesType<TYPES, LEAF, Message<TYPES, I>>;

    /// Sign a validating or commitment proposal.
    fn sign_validating_or_commitment_proposal<I: NodeImplementation<TYPES>>(
        &self,
        leaf_commitment: &Commitment<LEAF>,
    ) -> EncodedSignature;

    /// Sign a block commitment.
    fn sign_block_commitment(
        &self,
        block_commitment: Commitment<TYPES::BlockType>,
    ) -> EncodedSignature;

    /// Sign a positive vote on validating or commitment proposal.
    ///
    /// The leaf commitment and the type of the vote (yes) are signed, which is the minimum amount
    /// of information necessary for any user of the subsequently constructed QC to check that this
    /// node voted `Yes` on that leaf. The leaf is expected to be reconstructed based on other
    /// information in the yes vote.
    fn sign_yes_vote(
        &self,
        leaf_commitment: Commitment<LEAF>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Sign a neagtive vote on validating or commitment proposal.
    ///
    /// The leaf commitment and the type of the vote (no) are signed, which is the minimum amount
    /// of information necessary for any user of the subsequently constructed QC to check that this
    /// node voted `No` on that leaf.
    fn sign_no_vote(
        &self,
        leaf_commitment: Commitment<LEAF>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Create a message with a negative vote on validating or commitment proposal.
    fn create_no_message<I: NodeImplementation<TYPES, Leaf = LEAF>>(
        &self,
        justify_qc_commitment: Commitment<QuorumCertificate<TYPES, Commitment<LEAF>>>,
        leaf_commitment: Commitment<LEAF>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>
    where
        I::Exchanges: ExchangesType<TYPES, I::Leaf, Message<TYPES, I>>;
}

/// Standard implementation of [`QuroumExchangeType`] based on Hot Stuff consensus.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct QuorumExchange<
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: MEMBERSHIP,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(LEAF, PROPOSAL, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        LEAF: LeafType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > QuorumExchangeType<TYPES, LEAF, M>
    for QuorumExchange<TYPES, LEAF, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
    /// Create a message with a positive vote on validating or commitment proposal.
    fn create_yes_message<I: NodeImplementation<TYPES, Leaf = LEAF>>(
        &self,
        justify_qc_commitment: Commitment<QuorumCertificate<TYPES, Commitment<LEAF>>>,
        leaf_commitment: Commitment<LEAF>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>
    where
        I::Exchanges: ExchangesType<TYPES, LEAF, Message<TYPES, I>>,
    {
        let signature = self.sign_yes_vote(leaf_commitment);
        GeneralConsensusMessage::<TYPES, I>::Vote(QuorumVote::Yes(YesOrNoVote {
            justify_qc_commitment,
            signature,
            leaf_commitment,
            current_view,
            vote_token,
            vote_data: VoteData::Yes(leaf_commitment),
        }))
    }
    /// Sign a validating or commitment proposal.
    fn sign_validating_or_commitment_proposal<I: NodeImplementation<TYPES>>(
        &self,
        leaf_commitment: &Commitment<LEAF>,
    ) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, leaf_commitment.as_ref());
        signature
    }

    fn sign_block_commitment(
        &self,
        block_commitment: Commitment<<TYPES as NodeType>::BlockType>,
    ) -> EncodedSignature {
        TYPES::SignatureKey::sign(&self.private_key, block_commitment.as_ref())
    }

    /// Sign a positive vote on validating or commitment proposal.
    ///
    /// The leaf commitment and the type of the vote (yes) are signed, which is the minimum amount
    /// of information necessary for any user of the subsequently constructed QC to check that this
    /// node voted `Yes` on that leaf. The leaf is expected to be reconstructed based on other
    /// information in the yes vote.
    ///
    /// TODO GG: why return the pubkey? Some other `sign_xxx` methods do not return the pubkey.
    fn sign_yes_vote(
        &self,
        leaf_commitment: Commitment<LEAF>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::Yes(leaf_commitment).commit().as_ref(),
        );
        (self.public_key.to_bytes(), signature)
    }

    /// Sign a neagtive vote on validating or commitment proposal.
    ///
    /// The leaf commitment and the type of the vote (no) are signed, which is the minimum amount
    /// of information necessary for any user of the subsequently constructed QC to check that this
    /// node voted `No` on that leaf.
    /// TODO GG: why return the pubkey? Some other `sign_xxx` methods do not return the pubkey.
    fn sign_no_vote(
        &self,
        leaf_commitment: Commitment<LEAF>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::No(leaf_commitment).commit().as_ref(),
        );
        (self.public_key.to_bytes(), signature)
    }

    /// Create a message with a negative vote on validating or commitment proposal.
    fn create_no_message<I: NodeImplementation<TYPES, Leaf = LEAF>>(
        &self,
        justify_qc_commitment: Commitment<QuorumCertificate<TYPES, Commitment<LEAF>>>,
        leaf_commitment: Commitment<LEAF>,
        current_view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>
    where
        I::Exchanges: ExchangesType<TYPES, LEAF, Message<TYPES, I>>,
    {
        let signature = self.sign_no_vote(leaf_commitment);
        GeneralConsensusMessage::<TYPES, I>::Vote(QuorumVote::No(YesOrNoVote {
            justify_qc_commitment,
            signature,
            leaf_commitment,
            current_view,
            vote_token,
            vote_data: VoteData::No(leaf_commitment),
        }))
    }
}

impl<
        TYPES: NodeType,
        LEAF: LeafType<NodeType = TYPES>,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M>
    for QuorumExchange<TYPES, LEAF, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
    type Proposal = PROPOSAL;
    type Vote = QuorumVote<TYPES, Commitment<LEAF>>;
    type Certificate = QuorumCertificate<TYPES, Commitment<LEAF>>;
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;
    type Commitment = Commitment<LEAF>;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership =
            <Self as ConsensusExchange<TYPES, M>>::Membership::create_election(entries, config);
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

    fn membership(&self) -> &Self::Membership {
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
    /// Creates a precommit vote
    fn create_precommit_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>;

    /// Signs a precommit vote
    fn sign_precommit_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Creates a commit vote
    fn create_commit_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>;

    /// Signs a commit vote
    fn sign_commit_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Creates a finalize vote
    fn create_finalize_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>;

    /// Sings a finalize vote
    fn sign_finalize_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature);

    /// Validate a certificate.
    fn is_valid_view_sync_cert(&self, certificate: Self::Certificate, round: TYPES::Time) -> bool;

    /// Sign a certificate.
    fn sign_certificate_proposal(&self, certificate: Self::Certificate) -> EncodedSignature;
}

/// Standard implementation of [`ViewSyncExchangeType`] based on Hot Stuff consensus.
#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct ViewSyncExchange<
    TYPES: NodeType,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: MEMBERSHIP,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation in the stake table.
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(PROPOSAL, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        MEMBERSHIP: Membership<TYPES>,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ViewSyncExchangeType<TYPES, M> for ViewSyncExchange<TYPES, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
    fn create_precommit_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I> {
        let relay_pub_key = self.get_leader(round + relay).to_bytes();

        let vote_data_internal: ViewSyncData<TYPES> = ViewSyncData {
            relay: relay_pub_key.clone(),
            round,
        };

        let vote_data_internal_commitment = vote_data_internal.commit();

        let signature = self.sign_precommit_message(vote_data_internal_commitment);

        GeneralConsensusMessage::<TYPES, I>::ViewSyncVote(ViewSyncVote::PreCommit(
            ViewSyncVoteInternal {
                relay_pub_key,
                relay,
                round,
                signature,
                vote_token,
                vote_data: VoteData::ViewSyncPreCommit(vote_data_internal_commitment),
            },
        ))
    }

    fn sign_precommit_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::ViewSyncPreCommit(commitment).commit().as_ref(),
        );

        (self.public_key.to_bytes(), signature)
    }

    fn create_commit_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I> {
        let relay_pub_key = self.get_leader(round + relay).to_bytes();

        let vote_data_internal: ViewSyncData<TYPES> = ViewSyncData {
            relay: relay_pub_key.clone(),
            round,
        };

        let vote_data_internal_commitment = vote_data_internal.commit();

        let signature = self.sign_commit_message(vote_data_internal_commitment);

        GeneralConsensusMessage::<TYPES, I>::ViewSyncVote(ViewSyncVote::Commit(
            ViewSyncVoteInternal {
                relay_pub_key,
                relay,
                round,
                signature,
                vote_token,
                vote_data: VoteData::ViewSyncCommit(vote_data_internal_commitment),
            },
        ))
    }

    fn sign_commit_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::ViewSyncCommit(commitment).commit().as_ref(),
        );

        (self.public_key.to_bytes(), signature)
    }

    fn create_finalize_message<I: NodeImplementation<TYPES>>(
        &self,
        round: TYPES::Time,
        relay: u64,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I> {
        let relay_pub_key = self.get_leader(round + relay).to_bytes();

        let vote_data_internal: ViewSyncData<TYPES> = ViewSyncData {
            relay: relay_pub_key.clone(),
            round,
        };

        let vote_data_internal_commitment = vote_data_internal.commit();

        let signature = self.sign_finalize_message(vote_data_internal_commitment);

        GeneralConsensusMessage::<TYPES, I>::ViewSyncVote(ViewSyncVote::Finalize(
            ViewSyncVoteInternal {
                relay_pub_key,
                relay,
                round,
                signature,
                vote_token,
                vote_data: VoteData::ViewSyncFinalize(vote_data_internal_commitment),
            },
        ))
    }

    fn sign_finalize_message(
        &self,
        commitment: Commitment<ViewSyncData<TYPES>>,
    ) -> (EncodedPublicKey, EncodedSignature) {
        let signature = TYPES::SignatureKey::sign(
            &self.private_key,
            VoteData::ViewSyncFinalize(commitment).commit().as_ref(),
        );

        (self.public_key.to_bytes(), signature)
    }

    fn is_valid_view_sync_cert(&self, certificate: Self::Certificate, round: TYPES::Time) -> bool {
        // Sishan NOTE TODO: would be better to test this, looks like this func is never called.
        let (certificate_internal, _threshold, vote_data) = match certificate.clone() {
            ViewSyncCertificate::PreCommit(certificate_internal) => {
                let vote_data = ViewSyncData::<TYPES> {
                    relay: self
                        .get_leader(round + certificate_internal.relay)
                        .to_bytes(),
                    round,
                };
                (certificate_internal, self.failure_threshold(), vote_data)
            }
            ViewSyncCertificate::Commit(certificate_internal)
            | ViewSyncCertificate::Finalize(certificate_internal) => {
                let vote_data = ViewSyncData::<TYPES> {
                    relay: self
                        .get_leader(round + certificate_internal.relay)
                        .to_bytes(),
                    round,
                };
                (certificate_internal, self.success_threshold(), vote_data)
            }
        };
        match certificate_internal.signatures {
            AssembledSignature::ViewSyncPreCommit(raw_signatures) => {
                let real_commit = VoteData::ViewSyncPreCommit(vote_data.commit()).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().failure_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(
                    &real_qc_pp,
                    real_commit.as_ref(),
                    &raw_signatures,
                )
            }
            AssembledSignature::ViewSyncCommit(raw_signatures) => {
                let real_commit = VoteData::ViewSyncCommit(vote_data.commit()).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(
                    &real_qc_pp,
                    real_commit.as_ref(),
                    &raw_signatures,
                )
            }
            AssembledSignature::ViewSyncFinalize(raw_signatures) => {
                let real_commit = VoteData::ViewSyncFinalize(vote_data.commit()).commit();
                let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    self.membership().get_committee_qc_stake_table(),
                    U256::from(self.membership().success_threshold().get()),
                );
                <TYPES::SignatureKey as SignatureKey>::check(
                    &real_qc_pp,
                    real_commit.as_ref(),
                    &raw_signatures,
                )
            }
            _ => true,
        }
    }

    fn sign_certificate_proposal(&self, certificate: Self::Certificate) -> EncodedSignature {
        let signature = TYPES::SignatureKey::sign(&self.private_key, certificate.commit().as_ref());
        signature
    }
}

impl<
        TYPES: NodeType,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for ViewSyncExchange<TYPES, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
    type Proposal = PROPOSAL;
    type Vote = ViewSyncVote<TYPES>;
    type Certificate = ViewSyncCertificate<TYPES>;
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;
    type Commitment = Commitment<ViewSyncData<TYPES>>;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership =
            <Self as ConsensusExchange<TYPES, M>>::Membership::create_election(entries, config);
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

    fn membership(&self) -> &Self::Membership {
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
    PROPOSAL: ProposalType<NodeType = TYPES>,
    MEMBERSHIP: Membership<TYPES>,
    NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
    M: NetworkMsg,
> {
    /// The network being used by this exchange.
    network: NETWORK,
    /// The committee which votes on proposals.
    membership: MEMBERSHIP,
    /// This participant's public key.
    public_key: TYPES::SignatureKey,
    /// Entry with public key and staking value for certificate aggregation in the stake table.
    entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    /// This participant's private key.
    #[derivative(Debug = "ignore")]
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    #[doc(hidden)]
    _pd: PhantomData<(PROPOSAL, MEMBERSHIP, M)>,
}

impl<
        TYPES: NodeType,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > TimeoutExchange<TYPES, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
}

/// Trait defining functiosn for a `TimeoutExchange`
pub trait TimeoutExchangeType<TYPES: NodeType, M: NetworkMsg>: ConsensusExchange<TYPES, M> {
    /// Create and sign a timeout message
    fn create_timeout_message<I: NodeImplementation<TYPES>>(
        &self,
        view: TYPES::Time,
        vote_token: TYPES::VoteTokenType,
    ) -> GeneralConsensusMessage<TYPES, I>
    where
        I::Exchanges: ExchangesType<TYPES, I::Leaf, Message<TYPES, I>>,
    {
        let signature = TYPES::SignatureKey::sign(
            self.private_key(),
            VoteData::<Commitment<TYPES::Time>>::Timeout(view.commit())
                .commit()
                .as_ref(),
        );

        GeneralConsensusMessage::<TYPES, I>::TimeoutVote(TimeoutVote {
            signature: (self.public_key().to_bytes(), signature),
            current_view: view,
            vote_token,
        })
    }
}

impl<
        TYPES: NodeType,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > TimeoutExchangeType<TYPES, M> for TimeoutExchange<TYPES, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
}

// TODO ED Get rid of ProposalType as generic, is debt left over from Validating Consensus
impl<
        TYPES: NodeType,
        PROPOSAL: ProposalType<NodeType = TYPES>,
        MEMBERSHIP: Membership<TYPES>,
        NETWORK: CommunicationChannel<TYPES, M, MEMBERSHIP>,
        M: NetworkMsg,
    > ConsensusExchange<TYPES, M> for TimeoutExchange<TYPES, PROPOSAL, MEMBERSHIP, NETWORK, M>
{
    type Proposal = PROPOSAL;
    type Vote = TimeoutVote<TYPES>;
    type Certificate = TimeoutCertificate<TYPES>;
    type Membership = MEMBERSHIP;
    type Networking = NETWORK;
    type Commitment = Commitment<TYPES::Time>;

    fn create(
        entries: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
        network: Self::Networking,
        pk: TYPES::SignatureKey,
        entry: <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        sk: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        let membership =
            <Self as ConsensusExchange<TYPES, M>>::Membership::create_election(entries, config);
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

    fn membership(&self) -> &Self::Membership {
        &self.membership
    }
    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.public_key
    }
    fn private_key(&self) -> &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey {
        &self.private_key
    }
}

/// Testable implementation of a [`Membership`]. Will expose a method to generate a vote token used for testing.
pub trait TestableElection<TYPES: NodeType>: Membership<TYPES> {
    /// Generate a vote token used for testing.
    fn generate_test_vote_token() -> TYPES::VoteTokenType;
}
