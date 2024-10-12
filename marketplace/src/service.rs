use anyhow::bail;
use hotshot::types::Event;
use hotshot_builder_api::v0_3::{
    builder::{define_api, submit_api, BuildError, Error as BuilderApiError},
    data_source::{AcceptsTxnSubmits, BuilderDataSource},
};
use hotshot_types::bundle::Bundle;
use hotshot_types::traits::block_contents::BuilderFee;
use hotshot_types::{
    data::{DaProposal, Leaf, QuorumProposal, ViewNumber},
    event::EventType,
    message::Proposal,
    traits::{
        block_contents::BlockPayload,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::{BuilderSignatureKey, SignatureKey},
    },
    utils::BuilderCommitment,
    vid::VidCommitment,
};
use tracing::{error, instrument};
use vbs::version::StaticVersion;

use std::{fmt::Debug, marker::PhantomData, time::Duration};

use crate::{
    builder_state::{
        BuildBlockInfo, DaProposalMessage, DecideMessage, MessageType, QuorumProposalMessage,
        RequestMessage, ResponseMessage, TransactionSource,
    },
    utils::{BlockId, BuilderStateId, LegacyCommit as _, ParentBlockReferences},
};
pub use async_broadcast::{broadcast, RecvError, TryRecvError};
use async_broadcast::{InactiveReceiver, Sender as BroadcastSender, TrySendError};
use async_lock::RwLock;
use async_trait::async_trait;
use committable::{Commitment, Committable};
use derivative::Derivative;
use futures::stream::StreamExt;
use futures::{future::BoxFuture, Stream};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::{fmt::Display, time::Instant};
use tagged_base64::TaggedBase64;
use tide_disco::{app::AppError, method::ReadState, App};

// It holds all the necessary information for a block
#[derive(Debug)]
pub struct BlockInfo<TYPES: NodeType> {
    pub block_payload: TYPES::BlockPayload,
    pub metadata: <<TYPES as NodeType>::BlockPayload as BlockPayload<TYPES>>::Metadata,
    pub offered_fee: u64,
}

// It holds the information for the proposed block
#[derive(Debug)]
pub struct ProposedBlockId<TYPES: NodeType> {
    pub parent_commitment: VidCommitment,
    pub payload_commitment: BuilderCommitment,
    pub parent_view: TYPES::Time,
}

impl<TYPES: NodeType> ProposedBlockId<TYPES> {
    pub fn new(
        parent_commitment: VidCommitment,
        payload_commitment: BuilderCommitment,
        parent_view: TYPES::Time,
    ) -> Self {
        ProposedBlockId {
            parent_commitment,
            payload_commitment,
            parent_view,
        }
    }
}

#[derive(Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub struct BuilderStatesInfo<TYPES: NodeType> {
    // list of all the builder states spawned for a view
    pub vid_commitments: Vec<VidCommitment>,
    // list of all the proposed blocks for a view
    pub block_ids: Vec<ProposedBlockId<TYPES>>,
}

#[derive(Debug)]
pub struct ReceivedTransaction<TYPES: NodeType> {
    // the transaction
    pub tx: TYPES::Transaction,
    // its hash
    pub commit: Commitment<TYPES::Transaction>,
    // its source
    pub source: TransactionSource,
    // received time
    pub time_in: Instant,
}

