//! Provides the core consensus types

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use anyhow::{bail, ensure, Result};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
use committable::{Commitment, Committable};
use tracing::{debug, error};

pub use crate::utils::{View, ViewInner};
use crate::{
    data::{Leaf, QuorumProposal, VidDisperse, VidDisperseShare},
    error::HotShotError,
    message::Proposal,
    simple_certificate::{DaCertificate, QuorumCertificate, UpgradeCertificate},
    traits::{
        block_contents::BuilderFee,
        metrics::{Counter, Gauge, Histogram, Metrics, NoMetrics},
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
        BlockPayload, ValidatedState,
    },
    utils::{BuilderCommitment, StateAndDelta, Terminator},
    vid::VidCommitment,
    vote::HasViewNumber,
};

/// A type alias for `HashMap<Commitment<T>, T>`
pub type CommitmentMap<T> = HashMap<Commitment<T>, T>;

/// A type alias for `BTreeMap<T::Time, HashMap<T::SignatureKey, Proposal<T, VidDisperseShare<T>>>>`
pub type VidShares<TYPES> = BTreeMap<
    <TYPES as NodeType>::Time,
    HashMap<<TYPES as NodeType>::SignatureKey, Proposal<TYPES, VidDisperseShare<TYPES>>>,
>;

/// Type alias for consensus state wrapped in a lock.
pub type LockedConsensusState<TYPES> = Arc<RwLock<Consensus<TYPES>>>;

/// A reference to the consensus algorithm
///
/// This will contain the state of all rounds.
#[derive(custom_debug::Debug)]
pub struct Consensus<TYPES: NodeType> {
    /// The validated states that are currently loaded in memory.
    validated_state_map: BTreeMap<TYPES::Time, View<TYPES>>,

    /// All the VID shares we've received for current and future views.
    vid_shares: VidShares<TYPES>,

    /// All the DA certs we've received for current and future views.
    /// view -> DA cert
    saved_da_certs: HashMap<TYPES::Time, DaCertificate<TYPES>>,

    /// View number that is currently on.
    cur_view: TYPES::Time,

    /// Last proposals we sent out, None if we haven't proposed yet.
    /// Prevents duplicate proposals, and can be served to those trying to catchup
    last_proposals: BTreeMap<TYPES::Time, Proposal<TYPES, QuorumProposal<TYPES>>>,

    /// last view had a successful decide event
    last_decided_view: TYPES::Time,

    /// The `locked_qc` view number
    locked_view: TYPES::Time,

    /// Map of leaf hash -> leaf
    /// - contains undecided leaves
    /// - includes the MOST RECENT decided leaf
    saved_leaves: CommitmentMap<Leaf<TYPES>>,

    /// Saved payloads.
    ///
    /// Encoded transactions for every view if we got a payload for that view.
    saved_payloads: BTreeMap<TYPES::Time, Arc<[u8]>>,

    /// the highqc per spec
    high_qc: QuorumCertificate<TYPES>,

    /// A reference to the metrics trait
    pub metrics: Arc<ConsensusMetricsValue>,

    /// The most recent upgrade certificate this node formed.
    /// Note: this is ONLY for certificates that have been formed internally,
    /// so that we can propose with them.
    ///
    /// Certificates received from other nodes will get reattached regardless of this fields,
    /// since they will be present in the leaf we propose off of.
    dontuse_formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// most recent decided upgrade certificate
    dontuse_decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,
}

