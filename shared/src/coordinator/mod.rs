use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Bound,
    sync::Arc,
    time::Duration,
};

use async_broadcast::Sender;
use async_lock::{Mutex, RwLock};
use builder_state_map::BuilderStateMap;
use either::Either;
use hotshot_builder_api::v0_99::builder::BuildError;
use hotshot_types::{
    data::{DaProposal, QuorumProposal2},
    event::LeafInfo,
    traits::{
        block_contents::BlockHeader,
        node_implementation::{ConsensusTime, NodeType},
    },
};
use tracing::{error, info, warn};

use crate::{
    block::{BuilderStateId, ParentBlockReferences, ReceivedTransaction},
    state::BuilderState,
    utils::ProposalId,
};

pub mod builder_state_map;

type ProposalMap<Types> =
    HashMap<ProposalId<Types>, Either<QuorumProposal2<Types>, DaProposal<Types>>>;

/// Result of looking up a builder state by ID.
///
/// Different from an [`Option`] as it distinguishes between
/// two cases: one where the [`BuilderStateId`] is for a view that has
/// already been marked as decided - meaning there's no way
/// it will exist again - and another where the [`BuilderStateId`] is
/// for a not yet decided view, indicating a chance that
/// this entry may be populated at some point in the future.
#[derive(Clone, Debug)]
pub enum BuilderStateLookup<Types>
where
    Types: NodeType,
{
    /// Corresponding [`BuilderState`] doesn't exist
    NotFound,
    /// The view number looked up was already decided
    Decided,
    /// Successful lookup
    Found(Arc<BuilderState<Types>>),
}

/// A coordinator managing the lifecycle of [`BuilderState`]s.
///
/// Its responsibilities include:
/// - Storing builder states and allowing their lookup
/// - Spawning new builder states
/// - Distributing transactions to builder states through a broadcast channel
/// - Removing outdated builder states
///
/// <div class="warning">
///
///   Important: [`BuilderState`]s do not automatically remove transactions from the channel.
///   Refer to [`BuilderState::collect_txns`] for more details on manually dequeuing transactions.
///
/// </div>
///
/// For the coordinator to function correctly, the following handler functions
/// must be invoked when receiving corresponding HotShot events:
/// - [`Self::handle_decide`]
/// - [`Self::handle_quorum_proposal`]
/// - [`Self::handle_da_proposal`]
/// - [`Self::handle_transaction`]
pub struct BuilderStateCoordinator<Types>
where
    Types: NodeType,
{
    builder_states: RwLock<BuilderStateMap<Types>>,
    transaction_sender: Sender<Arc<ReceivedTransaction<Types>>>,
    proposals: Mutex<ProposalMap<Types>>,
}