#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct GlobalState<TYPES: NodeType> {
    // data store for the blocks
    pub blocks: lru::LruCache<BlockId<TYPES>, BlockInfo<TYPES>>,

    // registered builder states
    pub spawned_builder_states: HashMap<
        BuilderStateId<TYPES>,
        (
            // This is provided as an Option for convenience with initialization.
            // When we build the initial state, we don't necessarily want to
            // have to generate a valid `ParentBlockReferences` object and register its leaf
            // commitment, as doing such would require a bit of setup.  Additionally it would
            // result in the call signature to `GlobalState::new` changing.
            // However for every subsequent BuilderState, we expect this value
            // to be populated.
            Option<Commitment<Leaf<TYPES>>>,
            BroadcastSender<MessageType<TYPES>>,
        ),
    >,

    // builder state -> last built block , it is used to respond the client
    // if the req channel times out during get_available_blocks
    pub builder_state_to_last_built_block: HashMap<BuilderStateId<TYPES>, ResponseMessage<TYPES>>,

    // sending a transaction from the hotshot/private mempool to the builder states
    // NOTE: Currently, we don't differentiate between the transactions from the hotshot and the private mempool
    pub tx_sender: BroadcastSender<Arc<ReceivedTransaction<TYPES>>>,

    // last garbage collected view number
    pub last_garbage_collected_view_num: TYPES::Time,

    // highest view running builder task
    pub highest_view_num_builder_id: BuilderStateId<TYPES>,
}

impl<TYPES: NodeType> GlobalState<TYPES> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bootstrap_sender: BroadcastSender<MessageType<TYPES>>,
        tx_sender: BroadcastSender<Arc<ReceivedTransaction<TYPES>>>,
        bootstrapped_builder_state_id: VidCommitment,
        bootstrapped_view_num: TYPES::Time,
    ) -> Self {
        let mut spawned_builder_states = HashMap::new();
        let bootstrap_id = BuilderStateId {
            parent_commitment: bootstrapped_builder_state_id,
            parent_view: bootstrapped_view_num,
        };
        spawned_builder_states.insert(bootstrap_id.clone(), (None, bootstrap_sender.clone()));
        GlobalState {
            blocks: lru::LruCache::new(NonZeroUsize::new(256).unwrap()),
            spawned_builder_states,
            tx_sender,
            last_garbage_collected_view_num: bootstrapped_view_num,
            builder_state_to_last_built_block: Default::default(),
            highest_view_num_builder_id: bootstrap_id,
        }
    }

    pub fn register_builder_state(
        &mut self,
        parent_id: BuilderStateId<TYPES>,
        parent_block_references: ParentBlockReferences<TYPES>,
        request_sender: BroadcastSender<MessageType<TYPES>>,
    ) {
        // register the builder state
        self.spawned_builder_states.insert(
            parent_id.clone(),
            (Some(parent_block_references.leaf_commit), request_sender),
        );

        // keep track of the max view number
        if parent_id.parent_view > self.highest_view_num_builder_id.parent_view {
            tracing::info!("registering builder {parent_id} as highest",);
            self.highest_view_num_builder_id = parent_id;
        } else {
            tracing::warn!(
                "builder {parent_id} created; highest registered is {}",
                self.highest_view_num_builder_id,
            );
        }
    }

    pub fn update_global_state(
        &mut self,
        state_id: BuilderStateId<TYPES>,
        build_block_info: BuildBlockInfo<TYPES>,
        response_msg: ResponseMessage<TYPES>,
    ) {
        if self.blocks.contains(&build_block_info.id) {
            self.blocks.promote(&build_block_info.id)
        } else {
            self.blocks.push(
                build_block_info.id,
                BlockInfo {
                    block_payload: build_block_info.block_payload,
                    metadata: build_block_info.metadata,
                    offered_fee: build_block_info.offered_fee,
                },
            );
        }

        // update the builder state to last built block
        self.builder_state_to_last_built_block
            .insert(state_id, response_msg);
    }

    // remove the builder state handles based on the decide event
    pub fn remove_handles(&mut self, on_decide_view: TYPES::Time) -> TYPES::Time {
        // remove everything from the spawned builder states when view_num <= on_decide_view;
        // if we don't have a highest view > decide, use highest view as cutoff.
        let cutoff = std::cmp::min(self.highest_view_num_builder_id.parent_view, on_decide_view);
        self.spawned_builder_states
            .retain(|id, _| id.parent_view >= cutoff);

        let cutoff_u64 = cutoff.u64();
        let gc_view = if cutoff_u64 > 0 { cutoff_u64 - 1 } else { 0 };

        self.last_garbage_collected_view_num = TYPES::Time::new(gc_view);

        cutoff
    }

    // private mempool submit txn
    // Currently, we don't differentiate between the transactions from the hotshot and the private mempool
    pub async fn submit_client_txns(
        &self,
        txns: Vec<<TYPES as NodeType>::Transaction>,
    ) -> Vec<Result<Commitment<<TYPES as NodeType>::Transaction>, BuildError>> {
        handle_received_txns(&self.tx_sender, txns, TransactionSource::External).await
    }

    pub fn get_channel_for_matching_builder_or_highest_view_buider(
        &self,
        key: &BuilderStateId<TYPES>,
    ) -> Result<&BroadcastSender<MessageType<TYPES>>, BuildError> {
        if let Some(id_and_sender) = self.spawned_builder_states.get(key) {
            tracing::info!("Got matching builder for parent {}", key);
            Ok(&id_and_sender.1)
        } else {
            tracing::warn!(
                "failed to recover builder for parent {}, using higest view num builder with {}",
                key,
                self.highest_view_num_builder_id,
            );
            // get the sender for the highest view number builder
            self.spawned_builder_states
                .get(&self.highest_view_num_builder_id)
                .map(|(_, sender)| sender)
                .ok_or_else(|| BuildError::Error {
                    message: "No builder state found".to_string(),
                })
        }
    }

    // check for the existence of the builder state for a view
    pub fn check_builder_state_existence_for_a_view(&self, key: &TYPES::Time) -> bool {
        // iterate over the spawned builder states and check if the view number exists
        self.spawned_builder_states
            .iter()
            .any(|(id, _)| id.parent_view == *key)
    }

    pub fn should_view_handle_other_proposals(
        &self,
        builder_view: &TYPES::Time,
        proposal_view: &TYPES::Time,
    ) -> bool {
        *builder_view == self.highest_view_num_builder_id.parent_view
            && !self.check_builder_state_existence_for_a_view(proposal_view)
    }
}

