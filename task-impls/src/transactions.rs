use crate::events::SequencingHotShotEvent;
use async_compatibility_layer::async_primitives::subscribable_rwlock::SubscribableRwLock;
use async_compatibility_layer::{
    art::async_timeout, async_primitives::subscribable_rwlock::ReadView,
};
use async_lock::RwLock;
use bincode::config::Options;
use commit::Commitment;
use commit::Committable;
use either::{Left, Right};
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{HotShotTaskCompleted, TS},
    task_impls::HSTWithEvent,
};
use hotshot_types::{
    block_impl::{VIDBlockPayload, VIDTransaction},
    certificate::DACertificate,
    consensus::Consensus,
    data::{SequencingLeaf, VidDisperse, VidScheme, VidSchemeTrait},
    message::{Message, Proposal, SequencingMessage},
    traits::{
        block_contents::Transaction,
        consensus_api::SequencingConsensusApi,
        election::{CommitteeExchangeType, ConsensusExchange},
        node_implementation::{CommitteeEx, NodeImplementation, NodeType},
        BlockPayload,
    },
};
use hotshot_utils::bincode::bincode_opts;
use snafu::Snafu;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Instant,
};
use tracing::{debug, error, instrument, warn};

/// A type alias for `HashMap<Commitment<T>, T>`
type CommitmentMap<T> = HashMap<Commitment<T>, T>;

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a Transaction task
pub struct TransactionTaskState<
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
> where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// The state's api
    pub api: A,
    /// Global registry task for the state
    pub registry: GlobalRegistry,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,

    /// A list of undecided transactions
    pub transactions: Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>>,

    /// A list of transactions we've seen decided, but didn't receive
    pub seen_transactions: HashSet<Commitment<TYPES::Transaction>>,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,

    /// This state's ID
    pub id: u64,
}

