use crate::events::SequencingHotShotEvent;
use async_compatibility_layer::{
    art::{async_spawn, async_timeout},
    async_primitives::subscribable_rwlock::ReadView,
};
use async_lock::RwLock;
use bincode::config::Options;
use bitvec::prelude::*;
use commit::Committable;
use either::{Either, Left, Right};
use futures::FutureExt;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{FilterEvent, HandleEvent, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEvent, TaskBuilder},
};
use hotshot_types::{
    certificate::DACertificate,
    consensus::{Consensus, View},
    data::{DAProposal, ProposalType, SequencingLeaf},
    message::{CommitteeConsensusMessage, Message, Proposal, SequencingMessage},
    traits::{
        consensus_api::SequencingConsensusApi,
        election::{CommitteeExchangeType, ConsensusExchange, Membership},
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{CommitteeEx, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        state::ConsensusTime,
        Block, State,
    },
    utils::ViewInner,
    vote::VoteAccumulator,
};
use hotshot_utils::bincode::bincode_opts;
use snafu::Snafu;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Instant,
};
use tracing::{debug, error, instrument, warn};

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}
 
/// Tracks state of a DA task
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

    // pub transactions: Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>>,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,

    /// This state's ID
    pub id: u64,

    /// Event stream to publish events to the application layer
    pub output_event_stream: ChannelStream<Event<TYPES, I::Leaf>>,
}


impl<
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I> + 'static,
    > DATaskState<TYPES, I, A>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]

    pub async fn handle_event(
        &mut self,
        event: SequencingHotShotEvent<TYPES, I>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            SequencingHotShotEvent::TransactionsRecv(transactions) => {
                // TODO ED Add validation checks
                let mut consensus = self.consensus.write().await;
                consensus
                    .get_transactions()
                    .modify(|txns| {
                        for transaction in transactions {
                            let size = bincode_opts().serialized_size(&transaction).unwrap_or(0);

                            // If we didn't already know about this transaction, update our mempool metrics.
                            if !consensus.seen_transactions.remove(&transaction.commit())
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
            SequencingHotShotEvent::DAProposalRecv(proposal, sender) => {
            }
            SequencingHotShotEvent::LeafDecided(leaf_chain) => {
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
                
            }
            // TODO ED Update high QC through QCFormed event
            SequencingHotShotEvent::ViewChange(view) => {
            }

            SequencingHotShotEvent::Timeout(view) => {
                self.committee_exchange
                    .network()
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;
            }

            SequencingHotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {}
        }
        None
    }
}