// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Provides types useful for representing `HotShot`'s data structures
//!
//! This module provides types for representing consensus internal state, such as leaves,
//! `HotShot`'s version of a block, and proposals, messages upon which to reach the consensus.

use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
};

use anyhow::{ensure, Result};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;
use bincode::Options;
use committable::{Commitment, CommitmentBoundsArkless, Committable, RawCommitmentBuilder};
use derivative::Derivative;
use jf_vid::{precomputable::Precomputable, VidDisperse as JfVidDisperse, VidScheme};
use rand::Rng;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::spawn_blocking;
use tracing::error;
use vec1::Vec1;

use crate::{
    message::{Proposal, UpgradeLock},
    simple_certificate::{
        QuorumCertificate, TimeoutCertificate, UpgradeCertificate, ViewSyncFinalizeCertificate2,
    },
    simple_vote::{QuorumData, UpgradeProposalData, VersionedVoteData},
    traits::{
        block_contents::{
            vid_commitment, BlockHeader, BuilderFee, EncodeBytes, TestableBlock,
            GENESIS_VID_NUM_STORAGE_NODES,
        },
        election::Membership,
        node_implementation::{ConsensusTime, NodeType, Versions},
        signature_key::SignatureKey,
        states::TestableState,
        BlockPayload,
    },
    utils::bincode_opts,
    vid::{vid_scheme, VidCommitment, VidCommon, VidPrecomputeData, VidSchemeType, VidShare},
    vote::{Certificate, HasViewNumber},
};

/// Type-safe wrapper around `u64` so we know the thing we're talking about is a view number.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
pub struct ViewNumber(u64);

impl ConsensusTime for ViewNumber {
    /// Create a genesis view number (0)
    fn genesis() -> Self {
        Self(0)
    }
    /// Create a new `ViewNumber` with the given value.
    fn new(n: u64) -> Self {
        Self(n)
    }
    /// Returen the u64 format
    fn u64(&self) -> u64 {
        self.0
    }
}

impl Committable for ViewNumber {
    fn commit(&self) -> Commitment<Self> {
        let builder = RawCommitmentBuilder::new("View Number Commitment");
        builder.u64(self.0).finalize()
    }
}

impl std::ops::Add<u64> for ViewNumber {
    type Output = ViewNumber;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::AddAssign<u64> for ViewNumber {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl std::ops::Deref for ViewNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Sub<u64> for ViewNumber {
    type Output = ViewNumber;
    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

/// A proposal to start providing data availability for a block.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(bound = "TYPES: NodeType")]
pub struct DaProposal<TYPES: NodeType> {
    /// Encoded transactions in the block to be applied.
    pub encoded_transactions: Arc<[u8]>,
    /// Metadata of the block to be applied.
    pub metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    /// View this proposal applies to
    pub view_number: TYPES::Time,
}

/// A proposal to upgrade the network
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(bound = "TYPES: NodeType")]
pub struct UpgradeProposal<TYPES>
where
    TYPES: NodeType,
{
    /// The information about which version we are upgrading to.
    pub upgrade_proposal: UpgradeProposalData<TYPES>,
    /// View this proposal applies to
    pub view_number: TYPES::Time,
}

/// VID dispersal data
///
/// Like [`DaProposal`].
///
/// TODO move to vid.rs?
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct VidDisperse<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::Time,
    /// Block payload commitment
    pub payload_commitment: VidCommitment,
    /// A storage node's key and its corresponding VID share
    pub shares: BTreeMap<TYPES::SignatureKey, VidShare>,
    /// VID common data sent to all storage nodes
    pub common: VidCommon,
}

impl<TYPES: NodeType> VidDisperse<TYPES> {
    /// Create VID dispersal from a specified membership
    /// Uses the specified function to calculate share dispersal
    /// Allows for more complex stake table functionality
    pub fn from_membership(
        view_number: TYPES::Time,
        mut vid_disperse: JfVidDisperse<VidSchemeType>,
        membership: &TYPES::Membership,
    ) -> Self {
        let shares = membership
            .committee_members(view_number)
            .iter()
            .map(|node| (node.clone(), vid_disperse.shares.remove(0)))
            .collect();

        Self {
            view_number,
            shares,
            common: vid_disperse.common,
            payload_commitment: vid_disperse.commit,
        }
    }