/// Contains several `ConsensusMetrics` that we're interested in from the consensus interfaces
#[derive(Clone, Debug)]
pub struct ConsensusMetricsValue {
    /// The number of last synced block height
    pub last_synced_block_height: Box<dyn Gauge>,
    /// The number of last decided view
    pub last_decided_view: Box<dyn Gauge>,
    /// Number of timestamp for the last decided time
    pub last_decided_time: Box<dyn Gauge>,
    /// The current view
    pub current_view: Box<dyn Gauge>,
    /// Number of views that are in-flight since the last decided view
    pub number_of_views_since_last_decide: Box<dyn Gauge>,
    /// Number of views that are in-flight since the last anchor view
    pub number_of_views_per_decide_event: Box<dyn Histogram>,
    /// Duration of views as leader
    pub view_duration_as_leader: Box<dyn Histogram>,
    /// Number of invalid QCs we've seen since the last commit.
    pub invalid_qc: Box<dyn Gauge>,
    /// Number of outstanding transactions
    pub outstanding_transactions: Box<dyn Gauge>,
    /// Memory size in bytes of the serialized transactions still outstanding
    pub outstanding_transactions_memory_size: Box<dyn Gauge>,
    /// Number of views that timed out
    pub number_of_timeouts: Box<dyn Counter>,
    /// Number of views that timed out as leader
    pub number_of_timeouts_as_leader: Box<dyn Counter>,
    /// The number of empty blocks that have been proposed
    pub number_of_empty_blocks_proposed: Box<dyn Counter>,
}

impl ConsensusMetricsValue {
    /// Create a new instance of this [`ConsensusMetricsValue`] struct, setting all the counters and gauges
    #[must_use]
    pub fn new(metrics: &dyn Metrics) -> Self {
        Self {
            last_synced_block_height: metrics
                .create_gauge(String::from("last_synced_block_height"), None),
            last_decided_view: metrics.create_gauge(String::from("last_decided_view"), None),
            last_decided_time: metrics.create_gauge(String::from("last_decided_time"), None),
            current_view: metrics.create_gauge(String::from("current_view"), None),
            number_of_views_since_last_decide: metrics
                .create_gauge(String::from("number_of_views_since_last_decide"), None),
            number_of_views_per_decide_event: metrics
                .create_histogram(String::from("number_of_views_per_decide_event"), None),
            view_duration_as_leader: metrics
                .create_histogram(String::from("view_duration_as_leader"), None),
            invalid_qc: metrics.create_gauge(String::from("invalid_qc"), None),
            outstanding_transactions: metrics
                .create_gauge(String::from("outstanding_transactions"), None),
            outstanding_transactions_memory_size: metrics
                .create_gauge(String::from("outstanding_transactions_memory_size"), None),
            number_of_timeouts: metrics.create_counter(String::from("number_of_timeouts"), None),
            number_of_timeouts_as_leader: metrics
                .create_counter(String::from("number_of_timeouts_as_leader"), None),
            number_of_empty_blocks_proposed: metrics
                .create_counter(String::from("number_of_empty_blocks_proposed"), None),
        }
    }
}

impl Default for ConsensusMetricsValue {
    fn default() -> Self {
        Self::new(&*NoMetrics::boxed())
    }
}

