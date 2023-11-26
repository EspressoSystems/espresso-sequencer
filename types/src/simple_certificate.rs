//! Implementations of the simple certificate type.  Used for Quorum, DA, and Timeout Certificates

use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use commit::{Commitment, CommitmentBoundsArkless, Committable};
use ethereum_types::U256;

use crate::{
    data::Leaf,
    simple_vote::{
        DAData, QuorumData, TimeoutData, VIDData, ViewSyncCommitData, ViewSyncFinalizeData,
        ViewSyncPreCommitData, Voteable,
    },
    traits::{
        election::Membership, node_implementation::NodeType, signature_key::SignatureKey,
        state::ConsensusTime,
    },
    vote::{Certificate, HasViewNumber},
};

use serde::{Deserialize, Serialize};

/// A certificate which can be created by aggregating many simple votes on the commitment.
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct SimpleCertificate<TYPES: NodeType, VOTEABLE: Voteable> {
    /// The data this certificate is for.  I.e the thing that was voted on to create this Certificate
    pub data: VOTEABLE,
    /// commitment of all the votes this cert should be signed over
    pub vote_commitment: Commitment<VOTEABLE>,
    /// Which view this QC relates to
    pub view_number: TYPES::Time,
    /// assembled signature for certificate aggregation
    pub signatures: Option<<TYPES::SignatureKey as SignatureKey>::QCType>,
    /// If this QC is for the genesis block
    pub is_genesis: bool,
    /// phantom data for `MEMBERSHIP` and `TYPES`
    pub _pd: PhantomData<TYPES>,
}

impl<TYPES: NodeType, VOTEABLE: Voteable + 'static> Certificate<TYPES>
    for SimpleCertificate<TYPES, VOTEABLE>
{
    type Voteable = VOTEABLE;

    fn create_signed_certificate(
        vote_commitment: Commitment<VOTEABLE>,
        data: Self::Voteable,
        sig: <TYPES::SignatureKey as SignatureKey>::QCType,
        view: TYPES::Time,
    ) -> Self {
        SimpleCertificate {
            data,
            vote_commitment,
            view_number: view,
            signatures: Some(sig),
            is_genesis: false,
            _pd: PhantomData,
        }
    }
    fn is_valid_cert<MEMBERSHIP: Membership<TYPES>>(&self, membership: &MEMBERSHIP) -> bool {
        if self.is_genesis && self.view_number == TYPES::Time::genesis() {
            return true;
        }
        let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
            membership.get_committee_qc_stake_table(),
            U256::from(membership.success_threshold().get()),
        );
        <TYPES::SignatureKey as SignatureKey>::check(
            &real_qc_pp,
            self.vote_commitment.as_ref(),
            self.signatures.as_ref().unwrap(),
        )
    }
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64 {
        membership.success_threshold().into()
    }
    fn get_data(&self) -> &Self::Voteable {
        &self.data
    }
    fn get_data_commitment(&self) -> Commitment<Self::Voteable> {
        self.vote_commitment
    }
}

impl<TYPES: NodeType, VOTEABLE: Voteable + 'static> HasViewNumber<TYPES>
    for SimpleCertificate<TYPES, VOTEABLE>
{
    fn get_view_number(&self) -> TYPES::Time {
        self.view_number
    }
}
impl<TYPES: NodeType> Display for QuorumCertificate<TYPES> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "view: {:?}, is_genesis: {:?}",
            self.view_number, self.is_genesis
        )
    }
}

impl<TYPES: NodeType> QuorumCertificate<TYPES> {
    #[must_use]
    /// Creat the Genisis certificate
    pub fn genesis() -> Self {
        let data = QuorumData {
            leaf_commit: Commitment::<Leaf<TYPES>>::default_commitment_no_preimage(),
        };
        let commit = data.commit();
        Self {
            data,
            vote_commitment: commit,
            view_number: <TYPES::Time as ConsensusTime>::genesis(),
            signatures: None,
            is_genesis: true,
            _pd: PhantomData,
        }
    }
}

/// Type alias for a `QuorumCertificate`, which is a `SimpleCertificate` of `QuorumVotes`
pub type QuorumCertificate<TYPES> = SimpleCertificate<TYPES, QuorumData<TYPES>>;
/// Type alias for a DA certificate over `DAData`
pub type DACertificate<TYPES> = SimpleCertificate<TYPES, DAData<<TYPES as NodeType>::BlockPayload>>;
/// Type alias for a Timeout certificate over a view number
pub type TimeoutCertificate<TYPES> = SimpleCertificate<TYPES, TimeoutData<TYPES>>;
/// type alias for a VID certificate
pub type VIDCertificate<TYPES> =
    SimpleCertificate<TYPES, VIDData<<TYPES as NodeType>::BlockPayload>>;

// TODO ED Update this to use the correct threshold instead of the default `success_threshold`
/// Type alias for a `ViewSyncPreCommit` certificate over a view number
pub type ViewSyncPreCommitCertificate2<TYPES> =
    SimpleCertificate<TYPES, ViewSyncPreCommitData<TYPES>>;
/// Type alias for a `ViewSyncCommit` certificate over a view number
pub type ViewSyncCommitCertificate2<TYPES> = SimpleCertificate<TYPES, ViewSyncCommitData<TYPES>>;
/// Type alias for a `ViewSyncFinalize` certificate over a view number
pub type ViewSyncFinalizeCertificate2<TYPES> =
    SimpleCertificate<TYPES, ViewSyncFinalizeData<TYPES>>;