    /// Calculate the vid disperse information from the payload given a view and membership,
    /// optionally using precompute data from builder
    ///
    /// # Panics
    /// Panics if the VID calculation fails, this should not happen.
    #[allow(clippy::panic)]
    pub async fn calculate_vid_disperse(
        txns: Arc<[u8]>,
        membership: &Arc<TYPES::Membership>,
        view: TYPES::Time,
        precompute_data: Option<VidPrecomputeData>,
    ) -> Self {
        let num_nodes = membership.total_nodes();

        let vid_disperse = spawn_blocking(move || {
            precompute_data
                .map_or_else(
                    || vid_scheme(num_nodes).disperse(Arc::clone(&txns)),
                    |data| vid_scheme(num_nodes).disperse_precompute(Arc::clone(&txns), &data)
                )
                .unwrap_or_else(|err| panic!("VID disperse failure:(num_storage nodes,payload_byte_len)=({num_nodes},{}) error: {err}", txns.len()))
        }).await;
        #[cfg(async_executor_impl = "tokio")]
        // Tokio's JoinHandle's `Output` is `Result<T, JoinError>`, while in async-std it's just `T`
        // Unwrap here will just propagate any panic from the spawned task, it's not a new place we can panic.
        let vid_disperse = vid_disperse.unwrap();

        Self::from_membership(view, vid_disperse, membership.as_ref())
    }
}

/// Helper type to encapsulate the various ways that proposal certificates can be captured and
/// stored.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub enum ViewChangeEvidence<TYPES: NodeType> {
    /// Holds a timeout certificate.
    Timeout(TimeoutCertificate<TYPES>),
    /// Holds a view sync finalized certificate.
    ViewSync(ViewSyncFinalizeCertificate2<TYPES>),
}