pub struct ProxyGlobalState<TYPES: NodeType, H: BuilderHooks<TYPES>> {
    // global state
    global_state: Arc<RwLock<GlobalState<TYPES>>>,

    // hooks
    hooks: Arc<H>,

    // identity keys for the builder
    // May be ideal place as GlobalState interacts with hotshot apis
    // and then can sign on responders as desired
    builder_keys: (
        TYPES::BuilderSignatureKey, // pub key
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey, // private key
    ),

    // Maximum time allotted to wait for bundle before returning an error
    api_timeout: Duration,
}

impl<TYPES, H> ProxyGlobalState<TYPES, H>
where
    TYPES: NodeType,
    H: BuilderHooks<TYPES>,
    for<'a> <<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
    for<'a> <TYPES::SignatureKey as TryFrom<&'a TaggedBase64>>::Error: Display,
{
    pub fn new(
        global_state: Arc<RwLock<GlobalState<TYPES>>>,
        hooks: Arc<H>,
        builder_keys: (
            TYPES::BuilderSignatureKey,
            <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
        ),
        api_timeout: Duration,
    ) -> Self {
        ProxyGlobalState {
            hooks,
            global_state,
            builder_keys,
            api_timeout,
        }
    }

    /// Consumes `self` and returns a `tide_disco` [`App`] with builder and private mempool APIs registered
    pub fn into_app(self) -> Result<App<Self, BuilderApiError>, AppError> {
        let builder_api = define_api::<Self, TYPES>(&Default::default())?;

        // TODO: Replace StaticVersion with proper constant when added in HotShot
        let private_mempool_api =
            submit_api::<Self, TYPES, StaticVersion<0, 1>>(&Default::default())?;

        let mut app: App<ProxyGlobalState<TYPES, H>, BuilderApiError> = App::with_state(self);

        app.register_module(
            hotshot_types::constants::MARKETPLACE_BUILDER_MODULE,
            builder_api,
        )?;

        app.register_module("txn_submit", private_mempool_api)?;

        Ok(app)
    }
}