impl<
        TYPES: NodeType<Transaction = VIDTransaction, BlockType = VIDBlockPayload>,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > TransactionTaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction Handling Task", level = "error")]

    pub async fn handle_event(
        &mut self,
        event: SequencingHotShotEvent<TYPES, I>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            SequencingHotShotEvent::TransactionsRecv(transactions) => {
                let consensus = self.consensus.read().await;
                self.transactions
                    .modify(|txns| {
                        for transaction in transactions {
                            let size = bincode_opts().serialized_size(&transaction).unwrap_or(0);

                            // If we didn't already know about this transaction, update our mempool metrics.
                            if !self.seen_transactions.remove(&transaction.commit())
                                && txns.insert(transaction.commit(), transaction).is_none()
                            {
                                consensus.metrics.outstanding_transactions.update(1);
                                consensus
                                    .metrics
                                    .outstanding_transactions_memory_size
                                    .update(i64::try_from(size).unwrap_or_else(|e| {
                                        warn!("Conversion failed: {e}. Using the max value.");
                                        i64::MAX
                                    }));
                            }
                        }
                    })
                    .await;

                return None;
            }
            SequencingHotShotEvent::LeafDecided(leaf_chain) => {
                let mut included_txns = HashSet::new();
                let mut included_txn_size = 0;
                let mut included_txn_count = 0;
                for leaf in leaf_chain {
                    match &leaf.deltas {
                        Left(block) => {
                            let txns = block.contained_transactions();
                            for txn in txns {
                                included_txns.insert(txn);
                            }
                        }
                        Right(_) => {}
                    }
                }
                let consensus = self.consensus.read().await;
                let txns = self.transactions.cloned().await;

                let _ = included_txns.iter().map(|hash| {
                    if !txns.contains_key(hash) {
                        self.seen_transactions.insert(*hash);
                    }
                });
                drop(txns);
                self.transactions
                    .modify(|txns| {
                        *txns = txns
                            .drain()
                            .filter(|(txn_hash, txn)| {
                                if included_txns.contains(txn_hash) {
                                    included_txn_count += 1;
                                    included_txn_size +=
                                        bincode_opts().serialized_size(txn).unwrap_or_default();
                                    false
                                } else {
                                    true
                                }
                            })
                            .collect();
                    })
                    .await;

                consensus
                    .metrics
                    .outstanding_transactions
                    .update(-included_txn_count);
                consensus
                    .metrics
                    .outstanding_transactions_memory_size
                    .update(-(i64::try_from(included_txn_size).unwrap_or(i64::MAX)));
                return None;
            }
            SequencingHotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // If we are not the next leader (DA leader for this view) immediately exit
                if !self.committee_exchange.is_leader(self.cur_view + 1) {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }

                // ED Copy of parent_leaf() function from sequencing leader

                let consensus = self.consensus.read().await;
                let parent_view_number = &consensus.high_qc.view_number;

                let Some(parent_view) = consensus.state_map.get(parent_view_number) else {
                    error!(
                        "Couldn't find high QC parent in state map. Parent view {:?}",
                        parent_view_number
                    );
                    return None;
                };
                let Some(leaf) = parent_view.get_leaf_commitment() else {
                    error!(
                        ?parent_view_number,
                        ?parent_view,
                        "Parent of high QC points to a view without a proposal"
                    );
                    return None;
                };
                let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
                    error!("Failed to find high QC parent.");
                    return None;
                };
                let parent_leaf = leaf.clone();

                drop(consensus);

                let txns = self.wait_for_transactions(parent_leaf).await?;

                debug!("Prepare VID shares");
                if !txns.is_empty() {
                    // TODO https://github.com/EspressoSystems/HotShot/issues/1693
                    const NUM_STORAGE_NODES: usize = 10;
                    // TODO https://github.com/EspressoSystems/HotShot/issues/1693
                    const NUM_CHUNKS: usize = 5;
                    // TODO https://github.com/EspressoSystems/HotShot/issues/1686
                    let srs = hotshot_types::data::test_srs(NUM_STORAGE_NODES);
                    let vid = VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, &srs).unwrap();
                    // TODO https://github.com/EspressoSystems/jellyfish/issues/375
                    let mut txns_flatten = Vec::new();
                    for txn in &txns {
                        txns_flatten.extend(txn.bytes());
                    }
                    let vid_disperse = vid.disperse(&txns_flatten).unwrap();
                    let block = VIDBlockPayload::new(txns, vid_disperse.commit);

                    self.event_stream
                        .publish(SequencingHotShotEvent::BlockReady(block.clone(), view + 1))
                        .await;

                    self.event_stream
                        .publish(SequencingHotShotEvent::VidDisperseSend(
                            Proposal {
                                data: VidDisperse {
                                    view_number: view + 1,
                                    commitment: block.commit(),
                                    shares: vid_disperse.shares,
                                    common: vid_disperse.common,
                                },
                                // TODO (Keyao) This is also signed in DA task.
                                signature: self
                                    .committee_exchange
                                    .sign_da_proposal(&block.commit()),
                            },
                            // TODO don't send to committee, send to quorum (consensus.rs) https://github.com/EspressoSystems/HotShot/issues/1696
                            self.committee_exchange.public_key().clone(),
                        ))
                        .await;
                }
                return None;
            }
            SequencingHotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {}
        }
        None
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > TransactionTaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction Handling Task", level = "error")]
    async fn wait_for_transactions(
        &self,
        _parent_leaf: SequencingLeaf<TYPES>,
    ) -> Option<Vec<TYPES::Transaction>> {
        let task_start_time = Instant::now();

        // TODO (Keyao) Investigate the use of transaction hash
        // <https://github.com/EspressoSystems/HotShot/issues/1811>
        // let parent_leaf = self.parent_leaf().await?;
        // let previous_used_txns = match parent_leaf.deltas {
        //     Either::Left(block) => block.contained_transactions(),
        //     Either::Right(_commitment) => HashSet::new(),
        // };

        let receiver = self.transactions.subscribe().await;

        loop {
            let all_txns = self.transactions.cloned().await;
            debug!("Size of transactions: {}", all_txns.len());
            // TODO (Keyao) Investigate the use of transaction hash
            // <https://github.com/EspressoSystems/HotShot/issues/1811>
            // let unclaimed_txns: Vec<_> = all_txns
            //     .iter()
            //     .filter(|(txn_hash, _txn)| !previous_used_txns.contains(txn_hash))
            //     .collect();
            let unclaimed_txns = all_txns;

            let time_past = task_start_time.elapsed();
            if unclaimed_txns.len() < self.api.min_transactions()
                && (time_past < self.api.propose_max_round_time())
            {
                let duration = self.api.propose_max_round_time() - time_past;
                let result = async_timeout(duration, receiver.recv()).await;
                match result {
                    Err(_) => {
                        // Fall through below to updating new block
                        error!(
                            "propose_max_round_time passed, sending transactions we have so far"
                        );
                    }
                    Ok(Err(e)) => {
                        // Something unprecedented is wrong, and `transactions` has been dropped
                        error!("Channel receiver error for SubscribableRwLock {:?}", e);
                        return None;
                    }
                    Ok(Ok(_)) => continue,
                }
            }
            break;
        }
        let all_txns = self.transactions.cloned().await;
        // TODO (Keyao) Investigate the use of transaction hash
        // <https://github.com/EspressoSystems/HotShot/issues/1811>
        let txns: Vec<TYPES::Transaction> = all_txns.values().cloned().collect();
        // let txns: Vec<TYPES::Transaction> = all_txns
        //     .iter()
        //     .filter_map(|(txn_hash, txn)| {
        //         if previous_used_txns.contains(txn_hash) {
        //             None
        //         } else {
        //             Some(txn.clone())
        //         }
        //     })
        //     .collect();
        Some(txns)
    }

    /// Event filter for the transaction task
    pub fn filter(event: &SequencingHotShotEvent<TYPES, I>) -> bool {
        matches!(
            event,
            SequencingHotShotEvent::TransactionsRecv(_)
                | SequencingHotShotEvent::LeafDecided(_)
                | SequencingHotShotEvent::Shutdown
                | SequencingHotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for Transactions Task
impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > TS for TransactionTaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
}

/// Type alias for DA Task Types
pub type TransactionsTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    SequencingHotShotEvent<TYPES, I>,
    ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    TransactionTaskState<TYPES, I, A>,
>;
