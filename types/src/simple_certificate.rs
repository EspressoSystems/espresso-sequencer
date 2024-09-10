// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Implementations of the simple certificate type.  Used for Quorum, DA, and Timeout Certificates

use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
};

use anyhow::{ensure, Result};
use async_lock::RwLock;
use committable::{Commitment, Committable};
use ethereum_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    data::serialize_signature2,
    message::UpgradeLock,
    simple_vote::{
        DaData, QuorumData, TimeoutData, UpgradeProposalData, VersionedVoteData,
        ViewSyncCommitData, ViewSyncFinalizeData, ViewSyncPreCommitData, Voteable,
    },
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType, Versions},
        signature_key::SignatureKey,
    },
    vote::{Certificate, HasViewNumber},
};

/// Trait which allows use to inject different threshold calculations into a Certificate type
pub trait Threshold<TYPES: NodeType> {
    /// Calculate a threshold based on the membership
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64;
}

/// Defines a threshold which is 2f + 1 (Amount needed for Quorum)
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct SuccessThreshold {}

impl<TYPES: NodeType> Threshold<TYPES> for SuccessThreshold {
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64 {
        membership.success_threshold().into()
    }
}

/// Defines a threshold which is f + 1 (i.e at least one of the stake is honest)
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct OneHonestThreshold {}

impl<TYPES: NodeType> Threshold<TYPES> for OneHonestThreshold {
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64 {
        membership.failure_threshold().into()
    }
}

/// Defines a threshold which is 0.9n + 1 (i.e. over 90% of the nodes with stake)
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct UpgradeThreshold {}

impl<TYPES: NodeType> Threshold<TYPES> for UpgradeThreshold {
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64 {
        membership.upgrade_threshold().into()
    }
}

/// A certificate which can be created by aggregating many simple votes on the commitment.
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct SimpleCertificate<TYPES: NodeType, VOTEABLE: Voteable, THRESHOLD: Threshold<TYPES>> {
    /// The data this certificate is for.  I.e the thing that was voted on to create this Certificate
    pub data: VOTEABLE,
    /// commitment of all the votes this cert should be signed over
    vote_commitment: Commitment<VOTEABLE>,
    /// Which view this QC relates to
    pub view_number: TYPES::Time,
    /// assembled signature for certificate aggregation
    pub signatures: Option<<TYPES::SignatureKey as SignatureKey>::QcType>,
    /// phantom data for `THRESHOLD` and `TYPES`
    pub _pd: PhantomData<(TYPES, THRESHOLD)>,
}

impl<TYPES: NodeType, VOTEABLE: Voteable, THRESHOLD: Threshold<TYPES>>
    SimpleCertificate<TYPES, VOTEABLE, THRESHOLD>
{
    /// Creates a new instance of `SimpleCertificate`
    pub fn new(
        data: VOTEABLE,
        vote_commitment: Commitment<VOTEABLE>,
        view_number: TYPES::Time,
        signatures: Option<<TYPES::SignatureKey as SignatureKey>::QcType>,
        pd: PhantomData<(TYPES, THRESHOLD)>,
    ) -> Self {
        Self {
            data,
            vote_commitment,
            view_number,
            signatures,
            _pd: pd,
        }
    }
}

impl<TYPES: NodeType, VOTEABLE: Voteable + Committable, THRESHOLD: Threshold<TYPES>> Committable
    for SimpleCertificate<TYPES, VOTEABLE, THRESHOLD>
{
    fn commit(&self) -> Commitment<Self> {
        let signature_bytes = match self.signatures.as_ref() {
            Some(sigs) => serialize_signature2::<TYPES>(sigs),
            None => vec![],
        };
        committable::RawCommitmentBuilder::new("Certificate")
            .field("data", self.data.commit())
            .field("vote_commitment", self.vote_commitment)
            .field("view number", self.view_number.commit())
            .var_size_field("signatures", &signature_bytes)
            .finalize()
    }
}

impl<TYPES: NodeType, VOTEABLE: Voteable + 'static, THRESHOLD: Threshold<TYPES>> Certificate<TYPES>
    for SimpleCertificate<TYPES, VOTEABLE, THRESHOLD>
{
    type Voteable = VOTEABLE;
    type Threshold = THRESHOLD;

    fn create_signed_certificate<V: Versions>(
        vote_commitment: Commitment<VersionedVoteData<TYPES, VOTEABLE, V>>,
        data: Self::Voteable,
        sig: <TYPES::SignatureKey as SignatureKey>::QcType,
        view: TYPES::Time,
    ) -> Self {
        let vote_commitment_bytes: [u8; 32] = vote_commitment.into();

        SimpleCertificate {
            data,
            vote_commitment: Commitment::from_raw(vote_commitment_bytes),
            view_number: view,
            signatures: Some(sig),
            _pd: PhantomData,
        }
    }
    async fn is_valid_cert<MEMBERSHIP: Membership<TYPES>, V: Versions>(
        &self,
        membership: &MEMBERSHIP,
        upgrade_lock: &UpgradeLock<TYPES, V>,
    ) -> bool {
        if self.view_number == TYPES::Time::genesis() {
            return true;
        }
        let real_qc_pp = <TYPES::SignatureKey as SignatureKey>::public_parameter(
            membership.stake_table(),
            U256::from(Self::threshold(membership)),
        );
        let Ok(commit) = self.date_commitment(upgrade_lock).await else {
            return false;
        };
        <TYPES::SignatureKey as SignatureKey>::check(
            &real_qc_pp,
            commit.as_ref(),
            self.signatures.as_ref().unwrap(),
        )
    }
    fn threshold<MEMBERSHIP: Membership<TYPES>>(membership: &MEMBERSHIP) -> u64 {
        THRESHOLD::threshold(membership)
    }
    fn date(&self) -> &Self::Voteable {
        &self.data
    }
    async fn date_commitment<V: Versions>(
        &self,
        upgrade_lock: &UpgradeLock<TYPES, V>,
    ) -> Result<Commitment<VersionedVoteData<TYPES, VOTEABLE, V>>> {
        Ok(
            VersionedVoteData::new(self.data.clone(), self.view_number, upgrade_lock)
                .await?
                .commit(),
        )
    }
}