/*
Handling Builder API responses
*/
#[async_trait]
impl<TYPES, H> BuilderDataSource<TYPES> for ProxyGlobalState<TYPES, H>
where
    TYPES: NodeType,
    H: BuilderHooks<TYPES>,
    for<'a> <<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
    for<'a> <TYPES::SignatureKey as TryFrom<&'a TaggedBase64>>::Error: Display,
{
    #[tracing::instrument(skip(self))]
    async fn bundle(
        &self,
        parent_view: u64,
        parent_hash: &VidCommitment,
        _view_number: u64,
    ) -> Result<Bundle<TYPES>, BuildError> {
        let start = Instant::now();

        let parent_view = TYPES::Time::new(parent_view);
        let state_id = BuilderStateId {
            parent_view,
            parent_commitment: *parent_hash,
        };

        loop {
            // Couldn't serve a bundle in time
            if start.elapsed() > self.api_timeout {
                tracing::warn!("Timeout while trying to serve a bundle");
                return Err(BuildError::NotFound);
            };

            let Some(id_and_sender) = self
                .global_state
                .read_arc()
                .await
                .spawned_builder_states
                .get(&state_id)
                .cloned()
            else {
                let global_state = self.global_state.read_arc().await;

                let past_gc = parent_view <= global_state.last_garbage_collected_view_num;
                // Used as an indicator that we're just bootstrapping, as they should be equal at bootstrap
                // and never otherwise.
                let last_gc_view = global_state.last_garbage_collected_view_num;
                let highest_observed_view = global_state.highest_view_num_builder_id.parent_view;
                let is_bootstrapping = last_gc_view == highest_observed_view;

                // Explicitly drop `global_state` to avoid the lock while sleeping in `else`.
                drop(global_state);

                if past_gc && !is_bootstrapping {
                    // If we couldn't find the state because the view has already been decided, we can just return an error
                    tracing::warn!(
                        last_gc_view = ?last_gc_view,
                        highest_observed_view = ?highest_observed_view,
                        "Requested a bundle for view we already GCd as decided",
                    );
                    return Err(BuildError::Error {
                        message: "Request for a bundle for a view that has already been decided."
                            .to_owned(),
                    });
                } else {
                    // If we couldn't find the state because it hasn't yet been created, try again
                    async_compatibility_layer::art::async_sleep(self.api_timeout / 10).await;
                    continue;
                }
            };

            let (response_sender, response_receiver) =
                async_compatibility_layer::channel::unbounded();

            let request = RequestMessage {
                requested_view_number: parent_view,
                response_channel: response_sender,
            };

            id_and_sender
                .1
                .broadcast(MessageType::RequestMessage(request))
                .await
                .map_err(|err| {
                    tracing::warn!(%err, "Error requesting bundle");

                    BuildError::Error {
                        message: "Error requesting bundle".to_owned(),
                    }
                })?;

            let response = async_compatibility_layer::art::async_timeout(
                self.api_timeout.saturating_sub(start.elapsed()),
                response_receiver.recv(),
            )
            .await
            .map_err(|err| {
                tracing::warn!(%err, "Couldn't get a bundle in time");

                BuildError::NotFound
            })?
            .map_err(|err| {
                tracing::warn!(%err, "Channel closed while waiting for bundle");

                BuildError::Error {
                    message: "Channel closed while waiting for bundle".to_owned(),
                }
            })?;

            let fee_signature =
                <TYPES::BuilderSignatureKey as BuilderSignatureKey>::sign_sequencing_fee_marketplace(
                    &self.builder_keys.1,
                    response.offered_fee,
                )
                .map_err(|e| BuildError::Error {
                    message: e.to_string(),
                })?;

            let sequencing_fee: BuilderFee<TYPES> = BuilderFee {
                fee_amount: response.offered_fee,
                fee_account: self.builder_keys.0.clone(),
                fee_signature,
            };

            let commitments = response
                .transactions
                .iter()
                .flat_map(|txn| <[u8; 32]>::from(txn.commit()))
                .collect::<Vec<u8>>();

            let signature =
                <TYPES::BuilderSignatureKey as BuilderSignatureKey>::sign_builder_message(
                    &self.builder_keys.1,
                    &commitments,
                )
                .map_err(|e| BuildError::Error {
                    message: e.to_string(),
                })?;

            let bundle = Bundle {
                sequencing_fee,
                transactions: response.transactions,
                signature,
            };

            tracing::info!("Serving bundle");
            tracing::trace!(?bundle);

            return Ok(bundle);
        }
    }

    async fn builder_address(
        &self,
    ) -> Result<<TYPES as NodeType>::BuilderSignatureKey, BuildError> {
        Ok(self.builder_keys.0.clone())
    }
}

