//! Vote, Accumulator, and Certificate Types

use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use bincode::Options;
use bitvec::vec::BitVec;
use commit::Commitment;
use either::Either;
use ethereum_types::U256;
use hotshot_utils::bincode::bincode_opts;
use tracing::error;

use crate::{
    simple_vote::Voteable,
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey, StakeTableEntryType},
    },
};

/// A simple vote that has a signer and commitment to the data voted on.
pub trait Vote<TYPES: NodeType>: HasViewNumber<TYPES> {
    /// Type of data commitment this vote uses.
    type Commitment: Voteable;

    /// Get the signature of the vote sender
    fn get_signature(&self) -> EncodedSignature;
    /// Gets the data which was voted on by this vote
    fn get_data(&self) -> &Self::Commitment;
    /// Gets the Data commitment of the vote
    fn get_data_commitment(&self) -> Commitment<Self::Commitment>;

    /// Gets the public signature key of the votes creator/sender
    fn get_signing_key(&self) -> TYPES::SignatureKey;
    // fn create_signed_vote(Self::Commitment, Self::Membership) ??
}

/// Any type that is associated with a view
pub trait HasViewNumber<TYPES: NodeType> {
    /// Returns the view number the type refers to.
    fn get_view_number(&self) -> TYPES::Time;
}

/**
The certificate formed from the collection of signatures a committee.
The committee is defined by the `Membership` associated type.
The votes all must be over the `Commitment` associated type.
*/
pub trait Certificate<TYPES: NodeType>: HasViewNumber<TYPES> {
    /// The data commitment this certificate certifies.
    type Voteable: Voteable;

    /// Build a certificate from the data commitment and the quorum of signers
    fn create_signed_certificate(
        vote_commitment: Commitment<Self::Voteable>,
        data: Self::Voteable,
        sig: <TYPES::SignatureKey as SignatureKey>::QCType,
        view: TYPES::Time,
    ) -> Self;

    /// Checks if the cert is valid
    fn is_valid_cert<MEMBERSHIP: Membership<TYPES>>(&self, membership: &MEMBERSHIP) -> bool;
    /// Returns the amount of stake needed to create this certificate
    // TODO: Make this a static ratio of the total stake of `Membership`
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64;
    /// Get the commitment which was voted on
    fn get_data(&self) -> &Self::Voteable;
    /// Get the vote commitment which the votes commit to
    fn get_data_commitment(&self) -> Commitment<Self::Voteable>;
}

/// Accumulates votes until a certificate is formed.  This implementation works for all simple vote and certificate pairs
pub struct VoteAccumulator<
    TYPES: NodeType,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
> {
    /// Map of all signatures accumlated so far
    pub vote_outcomes: VoteMap2<Commitment<VOTE::Commitment>>,
    /// A list of valid signatures for certificate aggregation
    pub sig_lists: Vec<<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType>,
    /// A bitvec to indicate which node is active and send out a valid signature for certificate aggregation, this automatically do uniqueness check
    pub signers: BitVec,
    /// Phantom data to specify the types this accumulator is for
    pub phantom: PhantomData<(TYPES, VOTE, CERT)>,
}

impl<TYPES: NodeType, VOTE: Vote<TYPES>, CERT: Certificate<TYPES, Voteable = VOTE::Commitment>>
    VoteAccumulator<TYPES, VOTE, CERT>
{
    /// Add a vote to the total accumulated votes.  Returns the accumulator or the certificate if we
    /// have accumulated enough votes to exceed the threshold for creating a certificate.
    ///
    /// # Panics
    /// Panics if the vote comes from a node not in the stake table
    pub fn accumulate(mut self, vote: &VOTE, membership: &TYPES::Membership) -> Either<Self, CERT> {
        let key = vote.get_signing_key();

        let vote_commitment = vote.get_data_commitment();
        if !key.validate(&vote.get_signature(), vote_commitment.as_ref()) {
            error!("Invalid vote! Vote Data {:?}", vote.get_data());
            return Either::Left(self);
        }

        let Some(stake_table_entry) = membership.get_stake(&key) else {
            return Either::Left(self);
        };
        let stake_table = membership.get_committee_qc_stake_table();
        let vote_node_id = stake_table
            .iter()
            .position(|x| *x == stake_table_entry.clone())
            .unwrap();

        let encoded_key = key.to_bytes();

        // Deserialize the signature so that it can be assembeld into a QC
        // TODO ED Update this once we've gotten rid of EncodedSignature
        let original_signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType =
            bincode_opts()
                .deserialize(&vote.get_signature().0)
                .expect("Deserialization on the signature shouldn't be able to fail.");

        let (total_stake_casted, total_vote_map) = self
            .vote_outcomes
            .entry(vote_commitment)
            .or_insert_with(|| (U256::from(0), BTreeMap::new()));

        // Check for duplicate vote
        // TODO ED Re-encoding signature key to bytes until we get rid of EncodedKey
        // Have to do this because SignatureKey is not hashable
        if total_vote_map.contains_key(&encoded_key) {
            return Either::Left(self);
        }

        if self.signers.get(vote_node_id).as_deref() == Some(&true) {
            error!("Node id is already in signers list");
            return Either::Left(self);
        }
        self.signers.set(vote_node_id, true);
        self.sig_lists.push(original_signature);

        // TODO: Get the stake from the stake table entry.
        *total_stake_casted += stake_table_entry.get_stake();
        total_vote_map.insert(
            encoded_key,
            (vote.get_signature(), vote.get_data_commitment()),
        );

        if *total_stake_casted >= CERT::threshold(membership).into() {
            // Assemble QC
            let real_qc_pp: <<TYPES as NodeType>::SignatureKey as SignatureKey>::QCParams =
                <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                    stake_table,
                    U256::from(CERT::threshold(membership)),
                );

            let real_qc_sig = <TYPES::SignatureKey as SignatureKey>::assemble(
                &real_qc_pp,
                self.signers.as_bitslice(),
                &self.sig_lists[..],
            );

            let cert = CERT::create_signed_certificate(
                vote.get_data_commitment(),
                vote.get_data().clone(),
                real_qc_sig,
                vote.get_view_number(),
            );
            return Either::Right(cert);
        }
        Either::Left(self)
    }
}

/// Mapping of commitments to vote tokens by key.
type VoteMap2<COMMITMENT> = HashMap<
    COMMITMENT,
    (
        U256,
        BTreeMap<EncodedPublicKey, (EncodedSignature, COMMITMENT)>,
    ),
>;