impl<TYPES: NodeType> ViewChangeEvidence<TYPES> {
    /// Check that the given ViewChangeEvidence is relevant to the current view.
    pub fn is_valid_for_view(&self, view: &TYPES::Time) -> bool {
        match self {
            ViewChangeEvidence::Timeout(timeout_cert) => timeout_cert.date().view == *view - 1,
            ViewChangeEvidence::ViewSync(view_sync_cert) => view_sync_cert.view_number == *view,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
/// VID share and associated metadata for a single node
pub struct VidDisperseShare<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::Time,
    /// Block payload commitment
    pub payload_commitment: VidCommitment,
    /// A storage node's key and its corresponding VID share
    pub share: VidShare,
    /// VID common data sent to all storage nodes
    pub common: VidCommon,
    /// a public key of the share recipient
    pub recipient_key: TYPES::SignatureKey,
}

impl<TYPES: NodeType> VidDisperseShare<TYPES> {
    /// Create a vector of `VidDisperseShare` from `VidDisperse`
    pub fn from_vid_disperse(vid_disperse: VidDisperse<TYPES>) -> Vec<Self> {
        vid_disperse
            .shares
            .into_iter()
            .map(|(recipient_key, share)| VidDisperseShare {
                share,
                recipient_key,
                view_number: vid_disperse.view_number,
                common: vid_disperse.common.clone(),
                payload_commitment: vid_disperse.payload_commitment,
            })
            .collect()
    }

    /// Consume `self` and return a `Proposal`
    pub fn to_proposal(
        self,
        private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Option<Proposal<TYPES, Self>> {
        let Ok(signature) =
            TYPES::SignatureKey::sign(private_key, self.payload_commitment.as_ref())
        else {
            error!("VID: failed to sign dispersal share payload");
            return None;
        };
        Some(Proposal {
            signature,
            _pd: PhantomData,
            data: self,
        })
    }

    /// Create `VidDisperse` out of an iterator to `VidDisperseShare`s
    pub fn to_vid_disperse<'a, I>(mut it: I) -> Option<VidDisperse<TYPES>>
    where
        I: Iterator<Item = &'a VidDisperseShare<TYPES>>,
    {
        let first_vid_disperse_share = it.next()?.clone();
        let mut share_map = BTreeMap::new();
        share_map.insert(
            first_vid_disperse_share.recipient_key,
            first_vid_disperse_share.share,
        );
        let mut vid_disperse = VidDisperse {
            view_number: first_vid_disperse_share.view_number,
            payload_commitment: first_vid_disperse_share.payload_commitment,
            common: first_vid_disperse_share.common,
            shares: share_map,
        };
        let _ = it.map(|vid_disperse_share| {
            vid_disperse.shares.insert(
                vid_disperse_share.recipient_key.clone(),
                vid_disperse_share.share.clone(),
            )
        });
        Some(vid_disperse)
    }

    /// Split a VID share proposal into a proposal for each recipient.
    pub fn to_vid_share_proposals(
        vid_disperse_proposal: Proposal<TYPES, VidDisperse<TYPES>>,
    ) -> Vec<Proposal<TYPES, VidDisperseShare<TYPES>>> {
        vid_disperse_proposal
            .data
            .shares
            .into_iter()
            .map(|(recipient_key, share)| Proposal {
                data: VidDisperseShare {
                    share,
                    recipient_key,
                    view_number: vid_disperse_proposal.data.view_number,
                    common: vid_disperse_proposal.data.common.clone(),
                    payload_commitment: vid_disperse_proposal.data.payload_commitment,
                },
                signature: vid_disperse_proposal.signature.clone(),
                _pd: vid_disperse_proposal._pd,
            })
            .collect()
    }
}

/// Proposal to append a block.
#[derive(custom_debug::Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(bound(deserialize = ""))]
pub struct QuorumProposal<TYPES: NodeType> {
    /// The block header to append
    pub block_header: TYPES::BlockHeader,

    /// CurView from leader when proposing leaf
    pub view_number: TYPES::Time,

    /// Per spec, justification
    pub justify_qc: QuorumCertificate<TYPES>,

    /// Possible upgrade certificate, which the leader may optionally attach.
    pub upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// Possible timeout or view sync certificate.
    /// - A timeout certificate is only present if the justify_qc is not for the preceding view
    /// - A view sync certificate is only present if the justify_qc and timeout_cert are not
    /// present.
    pub proposal_certificate: Option<ViewChangeEvidence<TYPES>>,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for DaProposal<TYPES> {
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for VidDisperse<TYPES> {
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for VidDisperseShare<TYPES> {
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for QuorumProposal<TYPES> {
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for UpgradeProposal<TYPES> {
    fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
}

/// The error type for block and its transactions.
#[derive(Snafu, Debug, Serialize, Deserialize)]
pub enum BlockError {
    /// Invalid block header.
    InvalidBlockHeader,
    /// Invalid transaction length.
    InvalidTransactionLength,
    /// Inconsistent payload commitment.
    InconsistentPayloadCommitment,
}

/// Additional functions required to use a [`Leaf`] with hotshot-testing.
pub trait TestableLeaf {
    /// Type of nodes participating in the network.
    type NodeType: NodeType;

    /// Create a transaction that can be added to the block contained in this leaf.
    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockPayload as BlockPayload<Self::NodeType>>::Transaction;
}

/// This is the consensus-internal analogous concept to a block, and it contains the block proper,
/// as well as the hash of its parent `Leaf`.
/// NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::BlockPayload`
#[derive(Serialize, Deserialize, Clone, Debug, Derivative, Eq)]
#[serde(bound(deserialize = ""))]
pub struct Leaf<TYPES: NodeType> {
    /// CurView from leader when proposing leaf
    view_number: TYPES::Time,

    /// Per spec, justification
    justify_qc: QuorumCertificate<TYPES>,

    /// The hash of the parent `Leaf`
    /// So we can ask if it extends
    parent_commitment: Commitment<Self>,

    /// Block header.
    block_header: TYPES::BlockHeader,

    /// Optional upgrade certificate, if one was attached to the quorum proposal for this view.
    upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// Optional block payload.
    ///
    /// It may be empty for nodes not in the DA committee.
    block_payload: Option<TYPES::BlockPayload>,
}

impl<TYPES: NodeType> Leaf<TYPES> {
    #[allow(clippy::unused_async)]
    /// Calculate the leaf commitment,
    /// which is gated on the version to include the block header.
    pub async fn commit<V: Versions>(
        &self,
        _upgrade_lock: &UpgradeLock<TYPES, V>,
    ) -> Commitment<Self> {
        <Self as Committable>::commit(self)
    }
}

impl<TYPES: NodeType> PartialEq for Leaf<TYPES> {
    fn eq(&self, other: &Self) -> bool {
        self.view_number == other.view_number
            && self.justify_qc == other.justify_qc
            && self.parent_commitment == other.parent_commitment
            && self.block_header == other.block_header
    }
}

impl<TYPES: NodeType> Hash for Leaf<TYPES> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.view_number.hash(state);
        self.justify_qc.hash(state);
        self.parent_commitment.hash(state);
        self.block_header.hash(state);
    }
}

impl<TYPES: NodeType> Display for Leaf<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "view: {:?}, height: {:?}, justify: {}",
            self.view_number,
            self.height(),
            self.justify_qc
        )
    }
}

impl<TYPES: NodeType> QuorumCertificate<TYPES> {
    #[must_use]
    /// Creat the Genesis certificate
    pub async fn genesis<V: Versions>(
        validated_state: &TYPES::ValidatedState,
        instance_state: &TYPES::InstanceState,
    ) -> Self {
        // since this is genesis, we should never have a decided upgrade certificate.
        let upgrade_lock = UpgradeLock::<TYPES, V>::new();

        let genesis_view = <TYPES::Time as ConsensusTime>::genesis();

        let data = QuorumData {
            leaf_commit: Leaf::genesis(validated_state, instance_state)
                .await
                .commit(&upgrade_lock)
                .await,
        };

        let versioned_data =
            VersionedVoteData::<_, _, V>::new_infallible(data.clone(), genesis_view, &upgrade_lock)
                .await;

        let bytes: [u8; 32] = versioned_data.commit().into();

        Self::new(
            data,
            Commitment::from_raw(bytes),
            genesis_view,
            None,
            PhantomData,
        )
    }
}

impl<TYPES: NodeType> Leaf<TYPES> {
    /// Create a new leaf from its components.
    ///
    /// # Panics
    ///
    /// Panics if the genesis payload (`TYPES::BlockPayload::genesis()`) is malformed (unable to be
    /// interpreted as bytes).
    #[must_use]
    pub async fn genesis(
        validated_state: &TYPES::ValidatedState,
        instance_state: &TYPES::InstanceState,
    ) -> Self {
        let (payload, metadata) =
            TYPES::BlockPayload::from_transactions([], validated_state, instance_state)
                .await
                .unwrap();
        let builder_commitment = payload.builder_commitment(&metadata);
        let payload_bytes = payload.encode();

        let payload_commitment = vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES);

        let block_header = TYPES::BlockHeader::genesis(
            instance_state,
            payload_commitment,
            builder_commitment,
            metadata,
        );

        let null_quorum_data = QuorumData {
            leaf_commit: Commitment::<Leaf<TYPES>>::default_commitment_no_preimage(),
        };

        let justify_qc = QuorumCertificate::new(
            null_quorum_data.clone(),
            null_quorum_data.commit(),
            <TYPES::Time as ConsensusTime>::genesis(),
            None,
            PhantomData,
        );

        Self {
            view_number: TYPES::Time::genesis(),
            justify_qc,
            parent_commitment: null_quorum_data.leaf_commit,
            upgrade_certificate: None,
            block_header: block_header.clone(),
            block_payload: Some(payload),
        }
    }

    /// Time when this leaf was created.
    pub fn view_number(&self) -> TYPES::Time {
        self.view_number
    }
    /// Height of this leaf in the chain.
    ///
    /// Equivalently, this is the number of leaves before this one in the chain.
    pub fn height(&self) -> u64 {
        self.block_header.block_number()
    }
    /// The QC linking this leaf to its parent in the chain.
    pub fn justify_qc(&self) -> QuorumCertificate<TYPES> {
        self.justify_qc.clone()
    }
    /// The QC linking this leaf to its parent in the chain.
    pub fn upgrade_certificate(&self) -> Option<UpgradeCertificate<TYPES>> {
        self.upgrade_certificate.clone()
    }
    /// Commitment to this leaf's parent.
    pub fn parent_commitment(&self) -> Commitment<Self> {
        self.parent_commitment
    }
    /// The block header contained in this leaf.
    pub fn block_header(&self) -> &<TYPES as NodeType>::BlockHeader {
        &self.block_header
    }

    /// Get a mutable reference to the block header contained in this leaf.
    pub fn block_header_mut(&mut self) -> &mut <TYPES as NodeType>::BlockHeader {
        &mut self.block_header
    }
    /// Fill this leaf with the block payload.
    ///
    /// # Errors
    ///
    /// Fails if the payload commitment doesn't match `self.block_header.payload_commitment()`
    /// or if the transactions are of invalid length
    pub fn fill_block_payload(
        &mut self,
        block_payload: TYPES::BlockPayload,
        num_storage_nodes: usize,
    ) -> Result<(), BlockError> {
        let encoded_txns = block_payload.encode();
        let commitment = vid_commitment(&encoded_txns, num_storage_nodes);
        if commitment != self.block_header.payload_commitment() {
            return Err(BlockError::InconsistentPayloadCommitment);
        }
        self.block_payload = Some(block_payload);
        Ok(())
    }

    /// Fill this leaf with the block payload, without checking
    /// header and payload consistency
    pub fn fill_block_payload_unchecked(&mut self, block_payload: TYPES::BlockPayload) {
        self.block_payload = Some(block_payload);
    }

    /// Optional block payload.
    pub fn block_payload(&self) -> Option<TYPES::BlockPayload> {
        self.block_payload.clone()
    }

    /// A commitment to the block payload contained in this leaf.
    pub fn payload_commitment(&self) -> VidCommitment {
        self.block_header().payload_commitment()
    }

    // TODO: Replace this function with `extends_upgrade` after the following issue is done:
    // https://github.com/EspressoSystems/HotShot/issues/3357.
    /// Validate that a leaf has the right upgrade certificate to be the immediate child of another leaf
    ///
    /// This may not be a complete function. Please double-check that it performs the checks you expect before subtituting validation logic with it.
    ///
    /// # Errors
    /// Returns an error if the certificates are not identical, or that when we no longer see a
    /// cert, it's for the right reason.
    pub fn temp_extends_upgrade(
        &self,
        parent: &Self,
        decided_upgrade_certificate: &Option<UpgradeCertificate<TYPES>>,
    ) -> Result<()> {
        match (self.upgrade_certificate(), parent.upgrade_certificate()) {
            // Easiest cases are:
            //   - no upgrade certificate on either: this is the most common case, and is always fine.
            //   - if the parent didn't have a certificate, but we see one now, it just means that we have begun an upgrade: again, this is always fine.
            (None | Some(_), None) => {}
            // If we no longer see a cert, we have to make sure that we either:
            //    - no longer care because we have passed new_version_first_view, or
            //    - no longer care because we have passed `decide_by` without deciding the certificate.
            (None, Some(parent_cert)) => {
                ensure!(self.view_number() > parent_cert.data.new_version_first_view
                    || (self.view_number() > parent_cert.data.decide_by && decided_upgrade_certificate.is_none()),
                       "The new leaf is missing an upgrade certificate that was present in its parent, and should still be live."
                );
            }
            // If we both have a certificate, they should be identical.
            // Technically, this prevents us from initiating a new upgrade in the view immediately following an upgrade.
            // I think this is a fairly lax restriction.
            (Some(cert), Some(parent_cert)) => {
                ensure!(cert == parent_cert, "The new leaf does not extend the parent leaf, because it has attached a different upgrade certificate.");
            }
        }

        // This check should be added once we sort out the genesis leaf/justify_qc issue.
        // ensure!(self.parent_commitment() == parent_leaf.commit(), "The commitment of the parent leaf does not match the specified parent commitment.");

        Ok(())
    }

    /// Validate that a leaf has the right upgrade certificate to be the immediate child of another leaf
    ///
    /// This may not be a complete function. Please double-check that it performs the checks you expect before subtituting validation logic with it.
    ///
    /// # Errors
    /// Returns an error if the certificates are not identical, or that when we no longer see a
    /// cert, it's for the right reason.
    pub async fn extends_upgrade(
        &self,
        parent: &Self,
        decided_upgrade_certificate: &Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    ) -> Result<()> {
        match (self.upgrade_certificate(), parent.upgrade_certificate()) {
            // Easiest cases are:
            //   - no upgrade certificate on either: this is the most common case, and is always fine.
            //   - if the parent didn't have a certificate, but we see one now, it just means that we have begun an upgrade: again, this is always fine.
            (None | Some(_), None) => {}
            // If we no longer see a cert, we have to make sure that we either:
            //    - no longer care because we have passed new_version_first_view, or
            //    - no longer care because we have passed `decide_by` without deciding the certificate.
            (None, Some(parent_cert)) => {
                let decided_upgrade_certificate_read = decided_upgrade_certificate.read().await;
                ensure!(self.view_number() > parent_cert.data.new_version_first_view
                    || (self.view_number() > parent_cert.data.decide_by && decided_upgrade_certificate_read.is_none()),
                       "The new leaf is missing an upgrade certificate that was present in its parent, and should still be live."
                );
            }
            // If we both have a certificate, they should be identical.
            // Technically, this prevents us from initiating a new upgrade in the view immediately following an upgrade.
            // I think this is a fairly lax restriction.
            (Some(cert), Some(parent_cert)) => {
                ensure!(cert == parent_cert, "The new leaf does not extend the parent leaf, because it has attached a different upgrade certificate.");
            }
        }

        // This check should be added once we sort out the genesis leaf/justify_qc issue.
        // ensure!(self.parent_commitment() == parent_leaf.commit(), "The commitment of the parent leaf does not match the specified parent commitment.");

        Ok(())
    }
}

impl<TYPES: NodeType> TestableLeaf for Leaf<TYPES>
where
    TYPES::ValidatedState: TestableState<TYPES>,
    TYPES::BlockPayload: TestableBlock<TYPES>,
{
    type NodeType = TYPES;

    fn create_random_transaction(
        &self,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <<Self::NodeType as NodeType>::BlockPayload as BlockPayload<Self::NodeType>>::Transaction
    {
        TYPES::ValidatedState::create_random_transaction(None, rng, padding)
    }
}
/// Fake the thing a genesis block points to. Needed to avoid infinite recursion
#[must_use]
pub fn fake_commitment<S: Committable>() -> Commitment<S> {
    RawCommitmentBuilder::new("Dummy commitment for arbitrary genesis").finalize()
}

/// create a random commitment
#[must_use]
pub fn random_commitment<S: Committable>(rng: &mut dyn rand::RngCore) -> Commitment<S> {
    let random_array: Vec<u8> = (0u8..100u8).map(|_| rng.gen_range(0..255)).collect();
    RawCommitmentBuilder::new("Random Commitment")
        .constant_str("Random Field")
        .var_size_bytes(&random_array)
        .finalize()
}

/// Serialization for the QC assembled signature
/// # Panics
/// if serialization fails
pub fn serialize_signature2<TYPES: NodeType>(
    signatures: &<TYPES::SignatureKey as SignatureKey>::QcType,
) -> Vec<u8> {
    let mut signatures_bytes = vec![];
    signatures_bytes.extend("Yes".as_bytes());

    let (sig, proof) = TYPES::SignatureKey::sig_proof(signatures);
    let proof_bytes = bincode_opts()
        .serialize(&proof.as_bitslice())
        .expect("This serialization shouldn't be able to fail");
    signatures_bytes.extend("bitvec proof".as_bytes());
    signatures_bytes.extend(proof_bytes.as_slice());
    let sig_bytes = bincode_opts()
        .serialize(&sig)
        .expect("This serialization shouldn't be able to fail");
    signatures_bytes.extend("aggregated signature".as_bytes());
    signatures_bytes.extend(sig_bytes.as_slice());
    signatures_bytes
}

impl<TYPES: NodeType> Committable for Leaf<TYPES> {
    fn commit(&self) -> committable::Commitment<Self> {
        RawCommitmentBuilder::new("leaf commitment")
            .u64_field("view number", *self.view_number)
            .field("parent leaf commitment", self.parent_commitment)
            .field("block header", self.block_header.commit())
            .field("justify qc", self.justify_qc.commit())
            .optional("upgrade certificate", &self.upgrade_certificate)
            .finalize()
    }
}

impl<TYPES: NodeType> Leaf<TYPES> {
    /// Constructs a leaf from a given quorum proposal.
    pub fn from_quorum_proposal(quorum_proposal: &QuorumProposal<TYPES>) -> Self {
        // WARNING: Do NOT change this to a wildcard match, or reference the fields directly in the construction of the leaf.
        // The point of this match is that we will get a compile-time error if we add a field without updating this.
        let QuorumProposal {
            view_number,
            justify_qc,
            block_header,
            upgrade_certificate,
            proposal_certificate: _,
        } = quorum_proposal;
        Leaf {
            view_number: *view_number,
            justify_qc: justify_qc.clone(),
            parent_commitment: justify_qc.date().leaf_commit,
            block_header: block_header.clone(),
            upgrade_certificate: upgrade_certificate.clone(),
            block_payload: None,
        }
    }
}

pub mod null_block {
    #![allow(missing_docs)]

    use jf_vid::VidScheme;
    use memoize::memoize;
    use vbs::version::StaticVersionType;

    use crate::{
        traits::{
            block_contents::BuilderFee,
            node_implementation::{NodeType, Versions},
            signature_key::BuilderSignatureKey,
            BlockPayload,
        },
        vid::{vid_scheme, VidCommitment},
    };

    /// The commitment for a null block payload.
    ///
    /// Note: the commitment depends on the network (via `num_storage_nodes`),
    /// and may change (albeit rarely) during execution.
    ///
    /// We memoize the result to avoid having to recalculate it.
    #[memoize(SharedCache, Capacity: 10)]
    #[must_use]
    pub fn commitment(num_storage_nodes: usize) -> Option<VidCommitment> {
        let vid_result = vid_scheme(num_storage_nodes).commit_only(&Vec::new());

        match vid_result {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    /// Builder fee data for a null block payload
    #[must_use]
    pub fn builder_fee<TYPES: NodeType, V: Versions>(
        num_storage_nodes: usize,
        version: vbs::version::Version,
    ) -> Option<BuilderFee<TYPES>> {
        /// Arbitrary fee amount, this block doesn't actually come from a builder
        const FEE_AMOUNT: u64 = 0;

        let (pub_key, priv_key) =
            <TYPES::BuilderSignatureKey as BuilderSignatureKey>::generated_from_seed_indexed(
                [0_u8; 32], 0,
            );

        if version >= V::Marketplace::VERSION {
            match TYPES::BuilderSignatureKey::sign_sequencing_fee_marketplace(&priv_key, FEE_AMOUNT)
            {
                Ok(sig) => Some(BuilderFee {
                    fee_amount: FEE_AMOUNT,
                    fee_account: pub_key,
                    fee_signature: sig,
                }),
                Err(_) => None,
            }
        } else {
            let (_null_block, null_block_metadata) =
                <TYPES::BlockPayload as BlockPayload<TYPES>>::empty();

            match TYPES::BuilderSignatureKey::sign_fee(
                &priv_key,
                FEE_AMOUNT,
                &null_block_metadata,
                &commitment(num_storage_nodes)?,
            ) {
                Ok(sig) => Some(BuilderFee {
                    fee_amount: FEE_AMOUNT,
                    fee_account: pub_key,
                    fee_signature: sig,
                }),
                Err(_) => None,
            }
        }
    }
}

/// A packed bundle constructed from a sequence of bundles.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PackedBundle<TYPES: NodeType> {
    /// The combined transactions as bytes.
    pub encoded_transactions: Arc<[u8]>,

    /// The metadata of the block.
    pub metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,

    /// The view number that this block is associated with.
    pub view_number: TYPES::Time,

    /// The sequencing fee for submitting bundles.
    pub sequencing_fees: Vec1<BuilderFee<TYPES>>,

    /// The Vid precompute for the block.
    pub vid_precompute: Option<VidPrecomputeData>,

    /// The auction results for the block, if it was produced as the result of an auction
    pub auction_result: Option<TYPES::AuctionResult>,
}

impl<TYPES: NodeType> PackedBundle<TYPES> {
    /// Create a new [`PackedBundle`].
    pub fn new(
        encoded_transactions: Arc<[u8]>,
        metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
        view_number: TYPES::Time,
        sequencing_fees: Vec1<BuilderFee<TYPES>>,
        vid_precompute: Option<VidPrecomputeData>,
        auction_result: Option<TYPES::AuctionResult>,
    ) -> Self {
        Self {
            encoded_transactions,
            metadata,
            view_number,
            sequencing_fees,
            vid_precompute,
            auction_result,
        }
    }
}