impl<TYPES: NodeType, VOTEABLE: Voteable + 'static, THRESHOLD: Threshold<TYPES>>
    HasViewNumber<TYPES> for SimpleCertificate<TYPES, VOTEABLE, THRESHOLD>
{
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}
impl<TYPES: NodeType> Display for QuorumCertificate<TYPES> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "view: {:?}", self.view_number)
    }
}

impl<TYPES: NodeType> UpgradeCertificate<TYPES> {
    // TODO: Replace this function with `is_relevant` after the following issue is done:
    // https://github.com/EspressoSystems/HotShot/issues/3357.
    /// Determines whether or not a certificate is relevant (i.e. we still have time to reach a
    /// decide)
    ///
    /// # Errors
    /// Returns an error when the certificate is no longer relevant
    pub fn temp_is_relevant(
        &self,
        view_number: TYPES::Time,
        decided_upgrade_certificate: Option<Self>,
    ) -> Result<()> {
        ensure!(
            self.data.decide_by >= view_number
                || decided_upgrade_certificate.is_some_and(|cert| cert == *self),
            "Upgrade certificate is no longer relevant."
        );

        Ok(())
    }

    /// Determines whether or not a certificate is relevant (i.e. we still have time to reach a
    /// decide)
    ///
    /// # Errors
    /// Returns an error when the certificate is no longer relevant
    pub async fn is_relevant(
        &self,
        view_number: TYPES::Time,
        decided_upgrade_certificate: Arc<RwLock<Option<Self>>>,
    ) -> Result<()> {
        let decided_upgrade_certificate_read = decided_upgrade_certificate.read().await;
        ensure!(
            self.data.decide_by >= view_number
                || decided_upgrade_certificate_read
                    .clone()
                    .is_some_and(|cert| cert == *self),
            "Upgrade certificate is no longer relevant."
        );

        Ok(())
    }

    /// Validate an upgrade certificate.
    /// # Errors
    /// Returns an error when the upgrade certificate is invalid.
    pub async fn validate<V: Versions>(
        upgrade_certificate: &Option<Self>,
        quorum_membership: &TYPES::Membership,
        upgrade_lock: &UpgradeLock<TYPES, V>,
    ) -> Result<()> {
        if let Some(ref cert) = upgrade_certificate {
            ensure!(
                cert.is_valid_cert(quorum_membership, upgrade_lock).await,
                "Invalid upgrade certificate."
            );
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Given an upgrade certificate and a view, tests whether the view is in the period
    /// where we are upgrading, which requires that we propose with null blocks.
    pub fn upgrading_in(&self, view: TYPES::Time) -> bool {
        view > self.data.old_version_last_view && view < self.data.new_version_first_view
    }
}

/// Type alias for a `QuorumCertificate`, which is a `SimpleCertificate` of `QuorumVotes`
pub type QuorumCertificate<TYPES> = SimpleCertificate<TYPES, QuorumData<TYPES>, SuccessThreshold>;
/// Type alias for a DA certificate over `DaData`
pub type DaCertificate<TYPES> = SimpleCertificate<TYPES, DaData, SuccessThreshold>;
/// Type alias for a Timeout certificate over a view number
pub type TimeoutCertificate<TYPES> = SimpleCertificate<TYPES, TimeoutData<TYPES>, SuccessThreshold>;
/// Type alias for a `ViewSyncPreCommit` certificate over a view number
pub type ViewSyncPreCommitCertificate2<TYPES> =
    SimpleCertificate<TYPES, ViewSyncPreCommitData<TYPES>, OneHonestThreshold>;
/// Type alias for a `ViewSyncCommit` certificate over a view number
pub type ViewSyncCommitCertificate2<TYPES> =
    SimpleCertificate<TYPES, ViewSyncCommitData<TYPES>, SuccessThreshold>;
/// Type alias for a `ViewSyncFinalize` certificate over a view number
pub type ViewSyncFinalizeCertificate2<TYPES> =
    SimpleCertificate<TYPES, ViewSyncFinalizeData<TYPES>, SuccessThreshold>;
/// Type alias for a `UpgradeCertificate`, which is a `SimpleCertificate` of `UpgradeProposalData`
pub type UpgradeCertificate<TYPES> =
    SimpleCertificate<TYPES, UpgradeProposalData<TYPES>, UpgradeThreshold>;
