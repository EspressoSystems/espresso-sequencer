use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    num::NonZeroU64,
};

use bincode::Options;
use bitvec::vec::BitVec;
use commit::CommitmentBounds;
use either::Either;
use ethereum_types::U256;
use hotshot_utils::bincode::bincode_opts;
use tracing::error;

use crate::{
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{EncodedPublicKey, EncodedSignature, SignatureKey},
    },
};

pub trait Vote2<TYPES: NodeType> {
    type Membership: Membership<TYPES>;
    type Commitment: CommitmentBounds;

    fn get_signature(&self) -> EncodedSignature;
    fn get_data_commitment(&self) -> Self::Commitment;
    fn get_signing_key(&self) -> TYPES::SignatureKey;
    // fn create_signed_vote(Self::Commitment, Self::Membership) ??
}

pub trait ViewNumber<TYPES: NodeType> {
    fn get_view_number(&self) -> TYPES::Time;
}

pub trait Certificate2<TYPES: NodeType> {
    type Membership: Membership<TYPES>;
    type Commitment: CommitmentBounds;

    fn create_signed_certificate(
        data_commitment: Self::Commitment,
        sig: <TYPES::SignatureKey as SignatureKey>::QCType,
    ) -> Self;
    fn is_valid_cert(&self) -> bool;
    fn threshold() -> u64;
    fn get_data_commitment(&self) -> Self::Commitment;
}

pub struct VoteAccumulator2<
    TYPES: NodeType,
    VOTE: Vote2<TYPES>,
    CERT: Certificate2<TYPES, Commitment = VOTE::Commitment>,
> {
    /// Map of all signatures accumlated so far
    pub vote_outcomes: VoteMap2<VOTE::Commitment>,
    /// A quorum's worth of stake, generally 2f + 1
    pub success_threshold: NonZeroU64,
    /// A list of valid signatures for certificate aggregation
    pub sig_lists: Vec<<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType>,
    /// A bitvec to indicate which node is active and send out a valid signature for certificate aggregation, this automatically do uniqueness check
    pub signers: BitVec,
    /// Phantom data to specify the vote this accumulator is for
    pub phantom: PhantomData<(TYPES, VOTE, CERT)>,
}

impl<
        TYPES: NodeType,
        VOTE: Vote2<TYPES>,
        CERT: Certificate2<TYPES, Commitment = VOTE::Commitment>,
    > VoteAccumulator2<TYPES, VOTE, CERT>
{
    fn accumulate(mut self, vote: &VOTE, membership: &VOTE::Membership) -> Either<Self, CERT> {
        let key = vote.get_signing_key();

        let vote_commitment = vote.get_data_commitment();
        if !key.validate(
            &vote.get_signature(),
            &bincode_opts().serialize(&vote_commitment).unwrap(),
        ) {
            error!("Vote data is {:?}", vote.get_data_commitment());
            error!("Invalid vote! Data");
            return Either::Left(self);
        }

        // TODO: Lookup the actual stake
        let stake_table_entry: <<TYPES as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry = key.get_stake_table_entry(1u64);
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
            .or_insert_with(|| (0, BTreeMap::new()));

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
        *total_stake_casted += 1;
        total_vote_map.insert(
            encoded_key.clone(),
            (vote.get_signature(), vote.get_data_commitment()),
        );

        if *total_stake_casted >= u64::from(CERT::threshold()) {
            // Assemble QC
            let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
                stake_table.clone(),
                U256::from(self.success_threshold.get()),
            );

            let real_qc_sig = <TYPES::SignatureKey as SignatureKey>::assemble(
                &real_qc_pp,
                self.signers.as_bitslice(),
                &self.sig_lists[..],
            );

            let cert = CERT::create_signed_certificate(vote.get_data_commitment(), real_qc_sig);
            return Either::Right(cert);
        }
        Either::Left(self)
    }
}

/// Mapping of commitments to vote tokens by key.
type VoteMap2<COMMITMENT> = HashMap<
    COMMITMENT,
    (
        u64,
        BTreeMap<EncodedPublicKey, (EncodedSignature, COMMITMENT)>,
    ),
>;