#[async_trait]
impl<TYPES, H> AcceptsTxnSubmits<TYPES> for ProxyGlobalState<TYPES, H>
where
    TYPES: NodeType,
    H: BuilderHooks<TYPES>,
{
    async fn submit_txns(
        &self,
        txns: Vec<<TYPES as NodeType>::Transaction>,
    ) -> Result<Vec<Commitment<<TYPES as NodeType>::Transaction>>, BuildError> {
        tracing::debug!(
            "Submitting {:?} transactions to the builder states{:?}",
            txns.len(),
            txns.iter().map(|txn| txn.commit()).collect::<Vec<_>>()
        );
        let txns = self.hooks.process_transactions(txns).await;
        let response = self
            .global_state
            .read_arc()
            .await
            .submit_client_txns(txns)
            .await;

        tracing::debug!(
            "Transaction submitted to the builder states, sending response: {:?}",
            response
        );

        // NOTE: ideally we want to respond with original Vec<Result>
        // instead of Result<Vec> not to loose any information,
        //  but this requires changes to builder API
        response.into_iter().collect()
    }
}
#[async_trait]
impl<TYPES, H> ReadState for ProxyGlobalState<TYPES, H>
where
    TYPES: NodeType,
    H: BuilderHooks<TYPES> + 'static,
{
    type State = ProxyGlobalState<TYPES, H>;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

pub fn broadcast_channels<TYPES: NodeType>(
    capacity: usize,
) -> (BroadcastSenders<TYPES>, BroadcastReceivers<TYPES>) {
    macro_rules! pair {
        ($s:ident, $r:ident) => {
            let ($s, $r) = broadcast(capacity);
            let $r = $r.deactivate();
        };
    }

    pair!(tx_sender, tx_receiver);
    pair!(da_sender, da_receiver);
    pair!(quorum_sender, quorum_proposal_receiver);
    pair!(decide_sender, decide_receiver);

    (
        BroadcastSenders {
            transactions: tx_sender,
            da_proposal: da_sender,
            quorum_proposal: quorum_sender,
            decide: decide_sender,
        },
        BroadcastReceivers {
            transactions: tx_receiver,
            da_proposal: da_receiver,
            quorum_proposal: quorum_proposal_receiver,
            decide: decide_receiver,
        },
    )
}

// Receivers for HotShot events for the builder states
pub struct BroadcastReceivers<TYPES: NodeType> {
    /// For transactions, shared.
    pub transactions: InactiveReceiver<Arc<ReceivedTransaction<TYPES>>>,
    /// For the DA proposal.
    pub da_proposal: InactiveReceiver<MessageType<TYPES>>,
    /// For the quorum proposal.
    pub quorum_proposal: InactiveReceiver<MessageType<TYPES>>,
    /// For the decide.
    pub decide: InactiveReceiver<MessageType<TYPES>>,
}

// Senders to broadcast data from HotShot to the builder states.
pub struct BroadcastSenders<TYPES: NodeType> {
    /// For transactions, shared.
    pub transactions: BroadcastSender<Arc<ReceivedTransaction<TYPES>>>,
    /// For the DA proposal.
    pub da_proposal: BroadcastSender<MessageType<TYPES>>,
    /// For the quorum proposal.
    pub quorum_proposal: BroadcastSender<MessageType<TYPES>>,
    /// For the decide.
    pub decide: BroadcastSender<MessageType<TYPES>>,
}

#[async_trait]
pub trait BuilderHooks<TYPES: NodeType>: Sync + Send + 'static {
    #[inline(always)]
    async fn process_transactions(
        self: &Arc<Self>,
        transactions: Vec<TYPES::Transaction>,
    ) -> Vec<TYPES::Transaction> {
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(self: &Arc<Self>, _event: &Event<TYPES>) {}
}

pub struct NoHooks<TYPES: NodeType>(pub PhantomData<TYPES>);

impl<TYPES: NodeType> BuilderHooks<TYPES> for NoHooks<TYPES> {}

/// Run builder service,
/// Refer to documentation for [`ProxyGlobalState`] for more details
pub async fn run_builder_service<TYPES: NodeType<Time = ViewNumber>>(
    hooks: Arc<impl BuilderHooks<TYPES>>,
    senders: BroadcastSenders<TYPES>,
    hotshot_event_stream: impl Stream<Item = Event<TYPES>>,
) -> Result<(), anyhow::Error> {
    let mut hotshot_event_stream = std::pin::pin!(hotshot_event_stream);
    loop {
        let Some(event) = hotshot_event_stream.next().await else {
            bail!("Event stream ended");
        };

        hooks.handle_hotshot_event(&event).await;

        match event.event {
            EventType::Error { error } => {
                error!("Error event in HotShot: {:?}", error);
            }
            // tx event
            EventType::Transactions { transactions } => {
                let transactions = hooks.process_transactions(transactions).await;

                for res in handle_received_txns(
                    &senders.transactions,
                    transactions,
                    TransactionSource::HotShot,
                )
                .await
                {
                    if let Err(e) = res {
                        tracing::warn!("Failed to handle transactions; {:?}", e);
                    }
                }
            }
            // decide event
            EventType::Decide {
                block_size: _,
                leaf_chain,
                qc: _,
            } => {
                let latest_decide_view_num = leaf_chain[0].leaf.view_number();
                handle_decide_event(&senders.decide, latest_decide_view_num).await;
            }
            // DA proposal event
            EventType::DaProposal { proposal, sender } => {
                handle_da_event(&senders.da_proposal, proposal, sender).await;
            }
            // Quorum proposal event
            EventType::QuorumProposal { proposal, sender } => {
                handle_quorum_event(&senders.quorum_proposal, Arc::new(proposal), sender).await;
            }
            _ => {
                tracing::trace!("Unhandled event from Builder: {:?}", event.event);
            }
        }
    }
}

/*
Utility functions to handle the hotshot events
*/
#[instrument(skip_all, fields(sender, da_proposal.data.view_number))]
async fn handle_da_event<TYPES: NodeType>(
    da_channel_sender: &BroadcastSender<MessageType<TYPES>>,
    da_proposal: Proposal<TYPES, DaProposal<TYPES>>,
    sender: <TYPES as NodeType>::SignatureKey,
) {
    // get the encoded transactions hash
    let encoded_txns_hash = Sha256::digest(&da_proposal.data.encoded_transactions);
    // check if the sender is the leader and the signature is valid; if yes, broadcast the DA proposal
    if !sender.validate(&da_proposal.signature, &encoded_txns_hash) {
        error!("Validation Failure on DaProposal");
        return;
    }

    let view_number = da_proposal.data.view_number;
    tracing::debug!("Sending DA proposal to the builder states",);

    // form a block payload from the encoded transactions
    let block_payload = <TYPES::BlockPayload as BlockPayload<TYPES>>::from_bytes(
        &da_proposal.data.encoded_transactions,
        &da_proposal.data.metadata,
    );
    // get the builder commitment from the block payload
    let builder_commitment = block_payload.builder_commitment(&da_proposal.data.metadata);

    let txn_commitments = block_payload
        .transactions(&da_proposal.data.metadata)
        // TODO:
        //.filter(|txn| txn.namespace_id() != namespace_id)
        .map(|txn| txn.commit())
        .collect();

    let da_msg = DaProposalMessage {
        view_number,
        txn_commitments,
        sender,
        builder_commitment,
    };

    if let Err(e) = da_channel_sender
        .broadcast(MessageType::DaProposalMessage(Arc::new(da_msg)))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send DA proposal to builder states for view {:?}",
            view_number
        );
    }
}

