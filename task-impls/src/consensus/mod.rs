use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};

use async_broadcast::Sender;
use async_compatibility_layer::art::async_spawn;
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use chrono::Utc;
use committable::Committable;
use futures::{future::join_all, FutureExt};
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::{CommitmentAndMetadata, Consensus, View},
    data::{Leaf, QuorumProposal, ViewChangeEvidence},
    event::{Event, EventType, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, TimeoutCertificate, UpgradeCertificate},
    simple_vote::{QuorumVote, TimeoutData, TimeoutVote},
    traits::{
        block_contents::BlockHeader,
        consensus_api::ConsensusApi,
        election::Membership,
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        states::ValidatedState,
        storage::Storage,
        BlockPayload,
    },
    utils::{Terminator, ViewInner},
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, warn};
use vbs::version::Version;
#[cfg(not(feature = "dependency-tasks"))]
use {
    crate::helpers::AnyhowTracing,
    hotshot_types::{
        data::{null_block, VidDisperseShare},
        message::GeneralConsensusMessage,
        simple_vote::QuorumData,
    },
};

use crate::{
    consensus::{proposal::validate_proposal, view_change::update_view},
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::{broadcast_event, cancel_task},
    vote_collection::{
        create_vote_accumulator, AccumulatorInfo, HandleVoteEvent, VoteCollectionTaskState,
    },
};

/// Handles proposal-related functionality.
pub(crate) mod proposal;

/// Handles view-change related functionality.
pub(crate) mod view_change;

/// Alias for Optional type for Vote Collectors
type VoteCollectorOption<TYPES, VOTE, CERT> = Option<VoteCollectionTaskState<TYPES, VOTE, CERT>>;

