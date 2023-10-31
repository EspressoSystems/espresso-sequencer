#![allow(dead_code)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(missing_docs)]

use std::marker::PhantomData;

use commit::Commitment;
use ethereum_types::U256;

use crate::{
    simple_vote::Voteable,
    traits::{
        election::Membership, node_implementation::NodeType, signature_key::SignatureKey,
        state::ConsensusTime,
    },
    vote2::Certificate2,
};

/// A certificate which can be created by aggregating many simple votes on the commitment.
pub struct SimpleCertificate<TYPES: NodeType, VOTEABLE: Voteable, MEMBERSHIP: Membership<TYPES>> {
    /// commitment to previous leaf
    pub leaf_commitment: VOTEABLE,
    /// commitment of all the voetes this should be signed over
    pub vote_commitment: Commitment<VOTEABLE>,
    /// Which view this QC relates to
    pub view_number: TYPES::Time,
    /// assembled signature for certificate aggregation
    pub signatures: <TYPES::SignatureKey as SignatureKey>::QCType,
    /// If this QC is for the genesis block
    pub is_genesis: bool,
    /// phantom data for `MEMBERSHIP` and `TYPES`
    _pd: PhantomData<(TYPES, MEMBERSHIP)>,
}

impl<TYPES: NodeType, VOTEABLE: Voteable + 'static, MEMBERSHIP: Membership<TYPES>>
    Certificate2<TYPES> for SimpleCertificate<TYPES, VOTEABLE, MEMBERSHIP>
{
    type Commitment = VOTEABLE;
    type Membership = MEMBERSHIP;

    fn create_signed_certificate(
        vote_commitment: Commitment<VOTEABLE>,
        data: Self::Commitment,
        sig: <TYPES::SignatureKey as SignatureKey>::QCType,
        view: TYPES::Time,
    ) -> Self {
        SimpleCertificate {
            leaf_commitment: data,
            vote_commitment,
            view_number: view,
            signatures: sig,
            is_genesis: false,
            _pd: PhantomData,
        }
    }
    fn is_valid_cert(
        &self,
        vote_commitment: Commitment<VOTEABLE>,
        membership: &MEMBERSHIP,
    ) -> bool {
        if vote_commitment != self.vote_commitment {
            return false;
        }
        if self.is_genesis && self.view_number == TYPES::Time::genesis() {
            return true;
        }
        let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
            membership.get_committee_qc_stake_table(),
            U256::from(membership.success_threshold().get()),
        );
        <TYPES::SignatureKey as SignatureKey>::check(
            &real_qc_pp,
            vote_commitment.as_ref(),
            &self.signatures,
        )
    }
    fn threshold(membership: &MEMBERSHIP) -> u64 {
        membership.success_threshold().into()
    }
    fn get_data(&self) -> &Self::Commitment {
        &self.leaf_commitment
    }
    fn get_data_commitment(&self) -> Commitment<Self::Commitment> {
        self.vote_commitment
    }
}