#[instrument(skip_all, fields(sender, quorum_proposal.data.view_number))]
async fn handle_quorum_event<TYPES: NodeType>(
    quorum_channel_sender: &BroadcastSender<MessageType<TYPES>>,
    quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    sender: <TYPES as NodeType>::SignatureKey,
) {
    let leaf = Leaf::from_quorum_proposal(&quorum_proposal.data);

    // check if the sender is the leader and the signature is valid; if yes, broadcast the Quorum proposal
    if !sender.validate(&quorum_proposal.signature, leaf.legacy_commit().as_ref()) {
        error!("Validation Failure on QuorumProposal");
        return;
    };

    let quorum_msg = QuorumProposalMessage::<TYPES> {
        proposal: quorum_proposal,
        sender,
    };
    let view_number = quorum_msg.proposal.data.view_number;
    tracing::debug!(
        "Sending Quorum proposal to the builder states for view {:?}",
        view_number
    );
    if let Err(e) = quorum_channel_sender
        .broadcast(MessageType::QuorumProposalMessage(quorum_msg))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send Quorum proposal to builder states for view {:?}",
            view_number
        );
    }
}

async fn handle_decide_event<TYPES: NodeType>(
    decide_channel_sender: &BroadcastSender<MessageType<TYPES>>,
    latest_decide_view_number: TYPES::Time,
) {
    let decide_msg: DecideMessage<TYPES> = DecideMessage::<TYPES> {
        latest_decide_view_number,
    };
    tracing::debug!(
        "Sending Decide event to builder states for view {:?}",
        latest_decide_view_number
    );
    if let Err(e) = decide_channel_sender
        .broadcast(MessageType::DecideMessage(decide_msg))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send Decide event to builder states for view {:?}",
            latest_decide_view_number
        );
    }
}