/// The state for the consensus task.  Contains all of the information for the implementation
/// of consensus
pub struct ConsensusTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
    /// Our public key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,
    /// View timeout from config.
    pub timeout: u64,
    /// Round start delay from config, in milliseconds.
    pub round_start_delay: u64,
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// The commitment to the current block payload and its metadata submitted to DA.
    pub payload_commitment_and_metadata: Option<CommitmentAndMetadata<TYPES>>,

    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for DA committee
    pub committee_network: Arc<I::CommitteeNetwork>,

    /// Membership for Timeout votes/certs
    pub timeout_membership: Arc<TYPES::Membership>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Membership for DA committee Votes/certs
    pub committee_membership: Arc<TYPES::Membership>,

    /// Consensus api
    pub api: A,

    /// Current Vote collection task, with it's view.
    pub vote_collector:
        RwLock<VoteCollectorOption<TYPES, QuorumVote<TYPES>, QuorumCertificate<TYPES>>>,

    /// Current timeout vote collection task with its view
    pub timeout_vote_collector:
        RwLock<VoteCollectorOption<TYPES, TimeoutVote<TYPES>, TimeoutCertificate<TYPES>>>,

    /// timeout task handle
    pub timeout_task: Option<JoinHandle<()>>,

    /// Spawned tasks related to a specific view, so we can cancel them when
    /// they are stale
    pub spawned_tasks: BTreeMap<TYPES::Time, Vec<JoinHandle<()>>>,

    /// The most recent upgrade certificate this node formed.
    /// Note: this is ONLY for certificates that have been formed internally,
    /// so that we can propose with them.
    ///
    /// Certificates received from other nodes will get reattached regardless of this fields,
    /// since they will be present in the leaf we propose off of.
    pub formed_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,

    /// last View Sync Certificate or Timeout Certificate this node formed.
    pub proposal_cert: Option<ViewChangeEvidence<TYPES>>,

    /// most recent decided upgrade certificate
    pub decided_upgrade_cert: Option<UpgradeCertificate<TYPES>>,

    /// Globally shared reference to the current network version.
    pub version: Arc<RwLock<Version>>,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// The most recent proposal we have, will correspond to the current view if Some()
    /// Will be none if the view advanced through timeout/view_sync
    pub current_proposal: Option<QuorumProposal<TYPES>>,

    // ED Should replace this with config information since we need it anyway
    /// The node's id
    pub id: u64,

    /// This node's storage ref
    pub storage: Arc<RwLock<I::Storage>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    ConsensusTaskState<TYPES, I, A>
{
    /// Cancel all tasks the consensus tasks has spawned before the given view
    async fn cancel_tasks(&mut self, view: TYPES::Time) {
        let keep = self.spawned_tasks.split_off(&view);
        let mut cancel = Vec::new();
        while let Some((_, tasks)) = self.spawned_tasks.pop_first() {
            let mut to_cancel = tasks.into_iter().map(cancel_task).collect();
            cancel.append(&mut to_cancel);
        }
        self.spawned_tasks = keep;
        join_all(cancel).await;
    }

    /// Ignores old vote behavior and lets `QuorumVoteTask` take over.
    #[cfg(feature = "dependency-tasks")]
    async fn vote_if_able(&mut self, _event_stream: &Sender<Arc<HotShotEvent<TYPES>>>) -> bool {
        false
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus vote if able", level = "error")]
    #[cfg(not(feature = "dependency-tasks"))]
    // Check if we are able to vote, like whether the proposal is valid,
    // whether we have DAC and VID share, and if so, vote.
    async fn vote_if_able(&mut self, event_stream: &Sender<Arc<HotShotEvent<TYPES>>>) -> bool {
        if !self.quorum_membership.has_stake(&self.public_key) {
            debug!(
                "We were not chosen for consensus committee on {:?}",
                self.cur_view
            );
            return false;
        }

        if let Some(proposal) = &self.current_proposal {
            let consensus = self.consensus.read().await;
            // Only vote if you has seen the VID share for this view
            let Some(vid_shares) = consensus.vid_shares.get(&proposal.view_number) else {
                debug!(
                    "We have not seen the VID share for this view {:?} yet, so we cannot vote.",
                    proposal.view_number
                );
                return false;
            };

            if let Some(upgrade_cert) = &self.decided_upgrade_cert {
                if upgrade_cert.in_interim(self.cur_view)
                    && Some(proposal.block_header.payload_commitment())
                        != null_block::commitment(self.quorum_membership.total_nodes())
                {
                    info!("Refusing to vote on proposal because it does not have a null commitment, and we are between versions. Expected:\n\n{:?}\n\nActual:{:?}", null_block::commitment(self.quorum_membership.total_nodes()), Some(proposal.block_header.payload_commitment()));
                    return false;
                }
            }

            // Only vote if you have the DA cert
            // ED Need to update the view number this is stored under?
            if let Some(cert) = consensus.saved_da_certs.get(&(proposal.get_view_number())) {
                let view = cert.view_number;
                // TODO: do some of this logic without the vote token check, only do that when voting.
                let justify_qc = proposal.justify_qc.clone();
                let parent = if justify_qc.is_genesis {
                    Some(Leaf::genesis(&consensus.instance_state))
                } else {
                    consensus
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                };

                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some(parent) = parent else {
                    warn!(
                                "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
                                justify_qc.get_data().leaf_commit,
                                proposal.view_number,
                            );
                    return false;
                };
                let parent_commitment = parent.commit();

                let mut proposed_leaf = Leaf::from_quorum_proposal(proposal);
                proposed_leaf.set_parent_commitment(parent_commitment);

                // Validate the DAC.
                let message = if cert.is_valid_cert(self.committee_membership.as_ref()) {
                    // Validate the block payload commitment for non-genesis DAC.
                    if !cert.is_genesis
                        && cert.get_data().payload_commit
                            != proposal.block_header.payload_commitment()
                    {
                        error!("Block payload commitment does not equal da cert payload commitment. View = {}", *view);
                        return false;
                    }
                    if let Ok(vote) = QuorumVote::<TYPES>::create_signed_vote(
                        QuorumData {
                            leaf_commit: proposed_leaf.commit(),
                        },
                        view,
                        &self.public_key,
                        &self.private_key,
                    ) {
                        GeneralConsensusMessage::<TYPES>::Vote(vote)
                    } else {
                        error!("Unable to sign quorum vote!");
                        return false;
                    }
                } else {
                    error!(
                        "Invalid DAC in proposal! Skipping proposal. {:?} cur view is: {:?}",
                        cert, self.cur_view
                    );
                    return false;
                };

                if let GeneralConsensusMessage::Vote(vote) = message {
                    debug!(
                        "Sending vote to next quorum leader {:?}",
                        vote.get_view_number() + 1
                    );
                    // Add to the storage that we have received the VID disperse for a specific view
                    if let Some(vid_share) = vid_shares.get(&self.public_key) {
                        if let Err(e) = self.storage.write().await.append_vid(vid_share).await {
                            error!(
                                "Failed to store VID Disperse Proposal with error {:?}, aborting vote",
                                e
                            );
                            return false;
                        }
                    } else {
                        error!("Did not get a VID share for our public key, aborting vote");
                        return false;
                    }
                    broadcast_event(Arc::new(HotShotEvent::QuorumVoteSend(vote)), event_stream)
                        .await;
                    return true;
                }
            }
            debug!(
                "Received VID share, but couldn't find DAC cert for view {:?}",
                *proposal.get_view_number(),
            );
            return false;
        }
        debug!(
            "Could not vote because we don't have a proposal yet for view {}",
            *self.cur_view
        );
        false
    }

    /// Validates whether the VID Dispersal Proposal is correctly signed
    #[cfg(not(feature = "dependency-tasks"))]
    fn validate_disperse(&self, disperse: &Proposal<TYPES, VidDisperseShare<TYPES>>) -> bool {
        let view = disperse.data.get_view_number();
        let payload_commitment = disperse.data.payload_commitment;
        // Check whether the data comes from the right leader for this view
        if self
            .quorum_membership
            .get_leader(view)
            .validate(&disperse.signature, payload_commitment.as_ref())
        {
            return true;
        }
        // or the data was calculated and signed by the current node
        if self
            .public_key
            .validate(&disperse.signature, payload_commitment.as_ref())
        {
            return true;
        }
        // or the data was signed by one of the staked DA committee members
        for da_member in self.committee_membership.get_staked_committee(view) {
            if da_member.validate(&disperse.signature, payload_commitment.as_ref()) {
                return true;
            }
        }
        false
    }

    /// Handles a consensus event received on the event stream
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus replica task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            #[cfg(not(feature = "dependency-tasks"))]
            HotShotEvent::QuorumProposalRecv(proposal, sender) => {
                let sender = sender.clone();
                debug!(
                    "Received Quorum Proposal for view {}",
                    *proposal.data.view_number
                );

                // stop polling for the received proposal
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(
                        *proposal.data.view_number,
                    ))
                    .await;

                let view = proposal.data.get_view_number();
                if view < self.cur_view {
                    debug!("Proposal is from an older view {:?}", proposal.data.clone());
                    return;
                }

                let view_leader_key = self.quorum_membership.get_leader(view);
                if view_leader_key != sender {
                    warn!("Leader key does not match key in proposal");
                    return;
                }

                // Verify a timeout certificate OR a view sync certificate exists and is valid.
                if proposal.data.justify_qc.get_view_number() != view - 1 {
                    if let Some(received_proposal_cert) = proposal.data.proposal_certificate.clone()
                    {
                        match received_proposal_cert {
                            ViewChangeEvidence::Timeout(timeout_cert) => {
                                if timeout_cert.get_data().view != view - 1 {
                                    warn!("Timeout certificate for view {} was not for the immediately preceding view", *view);
                                    return;
                                }

                                if !timeout_cert.is_valid_cert(self.timeout_membership.as_ref()) {
                                    warn!("Timeout certificate for view {} was invalid", *view);
                                    return;
                                }
                            }
                            ViewChangeEvidence::ViewSync(view_sync_cert) => {
                                if view_sync_cert.view_number != view {
                                    debug!(
                                        "Cert view number {:?} does not match proposal view number {:?}",
                                        view_sync_cert.view_number, view
                                    );
                                    return;
                                }

                                // View sync certs must also be valid.
                                if !view_sync_cert.is_valid_cert(self.quorum_membership.as_ref()) {
                                    debug!("Invalid ViewSyncFinalize cert provided");
                                    return;
                                }
                            }
                        }
                    } else {
                        warn!(
                            "Quorum proposal for view {} needed a timeout or view sync certificate, but did not have one",
                            *view);
                        return;
                    };
                }

                let justify_qc = proposal.data.justify_qc.clone();

                if !justify_qc.is_valid_cert(self.quorum_membership.as_ref()) {
                    error!("Invalid justify_qc in proposal for view {}", *view);
                    let consensus = self.consensus.write().await;
                    consensus.metrics.invalid_qc.update(1);
                    return;
                }

                // Validate the upgrade certificate -- this is just a signature validation.
                // Note that we don't do anything with the certificate directly if this passes; it eventually gets stored as part of the leaf if nothing goes wrong.
                if let Err(e) = UpgradeCertificate::validate(
                    &proposal.data.upgrade_certificate,
                    &self.quorum_membership,
                ) {
                    warn!("{:?}", e);

                    return;
                }

                // NOTE: We could update our view with a valid TC but invalid QC, but that is not what we do here
                if let Err(e) = update_view::<TYPES, I>(
                    self.public_key.clone(),
                    view,
                    &event_stream,
                    self.quorum_membership.clone(),
                    self.quorum_network.clone(),
                    self.timeout,
                    self.consensus.clone(),
                    &mut self.cur_view,
                    &mut self.timeout_task,
                )
                .await
                {
                    warn!("Failed to update view; error = {e:?}");
                }

                let consensus = self.consensus.upgradable_read().await;

                // Get the parent leaf and state.
                let parent = if justify_qc.is_genesis {
                    // Send the `Decide` event for the genesis block if the justify QC is genesis.
                    let leaf = Leaf::genesis(&consensus.instance_state);
                    let (validated_state, state_delta) =
                        TYPES::ValidatedState::genesis(&consensus.instance_state);
                    let state = Arc::new(validated_state);
                    broadcast_event(
                        Event {
                            view_number: TYPES::Time::genesis(),
                            event: EventType::Decide {
                                leaf_chain: Arc::new(vec![LeafInfo::new(
                                    leaf.clone(),
                                    state.clone(),
                                    Some(Arc::new(state_delta)),
                                    None,
                                )]),
                                qc: Arc::new(justify_qc.clone()),
                                block_size: None,
                            },
                        },
                        &self.output_event_stream,
                    )
                    .await;
                    Some((leaf, state))
                } else {
                    match consensus
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                    {
                        Some(leaf) => {
                            if let (Some(state), _) =
                                consensus.get_state_and_delta(leaf.get_view_number())
                            {
                                Some((leaf, state.clone()))
                            } else {
                                error!("Parent state not found! Consensus internally inconsistent");
                                return;
                            }
                        }
                        None => None,
                    }
                };

                if justify_qc.get_view_number() > consensus.high_qc.view_number {
                    if let Err(e) = self
                        .storage
                        .write()
                        .await
                        .update_high_qc(justify_qc.clone())
                        .await
                    {
                        warn!("Failed to store High QC not voting. Error: {:?}", e);
                        return;
                    }
                }

                let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;

                if justify_qc.get_view_number() > consensus.high_qc.view_number {
                    debug!("Updating high QC");
                    consensus.high_qc = justify_qc.clone();
                }

                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some((parent_leaf, parent_state)) = parent else {
                    warn!(
                        "Proposal's parent missing from storage with commitment: {:?}",
                        justify_qc.get_data().leaf_commit
                    );
                    let leaf = Leaf::from_quorum_proposal(&proposal.data);

                    let state = Arc::new(
                        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(
                            &proposal.data.block_header,
                        ),
                    );

                    consensus.validated_state_map.insert(
                        view,
                        View {
                            view_inner: ViewInner::Leaf {
                                leaf: leaf.commit(),
                                state,
                                delta: None,
                            },
                        },
                    );
                    consensus.saved_leaves.insert(leaf.commit(), leaf.clone());

                    if let Err(e) = self
                        .storage
                        .write()
                        .await
                        .update_undecided_state(
                            consensus.saved_leaves.clone(),
                            consensus.validated_state_map.clone(),
                        )
                        .await
                    {
                        warn!("Couldn't store undecided state.  Error: {:?}", e);
                    }

                    // If we are missing the parent from storage, the safety check will fail.  But we can
                    // still vote if the liveness check succeeds.
                    let liveness_check = justify_qc.get_view_number() > consensus.locked_view;

                    let high_qc = consensus.high_qc.clone();
                    let locked_view = consensus.locked_view;

                    drop(consensus);

                    if liveness_check {
                        self.current_proposal = Some(proposal.data.clone());
                        let new_view = proposal.data.view_number + 1;

                        // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
                        let should_propose = self.quorum_membership.get_leader(new_view)
                            == self.public_key
                            && high_qc.view_number
                                == self.current_proposal.clone().unwrap().view_number;
                        let qc = high_qc.clone();
                        if should_propose {
                            debug!(
                                "Attempting to publish proposal after voting; now in view: {}",
                                *new_view
                            );
                            self.publish_proposal_if_able(qc.view_number + 1, &event_stream)
                                .await;
                        }
                        if self.vote_if_able(&event_stream).await {
                            self.current_proposal = None;
                        }
                    }
                    warn!("Failed liveneess check; cannot find parent either\n High QC is {:?}  Proposal QC is {:?}  Locked view is {:?}", high_qc, proposal.data.clone(), locked_view);

                    return;
                };

                self.spawned_tasks
                    .entry(proposal.data.get_view_number())
                    .or_default()
                    .push(async_spawn(
                        validate_proposal(
                            proposal.clone(),
                            parent_leaf,
                            self.consensus.clone(),
                            self.decided_upgrade_cert.clone(),
                            self.quorum_membership.clone(),
                            parent_state.clone(),
                            view_leader_key,
                            event_stream.clone(),
                            sender,
                            self.output_event_stream.clone(),
                            self.storage.clone(),
                        )
                        .map(AnyhowTracing::err_as_debug),
                    ));
            }
            HotShotEvent::QuorumProposalValidated(proposal, _) => {
                let consensus = self.consensus.upgradable_read().await;
                let view = proposal.get_view_number();
                self.current_proposal = Some(proposal.clone());
                let mut new_anchor_view = consensus.last_decided_view;
                let mut new_locked_view = consensus.locked_view;
                let mut last_view_number_visited = view;
                let mut new_commit_reached: bool = false;
                let mut new_decide_reached = false;
                let mut new_decide_qc = None;
                let mut leaf_views = Vec::new();
                let mut leafs_decided = Vec::new();
                let mut included_txns = HashSet::new();
                let old_anchor_view = consensus.last_decided_view;
                let parent_view = proposal.justify_qc.get_view_number();
                let mut current_chain_length = 0usize;
                if parent_view + 1 == view {
                    current_chain_length += 1;
                    if let Err(e) = consensus.visit_leaf_ancestors(
                        parent_view,
                        Terminator::Exclusive(old_anchor_view),
                        true,
                        |leaf, state, delta| {
                            if !new_decide_reached {
                                if last_view_number_visited == leaf.get_view_number() + 1 {
                                    last_view_number_visited = leaf.get_view_number();
                                    current_chain_length += 1;
                                    if current_chain_length == 2 {
                                        new_locked_view = leaf.get_view_number();
                                        new_commit_reached = true;
                                        // The next leaf in the chain, if there is one, is decided, so this
                                        // leaf's justify_qc would become the QC for the decided chain.
                                        new_decide_qc = Some(leaf.get_justify_qc().clone());
                                    } else if current_chain_length == 3 {
                                        new_anchor_view = leaf.get_view_number();
                                        new_decide_reached = true;
                                    }
                                } else {
                                    // nothing more to do here... we don't have a new chain extension
                                    return false;
                                }
                            }
                            // starting from the first iteration with a three chain, e.g. right after the else if case nested in the if case above
                            if new_decide_reached {
                                let mut leaf = leaf.clone();
                                if leaf.get_view_number() == new_anchor_view {
                                    consensus
                                        .metrics
                                        .last_synced_block_height
                                        .set(usize::try_from(leaf.get_height()).unwrap_or(0));
                                }
                                if let Some(cert) = leaf.get_upgrade_certificate() {
                                  if cert.data.decide_by < view {
                                    warn!("Failed to decide an upgrade certificate in time. Ignoring.");
                                  } else {
                                    info!("Updating consensus state with decided upgrade certificate: {:?}", cert);
                                    self.decided_upgrade_cert = Some(cert.clone());
                                  }
                                }
                                // If the block payload is available for this leaf, include it in
                                // the leaf chain that we send to the client.
                                if let Some(encoded_txns) =
                                    consensus.saved_payloads.get(&leaf.get_view_number())
                                {
                                    let payload = BlockPayload::from_bytes(
                                        encoded_txns.clone().into_iter(),
                                        leaf.get_block_header().metadata(),
                                    );

                                    leaf.fill_block_payload_unchecked(payload);
                                }

                                // Get the VID share at the leaf's view number, corresponding to our key
                                // (if one exists)
                                let vid_share = consensus
                                        .vid_shares
                                        .get(&leaf.get_view_number())
                                        .unwrap_or(&HashMap::new())
                                        .get(&self.public_key).cloned().map(|prop| prop.data);

                                // Add our data into a new `LeafInfo`
                                leaf_views.push(LeafInfo::new(leaf.clone(), state.clone(), delta.clone(), vid_share));
                                leafs_decided.push(leaf.clone());
                                if let Some(ref payload) = leaf.get_block_payload() {
                                    for txn in payload
                                        .transaction_commitments(leaf.get_block_header().metadata())
                                    {
                                        included_txns.insert(txn);
                                    }
                                }
                            }
                            true
                        },
                    ) {
                        error!("view publish error {e}");
                    }
                }

                let included_txns_set: HashSet<_> = if new_decide_reached {
                    included_txns
                } else {
                    HashSet::new()
                };

                let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
                if new_commit_reached {
                    consensus.locked_view = new_locked_view;
                }
                #[allow(clippy::cast_precision_loss)]
                if new_decide_reached {
                    broadcast_event(
                        Arc::new(HotShotEvent::LeafDecided(leafs_decided)),
                        &event_stream,
                    )
                    .await;
                    let decide_sent = broadcast_event(
                        Event {
                            view_number: consensus.last_decided_view,
                            event: EventType::Decide {
                                leaf_chain: Arc::new(leaf_views),
                                qc: Arc::new(new_decide_qc.unwrap()),
                                block_size: Some(included_txns_set.len().try_into().unwrap()),
                            },
                        },
                        &self.output_event_stream,
                    );
                    let old_anchor_view = consensus.last_decided_view;
                    consensus.collect_garbage(old_anchor_view, new_anchor_view);
                    consensus.last_decided_view = new_anchor_view;
                    consensus
                        .metrics
                        .last_decided_time
                        .set(Utc::now().timestamp().try_into().unwrap());
                    consensus.metrics.invalid_qc.set(0);
                    consensus
                        .metrics
                        .last_decided_view
                        .set(usize::try_from(consensus.last_decided_view.get_u64()).unwrap());
                    let cur_number_of_views_per_decide_event =
                        *self.cur_view - consensus.last_decided_view.get_u64();
                    consensus
                        .metrics
                        .number_of_views_per_decide_event
                        .add_point(cur_number_of_views_per_decide_event as f64);

                    debug!("Sending Decide for view {:?}", consensus.last_decided_view);
                    debug!("Decided txns len {:?}", included_txns_set.len());
                    decide_sent.await;
                    debug!("decide send succeeded");
                }

                let new_view = self.current_proposal.clone().unwrap().view_number + 1;
                // In future we can use the mempool model where we fetch the proposal if we don't have it, instead of having to wait for it here
                // This is for the case where we form a QC but have not yet seen the previous proposal ourselves
                let should_propose = self.quorum_membership.get_leader(new_view) == self.public_key
                    && consensus.high_qc.view_number
                        == self.current_proposal.clone().unwrap().view_number;
                // todo get rid of this clone
                let qc = consensus.high_qc.clone();

                drop(consensus);
                if new_decide_reached {
                    self.cancel_tasks(new_anchor_view).await;
                }
                if should_propose {
                    debug!(
                        "Attempting to publish proposal after voting; now in view: {}",
                        *new_view
                    );
                    self.publish_proposal_if_able(qc.view_number + 1, &event_stream)
                        .await;
                }

                if !self.vote_if_able(&event_stream).await {
                    return;
                }
                self.current_proposal = None;
            }
            HotShotEvent::QuorumVoteRecv(ref vote) => {
                debug!("Received quorum vote: {:?}", vote.get_view_number());
                if self
                    .quorum_membership
                    .get_leader(vote.get_view_number() + 1)
                    != self.public_key
                {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view_number() + 1,
                        self.quorum_membership
                            .get_leader(vote.get_view_number() + 2)
                            == self.public_key
                    );
                    return;
                }
                let mut collector = self.vote_collector.write().await;

                if collector.is_none() || vote.get_view_number() > collector.as_ref().unwrap().view
                {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.quorum_membership.clone(),
                        view: vote.get_view_number(),
                        id: self.id,
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        QuorumVote<TYPES>,
                        QuorumCertificate<TYPES>,
                    >(&info, vote.clone(), event, &event_stream)
                    .await;
                } else {
                    let result = collector
                        .as_mut()
                        .unwrap()
                        .handle_event(event.clone(), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        *collector = None;
                        // The protocol has finished
                        return;
                    }
                }
            }
            HotShotEvent::TimeoutVoteRecv(ref vote) => {
                if self
                    .timeout_membership
                    .get_leader(vote.get_view_number() + 1)
                    != self.public_key
                {
                    error!(
                        "We are not the leader for view {} are we the leader for view + 1? {}",
                        *vote.get_view_number() + 1,
                        self.timeout_membership
                            .get_leader(vote.get_view_number() + 2)
                            == self.public_key
                    );
                    return;
                }
                let mut collector = self.timeout_vote_collector.write().await;

                if collector.is_none() || vote.get_view_number() > collector.as_ref().unwrap().view
                {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.quorum_membership.clone(),
                        view: vote.get_view_number(),
                        id: self.id,
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        TimeoutVote<TYPES>,
                        TimeoutCertificate<TYPES>,
                    >(&info, vote.clone(), event, &event_stream)
                    .await;
                } else {
                    let result = collector
                        .as_mut()
                        .unwrap()
                        .handle_event(event.clone(), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        *collector = None;
                        // The protocol has finished
                        return;
                    }
                }
            }
            HotShotEvent::QCFormed(cert) => {
                debug!("QC Formed event happened!");

                if let either::Right(qc) = cert.clone() {
                    self.proposal_cert = Some(ViewChangeEvidence::Timeout(qc.clone()));
                    // cancel poll for votes
                    self.quorum_network
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    debug!(
                        "Attempting to publish proposal after forming a TC for view {}",
                        *qc.view_number
                    );

                    let view = qc.view_number + 1;

                    self.publish_proposal_if_able(view, &event_stream).await;
                }
                if let either::Left(qc) = cert {
                    if let Err(e) = self.storage.write().await.update_high_qc(qc.clone()).await {
                        warn!("Failed to store High QC of QC we formed. Error: {:?}", e);
                    }

                    let mut consensus = self.consensus.write().await;
                    consensus.high_qc = qc.clone();

                    // cancel poll for votes
                    self.quorum_network
                        .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                            *qc.view_number,
                        ))
                        .await;

                    drop(consensus);
                    debug!(
                        "Attempting to publish proposal after forming a QC for view {}",
                        *qc.view_number
                    );

                    self.publish_proposal_if_able(qc.view_number + 1, &event_stream)
                        .await;
                }
            }
            HotShotEvent::UpgradeCertificateFormed(cert) => {
                debug!(
                    "Upgrade certificate received for view {}!",
                    *cert.view_number
                );

                // Update our current upgrade_cert as long as we still have a chance of reaching a decide on it in time.
                if cert.data.decide_by >= self.cur_view + 3 {
                    debug!("Updating current formed_upgrade_certificate");

                    self.formed_upgrade_certificate = Some(cert.clone());
                }
            }
            #[cfg(not(feature = "dependency-tasks"))]
            HotShotEvent::DACertificateRecv(cert) => {
                debug!("DAC Received for view {}!", *cert.view_number);
                let view = cert.view_number;

                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForDAC(*view))
                    .await;

                self.committee_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                self.consensus
                    .write()
                    .await
                    .saved_da_certs
                    .insert(view, cert.clone());

                if self.vote_if_able(&event_stream).await {
                    self.current_proposal = None;
                }
            }
            #[cfg(not(feature = "dependency-tasks"))]
            HotShotEvent::VIDShareRecv(disperse) => {
                let view = disperse.data.get_view_number();

                debug!(
                    "VID disperse received for view: {:?} in consensus task",
                    view
                );

                // Allow VID disperse date that is one view older, in case we have updated the
                // view.
                // Adding `+ 1` on the LHS rather than `- 1` on the RHS, to avoid the overflow
                // error due to subtracting the genesis view number.
                if view + 1 < self.cur_view {
                    warn!("Throwing away VID disperse data that is more than one view older");
                    return;
                }

                debug!("VID disperse data is not more than one view older.");

                if !self.validate_disperse(disperse) {
                    warn!("Could not verify VID dispersal/share sig.");
                    return;
                }

                self.consensus
                    .write()
                    .await
                    .vid_shares
                    .entry(view)
                    .or_default()
                    .insert(disperse.data.recipient_key.clone(), disperse.clone());
                if disperse.data.recipient_key != self.public_key {
                    return;
                }
                // stop polling for the received disperse after verifying it's valid
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDDisperse(
                        *disperse.data.view_number,
                    ))
                    .await;
                if self.vote_if_able(&event_stream).await {
                    self.current_proposal = None;
                }
            }
            HotShotEvent::ViewChange(new_view) => {
                let new_view = *new_view;
                debug!("View Change event for view {} in consensus task", *new_view);

                let old_view_number = self.cur_view;

                // Start polling for VID disperse for the new view
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *old_view_number + 1,
                    ))
                    .await;

                // If we have a decided upgrade certificate,
                // we may need to upgrade the protocol version on a view change.
                if let Some(ref cert) = self.decided_upgrade_cert {
                    if new_view == cert.data.new_version_first_view {
                        warn!(
                            "Updating version based on a decided upgrade cert: {:?}",
                            cert
                        );
                        let mut version = self.version.write().await;
                        *version = cert.data.new_version;

                        broadcast_event(
                            Arc::new(HotShotEvent::VersionUpgrade(cert.data.new_version)),
                            &event_stream,
                        )
                        .await;
                    }
                }

                // update the view in state to the one in the message
                // Publish a view change event to the application
                // Returns if the view does not need updating.
                if let Err(e) = update_view::<TYPES, I>(
                    self.public_key.clone(),
                    new_view,
                    &event_stream,
                    self.quorum_membership.clone(),
                    self.quorum_network.clone(),
                    self.timeout,
                    self.consensus.clone(),
                    &mut self.cur_view,
                    &mut self.timeout_task,
                )
                .await
                {
                    warn!("Failed to update view; error = {e:?}");
                    return;
                }

                broadcast_event(
                    Event {
                        view_number: old_view_number,
                        event: EventType::ViewFinished {
                            view_number: old_view_number,
                        },
                    },
                    &self.output_event_stream,
                )
                .await;
            }
            HotShotEvent::Timeout(view) => {
                let view = *view;
                // NOTE: We may optionally have the timeout task listen for view change events
                if self.cur_view >= view {
                    return;
                }
                if !self.timeout_membership.has_stake(&self.public_key) {
                    debug!(
                        "We were not chosen for consensus committee on {:?}",
                        self.cur_view
                    );
                    return;
                }

                // cancel poll for votes
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                // cancel poll for proposal
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(*view))
                    .await;

                let Ok(vote) = TimeoutVote::create_signed_vote(
                    TimeoutData { view },
                    view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign TimeoutData!");
                    return;
                };

                broadcast_event(Arc::new(HotShotEvent::TimeoutVoteSend(vote)), &event_stream).await;
                broadcast_event(
                    Event {
                        view_number: view,
                        event: EventType::ViewTimeout { view_number: view },
                    },
                    &self.output_event_stream,
                )
                .await;
                debug!(
                    "We did not receive evidence for view {} in time, sending timeout vote for that view!",
                    *view
                );

                broadcast_event(
                    Event {
                        view_number: view,
                        event: EventType::ReplicaViewTimeout { view_number: view },
                    },
                    &self.output_event_stream,
                )
                .await;
                let consensus = self.consensus.read().await;
                consensus.metrics.number_of_timeouts.add(1);
            }
            HotShotEvent::SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                metadata,
                view,
                fee,
            ) => {
                let view = *view;
                debug!("got commit and meta {:?}", payload_commitment);
                self.payload_commitment_and_metadata = Some(CommitmentAndMetadata {
                    commitment: *payload_commitment,
                    builder_commitment: builder_commitment.clone(),
                    metadata: metadata.clone(),
                    fee: fee.clone(),
                });
                if self.quorum_membership.get_leader(view) == self.public_key
                    && self.consensus.read().await.high_qc.get_view_number() + 1 == view
                {
                    self.publish_proposal_if_able(view, &event_stream).await;
                }

                if let Some(cert) = &self.proposal_cert {
                    match cert {
                        ViewChangeEvidence::Timeout(tc) => {
                            if self.quorum_membership.get_leader(tc.get_view_number() + 1)
                                == self.public_key
                            {
                                self.publish_proposal_if_able(view, &event_stream).await;
                            }
                        }
                        ViewChangeEvidence::ViewSync(vsc) => {
                            if self.quorum_membership.get_leader(vsc.get_view_number())
                                == self.public_key
                            {
                                self.publish_proposal_if_able(view, &event_stream).await;
                            }
                        }
                    }
                }
            }
            HotShotEvent::ViewSyncFinalizeCertificate2Recv(certificate) => {
                if !certificate.is_valid_cert(self.quorum_membership.as_ref()) {
                    warn!(
                        "View Sync Finalize certificate {:?} was invalid",
                        certificate.get_data()
                    );
                    return;
                }

                self.proposal_cert = Some(ViewChangeEvidence::ViewSync(certificate.clone()));

                // cancel poll for votes
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(
                        *certificate.view_number - 1,
                    ))
                    .await;

                let view = certificate.view_number;

                if self.quorum_membership.get_leader(view) == self.public_key {
                    debug!(
                        "Attempting to publish proposal after forming a View Sync Finalized Cert for view {}",
                        *certificate.view_number
                    );
                    self.publish_proposal_if_able(view, &event_stream).await;
                }
            }
            _ => {}
        }
    }

    /// Ignores old propose behavior and lets QuorumProposalTask take over.
    #[cfg(feature = "dependency-tasks")]
    pub async fn publish_proposal_if_able(
        &mut self,
        _view: TYPES::Time,
        _event_stream: &Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
    }

    /// Sends a proposal if possible from the high qc we have
    #[allow(clippy::too_many_lines)]
    #[cfg(not(feature = "dependency-tasks"))]
    pub async fn publish_proposal_if_able(
        &mut self,
        view: TYPES::Time,
        event_stream: &Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        use crate::consensus::proposal::create_and_send_proposal;

        if self.quorum_membership.get_leader(view) != self.public_key {
            // This is expected for view 1, so skipping the logging.
            if view != TYPES::Time::new(1) {
                error!(
                    "Somehow we formed a QC but are not the leader for the next view {:?}",
                    view
                );
            }
            return;
        }

        let consensus = self.consensus.read().await;
        let parent_view_number = &consensus.high_qc.get_view_number();
        let mut reached_decided = false;

        let Some(parent_view) = consensus.validated_state_map.get(parent_view_number) else {
            // This should have been added by the replica?
            error!("Couldn't find parent view in state map, waiting for replica to see proposal\n parent view number: {}", **parent_view_number);
            return;
        };
        // Leaf hash in view inner does not match high qc hash - Why?
        let Some((leaf_commitment, state)) = parent_view.get_leaf_and_state() else {
            error!(
                ?parent_view_number,
                ?parent_view,
                "Parent of high QC points to a view without a proposal"
            );
            return;
        };
        if leaf_commitment != consensus.high_qc.get_data().leaf_commit {
            // NOTE: This happens on the genesis block
            debug!(
                "They don't equal: {:?}   {:?}",
                leaf_commitment,
                consensus.high_qc.get_data().leaf_commit
            );
        }
        let Some(leaf) = consensus.saved_leaves.get(&leaf_commitment) else {
            error!("Failed to find high QC of parent.");
            return;
        };
        if leaf.get_view_number() == consensus.last_decided_view {
            reached_decided = true;
        }

        let parent_leaf = leaf.clone();

        let original_parent_hash = parent_leaf.commit();

        let mut next_parent_hash = original_parent_hash;

        // Walk back until we find a decide
        if !reached_decided {
            debug!("We have not reached decide from view {:?}", self.cur_view);
            while let Some(next_parent_leaf) = consensus.saved_leaves.get(&next_parent_hash) {
                if next_parent_leaf.get_view_number() <= consensus.last_decided_view {
                    break;
                }
                next_parent_hash = next_parent_leaf.get_parent_commitment();
            }
            debug!("updated saved leaves");
            // TODO do some sort of sanity check on the view number that it matches decided
        }

        // Special case: if we have a decided upgrade certificate AND it does not apply a version to the current view, we MUST propose with a null block.
        if let Some(upgrade_cert) = &self.decided_upgrade_cert {
            if upgrade_cert.in_interim(self.cur_view) {
                let Ok((payload, metadata)) =
                    <TYPES::BlockPayload as BlockPayload>::from_transactions(Vec::new())
                else {
                    error!("Failed to build null block payload and metadata");
                    return;
                };

                let builder_commitment = payload.builder_commitment(&metadata);
                let Some(null_block_commitment) =
                    null_block::commitment(self.quorum_membership.total_nodes())
                else {
                    // This should never happen.
                    error!("Failed to calculate null block commitment");
                    return;
                };

                let Some(null_block_fee) =
                    null_block::builder_fee::<TYPES>(self.quorum_membership.total_nodes())
                else {
                    // This should never happen.
                    error!("Failed to calculate null block fee info");
                    return;
                };

                let pub_key = self.public_key.clone();
                let priv_key = self.private_key.clone();
                let consensus = self.consensus.clone();
                let sender = event_stream.clone();
                let delay = self.round_start_delay;
                let parent = parent_leaf.clone();
                let state = state.clone();
                let upgrade_cert = self.decided_upgrade_cert.clone();
                self.spawned_tasks
                    .entry(view)
                    .or_default()
                    .push(async_spawn(async move {
                        create_and_send_proposal(
                            pub_key,
                            priv_key,
                            consensus,
                            sender,
                            view,
                            CommitmentAndMetadata {
                                commitment: null_block_commitment,
                                builder_commitment,
                                metadata,
                                fee: null_block_fee,
                            },
                            parent,
                            state,
                            upgrade_cert,
                            None,
                            delay,
                        )
                        .await;
                    }));
                return;
            }
        }

        if let Some(commit_and_metadata) = &self.payload_commitment_and_metadata {
            // In order of priority, we should try to attach:
            //   - the parent certificate if it exists, or
            //   - our own certificate that we formed.
            // In either case, we need to ensure that the certificate is still relevant.
            //
            // Note: once we reach a point of potentially propose with our formed upgrade certificate, we will ALWAYS drop it. If we cannot immediately use it for whatever reason, we choose to discard it.
            // It is possible that multiple nodes form separate upgrade certificates for the some upgrade if we are not careful about voting. But this shouldn't bother us: the first leader to propose is the one whose certificate will be used. And if that fails to reach a decide for whatever reason, we may lose our own certificate, but something will likely have gone wrong there anyway.
            let formed_upgrade_certificate = self.formed_upgrade_certificate.take();
            let mut proposal_upgrade_certificate = parent_leaf
                .get_upgrade_certificate()
                .or(formed_upgrade_certificate);

            if !proposal_upgrade_certificate.clone().is_some_and(|cert| {
                cert.is_relevant(view, self.decided_upgrade_cert.clone())
                    .is_ok()
            }) {
                proposal_upgrade_certificate = None;
            }

            // We only want to proposal to be attached if any of them are valid.
            let proposal_certificate = self
                .proposal_cert
                .as_ref()
                .filter(|cert| cert.is_valid_for_view(&view))
                .cloned();
            let pub_key = self.public_key.clone();
            let priv_key = self.private_key.clone();
            let consensus = self.consensus.clone();
            let sender = event_stream.clone();
            let state = state.clone();
            let delay = self.round_start_delay;
            let commitment_and_metadata = commit_and_metadata.clone();
            self.spawned_tasks
                .entry(view)
                .or_default()
                .push(async_spawn(async move {
                    create_and_send_proposal(
                        pub_key,
                        priv_key,
                        consensus,
                        sender,
                        view,
                        commitment_and_metadata,
                        parent_leaf.clone(),
                        state,
                        proposal_upgrade_certificate,
                        proposal_certificate,
                        delay,
                    )
                    .await;
                }));

            self.proposal_cert = None;
            self.payload_commitment_and_metadata = None;
        }
        debug!("Cannot propose because we don't have the VID payload commitment and metadata");
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TaskState
    for ConsensusTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;
    type Output = ();
    fn filter(&self, event: &Arc<HotShotEvent<TYPES>>) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::QuorumProposalRecv(_, _)
                | HotShotEvent::QuorumVoteRecv(_)
                | HotShotEvent::QuorumProposalValidated(..)
                | HotShotEvent::QCFormed(_)
                | HotShotEvent::UpgradeCertificateFormed(_)
                | HotShotEvent::DACertificateRecv(_)
                | HotShotEvent::ViewChange(_)
                | HotShotEvent::SendPayloadCommitmentAndMetadata(..)
                | HotShotEvent::Timeout(_)
                | HotShotEvent::TimeoutVoteRecv(_)
                | HotShotEvent::VIDShareRecv(..)
                | HotShotEvent::ViewSyncFinalizeCertificate2Recv(_)
                | HotShotEvent::Shutdown,
        )
    }
    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()>
    where
        Self: Sized,
    {
        let sender = task.clone_sender();
        tracing::trace!("sender queue len {}", sender.len());
        task.state_mut().handle(event, sender).await;
        None
    }
    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}
