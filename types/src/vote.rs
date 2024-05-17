//! Vote, Accumulator, and Certificate Types

use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use bitvec::{bitvec, vec::BitVec};
use committable::Commitment;
use either::Either;
use ethereum_types::U256;
use tracing::error;

use crate::{
    data::{Leaf, QuorumProposal, VidDisperseShare},
    message::Proposal,
    simple_certificate::{DaCertificate, Threshold},
    simple_vote::Voteable,
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{SignatureKey, StakeTableEntryType},
    },
};

/// A simple vote that has a signer and commitment to the data voted on.
pub trait Vote<TYPES: NodeType>: HasViewNumber<TYPES> {
    /// Type of data commitment this vote uses.
    type Commitment: Voteable;

    /// Get the signature of the vote sender
    fn signature(&self) -> <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType;
    /// Gets the data which was voted on by this vote
    fn date(&self) -> &Self::Commitment;
    /// Gets the Data commitment of the vote
    fn date_commitment(&self) -> Commitment<Self::Commitment>;

    /// Gets the public signature key of the votes creator/sender
    fn signing_key(&self) -> TYPES::SignatureKey;
}

/// Any type that is associated with a view
pub trait HasViewNumber<TYPES: NodeType> {
    /// Returns the view number the type refers to.
    fn view_number(&self) -> TYPES::Time;
}

/**
The certificate formed from the collection of signatures a committee.
The committee is defined by the `Membership` associated type.
The votes all must be over the `Commitment` associated type.
*/
pub trait Certificate<TYPES: NodeType>: HasViewNumber<TYPES> {
    /// The data commitment this certificate certifies.
    type Voteable: Voteable;

    /// Threshold Functions
    type Threshold: Threshold<TYPES>;

    /// Build a certificate from the data commitment and the quorum of signers
    fn create_signed_certificate(
        vote_commitment: Commitment<Self::Voteable>,
        data: Self::Voteable,
        sig: <TYPES::SignatureKey as SignatureKey>::QcType,
        view: TYPES::Time,
    ) -> Self;

    /// Checks if the cert is valid
    fn is_valid_cert<MEMBERSHIP: Membership<TYPES>>(&self, membership: &MEMBERSHIP) -> bool;
    /// Returns the amount of stake needed to create this certificate
    // TODO: Make this a static ratio of the total stake of `Membership`
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64;
    /// Get the commitment which was voted on
    fn date(&self) -> &Self::Voteable;
    /// Get the vote commitment which the votes commit to
    fn date_commitment(&self) -> Commitment<Self::Voteable>;
}
/// Mapping of vote commitment to signatures and bitvec
type SignersMap<COMMITMENT, KEY> = HashMap<
    COMMITMENT,
    (
        BitVec,
        Vec<<KEY as SignatureKey>::PureAssembledSignatureType>,
    ),
>;
/// Accumulates votes until a certificate is formed.  This implementation works for all simple vote and certificate pairs
pub struct VoteAccumulator<
    TYPES: NodeType,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
> {
    /// Map of all signatures accumulated so far
    pub vote_outcomes: VoteMap2<
        Commitment<VOTE::Commitment>,
        TYPES::SignatureKey,
        <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    >,
    /// A bitvec to indicate which node is active and send out a valid signature for certificate aggregation, this automatically do uniqueness check
    /// And a list of valid signatures for certificate aggregation
    pub signers: SignersMap<Commitment<VOTE::Commitment>, TYPES::SignatureKey>,
    /// Phantom data to specify the types this accumulator is for
    pub phantom: PhantomData<(TYPES, VOTE, CERT)>,
}

impl<TYPES: NodeType, VOTE: Vote<TYPES>, CERT: Certificate<TYPES, Voteable = VOTE::Commitment>>
    VoteAccumulator<TYPES, VOTE, CERT>
{
    /// Add a vote to the total accumulated votes.  Returns the accumulator or the certificate if we
    /// have accumulated enough votes to exceed the threshold for creating a certificate.
    pub fn accumulate(&mut self, vote: &VOTE, membership: &TYPES::Membership) -> Either<(), CERT> {
        let key = vote.signing_key();

        let vote_commitment = vote.date_commitment();
        if !key.validate(&vote.signature(), vote_commitment.as_ref()) {
            error!("Invalid vote! Vote Data {:?}", vote.date());
            return Either::Left(());
        }

        let Some(stake_table_entry) = membership.stake(&key) else {
            return Either::Left(());
        };
        let stake_table = membership.committee_qc_stake_table();
        let Some(vote_node_id) = stake_table
            .iter()
            .position(|x| *x == stake_table_entry.clone())
        else {
            return Either::Left(());
        };

        let original_signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType =
            vote.signature();

        let (total_stake_casted, total_vote_map) = self
            .vote_outcomes
            .entry(vote_commitment)
            .or_insert_with(|| (U256::from(0), BTreeMap::new()));

        // Check for duplicate vote
        if total_vote_map.contains_key(&key) {
            return Either::Left(());
        }
        let (signers, sig_list) = self
            .signers
            .entry(vote_commitment)
            .or_insert((bitvec![0; membership.total_nodes()], Vec::new()));
        if signers.get(vote_node_id).as_deref() == Some(&true) {
            error!("Node id is already in signers list");
            return Either::Left(());
        }
        signers.set(vote_node_id, true);
        sig_list.push(original_signature);

        // TODO: Get the stake from the stake table entry.
        *total_stake_casted += stake_table_entry.stake();
        total_vote_map.insert(key, (vote.signature(), vote.date_commitment()));

        if *total_stake_casted >= CERT::threshold(membership).into() {
            // Assemble QC
            let real_qc_pp: <<TYPES as NodeType>::SignatureKey as SignatureKey>::QcParams =
                <TYPES::SignatureKey as SignatureKey>::public_parameter(
                    stake_table,
                    U256::from(CERT::threshold(membership)),
                );

            let real_qc_sig = <TYPES::SignatureKey as SignatureKey>::assemble(
                &real_qc_pp,
                signers.as_bitslice(),
                &sig_list[..],
            );

            let cert = CERT::create_signed_certificate(
                vote.date_commitment(),
                vote.date().clone(),
                real_qc_sig,
                vote.view_number(),
            );
            return Either::Right(cert);
        }
        Either::Left(())
    }
}

/// Mapping of commitments to vote tokens by key.
type VoteMap2<COMMITMENT, PK, SIG> = HashMap<COMMITMENT, (U256, BTreeMap<PK, (SIG, COMMITMENT)>)>;

/// Payload for the `HotShotEvents::VoteNow` event type. The proposal and leaf are
/// obtained via a `QuorumProposalValidated` event being processed.
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct VoteDependencyData<TYPES: NodeType> {
    /// The quorum proposal (not necessarily valid).
    pub quorum_proposal: QuorumProposal<TYPES>,

    /// The leaf we've obtained from the `QuorumProposalValidated` event. This is the
    /// parent leaf.
    pub parent_leaf: Leaf<TYPES>,

    /// The Vid disperse proposal.
    pub disperse_share: Proposal<TYPES, VidDisperseShare<TYPES>>,

    /// The DA certificate.
    pub da_cert: DaCertificate<TYPES>,
}