pub(crate) async fn handle_received_txns<TYPES: NodeType>(
    tx_sender: &BroadcastSender<Arc<ReceivedTransaction<TYPES>>>,
    txns: Vec<TYPES::Transaction>,
    source: TransactionSource,
) -> Vec<Result<Commitment<<TYPES as NodeType>::Transaction>, BuildError>> {
    let mut results = Vec::with_capacity(txns.len());
    let time_in = Instant::now();
    for tx in txns.into_iter() {
        let commit = tx.commit();
        let res = tx_sender
            .try_broadcast(Arc::new(ReceivedTransaction {
                tx,
                source: source.clone(),
                commit,
                time_in,
            }))
            .inspect(|val| {
                if let Some(evicted_txn) = val {
                    tracing::warn!(
                        "Overflow mode enabled, transaction {} evicted",
                        evicted_txn.commit
                    );
                }
            })
            .map(|_| commit)
            .inspect_err(|err| {
                tracing::warn!("Failed to broadcast txn with commit {:?}: {}", commit, err);
            })
            .map_err(|err| match err {
                TrySendError::Full(_) => BuildError::Error {
                    message: "Too many transactions".to_owned(),
                },
                e => BuildError::Error {
                    message: format!("Internal error when submitting transaction: {}", e),
                },
            });
        results.push(res);
    }
    results
}