impl<Types> BuilderStateCoordinator<Types>
where
    Types: NodeType,
{
    /// Constructs a new [`BuilderState`] coordinator.
    /// `txn_channel_capacity` controls the size of the channel used to distribute transactions to [`BuilderState`]s.
    /// `txn_garbage_collect_duration` specifies the duration for which the coordinator retains the hashes of transactions
    /// that have been marked as included by its [`BuilderState`]s. Once this duration has elapsed, new [`BuilderState`]s
    /// can include duplicates of older transactions should such be received again.
    pub fn new(txn_channel_capacity: usize, txn_garbage_collect_duration: Duration) -> Self {
        let (txn_sender, txn_receiver) = async_broadcast::broadcast(txn_channel_capacity);
        let bootstrap_state = BuilderState::new(
            ParentBlockReferences::bootstrap(),
            txn_garbage_collect_duration,
            txn_receiver,
            Types::ValidatedState::default(),
        );
        let mut builder_states = BuilderStateMap::new();
        builder_states.insert(bootstrap_state);

        Self {
            transaction_sender: txn_sender,
            builder_states: RwLock::new(builder_states),
            proposals: Mutex::new(ProposalMap::new()),
        }
    }

    /// This function should be called whenever new decide events are received from HotShot.
    /// Its main responsibility is to perform garbage collection of [`BuilderState`]s for older views.
    /// The function returns the [`BuilderState`]s that have been garbage collected.
    #[tracing::instrument(skip_all)]
    pub async fn handle_decide(
        &self,
        leaf_chain: Arc<Vec<LeafInfo<Types>>>,
    ) -> BuilderStateMap<Types> {
        let latest_decide_view_num = leaf_chain[0].leaf.view_number();
        let mut builder_states = self.builder_states.write().await;
        let highest_active_view_num = builder_states
            .highest_view()
            .unwrap_or(Types::View::genesis());
        let cutoff = Types::View::new(*latest_decide_view_num.min(highest_active_view_num));
        builder_states.prune(cutoff)
    }

    /// This function should be called whenever new transactions are received from HotShot.
    /// <div class="warning">
    ///
    ///   Important: [`BuilderState`]s do not automatically remove transactions from the channel.
    ///   Refer to [`BuilderState::collect_txns`] for more details on manually dequeuing transactions.
    ///
    /// </div>
    #[tracing::instrument(skip_all, fields(transaction = %transaction.commit))]
    #[must_use]
    pub async fn handle_transaction(
        &self,
        transaction: ReceivedTransaction<Types>,
    ) -> Result<(), BuildError> {
        match self.transaction_sender.try_broadcast(Arc::new(transaction)) {
            Ok(None) => Ok(()),
            Ok(Some(evicted_txn)) => {
                warn!(
                    ?evicted_txn.commit,
                    "Overflow mode enabled, transaction evicted",
                );
                Ok(())
            }
            Err(err) => {
                warn!(?err, "Failed to broadcast txn");
                Err(BuildError::Error(err.to_string()))
            }
        }
    }

    /// This function should be called whenever new DA Proposal is recieved from HotShot.
    /// Coordinator uses matching Quorum and DA proposals to track creation of new blocks
    /// and spawning corresponding builder states for those.
    pub async fn handle_da_proposal(&self, da_proposal: DaProposal<Types>) {
        let proposal_id = ProposalId::from_da_proposal(&da_proposal);
        self.handle_proposal(proposal_id, Either::Right(da_proposal))
            .await;
    }

    /// This function should be called whenever new Quorum Proposal is recieved from HotShot.
    /// Coordinator uses matching Quorum and DA proposals to track creation of new blocks
    /// and spawning corresponding builder states for those.
    pub async fn handle_quorum_proposal(&self, quorum_proposal: QuorumProposal2<Types>) {
        let proposal_id = ProposalId::from_quorum_proposal(&quorum_proposal);
        self.handle_proposal(proposal_id, Either::Left(quorum_proposal))
            .await;
    }

    /// Generalized function to handle Quorum and DA proposals. The behavior is as follows:
    ///
    /// - If a matching proposal of the other kind exists for this [`ProposalId`], remove it
    ///   from storage and spawn a new [`BuilderState`] from the resulting proposal pair.
    /// - If a proposal of the same kind is stored, do nothing.
    /// - If there are no records for this [`ProposalId`], store it.
    #[tracing::instrument(skip_all)]
    async fn handle_proposal(
        &self,
        proposal_id: ProposalId<Types>,
        proposal: Either<QuorumProposal2<Types>, DaProposal<Types>>,
    ) {
        match self.proposals.lock().await.entry(proposal_id) {
            Entry::Occupied(entry) => {
                if entry.get().is_left() == proposal.is_left() {
                    // Duplicate proposal, ignore.
                    return;
                }

                match (entry.remove(), proposal) {
                    (Either::Right(da_proposal), Either::Left(quorum_proposal))
                    | (Either::Left(quorum_proposal), Either::Right(da_proposal)) => {
                        self.spawn_builder_state(quorum_proposal, da_proposal).await
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(proposal);
            }
        }
    }

    /// Get the lowest view we have stored
    pub async fn lowest_view(&self) -> Types::View {
        self.builder_states
            .read()
            .await
            .lowest_view()
            .unwrap_or(Types::View::genesis())
    }

    /// Looks up a [`BuilderState`] by id.
    ///
    /// Refer to [`BuilderStateLookup`] for more information on return value
    #[tracing::instrument(skip_all)]
    #[must_use]
    pub async fn lookup_builder_state(
        &self,
        id: &BuilderStateId<Types>,
    ) -> BuilderStateLookup<Types> {
        if let Some(entry) = self.builder_states.read().await.get(id).cloned() {
            return BuilderStateLookup::Found(entry);
        }

        if self.lowest_view().await > id.parent_view {
            return BuilderStateLookup::Decided;
        }

        BuilderStateLookup::NotFound
    }

    /// Looks up the builder state with the highest view number.
    /// If there are no builder states at all, returns [`None`].
    #[tracing::instrument(skip_all)]
    #[must_use]
    pub async fn highest_view_builder(&self) -> Option<Arc<BuilderState<Types>>> {
        self.builder_states
            .read()
            .await
            .highest_view_builder()
            .cloned()
    }

    /// Spawn a new builder state off of matching pair of Quorum and DA proposals, store it in [`Self::builder_states`]
    async fn spawn_builder_state(
        &self,
        quorum_proposal: QuorumProposal2<Types>,
        da_proposal: DaProposal<Types>,
    ) {
        assert_eq!(quorum_proposal.view_number, da_proposal.view_number);

        let mut candidate_parents = self.find_builder_states_to_extend(&quorum_proposal).await;

        if candidate_parents.is_empty() {
            error!(
                ?quorum_proposal,
                ?da_proposal,
                "Couldn't find a parent for new builder state"
            );
        }

        if candidate_parents.len() > 1 {
            info!(
                ?candidate_parents,
                "Multiple candidates for new builder state parent"
            );
        }

        // if we have multiple candidate states, this is the simplest way to choose one
        let Some(parent_state) = candidate_parents.pop() else {
            return;
        };

        let child_state = parent_state
            .new_child(quorum_proposal.clone(), da_proposal.clone())
            .await;

        self.builder_states.write().await.insert(child_state);
    }

    /// This is an utility function that is used to determine which [`BuilderState`]s
    /// are the best fit to extend from for given [`QuorumProposal2`]
    ///
    /// In an ideal circumstance the best [`BuilderState`] to extend from is going to
    /// be the one that is immediately preceding the [`QuorumProposal2`] that we are
    /// attempting to extend from. However, if all we know is the view number of
    /// the [`QuorumProposal2`] that we are attempting to extend from, then we may end
    /// up in a scenario where we have multiple [`BuilderState`]s that are all equally
    /// valid to extend from.  When this happens, we have the potential for a data
    /// race.
    ///
    /// The primary cause of this has to due with the interface of the
    /// [`BuilderStateCoordinator`]'s API.  In general, we want to be able to retrieve
    /// a [`BuilderState`] via the [`BuilderStateId`]. The [`BuilderStateId`] only references
    /// a [`ViewNumber`](hotshot_types::data::ViewNumber) and a [`VidCommitment`](`hotshot_types::vid::VidCommitment`).
    /// While this information is available in the [`QuorumProposal2`],
    /// it only helps us to rule out [`BuilderState`]s that already exist.
    /// It does **NOT** help us to pick a [`BuilderState`] that is the best fit to extend from.
    ///
    /// This is where the `justify_qc` comes in to consideration.  The `justify_qc`
    /// contains the previous [`ViewNumber`](hotshot_types::data::ViewNumber) that is
    /// being extended from, and in addition it also contains the previous
    /// [`Commitment<Leaf<Types>>`](`committable::Commitment`)
    /// that is being built on top of.  Since our [`BuilderState`]s store identifying
    /// information that contains this same `leaf_commit` we can compare these
    /// directly to ensure that we are extending from the correct [`BuilderState`].
    ///
    /// This function determines the best [`BuilderState`] in the following steps:
    ///
    /// 1. If we have a [`BuilderState`] that is already spawned for the current
    ///    [`QuorumProposal2`], then we should should return no states, as one already
    ///    exists.  This will prevent us from attempting to spawn duplicate
    ///    [`BuilderState`]s.
    /// 2. Attempt to find all [`BuilderState`]s that are recorded within
    ///    coordinator that have matching view number and leaf commitments. There
    ///    *should* only be one of these.  But all would be valid extension points.
    /// 3. If we can't find any [`BuilderState`]s that match the view number
    ///    and leaf commitment, then we should return for the maximum stored view
    ///    number that is smaller than the current [`QuorumProposal2`].
    /// 4. If there is is only one [`BuilderState`] stored in the coordinator, then
    ///    we should return that [`BuilderState`] as the best fit.
    /// 5. If none of the other criteria match, we return an empty result as it is
    ///    unclear what to do in this case.
    ///
    /// <div class="warning">
    ///
    ///  Note: Any time this function returns more than a single entry in its
    ///  result, there is a potential for a race condition.  This is
    ///  because there are multiple [`BuilderState`]s that are equally valid to
    ///  extend from.  This race could be avoided by just picking one of the
    ///  entries in the resulting [`Vec`], but this is not done here in order
    ///  to allow us to highlight the possibility of the race.
    ///
    /// </div>
    #[must_use]
    async fn find_builder_states_to_extend(
        &self,
        quorum_proposal: &QuorumProposal2<Types>,
    ) -> Vec<Arc<BuilderState<Types>>> {
        // This is ID of the state we want to spawn
        let current_builder_state_id = BuilderStateId {
            parent_view: quorum_proposal.view_number,
            parent_commitment: quorum_proposal.block_header.payload_commitment(),
        };

        let builder_states = self.builder_states.read().await;

        // The first step is to check if we already have a spawned [BuilderState].
        // If we do, then we should indicate that there is no best fit, as we
        // don't want to spawn another [BuilderState].
        if builder_states.get(&current_builder_state_id).is_some() {
            // We already have a spawned [BuilderState] for this proposal.
            // So we should just ignore it.
            return Vec::new();
        }

        // Next we want to see if there is an immediate match for a [BuilderState]
        // that we can extend from.  This is the most ideal situation, as it
        // implies that we are extending from the correct [BuilderState].
        // We do this by checking the `justify_qc` stored within the
        // [QuorumProposal], and checking it against the current spawned
        // [BuilderState]s
        let justify_qc = &quorum_proposal.justify_qc;

        let existing_states = builder_states
            .bucket(&justify_qc.view_number)
            .filter(|state| {
                state.parent_block_references.leaf_commit == justify_qc.data.leaf_commit
            })
            .cloned()
            .collect::<Vec<_>>();

        // If we found any matching [BuilderState]s, then we should return them
        // as the best fit.
        if !existing_states.is_empty() {
            return existing_states;
        }

        warn!("No ideal match for builder state to extend");

        // At this point, we don't have any "ideal" matches or scenarios.  So we
        // need to look for a suitable fall-back. The best fallback condition to
        // start with is any [BuilderState] that has the maximum spawned view
        // number whose value is smaller than the current [QuorumProposal].
        if let Some(states) = builder_states
            .range((
                Bound::Unbounded,
                Bound::Excluded(current_builder_state_id.parent_view),
            ))
            .next()
        {
            // If we have a maximum view number that meets our criteria, then we should
            // return all [BuilderStateId]s that match this view number.
            // This can lead to multiple [BuilderStateId]s being returned.
            // If we are the maximum stored view number smaller than the quorum
            // proposal's view number, then we are the best fit.
            return states.cloned().collect();
        }

        // This is our last ditch effort to continue making progress.  If there is
        // only one [BuilderState] active, then we should return that as the best
        // fit, as it will be the only way we can continue making progress with
        // the builder.
        if builder_states.len() == 1 {
            return builder_states
                .highest_view_builder()
                .cloned()
                .into_iter()
                .collect();
        }
        drop(builder_states);

        // This implies that there are only larger [BuilderState]s active than
        // the one we are.  This is weird, it implies that some sort of time
        // travel has occurred view-wise.  It is unclear what to do in this
        // situation.
        warn!("View time-travel");
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use hotshot_example_types::node_types::TestTypes;

    use crate::{
        block::TransactionSource,
        testing::{
            constants::{TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD},
            mock,
        },
    };

    use super::*;

    type BuilderStateCoordinator = super::BuilderStateCoordinator<TestTypes>;

    #[tokio::test]
    async fn test_coordinator_new() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        assert_eq!(
            coordinator.builder_states.read().await.len(),
            1,
            "The coordinator should be populated with a bootstrap builder state."
        );

        assert_eq!(
            coordinator
                .builder_states
                .read()
                .await
                .highest_view_builder()
                .unwrap()
                .parent_block_references,
            ParentBlockReferences::bootstrap(),
            "The coordinator should be populated with a bootstrap builder state."
        );

        assert!(
            coordinator.proposals.lock().await.is_empty(),
            "The coordinator should not have any proposals stored."
        );

        assert_eq!(
            coordinator.transaction_sender.capacity(),
            TEST_CHANNEL_BUFFER_SIZE,
            "The coordinator TX channel should have the capacity passed to new.",
        );

        assert_eq!(
            coordinator
                .builder_states
                .read()
                .await
                .highest_view_builder()
                .unwrap()
                .included_txns
                .period,
            TEST_INCLUDED_TX_GC_PERIOD,
            "Coordinator-created builder states should have the GC period passed to new.",
        );
    }

    #[tokio::test]
    async fn test_handle_proposal_matching_types_creates_builder_state() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        let (da_proposal, quorum_proposal) = mock::proposals(7).await;

        // First, insert a DA proposal into the coordinator
        coordinator.handle_da_proposal(da_proposal).await;

        // Now, insert a matching quorum proposal
        coordinator.handle_quorum_proposal(quorum_proposal).await;

        // Verify that a new BuilderState has been created
        let builder_states = coordinator.builder_states.read().await;
        assert_eq!(
            builder_states.len(),
            2,
            "The coordinator should have 2 builder states: one bootstrap and one created from matching proposals."
        );
    }

    #[tokio::test]
    async fn test_handle_proposal_duplicate_proposal_ignored() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        let (proposal, _) = mock::proposals(7).await;

        // Insert the same DA proposal twice
        coordinator.handle_da_proposal(proposal.clone()).await;
        coordinator.handle_da_proposal(proposal).await;

        // Check that only one proposal exists in the storage
        let proposals = coordinator.proposals.lock().await;
        assert_eq!(
            proposals.len(),
            1,
            "There should be exactly one DA proposal stored, duplicates should be ignored."
        );
    }

    #[tokio::test]
    async fn test_handle_proposal_stores_new_proposal_when_no_match() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        let (proposal, _) = mock::proposals(1).await;
        let proposal_id = ProposalId::from_da_proposal(&proposal);

        // Insert a new DA proposal
        coordinator.handle_da_proposal(proposal).await;

        // Verify it's stored in proposals
        let proposals = coordinator.proposals.lock().await;
        assert_eq!(
            proposals.len(),
            1,
            "The proposals map should contain one proposal after adding a new DA proposal."
        );
        assert!(
            proposals.contains_key(&proposal_id),
            "The proposals map should contain the inserted DA proposal."
        );
    }

    #[tokio::test]
    async fn test_handle_proposal_same_view_different_proposals() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        let view_number = 9; // arbitrary
        let (da_proposal_1, quorum_proposal_1) = mock::proposals(view_number).await;
        let (da_proposal_2, quorum_proposal_2) = mock::proposals(view_number).await;

        // First, insert a DA proposal into the coordinator
        coordinator.handle_da_proposal(da_proposal_1).await;

        // Now, insert a quorum proposal with matching view but different payload
        coordinator.handle_quorum_proposal(quorum_proposal_2).await;

        // Verify that no new builder states has been created
        assert_eq!(
            coordinator.builder_states.read().await.len(),
            1,
            "The coordinator should have 2 builder states: one bootstrap and one created from matching proposals."
        );

        // Complete both proposal sets
        coordinator.handle_quorum_proposal(quorum_proposal_1).await;
        coordinator.handle_da_proposal(da_proposal_2).await;

        // Verify that inserting matching proposals spawns both new builder states
        assert_eq!(
            coordinator.builder_states.read().await.len(),
            3,
            "The coordinator should have 2 builder states: one bootstrap and one created from matching proposals."
        );
    }

    #[tokio::test]
    async fn test_decide_reaps_old_proposals() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        for view in 0..100 {
            let (da_proposal, quorum_proposal) = mock::proposals(view).await;
            coordinator.handle_quorum_proposal(quorum_proposal).await;
            coordinator.handle_da_proposal(da_proposal).await;
        }

        // Verify that inserting matching proposals spawn new builder states
        assert_eq!(
            coordinator.builder_states.read().await.len() as u64,
            101,
            "The coordinator should have 101 builder states: one bootstrap and one created from each pair of proposals"
        );

        coordinator
            .handle_decide(mock::decide_leaf_chain(97).await)
            .await;

        // Verify that after decide event we have only proposals for views 97, 98 and 99 left
        assert_eq!(
            coordinator.builder_states.read().await.len() as u64,
            3,
            "The coordinator should have 2 builder states left after pruning"
        );

        coordinator
            .handle_decide(mock::decide_leaf_chain(u64::MAX).await)
            .await;

        // Verify that decides do not prune the last remaining view
        assert_eq!(
            coordinator.builder_states.read().await.len() as u64,
            1,
            "The coordinator should have 2 builder states left after pruning"
        );
    }

    #[tokio::test]
    async fn test_transaction_overflow() {
        let coordinator =
            BuilderStateCoordinator::new(TEST_CHANNEL_BUFFER_SIZE, TEST_INCLUDED_TX_GC_PERIOD);

        // Coordinator should handle transactions while there's space in the buffer
        for _ in 0..TEST_CHANNEL_BUFFER_SIZE {
            coordinator
                .handle_transaction(ReceivedTransaction::new(
                    mock::transaction(),
                    TransactionSource::Public,
                ))
                .await
                .unwrap();
        }

        // Coordinator should return an error when the channel overflows
        coordinator
            .handle_transaction(ReceivedTransaction::new(
                mock::transaction(),
                TransactionSource::Public,
            ))
            .await
            .unwrap_err();

        // Clear the channel
        coordinator
            .builder_states
            .read()
            .await
            .highest_view_builder()
            .unwrap()
            .collect_txns(Instant::now() + Duration::from_millis(100))
            .await;

        // After clearing the channel, coordinator should handle transactions again
        for _ in 0..TEST_CHANNEL_BUFFER_SIZE {
            coordinator
                .handle_transaction(ReceivedTransaction::new(
                    mock::transaction(),
                    TransactionSource::Public,
                ))
                .await
                .unwrap();
        }
    }
}