impl<TYPES: NodeType> Consensus<TYPES> {
    /// Constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        validated_state_map: BTreeMap<TYPES::Time, View<TYPES>>,
        cur_view: TYPES::Time,
        locked_view: TYPES::Time,
        last_decided_view: TYPES::Time,
        last_proposals: BTreeMap<TYPES::Time, Proposal<TYPES, QuorumProposal<TYPES>>>,
        saved_leaves: CommitmentMap<Leaf<TYPES>>,
        saved_payloads: BTreeMap<TYPES::Time, Arc<[u8]>>,
        high_qc: QuorumCertificate<TYPES>,
        metrics: Arc<ConsensusMetricsValue>,
    ) -> Self {
        Consensus {
            validated_state_map,
            vid_shares: BTreeMap::new(),
            saved_da_certs: HashMap::new(),
            cur_view,
            last_decided_view,
            last_proposals,
            locked_view,
            saved_leaves,
            saved_payloads,
            high_qc,
            metrics,
            dontuse_decided_upgrade_cert: None,
            dontuse_formed_upgrade_certificate: None,
        }
    }

    /// Get the current view.
    pub fn cur_view(&self) -> TYPES::Time {
        self.cur_view
    }

    /// Get the last decided view.
    pub fn last_decided_view(&self) -> TYPES::Time {
        self.last_decided_view
    }

    /// Get the locked view.
    pub fn locked_view(&self) -> TYPES::Time {
        self.locked_view
    }

    /// Get the high QC.
    pub fn high_qc(&self) -> &QuorumCertificate<TYPES> {
        &self.high_qc
    }

    /// Get the validated state map.
    pub fn validated_state_map(&self) -> &BTreeMap<TYPES::Time, View<TYPES>> {
        &self.validated_state_map
    }

    /// Get the saved leaves.
    pub fn saved_leaves(&self) -> &CommitmentMap<Leaf<TYPES>> {
        &self.saved_leaves
    }

    /// Get the saved payloads.
    pub fn saved_payloads(&self) -> &BTreeMap<TYPES::Time, Arc<[u8]>> {
        &self.saved_payloads
    }

    /// Get the vid shares.
    pub fn vid_shares(&self) -> &VidShares<TYPES> {
        &self.vid_shares
    }

    /// Get the saved DA certs.
    pub fn saved_da_certs(&self) -> &HashMap<TYPES::Time, DaCertificate<TYPES>> {
        &self.saved_da_certs
    }

    /// Get the map of our recent proposals
    pub fn last_proposals(&self) -> &BTreeMap<TYPES::Time, Proposal<TYPES, QuorumProposal<TYPES>>> {
        &self.last_proposals
    }

    /// Update the current view.
    /// # Errors
    /// Can return an error when the new view_number is not higher than the existing view number.
    pub fn update_view(&mut self, view_number: TYPES::Time) -> Result<()> {
        ensure!(
            view_number > self.cur_view,
            "New view isn't newer than the current view."
        );
        self.cur_view = view_number;
        Ok(())
    }

    /// Update the last proposal.
    ///
    /// # Errors
    /// Can return an error when the new view_number is not higher than the existing proposed view number.
    pub fn update_last_proposed_view(
        &mut self,
        proposal: Proposal<TYPES, QuorumProposal<TYPES>>,
    ) -> Result<()> {
        ensure!(
            proposal.data.view_number()
                > self
                    .last_proposals
                    .last_key_value()
                    .map_or(TYPES::Time::genesis(), |(k, _)| { *k }),
            "New view isn't newer than the previously proposed view."
        );
        self.last_proposals
            .insert(proposal.data.view_number(), proposal);
        Ok(())
    }

    /// Update the last decided view.
    ///
    /// # Errors
    /// Can return an error when the new view_number is not higher than the existing decided view number.
    pub fn update_last_decided_view(&mut self, view_number: TYPES::Time) -> Result<()> {
        ensure!(
            view_number > self.last_decided_view,
            "New view isn't newer than the previously decided view."
        );
        self.last_decided_view = view_number;
        Ok(())
    }

    /// Update the locked view.
    ///
    /// # Errors
    /// Can return an error when the new view_number is not higher than the existing locked view number.
    pub fn update_locked_view(&mut self, view_number: TYPES::Time) -> Result<()> {
        ensure!(
            view_number > self.locked_view,
            "New view isn't newer than the previously locked view."
        );
        self.locked_view = view_number;
        Ok(())
    }

    /// Update the validated state map with a new view_number/view combo.
    ///
    /// # Errors
    /// Can return an error when the new view contains less information than the exisiting view
    /// with the same view number.
    pub fn update_validated_state_map(
        &mut self,
        view_number: TYPES::Time,
        view: View<TYPES>,
    ) -> Result<()> {
        if let Some(existing_view) = self.validated_state_map().get(&view_number) {
            if let ViewInner::Leaf { .. } = existing_view.view_inner {
                match view.view_inner {
                    ViewInner::Leaf { ref delta, .. } => {
                        ensure!(
                            delta.is_some(),
                            "Skipping the state update to not override a `Leaf` view with `None` state delta."
                        );
                    }
                    _ => {
                        bail!("Skipping the state update to not override a `Leaf` view with a non-`Leaf` view.");
                    }
                }
            }
        }
        self.validated_state_map.insert(view_number, view);
        Ok(())
    }

    /// Update the saved leaves with a new leaf.
    pub fn update_saved_leaves(&mut self, leaf: Leaf<TYPES>) {
        self.saved_leaves.insert(leaf.commit(), leaf);
    }

    /// Update the saved payloads with a new encoded transaction.
    ///
    /// # Errors
    /// Can return an error when there's an existing payload corresponding to the same view number.
    pub fn update_saved_payloads(
        &mut self,
        view_number: TYPES::Time,
        encoded_transaction: Arc<[u8]>,
    ) -> Result<()> {
        ensure!(
            !self.saved_payloads.contains_key(&view_number),
            "Payload with the same view already exists."
        );
        self.saved_payloads.insert(view_number, encoded_transaction);
        Ok(())
    }

    /// Update the high QC if given a newer one.
    /// # Errors
    /// Can return an error when the provided high_qc is not newer than the existing entry.
    pub fn update_high_qc(&mut self, high_qc: QuorumCertificate<TYPES>) -> Result<()> {
        ensure!(
            high_qc.view_number > self.high_qc.view_number,
            "High QC with an equal or higher view exists."
        );
        debug!("Updating high QC");
        self.high_qc = high_qc;

        Ok(())
    }

    /// Add a new entry to the vid_shares map.
    pub fn update_vid_shares(
        &mut self,
        view_number: TYPES::Time,
        disperse: Proposal<TYPES, VidDisperseShare<TYPES>>,
    ) {
        self.vid_shares
            .entry(view_number)
            .or_default()
            .insert(disperse.data.recipient_key.clone(), disperse);
    }

    /// Add a new entry to the da_certs map.
    pub fn update_saved_da_certs(&mut self, view_number: TYPES::Time, cert: DaCertificate<TYPES>) {
        self.saved_da_certs.insert(view_number, cert);
    }

    /// Update the most recent decided upgrade certificate.
    pub fn update_dontuse_decided_upgrade_cert(&mut self, cert: Option<UpgradeCertificate<TYPES>>) {
        self.dontuse_decided_upgrade_cert = cert;
    }

    /// gather information from the parent chain of leaves
    /// # Errors
    /// If the leaf or its ancestors are not found in storage
    pub fn visit_leaf_ancestors<F>(
        &self,
        start_from: TYPES::Time,
        terminator: Terminator<TYPES::Time>,
        ok_when_finished: bool,
        mut f: F,
    ) -> Result<(), HotShotError<TYPES>>
    where
        F: FnMut(
            &Leaf<TYPES>,
            Arc<<TYPES as NodeType>::ValidatedState>,
            Option<Arc<<<TYPES as NodeType>::ValidatedState as ValidatedState<TYPES>>::Delta>>,
        ) -> bool,
    {
        let mut next_leaf = if let Some(view) = self.validated_state_map.get(&start_from) {
            view.leaf_commitment()
                .ok_or_else(|| HotShotError::InvalidState {
                    context: format!(
                        "Visited failed view {start_from:?} leaf. Expected successfuil leaf"
                    ),
                })?
        } else {
            return Err(HotShotError::InvalidState {
                context: format!("View {start_from:?} leaf does not exist in state map "),
            });
        };

        while let Some(leaf) = self.saved_leaves.get(&next_leaf) {
            let view = leaf.view_number();
            if let (Some(state), delta) = self.state_and_delta(view) {
                if let Terminator::Exclusive(stop_before) = terminator {
                    if stop_before == view {
                        if ok_when_finished {
                            return Ok(());
                        }
                        break;
                    }
                }
                next_leaf = leaf.parent_commitment();
                if !f(leaf, state, delta) {
                    return Ok(());
                }
                if let Terminator::Inclusive(stop_after) = terminator {
                    if stop_after == view {
                        if ok_when_finished {
                            return Ok(());
                        }
                        break;
                    }
                }
            } else {
                return Err(HotShotError::InvalidState {
                    context: format!("View {view:?} state does not exist in state map "),
                });
            }
        }
        Err(HotShotError::LeafNotFound {})
    }

    /// Garbage collects based on state change right now, this removes from both the
    /// `saved_payloads` and `validated_state_map` fields of `Consensus`.
    /// # Panics
    /// On inconsistent stored entries
    pub fn collect_garbage(&mut self, old_anchor_view: TYPES::Time, new_anchor_view: TYPES::Time) {
        // state check
        let anchor_entry = self
            .validated_state_map
            .iter()
            .next()
            .expect("INCONSISTENT STATE: anchor leaf not in state map!");
        if *anchor_entry.0 != old_anchor_view {
            error!(
                "Something about GC has failed. Older leaf exists than the previous anchor leaf."
            );
        }
        // perform gc
        self.saved_da_certs
            .retain(|view_number, _| *view_number >= old_anchor_view);
        self.validated_state_map
            .range(old_anchor_view..new_anchor_view)
            .filter_map(|(_view_number, view)| view.leaf_commitment())
            .for_each(|leaf| {
                self.saved_leaves.remove(&leaf);
            });
        self.validated_state_map = self.validated_state_map.split_off(&new_anchor_view);
        self.saved_payloads = self.saved_payloads.split_off(&new_anchor_view);
        self.vid_shares = self.vid_shares.split_off(&new_anchor_view);
        self.last_proposals = self.last_proposals.split_off(&new_anchor_view);
    }

    /// Gets the last decided leaf.
    ///
    /// # Panics
    /// if the last decided view's leaf does not exist in the state map or saved leaves, which
    /// should never happen.
    #[must_use]
    pub fn decided_leaf(&self) -> Leaf<TYPES> {
        let decided_view_num = self.last_decided_view;
        let view = self.validated_state_map.get(&decided_view_num).unwrap();
        let leaf = view
            .leaf_commitment()
            .expect("Decided leaf not found! Consensus internally inconsistent");
        self.saved_leaves.get(&leaf).unwrap().clone()
    }

    /// Gets the validated state with the given view number, if in the state map.
    #[must_use]
    pub fn state(&self, view_number: TYPES::Time) -> Option<&Arc<TYPES::ValidatedState>> {
        match self.validated_state_map.get(&view_number) {
            Some(view) => view.state(),
            None => None,
        }
    }

    /// Gets the validated state and state delta with the given view number, if in the state map.
    #[must_use]
    pub fn state_and_delta(&self, view_number: TYPES::Time) -> StateAndDelta<TYPES> {
        match self.validated_state_map.get(&view_number) {
            Some(view) => view.state_and_delta(),
            None => (None, None),
        }
    }

    /// Gets the last decided validated state.
    ///
    /// # Panics
    /// If the last decided view's state does not exist in the state map, which should never
    /// happen.
    #[must_use]
    pub fn decided_state(&self) -> Arc<TYPES::ValidatedState> {
        let decided_view_num = self.last_decided_view;
        self.state_and_delta(decided_view_num)
            .0
            .expect("Decided state not found! Consensus internally inconsistent")
    }

    /// Associated helper function:
    /// Takes `LockedConsensusState` which will be updated; locks it for read and write accordingly.
    /// Calculates `VidDisperse` based on the view, the txns and the membership,
    /// and updates `vid_shares` map with the signed `VidDisperseShare` proposals.
    /// Returned `Option` indicates whether the update has actually happened or not.
    pub async fn calculate_and_update_vid(
        consensus: LockedConsensusState<TYPES>,
        view: <TYPES as NodeType>::Time,
        membership: Arc<TYPES::Membership>,
        private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Option<()> {
        let consensus = consensus.upgradable_read().await;
        let txns = consensus.saved_payloads().get(&view)?;
        let vid =
            VidDisperse::calculate_vid_disperse(Arc::clone(txns), &membership, view, None).await;
        let shares = VidDisperseShare::from_vid_disperse(vid);
        let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
        for share in shares {
            if let Some(prop) = share.to_proposal(private_key) {
                consensus.update_vid_shares(view, prop);
            }
        }
        Some(())
    }
}

/// Alias for the block payload commitment and the associated metadata. The primary data
/// needed in order to submit a proposal.
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct CommitmentAndMetadata<TYPES: NodeType> {
    /// Vid Commitment
    pub commitment: VidCommitment,
    /// Builder Commitment
    pub builder_commitment: BuilderCommitment,
    /// Metadata for the block payload
    pub metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    /// Builder fee data
    pub fee: BuilderFee<TYPES>,
    /// View number this block is for
    pub block_view: TYPES::Time,
}
